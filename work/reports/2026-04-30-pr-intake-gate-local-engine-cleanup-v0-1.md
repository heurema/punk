---
id: report_2026_04_30_pr_intake_gate_local_engine_cleanup_v0_1
goal_id: goal_remove_legacy_pr_intake_gate_local_engine_v0_1
actor: vitaly
created_at: 2026-04-30
kind: handoff
---

## Summary

Removed the now-unused local PR Intake Gate script/test harness after Punk moved workflow execution to the shared `heurema/repo-governance` action.

## Research Gate

Classification: R1
Required: yes
Rationale:
This removes stale repository-governance automation code after moving PR Intake Gate execution to a shared action. It does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior.
Research refs:
- `knowledge/research/2026-04-29-pr-intake-gate-review.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/RESEARCH-GATE.md`
- `CONTRIBUTING.md`
Decision:
Delete the local engine/test copy and keep shared-action execution plus Punk-local policy as the active architecture.

## Files Changed

- `scripts/pr_intake_gate.py`
- `scripts/test_pr_intake_gate.py`
- `scripts/check.sh`
- `CONTRIBUTING.md`
- `CHANGELOG.md`
- `work/STATUS.md`
- `work/goals/goal_remove_legacy_pr_intake_gate_local_engine_v0_1.md`
- `work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md`

## Implementation Summary

- Deleted the legacy local gate engine and fixture-backed local test harness.
- Removed the obsolete `scripts/check.sh pr-intake-gate` target so repo-native checks do not point at deleted files.
- Replaced contributor-doc local-check instructions with the shared action, local policy, and PR-check verification path.
- Recorded the maintainer-requested scope override in the work ledger.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The diff removes obsolete repo-governance automation and updates work-ledger/docs pointers without changing canonical product docs or runtime behavior."
  touched_surfaces:
    - "scripts/check.sh"
    - "CONTRIBUTING.md"
    - "CHANGELOG.md"
    - "work/STATUS.md"
  required_updates:
    - "CONTRIBUTING.md"
    - "CHANGELOG.md"
    - "work/STATUS.md"
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `git diff --check` - PASS
- `scripts/check.sh docs-governance --files scripts/check.sh CONTRIBUTING.md CHANGELOG.md work/STATUS.md work/goals/goal_remove_legacy_pr_intake_gate_local_engine_v0_1.md work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md --report work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md` - PASS, 0 failures, 0 warnings

## Scope Boundaries Preserved

- no Rust code changes;
- no `.punk` runtime state;
- no product-doc changes under `docs/product/**`;
- no shared workflow or Punk-local policy changes;
- no active CLI behavior changes;
- no runtime gate decision or proofpack behavior changes;
- no provider/model/agent execution requirement.

## Next Recommended Action

Return to `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
