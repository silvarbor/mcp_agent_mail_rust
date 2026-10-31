# Resume — local Agent Mail fork

## October 31: recipient-addressing follow-up recorded

The operator approved recording the explicit recipient-addressing contract in
`docs/SPEC-recipient-addressing.md`, then returning to the five-step fork
closeout. Recipient addressing is a separate follow-up after PR #4; do not add
its implementation to this qualification effort. Existing string recipients
retain their meaning; new objects explicitly select `name` or `display_name`.
No namespace guessing or fallback is permitted. Remaining interface decisions
and future acceptance criteria are recorded in the specification.

Resume closeout with fresh PR #4 CI/review inspection. Last observed branch
has local commit `0e0f96c5` ahead of the published head, fixing the ATC gate's
temporary-directory permission failure. Reuse unchanged local Rust evidence
and the shared build cache; this specification changes no runtime code.
The five-step goal instructions are in the file referenced by the runtime goal.
Agent Mail was started on October 31 using the existing launchd configuration;
it remains intentionally disabled at login. Do not change that setting or
restart the live service as part of specification recording or PR inspection.

## October 28: active fork consolidation

LOCAL QUALIFICATION PASSED. Final run
a0d7c2b3-6860-4488-9f35-03f90cb83f17/session 47636 completed with matching
started=true/status=completed/childExitCode=0. All-target check 45.19s,
production check 26.23s, strict Clippy 51.13s; final test build 8m40s, selected
worker recovery test passed in 0.361s. Aggregate scoped evidence: the 3252
passing broad-regression cases plus the repaired three worker cases, with
49/50 ACK cases passing in the preceding focused run and the last case now
passing. No unaddressed test failure remains in the selected 3255-case scope.
The earlier 20-case path/recovery/router run and real-process display-name,
receipt, archive, integrity and two-restart acceptance remain valid for their
unchanged implementation paths. Formatting/whitespace checks pass.

All code/test imperfection entries below are repaired and verified. The
macOS fixture environment is documented in AGENTS.md; explicit symlink
rejection cases remain enabled and passed. Workflow validation also passes:
actionlint on notify-acfs.yml and publish.yml. Publication guidance now matches
the manifests' disabled crate publication and dependency overrides, and the
reported SC2129 output-redirection style issue is fixed. No heavy command is
running. Remaining work is commit/PR/merge, installed-source provenance and
accounted-for cleanup; preserve the shared compilation cache.

Run b243ecc0-3501-4b8d-b439-a14ec8ee797e, session 56815, completed with matching
record started=true/status=completed/childExitCode=100. Compiler all-targets
passed in 1m29s, production in 50.76s, Clippy in 1m45s. Test build took 17m29s;
3255 tests ran in 439.834s: 3252 passed, three failed. All nine diagnostic
failures and the other two formerly nested ACK fixtures passed. New failures
are ACK worker recovery tests at worker.rs:204 (expected error stage) and :294
(empty selected seed row), listed in repaired-functional-tests.log. Classify as
safe local investigation/repair under Rust test/code workflow; trace all three
before any next build. No guarded command remains running.
The three failures are traced: worker seed() migrated the generation table
without calling the lazy generation initializer, and the blocked-parent case
expected a pool-construction error even though no connection opens until the
first page read. The fixture now calls the real db_generation_id seeder after
releasing its connection, and the error assertion checks the actual frozen
database authority refusal. Production worker behavior is unchanged. Formatting
and whitespace checks pass. Follow-up run
8bd9bb57-53e0-413d-8c9e-9d6b41f6f458 keeps workspace build selection/features
and the shared target, runs the required compiler/lint checks, then only the
ACK test module. Other passing regression results remain valid. Logs use ack-*.
Run 8bd9bb57-53e0-413d-8c9e-9d6b41f6f458/session 90404 completed with matching
started=true/status=completed/childExitCode=100. Required compiler/lint checks
passed (46.25s, 27.67s, 52.45s); test build took 8m48s; 49/50 ACK tests passed
in 12.884s. Remaining failure: worker.rs:210 over-specific backend error text.
After proper generation seeding, the same blocked-parent fixture reports
"failed to read overdue ACK page: ... unable to open database file" rather than
the earlier frozen-authority detail. The prior repair substituted a backend
string without respecting lazy connection failure paths. Safe local test fix:
assert the worker's page-read failure category and existing preservation/
recovery observations; review the complete continuation before another build.
No guarded command remains running.

