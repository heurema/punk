---
id: report_2026_04_20_map_frontmatter_eval_cases
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add repo-tracked eval fixture cases for the advisory `DOCUMENTATION-MAP.md` versus frontmatter consistency warnings.

## What changed

- Added `evals/cases/docs/map-frontmatter/README.md`.
- Added pass fixture coverage for a clean map/frontmatter match.
- Added warning fixture coverage for:
  - missing frontmatter on map owner docs;
  - owner docs that are not canonical;
  - owner docs that are not active;
  - map surfaces not reflected in `canonical_for`;
  - canonical docs missing from the map owner table.
- Kept all warning scenarios as `result: pass` with `warnings`, preserving the current authority model.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Adds repo-tracked eval fixture cases for advisory map/frontmatter consistency warnings."
  touched_surfaces:
    - eval
    - documentation-map
  required_updates:
    - evals/cases/docs/map-frontmatter/README.md
    - evals/cases/docs/map-frontmatter/map-owner-row-matches-frontmatter.pass.yaml
    - evals/cases/docs/map-frontmatter/map-owner-doc-missing-frontmatter.warn.yaml
    - evals/cases/docs/map-frontmatter/map-owner-doc-not-canonical.warn.yaml
    - evals/cases/docs/map-frontmatter/map-owner-doc-not-active.warn.yaml
    - evals/cases/docs/map-frontmatter/map-surface-missing-from-canonical-for.warn.yaml
    - evals/cases/docs/map-frontmatter/frontmatter-owner-missing-from-map.warn.yaml
    - work/reports/2026-04-20-map-frontmatter-eval-cases.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `python3 - <<'PY' ...` fixture sanity check for file presence, case ids, and expected warning/pass shapes.
- `python3 scripts/check_docs_governance.py --files evals/cases/docs/map-frontmatter/README.md evals/cases/docs/map-frontmatter/map-owner-row-matches-frontmatter.pass.yaml evals/cases/docs/map-frontmatter/map-owner-doc-missing-frontmatter.warn.yaml evals/cases/docs/map-frontmatter/map-owner-doc-not-canonical.warn.yaml evals/cases/docs/map-frontmatter/map-owner-doc-not-active.warn.yaml evals/cases/docs/map-frontmatter/map-surface-missing-from-canonical-for.warn.yaml evals/cases/docs/map-frontmatter/frontmatter-owner-missing-from-map.warn.yaml work/reports/2026-04-20-map-frontmatter-eval-cases.md --report work/reports/2026-04-20-map-frontmatter-eval-cases.md`
- `scripts/check.sh`

## What remains

- Decide separately whether to add explicit spec text for the advisory map/frontmatter check family.
- Decide separately whether `README.md` should remain metadata-free or gain a dedicated entry kind.

## Risks

- These cases are fixture coverage only; they do not add a dedicated deterministic runner.
- The advisory matching remains heuristic and should not be interpreted as semantic equivalence proof.

## Knowledge updates needed

- None beyond the new fixtures and this report.
