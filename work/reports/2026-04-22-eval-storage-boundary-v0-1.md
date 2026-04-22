---
id: report_2026_04_22_eval_storage_boundary_v0_1
goal_id: goal_define_eval_storage_boundary_v0_1
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Define `eval-storage-boundary.v0.1` as a design/spec artifact for future eval storage without implementing `.punk/evals`, baseline, waiver, validators, or runtime persistence.

## Research Gate

Classification: R1
Required: yes
Rationale:
R2 research has already been completed. This diff only creates a proposed eval storage boundary spec and does not implement storage, baseline, waiver, validators, runtime persistence, or eval policy changes.
Research refs:
- `knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md`
Decision:
Proceed with design/spec boundary only.

## Files Changed

- `evals/specs/eval-storage-boundary.v0.1.md`
- `work/goals/goal_define_eval_storage_boundary_v0_1.md`
- `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-eval-storage-boundary-v0-1.md`

## Spec Summary

- defines eval storage as future, not active now;
- states that stored eval reports are evidence, not decisions;
- requires future storage to be append-only and run-scoped;
- forbids mutable `latest` from becoming canonical truth;
- requires future stored reports to identify schema/version and stable run identity;
- requires future stored reports to be hash-referenceable by later gate/proof steps;
- defines baseline as future work that must reference an explicit selected report rather than a mutable latest view;
- defines waiver as future work that must require scope, reason, owner/approver, and expiry or revisit trigger.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds a design/spec artifact and work-ledger updates without changing canonical product docs or runtime/code surfaces."
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files evals/specs/eval-storage-boundary.v0.1.md work/reports/2026-04-22-eval-storage-boundary-v0-1.md --report work/reports/2026-04-22-eval-storage-boundary-v0-1.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no `.punk/evals`
- no file writes
- no report persistence
- no baseline comparison implementation
- no waiver ledger implementation
- no validators
- no gate/proof implementation

## Deferred

- `.punk/evals` implementation
- report persistence
- baseline comparison implementation
- waiver ledger implementation
- gate/proof reference mechanics implementation
- remote storage or dashboards

## Next Recommended Action

`work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md`