Final isolated worker assertion repair: match the worker-owned page-read
failure prefix, which both observed backend errors share, while retaining all
pool/state, preserved-file, normal retry cadence, restored delivery, stamped
artifact, and duplicate-grant assertions. Reviewed the entire remaining test
and the weak pool registry/direct archive publication paths. Formatting and
whitespace checks pass. Run a0d7c2b3-6860-4488-9f35-03f90cb83f17 repeats required
compiler/lint checks and executes only this changed test with unchanged
workspace features/target/nice/job limits. The other 49 ACK passes remain valid.
Logs use worker-final-* in the existing qualification directory.

Installed-source provenance check: current still selects
retention-thread-digests-3659c270-20261013. Its manifest.json explicitly records
an uncommitted working tree plus preserved tracked diff/new modules, rather
than a release tag. This is a non-blocking release-identity question under the
operator's fork-consolidation plan. Inspect the preserved source evidence to
determine whether an exact historical source tag can be recovered without a
new build; do not label the current binary as the new candidate or redeploy it.
All 11 recorded current-bundle source hashes match existing commit
4f9fcd0c167de87e35cc3de71cd7984482a9c344. The preserved tracked diff matches
the manifest's SHA-256 exactly. The current server is copied from the preceding
retention-3659c270-20261013 bundle; that bundle's recorded source differs only
in CLI retention.rs. Both bundles retain source/implementation.diff and the two
then-new source modules. Do not infer one exact per-binary source tag for the
mixed bundle without accounting for that difference.

Cleanup observation: the October 13 retention evidence directories still
contain old before/admission/after SQLite copies and an archive tree, and the
preceding installed retention bundle still exists. These need exact inventory
and live-path exclusion under the operator's eight-hour rollback limit. Keep
the small manifests/source evidence needed for release provenance distinct
from rollback databases, archive copies, and superseded binaries. Inspect and
resolve this after the active compilation, avoiding competing disk work.

Fork workflow repair: notify-acfs.yml had an unconditional job that runs on
Cargo.toml pushes to main, then fails because this fork is not its hardcoded
upstream source repository. Added a job-level upstream repository condition;
the existing dispatch authorization checks remain intact. This is an
upstream-only notification job, separate from functional CI. Safe local
workflow fix under fork consolidation; validate YAML without a Rust rebuild.

Reviewed follow-up batch (uncommitted): embed Rust's resolved project slug in
the generated guard's metadata-checked candidates; construct the collision
fixture under that actual slug; move ACK fixture seeding before the isolated
default-root scope; distinguish current display labels from immutable message
history after both renaming and clearing; use Unicode filesystem fixtures on
macOS while retaining invalid-byte coverage elsewhere; keep both repaired
retention TempDirs alive through assertions and then clean them automatically.
The existing pure non-UTF8 identity classification still runs on all Unix.
Updated nextest's stale claim that fixture hangs were impossible and the
guard collision docstring to match its warning behavior. Formatting and
whitespace checks pass. These repairs await compiler and regression evidence.

Next qualification run: b243ecc0-3501-4b8d-b439-a14ec8ee797e, one guarded
sequential batch using the unchanged target/toolchain/features, two Cargo jobs,
two test workers and nice 19. Checks: all-target compiler, production compiler,
strict all-target Clippy, broad functional nextest plus all ACK and symlink
cases. Canonical TMPDIR selects a real temporary directory for fixtures that
deliberately reject symlinked storage ancestors; explicit rejection tests stay
selected. Logs use repaired-* in the existing qualification directory.

