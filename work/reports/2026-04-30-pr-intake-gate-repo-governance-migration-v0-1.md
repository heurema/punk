---
id: report_2026_04_30_pr_intake_gate_repo_governance_migration_v0_1
goal_id: goal_migrate_pr_intake_gate_to_repo_governance_v0_1
actor: vitaly
created_at: 2026-04-30
kind: handoff
---

## Summary

Migrated the Punk PR Intake Gate workflow to the shared `heurema/repo-governance` action while keeping Punk-specific policy local.

## Research Gate

Classification: R1
Required: yes
Rationale:
This changes repository governance automation by moving PR Intake Gate execution to a shared action. It does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior.
Research refs:
- `knowledge/research/2026-04-29-pr-intake-gate-review.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/RESEARCH-GATE.md`
- `CONTRIBUTING.md`
Decision:
Use the shared action engine and keep `.github/pr-intake-gate.yml` as Punk-local policy.

## Files Changed

- `.github/workflows/pr-intake-gate.yml`
- `.github/pr-intake-gate.yml`
- `CHANGELOG.md`
- `work/STATUS.md`
- `work/goals/goal_migrate_pr_intake_gate_to_repo_governance_v0_1.md`
- `work/reports/2026-04-30-pr-intake-gate-repo-governance-migration-v0-1.md`

## Implementation Summary

- Replaced the workflow step that ran `python3 scripts/pr_intake_gate.py` with `uses: heurema/repo-governance/actions/pr-intake-gate@v0.1.0`.
- Preserved trusted base checkout and workflow permissions.
- Added Punk project metadata and high-risk description to `.github/pr-intake-gate.yml` for shared action summaries and comments.
- Left existing local script and fixture test files untouched to avoid mixing operational migration with historical cleanup.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The diff changes repo-governance automation and work-ledger state without changing canonical product docs or runtime behavior."
  touched_surfaces:
    - ".github/pr-intake-gate.yml"
    - ".github/workflows/pr-intake-gate.yml"
    - "CHANGELOG.md"
    - "work/STATUS.md"
  required_updates:
    - "CHANGELOG.md"
    - "work/STATUS.md"
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- Shared action dry-run fixture: external high-risk workflow change failed with verdict `high-risk` - PASS
- Shared action dry-run fixture: trusted admin high-risk workflow change passed - PASS
- `git diff --check` - PASS
- `scripts/check.sh docs-governance --files .github/pr-intake-gate.yml .github/workflows/pr-intake-gate.yml CHANGELOG.md work/STATUS.md work/goals/goal_migrate_pr_intake_gate_to_repo_governance_v0_1.md work/reports/2026-04-30-pr-intake-gate-repo-governance-migration-v0-1.md --report work/reports/2026-04-30-pr-intake-gate-repo-governance-migration-v0-1.md` - PASS, 0 failures, 0 warnings

## Scope Boundaries Preserved

- no Rust code changes;
- no `.punk` runtime state;
- no product-doc changes under `docs/product/**`;
- no active CLI behavior changes;
- no runtime gate decision or proofpack behavior changes;
- no provider/model/agent execution requirement.

## Deferred

- Removing or archiving the now-legacy local `scripts/pr_intake_gate.py` and `scripts/test_pr_intake_gate.py` can be a separate cleanup after shared-action rollout is stable.

## Next Recommended Action

Return to `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
