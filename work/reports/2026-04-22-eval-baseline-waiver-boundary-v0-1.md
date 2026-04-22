---
id: report_2026_04_22_eval_baseline_waiver_boundary_v0_1
goal_id: goal_define_eval_baseline_waiver_boundary_v0_1
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Define `eval-baseline-waiver-boundary.v0.1` as a design/spec artifact for future baseline and waiver semantics without implementing baseline comparison, waiver ledger files, validators, runtime persistence, or gate/proof behavior.

## Research Gate

Classification: R1
Required: yes
Rationale:
R2 research and eval storage boundary spec already exist. This diff only creates a proposed baseline/waiver boundary spec and does not implement baseline comparison, waiver ledger, validators, runtime persistence, or eval policy changes.
Research refs:
- `knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md`
- `evals/specs/eval-storage-boundary.v0.1.md`
Decision:
Proceed with design/spec boundary only.

## Files Changed

- `evals/specs/eval-baseline-waiver-boundary.v0.1.md`
- `work/goals/goal_define_eval_baseline_waiver_boundary_v0_1.md`
- `work/goals/goal_run_second_work_ledger_review.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-eval-baseline-waiver-boundary-v0-1.md`

## Spec Summary

- defines baseline as future-only and explicitly selected, never implicit mutable `latest`;
- keeps baseline comparison as assessment rather than acceptance;
- defines minimum future baseline metadata including explicit ref/hash and selector identity;
- defines waiver as future-only, explicit, scoped, reasoned, approver-tagged, and revisitable;
- keeps waivers as evidence/governance records rather than acceptance records;
- keeps both baseline and waiver behind separate future implementation goals.

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
- `scripts/check.sh docs-governance --files evals/specs/eval-baseline-waiver-boundary.v0.1.md work/reports/2026-04-22-eval-baseline-waiver-boundary-v0-1.md --report work/reports/2026-04-22-eval-baseline-waiver-boundary-v0-1.md`
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

- baseline comparison implementation
- waiver ledger implementation
- `.punk/evals` runtime storage
- validator rules for baseline or waiver records
- gate/proof reference mechanics implementation
- remote storage or dashboards

## Next Recommended Action

`work/goals/goal_run_second_work_ledger_review.md`