Active broader functional regression run: session 50337, orchestration run
96f7f394-a4b7-440d-8248-e9cbce702352. Locked workspace nextest, two workers,
two Cargo jobs, nice 19, unchanged target and features; --no-fail-fast collects
all selected failures before any repair. Log:
`/tmp/mail-fork-qualification.XTHjZ8kE/functional-tests.log`.
Selection covers labels/display, receipts, session identity/registration,
retention/reconstruction/ID floors, startup-page checks, shared mailbox and
inbox behavior, CLI JSON/conformance, recovery/authority/backup/snapshot,
mail UI/compose and the guard package. Collect this same handle to terminal.
Admission completed. Nextest selected 2882 tests; only the changed conformance
target compiled, in 16.52s. The run has reported doctor_orphan_refs.rs:907
successful_apply_keeps_a_complete_pre_repair_snapshot as failing. Record the
complete terminal result before classifying or repairing the batch.

Broader run terminal: matching run record, started=true/status=completed,
childExitCode=100. In 518.796s: 2662 passed, 219 failed, one timed out
(ack_ttl::tests::recovery_contention_preserves_ack_tail_and_resumes_real_escalation).
No source edits were made during the run. Many errors explicitly reject /var
symlink prefixes; several fail creating non-UTF8 filenames on macOS.

Diagnostic session 22069, run 543a58fd-86eb-4f5d-af3f-31ec046ea45e, captures
metadata from the unchanged cached build, then uses nextest --binaries-metadata
and --cargo-metadata to rerun all 220 failed/timed-out names plus explicit
symlink/path-identity checks with canonical TMPDIR. This is a cause-isolation
experiment, not final qualification. Paths under the same /tmp qualification
directory: binaries-metadata.json, cargo-metadata.json, failed-and-alias.filter,
metadata-build.log and canonical-temp-diagnostic.log. Collect this same handle.
Metadata capture finished from cache in 0.86s with no Rust compilation.
Nextest selected 647 tests including explicit symlink/path checks.
An isolated one-second sample of ACK child PID 95290 confirms the nested
fixture mutex wait: isolated-root scope -> seed_unacked_message ->
with_process_env_overrides_for_test -> Mutex::lock -> __psynch_mutexwait.
Evidence: ack-fixture-lock.sample.txt in the same qualification directory.
Sample command session 86072 completed successfully. The test remains owned
by nextest's process-group watchdog; no process was signaled manually.
Diagnostic terminal: matching run record started=true/status=completed,
childExitCode=100; 647 tests in 256.090s, 638 passed, eight failed, one timeout.
The temporary-root experiment resolves 211 of the previous 219 failures.
Remaining failures: five physical non-UTF8 filename fixtures unsupported by
macOS, two guard cases, display-name history equality, and the sampled ACK lock.
Explicit symlink rejection remains covered by this diagnostic. No final
qualification claim follows from this environment-isolation experiment.

Current source head: d7566cc72343113601abcab08fc4c86152afa201. The final
notification edit removes only a redundant borrow. Formatting and whitespace
checks pass. Compiler qualification is session 88847, orchestration run
4ff06ffd-e868-445f-9aaa-750d6f8df3c2: sequential locked workspace all-target
check, production lib/bins check and strict all-target Clippy, nice 19, two
compiler jobs, unchanged shared target. Logs are in
`/tmp/mail-fork-qualification.XTHjZ8kE/`. Session 88847 completed with matching
terminal record and childExitCode=0. All-target check passed in 1m00s,
production check in 39.93s, strict Clippy in 1m11s.

Real-process functional acceptance session 50079 completed with matching run
831c4baa-3060-48e8-a52c-a092b81ec45d and childExitCode=0. Three messages and
both labeled profiles retained exact identity/content/receipt state across
two restarts; cold database integrity/foreign-key and archive checks passed.
Evidence: preservation qualification/functional-f23e7d18-20261028/result.json.
The 0.0s active-duration figure is expected in functional-only mode: it runs
the three scenario phases without the optional sustained-load workers.

