# Resume — local Agent Mail fork

Updated 2026-10-10. This is an operational handoff. Beads owns the work plan;
this note records how to resume it and where its evidence lives.

## Start here

1. Read the current `AGENTS.md` and the operator's conversation instructions.
2. Inspect `git status --short --branch` and recent commits. Preserve other work.
   The current branch is `work/automatic-position-labels`; this checkpoint changes
   tracking and this note only. Native source remains pinned at
   `1f309419df08062942ae2f0c29b0990cf631cf5c`.
3. Read `br-vas72`, then `br-0egjn`. Finish the remaining label acceptance before
   starting the stabilization implementation, unless new evidence establishes
   an immediate runtime risk.
4. Select at most two local implementation items. The first stabilization task
   is `br-vas72.1`: bounded isolated triage of three serious historical reports.
   The selected baseline repairs can follow; avoid a repository-wide rewrite.
5. Before mailbox work, use the repository's cheap doctor health check. Discover
   current service ownership before any maintenance; PID values from old notes
   are stale. This planning checkpoint authorizes no restart or mailbox repair.

There is no root `spx/` tree in this fork. Use the explicit operator contract and
Beads. Formal spec-node audits need a governing node and assertion links; do not
invent a tree or claim such an audit passed here. Language, commit, contribution,
and mail-operation skills still govern their applicable operations.

## Beads plan and confidence labels

The plan is **br-vas72: Stabilize the local fork with verified debt priorities**.
Its description contains the sequence, owners, evidence rules and completion
conditions. The fork execution owner is Agent Mail Maintainer. Existing records
retain their ownership, priorities, statuses and dependencies.

Before this plan, the complete JSONL inventory contained 176 non-terminal
records: 67 open, 95 in progress, 12 blocked and two deferred. An earlier reported
99-record count was incomplete. Four planning records were added; a backlog
count is never a count of confirmed current defects.

| Label | Meaning | Initial records |
| --- | --- | --- |
| `debt-verified` | Dated local observations have retained evidence; each bead states the evidence date and limits. | 6 |
| `stabilization-selected` | Selected work for this bounded pass; this also marks the plan and first triage task. | Five existing baseline repairs |
| `debt-revalidate` | Historical or unclassified report requiring current-build evidence. This can include feature and release tasks. | 164 |
| `debt-risk-first` | High-impact historical report to probe first; it remains unverified on the current build. | 3, also `debt-revalidate` |
| `debt-stale-candidate` | An older dependency transition appears superseded; inspect its complete record before changing status. | 3 |
| `stabilization-current` | Label workflow already underway, with its own completion criteria. | `br-0egjn` |
| `backlog-approved-deferred` | Optional work with existing operator-approved revisit conditions. | `br-5epfd`, `br-6wo0t` |
| `debt-plan` / `debt-planned` | Scheduling records; these labels assert no defect or passing probe. | Epic / three new tasks |

`debt-verified` on the four server-test beads means their 2026-10-09 logs were
re-read on 2026-10-10. Tests were not rerun for this planning checkpoint. A new
implementation session must reproduce the affected cases against its current
source before fixing them. Promote a revalidation record only with exact
source/binary identity, environment, command, result and evidence location.
Close or supersede a stale candidate only after checking its notes, dependencies
and current code; retain history. Existing P0 labels remain intact.

Use this available Beads binary:

```sh
BR=/Users/shz/.local/state/mcp-agent-mail/refactor/cli-modules-20261008/tools/beads/br
"$BR" show br-vas72
"$BR" list --label stabilization-selected
"$BR" list --label debt-verified
"$BR" list --label debt-risk-first
"$BR" list --label debt-stale-candidate
"$BR" list --label debt-revalidate
"$BR" dep cycles --json
```

`.beads/issues.jsonl` is truth; `beads.db` is disposable. Run `br sync
--import-only` after a pull. Keep one graph writer. After mutations, verify the
JSONL classifications and graph cycles; consume complete command results before
deciding whether a mutation succeeded. A yielded command session needs collecting,
not a duplicate mutation.

## Stabilization sequence

1. **Critical triage — br-vas72.1.** Read and probe `br-2hpuk` (migration/short-row
   reads and migrating read-only fallback), `br-kp1in.13` (archive-drain stall),
   and `br-kp1in.33` (concurrent-send SIGBUS). Use bounded isolated stores and
   pinned binaries. A reproduced corruption, crash or permanent stall takes
   priority over further upgrades/features. A successful short probe establishes
   only its stated bounds. Read the old repro before choosing a current one.
