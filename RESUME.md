# Resume — local Agent Mail fork

Updated 2026-10-11. This is an operational handoff. Beads owns the work plan;
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
4. Select at most two local implementation items. The bounded assessment
   `br-vas72.1` completed on 2026-10-11; all three original P0 reports remain
   open. Migration mitigations passed; drain/crash runtime probes need Linux.
   Finish label consumer integration with its named owner, then select a baseline repair.
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

The 2026-10-11 triage evidence is
`/Users/shz/.local/state/mcp-agent-mail/stabilization/20261011-triage/triage.json`.
Three focused real-store regressions passed: v31 physical materialization,
older-schema robot refusal, and WAL-only initialized-family detection. The
installed binary refused `robot projects` against a new isolated synthetic
older-schema database with expected exit 1 and byte-identical SHA-256 before
and after. These results support the fork mitigations. They do not resolve
the upstream decoder defect or the original incident's live remediation.
Current drain/crash code was reviewed; the retained release-smoke harness and
original mapping/core capture need Linux `/proc`, unavailable on this Darwin
arm64 host. No runtime soak, crash qualification or shell-skip PASS is claimed.
`br-vas72.1` is closed as an assessment with documented blockers, as its
checklist permits; the original P0s retain status/severity and next actions.

Stable names and IDs remain routing/authorization identities. Duplicate labels
are legal. Arbitrary labels never resolve recipients. Unicode/spaces, trimming,
blank clearing, omission-preserves, control rejection and the 128-scalar limit
remain native behavior. Native messages/events carry current nullable
`sender_display_name`; visible primary recipients have `to_display_names`.
CC/BCC privacy remains intact.

Native changes are installed locally at source `1f309419`. The earlier UI/CPU
work is in merged fork PR #2. Coding-agents **0.12.0** now contains Plugins Change
#414, merged at Plugins commit `5acfed14`. Director mail **12674** supersedes
the consumer completion report in **10918**: the monitor still prints stable
mail names, and the startup/resume scripts supply no label registration step.
Read-only inspection of the named scripts confirmed this. Native and Plugins
delivery is retained; consumer/startup integration remains open. The Director
identifies Plugins as its owner in 12682.

Holder mail **10639** cites the attested probe at Plugins commit
`f5cded5150ad486f02c637a77c1790ff1f6ab0c1`. The retained probe, inspected at
`5acfed14`, passed registration, immediate authenticated sends after startup and
rename, both inbox body modes, and labeled/unlabeled/delimiter-fallback formatting.
It explicitly did not test the adapter event stream, doorbell parsing, or actual
pane delivery. Native event regression coverage is separate. Collect the
Director's consumer change/evidence paths and normal startup/monitor acceptance
before closing `br-0egjn`; its whole-workflow checklist remains open. On
2026-10-11 own registration preserved GreenLake ID 267; the next authenticated
send returned its label. Pointer `[Agent Mail Maintainer <GreenLake>] mail 12672`
was submitted once with checked trailing Enter, parsed to GreenLake/12672, and
the Director confirmed receiving it verbatim. Copy 12673 reached the Plugins
Maintainer. This proves the real labeled pane/parser path, while monitor,
unlabeled/delimiter consumer and owner-resume acceptance remain open.

