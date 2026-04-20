---
id: report_2026_04_20_publishing_plane_rename
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Rename the repo-tracked public narrative plane from `public/` to `publishing/` before more references and automation accumulate around the old name.

## What changed

- Renamed the top-level repo-tracked publication surface from `public/` to `publishing/`.
- Updated canonical docs, ADRs, templates, goals, eval specs, and the docs-governance checker to use the new path.
- Updated site/source links and repo tree examples to point at `publishing/`.
- Recorded the naming rationale in canonical docs so the publication plane is not confused with a conventional frontend `public/` static-assets directory.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Renames the repo-tracked publication truth surface from public/ to publishing/ to remove naming ambiguity and align future PubPunk adoption."
  touched_surfaces:
    - public-narrative-plane
    - project-memory
    - documentation-map
    - doc-governance
    - site-links
  required_updates:
    - README.md
    - docs/adr/ADR-0007-public-narrative-plane.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/DOC-GOVERNANCE.md
    - scripts/check_docs_governance.py
    - site/src/data/content.ts
    - site/src/data/journal.ts
    - work/reports/2026-04-20-publishing-plane-rename.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
    - site-build
```

## Checks run

- `python3 scripts/check_docs_governance.py --repo . --files README.md docs/adr/ADR-0007-public-narrative-plane.md docs/adr/ADR-0008-knowledge-vault-boundaries.md docs/modules/pubpunk.md docs/product/ARCHITECTURE.md docs/product/DOC-GOVERNANCE.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/JOURNAL.md docs/product/MODULES.md docs/product/PROJECT-MEMORY.md docs/product/PUBLIC-NARRATIVE.md docs/product/RED-TEAM-REVIEW.md docs/product/RESEARCH-INTAKE.md docs/product/ROADMAP.md docs/product/START-HERE.md evals/specs/2026-04-19-doc-consistency-evals.md evals/specs/docs-consistency.v0.1.md publishing/README.md publishing/_templates/metrics-snapshot.md publishing/_templates/publication-receipt.md publishing/channels/_shared/RULE-HIERARCHY.md publishing/channels/_shared/review-checklist.md publishing/channels/blog.md publishing/channels/linkedin.md publishing/channels/telegram.md publishing/channels/x.md publishing/metrics/README.md publishing/narrative/story-program.md publishing/posts/0001-first-public-build-note.md publishing/publications/README.md publishing/stories/story-0001-origin.md scripts/check_docs_governance.py work/goals/goal_publish_first_punk_story.md work/reports/2026-04-20-publishing-plane-rename.md brand/landing/PUNK-LANDING-DESIGN.md`
- `npm --prefix site ci`
- `npm --prefix site run build`

## What remains

- Decide later whether the root `public/` name should stay unclaimed or be reserved for future site/static assets.
- Update any external references outside this repository that still point to `public/`.

## Risks

- Any unpublished notes, bookmarks, or cross-repo references to the old `public/` path will break until they are updated.
- Future automation must adopt `publishing/` directly and should not reintroduce `public/` as a parallel truth surface.

## Knowledge updates needed

- None beyond this report and the updated canonical docs.
