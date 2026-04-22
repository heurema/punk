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
current_focus: "Define a design-only eval baseline and waiver boundary v0.1 before any implementation"
selected_next: "work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: define a design-only eval baseline and waiver boundary v0.1 before any implementation.
- Selected next: `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md`
- Why this is next: the storage boundary spec is now defined and explicitly defers baseline/waiver to a separate boundary step.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - smoke eval remains an assessment surface, not a decision surface.
  - `.punk/evals`, baseline, and waiver implementation remain deferred behind the baseline/waiver boundary design step.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md` | `ready` | The storage boundary is now explicit, so the next honest design step is to define baseline and waiver boundaries separately. | — |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Project Memory storage research stays deferred behind the eval baseline/waiver boundary step. | `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md` |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| `.punk/evals` report storage | `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md` | Keep storage implementation deferred while baseline/waiver boundaries are still being specified. |
| Baseline and waiver behavior | `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md` | Define the boundary spec before any implementation. |
| Project Memory storage research | `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md` | Finish the eval baseline/waiver boundary step before broader storage work. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
| 2026-04-22 | Defined the eval storage boundary v0.1 spec | `work/goals/goal_define_eval_storage_boundary_v0_1.md`, `work/reports/2026-04-22-eval-storage-boundary-v0-1.md`, `evals/specs/eval-storage-boundary.v0.1.md` |
| 2026-04-22 | Researched eval storage, baseline, and waiver boundary | `work/goals/goal_research_eval_storage_and_baseline_boundary.md`, `work/reports/2026-04-22-eval-storage-baseline-boundary-research.md`, `knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md` |
| 2026-04-22 | Added opt-in smoke eval JSON output v0.1 | `work/goals/goal_add_smoke_eval_json_output_v0_1.md`, `work/reports/2026-04-22-smoke-eval-json-output-v0-1.md` |
| 2026-04-22 | Defined the schema-only smoke eval report v0.1 proposal | `work/goals/goal_define_smoke_eval_report_schema_v0_1.md`, `work/reports/2026-04-22-smoke-eval-report-schema-v0-1.md`, `evals/specs/smoke-eval-report.v0.1.md` |
| 2026-04-22 | Researched eval report schema before machine output | `work/goals/goal_research_eval_report_schema_before_machine_output.md`, `work/reports/2026-04-22-eval-report-schema-research.md`, `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md` |
| 2026-04-22 | Added the internal smoke eval report artifact shape | `work/goals/goal_add_smoke_eval_report_artifact_shape.md`, `work/reports/2026-04-22-smoke-eval-report-artifact-shape.md` |
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
- Command: `python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && scripts/check.sh docs-governance --files evals/specs/eval-storage-boundary.v0.1.md work/reports/2026-04-22-eval-storage-boundary-v0-1.md --report work/reports/2026-04-22-eval-storage-boundary-v0-1.md && grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true && git diff --name-only`
- Result: `PASS`
- Notes:
  - `selected_next` moves to `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md`
  - the storage boundary remains design-only and does not activate `.punk/evals`
  - baseline, waiver, validator, and proof integration remain out of scope
