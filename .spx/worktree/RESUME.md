# Resume — local Agent Mail fork

## Recovery on 2026-10-23

CURRENT: wrapper-lifetime repair cec33fb274eafa38f71722d782396402c4fb50ca
passes all ten append tests, both workspace compilation checks, strict Clippy,
fmt and diffcheck (validation command 79127 exited 0). Build 59364 exited 0 in
5m08s. Frozen binary atc-wrapper-candidate-20261023/am has SHA-256
035c8b1e0c08248ba27796d074a938b706bbbbe055591f9d23b442815929193e;
source-commit.txt records eaf47423039131f00c33e7852df553b25b4b1dcf.
Canonical gate 9817 PASSED unchanged: off p95 37.936167ms, shadow 36.713459ms,
live 36.644292ms, allowed 39.83ms; canary quick_check passed with one ATC row.
Evidence: atc-wrapper-gate-20261023*.

Paired diagnostic 47902 exited 0, 300 samples per store, same invocation with
original baseline. Treatment live overhead baseline 14.655074%, candidate
1.648694%; candidate live p95 9.486760% lower. Treatment shadow baseline
-2.346038%, candidate 1.028690%. All twelve stores passed integrity and exact
message/ATC counts. All-off controls varied: baseline shadow/live
7.465622%/6.280847%, candidate 3.251386%/0.139844%. Keep those controls with the
timing results; diagnostic qualification remains false. Evidence:
atc-wrapper-paired-20261023{,.log}. Fresh CI remains required.

Implementation audit run 2026-10-23_09-21-57-445-699dd5402eb4 is active for
02927908...eaf47423. It initially stopped on absent spx governance; the same
auditor resumed under the operator's explicit source-repair/review authorization,
which takes precedence over skill source-reading restrictions. No spec
conformance is claimed; actual repository instructions and Rust standards apply.
No replacement audit run was opened and no approval is claimed yet.

Archive probe build command 94717 exited 0 in 1m18s (storage --test it --no-run).
Current-source APFS gate 52869 exited 101: single p95 27.504ms passes 35ms;
batch-100 p95 323.050ms fails 300ms. Batch writes grow from 118.217ms to roughly
173ms while flush remains around 150ms. Crash/reopen retained 100 canonical
messages and two commits. Evidence: archive-current-apfs-20261023{,.log}.
Intrusive profile commands 22224 (probe exit 101) and 86956 (sample exit 0)
completed; archive-profile-20261023/sample.txt localizes repeated canonical-ID
scan reads and Git blob_path work. Within the scan, sampled read_message_file
stacks include 35 prefix-check lstat samples and 24 open samples, versus three
JSON-parser samples. Counts are inclusive, not elapsed-time attribution.
The profile omitted the required load admission and was instrumented; it is
localization evidence only, never qualification. Restore load admission before
the next heavy operation; do not rerun unchanged timing gates until green.
No heavy command remains active. Investigate reducing repeated path traversal
while preserving symlink/authority and collision protections; no storage
implementation change has been made. The preserved Linux serial build
is terminal exit 101/SIGKILL and supplies no probe result; the reviewed workflow
artifact change is prepared to retain the CI-built Linux probe after publication.
No push, merge, live installation or restart has occurred in this continuation.

Wrapper-lifetime repair evidence:
Profile command 15706 and macOS
sample command 35177 both exited 0. Evidence lives under
atc-schema-profile-20261023. All six stores passed integrity and counts;
sampled treatment timings are diagnostic only. The live sample includes 61
connection-open samples and 41 connection-drop samples under
record_atc_message_observations_from_tool_payload. That caller reconstructs a
DbPool wrapper for each observation; from_shared_pool_with_authority initializes
a new append cache, so wrapper Drop closes it after each request. Clone-only
reuse evidence did not cover this production reconstruction boundary.
Current imperfection: cache lifetime does not follow the shared pool across
wrappers (pool.rs constructors; live-sample.txt lines 1840 and 2120). Safe local
fix under Rust implementation/test guidance: share cache ownership through weak
registry references keyed by underlying pool identity, with wrappers owning the
cache and final-wrapper drop releasing it. New actual factory-path regression
checks retained WAL between wrappers and closure after final-owner drop. RED
command 47973 exited 101 at fixture setup: canonicalizing the configured database
URL changed the test-only ephemeral archive root, so authority validation
correctly refused the wrapper. The fixture now reuses the original configured
URL spelling. This first failure supplies no cache-lifetime RED evidence.
Corrected RED command 36692 exited 101 at the retained-WAL assertion after
dropping the transient wrapper, establishing the actual lifetime failure.
Logs: atc-wrapper-red-20261023.log and atc-wrapper-red-authority-20261023.log.
Both pool constructors now obtain an append cache from a weak registry keyed by
underlying pool identity. All reconstructed wrappers retain the same cache;
neither registry reference keeps a pool or connection alive. Formatting and
diffcheck passed. GREEN command 99256 passed the retention assertion and nine
existing append tests, then failed the added final-owner shutdown assertion.
Lifetime diagnostics 26444 and 89576 confirmed a remaining strong owner:
counts were 2 before/after transient wrappers and 1 after the test owner dropped.
Source inspection found the intended bounded resident owner in pool_registry.rs.
The test now evicts the production resident set using its declared capacity,
then requires both weak-cache expiry and WAL cleanup; no assertion was removed.
Temporary ownership prints are removed. GREEN command 29702 exited 0 with all
ten append tests passing, including cache expiry and WAL cleanup after residency
eviction. Production/test repair committed as
cec33fb274eafa38f71722d782396402c4fb50ca. Fresh sync-base returned already_current
against 0b289a5afea198449b26d6b974b76a4948404ca6. Validation command 79127 is active:
workspace all-targets check, production lib/bins check, strict workspace Clippy,
fmt and diffcheck, sequentially. Collect the same handle before another heavy
command. Logs are atc-wrapper-check-all-20261023.log,
atc-wrapper-check-production-20261023.log and atc-wrapper-clippy-20261023.log.
Regression logs:
atc-wrapper-{green,lifetime,owners,residency-green}-20261023.log.
Workspace checks, strict Clippy, committed review and performance remain pending
for this ownership repair. No publication or live mutation has occurred.

Previous checkpoint:
committed implementation audit
2026-10-23_08-14-12-558-05b0a2e930a5 approved the exact range
88026bb2...a6dbf200 with zero findings; supported journal projection consumed.
Candidate build 47872 exited 0 in 6m09s. Frozen development binary is
atc-reuse-candidate-20261023/am, SHA-256
d444f91690268cc351362428e7296604373b5d47c76867f7c132ad4279c0a5b0;
source-commit.txt records a6dbf200e0828dbceecf0b488149cfe703970cb8.
Paired diagnostic 66580 completed exit 0 after a 600s host-load wait. It compared
both binaries in one invocation with 300 samples per store and all-off controls.
Baseline live overhead was 14.889%; candidate live overhead was 5.043634%, still
above the unchanged 5% budget. Candidate live p95 was 42.025375ms versus baseline
46.686875ms (9.9846% lower); candidate off was 40.007541ms. Candidate shadow was
-0.412837%. Null controls were baseline shadow/live 1.217469%/1.025839% and
candidate -2.282126%/-3.923666%. All twelve stores had 363 messages and passed
integrity, with 363 ATC rows only in each treatment/live store. Output root/log:
atc-reuse-paired-20261023{,.log}. Qualification remains false.

Next candidate is committed as faea8b49f13dbea6080cc3a96b2b4d10c750e2cd:
retain a completed schema inspection
against SQLite's read-only main.schema_version cookie on the retained connection.
Every call reads the cookie; changed versions trigger the full required-column
inspection. Before/after cookies must agree before an inspection is retained.
Replacement connections start without a cached inspection. The DDL regression
now creates a valid index, verifies append still works, then drops the experience
table and requires the migration result. Authoritative semantics were checked at
https://www.sqlite.org/pragma.html#pragma_schema_version. Command 63152 completed
exit 0: nine append tests, both workspace checks, strict Clippy, fmt and diffcheck.
Committed schema-cache audit 2026-10-23_08-42-52-064-46d55b62438e approved
the code and architecture units with zero findings; supported journal projection
consumed. Build 29028 exited 0 in 5m05s. Frozen binary
atc-schema-candidate-20261023/am has SHA-256
f1f3f6dd371dee762e2929afa40f5f0d5224ebf30fe4a86fd30e2f6f2182404c;
source-commit.txt records a8b6bbd604eeb24f75a751d0e02fe08a776902ba.
Paired comparison command 30028 completed exit 0, using the original baseline and this
candidate, 300 samples per store, with a 900s process-group deadline. Output is
atc-schema-paired-20261023{,.log}. Host-load admission was immediate. The all-off
control completed: baseline shadow/live 1.373964%/-2.925047%, candidate
-1.660405%/-3.922220%. Treatment baseline live overhead was 12.023015%,
candidate 4.905465%; candidate live p95 was 6.235367% below baseline in the
same invocation. All twelve stores passed integrity and exact message/ATC
counts. This remains diagnostic evidence with qualification false. Fresh
sync-base returned already_current against main at
0b289a5afea198449b26d6b974b76a4948404ca6 with unchanged branch diff.
Canonical gate command 74504 completed exit 3 (regression), using the same frozen
candidate and isolated runtime/canary directories at atc-schema-gate-20261023*.
Off p95 was 34.300208ms, shadow 35.321750ms (+2.98%), live 38.160916ms
(+11.26%, above 5%). Canary quick_check passed with one ATC row; benchmark
execution itself exited 0. No fallback was applied. Profile the remaining
live-write cost before another candidate; do not repeat the gate until it passes
or treat the paired diagnostic as qualification. No heavy command is active.
No push, merge, live installation or restart has occurred in this continuation.

Connection-reuse candidate is committed as
622292162537767e5b78349a46d4c9577e18f1ff in pool.rs, queries.rs and the
atc_queries.rs test helper. A real
baseline run of `cargo test -p mcp-agent-mail-db --lib
append_atc_reuses_connection_with_committed_rows_visible_to_fresh_readers --
--nocapture` ran one test and failed at the retained-WAL assertion. The earlier
attempt with an unqualified `--exact` filter ran zero tests and supplies no
evidence. Candidate uses a pool-shared asupersync try-lock, a fresh connection
when contended, per-call authority/schema checks, Unix device/inode generation
checks, and FULL synchronous commits for all canonical ATC writers. Added real
SQLite tests cover committed independent reads, contention, schema changes,
authority rejection, and replacement with preservation of original rows.
Focused candidate command 58289 passed seven append tests. Command 54828
subsequently passed eight, adding an abrupt process-exit test that bypasses
Drop/checkpoint and recovers the committed row, plus per-pool durability
isolation. Command 40190 passed 29 ATC query/rollup tests, both required
workspace checks (all-targets and production lib/bins), strict workspace Clippy,
fmt and diffcheck. No performance qualification, publication or installation.

The process-global relaxed-durability test override was replaced with a
test-only per-pool flag. The real-connection test observes OFF in the rollup
fixture and FULL in an independent append pool; 29 rollup tests also pass.
Nextest is unavailable here; selected Cargo tests use timeout with process-group
termination rather than leaving a long test run unbounded.

Advisory implementation audit 2026-10-23_07-43-34-922-2dc1d51ba2fb rejected the
candidate with one blocking finding: a panic after BEGIN IMMEDIATE can leave
the cached connection/transaction inside a poisoned mutex, blocking subsequent
fresh writers. Current ledger classification: safe local ownership repair in
queries.rs, governed by Rust remediation and the operator's integrity and
responsiveness requirements. Proposed repair takes the connection out of the
cache during schema/transaction work and restores it only on successful return;
unwinding then drops the connection and rolls back. A real SQLite regression
uses an eviction tracing callback panic and a zero-wait independent write-lock
probe. RED command 49592 ran one test and failed with database-is-locked at
that zero-wait probe after the intended observer panic (exit 101). The ownership
repair is applied: take the connection out of the cache before inspection and
transaction work, restore it after successful readback only. GREEN and full
validation command 78470 passed the post-panic write-lock probe but failed the
later append readback: its terminal row shared the old rows' resolution time,
so documented cutoff-tie compaction removed it too. The later append now uses
a later timestamp; the RED lock-probe scenario and assertion are unchanged.
Command 39658 passed nine append tests, 29 rollup tests, all-targets check and
production lib/bins check. Strict Clippy found two test-code issues: callback
type complexity and manual if/panic. These are repaired with an EventObserver
type alias and the suggested assert form of the injected panic. Command 19247
completed exit 0: nine append tests, strict workspace Clippy, fmt, diffcheck.
Production code did not change after both workspace checks passed. The unwind
defect is fixed with red/green real-SQLite evidence; fresh committed review and
the candidate binary build/performance comparison remain required. Commit hooks
ran normally; no source changes remain outside the committed checkpoint.

External paired diagnostic at preservation-directory
atc-paired-diagnostic-20261023.py passed Ruff and strict mypy. Smoke command
43413 exited 0 using the same frozen baseline in both positions, one measured
sample per store plus 63 seed/warmup sends. All twelve stores passed integrity,
message counts and mode-specific ATC counts. Results are in
atc-paired-smoke-20261023; smoke timings are not performance evidence.
The full diagnostic keeps both binaries' off/shadow/live servers in the same
invocation, balances request order, and includes all-off controls. Defaults to
300 samples per store and retains qualification:false. It has not yet run
against the new candidate, which still needs a binary build after remediation.

Continuation checkpoint: candidate diagnostic 76977 completed with exit 0;
sampling command 74531 also completed with exit 0. The host waiter admitted
the run after 512 seconds at normalized load 0.373/0.689/0.733. No heavy command
remains active. Results and live-sample.txt are under the preservation directory
`atc-candidate-profile-20261023-resumed`. All six stores passed integrity/count
checks; treatment live retained 363 ATC rows and the other modes retained zero.
The all-off control measured -1.842%/-0.867%; treatment measured shadow 0.893%
and live 62.658%. The live process was sampled during treatment, so those
treatment timings are diagnostic and cannot qualify performance.

Candidate profile localizes the next optimization: 47 inclusive samples in
connection opening, 39 in closing, 30 in append/readback, and 10 in schema
inspection. WAL close includes checkpoint/fsync/unmap/unlink work. Investigate
a pool-owned reusable canonical ATC append connection while retaining per-call
filesystem-authority checks, schema checks, transactional row ceiling and
post-commit readback. Reuse must detect sidecar replacement and preserve commit
durability; retaining a NORMAL connection without the current last-close
checkpoint can change power-loss behavior. A FULL-synchronous reusable WAL
connection is a candidate to investigate, not an approved implementation.
Concurrency, replacement, shutdown/crash and durability evidence are required.

Independent review 2026-10-23_07-19-13-017-053bcf3430d5 approved all 11 changed
files with zero findings at clean committed head
88026bb2d237f2cda221071e51d1789bfb9e22fd against 111ccfd864eb. Projection was
read through spx journal render. No push or installation occurred. Review
approval establishes code review only; performance qualification still fails.

Fresh GitHub inspection confirms PR #3 remains OPEN at 111ccfd864eb. Failed
logs for 39856238361, 39856238377 and 39856238404 are preserved as
`ci-resume-<run>-failed-20261023.log`; the archive run's empty failed-step
projection required its full log, retained with `-full-` in place of `-failed-`.
Archive fsync failures: APFS single p95 69.844ms / 35ms and batch 417.516ms /
300ms; ext4 batch 387.395ms / 250ms; XFS batch 404.659ms / 250ms. ATC reports
regression. Archive benchmark exited 143 during compilation; the local workflow
already separates build and measurement deadlines, still awaiting new CI.

Safe local reporting defect fixed in scripts/bench_archive_perf_gate.sh:
errexit previously bypassed bench_rc capture and summary/comment publication on
Cargo failure. The guarded pipeline retains a nonzero status and reaches report
generation; explicit cd failure still stops the subshell. A real invalid Cargo
manifest reproduces exit 101 with only bench.log before the fix. After the fix,
the gate exits 1 and emits summary.json with runtime_error and bench_exit_code
101 plus comment.md. Red/green/final evidence is under preservation-directory
`archive-gate-failure-20261023`. Shell syntax and git diff --check pass. This
repair is committed as 321fe263 and changes failure reporting only; no performance gate passes
are claimed. Required review remains pending with the other local repairs.

The operator requested continuation in a fresh session. The original delivery
authorization was recovered from
`/Users/shz/.local/state/codex/accounts/outcomeeng/attachments/ff887783-3b13-4aed-90a0-55e382ea9c25/pasted-text-1.txt`.
Its scope remains CI repairs, required reviews, fork PR #3 merge, supervised
installation, a full 24-hour observation, and authorized build cleanup with
rollback material retained. A continuing goal now tracks that scope here.

Checkout inspection found HEAD `653fd585`, ten commits ahead of the locally
recorded remote branch, with only this resume file modified. Remote currency
has not yet been refreshed. Preserve the pending notes.

The formerly active baseline sensitivity command completed with exit 0 after
the old turn ended. Its retained `atc-interleaved-baseline-20261023` results
report all-off control differences of -4.557% / -1.079%, and treatment shadow
/ live overhead of -1.189% / 15.813%. Each treatment store retained 363 messages,
passed integrity, and had the expected ATC row count (0 / 0 / 363). These are
diagnostic results with `qualification: false`. The earlier candidate live
result of 7.317% also exceeds the unchanged 5% budget. Separate invocations do
not establish a matched baseline/candidate performance win.

Next: inspect the measurement protocol and retained candidate evidence, resolve
the outstanding performance failure, then complete exact-source validation and
required reviews before publication and supervised installation. Historical
active-command statements below are superseded by completed evidence where
explicitly recorded. Missing suite instructions remain tracked under the
operator-approved deferral br-rpa0b.

LATEST: no compiler active. All-offcontrol10905 terminal0: identical off-mode
runs produced73.64% apparent overhead, proving local sequential-method false
positives. Interleavedprototype35037 terminal0:300samples/mode,363messages/store,
all integrity/ATCcounts pass. Nullcontrol differences0.905/0.793%; actualshadow
4.840%,live7.317% (live stillfails5%). Externalprototype only; no gatechanged.
Baseline sensitivity run now active via sameprototype script/frozenoriginal am;
root/log atc-interleaved-baseline-20261023. Collect handle before heavywork.
Rust candidate implementation audit2026-10-22_23-07-37-319-3870bba5a51e approved
4requiredcode/architecture units,zero findings for8d5f6635..27c1efd0. Correctness
evidence only. Fullcleanbranchreview stillpending.
Authorized cargo clean --release19432 succeeded at2026-10-23T01:11:39+0200:
8470files/3.7GiBremoved,8.3GiBfree. Onlyobsolete targetrelease; rollback/frozen
binaries/evidence/live data and targetdebug preserved. Original benchmark script
copied to evidence-root/bench_atc_transport-before-interleave-20261023.py.

CURRENT: conformance36804 passed83/83 in32.05s under HTTP_PORT=1 and
PROJECT_IDENTITY_MODE=git-common-dir. Initial68369 passed82/83, revealing the
identity fixture leak after endpoint/catalog repairs; explicit dir-mode fix
committed6a03d2b87614237b4b92702850230dc23631dcb4. fmt/diff90851 passed.
No Rust compilation active. All-off negative-control10905 running:9fixed runs,
same frozen candidate binary, identical off behavior in all positions. Script
and log at evidence-root/atc-null-control-20261023{.py,.log}; results in same
basename directory. Diagnostic only; collect same handle, don't start compiler
until it exits. Designed to test false positives of sequential mode comparisons.
Counterbalanced1424 completed0 with18runs/allintegrity checks pass; baseline
shadow overhead1.28/23.84/36.42%, live25.19/18.01/-10.18%; candidate shadow
22.03/-19.89/-41.68%,live18.04/47.33/6.79%. No gate pass claimed or limits changed.
Evidence atc-counterbalanced-20261023/round-*/result.json. Timings alone cannot
attribute these swings; direct shadow sample shows11ATC-observer samples vs
1768archive-write samples (inclusive/overlapping). Preserve schema, authority,
collision and durability protections while investigating.
Required implementation auditor wrapper_manifest_review now checks exact Rust
candidate8d5f6635..27c1efd0 for correctness; collect returned projection. Shell
advisory review read both changed files withzero findings but failed scope due
dirty unrelated files; reusable clean-tree review still required. Full goal
remains CIrepairs, exact qualification, forkmerge, supervisedcutover,24hwatch.