2. **Baseline repairs.** Reuse `br-mxkly` (canonical temporary roots), `br-wcysp`
   (non-UTF8 filesystem evidence), `br-cn35f` (shared recovery admission state),
   and `br-jc50n` (disabled-worktrees ATC observation contract). Complete
   `br-rpa0b` for the portable suite-rule source and copied-logic tests. Preserve
   symlink rejection, recovery admission and meaningful filename coverage. Run
   focused cases and then the full affected server suite. Skip no failing tests
   and weaken no invariant to obtain a passing result.
3. **Installed acceptance — br-vas72.2.** This task is blocked by the triage task
   and five baseline repairs. Revise suitable existing harnesses for real
   registration/authenticated sends, inbox/events, relationships, graceful
   restart/durability and archive drain under modest mixed load. Browser checks
   use 1,000 messages, unchanged/changing content and failed refreshes; preserve
   automatic updates and bound requests/resource trends. Include daemon-owned
   reads and offline fallback. Declare workload, bounds and deadline first.
4. **Archive diagnosis — br-b66ku.** Before archive/mailbox maintenance, inventory
   both authorities and all affected identities read-only. A bounded scan of
   the service stderr log on 2026-10-10 found 5,394 warnings across 70 distinct
   agent IDs, lines 28367–41060, through 09:04:38Z. The original four-ID report
   described one startup observation. Both authorities were preserved. The
   approved diagnostic deferral authorizes no repair or bulk re-registration.
5. **One module extraction — br-vas72.3.** Blocked by installed acceptance. Choose
   a cohesive boundary from actual change/review friction. Measured on 2026-10-10:
   CLI `lib.rs` 60,492 lines; server `lib.rs` 37,294; DB `queries.rs` 36,254.
   Characterize production behavior before moving code. Keep structural moves
   and behavioral fixes in separate changesets. Reassess after one extraction.
6. **Reassessment.** Review the three stale candidates `br-y5ogm`, `br-ku0kl`,
   `br-t5mzd` against current dependency pins. Select further operational or
   build cleanup only when it helps this fork. The pass can finish while the
   remaining backlog stays open; the epic defines its evidence-based boundary.

Selected server evidence:
`/Users/shz/.local/state/mcp-agent-mail/ui-labels/20261009/logs/` contains
`ui-regressions-canonical-tmp.log` (4,929 passed, seven failed),
`startup-recovery-isolated.log` (isolated recovery case passed), and
`atc-observation-isolated.log` (isolated ATC case failed). The production
copied-logic locations are `messaging.rs:8523–8555`; the unavailable Linux
suite-rule pointer is `AGENTS.md:13–15`. Follow `br-rpa0b` to restore the
authoritative portable rule source without weakening the rules.

## Label delivery state

Stable names and IDs remain routing/authorization identities. Duplicate labels
are legal. Arbitrary labels never resolve recipients. Unicode/spaces, trimming,
blank clearing, omission-preserves, control rejection and the 128-scalar limit
remain native behavior. Native messages/events carry current nullable
`sender_display_name`; visible primary recipients have `to_display_names`.
CC/BCC privacy remains intact.

Native changes are installed locally at source `1f309419`. The earlier UI/CPU
work is in merged fork PR #2. Coding-agents **0.12.0** now contains Plugins Change
#414, merged at Plugins commit `5acfed14`. Director mail **10918** reports his
self-registration as Methodology Director on CobaltSpire (ID 180), resume
registration steps, and label-aware monitor/inbox consumers complete.

Holder mail **10639** cites the attested probe at Plugins commit
`f5cded5150ad486f02c637a77c1790ff1f6ab0c1`. The retained probe, inspected at
`5acfed14`, passed registration, immediate authenticated sends after startup and
rename, both inbox body modes, and labeled/unlabeled/delimiter-fallback formatting.
It explicitly did not test the adapter event stream, doorbell parsing, or actual
pane delivery. Native event regression coverage is separate. Collect the
Director's consumer commit/evidence paths and the final real-pane/parser
acceptance before closing `br-0egjn`; its whole-workflow checklist remains open.

Plugin labels live in the capability `response` beside immutable records;
`data.records` retains stable identities without label fields. The plugin carries
recipient maps on send results only. Ordinary pointers are
`[Label <StableName>] mail ID`; delimiter/control labels use the stable-address
pointer while the full label remains in native metadata. Position owners supply
their own assigned label; the operator maintains no secondary mapping.

## Mail coordination

