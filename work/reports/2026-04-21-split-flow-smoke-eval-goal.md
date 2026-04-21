---
id: report_2026_04_21_split_flow_smoke_eval_goal
goal_id: goal_add_flow_and_smoke_eval
actor: vitaly
created_at: 2026-04-21
kind: handoff
---

# Split Flow + Smoke Eval Goal

## Summary

Split the broad `goal_add_flow_and_smoke_eval` into smaller bounded goals aligned with the roadmap phases.

## Why

The original goal mixed Phase 1 flow/event kernel work with Phase 2 smoke eval harness work.

## Changes

- marked `work/goals/goal_add_flow_and_smoke_eval.md` as `superseded`
- added `work/goals/goal_add_flow_state_kernel.md`
- added `work/goals/goal_add_append_only_event_log.md`
- added `work/goals/goal_add_flow_inspect_command.md`
- added `work/goals/goal_add_smoke_eval_harness.md`
- updated `work/STATUS.md` so `selected_next` now points to `work/goals/goal_add_flow_state_kernel.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Splits a broad work goal into smaller bounded goals and updates the manual Work Ledger state."
  touched_surfaces:
    - project-memory
  required_updates:
    - work/STATUS.md
    - work/_templates/goal.md
    - work/goals/goal_add_flow_and_smoke_eval.md
    - work/goals/goal_add_flow_state_kernel.md
    - work/goals/goal_add_append_only_event_log.md
    - work/goals/goal_add_flow_inspect_command.md
    - work/goals/goal_add_smoke_eval_harness.md
    - work/reports/2026-04-21-split-flow-smoke-eval-goal.md
  supersedes:
    - old_ref: work/goals/goal_add_flow_and_smoke_eval.md
      new_ref: work/goals/goal_add_flow_state_kernel.md
      action: mark_superseded
  archive_plan: []
  evals_required:
    - work-ledger-checks
```

## Validation

- `python3 scripts/check_work_ledger.py`: PASS

## Out of Scope

- no Rust code changes
- no runtime behavior
- no `.punk/` writes
- no CLI implementation
- no eval implementation