Current checkpoints: wrapper8d5f6635, ATC27c1efd03d28773f1cc5a7ea7df487ad929cebcd,
conformance29971c31ce676e25f2a50a927f4129d1e6d2d496. Normal hooks, separate concerns.
Conformance68369 still compiling at latest live poll; two jobs, disk7.7GiB free.
Runtime qualification and matched gate status unchanged. wrapper_changes_review
runner includes dirty files even with explicit committed endpoints; its shell
review is advisory and unrelated paths must be marked unread. After this note
checkpoint, request clean-tree review f15ee021..HEAD for all new repairs.
Do not present optional/skipped implementation audit as substantive review.

Wrapper implementation audit2026-10-22_22-50-26-610-7ed13bb8dba4 returned approved
but both shell paths were optional/skipped coverage gaps: this supplies no
substantive shell review. Required changes-reviewer wrapper_changes_review now
reading exact f15ee021...8d5f6635 through review-changes runner. Collect its token
and supported projection before claiming independently reviewed wrapper repair.

LATEST recovery checkpoint: wrapper fix committed8d5f6635424ab57a495e145a4d2cfcc1b9f0abbb.
Metadata regression38182 failed both preservation assertions before correction;
green32887 passed14 checks. Evidence wrapper-metadata-{red,green}-20261023.log.
Safe local metadata-reset defect fixed: append only absent wrapper file entries
after original validation, preserving all original metadata/hashes. Syntax/diff
pass. Required wrapper_manifest_review now auditing exact f15ee021...8d5f6635;
initial attempt supplied no range and made no audit claim. Collect review result.
Conformance68369 remains active (about12min compilation, two rustc jobs); no
duplicate build. Disk7.9GiB free at latest check. Other four Rust files and this
note remain dirty; no push/deployment. Continue same handle, then separate
conformance and ATC checkpoints/reviews. ATC matched gate remains failing;
full qualification, merge, supervised cutover and24h observation remain required.

Recovery continuation: authorization file and checkpoint reread. Delivery remains
active; no operator question pending. Session68369 still compiling conformance;
all-target/production/strict Clippy/fmt/diff checks passed. Wrapper final regression
49669 passed12 assertions, but inspection found full manifest regeneration resets
delegate metadata via fresh e2e_lib counters/timestamps. Imperfection: test_helpers.sh
wrapper finalization; safe local fix under artifact contract/Python author skills.
Add preservation regression (38182 red run), then append only completed wrapper
entries to validated original manifest. Preserve all original metadata and hashes.
No deployment or publication has occurred.

LATEST: active68369 full validation. All-target PASS67s,production PASS45.57s;
strictClippy running,then fmt/diff,then full conformance --test it with inherited
HTTP_HOST127.0.0.1/HTTP_PORT1. Logs atc-direct-{all-targets,production,clippy} and
conformance-repair, all suffix20261023. Two jobs; collect before new heavy work.
Candidate46629 terminal0:3append+4insert+1corrected schema tests pass,devbuild4m19s.
Frozen am at evidence-root/atc-direct-candidate-20261023/am (source.patch retained),
SHA256ff897c0c87cf67c79e7511f8154fa1ce37f1ead9ab9662541c5c0ae338c760a1.
Matched95649 terminal3: both gatesfail; baseline off/shadow/livep95=
35.830708/37.118542/54.477750ms (live52.04%); candidate32.297459/38.703708/
33.725792ms (live4.42%passes,shadow19.84%fails). Both integrityok; evidence
atc-direct-matched-20261023. Retain direct candidate; full ATCgate NOT passed.
Shadow does not write sidecar or build durable rows; inspect actual shadow cost
before further optimization. No retry-until-green or threshold relaxation.

Native CI39856238367 finishedfailure: conformance81pass2fail; artifact validation
4/9invalid. Full failed-step log ci-native-111ccfd8-failed-20261023.log.
Conformance repairs now uncommitted: setup_fixture_env explicitly supplies
Config::default HTTP host/port, insulating captured-default fixtures from CI's
sentinelport1; error catalog adds authorized INVALID_DISPLAY_NAME with rationale.
Validation is the full conformance suite under sentinel environment in68369.
Artifact failure is live wrapper/stderr_capture.txt changing after delegate
manifest: http4983->5281bytes, tui3163->3889. Governed by e2e artifact contract;
safe local repair active. No shell edits yet. Next add regression to existing
tests/e2e/test_artifacts_schema.sh using actual wrapper_exec + delegate that
writes manifest then late stderr. Fix tests/e2e/test_helpers.sh by capturing
stderr outside bundle until delegate exit, validating original delegate bundle
before adding completed wrapper records, then re-sealing/validating. Preserve
corruption detection: don't blindly rehash changed original artifacts. Include
negative tampered-original test and preserve delegate nonzero exit.

Diagnostics commits f15ee021 remain reviewed but unpushed. Native CI is now
terminal, so cancellation concern ended. Fresh origin/main fetch remainsancestor.
spx worktree status shows upstream-integration noholder (same br-rpa0b exception).
Push preflight skill requires clean tree; finish/checkpoint current authorized
repairs then review exact scope before publication. No permission question pending.

CURRENT2026-10-23: direct ATC candidate in queries.rs/server lib.rs,uncommitted.
Public append_atc_experience_if_initialized returns None for memory/incomplete
schema, otherwise revalidates authority/schema and uses one canonical connection
for inspection, transaction/row ceiling, post-COMMIT stored-row readback and close.
Async append uses it too; server observation bypasses per-row worker/runtime
only when ready. Missing schema retains existing migration worker; errors do
not retry. No durability/cache/schema-check relaxation. Candidate not qualified.
Baseline10206 passed3append+4insert tests; corrected sidecar-schema3668 passed1.
New synchronous duplicate assertion compares complete stored row inside active
runtime. Active46629 sequentially runs3append,4insert,1schema tests then builds
development am. Logs atc-direct-candidate-{append,insert,schema,build}-20261023.log.
Two jobs, no incremental. Collect before further heavy work; runtime source frozen.
Next preserve candidate development binary/source diff and run SAME invocation
baseline/candidate real ATC gate, both modes/results retained even when failing.
Use frozen baseline evidence-root/atc-symbolized-profile-20261023/am; its runtime
source matches111ccfd8. Candidate debug should match CI profile; release comparisons
remain separate. Full gates, review, release/runtime/browser evidence still required.

Diagnostics90253 completed0(devbuild6m7s);15488 completed0(563messages/ATC rows,
integrityok). Symbolized profile at atc-symbolized-profile-20261023/sample.txt:
370ATCthreads,1118thread samples; inclusive open_canonical_atc_conn521,
schema inspection281,append797,readback298. Inclusive values overlap.
Stacks confirm connection PRAGMA/schema parsing and runtime/timer-wheel creation.
Sampling/debug timings are diagnostic only. Sidecar test fixture now seeds/checks
atc_sidecar_sqlite_path and asserts primary remains absent; regression passed.

Diagnostics headf15ee021cc71d9b87349cf0705eb4220aabc1ec1 committed,independent
review2026-10-22_22-18-25-333-66b858505676 approved4/4 zero findings. Not pushed
yet while native CI39856238367 remains in_progress; avoid cancelling its evidence.

Active90253 builds baseline development am for symbolized diagnosis, two jobs,
no incremental, existing target; log atc-symbolized-baseline-build-20261023.log.
Runtime source equals111ccfd8. Collect before new heavy work. Preparing the
workflow diagnostics changes for independent review/publication so Linux probe
artifacts become available; performance budgets/durability remain unchanged.
Native CI39856238367 still in_progress at fresh check, dashboard success.

Diagnostic12592 terminal0: frozen release baseline,500measured sends plus63seed/
warmup,563messages and563ATC rows,full integrityok. External profile wrapper
enables debug logs and15s macOS sample; timings are diagnostic only. Evidence
at evidence-root/atc-release-profile-20261023/{sample.txt,diagnostic.json,
store/server.log}. Append debug563observations median4744us,p957471us.
Sample has395atc-ledger threads/1021thread samples; selected syscall samples
open134,close96,pwrite160,fsync67,fcntl22. Release symbols stripped, so no
Rust-function-level attribution. Do not infer a dominant individual subcost.
Next inspect lifecycle/schema checks with symbolized diagnostic evidence before
another production candidate; no duplicate heavy command is active.

MATCHED RESULT77457 terminal3: baseline and candidate both fail original5%gate.
Baseline off/shadow/live p95=31.443916/33.734583/32.402542ms; shadow7.28%fails.
Candidate off/shadow/live p95=30.368084/30.781375/37.274458ms; live22.74%fails.
Both transport commands succeeded and canary quick_check=ok with1ATC row.
Connection-reuse experiment REVERTED manually; queries.rs now byte-identical
to HEAD (git diff --exit-code passes). Candidate source.patch/binaries/results
remain preserved. No win claimed, no retries to obtain a passing sample.
No heavy command remains active. target/release still contains the rejected
candidate: use frozen release-111ccfd8-binaries for baseline, never install
target/release as current source. Continue diagnosing synchronous ATC costs
and filesystem latency; workflow repairs remain uncommitted, service unchanged.

LATEST: build54394 terminal0; both release executables finished16m33s. Frozen
at evidence-root/qualification/release-atc-reuse-20261023-binaries, together
with source.patch, base-commit.txt and SHA256SUMS. am0.3.39 hash
505547cac8dc228401e5a24b8771522f22284f931062d15f39e38e4c70abf6cd;
server09159220ae2c66fd6cf54dc855f2615fa27cd7fbcca005da7143e29fe509cbd1.
Active77457 runs matched real ATC gate baseline then candidate sequentially,
using frozen release binaries and separate isolated stores. Both run even if
baseline fails. Evidence-root/atc-reuse-matched-20261023 holds logs, results,
stores and exit codes. Each has600s process-group deadline; original5%limit.
No compiler overlaps the comparison. Collect77457 before new heavy work.

Recovery 2026-10-23: operator acceptance file reread; full delivery remains
active with no pending question. Build54394 still active; collect same handle.
GitHub authentication succeeds. New active imperfection: APFS CI job120095063098
fails single p95=69.844ms/35ms and batch p95=417.516ms/300ms at head111ccfd8.
Evidence: evidence-root/fsync-apfs-111ccfd8-failed-20261023.log, assertion at
crates/mcp-agent-mail-storage/tests/fsync_matrix.rs:404. Classification: safe
local diagnosis/repair under CI inspection and Rust performance workflow.
Retain budgets and durability; earlier local APFS pass does not close this.
Native Build & Test remains pending; other recorded CI results unchanged.

## Current resume checkpoint — active, 2026-10-22

Current54394 has passed all-target59.31s, production36.28s, strictClippy72s,
formatting and whitespace; BOTH release executables are now compiling. Keep
runtime source fixed until collection. New workflow-only repairs also exist:
Archive Perf has a separate60min locked build (logs retained),30min measurement,
95min job, two compiler jobs, pinned repository compiler and cache-on-failure.
Bench commands/budgets remain. SIGTERM143 cause is still unqualified; this
prepares a bounded diagnosable rerun, not a claimed CI pass. Linux fsync now
preserves one probe binary as a7day artifact for local diagnosis without the
failed Linux VM compilation. Actionlint and whitespace pass. Toolchain comment
now describes the workspace requirement without the stale FastMCP version.

Candidate97083 completed0: append3/3 and insert4/4 pass (0.06s/0.15s),
including full-row round-trip property coverage. Active54394 runs sequential
workspace all-target check, production lib/bins check, strictClippy, fmt/whitespace,
then builds BOTH release binaries. Two jobs, no incremental, physical TMPDIR,
compiler library path restored. Logs atc-reuse-{all-targets,production,clippy,
release}-20261022.log under evidence-root. Collect the same handle; later logs
exist only after earlier gates pass. Source remains the uncommitted DB helper/
read-back change plus this note. No performance pass, review, push or deployment.

ATC read-back candidate now exists in DB queries.rs, uncommitted. It extracts
the existing canonical SELECT into a connection-borrowing helper and performs
read-back after successful COMMIT on the insert connection, before closing it.
Schema inspection, row-cap enforcement, transaction/rollback and stored-row
decoding remain. Baseline61174 passed3append tests after12m35s compilation;
baseline82202 passed4insert tests including full-row property coverage.
Candidate97083 now runs those same groups sequentially. Formatting91346 and
whitespace pass. Collect tests before another heavy job; no performance result
or full compiler/Clippy result is claimed for this candidate yet.
Use frozen release111ccfd8 for the matched comparison, since its exact source
identity is known. Source parity for the preserved debug executable has not been
established; do not treat it as an exact-head comparison. Build the candidate release
after focused tests, compare both in one invocation, then full gates/review.

Latest continuation: reviewed/pushed head111ccfd8 remains the source baseline.
Release build38320 completed successfully in18m28s. Both executables are frozen
under evidence-root/qualification/release-111ccfd8-binaries; hashes and current
process handles are in evidence-root/continuation-111ccfd8-20261022.txt.
These are baseline binaries; installation remains unqualified. The live service
is unchanged. Full delivery, fork merge, cutover and24h observation remain active.

Current CI passes dashboard WASM, both installers, observability, btrfs,
tmpfs and ext4-journal. ATC fails10.06%overhead against5%; ext4-ordered and
xfs batchp95 fail387.395/404.659ms against250ms. Archive Perf terminated143
during cold compilation, without a benchmark verdict. Its termination actor
is unqualified. Native tests and APFS remain running. These are active repairs,
not deferred findings. Exact logs/artifacts are in the external checkpoint.

ATC append diagnostic:113successful durable observations, median2906us in
the synchronous append path. It includes schema inspection, connection setup,
write and read-back; no individual subcost attribution. Diagnostic overlapped
compilation and supplies no performance qualification. Source inspection found
an avoidable read-back connection reopen. Before editing, focused unchanged
DB append tests compile in session61174, twojobs/single test thread; collect
atc-append-baseline-20261022.log before another heavy command. No ATC source
change yet. Next run preservation tests, make the bounded connection change,
then matched real-transport comparison with the unchanged budget.

FINAL LOCAL GATES85181 terminal0: workspace all-target47.85s,production28.83s,
strictClippy56.86s,formatting and whitespace PASS. No active heavy command.
Repairs checkpointed by concern with normal hooks: CLI fixture de6f142a,
storage fixture c4ee59fe,storage performance4a63da7a,benchmark repairb46eeb2a.
Next required independent review spans remote PR head9271c52f through the
new clean local head, including the two earlier workflow fixes. Full native
CI/storage suite, Linux filesystem gates, final-binary runtime/browser proof,
fork merge, supervised cutover and24h observation remain. Earlier candidate
and check-active paragraphs below are historical. No publication or live change.

DIGEST COMPARISON61630 terminal0: same-invocation APFS baseline and candidate
both pass original35/300ms budgets. Batch p50/p95 baseline252.531/284.936ms,
candidate242.350/270.019ms. Single p95 baseline22.915ms,candidate30.553ms.
Both crash/reopen probes retain100messages/two commits. Evidence and binary
hashes: fsync-digest-batch-matched-20261022/{baseline,candidate}. This supports
a modest batch improvement on this run; no general speedup or Linux pass.
Digest safety77024 PASS6/6,0.69s; integration probe build21.92s. Candidate
message regressions14/14 also pass. Required workspace all-target,production,
strictClippy and formatting now run sequentially; collect the current session
before another heavy job. Logs digest-final-{all-targets,production,clippy}
with -20261022.log. Full storage suite and independent review remain required;
then publish reviewed repairs and obtain fresh CI. No service change.

DIGEST BATCH candidate is now uncommitted in storage/lib.rs. Batch writes group
rendered thread entries by sanitized digest filename, retain per-file input
order and first header spelling, and append once per digest under the existing
project lock/fence. Single-message formatting, path checks and best-effort
digest publication remain. Git filtering and synchronization are unchanged.
Existing digest baseline6/6 and layout baseline1/1 passed before edits.
Candidate write_message_ group14/14 passes0.82s, including expanded complete
archive comparison with interleaved threads and case-colliding filenames.
Session77024 runs final digest safety group then rebuilds the integration
probe; collect before measurement. Logs digest-batch-{safety,probe-build} and
digest-batch-messages with -20261022.log. No performance claim yet. Compare
preserved storage-before-tree-cache-20261022 against rebuilt probe sequentially
in one invocation on fresh APFS stores, unchanged35/300ms budgets.

RECOVERY UPDATE: session19579 completed. Matched baseline batch p95 was
305.682ms; tree-cache candidate312.981ms, both above300ms. Single p95 was
24.444/20.084ms; both crash probes retained100messages/two commits. The cache
production change was reverted; its patch is preserved in evidence-root/
tree-cache-experiment-20261022.patch. The real Git safety test remains as
tree_updates_preserve_bases_deletions_and_update_order. Current compiled probe
still contains the rejected experiment until rebuilt; do not qualify it as
current source. Earlier candidate-active paragraphs below are historical.
The full delivery goal remains active, with no pending operator choice.
Inspection confirms flush_sync requests immediate worker readiness and polls
completion every5ms; no evidence currently attributes the130ms flush duration
to an intentional coalescing delay. Continue diagnosis of actual commit work.
Missing /data/projects/AGENTS.md remains covered by operator-deferred br-rpa0b.
Current-source tree regression17719 terminal0:1passed,0failed,0.02s after
34.32s compile; log tree-semantics-after-revert-20261022.log. Whitespace check
passes. No active heavy process remains. GitHub access succeeds; PR3 is OPEN
at9271c52f with mergeStateStatus UNSTABLE. No new source commit or deployment.
Profile inspection narrows the next investigation: sample.txt lines1161–1255
show one worker's86active samples in blob_path, including attribute/filter
lookup, canonicalization, object creation and object timestamp refresh. These
sample counts are diagnostic, not elapsed-time attribution or qualification.
git2 blob_path preserves path-dependent Git filters; substituting repo.blob
would alter that behavior. Preserve filters, deletions and path safety in any
candidate optimization. The earlier subtree-cache experiment gave no win.

TREE CACHE candidate implemented, uncommitted: build_tree_with_updates now uses
a per-call cache keyed by immutable base treeOID and exact ordered relative
updates. Reuses identical canonical/inbox/outbox subtrees; preserves distinct
bases, last-write order and deletion semantics. No sync/path guard changed.
Real Git regression PASS1/1; lockfree group3/3; message-write group14/14.
Session66825 terminal0 (unit compile35.61s, probe compile23.85s). Logs
tree-cache-{safety,lockfree,messages,probe-build}-20261022.log.
Pre-change probe preserved at evidence-root/storage-before-tree-cache-20261022,
SHA256462f1cb930c78139de77ee4ac850a641177459130d7c20b0bbdff63c45d0c0f2.
Session19579 runs same-invocation APFS baseline then candidate, independent
stores, original35/300ms budgets, each process-group deadline180s. Collect it
before another heavy run. Evidence fsync-tree-cache-matched-20261022/{baseline,
candidate}/{probe.log,summary.json}; no passing performance claim yet.

LATEST checks79486 terminal0: all-target18.15s,production11.92s,strictClippy18.13s,
formatting/whitespace PASS. No heavy session remains. ATC gate has real-path local
pass and56E2E assertions; final head still needs independent review and CI.
Next bounded repair: storage batchp95 remains311.836/300ms. The profile shows
repeated identical Git subtree construction across canonical/inbox/outbox.
Evaluate a per-commit cache keyed by base treeOID plus exact ordered updates;
preserve distinct bases, update/deletion order, all sync and path guards. Add
real Git preservation tests and rerun the unchanged APFS/filesystem budgets.
No cache edit exists yet. Keep source candidate stable per measured run; retain
all current uncommitted CLI/ATC/storage repairs and do not rebuild the live app.

