---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: vitaly
ledger_version: work-ledger.v0.1
dogfooding_level: 0
updated_at: 2026-04-22
current_phase: "Dogfooding Level 0 / Phase 2 preparation"
current_focus: "Expose the existing smoke harness through one honest CLI command after stabilizing the manual review loop"
selected_next: "work/goals/goal_add_smoke_eval_cli_command.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: expose the existing smoke harness through one honest CLI command after stabilizing the manual review loop.
- Selected next: `work/goals/goal_add_smoke_eval_cli_command.md`
- Why this is next: the reusable review loop template is now in place, so the next conservative operator-surface follow-up is a narrow smoke-eval CLI command over the existing library-first harness.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - the review loop remains advisory and does not become automation.
  - the next eval CLI surface remains honest about local-only smoke execution.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_add_smoke_eval_cli_command.md` | `ready` | The library-first smoke harness exists and can now be exposed through a bounded CLI command. | — |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Storage research stays deferred behind the review-loop follow-up and smoke eval CLI surface. | `work/goals/goal_add_smoke_eval_cli_command.md` |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| Project Memory storage research | `work/goals/goal_add_smoke_eval_cli_command.md` | Land the narrow smoke eval CLI surface before revisiting longer-horizon storage research. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
| 2026-04-22 | Added the reusable Work Ledger review loop | `work/goals/goal_add_work_ledger_review_loop.md`, `work/reports/2026-04-22-work-ledger-review-loop.md` |
| 2026-04-22 | Ran the first advisory Work Ledger Review | `work/goals/goal_run_first_work_ledger_review.md`, `work/reports/2026-04-22-work-ledger-review.md` |
| 2026-04-22 | Added the first smoke eval harness | `work/goals/goal_add_smoke_eval_harness.md`, `work/reports/2026-04-22-smoke-eval-harness.md` |
| 2026-04-22 | Added the first bounded `punk flow inspect` command | `work/goals/goal_add_flow_inspect_command.md`, `work/reports/2026-04-22-flow-inspect-command.md` |
| 2026-04-22 | Added Research Gate preflight to the repo workflow skill | `work/goals/goal_add_research_gate_preflight_to_workflow_skill.md`, `work/reports/2026-04-22-research-gate-preflight.md` |
| 2026-04-22 | Connected flow transitions to the append-only event log | `work/goals/goal_connect_flow_transitions_to_event_log.md`, `work/reports/2026-04-22-connect-flow-transitions-to-event-log.md` |
| 2026-04-21 | Added the append-only event log kernel | `work/goals/goal_add_append_only_event_log.md`, `work/reports/2026-04-21-append-only-event-log.md` |
| 2026-04-21 | Added the Codex project workflow skill | `work/goals/goal_add_codex_project_workflow_skill.md`, `work/reports/2026-04-21-codex-project-workflow-skill.md` |
| 2026-04-21 | Added the pure flow state kernel | `work/goals/goal_add_flow_state_kernel.md`, `work/reports/2026-04-21-flow-state-kernel.md` |
| 2026-04-21 | Split the broad flow + smoke eval goal into smaller bounded goals | `work/reports/2026-04-21-split-flow-smoke-eval-goal.md` |
| 2026-04-21 | Bootstrapped the Phase 0 active-core workspace skeleton | `work/goals/goal_bootstrap_punk_core.md`, `work/reports/2026-04-21-phase-0-workspace-skeleton.md` |
| 2026-04-21 | Removed accidental top-level `public/` duplication | `work/reports/2026-04-21-remove-accidental-public-duplication.md` |
| 2026-04-21 | Bootstrapped the manual Work Ledger surface | `work/goals/goal_bootstrap_manual_work_ledger.md`, `work/reports/2026-04-21-manual-work-ledger-bootstrap.md` |
| 2026-04-20 | Recorded docs-governance v0 status and froze further v0 checker growth | `work/reports/2026-04-20-docs-governance-v0-status.md` |

## Validation

- Last checked: 2026-04-22
- Command: `python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && scripts/check.sh docs-governance --files work/reports/2026-04-22-work-ledger-review-loop.md --report work/reports/2026-04-22-work-ledger-review-loop.md && grep -R "$PWD" -n work docs scripts .agents AGENTS.md || true`
- Result: `PASS`
- Notes:
  - `selected_next` moves to `work/goals/goal_add_smoke_eval_cli_command.md`
  - the review loop remains manual and advisory only
  - smoke eval CLI stays a separate future operator-surface diff
