id: report_2026_04_21_remove_accidental_public_duplication
goal_id: goal_publish_first_punk_story
actor: vitaly
created_at: 2026-04-21
kind: handoff
maintenance_reason: "Remove accidental duplicate public surface; canonical public narrative lives under publishing/."
---

## Goal

Remove the accidentally reintroduced top-level `public/` duplicate so `publishing/` remains the only repo-tracked public narrative truth surface.

## What changed

- Deleted the accidentally reintroduced files under `public/channels/`.
- Kept `publishing/channels/` as the canonical location for shared channel rules and voice/style sources.
- Left `site/public/` untouched because it is a separate site static-assets directory, not the project-memory publication plane.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Removes accidentally reintroduced top-level public/ channel duplicates so publishing/ remains the only canonical public narrative surface."
  touched_surfaces:
    - public-narrative-plane
    - project-memory
  required_updates:
    - work/reports/2026-04-21-remove-accidental-public-duplication.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `scripts/check.sh docs-governance --files work/reports/2026-04-21-remove-accidental-public-duplication.md --report work/reports/2026-04-21-remove-accidental-public-duplication.md`
- `git ls-files public`
- `find public -type f`

## What remains

- Optionally add a separate guard so top-level `public/` duplicates cannot reappear in future commits.

## Risks

- Historical reports and docs will still mention the old `public/` path where they describe the rename or the accidental regression; that history is intentional.
- This repair does not add a prevention rule yet; it only restores the intended tree shape.

## Knowledge updates needed

- `publishing/` remains the only repo-tracked public narrative truth surface.