ATC/benchmark final E2E61533 terminal0:56assertionspass,0fail,0skip,artifact
validation passes; artifacts tests/artifacts/bench/20261022_205644 and log
bench-e2e-final-20261022.log. New real-server negative case disables ATC behind
the driver, completes sends, then correctly rejects live rowcount0. This proves
successful fast requests cannot substitute for the durable ATC path.
Bench unit26002 PASS26/26. Repaired general catalog full run succeeds with12
supported commands; four dead entries removed after actual CLI parser refusal.
Full E2E37556 had53pass/1fail solely because saved-baseline comparison returned
valid regression3 (help p95 7.22→8.12ms). The test now accepts0or3 only and checks
exit agrees with regression field; existing forced-regression test and5%ATC
budget preserved. No pass gained by changing a performance limit.
Required all-target/production/Clippy/fmt checks now run sequentially; collect
the active session from the turn before another heavy command. Logs
atc-transport-{all-targets,production,clippy}-20261022.log. Python Ruff/format/
strict mypy and shellcheck/syntax pass. No remote publication or live change.
Failed historical artifact bundle remains intact at bench/20261022_204441:
command guard rejected relocation into the dot-state evidence directory; no
move/deletion occurred. Its old malformed-input record is retained failure
evidence; new runs use raw.stdout.txt and valid adjacent rejection JSON.

ATC transport repair underway: new scripts/bench_atc_transport.py runs real
persistent MCP stdio per mode, identical60seed+3warmup+50measured sends,
isolated HOME/config/DB/archive, full integrity and exact113message/ATC-row
checks after EOF. Live must persist113ATC rows; off/shadow0. Protocol/exit/
persistence failures fail measurement. The shell gate invokes this driver;
historical CLI process timings are explicitly incomparable,5%budget unchanged.
First real gate43797 PASS: p95 off46.441459/shadow35.847750/live35.215959ms;
three stores113messages/integrityok,live113ATC; separatecanary1ATCrow/quick_checkok.
Evidence atc-transport-gate-20261022; no speedup claim from noisy negativeoverhead.
Later helper edits add binarySHA provenance and reap peers even on brokenpipe.
Ruff/mypy pass; shellcheck subshell-binary defect corrected without suppression.

Full benchmark E2E73242 terminal1:50assertionspass,1full-run casefail; all new
protocol/durability assertions passed. Artifact validation also rejected the
deliberately malformed JSONL fixture: raw protocol fixture now has.stdout.txt
extension with separate valid JSON rejection verdict. No evidence deleted.
The full-run failures: lint/typecheck had no Cargo project; mail count is not a
CLI command; inbox inherited server discovery. Direct CLI also rejects the
three conditional stub-encode catalog entries. Repair in bench.rs removes four
unsupported entries and validates every remaining entry against actual Clap;
seeded runs pin HTTP127.0.0.1:1, independently proven by successful inbox run in
bench-inbox-local-valid-20261022.json. E2E now creates a small real Cargo fixture.
No performance threshold or >=10 benchmark assertion changed. Source uncommitted.
Session26002 runs bench::tests:: (twojobs/singlethread); log
bench-catalog-repair-20261022.log. Collect, then full E2E rerun, compiler checks
and exact-head review. Prior failed E2E artifacts bench/20261022_204441 preserved.
No service change, publication or observation start. Storage batch stillfails.

Session92431 terminal0: workspace all-target and production lib/bins checks,
strict Clippy, formatting and whitespace checks all PASS after the CLI fixture
repairs. No heavy process remains from this continuation. CLI repairs remain
uncommitted alongside the earlier storage/ATC work. Next complete real-transport
ATC gate correction and storage batch-performance repair, then fresh review/CI
and final-binary qualification. All deployment/24h obligations remain open.

LATEST: focused guard28697 PASSED1/1 after5m57s compile. Parent identity stress
run66573 guard group PASSED4/4 in2.17s with AGENT_NAME=ScarletGoose. Snapshot
group initially3pass/1fail: products_inbox fixture omitted the delivered nullable
sender_display_name and empty to_display_names fields. Added only those four
JSON entries after checking the response contract; no golden regeneration.
Final snapshot89823 PASSED4/4 in6.85s, including both shell normalization and
negative cases preserving status, path and unsupported shell/details.
Logs guard-suite-host-identity,cli-snapshots-portable,cli-snapshots-label-fields
with -20261022.log under evidence-root; retain the failed intermediate evidence.
Session92431 now runs sequential all-target,production,strictClippy,fmt/diff
checks (cli-portability-{all-targets,production,clippy}-20261022.log). Collect
before another heavy run. No commit, push, merge or installation change.
Archive cancelled job log unavailable: gh reports log not found; attempt2 API
download returns empty ZIP (archive-9271-attempt2-logs-20261022.zip). The45min
deadline annotation is established, but compilation/runtime cause is unqualified.
Its step metadata marks gate success at13min and skips artifact/verdict stages;
do not infer a benchmark pass. Fresh CI must supply actual evidence.

Recovery reconciled the operator goal with delivery: upstream inspection is
complete separately; PR3 CI repair, exact-binary qualification, fork merge,
supervised upgrade and full24h observation remain active. Authentication verified
as simonheimlicher; PR3 remains OPEN at9271c52f. No operator question pending.
Prior session87116 completed successfully: workspace all-target51.09s,
production30.26s, strict Clippy1m03s and debug am build4m59s. No live change.

Native CI39836909829 ended failure: CLI integration274pass/2fail/6ignored.
Snapshot assumed zsh where Linux used bash; no-identity guard fixture inherited
the host environment. Both repairs are now uncommitted: normalize only supported
successful binary-resolution shell suffixes, preserve path/status; env_clear
plus isolated HOME/config for the guard child. Session28697 runs focused guard
test with two compiler jobs and physical TMPDIR; log guard-hermetic-20261022.log.
Collect before another heavy command. Required full checks/review still pending.
Archive Perf39836909647 attempt2 cancelled at45min job limit; no verdict.

ATC real persistent-stdio diagnostic completed: off p95 31.621333ms,
shadow32.848ms, live33.128292ms (live overhead4.77%). Independent stores each
retain114messages and integrityok; live sidecar retains113ATC observations,
off/shadow0. Evidence atc-real-stdio-_zx9vzww/results.json. This is diagnostic,
not a CI gate pass. Existing CLI benchmark bypasses server ATC observation and
runs modes on a growing shared store; replace gate timing with real transport
and equivalent isolated stores before relying on its result. ATC policy tests
now27pass (atc-sidecar-regression-p6c0tw5q/regression.log), shellcheck passes.
The actual ATC database is sibling atc.sqlite3; canary now checks that sidecar.
Storage batch remains311.836/300ms FAIL; no threshold waiver or deployment.

ACTIVE HEAVY SESSION87116: sequential workspace all-target check, production
lib/bins check, strict Clippy, then current debug am build. Two jobs/no
incremental/physical TMPDIR. Logs storage-group-{all-targets,production,clippy}
and atc-current-am-build with -20261022.log. Collect before any other heavy run.
No pending operator question. Native CI39836909829 and Archive Perf attempt2
39836909647 remained in_progress at last read. Reinspect before publication.

APFS candidate gate4400 terminal101: single p95 32.895/35ms PASS, batch p95
311.836/300ms FAIL. Crash probe retained100canonical messages and2commits.
Evidence fsync-group-apfs-20261022/{summary,environment}.json and sibling.log.
Batch write112–164ms, flush123–166ms. Do not claim threshold pass or retry until
green. Diagnostic sampled repetition60075 also terminal101; numbers under
sampling are NOT qualification. fsync-profile-qt3qhhre/sample.txt shows path
canonicalization/metadata and Git build_tree_with_updates/ODB writes; fsync
is only one contributor. Preserve all durability and symlink checks when
choosing the next bounded optimization. No further production edit yet.
Next run workspace correctness checks and build current debug am for genuine
ATC protocol/path qualification; old binary handshake probes cannot establish
current-head ATC persistence. Grouped-sync experiment remains uncommitted and
performance-unqualified. No release/deployment/observation started.

Storage full retry63198 terminal101:696passed,3failed,1ignored,990.26s.
Terminal details confirm both filename fixtures failed at write with APFS
errno92 and alive-PID fixture failed its no-removal assertion at zero expiry.
All three repaired focused tests now PASS1/1 each (0.01/0.02/0.05s), logs
storage-ref-portable,storage-detect-portable,storage-alive-pid-portable with
-20261022.log. Staged safety rerun PASS6/6 (0.04s), storage-staged-final log.
Session25705 now compiles integration probe --test it --no-run, two jobs.
Collect it, then run existing APFS fsync-matrix script with workspace target and
physical TMPDIR; budgets35ms single/300ms batch unchanged. No performance result
yet. Full suite has not been rerun after fixture-only repairs; preserve exact
aggregate/focused distinction. All workspace checks and required review remain.

ATC final focused policy/protocol lane PASS24 assertions, session30328 terminal0,
evidence atc-policy-final-fek0jmnj/regression.log. Added whole-gate failure probe
using a rejecting stdio process: passing synthetic timings cannot hide protocol
failure; exit1/runtime_error/protocol_failed all checked. Existing 5% regression
and advisory policies remain unchanged. Source/validator run in separate shell
so its globals cannot change the caller. shellcheck -x -P SCRIPTDIR passes.
Storage retry63198 is in final wbq tests; await terminal diagnostic/result.
nextest unavailable (cargo reports no such command); standard Cargo validation
continues. Do not install another build tool or start duplicate heavy jobs.

Third storage fixture repair staged in lib.rs: fresh alive-owner test passed
max_age=0, which explicitly expires unknown-start-time owners on macOS in
try_clean_stale_git_lock. It now uses ARCHIVE_INDEX_SYNC_STALE_LOCK_SECS, keeping
the same no-removal/existing-file assertions. No production expiry behavior
changed. Await original suite terminal details and run all three repaired tests
before full current-source validation. Current run63198 continues on old binary.

ATC response regressions added to existing tests/e2e/test_bench.sh. Gate entry
is now main() guarded by BASH_SOURCE so tests source its actual validator.
Six protocol cases PASS; existing relative-overhead/advisory/zero-row cases also
PASS (21 assertions total) via the unchanged case blocks, evidence
atc-policy-regression-i95h7c5n/regression.log. shellcheck -x -P SCRIPTDIR and
bash -n pass for both shell files. Full bench E2E/current binary still required.
These tests do not validate actual ATC latency; original failure remains open.

Raw filename probe invalid-ref-name-t7whj65j confirmed APFS errno92 for bad-ff
bytes. Repaired both failed recovery fixtures to use raw packed-refs content,
retaining refusal/no-publication/byte-preservation assertions on all platforms.
Edits in recovery.rs and recovery/ref_backup.rs are uncompiled. Current full
storage run uses the earlier binary and additionally reports
stale_lock_cleanup_respects_alive_pid FAILED; collect terminal panic before
diagnosis. Classification: active Rust validation repair, no deferral or pass.

Latest active storage retry session63198 (two jobs/single test thread). It has
reported failures in incomplete_ref_name_snapshot_is_never_published and
detection_refuses_non_utf8_names_instead_of_inventing_a_prunable_name. Await
terminal diagnostics; both fixtures write invalid UTF-8 filenames on APFS, but
the exact failure is not yet collected. No fixture edit has been made.

ATC canary defect confirmed in captured CI JSONL: requests2–5 all rejected with
notifications/initialized required, while status.json claimed completed.
scripts/bench_atc_perf_gate.sh is now DIRTY: sends initialized notification,
validates exactly one successful response for ids1–5, clears inherited learning
disable for the isolated canary, and propagates canary protocol failure to the
gate verdict. bash -n and shellcheck pass. Captured failure now produces
protocol_failed (rechecked_canary_status.json). Isolated real canary with older
review-a91eead5 binary passes requests and quick_check, but has no ATC table:
atc-handshake-h3cckra2 and atc-handshake-explicit-_c9_sh24. This establishes only
protocol repair, not current-head durable ATC/performance qualification. Need
fresh binary after current heavy job, focused regression coverage and review.
The current candidate has no target/debug/am. Do not install older proof binary.
Archive CI attempt2 log endpoint returned404 BlobNotFound while run still says
in_progress; saved response archive-ci-9271-attempt2-20261022.log is no test
verdict. Collect terminal run before deciding repair. No new push or cutover.

Recovery continuation: authentication succeeds as simonheimlicher. Original
delivery goal remains active; upstream inspection is separately complete.
Storage run29656 was mistakenly interrupted after a buffered log suggested no
progress. Sample storage-group-hang-sample-20261022.txt actually shows the later
io_metrics_nonzero_after_archive_write test in bounded flush_sync; terminal101
records SIGINT during test_find_commit_for_path. No hang or fixture failure is
established. No fixture changed. Full suite restarted, log
storage-group-full-retry-20261022.log, followed by probe compilation. Collect
before further heavy commands. Preserve interrupted evidence; do not count pass.
ATC39836909629 failed actual live overhead11.10% against5% (off235.647ms,
shadow242.234ms,live261.806ms). Investigation: bench.rs seeds one shared mailbox,
runs modes sequentially; local send calls messaging::send_message directly,
whereas ATC observation recording lives in server post-processing. Establish
actual path coverage and equivalent workloads before any benchmark repair.
Artifacts at atc-9271-20261022/atc-perf-gate; no budget waiver authorized.

Grouped staging focused tests PASS6/6, log staged-group-pipe-20261022.log,
session29720 terminal0, tests0.06s after rebuild. First run25287 exited101 with
5pass/1fail because macOS fsync(/dev/null) succeeds (F_FULLFSYNC fails later),
so it did not inject the intended group-barrier failure. Changed both pending
sync fixtures to real pipe descriptors; final-remainder case fails after one
regular file synced. Existing Apple full-device failure fixture remains /dev/null.
Full storage library tests followed by integration-probe compilation now running;
logs storage-group-full-20261022.log and storage-group-probe-build-20261022.log.
No performance claim or publication yet. Collect this active run before builds.

Review setup inspected2026-10-22: origin/main branch-protection endpoint returns
404 Branch not protected; repository Actions secret-name list is empty; no
review workflow or CODEOWNERS path found under.github; PR reviews/comments empty.
Local independent review evidence exists but no current-head CI review exists.
Finish the authorized CI/source/qualification work before the merge decision;
do not infer merge approval from absent branch protection or silently waive the
required review. No credentials were printed, configured or copied.

Latest CI infrastructure repair: APFS39836909679 was cancelled at its35min job
deadline while still compiling fsqlite, no benchmark verdict. Log
fsync-apfs-9271-20261022.log and GitHub check annotation confirm timeout. The
matrix script selected a temporary target outside rust-cache. Commit
72854661699b029041a9e7a398f799790fbae00b makes both platforms reuse workspace
target, gives no-run compilation60min separately from30min probe and95min job,
and saves failure caches. Actionlint/whitespace pass; no latency budget change.
Not pushed. Local focused storage tests25287 remain active in dependency compile;
do not start another heavy build. No candidate validation pass claimed.

Latest continuation at PR head9271c52f: all five repair commits were pushed after
review token2026-10-22_19-00-01-925-78aeab621d1e rendered approved,11files,0findings.
Dashboard default/exporter tests and Clippy pass in CI; WASM failed because the
action installed its target for stable while root rust-toolchain selected nightly.
Local commit4975bb7128efbb90cf347075502ef14e1445d4f7 sets job RUSTUP_TOOLCHAIN=stable;
actionlint/whitespace pass. Not pushed yet. Native CI39836909829 is still testing.
Fsync39836909679: tmpfs/btrfs pass; batch p95 failures ext4-ordered322.605/250ms,
ext4-journal399.018/350ms, xfs739.598/250ms. Singles pass. APFS still running.
Archive Perf39836909647 attempt1 runner shut down during compilation (exit143),
no benchmark verdict; rerun attempt2 requested with debug logging. No OOM claim.

Storage experiment is UNCOMMITTED in src/lib.rs: stage at most64 open files,
sync each group and the final remainder before publication, retain Apple final
device flush. Three new real-filesystem cases cover bounds/content and both
pending/group sync failures; existing Apple device-failure test still exercises
its later barrier. No measured improvement or passing candidate claim yet.
Focused tests run in session25287, two jobs/no incremental, configured single
test thread; log staged-group-candidate-20261022.log. Collect before another
heavy command. Existing unchanged baseline safety3/3pass is recorded in
group-sync-safety-20261019.log; git diff3de..HEAD over storage was empty before
this experiment. Baseline rebuild62963 was stopped130 after confirming that
preserved evidence. Release build16921 was deliberately stopped130 because the
new storage repair invalidates that candidate; dependencies preserved. No active
release build remains, and no final binaries are qualified for installation.

Qualification runtime_workload.py now points to installed adapter0.16.2 because
cache0.16.0 disappeared; syntax/project-key/Ruff/strict mypy pass. This operational
repair is outside the repository in evidence-root/qualification. Next complete
focused and durability tests, compare filesystem measurements, full compiler
checks and required review; publish only validated repairs, then finish CI,
runtime/browser qualification, fork merge, supervised upgrade and full24h window.
No service restart, upgrade or observation has occurred. Detailed continuation
facts and CI logs are under evidence-root/recovery-continuation-20261022.txt.

FINAL LOCAL CHECKS PASS: session74347 terminal0. Workspace all-target4m11s,
production lib/bin2m41s, strict Clippy1m24s, root/dashboard formatting and
diff whitespace all passed. CLI3021/0/7ignored; dashboard default146/0,
exporter165+9/0; WASM release1m00s passed. Both modified CI workflows pass
actionlint. Checkpoint the bounded repairs, then required independent review
before PR update. ATC cold-build workflow repair has local lint only; actual
CI rerun and all performance verdicts remain required. No heavy job active.

FULL CLI SERIAL PASS: session1650 terminal0;3021passed,0failed,7ignored,
579.23s tests after3m12s compilation. Log cli-full-serial-20261022.log. All ten
failures of the incorrect two-test-thread invocation pass under the configured
single test thread. No recovery production changes were made for those failures.
Next: standalone dashboard default/exporter/lint/WASM matrix on toolchain1.99.0,
two compiler jobs with debug info/incremental disabled; then workspace checks
and required review. Filesystem CI performance and remaining CI jobs still need
qualification. No merge, cutover, or24h observation has happened.

Dashboard default PASS:145passed/0failed,8.92s; session52130 terminal0, log
dashboard-default-green-20261022.log. First compiling run exposed two stale
test assumptions (dashboard-default-20261022.log): seeded cursor stays at the
start; slash already opens public search. Colon fixture now sends End before
typing; encoded Ctrl+P runner test starts on dashboard without slash. Existing
model test retains Ctrl+P non-escape from dashboard text entry. No input routing
production change. Exporter matrix now running, collect before another build;
log dashboard-exporter-20261022.log. Root formatting and whitespace passed.

Exporter test run passed164library+9command tests. Strict Clippy then exposed
unused shared agent renderer and native-only theme test lock in the browser.
Fixed by using the width-aware label renderer for browser Agents/ATC rows and
matching the lock's cfg to the native-only tests. Added duplicate Reviewer rows
with BlueLake/RedStone address assertions. Final exporter rerun passed165+9;
strict Clippy terminal0 (1.89s), logs dashboard-exporter-green-20261022.log and
dashboard-exporter-clippy-green-20261022.log. No lint suppression or skipped tests.
WASM release build active next, log dashboard-wasm-release-20261022.log; collect
before more heavy commands. Shared theme cfg edit needs fresh native checks.
WASM release now PASSED terminal0 in1m00s (session72377). Final default dashboard
tests followed sequentially by required workspace all-target, production lib/bin,
strict Clippy and fmt checks are running next. Logs dashboard-default-final,
ci-repair-all-targets,ci-repair-production,ci-repair-clippy with -20261022.log
under evidence-root. Two compiler jobs/no incremental/debug info disabled.
Current validation session74347; default final146passed/0failed,9.01s. Collect
remaining workspace checks; no duplicate build. ATC cancellation diagnosed:
run39280462698 hit its30min job limit while compiling fsqlite-vdbe0.3.18;
bench.log in ci-atc-artifacts-3de-20261022 contains no benchmark verdict.
Workflow now gives cold build a separate60min step, benchmark retains30min,
overall95min permits cleanup; explicit prebuilt am and cache-on-failure enabled.
Actionlint passes. This does not establish an ATC benchmark pass; rerun CI.

