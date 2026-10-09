#!/usr/bin/env bash
# test_mail_ui.sh - E2E test suite for Mail SSR UI routes (/mail/*)
#
# Verifies:
# - Mail UI routes render without crashing against a local sqlite DB + storage root
# - Key pages contain expected markers: index, project view, inbox, thread, search, compose
# - Message body sanitization neutralizes common XSS payloads (no javascript: URLs / event handlers)
#
# Artifacts:
# - Server logs: tests/artifacts/mail_ui/<timestamp>/logs/server_mail_ui.log
# - Per-route HTML/JSON responses + headers + curl stderr
# - Network traces: tests/artifacts/mail_ui/<timestamp>/network_trace.jsonl
# - Policy traces: tests/artifacts/mail_ui/<timestamp>/policy_trace.json
# - Seed tool calls (JSON-RPC request/response) for debugging

set -euo pipefail

E2E_SUITE="mail_ui"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=../../scripts/e2e_lib.sh
source "${SCRIPT_DIR}/../../scripts/e2e_lib.sh"

e2e_init_artifacts
e2e_banner "Mail UI E2E Test Suite"

if ! command -v curl >/dev/null 2>&1; then
    e2e_log "curl not found; skipping suite"
    e2e_skip "curl required"
    e2e_save_artifact "env_dump.txt" "$(e2e_dump_env 2>&1)"
    e2e_summary
    exit 0
fi

WORK="$(e2e_mktemp "e2e_mail_ui")"
DB_PATH="${WORK}/db.sqlite3"
STORAGE_ROOT="${WORK}/storage_root"
TOKEN="e2e-token"

if ! e2e_start_server_with_logs "${DB_PATH}" "${STORAGE_ROOT}" "mail_ui" \
    "HTTP_PATH=/api" \
    "HTTP_BEARER_TOKEN=${TOKEN}" \
    "HTTP_ALLOW_LOCALHOST_UNAUTHENTICATED=0" \
    "HTTP_RBAC_ENABLED=0" \
    "HTTP_RATE_LIMIT_ENABLED=0"; then
    e2e_fail "server failed to start (port not open)"
    e2e_save_artifact "env_dump.txt" "$(e2e_dump_env 2>&1)"
    e2e_summary
    exit 1
fi
MAIL_UI_BROWSER_SESSION=""
trap 'if [ -n "${MAIL_UI_BROWSER_SESSION}" ]; then mail_ui_browser close >/dev/null 2>&1 || true; fi; e2e_stop_server || true' EXIT

API_URL="${E2E_SERVER_URL%/mcp/}/api/"
NETWORK_TRACE_FILE="${E2E_ARTIFACT_DIR}/network_trace.jsonl"
POLICY_TRACE_FILE="${E2E_ARTIFACT_DIR}/policy_trace.json"
touch "${NETWORK_TRACE_FILE}"

append_network_trace() {
    local case_id="$1"
    local kind="$2"
    local method="$3"
    local url="$4"
    local status="$5"
    local expected_status="$6"
    local auth_mode="$7"
    local rc="$8"
    local elapsed_ms="$9"
    local headers_file="${10}"
    local body_file="${11}"
    local stderr_file="${12}"

    python3 - "${NETWORK_TRACE_FILE}" \
        "${case_id}" "${kind}" "${method}" "${url}" "${status}" "${expected_status}" "${auth_mode}" "${rc}" "${elapsed_ms}" \
        "${headers_file}" "${body_file}" "${stderr_file}" <<'PY'
import datetime as dt
import json
import os
import sys

(
    trace_path,
    case_id,
    kind,
    method,
    url,
    status,
    expected_status,
    auth_mode,
    rc,
    elapsed_ms,
    headers_path,
    body_path,
    stderr_path,
) = sys.argv[1:]

event = {
    "ts": dt.datetime.now(dt.timezone.utc).isoformat(),
    "suite": "mail_ui",
    "case_id": case_id,
    "kind": kind,
    "request": {"method": method, "url": url, "auth_mode": auth_mode},
    "response": {
        "status": status,
        "expected_status": expected_status,
        "curl_rc": int(rc),
        "elapsed_ms": int(elapsed_ms),
    },
    "artifacts": {
        "headers_path": os.path.basename(headers_path),
        "body_path": os.path.basename(body_path),
        "stderr_path": os.path.basename(stderr_path),
    },
}

with open(trace_path, "a", encoding="utf-8") as f:
    f.write(json.dumps(event, sort_keys=True) + "\n")
PY
}