Use the current installed coding-agents skills before mail/Prowl operations.
Version 0.12.0's adapter requires Python with `StrEnum`; this shell's default
Xcode `python3` is 3.9. `/opt/homebrew/bin/python3.13` successfully read the inbox
on 2026-10-10. Use that interpreter for the adapter rather than changing system
Python or falling back to raw mail commands.

Shared mail project working directory:
`/Users/shz/Code/outcomeeng/changes/changes.git`.
Own identity: Agent Mail Maintainer <GreenLake>. Do not re-register other owners
or print tokens. Peers: Methodology Director <CobaltSpire>, Plugins Maintainer
<ScarletGoose>, Plugins Contributor 2 <SilverHeron>. The Director authorized
direct holder coordination with ScarletGoose copied on each mail.

Peer worktrees are `/Users/shz/Code/outcomeeng/methodology/worktrees/director/`,
`/Users/shz/Code/outcomeeng/plugins/worktrees/maintainer/`, and
`/Users/shz/Code/outcomeeng/plugins/worktrees/contributor-2/`. Resolve fresh public
identities with the skill; do not infer panes from titles or read transcripts.
This session lacked a Prowl caller identity. The operator-authorized bounded
coordination exception was recorded in `br-0egjn`: uniquely matched public
worktrees, exact one-line source-generated doorbells, checked trailing Enter,
no fabricated caller or focus mutation. Preserve that scope; do not broaden it.

## Installed build, demo and validation

Frozen bundle:
`/Users/shz/.local/opt/mcp-agent-mail/automatic-labels-1f309419-20261009/bin`.
`am` version text is 0.3.36; distinguish this build by source and SHA-256:

- `am`: `84296033ef2dcfb60b0ffd1f153e672c0ed9dd167d6a24d1f49a723778002c23`
- `mcp-agent-mail`: `049e770a7fc676ea753a7c610c571d055df9ab6c008c4a332678ad63b214007d`

Live web UI: `http://127.0.0.1:8765/mail`. Supervisor: launchd `com.agent-mail`,
user UID 502. Do not dump its plist, environment or unfiltered supervisor output;
they contain the bearer token. The live mailbox is under
`/Users/shz/.local/share/mcp-agent-mail/git_mailbox_repo/`.

Evidence root:
`/Users/shz/.local/state/mcp-agent-mail/automatic-labels/20261009-1f309419`.
Read `validation.json`, `binaries.json`, `installed.json`, and `test-audit-context.json`.
The preserved prior bundle link and private database backup support reversible
maintenance; do not overwrite them. Installation verification retained 159
identities, 10,478 earlier messages, 13 reservations and 175 contacts.

Isolated demo helper: `<evidence-root>/demo/serve.sh`, port 8877, no auth.
Check its current process/port before starting it again. Store and agents are
retained. BlueLake and RedStone initially both had Reviewer; BlueLake became
Officer Alpha while its ID/address and message 2/event cursor 2 stayed intact.
Native rename/clear/collision proof is under `demo/native-*.json`. Plugin capture
and probe agents coexist there. Avoid deleting or re-registering other owners.

All required Cargo checks, production graph check, Clippy with warnings denied,
and formatting passed at native source `1f309419`. Focused label suites passed;
nine adjacent filters passed 106 test executions. This is not a full-workspace
test pass. The server's seven retained failures are the baseline repair queue.
No compiler or implementation tests were rerun for this documentation checkpoint.

Use pinned nightly executables from
`/Users/shz/.rustup/toolchains/nightly-2026-08-31-aarch64-apple-darwin/bin`;
put that directory first on PATH and set RUSTC/RUSTDOC explicitly so cargo-clippy
uses the same toolchain. Validation/builds run sequentially with two build jobs
under the operator's existing host-load override. Do not request it again.
For release stripping, scoped `DYLD_LIBRARY_PATH` pointing at that toolchain's
`lib` fixes its bundled rust-objcopy LLVM lookup; the repair is in `binaries.json`.

Preserve required checks after substantive source edits: workspace all-targets,
production `--lib --bins`, Clippy `-D warnings`, and formatting. Prefer the same
focused package set during tests to avoid accidental feature-graph rebuilds.
No full-suite passing claim, scanner waiver, or formal spec-node audit approval
is implied by the native evidence.

The operator requested local work. Publish no upstream PR, deployment or public
release as part of this plan. No file deletion or destructive Git operation is
authorized. Shared-store repairs require explicit authority, supervised draining,
backups and reversible hash-witnessed changes. Commit coherent local checkpoints
with normal hooks and explicit paths; preserve unrelated work.
