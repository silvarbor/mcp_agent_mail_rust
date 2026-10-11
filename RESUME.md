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
   The #461 Executor is active in Plugins `worktrees/executor-461` on
   `work/change-461`; collect its implementation head and real resume probe.
   #471 retains the actual monitor follow-up. The dependency guard approval
   and #468 producer repair are complete; do not request them again.
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
The concrete Frame is confirmed. The operator ordered execution of the six-step
acceptance plan; continue through implementation and normal-workflow evidence.

## Active execution — 2026-10-11

The Plugins Maintainer separated the existing four-node work in
[Change #461](https://github.com/outcomeeng/changes/issues/461) from Director
resume/monitor in [Change #471](https://github.com/outcomeeng/changes/issues/471).
The latter retains prerequisites #342 and #461. Native source dependencies for
the adapter, lifecycle instructions and notifications already exist; those
mutations need no #342 source. Monitor acceptance remains part of `br-0egjn`.
The Director approved #461's manual execution route in **13173**, preserving
its tests, audits and lifecycle records; it need not wait for the broad
final-format execution-skill upgrade #470. #342 has a live Plugins Contributor
recovering its branch and continuing the monitor prerequisite.

SPX Maintainer **13127** confirms the existing published 0.9.0 contract permits
class/kind `spec/spec` for the node-spec subject, retaining assertion identity
in unit IDs and findings. The assertion producer's `assertion` class is outside
that closed set. The Director and Plugins Maintainer agreed on a Plugins-side
repair in #468, executed by Contributor 2, with a floor-version contract test
and a real sealed assertion-audit run. No SPX release or weaker audit is needed.
The #461 Frame amends no ADR/PDR, so its earlier conditional operator text-review
gate does not apply.

The obsolete native #342 dependency link on #461 was removed at **18:30 UTC**
on 2026-10-11. The operator replied "Continue delivering fucking value" to the
exact scoped removal question. The authorized command, originally rejected
and reported by the Plugins owner in **13144**, was:
`gh api repos/outcomeeng/changes/issues/461/dependencies/blocked_by/5694106798 --method DELETE --silent`.
DCG's old grant 776086 was scoped to the owner's scratchpad and did not admit
the current caller. Fresh single-use grant 905609 admitted the exact command;
it exited zero. The unused old grant was revoked with
`dcg allow-once revoke 776086 --yes`. Fresh GETs verify #461 has no native
blockers and #471 still has #342 and #461. Neither issue was deleted. The
earlier pending-approval snapshots below are historical, not current blockers.

Fresh native/adapter baseline is retained under
`/Users/shz/.local/state/mcp-agent-mail/stabilization/20261011-label-workflow/`:
`baseline.json`, `label-routing-negative.json`, and `relationships.json`.
The isolated server runs on 8877 through the existing demo helper, whose
executable mode was corrected. Own fixture GreenLake ID 22 shares Reviewer with
retained RedStone; original message 24 stays addressed by stable name through
rename, omission and clear in both body modes. Immediate subsequent sends,
Unicode/unlabeled/delimiter pointer parsing, control rejection and the 128/129
Unicode-scalar boundary passed. Sending to Reviewer failed without delivery.
The focused real DB relationship test passed 1/1, preserving contacts,
reservations, token/contact policy and message identity. Source is unchanged.
This baseline proves neither automatic lifecycle triggers nor the future monitor.
Keep the task active, obtain each implementation's exact head, and validate the
normal workflow before closing it.

Execution snapshot at **14:28 UTC**: #461 is Available/Executable; no source
branch exists yet. Maintainer **13264** will assign its Executor when #468's
merge frees a slot. #468 is Plugins PR #649, release head
`7288a255692cfed121ccd4cf2e0d8d23ed3cfb48`; its current-head CI review passed,
while Quality Gate is still running. A skill audit was reported approved;
the floor-payload acceptance, sealed unit/finding/finish run and actual launched
assertion-auditor evidence remains uncollected. Question **13293** requests
those exact paths/results. Count only real sealed audit results for #461's gate.

#342 recovered its branch and has implementation at
`6128717882028ade069b3f7ac215723df79dbe22`. Its real-agent tests wait on Plugins
#472, the separate disposable Codex-home configuration repair. #471 remains
blocked by #342 and #461. Read-only inspection of the candidate monitor confirms
`position_mail.py` discards labels and `monitor.py` prints the stable sender.
The actual live Plugins reply **13264** carries `sender_display_name: null`.
The same three acceptance items remain open; no normal workflow passed yet.

Maintainer **13207** explicitly accepted a real resumed owner through shipped
orchestration/recovery, its assigned context supplying the label before first
mail. Retain the session, own registration, sent message, recipient inbox in
both body modes and exact source-generated pointer. An API driver performing
registration alone is insufficient. #471 must retain a real labeled monitor
line plus passing signal-edge, deadline and lock checks. The Director receives
mail only. Coordinate the actual pane recipient before submission; isolated
fixture message IDs must never be sent into the shared live mail project.
Facts **13257** made the native baseline available to Plugins; both that mail
and question **13293** were delivered mail-only to ScarletGoose. The dependency
guard question above is still unanswered. Preserve that pause and the existing
operator authority for two build jobs and tracking-only checkpoints.

Later evidence, **14:50 UTC**: #468 now has head
`19795b314159a747cd71818460f264995690cb80` in PR #649. Tests added in
`d297ccce9bc57abef0e1463e04289f1ca98a1650` are
`test_audit_assertions_payload.conformance.l3.py` and
`test_audit_assertions_sealed_run.scenario.l3.py` under the audit-assertions
node. The real CLI checks exposed a second producer defect, absent
`producerProvenance`, now repaired. The PR reports both focused tests passing
against SPX floor 0.8.0 and failing when the original class defect is restored.
Current-head CI review passed; Quality Gate remains running. These are
owner-reported test results; the actual launched-auditor seal after installation
is still required. Mail **13379** names the evidence files; its earlier
unpushed-state report is superseded by the current PR head.

The real candidate monitor gap was reproduced at source
`ac15a0817a7d109767e027d2d1e9c9447be96bf6`. Evidence is retained in
`/Users/shz/.local/state/mcp-agent-mail/stabilization/20261011-label-workflow/monitor-baseline.json`.
A fresh private state withheld existing mail through 27; own GreenLake
sent isolated message 28 with label Reviewer, and both send and actual inbox
returned that label. Actual monitor output was `MAIL 28: from GreenLake`;
the notification formatter produced `[Reviewer <GreenLake>] mail 28`.
The next poll emitted nothing and retained cursor 28. The three source-script
hashes remained unchanged, and the watch had no sessions/groups or pane reads.
This proves the consumer defect and its baseline mail edge, not final label,
deadline or lock acceptance. No fixture pointer entered a live pane. Mail
**13384** supplied this evidence to Plugins. Compare the repaired monitor with
a fresh private state and a new message; preserve the original baseline.

#461 still has no implementation head and its obsolete #342 link still exists.
The required one-time guard decision remains pending. On an explicit approval,
relay the exact scope to the Plugins Maintainer, let its owner perform the
approved action, and read back both #461 and #471 dependencies. Continue the
real owner-session and final monitor checks before closing checklist 1, 7 or 9.

Execution snapshot at **15:36 UTC**, superseding the earlier snapshots above:
#468 merged as Plugins PR #649 at
`437b9b57b5c2ec8d3101815d06d954f1f0d7bd15`, and spec-tree 0.105.3 is released.
Maintainer **13563** reports the actual installed assertion-auditor run
`2026-10-11_15-15-34-437-c04fa6cff606` sealed approved with zero findings and
an audited `spec/spec` unit. Its target is
`spx/15-validation.enabler/32-plugin-manifest.enabler/plugin-manifest.md`;
the floor-version checks accepted the payload on SPX 0.8.0. That producer
prerequisite is complete; no SPX release is required.

Director **13576** set #461 to **Normal (Capability, Shield)**, independently
read back at 15:26 UTC. This supersedes its earlier Deferred rank. Execution
still requires removal of the obsolete #342 edge, an available slot, and no
higher-ranked unblocked Executable Plugins Change waiting. The manual route
**13173** remains valid; #470 is a capacity competitor, not a source dependency.
#461 still has no implementation head. Its exact dependency-removal approval
question remains unanswered; neither elapsed time nor another owner's
readiness authorizes that rejected command.

#342 has a new handoff **6110595337** at 15:28:13 UTC, branch
`work/change-342-direct-positions-3`, head
`30cb2a60a23cc6882154e181e21a13923d7f1bd1`. It waits on decision review #473
and the full-gate setup repair #472; its tests have no passing verification on
that head. #472's implementation and audits are approved at
`af33d47d06b4a512b4d3ab139d9f75f95ce72735`, with its terminal full gate pending.
Maintainer **13505** confirms #472 is no product dependency of #461: an
existing owner can supply the real resume pilot. Do not mistake a handed-off
branch for verified delivery or copy its scripts into this checkout.

Maintainer **13651** confirms #342 released its slot. Both execution slots were
then assigned to Urgent Changes #425 and #454. #461 remains queued at Normal;
the dependency approval and execution assignment are both outstanding.
Requests **13666** and **13667** ask the Plugins Maintainer and Methodology
Director to use the already-supported own-label registration before their next
mail, without registering another owner. This can improve current correspondence
while the general trigger is implemented. Neither request establishes a
completed registration or the required startup/resume acceptance.

Execution snapshot at **19:11 UTC**, superseding the earlier scheduling notes:
The record rewrite is complete. Handoff **6112602015** releases #461 as
Executable with no blockers; its audit approved twice, including the absent
or stale stored-label precondition. Maintainer **14108** confirms the herdr
Executor `change-461` is running under the Maintainer's controlling claim,
whose session is `3eadb25c-ac31-4cf4-88eb-09e5c9c2cfd0`, in worktree
`/Users/shz/Code/outcomeeng/plugins/worktrees/executor-461`, pane `w51:p1`,
branch `work/change-461`, initial head
`8733b1b26b889e5d5fcb681cf1d684688a4454a6`. Claim **6112632075** matches this
claim session and worktree. The public herdr inventory does not expose an
Executor session UUID; do not equate it with the controlling claim. No
implementation commit or passing owner-resume probe
is claimed at this initial head. Collect the new source head and attested run.
The separate disposable-home repair #472 is merged as Plugins PR #650 in
this initial head. #471 remains open and still needs #342 and #461 delivery.

Mail **14083** supplied the pilot precondition: the resumed owner itself
prepares an absent or stale label in the isolated store, then the shipped
resume restores its assigned label before its first mail. Starting with the
already-correct label cannot establish that the trigger ran. Its send command
exceeded the adapter deadline after persistence; exact search found 14083,
and recipient answer **14108** explicitly confirms receipt and incorporation.
No resend was made. Cheap health and reservation checks passed. The retained
three-second `live-mail-timeout-sample.txt` establishes no root cause; record
this bounded send-latency observation in installed acceptance `br-vas72.2`.
No service restart, mailbox repair or timeout relaxation was performed.

Historical execution snapshot at **18:47 UTC**:
Maintainer **14042** reports the operator's new execution rule: Contributors
take a Change to Executable, and a herdr Executor implements it. The old hand
route is superseded. The installed execute skill expects the earlier record
form with a Nodes section. In response to **14054**, Maintainer **14063**
assigned BrightWillow to rewrite #461 immediately in that supported form,
preserving the confirmed four-node scope, settled answers and real owner-session
probe; it need not wait for #470. The fresh claim is **6112425324**,
session `16cd39b2-7ee0-426d-9d89-e96018a24a3f`, worktree
`/Users/shz/Code/outcomeeng/plugins/worktrees/ff030327`, intended branch
`work/change-461`. The Maintainer will start its herdr Executor as soon as this
record is released and provide the worktree, pane and head. This is an active
record rewrite, not implementation or a passing workflow probe.

Director **14035** retained Normal priority and declined further rank requests.
The obsolete dependency is resolved. Do not ask the operator for that same
permission again or treat #470/#472 as product prerequisites of #461. Collect
the actual Executor head and run the agreed acceptance. #471 still needs #342
delivery and #461 before its own monitor integration can finish.

Both requested owner registrations now have direct live evidence: checked
inbox **14042** reports `Plugins Maintainer` for ScarletGoose and **14035**
reports `Methodology Director` for CobaltSpire. Native inbox metadata reflects
the current label on older messages as well. Each owner registered itself;
no operator-maintained label roster or registration of another identity was used.
This improves current correspondence and does not establish the general
startup/resume trigger.

Real browser acceptance is retained in `web-label-refresh.json` and
`web-label-refresh.png` under the 20261011-label-workflow evidence directory.
The isolated page had 28 messages. Unchanged refresh preserved its array,
message object and DOM row. Natural automatic refresh showed rename to
`Officer Alpha Ω <GreenLake>` and clear to `GreenLake`, preserving message 28,
its original body, selection, routing address and all message IDs. An HTML-shaped
label remained literal text, inserted no image and executed no handler. Own
fixture GreenLake remained ID 22 and was restored to Reviewer; the owned
headless browser was closed. This is functional UI evidence, not a new
1,000-message CPU/memory benchmark or automatic owner-lifecycle proof.

Acceptance snapshot at **20:00 UTC**: native source remains `1f309419` and
Plugins #461 is on `work/change-461`, committed head
`cae5ddf9cc89378bf56af99a61ecb4f300825a6d`, after rebasing its initial two
commits onto `3af98c36daa2524f919c1205782075ca266ffcf8`. Only the agent-mail
spec and startup/resume register instructions are committed; the adapter
implementation is still an uncommitted round. Rows 2–4 have no delivery
claim. Its adapter file SHA-256 is
`ef46e3d4a56aaa901a8355b154d94f9d7b96eed90e17b026339e7b1f54cd8541`.

Root ran the candidate's actual source-owned CLI against the retained isolated
8877 store. Four reads covered 42 records in both body modes: `senderDisplays`
paired each ID with its expected display while all immutable records stayed
byte-identical to the installed adapter's results. Three own-fixture stages
(rename to Officer Alpha Ω, clear, restore Reviewer) covered 96 more record
reads. GreenLake stayed ID 22; earlier messages changed their displayed label
without changing their IDs, bodies, sender or recipient. The source hash stayed
unchanged. Evidence: `inbox-projection-before.json`,
`inbox-projection-candidate.json`, and
`inbox-projection-rename-clear-candidate.json` in the evidence directory.
These are provisional uncommitted-candidate integration results, not formal
gate, automatic-resume, pane or monitor acceptance. The fixture is restored.

One precise formatter edge remains: `sender_display("BlueLake", "   ")`
returned whitespace plus `<BlueLake>` rather than the blank-label fallback.
Native registration already clears blank labels, so the real-store reads pass.
Maintainer **14193** records the correction from **14187** in #461 comment
**6112939996**, queued to its Executor: require nonblank eligibility with
`label.strip()` while preserving full eligible label text, and test spaces.
No source file belonging to that owner was edited here.

Public herdr observation at 19:29 UTC showed the implementer at the host-load
waiter for 15m22s. Maintainer **14175** declined transferring this session's
operator-approved sequential/two-job exception into Plugins Executors; it
requires direct operator authorization in his own session. He declined an
extra mid-round checkpoint in **14193**. Question **14225** asks the Director
for a concrete operating-model route. Root's authorized bounded checks above
continued here. No waiter, hook, assertion or audit was weakened. A public
recent-history read was refused while the Executor was working; no scrolling,
keystroke, interruption or history capture followed.

The remaining monitor route is concrete: Director **14130** presents #473's
same-plugin-import decision, returns #342 for its decision/spec revision, and
orders #471 refinement in parallel. Maintainer **14150** assigns BrightWillow;
claim **6112775696** matches `ff030327`. Handoff **6112983956** releases its
Proposed draft after the Advisor correction, retaining #342/#461 blockers,
real fixture acceptance and existing monitor edge/deadline/lock behavior.
It cites Director **14163** confirming operator approval of #473's import rule.
Approval is not merged delivery; #473 remains open. #342 received Proposed
confirmation **6113094450** at 19:56 UTC. Collect actual implementation heads.

All three peer positions now have current native labels. SPX Maintainer's
own registration response **14217**, following request **14214**, retains
RusticOwl ID 238; its next send has `sender_display_name: "SPX Maintainer"`.
Plugins Maintainer and Methodology Director proofs above remain valid. This
improves live correspondence without operator configuration and establishes
no general automatic-resume trigger or change to orchestration rank.

The live inbox read in session 1825 exceeded its 30-second adapter deadline.
Fresh native health reports green, passed integrity, aligned archive/database,
empty queues, and archive-commit tails above the deadline (git p99 51,003ms;
queue p99 53,389ms). Preserve `live-inbox-deadline-observation.json` for the
already-planned installed acceptance `br-vas72.2`. The diagnostics identify
the reported slow stage; they establish no exact cause for this inbox failure.
No crash, corruption, permanent stall, descriptor proof, restart, repair or
timeout relaxation is claimed. A later new bounded inbox read succeeded.

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
