---
id: report_2026_04_20_docs_governance_v0_status
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Record the completed docs-governance v0 state and freeze further v0 checker expansion.

## Status

Docs governance v0 is established.

## What exists

- governance docs
- owner/view split
- README entry policy
- doc artifact schema
- `DocImpact` schema
- docs consistency eval spec
- repo-tracked fixture cases
- deterministic internal checker
- local hard check path
- CI changed-files hard check path
- core canonical doc frontmatter migration
- secondary canonical doc frontmatter migration

## Hard checks

- frontmatter on touched canonical docs
- relative links
- `DocImpact` for meaningful changes
- superseded/current references
- parked-as-active wording

## Warning checks

- map/frontmatter view consistency
- expired `review_after`
- glossary undeclared term headings
- glossary redefined term headings
- duplicate definition candidates

## Intentional non-goals

- no public CLI
- no repo-wide legacy blocker
- no LLM/semantic judge
- no auto-rewrite
- no external docs source of truth
- no `README.md` frontmatter migration for now

## Operating rule

New meaningful changes must carry `DocImpact`.

New and touched canonical docs must pass scoped docs-governance checks.

Warnings should be reviewed, but they do not block v0 by themselves.

## Remaining risks

- heuristics may miss semantic drift
- map/frontmatter warnings are advisory only
- `README.md` policy is documented but not schema-backed
- legacy docs outside touched scope may still lack metadata
- direct-to-main flow depends on local discipline plus CI

## V1 candidates

- better CI summaries for docs-governance findings
- generated owner view derived from frontmatter
- repo-wide warning mode
- glossary term suggestions
- inspectable docs drift/status view later

## Recommended next direction

Start v1 with better CI summaries.

Do not expand v0 checker heuristics further before that workflow improvement lands.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Records the completed docs-governance v0 state and freezes additional v0 checker expansion."
  touched_surfaces:
    - project-memory
    - eval
    - documentation-map
  required_updates:
    - work/reports/2026-04-20-docs-governance-v0-status.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `scripts/check.sh docs-governance --files work/reports/2026-04-20-docs-governance-v0-status.md --report work/reports/2026-04-20-docs-governance-v0-status.md`
- `scripts/check.sh`

## What remains

- Build a better CI summary surface for docs-governance results.
- Keep generated owner view as a later derived artifact, not a new authority source.

## Risks

- The current checker remains intentionally scoped; untouched legacy docs still rely on opportunistic migration.
- Advisory warnings depend on humans to review them even when the pipeline stays green.

## Knowledge updates needed

- None beyond this status report.