Recovery continuation: all ten original CLI failures have focused passing
reruns. Inbox21passed; Beads23passed plus br_json_command3passed; shared-mailbox
group19passed/2ignored after required-column failure injection; doctor group
3passed after correcting the live-owner probe expectation to require refusal.
Doctor evidence: doctor-probe-green-20261022.log, terminal0, compile1m46s,
test0.51s. Production recovery admission remains unchanged. Full CLI suite now
runs the compiled test binary with two test threads and verified br on PATH;
log cli-full-repaired-20261022.log. Collect terminal status before next heavy job.
Full run has now reported doctor_restore_uses_sidecar_safe_restore_path FAILED
(log line2519); detailed panic remains buffered until completion. Reproduce and
repair this failure next, before dashboard validation. All ten original CI
failures still have focused passes; this additional failure prevents a suite
pass. Classification: active safe-local diagnosis/repair under Rust test workflow.
Correction: that run used --test-threads=2, overriding .cargo/config.toml:84
RUST_TEST_THREADS=1. Terminal result3011passed/10failed/7ignored,679.31s.
Nine failures were process-global recovery-admission contention; the tenth an
inbox-open deadline. Restore isolated rerun passed1/1 in1.30s. Do not modify
recovery admission for this invalid invocation. Rerun through Cargo using its
configured single test thread, two compiler jobs, verified br on PATH. Preserve
the failed parallel-run evidence; it is not the repository's CI configuration.
Correct serial Cargo rerun is active in process session1650; log
cli-full-serial-20261022.log. Collect it before another heavy command. It rebuilds
the shared server formatter and CLI, then uses Cargo's RUST_TEST_THREADS=1.
If any of the ten parallel-only failures remain, diagnose the actual serial
failure; do not assume every failure was caused by test concurrency.
Serial rerun compilation completed3m12s;3028tests are executing, session1650
still live. Stable toolchain1.99.0 now has wasm32-unknown-unknown installed;
use it for the standalone dashboard matrix after serial tests finish, with
two jobs, incremental off and debug info disabled to bound disk use.

Dashboard compile repairs are uncommitted and unqualified: portable MailScreen
includes claims_key; search-limit test import restored; unused test trait import
removed; shared label-width rendering formats its already-computed address
directly, removing the native core-model dependency. Native/browser validation,
required review and CI still remain. No merge, deployment or observation started.

Recovery artifact check: /tmp/agent-mail-integration-publication.ZPzght and its
upgrade-checklist.md are absent. The durable operator plan and qualification
observer remain available. Reconstruct the cutover checklist from the approved
plan with freshly qualified binary identities before deployment; never reuse
the superseded staged binary identity. This is an authorized local preparation
repair, owned by the integration delivery workflow.

Replacement cutover checklist (durable replacement for missing scratch file):
1. Finish all CI repairs, required checks/reviews at the exact candidate head,
   runtime and browser qualification; record both binary SHA256 values and
   package an immutable release directory. Do not use earlier staged binaries.
2. Merge fork PR3 under the approved review workflow. Preserve source, current
   installation target and rollback binaries; verify candidate source identity.
3. Notify coordinating positions, gracefully stop launchd com.agent-mail and
   confirm doctor drain permits mutation. No live-owner override or forced kill.
4. Snapshot the stopped mailbox family (main/WAL/SHM and engine sidecars), archive,
   supervisor plist and current release identity together. Verify backup hashes,
   integrity and continuity before switching the current symlink.
5. Install the qualified pair, restart the supervisor, confirm health, then send,
   read and acknowledge fresh mail. Verify labels, routing and existing records.
6. Start qualification/observe_service.py only after successful cutover; retain
   a full24h of integrity, request-failure and resource observations. Preserve
   every post-cutover message if rollback becomes necessary.
7. Remove only authorized task build artifacts after qualification/installation;
   preserve source, evidence, rollback and Time Machine snapshots. Update tracking
   and this note with terminal outcomes before declaring the delivery complete.

Observation preparation repair fixed: qualification/observe_service.py now
requires --service-target and uses launchctl print for its supervised PID;
processes() no longer calls lsof. Syntax and real read-only discovery passed:
gui/502/com.agent-mail returned PID62353,381472KiB RSS,4.1% CPU. This session
has not restarted/replaced the service; discover current ownership again before
cutover. The full24h observer has not been started. Python implementation
workflow governs this operational helper. Ruff and strict mypy both passed.

Latest results: focused inbox session45292 exited0,21passed including both
CI failures,4.65s test execution. Beads session59291 exited0,23passed,6.66s,
using the verified br0.6.0 on PATH. The separate br_json_command filter is
running next; log beads-command-20261022.log. Full CI remains unqualified.

Goal reactivated. Prior native build30069 has no process handle and PID38061
is absent; its log ends during compilation without a success verdict. Do not
claim that build passed. No replacement service has been installed.
CI at3de21b33 is terminal. Native CLI:3022passed,10failed,9ignored.
Failure groups: six Beads fixtures cannot launch br; two direct inbox fixtures
lack agents.display_name; one shared-mailbox scoped-query error assertion;
one malformed-file doctor-probe expectation. Dashboard has five compile/lint
errors (missing claims_key trait member, missing core models formatter in its
portable facade, missing search-limit test import, unused MailScreen import).
Archive Perf terminated143 after runner shutdown, so produced no verdict.
Filesystem ext4-ordered/ext4-journal/xfs/apfs failed; btrfs/tmpfs passed.
ATC cancelled. Installer jobs and observability passed; downstream native
jobs skipped after native test failure. Preserve all these obligations.

First repair in progress: add nullable display_name to the two direct-inbox
fixture schemas, keeping assertions unchanged. Focused check_inbox_ tests
session45292, two jobs/no incremental under standing operator load override.
Log evidence-root/check-inbox-fixtures-20261022.log. No concurrent heavy job.
Failed-job logs: ci-native-3de-failed-20261022.log and
ci-archive-3de-failed-20261022.log. Next collect focused tests, then fix the
next evidenced failure; full checks/review follow before publication.

Beads fixture prerequisite prepared without another compile: official v0.6.0
darwin_arm64 release and .sha256 downloaded into evidence-root/
beads-0.6.0-fixture-tools; shasum check passed, extracted br reports0.6.0.
Use that directory on PATH for subsequent Beads regression runs. No system
binary was installed. CI still needs the equivalent pinned Linux prerequisite.
The CI prerequisite is now patched in .github/workflows/ci.yml: official
v0.6.0 linux_amd64 asset, pinned SHA256, extract only br into RUNNER_TEMP and
publish its directory through GITHUB_PATH before workspace tests. actionlint
passed. The six real-command tests remain enabled. Runtime validation with
the prepared native br follows completion of test build45292.
Filesystem log ci-fsync-3de-failed-20261022.log confirms batch p95 misses:
ext4-ordered598.766/250ms, ext4-journal485.296/350ms, xfs323.410/250ms;
APFS single67.121/35ms and batch363.481/300ms. These are failed gates,
not qualified results. Preserve budgets while diagnosing the actual costs.

## Prior paused checkpoint

This section supersedes historical process/status statements below.
The operator requested preparation to resume; the goal remains paused.
Active checkout: /Users/shz/Code/silvarbor/agent-mail/upstream-integration.
Branch work/upstream-integration-20261015, head3de21b33f44901583068a08bf9a69f5a1ba4a292.
Only this resume note has uncommitted changes. Installed service is unchanged.

- Existing native runtime build session30069 is still active, cargo PID38061,
  with two compiler jobs. Log: evidence-root/build-storage-qualified-runtime-20261019.log.
  Collect it; start no duplicate or competing heavy local command.
- Linux container attempts26192 and73552 both ended101 with OOMKilled=true.
  No task build container remains running. Do not repeat that VM build without
  establishing sufficient memory. Other running containers belong to other work.
- PR3 at the exact head above remains open. Fresh checks: Archive Perf Gate and
  Dashboard WASM failed; both signed installer jobs, observability drift and
  tmpfs passed. ATC, other filesystem probes and Build & Test are in progress;
  Skip Guard queued. Formal reviews/comments remain empty.
- First resume action: collect failed-job logs and classify their actual causes,
  while collecting the existing native build. Fix one evidenced failure at a time.
  Keep this candidate stable until a specific failure requires a source change.
- Preserve build artifacts until the candidate is qualified and staged. Then
  complete required CI/review, fork merge, supervised backup/upgrade/smoke,
  full24h observation and authorized cleanup. None of these is complete.
- Live lsof containment below remains applied. Director mail17069 delivered.
  No waiter is intentionally left suspended; PID58747 was resumed after config.

Report checkpoints as completed outcomes, current failing requirement, and next
action. Introduce no unrelated cleanup or refactor into this delivery.

## Earlier evidence and history

2026-10-19 live lsof containment verified: operator requested stopping the
scans. A five-second process-parent sample found lsof children of short-lived
am CLI processes launched by wait_event.py PID58747, rather than children of
HTTP server PID99745. Suspended only that waiter, configured HTTP_HOST=127.0.0.1
and HTTP_PORT=8765 in ~/.config/mcp-agent-mail/config.env, then resumed it.
Installed CLI source local_server_url discovers ownership when neither endpoint
setting exists; explicit settings avoid this discovery. Ten seconds after
resume: seven am children observed, zero lsof processes. Independent adapter
inbox probe succeeded (exit0). No server restart or binary replacement.
Backup config-before-explicit-endpoint-20261019.env is mode0600 under evidence-root;
the live config is also mode0600. Preserve this credential-bearing backup and
never publish its contents. Evidence: lsof-configured-endpoint-20261019.json and
lsof-configured-inbox-20261019.json. br-nwdr9 remains the general unconfigured
discovery-cost issue; do not claim a universal source fix from configuration.

PR repair head3de21b33 is pushed and independently reviewed (19/19 files, no
findings); GitHub watch session11074 remains active with checks queued.
Linux two-job build26192 failed with Docker OOMKilled=true while compiling
asupersync. Cache preserved in agent-mail-fsync-build-cache:20261019; one-job
retry session73552/container agent-mail-fsync-serial-20261019 is active.
Log: linux-fsync-build-serial-20261019.log. Loop-device preflight passed for
ext4/xfs/btrfs. Continue collecting these handles, then remaining CI repairs,
fork merge, qualified upgrade, full24h observation and cleanup.

FINAL-SOURCE CHECKS COMPLETE: session 18414 exited zero. Workspace all-target
check passed (49.26s), production lib/bins passed (28.83s), strict workspace
Clippy passed (57.14s), and formatting passed. Combined evidence:
evidence-root/group-sync-final-checks-20261019.log. No build remains active.
Fourteen message-write regressions and three staged-write failure tests pass;
matched APFS candidate gate passes with batch p95 261.344ms. Linux ext4/xfs
qualification is still pending. Checkpoint this note and obtain independent
review of the committed repair range before pushing the fork PR update.

LOCAL CHECKPOINTS created with normal hooks on 2026-10-19:
01d968f761dcb7db27eae2d1117c2110c9793aa0 — APFS probe mode/table correction;
6cf0a3b1b89479e2324404be5be66b0e2cc7b173 — staged Apple device flush batching,
single preparation pass and two failure regressions;
044b2173453f1bce5c1e6df9ffd747bbbcf9094d — br-nwdr9 polling cost tracking.
All are local, unpublished. Remaining tracked change is this resume note.
Session 18414 final-source checks: all-target PASS 49.26s; production lib/bins
PASS 28.83s; strict Clippy/format still running at this checkpoint. Collect
terminal result, commit updated evidence note, then use required changes
reviewer on clean exact range c48d05fa3ae24182a410a267e5c35c070c10c289..HEAD.
APFS performance passes, Linux qualification remains open. No claim that all
PR CI failures are repaired yet; no merge or installation change occurred.

LATEST APFS PASS: session 55973 finished (14 message-write tests pass), and
matched probe session 14884 finished: baseline batch p95 1886.281ms versus
candidate 261.344ms, below unchanged 300ms; single 29.547 -> 24.704ms, below
35ms. Both crash probes retained 100 messages/two commits. Evidence under
evidence-root/fsync-single-preflight-matched-20261019/{baseline,candidate}.
Final-source full checks running in session 18414: workspace all-target,
production lib/bins, strict Clippy, fmt. Log group-sync-final-checks-20261019.log.
Collect terminal result before another heavy task. Earlier failure reports
below are historical. Linux ext4/xfs remain unqualified; no publication or
upgrade yet. Prepare separate commits for storage performance, probe metadata,
polling-debt tracking and resume evidence, then required independent review.

LATEST: session 25203 finished successfully; strict workspace Clippy passed
in 1m26s after the passing all-target check. Then removed duplicate batch
preflight helper/pass: append_message_bundle_files still validates collisions
while staging, and publication follows successful preparation of all entries.
Preparation-error logging now reports zero published files. Formatting and
diff whitespace pass. Session 55973 runs all 14 write_message_ regressions,
then rebuilds the integration benchmark. Log:
evidence-root/group-sync-single-preflight-20261019.log. Collect it next, then
run a fresh baseline/candidate comparison before claiming further improvement.
The prior full checks precede this final small optimization and need relevant
revalidation before publication. No commit, push, merge or upgrade occurred.

2026-10-19 current validation: session 25203 runs sequential all-target check
then strict workspace Clippy. All-target check PASSED in 2m56s. Clippy has
reached the CLI/conformance/application crates; collect its terminal result.
Logs: evidence-root/group-sync-all-targets-20261019.log and
group-sync-clippy-20261019.log. Source held fixed during these checks.
Next bounded optimization: batch currently computes paths/renders JSON and
checks each collision twice (reject_message_bundle_archive_collision_from_index
then append_message_bundle_files). Staged publication now prevents visible
message files until all entries succeed, so evaluate removing duplicate work
while preserving the collision-before-publication test and temp cleanup.
Do not remove checks without rerunning the existing rejection tests and a
fresh matched performance comparison. Linux budgets remain unqualified.

2026-10-19 Director report queued as br-nwdr9, P1, verified-important /
peer-measured: remove per-request whole-host lsof scans and bound inbox polling
cost. Original report preserved at evidence-root/director-lsof-finding-20261019.md.
Measure installed vs candidate first: pool.rs already has a non-Linux
subprocess-free mailbox_owner_executable_deleted path, and robot_commands.rs
has cheap recovery-lock detection. Queue acceptance includes zero healthy-path
lsof spawns, owner/recovery safety tests, and ids-after-cursor contract review.
Revisit during integration qualification. Client standing waits are paused.

Session 92365 COMPLETE: all 14 write_message_ tests passed; production
cargo check --workspace --lib --bins passed in 5m08s. No build remains active
from that session. Remaining workspace all-target/Clippy checks still pending.

ACTIVE STORAGE REPAIR (2026-10-19). Full operator objective remains repair
PR #3, merge fork, supervised local upgrade, 24-hour observation and cleanup.
Current HEAD 0453bfaf; nine local commits unpublished. The live service is
unchanged. Work is in upstream-integration, not the original checkout.

LATEST RESULT: sessions 40382 and 59825 completed. Three staged-write failure
tests pass; candidate probe compiled. Same-invocation baseline/candidate APFS
comparison: single p95 32.683 -> 20.089ms; batch p95 1780.775 -> 526.342ms,
batch median 1694.754 -> 277.825ms. Both crash probes retained 100 messages
and two Git commits. Both batch gates FAIL the unchanged 300ms budget.
Evidence: evidence-root/fsync-matched-group-sync-20261019/{baseline,candidate}/summary.json.
Continue profiling remaining write/Git costs and Linux failures; this candidate
is a measured improvement, not qualified for merge or installation.

Nextest is unavailable (`cargo nextest --version`: no such command). Use
bounded focused cargo-test commands for now; do not claim a full test run.
A sequential command now runs storage --lib write_message_ with physical
TMPDIR, then cargo check --workspace --lib --bins, two jobs/incremental off.
Logs: group-sync-message-regressions-20261019.log and
group-sync-production-check-20261019.log in the evidence root. Collect its
session handle from the conversation before launching another heavy command.

Uncommitted production change groups Apple device-cache flushes across staged
message files. Each file is fsynced; one retained descriptor per device gets
File::sync_data (F_FULLFSYNC) before publication. Message digests are appended
after publication using the original prepared path/timestamp. Other platforms
retain per-file sync_data. No latency budgets were changed. APFS metadata now
truthfully says full_flush. Linux performance failures still require repair.

Baseline message batch tests first failed at the macOS /var symlink guard.
Rerunning with physical TMPDIR passed five batch tests and the staged cleanup
test. Candidate also passes five batch tests. Added real staging-error and
Apple device-sync-error tests; validation session 40382 is running those,
then compiling storage --test it for the matched performance comparison.
Log: evidence-root/group-sync-safety-20261019.log. Collect that session.
The previous candidate compilation E0509 was corrected; second compile passed.

Preserved baseline probe: evidence-root/fsync-baseline-0453bfaf/storage-tests,
SHA256 0e4aa689c2d3f95004ceb722590860799f4f427ceadb1810925fe95ec7c4eec1.
Run this and the newly built candidate in the same invocation on fresh stores
with unchanged APFS single/batch budgets 35/300ms. Earlier APFS baseline was
36.909/1860.073ms; it is not a matched comparison. Crash check passed.
Schema validation session 33684 is complete: three passing tests.
Full workspace checks, focused safety tests, review and qualification of the
new runtime change remain before publication. Preserve all evidence and the
qualified older binaries; the newer source is not qualified for installation.

ACTIVE CI REPAIRS (2026-10-19). The new operator goal authorizes repairing
all PR #3 failures autonomously, merging the fork, upgrading locally and
observing for 24 hours. Earlier scope questions and blocked status are
superseded. The installed service remains unchanged.

Integration review a91eead5 and supplemental workflow review c48d05fa passed.
The 3600-second isolated workload passed 540 sends/540 read-ack cycles,
three cold integrity/FK/exact-continuity checks and two restarts. Browser
38 assertions and HTTP 102 assertions passed; matched CPU was comparable,
unchanged DOM/arrays were retained, and max refresh concurrency was one.
Full evidence and binary hashes are in
~/.local/state/mcp-agent-mail/integration/20261015-preservation/review-qualification-status.md.

Six new local commits end at 0b148f41249b99d5c68553299fdc782b6e8abe40:
installer history ERR handling, ATC runner temp path, APFS physical path,
dashboard lockfile package version, observability schemas/fixtures and
native CI cold-build deadline. No runtime Rust implementation changed.
Installer regressions: original source fails 6/52; repaired source passes
52/52 and all four full signed-release install modes. Observability drift
is zero with six checker unit tests and five negative-control E2E scenarios
passing. Dashboard locked metadata passes against CI's pinned FrankenTUI.
Actionlint, shell syntax and diff whitespace pass.

Active validation: exec session 33684 runs CARGO_BUILD_JOBS=2,
CARGO_INCREMENTAL=0 cargo test -p mcp-agent-mail-server --test it
observability_schemas::. Log: evidence-root/observability-rust-20261019.log.
Collect this process; do not start a duplicate. Build target was recreated
after the earlier authorized cleanup; clean it after validation. Disk was
12 GiB available during compilation. Minisign 0.12 was installed to run
the signed installer proof; scratch installs are isolated from the live store.