write_policy_trace() {
    python3 - "${POLICY_TRACE_FILE}" \
        "${E2E_ARTIFACT_DIR}/mail_index_status.txt" \
        "${E2E_ARTIFACT_DIR}/mail_no_auth_status.txt" \
        "${E2E_ARTIFACT_DIR}/mail_invalid_token_status.txt" <<'PY'
import datetime as dt
import json
import os
import sys

out_path, index_path, no_auth_path, bad_token_path = sys.argv[1:]

def read_status(path):
    try:
        return open(path, "r", encoding="utf-8").read().strip()
    except OSError:
        return None

checks = [
    {"case_id": "mail_index", "auth_mode": "bearer_valid", "expected_status": "200", "actual_status": read_status(index_path)},
    {"case_id": "mail_no_auth", "auth_mode": "none", "expected_status": "401", "actual_status": read_status(no_auth_path)},
    {"case_id": "mail_invalid_token", "auth_mode": "bearer_invalid", "expected_status": "401", "actual_status": read_status(bad_token_path)},
]
passed = all(c["actual_status"] == c["expected_status"] for c in checks)

payload = {
    "generated_at": dt.datetime.now(dt.timezone.utc).isoformat(),
    "suite": "mail_ui",
    "policy_inputs": {
        "http_allow_localhost_unauthenticated": os.getenv("HTTP_ALLOW_LOCALHOST_UNAUTHENTICATED", "0"),
        "http_rbac_enabled": os.getenv("HTTP_RBAC_ENABLED", "0"),
        "http_rate_limit_enabled": os.getenv("HTTP_RATE_LIMIT_ENABLED", "0"),
        "http_bearer_token_configured": bool(os.getenv("HTTP_BEARER_TOKEN")),
    },
    "checks": checks,
    "passed": passed,
}

with open(out_path, "w", encoding="utf-8") as f:
    json.dump(payload, f, indent=2, sort_keys=True)
PY
}

rpc_call() {
    local case_id="$1"
    local tool_name="$2"
    local args_json="$3"

    local helper_case="seed_${case_id}"
    local case_dir="${E2E_ARTIFACT_DIR}/${helper_case}"
    local request_file="${E2E_ARTIFACT_DIR}/seed_${case_id}_request.json"
    local headers_file="${E2E_ARTIFACT_DIR}/seed_${case_id}_headers.txt"
    local body_file="${E2E_ARTIFACT_DIR}/seed_${case_id}_body.json"
    local status_file="${E2E_ARTIFACT_DIR}/seed_${case_id}_status.txt"
    local timing_file="${E2E_ARTIFACT_DIR}/seed_${case_id}_timing.txt"
    local curl_stderr_file="${E2E_ARTIFACT_DIR}/seed_${case_id}_curl_stderr.txt"

    e2e_mark_case_start "${helper_case}"
    if ! e2e_rpc_call "${helper_case}" "${API_URL}" "${tool_name}" "${args_json}" "authorization: Bearer ${TOKEN}"; then
        :
    fi

    cp "${case_dir}/request.json" "${request_file}" 2>/dev/null || true
    cp "${case_dir}/headers.txt" "${headers_file}" 2>/dev/null || true
    cp "${case_dir}/response.json" "${body_file}" 2>/dev/null || true
    cp "${case_dir}/status.txt" "${status_file}" 2>/dev/null || true
    cp "${case_dir}/timing.txt" "${timing_file}" 2>/dev/null || true
    cp "${case_dir}/curl_stderr.txt" "${curl_stderr_file}" 2>/dev/null || true

    local status rc elapsed_ms
    status="$(cat "${status_file}" 2>/dev/null || echo "000")"
    elapsed_ms="$(cat "${timing_file}" 2>/dev/null || echo "0")"
    rc=0
    if [ "${status}" = "000" ]; then
        rc=1
    fi

    append_network_trace \
        "${case_id}" "mcp_rpc" "POST" "${API_URL}" "${status}" "200" "bearer_valid" "${rc}" "${elapsed_ms}" \
        "${headers_file}" "${body_file}" "${curl_stderr_file}"

    if [ "${status}" != "200" ]; then
        e2e_fail "seed_${case_id}: unexpected HTTP status ${status}"
        return 1
    fi
    return 0
}

extract_tool_json() {
    local resp_file="$1"
    python3 -c "
import json, sys
data = json.load(open(sys.argv[1], 'r', encoding='utf-8'))
res = data.get('result') or {}
content = res.get('content') or []
if content and isinstance(content[0], dict) and content[0].get('type') == 'text':
    print(content[0].get('text') or '')
else:
    # Fallback: dump result directly (best-effort)
    print(json.dumps(res))
" "$resp_file"
}

