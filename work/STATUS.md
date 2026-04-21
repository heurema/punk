---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: vitaly
ledger_version: work-ledger.v0.1
dogfooding_level: 0
updated_at: 2026-04-21
current_phase: "Dogfooding Level 0 / Phase 1 preparation"
current_focus: "Resume the append-only event log with the repo-scoped Codex workflow skill in place"
selected_next: "work/goals/goal_add_append_only_event_log.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: resume the append-only event log after installing a thin repo-scoped Codex workflow skill.
- Selected next: `work/goals/goal_add_append_only_event_log.md`
- Why this is next: the workflow guardrail is now in place, so event writing remains the smallest remaining Phase 1 foundation step and still unblocks later inspect and smoke-eval work.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - the workflow skill stays thin and does not become a second source of product truth.
  - event logging remains separate from CLI inspect and eval harness work.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_add_append_only_event_log.md` | `ready` | Next bounded Phase 1 step after the pure state kernel. | — |
| `work/goals/goal_add_flow_inspect_command.md` | `blocked` | Inspect is honest only after state and event evidence exist. | `work/goals/goal_add_append_only_event_log.md` |
| `work/goals/goal_add_smoke_eval_harness.md` | `blocked` | Smoke eval belongs after the first flow/event behavior exists. | `work/goals/goal_add_append_only_event_log.md` |
| `work/goals/goal_research_task_storage_before_project_memory.md` | `draft` | Storage research stays deferred until manual ledger semantics survive at least one real work cycle. | Manual ledger usage evidence |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| `work/goals/goal_add_flow_inspect_command.md` | `work/goals/goal_add_append_only_event_log.md` | Finish append-only event evidence before exposing inspect behavior. |
| `work/goals/goal_add_smoke_eval_harness.md` | `work/goals/goal_add_append_only_event_log.md` | Finish Phase 1 groundwork before starting Phase 2 eval harness work. |
| Project Memory storage research | Stable manual ledger semantics | Complete at least one cycle of selected goal -> report -> status update -> next selected goal. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
| 2026-04-21 | Added the Codex project workflow skill | `work/goals/goal_add_codex_project_workflow_skill.md`, `work/reports/2026-04-21-codex-project-workflow-skill.md` |
| 2026-04-21 | Added the pure flow state kernel | `work/goals/goal_add_flow_state_kernel.md`, `work/reports/2026-04-21-flow-state-kernel.md` |
| 2026-04-21 | Split the broad flow + smoke eval goal into smaller bounded goals | `work/reports/2026-04-21-split-flow-smoke-eval-goal.md` |
| 2026-04-21 | Bootstrapped the Phase 0 active-core workspace skeleton | `work/goals/goal_bootstrap_punk_core.md`, `work/reports/2026-04-21-phase-0-workspace-skeleton.md` |
| 2026-04-21 | Removed accidental top-level `public/` duplication | `work/reports/2026-04-21-remove-accidental-public-duplication.md` |
| 2026-04-21 | Bootstrapped the manual Work Ledger surface | `work/goals/goal_bootstrap_manual_work_ledger.md`, `work/reports/2026-04-21-manual-work-ledger-bootstrap.md` |
| 2026-04-20 | Recorded docs-governance v0 status and froze further v0 checker growth | `work/reports/2026-04-20-docs-governance-v0-status.md` |

## Validation

- Last checked: 2026-04-21
- Command: `python3 scripts/check_work_ledger.py && scripts/check.sh docs-governance --files work/reports/2026-04-21-codex-project-workflow-skill.md --report work/reports/2026-04-21-codex-project-workflow-skill.md`
- Result: `pass`
- Notes:
  - `selected_next` points to `work/goals/goal_add_append_only_event_log.md`
  - no goal is currently `in_progress`
  - the repo-scoped workflow skill is installed and `selected_next` remains `work/goals/goal_add_append_only_event_log.md`
  - no Rust code or `.punk/` runtime state changed in this diff
