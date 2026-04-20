---
id: report_2026_04_20_map_frontmatter_consistency_check
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add warning-level consistency checks between `docs/product/DOCUMENTATION-MAP.md` and canonical doc frontmatter without making the map a second source of truth.

## What changed

- Extended `scripts/check_docs_governance.py` with advisory checks for `docs/product/DOCUMENTATION-MAP.md`.
- Added warnings when a map owner target is missing frontmatter, is not `authority: canonical`, is not `status: active`, or lacks `canonical_for`.
- Added warnings when a map surface is not obviously reflected in the owner `canonical_for` declarations.
- Added warnings when a canonical doc declares `canonical_for` but is not listed in the map owner table.
- Kept the new checks warning-only so frontmatter remains the authoritative owner declaration and the map remains a reader-facing view.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds warning-level consistency checks between DOCUMENTATION-MAP reader-facing rows and canonical doc frontmatter without making the map a second source of truth."
  touched_surfaces:
    - eval
    - documentation-map
  required_updates:
    - scripts/check_docs_governance.py
    - work/reports/2026-04-20-map-frontmatter-consistency-check.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `python3 scripts/check_docs_governance.py --files scripts/check_docs_governance.py work/reports/2026-04-20-map-frontmatter-consistency-check.md --report work/reports/2026-04-20-map-frontmatter-consistency-check.md`
- `python3 scripts/check_docs_governance.py --files docs/product/DOCUMENTATION-MAP.md`
- `python3 scripts/check_docs_governance.py --repo "$TMP_REPO" --files docs/product/DOCUMENTATION-MAP.md`
- `scripts/check.sh`

## What remains

- Decide later whether `README.md` should stay metadata-free or receive a dedicated `repo-entry` / `public-entry` kind in a separate tiny diff.
- Add future warning-level checks for `review_after` drift and glossary-term drift.

## Risks

- The new map/frontmatter matching is intentionally heuristic and advisory; it should not be treated as proof of semantic equivalence.
- If the map table format changes significantly, the parser will need a bounded follow-up update.

## Knowledge updates needed

- None beyond this report and the checker change.
