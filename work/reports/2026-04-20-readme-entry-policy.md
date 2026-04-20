---
id: report_2026_04_20_readme_entry_policy
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Clarify `README.md` as the repo/public entry surface that remains outside canonical product-doc frontmatter migration for now.

## What changed

- Clarified in `docs/product/DOCUMENTATION-MAP.md` that `README.md` is the repo/public entry surface.
- Clarified in `docs/product/DOCUMENTATION-MAP.md` that `README.md` is intentionally outside canonical product-doc frontmatter migration for now.
- Clarified in `docs/product/DOCUMENTATION-MAP.md` that `README.md` may summarize and link, but must not own canonical product truth.
- Clarified the same policy in `docs/product/DOC-GOVERNANCE.md`.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Clarifies README.md as repo/public entry surface outside product-doc frontmatter migration."
  touched_surfaces:
    - documentation-map
    - project-memory
  required_updates:
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/DOC-GOVERNANCE.md
    - work/reports/2026-04-20-readme-entry-policy.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `scripts/check.sh docs-governance --files docs/product/DOCUMENTATION-MAP.md docs/product/DOC-GOVERNANCE.md work/reports/2026-04-20-readme-entry-policy.md --report work/reports/2026-04-20-readme-entry-policy.md`
- `scripts/check.sh`

## What remains

- Decide later whether `README.md` should remain metadata-free permanently or get a dedicated `repo-entry` / `public-entry` kind in a separate design diff.
- Add future advisory warnings for review windows, glossary drift, and duplicate definition candidates.

## Risks

- This policy intentionally leaves `README.md` without frontmatter, so any future move to add metadata must stay a separate bounded design decision.
- Entry-doc summaries can still drift if maintainers forget to update links; the policy clarifies boundaries but does not make README an authority surface.

## Knowledge updates needed

- None beyond this report and the policy wording.