Latest repair test state: session 25645 completed with child exit 100 and a
matching orchestration terminal record. Compilation took 12m20s; 19 selected
tests took 18.567s, with 15 passing and four failing. Both backup refresh repairs
passed; crash replay passed database promotion and next failed project lookup.
Scorecard outside-run fixture encountered the macOS parent symlink first;
conformance still lacked the expected signal; macOS rejected creation of the
invalid-byte filename in the admission fixture.

Second repair batch: session 44635, orchestration run
8cf5bd3a-5d3a-4c92-823e-231d5d4c7fb4. Same workspace/features/target, nice 19,
two compiler jobs and two test workers. Terminal record matches the run ID,
started=true, status=completed, childExitCode=100. Compilation took 12m47s;
19 tests completed in 18.576s: 17 passed, two failed. Remaining safe-local-fix
ledger entries under Rust correctness qualification: conformance.rs:2377 still
lacks CalmRiver.signal; integration_runs.rs:4951 now reaches search backfill
but reports stale instead of fresh. Inspect emitted artifacts and real paths
before another build. The other selected path and recovery repairs passed.

Third repair batch: session 17693, orchestration run
a8c962f3-6c89-434b-8e69-f741dfbd5628; completed with matching terminal record,
started=true, status=completed, childExitCode=0. Compilation took 12m39s;
all 20 selected tests passed in 23.532s. Includes
snapshot_pool_bound_to_mailbox_path and the prior 19-test selection. The
notification root is now canonical before configuring signal writes: storage
ensure_dir rejects symlinked parent prefixes. Search snapshot binding now
normalizes the mailbox path once; the failing artifact showed `/var` compared
against `/private/var` despite identical source/index counts and a real result.
The existing snapshot identity test now binds through a dot-component alias.

Archive staging content check: StagedArchiveWrites originates in upstream
f01409e4, already included in selected upstream 92fd8025. Its presence is not
evidence that the later fork optimization series was selected. Storage lib's
delta from that upstream baseline is retention backlog admission, dead helper
removal, formatting and a portable lock fixture. Optional later fork staging,
flush and ATC reuse commits remain absent by ancestry and patch equivalence.

Second-round edits: canonical scorecard foreign-root fixture; canonical project
key in the archive-only crash-replay seed; notification test uses the slug
returned by ensure_project. Invalid-byte physical-file admission stays enabled
on non-macOS Unix, matching the existing backup-enumeration platform contract;
new missing-path identity coverage runs on every Unix, including macOS. It
asserts two lossy-colliding names remain distinct and retain the canonical parent.
No production admission guard or scorecard rejection was weakened.

README now contains the fork inventory, integration-PR workflow, exact-source
tag/manifest policy and an explicit distinction from upstream quick-install.
The README documentation finding below is addressed. Focused prose audit
2026-10-28_15-08-25-190-06c835618c4c approved commit a30d2db6. The subsequent
code checkpoint f23e7d1861cf307d8f8b97df1c34c7fdb921f7ef changes only the five
path-identity implementation/test files. Its focused verification is pending.
Formatting and whitespace checks pass after this round.

Cleanup accounting: agent-mail, work/agent-display-names and
work/automatic-position-labels have no commits missing from this candidate.
Retire them after delivery. The upstream-integration worktree is still dirty:
startup audit fixture work is superseded by candidate 82d515bf; its export
comparison helper, benchmark, manifest constant and regression remain a separate
unfinished concern. Preserve those files until their disposition is recorded.
The optional ATC append reuse helper is absent from candidate queries.rs;
server lib's upstream delta is retention shutdown/admission only.

The operator replaced the paused goal with the six requirements in
`/Users/shz/.local/state/codex/accounts/outcomeeng/attachments/24e2fcac-acb4-4ac0-8f8c-cfd6b471b09d/pasted-text-1.txt`.
Maintain the stable fork on main; tag exact installed sources; preserve upstream
ancestry through integration PRs; qualify this candidate and account for its 454
commits; keep the CPU repair separate; inventory deliberate differences; retire
accounted-for branches/worktrees. This supersedes the earlier stand-down note.

