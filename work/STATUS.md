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
current_focus: "Define a schema-only smoke eval report v0.1 proposal before any machine-readable output or runtime report storage"
selected_next: "work/goals/goal_define_smoke_eval_report_schema_v0_1.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: define a schema-only smoke eval report v0.1 proposal before any machine-readable output or runtime report storage.
- Selected next: `work/goals/goal_define_smoke_eval_report_schema_v0_1.md`
- Why this is next: research confirmed that machine-readable output, storage, baseline, and waiver work should not proceed without an explicit schema-only proposal first.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - smoke eval remains an assessment surface, not a decision surface.
  - machine-readable output remains deferred until a schema-only proposal is defined.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_define_smoke_eval_report_schema_v0_1.md` | `ready` | Research recommends a schema-only proposal before any machine-readable output or storage work. | — |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Project Memory storage research stays deferred behind the eval-report schema proposal boundary. | `work/goals/goal_define_smoke_eval_report_schema_v0_1.md` |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| Stable machine-readable smoke eval output | `work/goals/goal_define_smoke_eval_report_schema_v0_1.md` | Define the v0.1 schema proposal before any output implementation. |
| `.punk/evals` report storage | `work/goals/goal_define_smoke_eval_report_schema_v0_1.md` | Clarify the schema and ownership boundary first. |
| Project Memory storage research | `work/goals/goal_define_smoke_eval_report_schema_v0_1.md` | Finish the eval-report schema proposal before broader storage work. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
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
- Command: `python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && scripts/check.sh docs-governance --files knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md work/reports/2026-04-22-eval-report-schema-research.md --report work/reports/2026-04-22-eval-report-schema-research.md && grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge || true`
- Result: `PASS`
- Notes:
  - `selected_next` moves to `work/goals/goal_define_smoke_eval_report_schema_v0_1.md`
  - stable machine-readable output remains deferred behind schema-only design work
  - `.punk/evals` storage, baseline, and waiver semantics remain out of scope
