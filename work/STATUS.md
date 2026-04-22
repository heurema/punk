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
current_focus: "Define one bounded smoke-eval report artifact shape after exposing the smoke harness through a narrow CLI command"
selected_next: "work/goals/goal_add_smoke_eval_report_artifact_shape.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: define one bounded smoke-eval report artifact shape after exposing the smoke harness through a narrow CLI command.
- Selected next: `work/goals/goal_add_smoke_eval_report_artifact_shape.md`
- Why this is next: the first honest `punk eval run smoke` command now exists, so the next conservative eval-plane step is to define one bounded report artifact shape before any runtime storage, baseline, or waiver work.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - smoke eval remains an assessment surface, not a decision surface.
  - report-shape work stays local-first and does not activate `.punk/evals`.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_add_smoke_eval_report_artifact_shape.md` | `ready` | The smoke harness now has an honest CLI wrapper and needs one bounded report artifact shape before any richer eval platform work. | — |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Storage research stays deferred behind the smoke eval CLI and report-shape follow-up. | `work/goals/goal_add_smoke_eval_report_artifact_shape.md` |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| Project Memory storage research | `work/goals/goal_add_smoke_eval_report_artifact_shape.md` | Land the bounded smoke-eval report artifact shape before revisiting longer-horizon storage research. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
| 2026-04-22 | Added the first bounded `punk eval run smoke` command | `work/goals/goal_add_smoke_eval_cli_command.md`, `work/reports/2026-04-22-smoke-eval-cli-command.md` |
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
- Command: `python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && cargo test -p punk-eval && cargo test -p punk-cli && cargo test -p punk-flow && cargo test -p punk-events && cargo check --workspace && cargo run -q -p punk-cli -- eval run smoke && scripts/check.sh docs-governance --files work/reports/2026-04-22-smoke-eval-cli-command.md --report work/reports/2026-04-22-smoke-eval-cli-command.md && grep -R "$PWD" -n work docs scripts .agents AGENTS.md || true`
- Result: `PASS`
- Notes:
  - `selected_next` moves to `work/goals/goal_add_smoke_eval_report_artifact_shape.md`
  - smoke eval remains a local assessment surface only
  - `.punk/evals` runtime activation, baseline/waiver, and richer report storage remain deferred