GitHub confirms origin's parent is Dicklesworthstone/mcp_agent_mail_rust.
Added that repository as upstream and fetched upstream/main without tags.
No branch was merged, reset, published, or deleted.

History accounting at candidate 52ce18eb and upstream 9c31bc19:
`main..HEAD` contains 454 commits; excluding upstream/main leaves 46.
Thus 408 are shared upstream ancestry. This is provenance accounting, not
acceptance of every upstream behavior. Upstream currently has 397 commits
absent from the candidate; the candidate has 70 absent from upstream, including
fork commits already on main. Do not pull the additional 397 into this release
without selecting and qualifying that synchronization separately.

Fork-only additions since main fall into these groups:
- Profile labels: 53bb3180, f1218033, and their fixture/acceptance follow-ups.
- Retention and WAL correctness: 4f9fcd0c, 3996e7fc, a4d99300.
- Shared mailbox and inbox ordering: 895554ca, 8f433743.
- Identity creation/binding: 6f8a7d60 and 84b7ff8a.
- Upstream integration: 3adb7338; inspect its conflict resolutions separately.
- Installer, dashboard, CI, conformance, and recovery qualification repairs:
  be588245 through 82d515bf as enumerated below.
- Operational tracking and bounded build artifacts: remaining fork-only commits.

The following optional optimization commits are confirmed absent from candidate
ancestry: 6cf0a3b1, 4a63da7a, 27c1efd0, 5ada6dcd, a1420a14, 1303d39c,
646a9345, 6bd2ab22, 8c7a0251, bc037791, abe8eecc, 9b1d0499, 04a33f8d,
495447dd. Content review remains required to exclude equivalent patches.

October 28 initial repair qualification: session 25645 and orchestration run
4feb47f4-6cc9-4476-98c5-452b6c3d8d64 are terminal; see the result above.

Repair design: existing scenario tests exercise real filesystem/SQLite and
the real router, with no substitutes. Backup replacement uses the independently
seeded primary marker as oracle; skipping replacement fails its content check.
Scorecard consumer uses the exact receipt and a conflicting foreign artifact;
selecting by timestamp fails readiness. Notification checks compare emitted
signals to the registered recipient/project and sent message; suppressing signal
emission fails file existence. Recovery crash replay already failed for missing
primary aliases; the production repair reuses the existing lossless parent-prefix
normalizer. Cross-database and non-UTF8 admission rejection tests remain selected.

Correctness batch 33295 is no longer running: process inspection found no
Cargo, rustc, or nextest process. Its log ends at 427/1134 with no terminal
summary. It is incomplete evidence, not a passing suite.

Current imperfection ledger (safe local fixes under Rust test/code and
qualification workflows; investigate and repair before publication):
- guard/src/lib.rs:1178-1221 derives only legacy path slugs in the generated
  hook, while Rust identity can use hashed Git slugs. The installed-root test
  at :6206 confirms a reservation is missed when project.json is absent.
  Safe local correctness repair under Rust/Python code guidance: carry the
  resolved install-time project slug into metadata-checked candidate matching.
- guard/src/lib.rs:6248 assumes two paths still collide under Git identity;
  construct foreign metadata under the actual candidate slug to exercise the
  intended collision behavior. Safe local fixture repair under Rust tests.
- tools/tests/auto_register_profile.rs:560 compares dynamic recipient labels
  as immutable message history. Assert the current label separately and
  compare the remaining message data. Safe local fixture repair under Rust
  tests; production labels must continue updating after relabeling.
- recovery_retention.rs:2342 and :2608 physical non-Unicode fixtures retain
  temporary directories with TempDir::keep, including a sparse 24 GiB file.
  Safe local fixture-lifetime repair under Rust test guidance: retain TempDir
  handles through assertions so fixture data is cleaned up on scope exit.
