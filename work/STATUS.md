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
current_focus: "Research eval storage, report history, baseline, and waiver boundaries before any `.punk/evals` runtime implementation"
selected_next: "work/goals/goal_research_eval_storage_and_baseline_boundary.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: research eval storage, report history, baseline, and waiver boundaries before any `.punk/evals` runtime implementation.
- Selected next: `work/goals/goal_research_eval_storage_and_baseline_boundary.md`
- Why this is next: opt-in JSON output now exists, so the next honest step is research-first boundary work before any storage, baseline, waiver, or report-history implementation.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - smoke eval remains an assessment surface, not a decision surface.
  - `.punk/evals` storage and baseline/waiver behavior remain deferred until research clarifies the boundary.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_research_eval_storage_and_baseline_boundary.md` | `ready` | JSON output now exists, so the next honest step is research-first work before storage, report history, baseline, or waiver implementation. | — |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Project Memory storage research stays deferred behind the eval storage and baseline boundary. | `work/goals/goal_research_eval_storage_and_baseline_boundary.md` |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| `.punk/evals` report storage | `work/goals/goal_research_eval_storage_and_baseline_boundary.md` | Clarify storage ownership and runtime/report-history boundary first. |
| Baseline and waiver behavior | `work/goals/goal_research_eval_storage_and_baseline_boundary.md` | Clarify policy and artifact implications before implementation. |
| Project Memory storage research | `work/goals/goal_research_eval_storage_and_baseline_boundary.md` | Finish eval storage and baseline research before broader storage work. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
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
- Command: `python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && cargo test -p punk-eval && cargo test -p punk-cli && cargo test -p punk-flow && cargo test -p punk-events && cargo check --workspace && cargo run -q -p punk-cli -- eval run smoke && cargo run -q -p punk-cli -- eval run smoke --format json && scripts/check.sh docs-governance --files work/reports/2026-04-22-smoke-eval-json-output-v0-1.md --report work/reports/2026-04-22-smoke-eval-json-output-v0-1.md && grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- Result: `PASS`
- Notes:
  - `selected_next` moves to `work/goals/goal_research_eval_storage_and_baseline_boundary.md`
  - `punk eval run smoke` keeps human output as default while `--format json` is opt-in
  - `.punk/evals` storage, baseline, waiver, validator, and export-adapter work remain out of scope