Correction request **12680** went mail-only to CobaltSpire; copy **12681**
reached ScarletGoose with checked pane submission. Director answer **12682**
declines consumer changes in his temporary scripts and identifies Plugins as
the startup/mail-tooling owner. His proposed Deferred rank has not been
accepted by the operator. The label task remains active. A follow-up to the
Plugins Maintainer (**12691**) requests a bounded normal startup/inbox/monitor integration
path and owner, with any actual controller dependency stated explicitly.
Plugins Maintainer answer **12698** confirms Plugins ownership and that no
existing Change delivers these paths; #429 and #342 do not supply them.
He will file one Proposed Change; entrypoint and controller dependencies are
still unframed. Owner/scope agreement exists; implementation remains absent.
On 2026-10-11 the operator instructed: "prioritize that bounded integration,
and permit the tracking-only commit". This overrides the Director's proposed
Deferred rank for this follow-up. Ask the Plugins Maintainer to frame and
execute the bounded startup/resume and inbox/monitor integration now. Keep
`br-0egjn` active until normal-workflow acceptance evidence is recorded.
The tracking checkpoint is `f0c6de2922376b511ee5e94a166934c3f1903ffc`.
Priority request **13003** reached the Plugins Maintainer with checked pane
submission; copy **13006** went mail-only to the Director. Maintainer answer
**13010** acknowledges the operator priority, moves Proposed
[Change #461](https://github.com/outcomeeng/changes/issues/461) ahead of Normal
Plugins work, and assigns its framing to Contributor 4. The Plugins Maintainer
owns execution through its Executor and confirms refinements. Concrete
startup/resume and inbox/monitor/doorbell entrypoints arrive with the confirmed
Frame. The accepted test list includes normal startup and resume, rename/clear
with stable ID/history, immediate authenticated send, body/no-body inbox,
Unicode labels, and absent/delimiter-safe fallback. Implementation remains open.
Conditional gates: operator review if the adapter ADR changes, and
[Change #468](https://github.com/outcomeeng/changes/issues/468) if an assertion
audit is required. That report describes `auditClass: assertion` rejected by
the current SPX contract; no local reproduction or workaround is claimed.
Preserve required evidence and wait for the concrete Frame before validation.
Read-only findings under `methodology/.spx/director/direct-positions/scripts/`:
`position_mail.py` discards raw response label metadata, `monitor.py` prints
`record.sender`, `restart_pane.py` supplies no label registration step, and
`mail_send.py` constructs a custom legacy doorbell. Preserve stable identities,
associate labels by store ID, and use source-generated pointers.

Plugin labels live in the capability `response` beside immutable records;
`data.records` retains stable identities without label fields. The plugin carries
recipient maps on send results only. Ordinary pointers are
`[Label <StableName>] mail ID`; delimiter/control labels use the stable-address
pointer while the full label remains in native metadata. Position owners supply
their own assigned label; the operator maintains no secondary mapping.

## Mail coordination

Use the current installed coding-agents skills before mail/Prowl operations.
The installed 0.12.2 adapter requires Python with `StrEnum`; this shell's default
Xcode `python3` is 3.9. `/opt/homebrew/bin/python3.13` successfully read the inbox
on 2026-10-10. Use that interpreter for the adapter rather than changing system
Python or falling back to raw mail commands.

Shared mail project working directory:
`/Users/shz/Code/outcomeeng/changes/changes.git`.
Own identity: Agent Mail Maintainer <GreenLake>. Do not re-register other owners
or print tokens. Peers: Methodology Director <CobaltSpire>, Plugins Maintainer
<ScarletGoose>, Plugins Contributor 2 <SilverHeron>. The Director authorized
direct holder coordination with ScarletGoose copied on each mail.

**Send no doorbells to the Methodology Director's pane.** Director mail 12674
states that it is the operator's conversation and pane input can answer an
operator question. Mail to CobaltSpire is mail-only. Use another authorized
recipient for pane acceptance.

Peer worktrees are `/Users/shz/Code/outcomeeng/methodology/worktrees/director/`,
`/Users/shz/Code/outcomeeng/plugins/worktrees/maintainer/`, and
`/Users/shz/Code/outcomeeng/plugins/worktrees/contributor-2/`. Resolve fresh public
identities with the skill; do not infer panes from titles or read transcripts.
This session previously lacked a Prowl caller identity. On 2026-10-11 the
inherited selector resolved an advisor pane. Scope resolver calls with
`env -u PROWL_PANE_ID PROWL_WORKTREE_PATH=/Users/shz/Code/silvarbor/agent-mail/agent-mail/`
so the public inventory resolves the actual Agent Mail worktree. The corrected
caller matched the branch and repository. The operator-authorized bounded
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
No native implementation edits or full compiler/workspace reruns occurred in
this tracking checkpoint. The three targeted triage regressions above passed
on 2026-10-11 with the pinned toolchain and two build jobs.

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

The operator's 2026-10-11 approval permits this tracking-only checkpoint despite
UBS exit 3: Markdown and JSONL are unsupported, so nothing was scanned. The
exception covers `RESUME.md`, `.beads/issues.jsonl`, and its generated sync
baseline `.beads/beads.base.jsonl`. JSONL integrity, graph cycles, and whitespace
checks passed. This exception provides no scanner approval for implementation
or publication; normal pre-commit hooks remain enabled.

The operator requested local work. Publish no upstream PR, deployment or public
release as part of this plan. No file deletion or destructive Git operation is
authorized. Shared-store repairs require explicit authority, supervised draining,
backups and reversible hash-witnessed changes. Commit coherent local checkpoints
with normal hooks and explicit paths; preserve unrelated work.