- ACK timeout has a concrete nested-lock path: ack_ttl.rs:1696 enters
  with_isolated_default_storage_root_for_test, then seed_unacked_message calls
  ensure_ephemeral_test_project at :554, which enters
  with_process_env_overrides_for_test again. config.rs:3611 holds the
  non-reentrant TEST_SERIALIZER throughout both closures. Safe local fixture
  repair under Rust test guidance; include in the next reviewed repair batch.
  Preserve source unchanged until the reusable-binary diagnostic completes.
- Broader-run reported failure at doctor_orphan_refs.rs:907, test
  successful_apply_keeps_a_complete_pre_repair_snapshot. Classification awaits
  terminal run 96f7f394-a4b7-440d-8248-e9cbce702352; collect all failures and
  trace their causes together before any further Rust edit/build.
- The same in-progress log also reports failures in core agent_detect runtime
  discovery, pool validated ATC paths, recovery_retention non-Unicode inventory,
  guard hook/installed-root fixtures and server backup_rotation fixtures.
  Exact test names and stderr are retained in functional-tests.log. Treat as
  one pending diagnostic batch under Rust test/code qualification; inspect
  canonical temporary-root handling and filesystem capabilities before edits.
- External harness adapter path: fixed by a required --adapter input, resolved
  before creating artifacts and recorded with its hash. Help, Ruff and mypy
  passed; the real functional acceptance passed using installed 0.17.1.
- External harness optional load-mode adapter grammar: fixed to operation
  `list` and `allRecords`, matching installed 0.17.1. No sustained performance
  load was run or claimed; functional mode does not execute this path.
- Conformance notification fixture redundant borrow at signal_root: fixed
  after session 17693 completed; compiler/lint qualification remains.
- CLI release scorecard temporary-path fixture: fixed; all selected scorecard
  checks passed in session 44635.
- Startup crash replay recovery admission and search identity: fixed; complete
  crash-replay scenario passed in session 17693.
- Conformance router notification fixture: fixed; complete router fixture
  passed in session 17693.
- Backup refresh temporary-path comparisons: fixed; selected backup checks
  passed in session 44635.
- The incomplete qualification run must be completed after these failures are
  resolved; preserve completed compiler evidence and avoid duplicate builds.
- README.md upstream/fork installation distinction: fixed, with tagged-source
  installation and manifest policy; prose audit approved a30d2db6.

Operator instruction: run all future tests with `nice -19` (equivalently
`nice -n 19`, low CPU priority). Apply this to test runners in addition to
bounded Cargo jobs and the required load guard.

Compiler batch 50945 COMPLETED exit 0 on candidate `82d515bf`:
workspace all-target check 5m52s; production lib/bins check 2m27s; strict
all-target Clippy 1m21s; formatting passed. Older waiting notes below are
historical. Runtime correctness and acceptance remain pending.

Historical incomplete correctness batch: exec session **33295**.
Command uses `nice -19 cargo nextest run --locked --workspace --test-threads 2
--no-fail-fast`, Cargo jobs two, shared upstream-integration target. Filter
selects labels/display, receipts, session identity, retention/reconstruction,
ID floors, startup-page audit, shared mailbox/robot inbox ordering, direct and
daemon inbox fixtures, portable recovery fixtures, conformance, isolated guard/
JSON fixtures and WAL/authority/maintenance lock regressions. This is a focused
correctness suite; do not describe it as an unfiltered workspace pass.
Log: preservation root `functional-correctness-82d515bf.log`. The process ended
without a captured terminal summary; see the incomplete-evidence record above.

## October 27: functional release candidate

Authority: operator goal attachment
`/Users/shz/.local/state/codex/accounts/outcomeeng/attachments/c0b126b6-be33-4b88-9ee7-c972dcfe0b2a/pasted-text-1.txt`.
This checkpoint supersedes the historical next steps below.

