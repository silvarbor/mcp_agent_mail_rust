# Resume — local Agent Mail fork

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
Next: finish checkpointing notes, create isolated integration worktree, inventory
upstream/fork overlaps and merge while retaining required fork behavior.

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
