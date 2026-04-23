---
id: report_2026_04_23_open_source_repository_baseline
goal_id: goal_add_open_source_repository_baseline
actor: vitaly
created_at: 2026-04-23
kind: handoff
---

## Summary

Add the first explicit open-source/community baseline for `punk`: Apache 2.0 licensing, contributor/community policy files, GitHub issue/PR intake, and work-ledger tracking for the scope override.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded repo-governance/docs/legal/config change. It does not change active-core runtime behavior, Rust crate semantics, or future parked capability boundaries.
Research refs:
- `AGENTS.md`
- `work/STATUS.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
Decision:
Proceed with repo-front licensing/community health only.

## Files Changed

- `README.md`
- `LICENSE`
- `NOTICE`
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `SUPPORT.md`
- `TRADEMARKS.md`
- `DCO.md`
- `.github/CODEOWNERS`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `.github/ISSUE_TEMPLATE/bug_report.yml`
- `.github/ISSUE_TEMPLATE/feature_request.yml`
- `.github/ISSUE_TEMPLATE/question.yml`
- `.github/ISSUE_TEMPLATE/config.yml`
- `work/goals/goal_add_open_source_repository_baseline.md`
- `work/reports/2026-04-23-open-source-repository-baseline.md`
- `work/STATUS.md`

## Implementation Summary

- adopts Apache License 2.0 for the repository;
- adds contributor, conduct, security, support, trademark, and DCO files;
- adds CODEOWNERS, issue templates, and a PR template tailored to Punk's Research Gate / Work Ledger workflow;
- keeps branding and handoff assets under separate rights via `NOTICE` and `TRADEMARKS.md`;
- keeps the active `selected_next` unchanged after this user-directed scope override.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds repo-front governance/legal/community docs and work-ledger tracking without changing canonical product docs."
  touched_surfaces:
    - "README.md"
    - "work/STATUS.md"
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files README.md work/STATUS.md work/goals/goal_add_open_source_repository_baseline.md work/reports/2026-04-23-open-source-repository-baseline.md --report work/reports/2026-04-23-open-source-repository-baseline.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals README.md CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md SUPPORT.md TRADEMARKS.md || true`
- `git diff --check`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code changes
- no `.punk/`
- no product-doc changes under `docs/product/`
- no active-core promotion
- no provider, marketplace, SaaS, or UI surface activation

## Deferred

- fourth advisory Work Ledger Review remains the live `selected_next`
- any active-core/product/runtime changes
- brand/reference asset provenance audit beyond explicit policy separation

## Next Recommended Action

`work/goals/goal_run_fourth_work_ledger_review.md`