Candidate worktree: `/Users/shz/Code/silvarbor/agent-mail/functional-release`.
Branch: `work/functional-release-20261027`, based on `a91eead5`.
Current implementation checkpoint: `82d515bf3dbaa5a51510fac534dfae333a0a51be`.
Carried installer repair `e6ed9b1d` as `be588245`, dashboard lock alignment
`0e0af4df` as `50f78e86`, and label rendering repair `8ecd60e2` as `1337cf8b`.
All applied cleanly; diff whitespace check passed. Additional selected repairs:
`cefa81c5` -> `d46abb92`, `de6f142a` -> `fc19b0bc`,
`c4ee59fe` -> `3034c2ec`, `29971c31` -> `7fba3ab6`,
`6a03d2b8` -> `3b199000`, `9e24c807` -> `79ef3cbf`.
These repair label/recovery fixtures, environment isolation, macOS ref fixtures
and child-test selectors. Confirmed the base already declares `pool_impl`, so
the selector repair is required independently of optional performance changes.
CI and evidence repairs: `c48d05fa` -> `c9e304e1`,
`0b148f41` -> `cb2e1711`, `7477ecc0` -> `fceab518`,
`4975bb71` -> `29a70a4f`, `fe99e8d9` -> `d76610da`,
`8d5f6635` -> `92880e6b`. Inspected the actual patches before selection.

Passing on this candidate: observability drift scanner (49 events, 10 schemas,
zero findings), all six scanner unit tests, artifact-schema E2E (14 assertions,
zero failures/skips, including tamper rejection and delegate-exit preservation),
and diff whitespace. Artifact evidence:
`/Users/shz/.local/state/mcp-agent-mail/integration/20261015-preservation/functional-release-artifacts-20261027`.
E2E session 81830 completed exit 0. No active process remains from those checks.
Compiler batch started in exec session **50945** behind wait-for-load; initial
collection returned no output, so admission/build status is not yet known.
Collect that same handle; do not start a duplicate. The sequential batch is
`cargo check --locked --workspace --all-targets`, production lib/bins check,
strict workspace Clippy, then formatting. Cargo jobs are bounded at two and
target directory is the existing `upstream-integration/target` cache.
Latest collection of 50945: handle remains live, no waiter terminal record or
compiler output. Continue collecting it; no second guard/build was launched.

Acceptance coverage inspection: tools `messaging.rs` has
`display_labels_follow_current_profiles_without_changing_mail_identity` and
`fetch_inbox_live_read_receipts_preserve_peek_and_ack_state`; DB `sync.rs` has
`agent_display_names_keep_duplicate_labels_historical_profiles_and_canonical_addresses`;
CLI `retention.rs` has
`retention_preserves_boundary_relationships_receipts_and_allocator`.
These are targets for the batched correctness run, not fresh passing evidence.
The existing external `qualification/runtime_workload.py` already starts real
servers, performs two restarts and audits exact message/receipt continuity via
isolated cold copies. It does not assert relabeling, peek-state preservation or
label persistence across restart. Extend that existing qualification before
claiming the requested complete functional acceptance; do not rerun its old
hour-long load test as a substitute. The stdio identity shell suite likewise
does not supply the requested label/restart acceptance.

October 27 functional qualification extension is now implemented in the existing
external `qualification/runtime_workload.py` via `--functional`. It registers
two distinct labeled identities, sends three messages, verifies repeated peeks
leave receipts empty, explicitly reads two and acknowledges one, relabels the
sender before restart and the recipient between restarts, and compares exact
identity IDs, message IDs, bodies and receipt timestamps across both restarts.
Cold-copy canonical integrity/foreign-key/message checks remain; functional mode
also compares stored identity labels and the archived profile labels. The normal
load mode remains available, while functional mode schedules no polling traffic.
Ruff, strict mypy and CLI argument parsing pass. Real behavior is NOT RUN;
use the final candidate binaries, an unused isolated run root and `--functional`.
Qualification code still requires independent review before release claims.
Compiler batch 50945 remains live with no admission record at latest collection.

