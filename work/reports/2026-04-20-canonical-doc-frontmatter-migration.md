---
id: report_2026_04_20_canonical_doc_frontmatter_migration
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add doc-artifact frontmatter to the first batch of core canonical product docs without changing their semantic text.

## What changed

- Added frontmatter to `docs/product/PUNK-LAWS.md`.
- Added frontmatter to `docs/product/ARCHITECTURE.md`.
- Added frontmatter to `docs/product/ROADMAP.md`.
- Added frontmatter to `docs/product/CRATE-STATUS.md`.
- Added frontmatter to `docs/product/RESEARCH-GATE.md`.
- Added frontmatter to `docs/product/RESEARCH-INTAKE.md`.
- Added frontmatter to `docs/product/TELEMETRY.md`.
- Added frontmatter to `docs/product/PROJECT-MEMORY.md`.
- Kept the body text unchanged; this is a metadata-only migration batch.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Adds doc-artifact frontmatter to the first batch of core canonical product docs without semantic changes."
  touched_surfaces:
    - documentation-map
    - project-memory
  required_updates:
    - docs/product/PUNK-LAWS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/RESEARCH-INTAKE.md
    - docs/product/TELEMETRY.md
    - docs/product/PROJECT-MEMORY.md
    - work/reports/2026-04-20-canonical-doc-frontmatter-migration.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `scripts/check.sh docs-governance --files docs/product/PUNK-LAWS.md docs/product/ARCHITECTURE.md docs/product/ROADMAP.md docs/product/CRATE-STATUS.md docs/product/RESEARCH-GATE.md docs/product/RESEARCH-INTAKE.md docs/product/TELEMETRY.md docs/product/PROJECT-MEMORY.md work/reports/2026-04-20-canonical-doc-frontmatter-migration.md --report work/reports/2026-04-20-canonical-doc-frontmatter-migration.md`
- staged `scripts/check.sh`

## What remains

- Batch 2 for entry docs and secondary product owners.
- Later checker improvements for map/frontmatter consistency, review windows, and glossary warnings.

## Risks

- Some core docs still keep legacy in-body status/date lines alongside the new frontmatter until later cleanup.
- This batch intentionally does not widen enforcement to untouched legacy docs.

## Knowledge updates needed

- Keep future metadata migrations semantic-free and bounded by canonical owner groups.