Remaining: collect schema test; review the committed CI repair scope;
diagnose/fix unchanged ext4/xfs batch-write latency budget failures; rerun
CI (archive perf's prior runner was shut down during compilation). Preserve
latency budgets, symlink protection and tests. Then complete fork merge,
supervised backup/upgrade/smoke and full 24-hour observation. PR #3 remains
at c48d05fa until the new commits are reviewed/pushed. Older sections below
are history; do not repeat completed reviews or qualification without a
relevant source change.

Filesystem diagnostic follow-up: downloaded ext4 artifact contained only
environment.json. The script passed a relative output path while Cargo runs
tests in the crate directory; canonicalizing OUTPUT_DIR fixes summary upload.
The probe now reports write and flush durations after capturing each unchanged
total sample. Formatting and whitespace checks pass; compiled validation is
pending after schema session 33684. Use the same target directory sequentially.

READY FOR SECOND REVIEW (2026-10-18). All repair source is committed through
84b7ff8a60b495c2e3955f6d81f89ded980e97de. Session 72503 completed exit 0:
13 session-identity integration tests, two registration regressions, strict
workspace Clippy (52.44s) and formatting pass. The final changes after the
passing workspace compilation checks affect test fixtures only. CLI priority
tests pass 3/3; shared-mailbox tool tests pass 3/3; compiled pending-worker
tests pass 17/17. The targeted schema, cursor, TUI receipt and cap-boundary
regressions also pass as recorded below. No active test/build process remains.

Next: commit this evidence note, then use the mandatory changes reviewer on
the clean exact range d036cf300e28dd199464fddc3f6446e99b07a52e..HEAD. This is the
second review after sealed rejection 2026-10-15_14-51-32-479-e93a537399f9.
The first review examined all 317 then-changed paths and found five blocking
items plus the duplicate-file debt; all six are addressed. No gate waiver is
claimed. Rebuild both runtime executables from the final committed revision,
then rerun applicable runtime/browser qualification before fork publication
and local upgrade. The approved 24-hour observation remains afterward.

Latest validation: session 7010 ended exit 101. All three project-mailbox tool
tests PASSED (2.65s), and 11 session-identity tests passed. Two eviction tests
stopped at the temporary-project guard. Added the explicit ephemeral-project
allowance inside their isolated fixture. Rerun 55826 passed 12/13; the second
eviction test's initial send returned DURABILITY_DEGRADED. Inspection found
its temporary archive could be dropped before queued writes completed.
Safe local fixture repair: ResetConfigCache now drains write-back and async
commits before the environment/tempdir can disappear, including unwinding.
The new registration unit fixture had the same lifetime hazard, so it now
uses the existing test_support::with_isolated_mailbox helper, which drains
and checks its own writes. No production durability guard was changed.

These last fixture changes are now committed and validated. All 13
session tests now PASS (10.11s, test-review-session-identity-final-3.log),
including both eviction cases with the archive drain in place. The same
sequential command also passed two registration regressions,
strict workspace Clippy, then formatting. Logs:
test-review-session-identity-final-3.log,
test-review-registration-admission-final.log, clippy-review-final-4.log,
fmt-review-final-2.log. Session 72503 is complete. Preserve earlier failure logs.

Local repair checkpoints created with ordinary hooks, no bypass:
- 6f8a7d600da86b32acd813f479f124e05fca61b9: registration admission and regressions.
- 895554caa61cd54b74a305c44da79f2d6e009350: shared delivery/schema authority and
  real TUI receipts (four files, priority edits excluded through a manually
  prepared index patch).
- a4d993001868071adaacfea107e7c029991e815e: approved duplicate worker removal,
  compiled independence coverage and canonical macOS fixture root.
- 8f4337435966aea91ade1ecdd40d1084ebc4723f: priority ordering before the server
  limit, optional MCP argument, robot request, native callers and regressions.
- 84b7ff8a60b495c2e3955f6d81f89ded980e97de: drain isolated identity-fixture
  archives and explicitly allow their ephemeral project paths. Current HEAD.
  All implementation and test changes are committed.

At the priority checkpoint only this evidence note remained unstaged; the
index was empty. The two fixture changes above are now committed. Commit payloads
and the manually edited shared-mail staging patch are preserved at
/tmp/agent-mail-repair-commits.qIoP4Q. CLI compilation passed in 6m32s, but the
initial filter used the filename rather than the extracted module name and
ran zero tests. Corrected direct invocation of that compiled binary with
robot::commands::tests::robot_inbox_server_ PASSED all three tests in 0.01s:
test-review-robot-inbox-exact.log. The zero-test run supplies no test evidence.
Session 7010 completed as recorded above. Do not dispatch the second review until
this evidence note is committed and deterministic checks pass.
No source files changed during this commit preparation; current compiler and
Clippy results still cover the source. Runtime binaries must be rebuilt after
the final checkpoint, before qualification or installation.

Final repair gates ALL PASSED at session 31869 exit 0: strict Clippy (1m10s,
clippy-review-final-3.log), workspace all-target compilation (1m17s,
check-review-final-all-targets-2.log), production-only compilation (51.41s,
check-review-final-production-2.log), and formatting (fmt-review-final.log).
Two earlier Clippy logs preserve the repaired findings: numeric separators,
original test annotations displaced by an insertion, and two empty-binding
assertions changed to equality assertions with diagnostic output. No production
behavior changed during lint repair.

Remaining: final focused CLI/tool regression checks, checkpoint and sealed
re-review, rebuild and applicable runtime/browser qualification, fork merge/
local upgrade, 24-hour observation and build cleanup. The goal is active; no
publication has occurred. A sequential test command now runs robot inbox CLI
tests, then the real project-mailbox tool tests, then session identity tool
tests; logs test-review-robot-inbox.log, test-review-project-mailbox-final.log,
test-review-session-identity-final.log. Collect session 7010 before another
heavy command; the later logs exist only if the earlier step succeeds.

Latest checkpoint (2026-10-18): the priority ordering API and robot request
are now wired end to end. `cargo check --workspace --all-targets` passed
(check-priority-api.log, 1m22s). The real MCP tool regression passed with
1,001 newer normal messages plus older direct/shared urgent messages: unread,
all-mail and overdue filters preserve priority before limiting; omitted
priority preserves newest-first; mark_read=false leaves both receipt stores
untouched. Evidence: test-review-priority-tool-it.log, 1 test, 1.12s. The first
test command targeted a nonexistent standalone project_mailbox target; the
correct aggregate target is `--test it`. No code issue remained from that
invocation error.

DB priority regression PASSED (one test, 1.43s), test-review-priority-db.log.
Both registration regressions PASSED (two tests, 0.86s),
test-review-registration-admission-2.log. They use a one-shot, thread-local,
test-only seam after the real lookup: deterministic competing registration
after absence was observed, and a transient lookup error. All subsequent
writes/token/session operations use the real production path. The initial
run stopped at the temporary-project path guard; the isolated fixture now
explicitly enables AM_ALLOW_EPHEMERAL_PROJECT_ROOTS. No production guard was
changed. Test-only line-count annotations were added after compilation began;
final checks must cover that exact source too.

Server pending-worker tests completed: 16 passed, including the replacement
two-direction lane independence test; one fixture failed at ensure_archive
because macOS /var is a symlink. Safe local repair: canonicalize the temporary
fixture root before deriving archive/database/project paths, preserving the
production symlink guard. The full group rerun PASSED all 17 tests in 5.70s,
test-review-replay-workers-2.log; session 80488 completed with exit 0.
Initial evidence is test-review-replay-workers.log. Formatting/whitespace
check session 86481 passed. Run final workspace gates and the second sealed
review before publication or upgrade. The live service remains unchanged.

The approved uncompiled duplicate removal and useful coverage comparison
remain recorded below; its replacement compiled worker test has now passed.
Earlier running-session notes below are historical unless named
in this checkpoint.

Priority repair, backend phase written: sync::InboxFetchOptions now has
priority_before: Option<i64>, default None in every existing constructor.
Direct and shared SQL apply the same CASE priority before LIMIT, and the
combined result uses that priority before final truncation. project_mailbox
owns priority_order_sql and inbox_priority. Added a real DB boundary regression
with 1000 newer normal messages and older urgent messages (with and without
ack requirements), for direct and shared delivery and unread/all/overdue
filters. This code is unverified and NOT yet wired through MCP or robot CLI:
queries.rs currently explicitly passes priority_before: None. Next: expose
optional priority ordering through fetch_inbox, route robot inbox to it, and
exercise real MCP candidate selection as the reviewer requested.

Server test session 96450 is still compiling the pre-priority backend snapshot.
Its success will cover the TUI receipt action, not the later DB ordering edits.
Formatting session 50961 pending collection. No overlapping heavy build has
been started. Identity race/failure regressions and full gates remain pending.

Latest verification: pooled schema regression session 79086 PASSED, one test
0.37s after 2m46s compile. Receipt-recipient helper regression also PASSED, one
test 0.06s, test-review-receipt-recipients.log. Added a real TUI action regression
using send_message for shared-only and mixed direct/shared mail, then actual
handle_action mark-read/acknowledge calls. It checks recipient counts, direct
precedence, independent shared receipt rows, and read timestamps retained on
acknowledgement. Test compilation is active as session 96450, log
/Users/shz/.local/state/mcp-agent-mail/integration/20261015-preservation/test-review-tui-shared-receipts.log.
Formatting session 71931 passed. Collect 96450 before another heavy command.
Priority ordering and identity race/failure regressions remain outstanding.

LATEST remediation checkpoint: cursor regression PASSED (one actual test),
test-review-shared-event-exact.log, 0.39s after 2m54s compile. Workspace all-target
check session 45115 PASSED in 1m24s for the first receipt/registration repair
version. A pooled regression was added while that check was running, so it is
not evidence for the newest test source. Formatting sessions 86448 and 61336
passed. No production-only check or strict Clippy has run on the repairs yet.

TUI receipts now resolve actual recipient IDs through a new shared-mailbox
helper, preflight all selected messages before writing, and use actual agent
names. A database helper regression covers shared-only recipients, direct
precedence, blocked viewers, unchanged peer receipts, and missing authority.
Actual TUI action tests remain needed. Pooled regression now checks product
inbox failure and bulk-read rollback when deliveries are renamed but receipts
remain, followed by successful read after schema restoration. Its test process
is active as session 79086; collect it before another heavy run.
Log: /Users/shz/.local/state/mcp-agent-mail/integration/20261015-preservation/test-review-pooled-schema.log.

Priority inbox repair remains unimplemented. Inspection confirms six ranking
buckets, so merely adding an urgent fetch window would leave ack-required and
read-unacked truncation defects. Prefer server-side priority ordering before
the 1000-row limit, with identical direct/shared ordering and real boundary
tests; fetch_inbox currently exposes no pagination or ordering option.

2026-10-18 remediation continues (uncommitted): identity registration now
propagates lookup errors other than NotFound and atomically creates explicit
names before any upsert when session identity is enabled and lookup was absent.
Concurrent creation and lookup-failure regressions are still required; this
change has not been compiled yet. The pooled schema regressions are also pending.
All review findings still require final verification; none is claimed closed.

DB session 96205 completed compilation in 2m45s but selected ZERO tests due
to the short exact filter. Corrected run is session 51006, log
/Users/shz/.local/state/mcp-agent-mail/integration/20261015-preservation/test-review-shared-event-exact.log.
Collect it before starting another heavy command. Formatting session 58141
passed; subsequent formatting after the identity change is session 13213.
Two-job host-load override remains authorized by the operator earlier in this
thread. No re-review, commit, publication, or service cutover has occurred.

2026-10-18 17:52Z remediation checkpoint: shared-schema guards now reuse the
catalog check in all three pooled fallbacks and the event reader. Event reads
also refuse retained shared events when both shared tables are absent. Added
an event-cursor regression that renames/restores authority without deleting
data. Pooled regressions and remaining review repairs are still pending.
DB test compilation is active in session 96205, log at
/Users/shz/.local/state/mcp-agent-mail/integration/20261015-preservation/test-review-shared-event.log.
Its filter was passed with --exact without the module prefix; after collecting
the compilation result, rerun using sync::tests::shared_event_cursor_rejects_missing_delivery_authority_until_restored.
Formatting session 58141 is also pending collection. Neither is a test pass.

Deletion authorization received verbatim: "Approve removing that duplicate
after coverage comparison (Recommended)". At 2026-10-18T17:52:49Z, apply_patch
Delete File removed crates/mcp-agent-mail-server/src/retention/pending/lane_workers.rs.
The six tests were compared with compiled pending.rs/supervision.rs tests;
the two-direction progress test was ported into pending.rs. Existing compiled
tests cover stop-before-join, duplicate-start exclusion, fresh cancellation,
restart/backoff, and joined shutdown. The replacement test passed on October 18.
The deleted source
remains recoverable from committed head d79eb636. No service mutation occurred.

CURRENT 2026-10-18: review completed and was recovered through the supported
journal projection. Run 2026-10-15_14-51-32-479-e93a537399f9 is sealed, rejected,
317/317 files examined, five blocking findings and one debt finding. Exact
reviewed head is d79eb63693d5b7dac37a095e07b5e208f6787cd1. No reviewer is live.
No fork PR exists for this branch. The live symlink still selects the October 13
release. The staged 0.3.39 release must be rebuilt after remediation.

Review remediation ledger (Rust remediation and merge review workflow):
- identity.rs:2795: registration confuses failed/absent lookup with atomic
  creation, allowing unowned session binding. Fixed; two real-path registration
  regressions pass (test-review-registration-admission-2.log).
- queries.rs:11318: three pooled shared-mailbox fallbacks suppress incomplete
  schema errors. Fixed; pooled rejection/rollback/restoration test passes.
- sync.rs:717: event reader can advance past unavailable shared deliveries.
  Fixed; cursor rejection/restoration regression passes.
- tui_screens/messages.rs:1860: operator receipts treat project addresses as
  agent names. Fixed; real shared-only/mixed TUI action regression passes.
- robot_commands.rs:19365: capped server candidates omit older higher-priority
  mail. Fixed through server-side priority ordering before the cap; DB and MCP
  cap-boundary regressions pass. Final CLI checks still pending.
- retention/pending/lane_workers.rs:1: uncompiled duplicate worker and tests.
  Fixed after explicit removal approval and coverage comparison. Replacement
  compiled worker independence regression passes. Full group rerun passes
  all 17 tests after the isolated macOS fixture path correction above.

This is the first completed review rejection. Repair the classes, add focused
regressions, run the required checks, checkpoint, then obtain the second review.
Do not publish or install the rejected version. Prior qualification remains
baseline evidence, not evidence for future repair code.

Local integration merge committed as3adb733859aa5d41e2e3b519cac08cc658ca3379.
No hook was installed; no bypass was used. A dynamic staging command was
blocked by the command guard; explicit literal paths succeeded instead.
Cached diff-check reports two space-only context lines in the imported
docs/evidence/release_smoke/2026-10-12_qbhs0-skip-per-commit-publication.patch.
Inspection confirms these are required unified-diff context markers in a
preserved evidence artifact,not source whitespace to rewrite. Beads graph has
zero dependency cycles. Only FIFO tracking and this resume remain for the
separate tracking commit,then the exact committed integration is reviewable.

CURRENT 2026-10-15: operator agreed to separate FIFO tracking and continue.
Created br-1tzh0 (P2,verified-important,verified-local,operator-deferred) in
the JSONL authority using the pinned br executable with --no-db. Its evidence,
revisit condition,and unchanged failing regression are recorded. No full stdio
pass is claimed. Proceed with integration checkpoint,final review,fork PR and
supervised upgrade under the already-approved eight-step plan. Historical
"pending FIFO decision" entries below are superseded by this approval.
The source diff still exactly matches the qualified patch except this resume;
no qualified implementation section drifted before the tracking update.

FINAL QUALIFICATION CHECKPOINT 2026-10-15 14:35Z. All test/build processes
completed; no active qualification sessions remain. Final combined build
session6565 passed5.96s,then stdio suite exited1 with17pass/1fail. Its sole
failure is immediate named-FIFO EOF on macOS,also17pass/1fail on installed
baseline. Malformed JSON now passes the original5s deadline. Scope choice
for separate tracking versus repair was asked asynchronously and remains
unanswered. Do not claim full stdio pass or silently waive publication gates.
Final binaries preserved as independent APFS clones in qualification/final-binaries:
am SHA25602f3cc7257c05c4cf4d0b722d8a269d36a2b1c36e37c145e631fc702f292f8ae
mcp-agent-mail SHA256685977cb1ce24eed18f43f69fa984c395e80fd5058504c358b73d47d22010892.
Tracked+staged source snapshot qualified-working-tree.patch SHA256
0a345577f056fa475d17e6fe25f1801546cc2450a07f2a7c012e2e9cf64b78e0;
git ls-files --others --exclude-standard returned empty. Ignored resume is separate.
Live release symlink and both installed hashes rechecked unchanged. No publication,
upgrade,or live mailbox repair occurred. Remaining: FIFO scope decision,final
integration review,commit/fork PR/merge,supervised upgrade,24-hour observation,
and generated-build cleanup after review needs end. The two-file narrow audit
could not run through the installed whole-changeset audit workflow; no approval
verdict is claimed. This does not replace the required final integration review.

14:31Z: HOUR WORKLOAD PASSED(session8651 exit0). 3600.020active seconds,
540sends,540read/acks,660doorbell+42monitor reads,two restarts/three clean exits.
All three cold full-integrity/FK checks PASS,exact message and receipt IDs
180/360/540,one reservation and durable checkpoint retained. Final evidence
qualification/workload-candidate-hour/result.json. The old qualified server
image ran all three phases; corrected server has separate HTTP/browser/stdio
passes documented below. Final combined executable build then isolated stdio
suite now running: build-qualification-final.log and stdio-corrected.log.
No live mailbox mutation,publication,or installation during qualification.

14:20Z: corrected server HTTP repeat PASSED102 assertions(session29047 exit0,
http-corrected.log). Corrected browser repeat PASSED38,zero skipped/failed
(session4933 exit0,browser-corrected.log). Browser harness reports SIGKILL on
cleanup; this suite proves its assertions,not graceful shutdown. The hour
workload's two supervised clean exits and cold checks supply that evidence.
Soak remains active nearing50minutes. No am rebuild yet.

14:17Z: server-only rebuild session73576 PASSED2m32s. Corrected server SHA256
685977cb1ce24eed18f43f69fa984c395e80fd5058504c358b73d47d22010892.
am hash remains da57fb90...,so soak adapter executable is unchanged.
New server ordinary-pipe controls PASSED3(session34006): EOF0/3.45s,
malformed parse-error exit1/4.26s,reused EOF0/3.25s. Evidence:
qualification/stdio-corrected-pipe-2/result.json. Initial probe passed the CLI
serve-stdio verb in MCP mode and correctly exited2; corrected invocation uses
the server's no-argument stdio entrypoint. Failed probe retained.
HTTP core repeat against rebuilt server ACTIVE session29047,
qualification/http-corrected.log. Rebuild actual am binary after soak ends,
then rerun the isolated stdio suite without loosening its deadlines. FIFO scope
question remains unanswered; do not infer permission to waive its failure.

14:11Z: SECOND checkpoint PASSED360 exact sends/read/acks,full integrity/FKs,
reservation/checkpoint continuity. Final phase already started at1792073425,
PID49057,using original qualified server image. Changed earlier build timing:
server-only rebuild now runs after final restart,so no subsequent soak server
launch can select the new image. am remains unchanged for adapter subprocesses.
Build log build-qualification-server.log; collect active session returned by
the tool before another build. Original server also preserved as independent
APFS clone qualification/runtime-binaries/mcp-agent-mail-frozen. Original am
and server hard links are beside it. Final rebuilt executable still requires
separate transport checks; do not attribute this hour to the rebuilt hash.

Additional pressure control: workload-candidate-hour/between-burst-health.json
is fully GREEN with zero pending commits and all critical verdicts green.
Minute samples alias the deliberate synchronized bursts; retain their red
pressure readings alongside this independent between-burst probe. Streamable
HTTP final count is17passed/0failed/0skipped. No new executables built yet.

13:57Z: workspace all-target,production,and strict Clippy gates for lifecycle
fix PASSED(session73981,1m00/39s/39s). HTTP core E2E PASSED102 assertions,
zero failures,one explicit focused-subsuites skip(session39568). Streamable
HTTP suite session86027 exit0; collect exact assertion count from its log.
Current tested executable SHA256 (before the stdio-only source fix rebuild):
am da57fb90b5160553f74713fbf06e4eca038446eb9c4074c7dbfd4d966596d651
mcp-agent-mail 2408b65510fc2a3e990fda59ebe7d2e6a081f15e16ab70f168e46df956bd18c4.
Guard falsely classified a hash-manifest command's filename as Redis SHUTDOWN;
used read-only shasum and recorded its output here instead. No Redis command ran.
Browser comparative results consolidated in qualification/browser-comparison-summary.json.

LATEST 13:53Z: first20-minute phase PASSED full canonical integrity/FKs and
exact180send/read/ack IDs,one active reservation,durable checkpoint. Phase2 is
running. Untraced180s failure comparison session44851 exit0: baseline CPU25.01%,
RSS319.7→258.5MiB,heap47.9→26.8MiB; candidate CPU24.12%,RSS320.0→262.4MiB,
heap47.5→27.0MiB. Both maxActive1,5requests,DOM grows by exactly1 error element.
This resolves the apparent trace-RSS growth for this bounded window; it does
not establish a long-term memory-leak absence. Compiler/production/Clippy gates
for the stdio shutdown fix ACTIVE session73981,logs check-qualification-all-targets,
check-qualification-production,clippy-qualification. Two jobs authorized.
No executable rebuild until workload session8651 finishes its three phases.
Implementation audit role could not express two-edits-only scope: worktree:HEAD
requires all modified/untracked paths. No audit verdict was obtained; full
integration review remains required before publication,as already planned.

QUALIFICATION UPDATE 2026-10-15 13:49Z:
Hour workload session8651 remains active (started about13:30Z); do not duplicate
or rebuild its executable paths before all three phases finish. At least162
successful sends and acknowledgements; all observed critical health checks pass.
Matched browser session74144 completed exit0. Each version/scenario uses1000
synthetic messages,50 rendered rows,5s stress refresh and60s process measurement.
Baseline/candidate CPU: unchanged23.63/21.30%,changing29.68/30.04%,failures22.28/24.51%.
These are comparable,not evidence of a general speed improvement. All scenarios
have maxActive1,stable routing addresses and working navigation; unchanged/error
responses preserve array and body identity. DOM remains bounded; heap decreases.
Trace RSS grew during failure scenarios. Untraced180s failures comparison ACTIVE
session44851,outputs qualification/memory-baseline-2 and memory-candidate.
First memory-baseline attempt used a wrong executable path and never started;
corrected to installed release bin/mcp-agent-mail, preserving failed fixture.
Stdio suite first attempt was stopped after discovering missing STORAGE_ROOT
isolation. Live owner refused access; startup had read the live archive into the
isolated DB. Fixed tests/e2e/test_stdio.sh to export its own archive root.
Candidate rerun stdio-candidate-2 reports malformed JSON correct response but
missed5s full-process deadline,plus immediate FIFO EOF15s timeout. Baseline suite
session19054 exit1:17pass/1fail, same FIFO EOF failure. Fresh ordinary pipes pass
for both releases; reused candidate store pipes pass4–6s. FIFO probe4/4 hangs.
Native Python OS probe confirms macOS poll(POLLIN) returns no readiness on empty
closed FIFO while direct read returns EOF. Evidence qualification/fifo-os-probe,
stdio-fifo-probe,stdio-pipe-probe,stdio-reused-probe. User scope question pending:
track FIFO transport separately or repair here. Do not silently waive it.
Found stdio shutdown hook omitted retention worker shutdown. Safe local repair
adds retention::shutdown() in shutdown_runtime_services; fmt and diff checks
pass, compile/smoke checks still required. HTTP shutdown already calls it, so
running HTTP evidence is before this stdio-only change. Do not claim exact
final-binary qualification until subsequent checks are recorded. Bounded audit
agent integration_shutdown_audit was asked to inspect these two fixes.

UPSTREAM INTEGRATION ACTIVE, 2026-10-15. The operator approved all eight steps
in the attached plan: preserve source; review upstream; isolated integration;
compiler and regression checks; a 60-minute isolated reliability workload;
1,000-message UI comparison; fork PR/merge and supervised local upgrade; then
24 hours of live observation. This authorization supersedes the older cleanup
publication limits recorded below. Do not mark the goal complete before the
observation window and its evidence are complete.

Preservation evidence: ~/.local/state/mcp-agent-mail/integration/20261015-preservation.
source.bundle contains complete verified Git history. working-files.tar.gz
preserves all 21 changed/untracked files with SHA-256 verification. Separate
local checkpoint commits preserve engine repairs, retention, and build storage.
The original upstream comparison was 408 upstream-only and 41 fork-only commits;
the new preservation commits increase the fork side. Selected upstream tip was
92fd8025; resolve its full ID from the saved inventory before integration.
Live binaries still match the October 13 rollback release recorded below.
Fresh health: one orphaned page, leaked_pages_only, all rows readable;
reservation parity 13/13. Investigate during the authorized reliability work
before choosing a live repair. No service restart or database repair performed.
Checkpoint complete: d036cf300e28dd199464fddc3f6446e99b07a52e, with separate
engine 3996e7fc, retention 4f9fcd0c and build dc78f19e commits. Original checkout
is clean. Integration worktree: /Users/shz/Code/silvarbor/agent-mail/upstream-integration,
branch work/upstream-integration-20261015. An uncommitted merge of exact upstream
92fd8025bcfff616b1b517e4f480d2b47458b1dc is ACTIVE there. Do not restart it.
Six of 15 conflicting files resolved: Cargo.toml, DB id_floor/schema, tools
macros/messaging, storage agent_reconcile. Preserve the runtime engine repair
pin; adopt upstream FastMCP 0.13.0, comrak/detection versions and version0.3.39.
The automatically merged identity helper lost display_name forwarding; fixed
both helper parameter and caller. Both migration sets and recorded IDs retained.
Remaining conflict: CLI lib.rs only. The other fourteen paths are resolved.
CLI robot inbox preserves labels/recipient metadata and upstream ack_overdue.
Web sender labels use upstream page_rows; unified search retains labels and
upstream full-body search. Cargo update --workspace succeeded after an offline
cache miss; all 20 runtime engine crates retain the repair pin while FastMCP
uses registry0.13.0. README/AGENTS describe that selection.
Beads merge preserves 2808 records including all23 fork-only IDs. Field-level
three-way reconciliation found only67 timestamp conflicts and one notes
conflict (br-2hpuk); both complete notes retained. Report: beads-merge.json.
Operator approved: "I Approve mechanical extraction". Applied the bounded CLI
extraction using preserved Git stages and git merge-file. Evidence directory:
cli-extraction-approved under the integration evidence root. Test merge had
zero conflicts; all1066 upstream and1028 fork function names survive (1067
combined). Upstream benchmark block was byte-identical to base; preserved the
fork's extracted bench module. Retained632 lines of new upstream quiesce helpers.
CLI lib.rs reduced from134489 conflicted lines to62016 before formatting.
Rustfmt and whitespace checks passed; lib.rs/tests.rs staged. ZERO unresolved
Git conflicts remain. Merge is still uncommitted pending full validation.
Workspace all-target check PASSED,session19221 exit0,5m09s,
check-workspace-all-targets.log. One platform-only dead-code warning for the
Linux Systemd supervisor on macOS was corrected with the same conditional
allow pattern used by the adjacent Launchd variant; next gates verify it.
Workspace formatting initially found wrapping differences in upstream files;
cargo fmt corrected them. fmt-workspace-2.log exit0 and git diff --check pass.
Production graph check PASSED,session78808 exit0,2m24s,without warnings,
check-workspace-production.log. Platform-only warning fix is verified.
Strict Clippy first run failed on pool_impl redundant_pub_crate diagnostics,
non-Linux fallible-signature lint and a non-octal permission literal.
Investigation: pool_registry publicly glob-reexports pool_impl; making the
helpers pub widens the public API. Retained pub(crate), documenting a scoped
redundant_pub_crate allowance on pool_impl. This permits intentional internal
visibility only; all other strict warnings remain enforced. Non-Linux helper
retains the Linux Result signature with a documented function-only allowance.
Permission literal is 0o0. Also moved misplaced cache documentation to its
actual function and shortened the fixture doc summary. Intermediate failures
are preserved in clippy-workspace-2.log and -3.log; no API widening remains.
Clippy run4 verified DB fixes,then found ATC Result handling style; replaced
if-let/false with is_ok_and, preserving poisoned-lock behavior. Run5 found
three server test lints: fixed chunks_exact with as_chunks, extension matching
with Path::extension,and duration subtraction with checked_sub. Run6 PASSED
all workspace targets with -D warnings,41.53s,session3973 exit0,
clippy-workspace-6.log. Final formatting check PASSED,fmt-workspace-3.log.
Nextest is unavailable on this host; use the timeout-supervised Cargo fallback.
CLI retention tests PASSED2,0.81s after23m54 build,session92312 exit0,
test-cli-retention.log. CLI test binary:
target/debug/build/mcp-agent-mail-cli/b469bf04399f87ff/out/mcp_agent_mail_cli-b469bf04399f87ff.
Extracted reset_lock_quiesce tests PASSED3 (test-cli-quiesce.log); plan grouping
PASSED1; registration display_name parsing PASSED1. All use the same binary.
User explicitly requested runtime/browser qualification next. Production binary
build active session8534,two jobs,timeout5400s,build-runtime.log; builds both am
and mcp-agent-mail with default production features. Collect before another build.
Qualification artifacts/scripts: evidence-root/qualification. runtime_workload.py
implements Director60s/90s adapter subprocess polling plus three concurrent HTTP
senders (bursts3 with3s spacing every60s),read/ack,2s checkpoint interval,three
phases/two clean restarts,cold canonical integrity/FKs and exact successful-send
and receipt ID comparison. Default3600 active seconds. Ruff/mypy strict pass.
Twelve-second baseline preflight PASSED18messages,two restarts; all three cold
checks pass,workload-preflight-baseline/result.json. This is harness validation,
NOT the required60-minute integration evidence. Adapter uses installed0.13.1.
Installed-release browser E2E PASSED38 assertions incl20browser checks,maxActive1,
qualification/browser-baseline,session87300 exit0. Fixture preserved at
qualification/tmp/e2e_mail_ui.lsdhx4; use only copies for further runs.
browser_measure.py/browser_probe.js implement three controlled1000-message
browser scenarios (unchanged/changing/500+503),5s refresh,60s measurements,
trace+rendererCPU/RSS+heap/DOM samples. Ruff/mypy pass. First baseline measurement
active session34534,performance-baseline.log. This run overlaps compilation;
repeat matched baseline/candidate measurements after build for fair comparison.
Production binary build PASSED,session8534 exit0,13m27s. Integrated browser E2E
PASSED38checks incl20interactive checks,session15693 exit0,browser-candidate.log.
First candidate preflight stopped on aggregate pressure red while all critical
verdicts/integrity were green. Evidence: commit queue p95~1028ms with7pending;
pressure red threshold250ms. Harness wrongly required aggregate green during
load; corrected to enforce critical health/status and record all pressure, as
the approved plan requires zero corruption/write failures,not zero transient
queue pressure. No product thresholds were changed. Candidate preflight2
PASSED30active seconds/27messages/two restarts/all three cold exact-ID,FKey,
integrity checks. The missing generation/checkpoint warning was diagnosed:
generation is intentionally seeded on first reservation,not by mail. Added
real reservation to workload; candidate hour run has generated its durable
reservation checkpoint already. No source fix required by these observations.
Candidate60minute run ACTIVE session8651,qualification/workload-candidate-hour.log,
root qualification/workload-candidate-hour. Starts three1200second phases with
two restarts,three senders,11pollers,Director monitor,read/acks,2s checkpoint.
Current counts after first cycle9sends/9acks/11poller reads/1monitor;no failure.
Matched browser baseline THEN candidate ACTIVE session74144,outputs
qualification/performance-{baseline,candidate}-matched. Same copied fixture and
1000message model,60s per scenario,5s refresh. Compiler is stopped for both.
First calibration browser run completed successfully (performance-baseline),
but overlapped compilation; do not use it as the final comparative result.
Candidate stdio suite ACTIVE session49274,
qualification/stdio-candidate.log. No live mailbox or service mutation.
Database-only all-target compilation initial session69802 ended101: new upstream
project_mailbox.rs InboxRow initializer lacked sender_display_name. Fixed shared
inbox SELECT/decode and product inbox SELECT (index16) to carry current labels.
Added shared-inbox label rename/clear regression and updated reader fixture schema.
Second compilation session70476 ended101 on a test InboxRow initializer;
added sender_display_name:None. Third database all-target check PASSED18.16s,
session35221 exit0, log check-db-3.log. This includes the new label test source.
Database project_mailbox::tests PASSED: 18 passed, zero failed, runtime1.51s
after13m56s compilation; session92106 exited0. Log test-project-mailbox.log.
Includes current sender label rename/clear with stable address/message identity.
Schema validation session12892 exited101:96 passed,2 failed. Both project event
upgrade tests exposed shared event SELECT missing sender_display_name (and
recipients_json) required by the fork's decoder. Rust implementation workflow:
safe local integration fix applied to project_mailbox::visible_events_sql;
rerun schema tests before considering this defect verified fixed. First failure
log preserved as test-schema.log. Rerun test-schema-2.log PASSED98 tests,
zero failed,5.69s; session35165 exited0. Both failed upgrade regressions now
pass on canonical and runtime engines; label migration regression also passes.
Missing shared-event decoder fields defect is verified fixed.
Pool validation test-pool.log PASSED5 tests,zero failed,0.54s; session41327
exited0. Includes real engine four-writer/256-row workload, generation retirement,
bounded owner eviction, and anonymous-memory lifetime checks.
Sync/event validation test-sync.log PASSED34 tests,zero failed,4.92s;
session68148 exited0. Next validation: tools crate all-target compilation,
two jobs, timeout1800s, log check-tools.log, session85345 exited101.
Storage lib.rs defined write_op_storage_root twice (lines1528/3628). Safe local
Rust integration fix: removed the identical first helper, retaining the shared
definition and restoring the direct-write documentation to its function.
Rerun check-tools-2.log session87529 exited101: duplicate helper fixed;
six upstream test calls lacked the optional display_name registration argument.
Safe local Rust test integration fix: add None in auto_register_profile,
project_mailbox,session_identity (three calls),session_identity_eviction.
Existing assertions retained. Third tools all-target check PASSED:
check-tools-3.log,session90463 exit0. Both compilation defects verified fixed;
affected files pass git diff --check. Runtime tools tests still required.
Server all-target check session74435 exited101,check-server.log:10 test
compilation errors. Safe local Rust test integration fixes applied: two
catch_up acquire diagnostics use into_result instead of requiring connection
Debug; two AgentSummary fixtures add display_name:None; five label message
tests pass upstream MessageFilter::NONE; TUI seed registration adds None.
Assertions preserved. Rerun check-server-2.log PASSED,session12043 exit0.
All ten compilation errors verified fixed. Server runtime display_name tests
PASSED21 tests,zero failed,1.09s after17m06s compilation;session99191 exit0,
log test-server-display-name.log. Covers web escaping,duplicate labels,Unicode,
compose addresses,and current labels in directory/message/thread histories.
Mail UI regression run PASSED152 tests,zero failed,19.92s;session66468 exit0,
test-mail-ui.log. Next archive catch_up integration tests,one test thread,
timeout600s,log test-archive-catch-up.log. Session41210 exited101:all3 failed;
first reconciliation repaired0 instead of4; catch-up returned backlog even on
shutdown. Required Rust integration defect under investigation. Added captured
WARN tracing to the fixture and first-report assertion diagnostics to expose
production refusal reasons; assertions unchanged. Rerun diagnostic log
test-archive-catch-up-2.log session18198 exited101:diagnostic subscriber lacked
a declared dependency. Added existing workspace tracing-subscriber as server
dev-dependency for captured warning diagnostics. Next offline diagnostic rerun
updates only required lock metadata; verified Cargo.lock adds just that existing
dependency to the server entry. Diagnostic session21326 exited101,all3 failed.
test-archive-catch-up-3.log proves all9 messages deferred because the fixture
used macOS /var symlink paths and the archive guard refused them. Safe local
Rust fixture fix: canonicalize the temp root before deriving DB/archive/project
paths, preserving guard behavior and every assertion. Rerun session94874 exited0,
test-archive-catch-up-4.log:3 passed,zero failed,4.56s after2m23s build.
Archive catch-up fixture defect verified fixed. Warning diagnostics remain useful
for future refusals. Server executable depends on CLI (crates/mcp-agent-mail/
Cargo.toml),so browser/live-process qualification cannot bypass the CLI conflict.
Tools runtime regression display_labels_follow_current_profiles_without_changing_mail_identity
PASSED1 test,zero failed,0.76s after10m11s compilation;session73756 exited0,
test-tools-current-labels.log. No validation process remains active.
CLI conflict resolution completed after explicit mechanical extraction approval.
Next full workspace gates,integrated executable,browser/workload,publication
and upgrade. Preserve build output until qualification completes; no service
mutation or deployment has occurred. Prior blocked condition is resolved.
Broader archive catch_up run using the existing server test executable passed
20 tests,zero failed,3.86s (includes the three integration tests),session96867
exit0,test-archive-catch-up-all.log. No second compiler job was started.
Database ID floor regressions passed34 tests,zero failed,0.40s using the existing
DB test executable;test-id-floor.log. Next bounded message_prune tests use the
same executable while tools compilation continues. Bounded candidate selection
passed1 test,test-message-prune.log;verified source/settlement/pool/cascade
pruning passed3 tests,test-verified-prune.log. CLI retention tests remain
pending; these database cases do not replace them.
Full CLI/workspace gates remain pending.
Build-storage preservation checked: git diff dc78f19e -- install-local.sh
.cargo/config.toml is empty; bash -n install-local.sh passed. The October13
installer lifecycle evidence below remains dated evidence for identical script
content; integrated binary installation still requires current validation.
Focused rustfmt check over ten edited integration files found one storage
function-signature layout difference; formatted it and reran successfully.
Logs fmt-integration-seams.log and fmt-integration-seams-2.log preserve both
results. Full workspace format gate awaits the CLI conflict resolution.
Mechanical extraction approval applies only to this CLI merge.
Completed compilation command:
CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 gtimeout --kill-after=15s3600
cargo +nightly-2026-08-31 check --locked -p mcp-agent-mail-db --all-targets.
All three compilation logs are retained. Third check includes final source.
Existing two-job host-load override applies. Build output must be cleaned after
qualification, as required by the approved plan.
Retention now explicitly expires shared-mailbox receipts/deliveries and verifies
their retained full rows; the existing real-database regression now includes
expired, exact-cutoff and other-project shared deliveries with receipt checks.
This regression is not yet executable while CLI lib.rs remains conflicted.
CLI test inventory saved as cli-test-inventory.json: upstream adds32 test/helper
functions and removes0 relative to the common base; extracted fork tests.rs
has34986lines. Preserve those additions in the module merge. Read-only inventory
used exact Git source, with no mechanical source edits while approval is pending.
Merged inbox refresh retains single-request admission, unchanged-object reuse,
failure backoff and lifecycle cleanup by inspection. Existing browser regression
is tests/e2e/test_mail_ui.sh; bash -n passes. Runtime/browser checks remain undone.
Next collect the focused database tests, resolve remaining CLI conflicts, inspect
automatically merged seams, and run the remaining approved checks. The dependency
lock has already been regenerated; avoid unnecessary dependency updates.
Full disposition and ledger: preservation evidence directory/disposition.md.
Installed UI baseline captured in separate browser session mail-integration-baseline
(now closed): live-ui-baseline.trace.json and live-ui-baseline.json under evidence.
1000messages, auto-refresh45s, requested30s idle recording, DOM2419before/after,
JS heap50857902→74256687bytes, refreshFailures0. Trace RunTask inclusive2916ms;
do not sum overlapping trace categories or equate JS heap with RSS. Live traffic
was uncontrolled. This is a baseline of installed0.3.36; integrated/changing/
failure comparisons and long-term growth checks still required.
Browser regression inspection confirms tests/e2e/test_mail_ui.sh exercises
1000 synthetic messages in the real browser model,50 rendered rows,search and
keyboard paging,unchanged array/body identity,label updates,one in-flight
refresh,500/503 backoff,navigation after failures,icon settling,and destroy
cleanup. This is controlled frontend evidence; run it against the integrated
server and separately measure CPU/memory with real mailbox data. Inspection
alone does not satisfy the browser qualification step.
Reliability harness prior art inspected: stabilization/20261012-engine-repair/
process-workload.py. It uses isolated native server/adapter subprocesses,11
pollers and a director monitor, but runs30 accelerated rounds and10 bursts.
It cannot satisfy the approved60-minute run unchanged. It also names cached
coding-agents0.13.0; resolve the installed adapter when preparing the new run.
Before qualification, adapt duration/cadence to60/90s, add supervised isolated
restart cycles, and compare successful send IDs with persisted records.
Classification: required integration validation work, still open. Preserve the
original incident script/evidence. Initial search included the engine checkout;
subsequent inspection was restricted to the exact root-level harness path.

BUILD DISK CLEANUP COMPLETE, 2026-10-13: operator authorized cleaning unused artifacts
and changing builds to favor disk space over speed. Cargo clean removed this
checkout's entire target directory (120721 files; du 141.3GiB allocated).
Removed1175 byte-verified duplicate recovery files from the exact Oct8 preflight
and cutover rollback/archive/doctor/reclaimable subdirectories, preserving all
original counterparts in the live doctor/reclaimable directory. Removed192 old
untracked Beads cache copies (~769MiB), keeping latest history pair and full
latest vacuum recovery family. All source, active tracking data, installed
bundles, live mailbox, final retention recovery and corruption forensics remain.
Evidence root: ~/.local/state/mcp-agent-mail/stabilization/20261013-build-cleanup.
Exact authorization, commands, times and witnesses are in build-clean-before.json,
cleanup-plan.json, cargo-clean.log and duplicate-clean-result.json.
Operator chose PRESERVE Time Machine snapshots. No snapshot deletion command
was run. Free space initially stayed~12GiB, then increased~155GiB to~167GiB;
tmutil listlocalsnapshots later reported none. Record this as external/system
expiration, with exact actor and trigger unqualified; never claim manual removal.
Build changes in progress: .cargo/config.toml build.incremental=false;
Cargo.toml dev debug=0/incremental=false and removed server debug override;
install-local.sh private build+target directories, CARGO_INCREMENTAL=0, locked
build, byte-checked installation and EXIT cleanup. A real invalid-rustflags
failure probe exposed Cargo's missing CACHEDIR.TAG refusal. Installer now seeds
the standard cache signature before compiler startup; retest is active.
Workspace checks PASSED with two jobs: all-target18m25s (includes fresh dependency
fetch), production lib/bins2m06s, Clippy-Dwarnings1m19s. Formatting, shell syntax,
shellcheck and diff whitespace also passed. Incremental cache remained0bytes.
Initial offline check lacked source cache; online retry preserves that failure
log. Corrected invalid-rustflags installer probe returned original exit101 and
cleaned its private directory. Initial failed-probe debris was then cleaned.
Unknown installer option returnedexit2. Validation target cleanup removed9176
files (reported2.2GiB). Full native release install completed35m14s, installed
both actual binaries, verified their bytes and automatically removed8498files
(reported4.0GiB) from its private build directory. Space/colon paths passed.
The native run exposed the pinned Darwin rust-objcopy missing libLLVM after
the system shell cleared inherited DYLD_LIBRARY_PATH. Final installer restores
the compiler sysroot library path internally and selects only am/server bins.
Exact final-script copy passed a real Cargo/LLVM lifecycle fixture with zero
warnings, an unavailable unused helper excluded, copied binaries verified and
private directory removed. Both actual native app binaries also passed stripping
and version probes. Final invalid-compiler probe preservedexit101 and cleaned.
Six temporary native/fixture/stripped proof binaries removed after hashes saved
(434463856bytes); all temporary build parents are gone. Tiny generated fixture
source/manifests and logs remain as replay evidence; no compiled cache remains.
Final checkout du242568KiB (~237MiB), downfrom142.2GiB. Filesystem free~153GiB,
upfrom~11.5GiB before cleanup; other host activity affects this net measurement.
Final doctor health GREEN. Verify final source/binary/space witnesses in
validation.json. Live current link and both installed binary hashes unchanged;
service never restarted/replaced. Source changes uncommitted/unpublished.
Formal prose audit token2026-10-13_16-26-14-427-1130727b3700 sealed unknown/failed
for missing rootspx; no text read. Implementation audit token
2026-10-13_16-27-01-520-fd141f0b5b0c BLOCKED at same coverage gap; workflow offers
no truthful terminal seal without coverage. Both limitations tracked br-rpa0b.
br-hxcv4 records approved cleanup and snapshot-preservation decision; broader
automatic recovery retention remains open. No commit, push, service restart,
or further mail pruning is authorized by this cleanup request.

CLOSEOUT: final qualification at evidence-root/validation.json; br-xydkn closed.
Independent native robot-message live-read lock failure is verified-important
br-zqm0g under the operator's separate diagnostics/tech-debt tracking preference;
optional scope question received no reply during independent closeout. Continue
retention delivery as requested; inspect that separate read path if the operator
selects expansion. No failure of send/inbox/receipt/health or data continuity.
Source/Beads uncommitted; no commit/push performed. Do not repeat completed
pruning or treat the stopped-service snapshots below as current state.

RETENTION COMPLETE, 2026-10-13 around01:29UTC. br-xydkn is CLOSED with
implemented/verified-live labels. launchd com.agent-mail ACTIVE PID99745,
health GREEN. Current link points to
~/.local/opt/mcp-agent-mail/retention-thread-digests-3659c270-20261013.
Native am hash c449d738975678fe950f21e81a0be2006e6c2a39648ff40f36cc80193aaa3055;
server unchanged qualified hash49edba55f2a5fd4a4ce42c9be58f1faa27cc992a437606b09c720248330ea23d.
Keep every prior bundle, corrupt-family evidence and recovery snapshot.
Final run: stabilization/20261012-retention/live-final-20261013 under state root.
Cutoff2026-10-12T01:28:38.697462Z: changes.git11414→156 messages,11258expired.
Global14670→3412; all other projects preserved.36740 files moved offline,
including1712derived thread digests. Root retention state complete=true,
global allocator floor14670, archive Git history records expiration.
Full before.sqlite3 and every moved copy preserved; no file deletion.
Independent SQLite integrityok/FK0, exact projects/agents/contacts/reservations,
all3412retained full messages and3446receipt rows match the snapshot.
Counts28projects172agents148contacts13reservations unchanged. Native validation
also compared retained events and signal receipts and rebuilt inbox counters.
Fresh self-mail14671 routed GreenLake/Agent Mail Maintainer and was read back;
receipt persisted read/ack timestamps2026-10-13T01:31:31.416282Z. Director
completion14672 was delivered with checked pane Enter, explicitly releasing
his/all positions' monitor/doorbell/mail-call standdown.
Final tests2/2PASS0.74s; all-target33.96s,production22.38s,Clippy47.86s,
format/diff whitespace pass. Corrected CLI release7m34s; prior BOTH binaries
release14m23s. No remote publication/source commit/recurring schedule.
Local source/Beads remain dirty; preserve all edits. Formal SPX audit remains
blocked by absent rootspx (recorded), no formal approval claimed.
Failed previews pruned nothing. Concurrent direct CLI ack/sends bypassed the
stopped supervisor and changed snapshots (inbox_stats, new14670); strict
admission refused. Checked pane facts requested all-position standdown;
immediate final preview/apply succeeded. Track writer-serialization evidence
in br-pcgj3; no owner override or forced kill occurred.
Separate native robot-message read refused live archive lock; the optional
scope question asks tracking versus inspection. Retention is complete and its
successful send/inbox/receipt/health checks are independent of that defect.
Physical DB remains91.5MiB (logical rows pruned; no VACUUM), recovery staging
~12.3GiB preserved. br-hxcv4 owns physical cleanup, still no deletion approval.
Evidence: result.json, independent-census.json, independent-full-record-comparison.json,
send-proof.json,inbox-proof.json,receipt-proof.json,completion-delivery.json in
the final run directory; qualified bundle manifest has exact source/binary hashes.

CORRECTED LIVE-LAYOUT QUALIFICATION, 2026-10-12 23:42UTC: live preview from
first qualified bundle stopped before any row/file pruning because it mistook
messages/threads/change-252-handoff-5956807978.md (derived digest without ID)
for a message. Service was immediately bootstrapped again and is healthy,
PID657, current link retention-3659c270-20261013. Native first preview created
only an intact offline snapshot at evidence-root/live-20261013/before.sqlite3;
no manifest or pending marker and no prune. Director had checked maintenance
notice14666. Initial launchctl bootout returned before PID12907 exited;
exclusive activity locks admitted preview only after it exited. Subsequent
doctor drain confirmed safe_to_mutate=true. No forced kill/owner override.
Fixed CLI scanner to exact dated canonical/inbox/outbox layouts, with derived
thread digests recorded separately and moved offline to invalidate stale body
previews. Plan version2; durable runtime retention state remains version1.
Corrected source tests2/2 PASS (0.74s, compilation2m55). Required Clippy/checks
are now rerunning, then build only corrected am (server code unchanged), package
another immutable bundle, fresh Director notice, drain/preview/apply/restart.
Do not reuse first partial run directory or rebuild unchanged server code.
No live mail removed. Still execute br-xydkn; user has authorized it.

LATEST EXECUTION, 2026-10-12 22:37 UTC: live pruning still NOT performed and
launchd service remains active. Native retention implementation is finished;
final required all-target check (27s), production lib/bins check (36.07s),
Clippy -Dwarnings (30.42s), formatting and diff-whitespace checks passed.
Two focused tests passed before the final durable-backlog guard (0.68s).
Final test rerun is now compiling; collect its process session from the turn
before starting another build. Initial compilation cost ~25min for the two
async-runtime/database families. Fixed fixture setup to use the real DbPool
startup (historical migration runner over latest schema duplicated reaper_exempt).
Fixed recovery allocator call to use CanonicalDbConn helper. Tests moved to
the end of retention.rs to satisfy Clippy; no lint suppressions.
Added require_drained_archive_backlog in storage/lib.rs: refuse queued JSON
archive work that could recreate old messages after restart. Native archive
commit records expired paths plus complete retention state; idempotent apply
does not mint another commit. Archive Git tests use a real repository.
All helpers and source edits are uncommitted. The source and live binaries
still differ: need qualified release build/package of BOTH native binaries,
guarded local current-link switch, a fresh Director maintenance notice,
supervised bootout/drain, native preview/apply, independent counts/health/write
proof, launchd bootstrap and checked completion. Do not stop the service while
builds run. No file deletion required: expire DB rows and move old archive
copies into the run directory. Physical recovery-copy cleanup is separate.
Director start fact14660 delivered with checked pane doorbell;
evidence execution-start-delivery.json under the retention evidence root.
Validation evidence is validation-in-progress.json there. Formal advisory
SPX audit remains blocked by absent root spx/, not an approval. br-xydkn is
in_progress; keep it open until live operation and checks complete.
AGENTS.md corrected storage src/archive.rs to its actual src/lib.rs (safe
instruction-reference repair). No CLAUDE.md exists in this checkout.

RETENTION EXECUTION AUTHORIZED: operator said "Execute it" for br-xydkn.
At 2026-10-12 22:00 UTC source implementation is underway; no live mail
pruned and the service remains healthy/running. New explicit offline
`am archive prune --project KEY --run-dir PATH` previews a full snapshot and
hashed archive-file manifest; `--apply --run-dir PATH` applies/resumes it.
Scope is changes.git only, strict 24h regardless of read/ack state, with a
snapshot and renamed expired archive copies outside the active storage root.
Source files: CLI retention.rs + command dispatch, DB mail_retention.rs,
id_floor.rs/reconstruct.rs, server startup guard. Preserve the engine repair
and other existing edits. Pending marker blocks updated runtime/reconstruction;
completed marker preserves the global message-ID floor. Transaction removes
expired recipients/events/signal receipts and rebuilds inbox counters.
Canonical private snapshots check all protected tables and retained full rows.
Interrupted archive movement resumes against the exact saved hashes.
Focused Rust fixture test compiling under session27580, two build jobs, bounded
20m; source check previously passed, remaining required gates not yet run.
Tests include strict boundary, four read/ack states, other project preservation,
relationships, stale preview refusal, pending startup refusal, resume/idempotence,
archive reconstruction and post-prune ID allocation. No final pass claim yet.
Advisory SPX implementation audit is BLOCKED by absent root spx/, run
2026-10-12_21-59-47-844-b504c20608d0 (`rg --files spx`, exit2). This Beads-governed
fork has the already-recorded coverage gap; no formal approval is claimed.
Target/debug measured ~116GiB of compiler artifacts; no cleanup performed.
br-hxcv4 owns separately approved disk-cleanup planning. Live maintenance starts
only after tests/checks and the Director notification. Do not replay old drains.

Saved after the 2026-10-12 20:25 UTC qualified repair and live recovery. This is the current operational handoff and
supersedes scheduling snapshots in the root `RESUME.md`. Read `AGENTS.md` and
the operator's conversation before acting. The operator wants concrete delivery,
with bounded engineering and minimal repeated permission requests.

NEW RETENTION CONSULTATION, 2026-10-12 21:35 UTC: operator asks to prune mail
older than24h and consult Director. Question14656 delivered with checked
doorbell; Director initial14657 was superseded by final14659. FINAL POLICY:
all changes.git mail older than24h can expire regardless of read/ack state;
no standing-order/thread exceptions remain. Director moved cited instructions
into durable expectations/skill and declared stale old unread backlog safe
to expire. Keep a dated offline recovery snapshot. DO NOT implement the earlier
unread/order-pin exception policy from the superseded reply.
Private automatic-backup copy asof21:20:34UTC integrityok. Frozen cutoff
2026-10-11 21:25:34UTC: changes.git11399total/259keep/11140expire;
whole store14655total/264keep/14391expire. Refresh before any apply. Director
recommendation is changes.git-scoped; other projects need concrete operator
scope. Per-agent census delivered14658 with checked pane doorbell. Retention
evidence ~/.local/state/mcp-agent-mail/stabilization/20261012-retention/:
director-reply-14659.json, strict24h-preview.json, backup-copy.json and
backup-retention-census.json. br-xydkn is verified-important, policy-verified,
implementation-pending; current native CLI has no age-prune operation. Plan
guarded dry-run/apply, active DB/archive/search/events consistency, no recovery
resurrection, identity/relationship preservation and retained offline snapshot.
No messages/files pruned or deleted, no service stop or restart this turn.
br-hxcv4 still owns physical cleanup: recovery staging~12GiB dominates disk,
versus DB91.5MiB, archive216MiB and Git100MiB. Prior exact duplicate-copy
deletion proposal remains unapproved. Session Prowl resolver lacked caller
metadata; supplying the exact harness worktree path via PROWL_WORKTREE_PATH
resolved the matching live caller E07627B6-D1D9-46EC-876A-4E3F645FC311 and
Director35B83E23-C979-40BB-AED2-03DA4A04800D, without guessing by title.

REPAIR COMPLETE: launchd com.agent-mail is ACTIVE, PID12907, at127.0.0.1:8765.
Current bundle is ~/.local/opt/mcp-agent-mail/engine-repair-3659c270-20261012.
Both native doctor health and MCP health are GREEN. Controlled live mail14654
was sent, read back with original GreenLake ID48/project13/display label,
and acknowledged; native MCP confirms ack_required=true and persisted
read_ts/ack_ts. Director received the requested checked pane completion with
trailing_enter_sent=true and explicit permission for his loops to resume.
Do not stop the healthy service or repeat reconstruction from old stop notes.
All earlier stopped-service snapshots below are historical.

Official reconstruction succeeded. Independent record comparison found six
recipient JSON arrays reordered and contact-row IDs reassigned. Original
membership and all other fields were intact. A separate healthy private
candidate restored exact recipient order and all148contact IDs; canonical
full integrity/FK and six-table comparison passed. Official doctor restore
promoted it; a fresh private copy of the promoted live family again proved
every row/field/ID identical to the pre-recovery snapshot in projects, agents,
messages, message_recipients, agent_links and file_reservations. Counts before
new control mail:28projects172agents14653messages15941recipients148contacts
13reservations. Current native source repair remains UNCOMMITTED. No remote
publication, file deletion, token rotation or foreign repository edit occurred.

Next reliability work remains verified-important br-pcgj3: observe real resumed
load and qualify a simpler single mailbox writer covering maintenance as well
as foreground writes. The operator asked about reduced concurrency and its
implications; this is a recommendation, not an executed mode change. Smaller
pool settings alone do not serialize separate checkpoint connections;
FSQLITE_CONCURRENT_MODE=false already leaves autocommit MVCC forced ON.
Exact incident transaction is still unproven; finite tests are not17h replay.

Separate verified P1 br-66fdv: installed coding-agents0.13.0 adapter turns native
robot ack_status=required into ackRequired=false, because it recognizes only
pending/acked. Native stored requirement and receipt persistence are correct.
Scope question offered tracking with Plugins Maintainer versus inspecting the
adapter next; no explicit choice yet. Honor existing fork-only ownership: no
generated-cache edits or foreign Executor start. Director has the exact facts
in his checked recovery pane message and delivered mail14655, correlation
engine-repair-20261012. The latter has no new doorbell/receipt/agreement claim.
Obsolete doctor top-level flags in recommendations/handbook are recorded in
the separately operator-selected instruction/test cleanup br-rpa0b.

Evidence root: ~/.local/state/mcp-agent-mail/stabilization/20261012-engine-repair.
cold-family holds10exact files/99,825,319bytes plus SHA manifest. Separate
canonical-cold-probe was opened with stock SQLite3.54.0 read-only. It confirms
31duplicate page references (including9511) and16index discrepancies; FKcheck
is empty. Counts28projects172agents14653messages15941recipients148contacts
13reservations. The original retained cold-family is never opened by an engine.
Native old-binary doctor reconstruct --dry-run --json passed: all14653archive
messages,28projects172agents15941recipients13reservations with DB-only salvage.
Metadata warnings concern missing project.json/fallback slugs and a slug
mismatch in the retained Plugins archive; preserve their evidence and original
DB human keys through the recovery continuity boundary.

Candidate implementation: Cargo.toml and Cargo.lock now pin the entire runtime
FrankenSQLite0.4.10 family to3659c270a42adac3ef1f980c36c731890d015306.
Embedded Beads0.3.18 is untouched. Upstream includes the WAL-generation fixes,
macOS retained-descriptor fix23079d4e, and equivalents of the four AM SQL/VFS
compatibility commits. Read-only upstream clone at <evidence-root>/frankensqlite
is detached at the initially inspected64edd26a; the3659c270object is also local.
No dependency-cache edits or publication.
AGENTS.md dependency table updated. New finite regression in existing
crates/mcp-agent-mail-db/tests/stress.rs:
concurrent_mailbox_poll_send_ack_and_checkpoint_preserves_pages
uses11fresh-connection readers,3send/read/ack workers,30checkpoint attempts,
90messages and independent canonical full-integrity/FK and receipt checks.
It is a probabilistic stress scenario, not proof that the incident's exact
trigger has been reproduced; actual adapter/HTTP process boundaries still
need installed-isolated coverage. Existing compatibility tests also required.

The first64edd26a candidate passed production check4m17s and all-target
check2m39s, then FAILED runtime tests before deployment. New regression
failed on first file-backed DB open with errno102 Operation not supported on
socket; existing NOCASE suite independently failed3file-backed cases and
passed its memory-only case. New upstream874fb59a SHM backing admission is
the only production-source difference between3659c270 and64edd26a. Its
commit explicitly claims no macOS/full-engine runtime validation; Darwin's
physical allocation receipt is unsupported here. The new pin precedes that
unrelated change while retaining every targeted corruption/descriptor fix.
Do not suppress the failing test or alter the guard. Rerun every required
gate at3659c270; old-candidate checks are not new-candidate evidence.

Final-pin stress regression PASSED: 90 sends/read/acks, 330 fresh reader
connections, 30 successful checkpoints, canonical full integrity and FK check.
It exposed an AM checkpoint classification defect: a busy strict checkpoint
was reported as generic SQLite failure. pool.rs now classifies busy>0 as
BusyRetryable while still refusing every incomplete checkpoint. Six focused
unit tests passed, including incomplete nonbusy refusal and display labels.
Compatibility/schema/identity integration: 12 passed, one declared heavy replay
ignored. Both final-pin workspace compile graphs passed (all-targets 1m44s,
production lib/bins 1m22s); formatting and Clippy passed (1m36s).
Release build of both native binaries PASSED in25m59s, session21507 terminal.
Its toolchain debug-strip helper could not find libLLVM.dylib; fixed packaging
by invoking rust-objcopy with explicit toolchain DYLD_LIBRARY_PATH and separate
outputs, preserving original build artifacts. Both strip commands exited0.
Prepared bundle ~/.local/opt/mcp-agent-mail/engine-repair-3659c270-20261012:
am SHA22eb2c1256562cfa10e09391bb6639f9387b8f9e32e8c6a8ddd5728dcf2b6e6b;
server SHAf2c05c8703122a88e86442af884517ddb4741d198eaba5c7c23692c5ddb55999.
Manifest records uncommitted source/diff hashes and previous current target.
Installed isolated process-workload experiment PASSED in44.8s, session71780
terminal. 330 doorbell+30 monitor fresh Python adapter reads;30 HTTP sends,
auto-marking reads and acknowledgements. Cold stock SQLite full integrity ok,
FKempty,30messages and30read+acked rows. FD samples2->8, stable8 through rounds
10/20/30 and completion; maxfd24. Graceful isolated server exit0. Full evidence
<evidence-root>/process-workload.json. Short accelerated scenario, not17h replay.
New doctor drain confirms safe_to_mutate=true. New-pin reconstruction preview
PASSED all14653messages/15941recipients/172agents/28projects/13reservations.
Official reconstruct --yes --json PASSED, session46201 terminal. Exact-record
candidate preservation, official restore and promoted-copy verification also
PASSED. Output <evidence-root>/reconstruct-result.json/.stderr (stdout has
two progress lines before JSON), exact-restore.stdout/.stderr,
exact-record-preservation.json and promoted-cold-check/continuity.json.
Current symlink switched reversibly and launchd restarted from unchanged plist.
The first Clippy command found the wrong cargo-clippy on PATH (Rust1.99);
rerun pins PATH as well as RUSTC/RUSTDOC to nightly-2026-08-31 Rust1.100.
Previous sessions50579,93070 and72300 are terminal failures. nextest is absent;
existing gtimeout is available and provides bounded process-group teardown.
Director received the build-failure/pause update through a checked pane send
with trailing_enter_sent=true. All native
validation/builds remain sequential,2jobs, under the operator's existing
load-gate exception. Toolchain nightly-2026-08-31, set RUSTC/RUSTDOC explicitly.
cargo fmt --check passed. All process, recovery, bundle-switch, restart and
live send/read/ack steps completed; no native test/build sessions remain active.
Do not claim long-term prevention solely from short checks. All10original
forensic cold-family hashes remain retained unchanged. Live descriptor sample
9/maxfd26; isolated stable8 is a short bounded observation, not a matched
long-duration performance claim. Native macOS doctor descriptor check uses
/proc and reports unavailable; actual counts came from lsof.
Formal implementation auditor returned BLOCKED because this Beads-governed
fork has no root spx/ tree; run2026-10-12_19-35-27-654-cf7da2f2f378,
rg --files spx exit2. No formal approval is claimed; do not fabricate a tree
or a prose fallback verdict. Evidence and limitation are tracked in br-pcgj3.
Disk free space fell to about11GiB during checking; target exceeds103GiB. No file
deletion authorization exists for the old16.2GiB duplicate-copy proposal.

Historical recurrence diagnosis: live doctor health was RED again. At 18:31 UTC the unchanged
PID49676 / binary reported page9511 owned by both freelist trunk[13] leaf[402]
and the messages B-tree. This supersedes every green-health snapshot below.
The service's full integrity check last passed at17:28:12Z and rejected the
same page at18:32:52Z. Repeated full checks passed for over17hours after restore.
No repair, restart, live database edit, source edit or deletion occurred during
this recurrence investigation. The operator asked why this keeps happening.

Leading cause: the pinned FrankenSQLite0.4.4 runtime revision db458bf lacks
upstream fixes a712b8ea4d2f0e9ad203985e9817f598a24b91bd (2026-09-24) and
566494d5775256992b4d9a522f5286fa38edfe89 (2026-09-30). Their primary commit
records describe live pages being reclaimed from stale abandonment pools
across WAL generations, then freed/granted again. The pinned pager retains
the unfixed refresh-before-new-extent ordering around line9769 and lacks
the commit-fold generation check around line26000. This matches the observed
free/live page alias, but the exact triggering transaction is UNPROVEN.
Periodic AM maintenance uses PASSIVE checkpoints; do not assert a logged
TRUNCATE caused this incident. An isolated reproduction must establish which
reset/allocator path runs in this workload. Current cold canonical confirmation
is absent: native doctor and service probes both fail, while the private
canonical copy was inconclusive under a live writer. Preserve the DB family
under supervised drain before independent physical diagnosis or repair.

Additional verified operational fault: lsof selected7,292 database-family
descriptors for PID49676, maximum fd7315, against a previously observed8192
soft limit. The pinned VFS deliberately retains redundant descriptors while
process lock claims remain; reuse_retained_file is Linux-only (unix.rs923).
macOS retention is a concrete resource problem; causation of corruption is
UNPROVEN. Current disk free space is about26.5GiB (98%used); no observed
ENOSPC/EMFILE in the targeted post-recovery log search. Only current owner is
PID49676; this does not exclude past transient openers.

Track all recurrence evidence under verified-important br-pcgj3. Source
prevention and a macOS workload regression take priority over labels/cleanup.
Recovery alone cannot close the bead. A broad engine update is not trivial:
the upstream566494d5 branch diverges from our pin by493upstream commits and
4fork-only commits, so retain the four AM SQL/VFS compatibility repairs.
Do not recommend FSQLITE_CONCURRENT_MODE=false as disabling MVCC: AM forces
autocommit concurrent modeON independently. Next bounded route is a preserved
cold family, independent integrity check, isolated macOS checkpoint/write
reproduction, then qualified targeted backport or engine update and supervised
recovery. The operator has not selected an architecture replacement.

Historical recovery: the operator/Methodology Director explicitly
requested supervised drain, reconstruction, restart and a checked reply to
pane `35B83E23-C979-40BB-AED2-03DA4A04800D` because mail was corrupt.
Initial live doctor health reproduced malformed index page 11418 and duplicate
page 9501 ownership. `launchctl bootout gui/502/com.agent-mail` stopped PID841;
doctor drain then confirmed safe_to_mutate=true. No live-owner override or
manual lock removal occurred.

`am doctor reconstruct --yes --json` built a valid archive-plus-salvage
candidate, but final promotion failed on the recovery election lock. No holder
was found after that process exited; the cause is unqualified. The official
`am doctor restore` promoted that same preserved candidate through its full
integrity/continuity/receipt boundary. Original corrupt DB and sidecars were
quarantined at `storage.sqlite3.corrupt-20261012_001051_298` and matching
siblings, plus forensic bundle
`doctor/forensics/storage.sqlite3/reconstruct-20261012_000714_846`.

Launchd was bootstrapped from its unchanged plist; new PID49676 was live and
port8765 reachable. Live `am doctor health` exited0 and MCPhealth was green,
full integrity passed, archive/DB agreed on172agents and14475messages, and
reservations were13/13. Candidate counts:28projects (24archived plus DB-only
state),15741message recipients,148contacts,13reservations. Native binary and
source unchanged; no token rotation or unrelated file cleanup.

The Director received a checked completion message through the requested pane:
Prowl send exited0 with trailing_enter_sent=true. Evidence including exact
commands, health and checked handback is
`~/.local/state/mcp-agent-mail/stabilization/20261012-mailbox-recovery/recovery.json`.
Verified-important P1 follow-ups are **br-pcgj3** (corruption recurrence) and
**br-0d2se** (reconstruct promotion lock refusal). They remain open; restored
availability is not proof that either cause is fixed. Reproduce on isolated
copies, never by another mutating repair against the healthy live store.
The service-status verb printed launchd environment values; use filtered
supervisor projections in future operational captures. The operator already
accepts local token exposure as no issue; do not rotate it or create a new
approval loop on that basis. Preserve the incident evidence independently of
the duplicate-copy cleanup, which still lacks deletion authorization.

Latest fork-only resume: live health is green; no new mail handoff or hosted
Executor for #461/#471/#342 was found, and retained #461 head is unchanged.
Do not repeat already-passing inbox checks while its owners remain paused.
The old request scratch files were cleared by the environment. Current bounded
inbox/inventory requests are under
`/var/folders/vg/stx1n5ys4_lgr08v49lg6t880000gp/T/greenlake-20261012/`.

Independent cleanup preparation for **br-hxcv4** is now concrete: the published
backup restored in memory with integrity ok, zero foreign-key findings and all
five recorded table counts matching (14,395 messages, 169 agents). Full byte
comparison proved 1,175 recovery files in the preflight and cutover rollback
`archive/doctor/reclaimable` directories equal the retained live originals.
Those two copied directories total 16.2 GiB allocated file bytes; APFS shared
blocks may reduce actual reclaimed capacity. Preserve all live originals,
rollback databases/archives outside those subdirectories, forensic bundles,
binaries and manifests. Detailed plan and exact witnesses are in
`stabilization/20261011-triage/disk-retention.md`,
`duplicate-recovery-copies-20261012.json`, and `backup-restore-20261012.json`
under the state root. No deletion is authorized or performed. Revalidate the
exact witnesses and retained backup before approved removal, then measure free
space and service health. The live quarantine is now about 11.3 GiB; this
inventory does not establish why each new backup generation was created.

Latest resume: the operator chose **Resume this fork** after receipt of Director
standdown **14377**. Continue this fork's local verification; the Plugins team
remains paused. Start no external Executor or peer source mutation under that
choice. Maintainer answer **14369** supplies the repair route: the assertion
producer defect was already repaired by #477 in installed spec-tree **0.105.5**;
the old Executor had loaded 0.105.3. Revise the complete-record oracle Decision,
then restart #461 on its retained head and rerun all gates when team work is
authorized again. No methodology/SPX source repair is required by that answer.

The own isolated-store check at committed head
`707a86da431e3b953c0a410b8c691c859f9d106b` passed **21 messages** across
RedStone with bodies and SilverFinch without bodies. Every complete canonical
record matches the installed-adapter baseline, and sender display entries match
the exact message IDs and expected label/null/delimiter fallback. Retained in
`inbox-projection-committed.json` under `resumedAcceptance`. No profiles or
messages changed. STATUS **14395** was delivered mail-only to the Director;
delivery is not acknowledgement or team continuation. br-0egjn remains open
with items 1, 7 and 9 unchecked. The previous 14360-awaiting-reply text below
is historical; 14369 is the collected answer.

Live health is green. At this resume check, reported git commit p99 was 470 ms
and the current 600-second window no longer exceeded the client deadline.
This observation does not diagnose or erase the retained earlier timeout.

## Checkout and authorized work

- Repository: `/Users/shz/Code/silvarbor/agent-mail/agent-mail`.
- Branch: `work/automatic-position-labels`.
- HEAD: `a3336f2b176d06faa00ec6c15d0493d877e6d966`.
- At save time, `.beads/issues.jsonl` and root `RESUME.md` have uncommitted
  tracking changes. This handoff is newly created. Preserve all changes.
- Native deployed source is `1f309419df08062942ae2f0c29b0990cf631cf5c`.
- The operator authorized sequential native validation with two build jobs and
  a tracking-only commit. Do not ask again for those permissions. The separate
  Plugins owner and Methodology Director explicitly restrict that validation
  exception to this session; do not bypass their Executor's waiter.
- No file deletion, shared-store repair, new deployment, upstream PR or public
  release is authorized by this handoff. Keep normal hooks and required checks.
- This fork has no root `spx/` specification tree. Beads and the explicit
  operator contract govern the bounded work. Do not invent formal spec verdicts.

## Deliver labels first

The native identity/display-label implementation is deployed. Routing addresses,
IDs, permissions, reservations and relationships remain stable. Display labels
allow duplicates, Unicode and spaces; whitespace-only input clears the label.
Recipients are selected by stable names, never by labels.

The remaining consumer integration is **br-0egjn**, still in progress. Its
last recorded checklist was 6/9, with items 1, 7 and 9 open. Do not close it
until actual owner startup/resume, pane pointer delivery/parser, shipped monitor
output, and installed acceptance are verified. The stabilization plan is
**br-vas72**; installed acceptance is **br-vas72.2**. Keep labels ahead of
general cleanup unless a concrete crash, corruption or permanent stall appears.

Native required Cargo checks, production graph check, Clippy and formatting
passed at `1f309419`; adjacent filters passed 106 test executions. This is not
a full-workspace test pass. Seven server test failures remain in the tracked
baseline repair queue. No fresh native compiler validation was needed for the
recent tracking-only changes.

## External implementation: current stop and next action

Plugins Change **#461** owns automatic owner label registration and normalized
inbox sender display. The last public herdr observation, around 21:03 UTC:

- Agent `change-461`, pane `w51:p1`, status `done`.
- Checkout `/Users/shz/Code/outcomeeng/plugins/worktrees/executor-461` was
  clean and detached at `707a86da431e3b953c0a410b8c691c859f9d106b`.
- Formatter and blank-label repair are committed. Test-writing rounds finished.
- Executor reported skill and implementation audits approved, spec audit
  approved after repair; these are reported results, not independently qualified
  verdicts in this session.
- Test-evidence review rejected twice: its oracle compares sender and ID rather
  than the complete canonical message record. The review gate reached its retry
  cap. Assertion audit is blocked and unsealed: adding a second unit to a
  file-scoped verification run failed.
- Rows 2–4 and the attested owner resume probe have not started. Nothing from
  this execution is merged or installed. Do not say testing is active merely
  because a test-writing agent was launched.

**Question 14360** was delivered to **Plugins Maintainer <ScarletGoose>**. It
asks for a bounded full-record oracle repair, the exact assertion-scope failure
artifacts, the repair owner/route through SPX Maintainer and Methodology Director,
and whether a resumed Executor round is assigned. No reply was collected before
this save. Delivery establishes neither acknowledgement nor agreement.
Answer 14369 is now collected and summarized above. Preserve required reviews;
grant no blanket waiver, edit no peer-owned source, and launch no duplicate
Executor. The owner repair route awaits the team resuming.

The earlier load wait ended for implementation. **Methodology Director
<CobaltSpire>**, message **14262**, confirmed the separate owner's wait/priority
route. Director ranks higher than **SPX Maintainer <RusticOwl>**. All three
peer owners now have their own stored labels, verified in real mail metadata.
Their one-time registrations do not prove the general automatic lifecycle.

Change **#471** owns monitor label output. It was confirmed **Framed** at
20:24 UTC, comment `6113359443`; it retains real blockers **#342** and **#461**.
#342 was Proposed/confirmed, with monitor implementation still requiring delivery.
#473's same-plugin sibling-import decision was operator-approved according to
Director message 14163; its issue was still open at the last inspection. Do not
equate approval with shipped runtime behavior. Obsolete #461→#342 dependency
was removed with approved scoped authority; do not remove #471's genuine edges.
#468 audit-producer repair and #472 repair were merged; do not reopen those
permissions or assert they solve the new assertion-scope failure.

## Retained real-path evidence

Evidence directory:
`/Users/shz/.local/state/mcp-agent-mail/stabilization/20261011-label-workflow/`.

- `baseline.json`, `label-routing-negative.json`, `relationships.json`: duplicate
  labels, own-label rename/clear/omission, preserved identity/history/contact/
  reservation/credential relationships, validation boundaries and routing.
- `web-label-refresh.json` and PNG: actual browser unchanged-array/object/row
  reuse, natural refresh after rename/clear, preserved selection/body/IDs,
  and HTML-shaped labels rendered as text. This is not a 1,000-message CPU benchmark.
- `monitor-baseline.json`: actual candidate monitor printed `from GreenLake`
  even though native message 28 carried `Reviewer`. Its edge/cursor behavior was
  retained. Final monitor acceptance needs fresh private state and a new message;
  do not replay old mail by altering the cursor.
- `inbox-projection-before.json`, `inbox-projection-candidate.json`, and
  `inbox-projection-rename-clear-candidate.json`: 138 candidate record reads
  retained immutable records byte-for-byte through own rename/clear/restore.
  This candidate was uncommitted; it was provisional integration evidence.
- `inbox-projection-committed.json`: 16 real stored messages checked against
  committed formatter/blank-label repair source
  `38d5bb9efd0467d99868a40615b1be43f6909524`; complete canonical records equal
  the installed adapter baseline, labels paired by exact immutable message ID.
  This is not verification of the final test head or released installation.
- The proven three-space formatter defect was fixed by its owner using
  `label.strip()` for eligibility while retaining eligible label text verbatim.
- `live-inbox-deadline-observation.json` and `live-mail-timeout-sample.txt`:
  an actual live adapter read timed out; later reads succeeded. Health was green,
  queues zero, but archive commit p99 exceeded the 30-second client deadline.
  No causal diagnosis or repair is established. Retained under br-vas72.2.

## Runtime and communication paths

Live web UI: `http://127.0.0.1:8765/mail`.
Live mailbox: `~/.local/share/mcp-agent-mail/git_mailbox_repo/storage.sqlite3`.
Supervisor: launchd `com.agent-mail`. Discover current ownership/PID before
maintenance. Do not dump the service environment, tokens or private messages.

Current binaries:
`/Users/shz/.local/opt/mcp-agent-mail/engine-repair-3659c270-20261012/bin`.
Native hashes: am `22eb2c1256562cfa10e09391bb6639f9387b8f9e32e8c6a8ddd5728dcf2b6e6b`;
server `f2c05c8703122a88e86442af884517ddb4741d198eaba5c7c23692c5ddb55999`.
Previous automatic-labels-1f309419-20261009 bundle remains intact. Current
manifest records prior symlink target, exact uncommitted source and diff hashes.
Reverting binaries alone does not remove known old-engine corruption risks.

Existing isolated server is at `http://127.0.0.1:8877/mcp/`, previously running
under exec session 60482. Confirm it remains available; do not start a duplicate.
Fixture base:
`/Users/shz/.local/state/mcp-agent-mail/automatic-labels/20261009-1f309419/demo`.
Use its `project`, `store`, `config`, `cache`, `state` directories and explicit
isolated environment. Own fixture **GreenLake ID22** is restored to **Reviewer**.
RedStone is the recipient fixture. Register only the caller's own identity.

Pre-recovery identity: **Agent Mail Maintainer <GreenLake>**, ID48 in project13;
the retained cold source confirms it. The older ID267 snapshot was superseded
by the morning reconstruction. Director is CobaltSpire ID73 in this project.
Shared mail working
directory `/Users/shz/Code/outcomeeng/changes/changes.git`. Source adapter resolves
the common Git directory; never derive or replace the project key manually.
Use current `coding-agents:operate-agent-mail`, `message-agents`, `operate-herdr`
and `operate-prowl` skills for their operations. Python must be
`/opt/homebrew/bin/python3.13`; system Python lacks `StrEnum`.

Existing request/evidence scratch directory:
`/var/folders/vg/stx1n5ys4_lgr08v49lg6t880000gp/T/greenlake-20261011`.
It contains the complete request, capability and result for mail 14360, plus
`greenlake-followup-inbox-1949.json` and
`greenlake-herdr-executor-visible.json` for bounded reads. Preserve all seven
capability result fields, collect yielded command completion, and do not resend
after an ambiguous timeout without confirming persistence. Raw inbox metadata
is under `response.inbox`; labels are outside immutable `data.records`.
Do not send Director pane doorbells. Resolve other pane targets freshly and
check Enter delivery when a doorbell is needed. Avoid scrolling a working
alternate-screen session; herdr rejects that operation.

## Verified disk cleanup: queue after labels

**br-hxcv4**, open P2, labels `verified-important`, `verified-local`,
`maintenance`, `disk-retention`. Read-only evidence:
`~/.local/state/mcp-agent-mail/stabilization/20261011-triage/disk-retention.md`.

- `~/Library/Logs/agent-mail`: 16 MiB, almost entirely `stderr.log`.
- `~/Library/Application Support/mcp-agent-mail`: 4 KiB cursor file.
- `~/Library/Caches/mcp-agent-mail`: empty. No matching Agent Mail reports found
  in `~/Library/Logs/DiagnosticReports`; this is not a full Library inventory.
- Live store: about 12 GiB; `doctor/reclaimable` is 10.2 GiB across 662 files
  (10,953,404,416 allocated bytes). Rotation moves backup generations to
  quarantine on the same disk and releases no capacity.
- Retained local state: about 21 GiB. Preflight is about 11 GiB, including
  8.9 GiB rollback; cutover rollback is 9.3 GiB. Upgrade copies retain recovery
  material again. Installed binary generations total 639 MiB.
- These are directory totals; APFS clones/snapshots can share physical blocks.

Next cleanup step is an exact inventory, exclusion of live paths, verification
of one retained restore generation, then explicit approval for exact deletion
scope. No deletion or move occurred. Moving into another local quarantine is
not proof of reclaimed disk capacity. Identify the backup-generation trigger
before implementing retention; inventory alone does not explain each event.

## Tracking and verification discipline

Beads executable:
`/Users/shz/.local/state/mcp-agent-mail/refactor/cli-modules-20261008/tools/beads/br`.
`.beads/issues.jsonl` is truth; keep one graph writer. Import after any pull;
collect mutations to terminal completion before retrying. Last graph cycle and
whitespace checks passed after disk tracking. No commit was made for those
latest changes or this handoff.

The operator's existing tracking-only exception covers unsupported-language UBS
exit 3 on root RESUME and Beads tracking; it grants no implementation scanner
approval. Do not silently extend that exception to publication or this new path.
No commit is required just to persist this note.

Root `RESUME.md` retains the detailed stabilization queue and historical
evidence. It contains old scheduling snapshots; use this current recovery and the
owner's latest reply to decide the next action. Keep user-facing progress tied
to completed code, tests and real-path acceptance. Avoid prolonged analysis,
repeated idle polling, duplicated approvals and status-only commits.