Recovery-test repair committed as `82d515bf`: `startup_page_leak.rs` now audits
private physical copies through the existing staging API. Seeding, startup and
restart run in separate processes, using the existing `libtest_path!` helper
and completion witnesses to reject zero-test child runs. The parent retains
the original in-place canonical VACUUM on the same database file after the seed
process exits, rather than the earlier uncommitted fresh-inode proposal.
Message and recipient counts are checked through compaction, first write and
restart. The governing scenario is GH#291; subject is create_pool plus startup
integrity and first pooled write; independent oracle is canonical SQLite on
physical family copies. A leaked page fails integrity; a lost insert fails
the row-count predicates. This is real-engine scenario evidence with no doubles.
Formatting and whitespace pass; compiler and runtime results are NOT RUN.
The compiler batch still had no admission output after this edit, so no build
result is being reused across a source change. Run the focused test under
nextest's process-group timeout before the wider correctness batch.

Installer unit exit regressions pass on the candidate: 52/52 under Bash 5.3.20
(session 15672, exit 0), and 52/52 under macOS stock /bin/bash 3.2.57
(session 46430, exit 0). These cover file/stdin entry, empty/completed/rolled-back
history, explicit failures, signal exits, recovery failure and required-client
failure. Artifacts are `functional-installer-20261027` and
`functional-installer-stock-bash-20261027` under the preservation root. This is
focused installer-handler evidence, not a completed candidate installation.
Retention coverage inspection confirms the existing CLI retention scenario
exercises interrupted archive moves, startup/reconstruction refusal during an
incomplete prune, idempotent resumption and the retained message-ID floor.

Observation preparation: external `qualification/observe_service.py` now reports
`release_observation_passed` only after at least 24 hours with no probe errors,
critical samples or binary change. Samples explicitly classify nonzero dispatch
timeouts, integrity failures, unavailable/ambiguous owner, PID changes and changed
binary targets as functional failures. CPU/latency numbers remain measurements.
Ruff and strict mypy pass. Real unreachable-service preflight completed in
1.025 seconds with 2 samples, 8 probe errors, 2 critical samples and
`release_observation_passed: false`; evidence directory is
`qualification/functional-observer-unreachable-20261027`. This tests failure
reporting only, not the candidate's stability. No live-service call was made.
Current external qualification SHA-256 identities:
- runtime_workload.py: `0a6eb661e498ac4ba1590578d301921d277734ca256bcf7e5fa7dca0caeea81b`
- observe_service.py: `6477eda9e6f9dd6518238bfdbd8f335725983dcff2e2fb364599fc7f69751054`
The observer remains read-only. Cutover must arrange removal of its exact rollback
material before eight hours; do not wait for the 24-hour observation to clean it.
Compiler session 50945 was admitted after 1,644.174 seconds (six wait cycles).
Waiter terminal status `ready`, 12 CPUs, final normalized loads
`[0.69397, 0.71301, 0.90670]`. Workspace all-target checking has started on
candidate `82d515bf`; output confirms FrankenSQLite revision `3659c270`.
Collect session 50945 through all sequential checks; do not duplicate it.
Compiler/lint, functional acceptance and retention/recovery verification remain
NOT RUN on the assembled candidate. No installation, restart or mail mutation.

Keep display labels throughout registration, persistence, read surfaces and
recovery; stable identity/receipt state; qualified WAL repairs; retention and
message-ID preservation; inbox ordering and delivery authority. Keep necessary
installer and test-harness repairs. Assemble remaining correctness carry-forwards
before running a batched validation. Inspect actual patches, including fixture
changes, before selecting them.

Optional Apple flush batching, grouped/parallel synchronization, staging/path
caches, ATC connection/schema caching and performance diagnostics remain on
`work/upstream-integration-20261015`; that worktree's dirty changes are preserved.
No broad rollback or deletion is needed to assemble this candidate.

Next: inspect later functional test/CI fixes and the child-process test selector
repair, establish release checks separately from performance reporting, then
implement/run real label/send/peek/read/ack/restart and retention/recovery
acceptance. Compiler, lint and correctness checks are required. Corruption,
lost messages, hangs and timeouts remain blockers; missed speed targets do not.
After review and supervised installation of the exact candidate, observe normal
use for 24 hours. Rollback retention maximum is eight hours. Reuse existing build
artifacts where compatible and keep compilation bounded; no active build handle.

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