http_get() {
    local case_id="$1"
    local url="$2"
    http_get_capture "${case_id}" "${url}" "200" "bearer_valid" "authorization: Bearer ${TOKEN}"
}

http_get_expect_status() {
    local case_id="$1"
    local url="$2"
    local expected_status="$3"
    http_get_capture "${case_id}" "${url}" "${expected_status}" "bearer_valid" "authorization: Bearer ${TOKEN}"
}

http_get_expect_status_no_auth() {
    local case_id="$1"
    local url="$2"
    local expected_status="$3"
    http_get_capture "${case_id}" "${url}" "${expected_status}" "none"
}

http_get_expect_status_invalid_token() {
    local case_id="$1"
    local url="$2"
    local expected_status="$3"
    http_get_capture "${case_id}" "${url}" "${expected_status}" "bearer_invalid" "authorization: Bearer invalid-token"
}

http_get_capture() {
    local case_id="$1"
    local url="$2"
    local expected_status="$3"
    local auth_mode="$4"
    local auth_header="${5:-}"

    local case_dir="${E2E_ARTIFACT_DIR}/${case_id}"
    local case_headers_file="${case_dir}/headers.txt"
    local case_body_file="${case_dir}/response.txt"
    local case_status_file="${case_dir}/status.txt"
    local case_timing_file="${case_dir}/timing.txt"
    local case_curl_stderr_file="${case_dir}/curl_stderr.txt"
    local case_curl_args_file="${case_dir}/curl_args.txt"

    local headers_file="${E2E_ARTIFACT_DIR}/${case_id}_headers.txt"
    local body_file="${E2E_ARTIFACT_DIR}/${case_id}_body.txt"
    local status_file="${E2E_ARTIFACT_DIR}/${case_id}_status.txt"
    local timing_file="${E2E_ARTIFACT_DIR}/${case_id}_timing.txt"
    local curl_stderr_file="${E2E_ARTIFACT_DIR}/${case_id}_curl_stderr.txt"
    local curl_args_file="${E2E_ARTIFACT_DIR}/${case_id}_curl_args.txt"

    mkdir -p "${case_dir}"
    e2e_mark_case_start "${case_id}"

    local args=(
        -sS
        -D "${case_headers_file}"
        -o "${case_body_file}"
        -w "%{http_code}"
        -X GET
        "${url}"
    )
    if [ -n "${auth_header}" ]; then
        args+=(-H "${auth_header}")
    fi

    printf "curl %s\n" "$(printf "%q " "${args[@]}")" > "${case_curl_args_file}"

    local start_ns end_ns elapsed_ms
    start_ns="$(date +%s%N)"
    set +e
    local status
    status="$(curl "${args[@]}" 2>"${case_curl_stderr_file}")"
    local rc=$?
    set -e
    end_ns="$(date +%s%N)"
    elapsed_ms=$(( (end_ns - start_ns) / 1000000 ))

    echo "${status}" > "${case_status_file}"
    echo "${elapsed_ms}" > "${case_timing_file}"

    cp "${case_headers_file}" "${headers_file}" 2>/dev/null || true
    cp "${case_body_file}" "${body_file}" 2>/dev/null || true
    cp "${case_status_file}" "${status_file}" 2>/dev/null || true
    cp "${case_timing_file}" "${timing_file}" 2>/dev/null || true
    cp "${case_curl_stderr_file}" "${curl_stderr_file}" 2>/dev/null || true
    cp "${case_curl_args_file}" "${curl_args_file}" 2>/dev/null || true

    append_network_trace \
        "${case_id}" "http_get" "GET" "${url}" "${status}" "${expected_status}" "${auth_mode}" "${rc}" "${elapsed_ms}" \
        "${headers_file}" "${body_file}" "${curl_stderr_file}"
    if [ "$rc" -ne 0 ]; then
        e2e_fail "${case_id}: curl failed rc=${rc}"
        return 1
    fi
    if [ "${status}" != "${expected_status}" ]; then
        e2e_fail "${case_id}: unexpected HTTP status ${status} (expected ${expected_status})"
        return 1
    fi
    return 0
}

# ---------------------------------------------------------------------------
# Seed data via JSON-RPC tools
# ---------------------------------------------------------------------------

e2e_case_banner "Seed: ensure_project + register_agent + send_message"

PROJECT_DIR="$(e2e_mktemp "e2e_mail_ui_project")"

rpc_call "ensure_project" "ensure_project" "{\"human_key\": \"${PROJECT_DIR}\"}" || true
PROJECT_JSON="$(extract_tool_json "${E2E_ARTIFACT_DIR}/seed_ensure_project_body.json")"
PROJECT_SLUG="$(python3 -c "import json, sys; print(json.loads(sys.argv[1])['slug'])" "$PROJECT_JSON")"

