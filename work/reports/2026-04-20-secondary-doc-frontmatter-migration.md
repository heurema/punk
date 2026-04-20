---
id: report_2026_04_20_secondary_doc_frontmatter_migration
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add doc-artifact frontmatter to the secondary canonical product docs without changing their semantic text.

## What changed

- Added frontmatter to `docs/product/START-HERE.md`.
- Added frontmatter to `docs/product/FLOW.md`.
- Added frontmatter to `docs/product/EVAL.md`.
- Added frontmatter to `docs/product/EVAL-PLANE.md`.
- Added frontmatter to `docs/product/CONTRACT-TRACKER.md`.
- Added frontmatter to `docs/product/MODULES.md`.
- Added frontmatter to `docs/product/PUBLIC-NARRATIVE.md`.
- Left document bodies unchanged.
- Intentionally left `README.md` out of this migration because it is a repo/public entry surface and needs a separate policy decision if it receives metadata later.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Adds doc-artifact frontmatter to secondary canonical product docs without semantic changes."
  touched_surfaces:
    - documentation-map
    - project-memory
  required_updates:
    - docs/product/START-HERE.md
    - docs/product/FLOW.md
    - docs/product/EVAL.md
    - docs/product/EVAL-PLANE.md
    - docs/product/CONTRACT-TRACKER.md
    - docs/product/MODULES.md
    - docs/product/PUBLIC-NARRATIVE.md
    - work/reports/2026-04-20-secondary-doc-frontmatter-migration.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `scripts/check.sh docs-governance --files docs/product/START-HERE.md docs/product/FLOW.md docs/product/EVAL.md docs/product/EVAL-PLANE.md docs/product/CONTRACT-TRACKER.md docs/product/MODULES.md docs/product/PUBLIC-NARRATIVE.md work/reports/2026-04-20-secondary-doc-frontmatter-migration.md --report work/reports/2026-04-20-secondary-doc-frontmatter-migration.md`
- `scripts/check.sh`

## What remains

- Decide separately whether `README.md` should stay metadata-free or adopt a dedicated `repo-entry` / `public-entry` kind.
- Add warning-level consistency checks later for `DOCUMENTATION-MAP.md` summaries versus canonical doc frontmatter.

## Risks

- This migration intentionally avoids semantic edits, so any future ownership cleanup still belongs in separate bounded diffs.
- `DOCUMENTATION-MAP.md` remains a human-readable view; it should not become a second owner authority surface.

## Knowledge updates needed

- None beyond the added frontmatter and this migration report.
