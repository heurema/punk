---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: vitaly
ledger_version: work-ledger.v0.1
dogfooding_level: 0
updated_at: 2026-04-22
current_phase: "Dogfooding Level 0 / Phase 1 preparation"
current_focus: "Expose the first bounded flow inspect surface after installing Research Gate preflight"
selected_next: "work/goals/goal_add_flow_inspect_command.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: expose the first bounded inspect surface now that flow transitions emit real event evidence and Research Gate preflight is installed.
- Selected next: `work/goals/goal_add_flow_inspect_command.md`
- Why this is next: flow inspect remains the next honest implementation goal, and the workflow now blocks future `R1+` work from silently skipping research preflight.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - Research Gate preflight is installed before the first CLI/operator surface.
  - inspect remains separate from smoke eval and `.punk/` runtime activation.
  - event evidence remains evidence only, not decision authority.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_add_flow_inspect_command.md` | `ready` | Next implementation surface now that flow/event evidence exists and Research Gate preflight is installed. | — |
| `work/goals/goal_add_smoke_eval_harness.md` | `blocked` | Smoke eval stays behind the first inspect surface and the existing flow/event evidence. | `work/goals/goal_add_flow_inspect_command.md` |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Storage research stays deferred until manual ledger semantics survive at least one real work cycle. | Manual ledger usage evidence |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| `work/goals/goal_add_smoke_eval_harness.md` | `work/goals/goal_add_flow_inspect_command.md` | Finish the first bounded inspect surface before starting Phase 2 eval harness work. |
| Project Memory storage research | Stable manual ledger semantics | Complete at least one cycle of selected goal -> report -> status update -> next selected goal. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
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
- Command: `python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && scripts/check.sh docs-governance --files work/reports/2026-04-22-research-gate-preflight.md --report work/reports/2026-04-22-research-gate-preflight.md`
- Result: `pass`
- Notes:
  - `selected_next` remains `work/goals/goal_add_flow_inspect_command.md`
  - Research Gate preflight was inserted before flow inspect and completed without superseding it
  - future `R1+` work must cite repo-tracked or user-provided research or stop for Deep Research
