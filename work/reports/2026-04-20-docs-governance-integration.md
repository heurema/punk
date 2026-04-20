---
id: report_2026_04_20_docs_governance_integration
goal_id: goal_example
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Integrate the existing docs governance checker into the repo's local and CI check paths without creating a public CLI surface or blocking on untouched legacy docs.

## What changed

- Added `scripts/check.sh` as the internal local check entrypoint.
- Wired `scripts/check_docs_governance.py` into `scripts/check.sh` with staged mode by default and explicit `docs-governance` passthrough mode.
- Added `.github/workflows/docs-governance.yml` to run the checker as a hard changed-files CI check on pull requests and pushes to `main`.
- Kept enforcement scoped to changed files by using `--base` / `--head` in CI and `--staged` locally.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Integrates the existing docs governance checker into local and CI changed-file checks."
  touched_surfaces:
    - eval
    - project-memory
  required_updates:
    - scripts/check.sh
    - .github/workflows/docs-governance.yml
    - work/reports/2026-04-20-docs-governance-integration.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `scripts/check.sh docs-governance --files scripts/check.sh .github/workflows/docs-governance.yml work/reports/2026-04-20-docs-governance-integration.md --report work/reports/2026-04-20-docs-governance-integration.md`
- `python3 scripts/check_docs_governance.py --repo /Users/vi/personal/heurema/punk --files scripts/check_docs_governance.py`
- local temp-repo assertions for pass/fail scenarios in `scripts/check_docs_governance.py`

## What remains

- Connect `scripts/check.sh` to any broader repo-local check bundle if one is added later.
- Start bounded frontmatter migration for current canonical docs.
- Expand deterministic checks later with anchors, map/frontmatter consistency, and glossary drift warnings.

## Risks

- CI currently depends on git history being available via `fetch-depth: 0`.
- The checker remains scoped and heuristic-based; legacy untouched docs are intentionally out of enforcement for now.

## Knowledge updates needed

- Future docs-governance docs may rename residual `documentation owner registry` wording to `map/view` terminology everywhere.