rpc_call "register_agent_sender" "register_agent" "{\"project_key\": \"${PROJECT_DIR}\", \"program\": \"e2e\", \"model\": \"test\", \"name\": \"RedFox\", \"task_description\": \"e2e seed\"}" || true
rpc_call "register_agent_recipient" "register_agent" "{\"project_key\": \"${PROJECT_DIR}\", \"program\": \"e2e\", \"model\": \"test\", \"name\": \"BlueBear\", \"task_description\": \"e2e seed\"}" || true

# Keep seed messaging focused on UI behavior (avoid contact-policy gating).
rpc_call "set_policy_sender_open" "set_contact_policy" "{\"project_key\": \"${PROJECT_DIR}\", \"agent_name\": \"RedFox\", \"policy\": \"open\"}" || true
rpc_call "set_policy_recipient_open" "set_contact_policy" "{\"project_key\": \"${PROJECT_DIR}\", \"agent_name\": \"BlueBear\", \"policy\": \"open\"}" || true

XSS_MD=$'Hello <script>alert(1)</script>\\n\\n[click](javascript:alert(2))\\n\\n<img src=\"x\" onerror=\"alert(3)\">\\n'
rpc_call "send_message" "send_message" "$(python3 -c "import json,sys; print(json.dumps({\"project_key\": sys.argv[1], \"sender_name\": \"RedFox\", \"to\": [\"BlueBear\"], \"subject\": \"[br-123] XSS probe\", \"body_md\": sys.argv[2], \"thread_id\": \"br-123\"}))" "${PROJECT_DIR}" "${XSS_MD}")" || true

e2e_pass "seeded project=${PROJECT_SLUG}"

# ---------------------------------------------------------------------------
# Fetch pages (/mail/*)
# ---------------------------------------------------------------------------

BASE_URL="${API_URL%/api/}"

e2e_case_banner "GET /mail (index)"
http_get "mail_index" "${BASE_URL}/mail" || true
MAIL_INDEX_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_index_body.txt")"
e2e_assert_contains "index includes Projects title" "${MAIL_INDEX_BODY}" "Projects"
e2e_assert_contains "index includes project slug" "${MAIL_INDEX_BODY}" "${PROJECT_SLUG}"

e2e_case_banner "GET /mail (missing auth bearer -> 401)"
http_get_expect_status_no_auth "mail_no_auth" "${BASE_URL}/mail" "401" || true

e2e_case_banner "GET /mail (invalid auth bearer -> 401)"
http_get_expect_status_invalid_token "mail_invalid_token" "${BASE_URL}/mail" "401" || true

e2e_case_banner "GET /mail/${PROJECT_SLUG} (project view)"
http_get "mail_project" "${BASE_URL}/mail/${PROJECT_SLUG}" || true
MAIL_PROJECT_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_project_body.txt")"
e2e_assert_contains "project page includes slug" "${MAIL_PROJECT_BODY}" "${PROJECT_SLUG}"

e2e_case_banner "GET /mail/${PROJECT_SLUG}/inbox/BlueBear (inbox)"
http_get "mail_inbox" "${BASE_URL}/mail/${PROJECT_SLUG}/inbox/BlueBear?limit=50&page=1" || true
MAIL_INBOX_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_inbox_body.txt")"
e2e_assert_contains "inbox contains subject" "${MAIL_INBOX_BODY}" "[br-123] XSS probe"

e2e_case_banner "GET /mail/${PROJECT_SLUG}/thread/br-123 (thread)"
http_get "mail_thread" "${BASE_URL}/mail/${PROJECT_SLUG}/thread/br-123" || true
MAIL_THREAD_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_thread_body.txt")"
e2e_assert_contains "thread contains subject" "${MAIL_THREAD_BODY}" "[br-123] XSS probe"
e2e_assert_contains "thread contains sender" "${MAIL_THREAD_BODY}" "RedFox"
e2e_assert_contains "thread has markdown text" "${MAIL_THREAD_BODY}" "click"
e2e_assert_contains "thread has img tag" "${MAIL_THREAD_BODY}" "<img"
e2e_assert_not_contains "thread strips script tag" "${MAIL_THREAD_BODY}" "<script>alert(1)"
e2e_assert_contains "thread preserves script text as plain text" "${MAIL_THREAD_BODY}" "alert(1)"
e2e_assert_contains "thread neutralizes javascript url" "${MAIL_THREAD_BODY}" "click"
e2e_assert_contains "thread strips onerror attr" "${MAIL_THREAD_BODY}" "img"
e2e_assert_not_contains "thread neutralizes javascript href (double quotes)" "${MAIL_THREAD_BODY}" "href=\"javascript:"
e2e_assert_not_contains "thread neutralizes javascript href (single quotes)" "${MAIL_THREAD_BODY}" "href='javascript:"
e2e_assert_not_contains "thread does not include onerror attribute" "${MAIL_THREAD_BODY}" "onerror="

e2e_case_banner "GET /mail/${PROJECT_SLUG}/search?q=br-123 (search results)"
http_get "mail_search" "${BASE_URL}/mail/${PROJECT_SLUG}/search?q=br-123&limit=10" || true
MAIL_SEARCH_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_search_body.txt")"
e2e_assert_contains "search includes subject" "${MAIL_SEARCH_BODY}" "[br-123] XSS probe"

e2e_case_banner "GET /mail/${PROJECT_SLUG}/search?q=<script>... (xss escaped)"
SEARCH_XSS_QUERY="%3Cscript%3Ealert(1)%3C%2Fscript%3E"
http_get "mail_search_xss_query" "${BASE_URL}/mail/${PROJECT_SLUG}/search?q=${SEARCH_XSS_QUERY}&limit=10" || true
MAIL_SEARCH_XSS_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_search_xss_query_body.txt")"
e2e_assert_not_contains "search query xss is not rendered as script tag" "${MAIL_SEARCH_XSS_BODY}" "<script>alert(1)</script>"
e2e_assert_contains "search query xss is html-escaped in input value" "${MAIL_SEARCH_XSS_BODY}" "value=\"&lt;script&gt;alert(1)&lt;&#x2f;script&gt;\""

e2e_case_banner "GET /mail/${PROJECT_SLUG}/overseer/compose (compose)"
http_get "mail_compose" "${BASE_URL}/mail/${PROJECT_SLUG}/overseer/compose" || true
MAIL_COMPOSE_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_compose_body.txt")"
e2e_assert_contains "compose includes agents" "${MAIL_COMPOSE_BODY}" "BlueBear"

e2e_case_banner "GET /mail/unified-inbox (unified)"
http_get "mail_unified" "${BASE_URL}/mail/unified-inbox?limit=50" || true
MAIL_UNIFIED_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_unified_body.txt")"
e2e_assert_contains "unified inbox includes subject" "${MAIL_UNIFIED_BODY}" "[br-123] XSS probe"

e2e_case_banner "GET /mail/api/unified-inbox (json api)"
http_get "mail_api_unified" "${BASE_URL}/mail/api/unified-inbox" || true
MAIL_API_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_api_unified_body.txt")"
e2e_assert_contains "api returns JSON array" "${MAIL_API_BODY}" "\"messages\""

e2e_case_banner "GET /mail/archive/browser/{project}/file invalid path (json 400)"
http_get_expect_status "mail_archive_file_invalid_path" "${BASE_URL}/mail/archive/browser/${PROJECT_SLUG}/file?path=../etc/passwd" "400" || true
MAIL_ARCHIVE_FILE_INVALID_HEADERS="$(cat "${E2E_ARTIFACT_DIR}/mail_archive_file_invalid_path_headers.txt")"
MAIL_ARCHIVE_FILE_INVALID_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_archive_file_invalid_path_body.txt")"
e2e_assert_contains "archive file invalid path has JSON content-type" "${MAIL_ARCHIVE_FILE_INVALID_HEADERS}" "application/json"
e2e_assert_contains "archive file invalid path body has detail" "${MAIL_ARCHIVE_FILE_INVALID_BODY}" "\"detail\""
e2e_assert_contains "archive file invalid path message" "${MAIL_ARCHIVE_FILE_INVALID_BODY}" "Invalid file path"

e2e_case_banner "GET /mail/archive/browser/{project}/file missing file (json 404)"
http_get_expect_status "mail_archive_file_not_found" "${BASE_URL}/mail/archive/browser/${PROJECT_SLUG}/file?path=messages/missing.md" "404" || true
MAIL_ARCHIVE_FILE_NOT_FOUND_HEADERS="$(cat "${E2E_ARTIFACT_DIR}/mail_archive_file_not_found_headers.txt")"
MAIL_ARCHIVE_FILE_NOT_FOUND_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_archive_file_not_found_body.txt")"
e2e_assert_contains "archive file missing has JSON content-type" "${MAIL_ARCHIVE_FILE_NOT_FOUND_HEADERS}" "application/json"
e2e_assert_contains "archive file missing has detail" "${MAIL_ARCHIVE_FILE_NOT_FOUND_BODY}" "\"detail\""
e2e_assert_contains "archive file missing message" "${MAIL_ARCHIVE_FILE_NOT_FOUND_BODY}" "File not found"

SNAP_TS="$(
python3 - <<'PY'
from datetime import datetime, timezone
print(datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M"))
PY
)"
e2e_case_banner "GET /mail/archive/time-travel/snapshot (json 200)"
http_get "mail_archive_snapshot" "${BASE_URL}/mail/archive/time-travel/snapshot?project=${PROJECT_SLUG}&agent=BlueBear&timestamp=${SNAP_TS}" || true
MAIL_ARCHIVE_SNAPSHOT_HEADERS="$(cat "${E2E_ARTIFACT_DIR}/mail_archive_snapshot_headers.txt")"
MAIL_ARCHIVE_SNAPSHOT_BODY="$(cat "${E2E_ARTIFACT_DIR}/mail_archive_snapshot_body.txt")"
e2e_assert_contains "archive snapshot has JSON content-type" "${MAIL_ARCHIVE_SNAPSHOT_HEADERS}" "application/json"
e2e_assert_contains "archive snapshot has messages key" "${MAIL_ARCHIVE_SNAPSHOT_BODY}" "\"messages\""
e2e_assert_contains "archive snapshot has snapshot_time key" "${MAIL_ARCHIVE_SNAPSHOT_BODY}" "\"snapshot_time\""
e2e_assert_contains "archive snapshot has commit_sha key" "${MAIL_ARCHIVE_SNAPSHOT_BODY}" "\"commit_sha\""
e2e_assert_contains "archive snapshot has requested_time key" "${MAIL_ARCHIVE_SNAPSHOT_BODY}" "\"requested_time\""

write_policy_trace
POLICY_TRACE_BODY="$(cat "${POLICY_TRACE_FILE}")"
e2e_assert_contains "policy trace includes suite" "${POLICY_TRACE_BODY}" "\"suite\": \"mail_ui\""
e2e_assert_contains "policy trace includes no-auth check" "${POLICY_TRACE_BODY}" "\"case_id\": \"mail_no_auth\""
e2e_assert_contains "policy trace includes invalid-token check" "${POLICY_TRACE_BODY}" "\"case_id\": \"mail_invalid_token\""
e2e_assert_contains "policy trace passed=true" "${POLICY_TRACE_BODY}" "\"passed\": true"

# Exercise Alpine, DOM rendering and Lucide in an owned Chromium session.
# Synthetic messages stay in this browser model; the mailbox seed stays small.
mail_ui_browser() {
    env -u AGENT_BROWSER_AUTO_CONNECT -u AGENT_BROWSER_CDP \
        -u AGENT_BROWSER_PROFILE -u AGENT_BROWSER_STATE -u AGENT_BROWSER_RESTORE \
        -u AGENT_BROWSER_SESSION_NAME -u AGENT_BROWSER_PROVIDER \
        -u AGENT_BROWSER_ARGS -u AGENT_BROWSER_HEADED \
        agent-browser --engine chrome --session "${MAIL_UI_BROWSER_SESSION}" "$@"
}

test_unified_inbox_browser() {
    if ! command -v agent-browser >/dev/null 2>&1; then
        e2e_skip "unified inbox browser regression: agent-browser required"
        return
    fi
    e2e_case_banner "Unified inbox browser regression (1000 messages)"
    MAIL_UI_BROWSER_SESSION="mail-ui-e2e-$$-${RANDOM}"
    local browser_log="${E2E_ARTIFACT_DIR}/unified_browser.log"
    local browser_result="${E2E_ARTIFACT_DIR}/unified_browser_result.json"
    if ! mail_ui_browser open "${BASE_URL}/mail/unified-inbox?limit=1000&token=${TOKEN}" >"${browser_log}" 2>&1 ||
        ! mail_ui_browser wait --fn "window.Alpine && window.lucide && document.querySelector('[x-data=\"unifiedInboxManager()\"]')?._x_dataStack?.[0]?.allMessages?.length > 0" >>"${browser_log}" 2>&1; then
        e2e_fail "unified inbox browser could not initialize; see unified_browser.log"
        return
    fi
    if ! mail_ui_browser --json eval --stdin >"${browser_result}" 2>>"${browser_log}" <<'JS'
(async () => {
  const root = document.querySelector('[x-data="unifiedInboxManager()"]');
  const inbox = Alpine.$data(root);
  const checks = [];
  const check = (name, passed) => checks.push({ name, passed: !!passed });
  const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
  const settle = async () => { await Alpine.nextTick(); await sleep(250); };
  const realFetch = window.fetch;
  const realTimeout = window.setTimeout;
  const realIcons = lucide.createIcons;
  const delays = new Map();
  let mode = 'unchanged', payload, release, active = 0, maxActive = 0, requests = 0, aborted = false;
  let iconCalls = 0, iconMutations = 0, probe;
  const observer = new MutationObserver(records => {
    iconMutations += records.flatMap(record => [...record.addedNodes])
      .filter(node => node instanceof Element && (node.matches('[data-lucide]') || node.querySelector('[data-lucide]'))).length;
  });
  try {
    inbox.autoRefreshEnabled = false;
    inbox.handleAutoRefreshToggle();
    const seed = inbox.allMessages[0];
    inbox.allMessages = Array.from({ length: 1000 }, (_, index) => ({
      ...seed, id: 100000 + index, subject: `Browser regression message ${index}`,
      created: new Date(Date.UTC(2026, 0, 1) + index * 1000).toISOString()
    }));
    inbox.selectedMessage = null;
    inbox.filterMessages();
    await settle();
    check('all 1000 messages remain searchable', inbox.filteredMessages.length === 1000);
    check('at most 50 message rows render', root.querySelectorAll('[data-message-id]').length === 50);
    inbox.searchQuery = 'Browser regression message 999';
    inbox.filterMessages();
    await settle();
    check('search finds a message outside the initial page', inbox.filteredMessages.length === 1 && inbox.filteredMessages[0].id === 100999);
    inbox.searchQuery = '';
    inbox.filterMessages();
    inbox.selectMessage(inbox.filteredMessages[49]);
    await settle();
    document.activeElement?.blur();
    document.dispatchEvent(new KeyboardEvent('keydown', { key: 'j', bubbles: true }));
    await settle();
    check('keyboard navigation crosses page boundary', inbox.messagePage === 1 && inbox.selectedMessage.id === inbox.filteredMessages[50].id);
    check('second page renders selected message within 50 rows', root.querySelectorAll('[data-message-id]').length === 50 && !!root.querySelector(`[data-message-id="${inbox.selectedMessage.id}"]`));

    payload = JSON.parse(JSON.stringify(inbox.allMessages));
    window.fetch = (input, options = {}) => {
      const url = new URL(typeof input === 'string' ? input : input.url, location.href);
      if (url.pathname !== '/mail/api/unified-inbox') return realFetch(input, options);
      requests++; active++; maxActive = Math.max(maxActive, active);
      return new Promise((resolve, reject) => {
        const finish = () => {
          active--;
          resolve(new Response(JSON.stringify({ messages: payload }), {
            status: mode === '503' ? 503 : mode === '500' ? 500 : 200,
            headers: { 'Content-Type': 'application/json' }
          }));
        };
        if (mode === 'deferred') {
          release = finish;
          options.signal.addEventListener('abort', () => {
            aborted = true; active--; reject(new DOMException('Aborted', 'AbortError'));
          }, { once: true });
        } else finish();
      });
    };
    const allBefore = inbox.allMessages, filteredBefore = inbox.filteredMessages;
    const selectedBefore = inbox.selectedMessage;
    const bodyBefore = document.getElementById(`message-body-${selectedBefore.id}`);
    const contentBefore = bodyBefore?.firstChild;
    inbox.selectedMessages = [selectedBefore.id];
    const selectionBefore = inbox.selectedMessages;
    await inbox.fetchLatestMessages({ silent: true });
    await settle();
    check('unchanged refresh preserves array and filter identity', inbox.allMessages === allBefore && inbox.filteredMessages === filteredBefore);
    check('unchanged refresh preserves selection and body DOM identity', inbox.selectedMessage === selectedBefore && inbox.selectedMessages === selectionBefore && document.getElementById(`message-body-${selectedBefore.id}`) === bodyBefore && bodyBefore?.firstChild === contentBefore && !!contentBefore);
    payload = payload.map(message => message.id === selectedBefore.id ? { ...message, sender_label: `Officer Alpha <${message.sender}>` } : message);
    payload.unshift({ ...payload[0], id: 101001, subject: 'New browser message', created: '2027-01-01T00:00:00Z' });
    await inbox.fetchLatestMessages({ silent: true });
    await settle();
    check('changed label and new message appear', inbox.allMessages.length === 1001 && inbox.selectedMessage.sender_label === `Officer Alpha <${selectedBefore.sender}>` && root.textContent.includes('Officer Alpha'));
    check('changed refresh retains selected address and message', inbox.selectedMessage.id === selectedBefore.id && inbox.selectedMessage.sender === selectedBefore.sender && inbox.selectedMessages.includes(selectedBefore.id));

    mode = 'deferred';
    const requestsBefore = requests;
    const inFlight = inbox.fetchLatestMessages({ silent: true });
    await inbox.fetchLatestMessages();
    inbox.autoRefreshSeconds = 0.01;
    inbox.autoRefreshEnabled = true;
    inbox.scheduleAutoRefresh();
    await sleep(50);
    check('manual and automatic overlap allow only one request', requests === requestsBefore + 1 && maxActive === 1 && active === 1);
    inbox.autoRefreshEnabled = false;
    mode = 'unchanged';
    release();
    await inFlight;

    window.setTimeout = (callback, delay, ...args) => {
      const handle = realTimeout(callback, delay, ...args);
      delays.set(handle, delay);
      return handle;
    };
    inbox.autoRefreshSeconds = 45;
    inbox.autoRefreshEnabled = true;
    const usableMessages = inbox.allMessages;
    for (const status of ['503', '500']) {
      mode = status;
      const failuresBefore = inbox.refreshFailures;
      const failedSelection = inbox.selectedMessage;
      await inbox.fetchLatestMessages({ silent: true });
      check(`HTTP ${status} preserves data and selection`, inbox.allMessages === usableMessages && inbox.selectedMessage === failedSelection && !!inbox.refreshError);
      check(`HTTP ${status} backs off automatic refresh`, inbox.refreshFailures === failuresBefore + 1 && delays.get(inbox.autoRefreshHandle) === 45000 * 2 ** inbox.refreshFailures);
      inbox.selectNextMessage();
      await settle();
      check(`HTTP ${status} leaves navigation usable`, inbox.selectedMessage && inbox.selectedMessage.id !== failedSelection.id);
    }
    inbox.autoRefreshEnabled = false;
    inbox.handleAutoRefreshToggle();
    await settle();
    lucide.createIcons = (...args) => { iconCalls++; return realIcons.apply(lucide, args); };
    observer.observe(document.body, { childList: true, subtree: true });
    await sleep(300);
    check('idle page does not repeat icon work', iconCalls === 0 && iconMutations === 0);
    probe = document.createElement('div');
    probe.innerHTML = '<i data-lucide="inbox"></i>';
    document.body.appendChild(probe);
    await sleep(250);
    check('new icon placeholder becomes SVG', !!probe.querySelector('svg') && !probe.querySelector('i') && iconCalls > 0);
    const callsAfterIcon = iconCalls, mutationsAfterIcon = iconMutations;
    await sleep(300);
    check('icon replacement settles without a mutation loop', iconCalls === callsAfterIcon && iconMutations === mutationsAfterIcon);

    mode = 'deferred';
    inbox.autoRefreshEnabled = true;
    const pending = inbox.fetchLatestMessages({ silent: true });
    const pendingController = inbox.refreshController;
    inbox.destroy();
    await pending;
    check('destroy aborts request and removes refresh handle', aborted && pendingController.signal.aborted && active === 0 && inbox.refreshController === null && inbox.autoRefreshHandle === null && inbox.destroyed);
  } catch (error) {
    check(`browser exception: ${error.stack || error}`, false);
  } finally {
    inbox.destroy();
    observer.disconnect();
    probe?.remove();
    window.fetch = realFetch;
    window.setTimeout = realTimeout;
    lucide.createIcons = realIcons;
  }
  return { passed: checks.length > 0 && checks.every(item => item.passed), checks, maxActive, iconCalls, iconMutations };
})()
JS
    then
        e2e_fail "unified inbox browser evaluation failed; see unified_browser_result.json"
        return
    fi
    if python3 - "${browser_result}" <<'PY'
import json
import sys

response = json.load(open(sys.argv[1], encoding="utf-8"))
result = (response.get("data") or {}).get("result")
if isinstance(result, str):
    result = json.loads(result)
if not response.get("success") or not isinstance(result, dict) or not result.get("passed"):
    print(json.dumps(result or response, indent=2))
    sys.exit(1)
print(f"{len(result['checks'])} browser checks passed; maximum concurrent refreshes={result['maxActive']}")
PY
    then
        e2e_pass "unified inbox browser pagination, refresh stability, failure backoff and icon settling"
    else
        e2e_fail "unified inbox browser regression failed; see unified_browser_result.json"
    fi
    mail_ui_browser close >>"${browser_log}" 2>&1 || true
    MAIL_UI_BROWSER_SESSION=""
}

test_unified_inbox_browser
e2e_summary
