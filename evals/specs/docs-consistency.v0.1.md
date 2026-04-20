# Docs consistency eval spec v0.1

Date: 2026-04-20
Status: proposed eval backlog

## Purpose

Treat documentation drift as a first-class assessable failure mode inside the existing `plot / cut / gate` lifecycle.

These evals assess docs integrity.

They do not accept work.

## Scope

The eval family covers:

- `DocImpact` presence and shape for meaningful changes;
- canonical owner consistency;
- frontmatter/status/authority on new or touched canonical docs;
- local link integrity;
- superseded/archive/retire discipline;
- research-vs-truth boundaries;
- code/doc drift for declared public or architecture surfaces.

## Rollout model

This is a cross-phase eval family, not a new runtime phase.

- **Phase 1:** define the spec, owner registry, glossary, and manual report discipline.
- **Phase 2:** run deterministic smoke checks on touched/current docs.
- **Phase 3:** accepted contract schema requires `doc_impact` or explicit `classification: none` rationale.
- **Phase 4:** project-memory link graph includes doc impact, supersession, and archive refs.
- **Phase 5:** project coherence gate consumes stale-doc and missing-update findings.

## Hard-gate checks

### docs_frontmatter_schema

Purpose: new or touched canonical docs carry required metadata.

Expected result:

- frontmatter exists;
- `id`, `kind`, `status`, `authority`, `owner`, `created_at`, `updated_at` are present;
- `status` and `authority` are allowed values;
- supersession/archive fields are present when relevant.

Catches:

- ownerless current truth;
- silent lifecycle state;
- historical docs pretending to be current.

### docs_no_broken_links

Purpose: local doc links resolve.

Expected result:

- every relative Markdown link points to an existing file or anchor target;
- moved/archived docs update inbound refs.

Catches:

- orphaned readers;
- broken archive replacements.

### docs_single_canonical_owner

Purpose: one current truth owner per surface.

Expected result:

- every truth surface in `DOCUMENTATION-MAP.md` has one canonical owner;
- entry docs do not redefine the owner surface.

Catches:

- duplicate truths;
- README drift over canonical docs.

### docs_no_parked_as_active

Purpose: parked or future capability is not presented as active.

Expected result:

- parked/future/module-hosted paths are labeled accurately;
- active docs do not show parked capabilities as default operator path.

Catches:

- roadmap inflation;
- accidental public surface promotion.

### docs_gate_decides_only

Purpose: doc text preserves assessment vs decision boundary.

Expected result:

- rules, evals, modules, and policies assess;
- only `gate` is described as the final decision writer.

Catches:

- documentation that weakens Punk law 10 by wording drift.

### docs_research_not_truth

Purpose: research and idea artifacts stay advisory/speculative until promoted.

Expected result:

- research/idea references in current docs are labeled as inputs or evidence;
- architecture or implementation truth cites ADR/roadmap/contract promotion when required.

Catches:

- copying research into truth without promotion;
- speculative backlog treated as active path.

### docs_superseded_not_current

Purpose: replaced truth stops being the default reference.

Expected result:

- superseded docs declare `superseded_by`;
- current docs and maps point to the replacement;
- historical references remain allowed in ADR/research/archive context.

Catches:

- stale current references;
- half-completed replacements.

### docs_archive_requires_replacement

Purpose: archives and retirements are explicit.

Expected result:

- archived docs record replacement or `archived_reason`;
- retired docs record `retired_reason`;
- deletion without reason is rejected for current truth.

Catches:

- silent truth loss;
- broken historical chain.

### docs_public_claim_supported

Purpose: public/operator claims are backed by canonical source or clearly marked opinion.

Expected result:

- public claims link to canonical owner, ADR, receipt, or status surface;
- unsupported claims are warnings or failures depending on severity.

Catches:

- marketing drift;
- status claims with no owner.

### docs_code_surface_requires_doc_impact

Purpose: meaningful changes cannot skip docs declaration.

Expected result:

- code/doc changes that touch declared public, architecture, dependency, or roadmap surfaces include `DocImpact`;
- `classification: none` includes a concrete rationale.

Catches:

- silent docs omission;
- untracked architectural drift.

## Warning-level checks

### docs_duplicate_definition_candidate

Term or rule appears to be defined in multiple places without an explicit owner.

### docs_stale_review_window

`review_after` is in the past.

### docs_todo_in_canonical_doc

Canonical docs still contain unresolved TODO/FIXME markers.

### docs_weak_source_in_research_claim

Tier C or weak sources carry more weight than the claim warrants.

### docs_missing_glossary_term

A new shared term appears without a glossary definition.

## Initial deterministic inputs

- changed paths from the current diff;
- `DocImpact.required_updates`;
- `docs/product/DOCUMENTATION-MAP.md`;
- `docs/product/GLOSSARY.md`;
- frontmatter/status fields on touched docs;
- relative Markdown links.

Path-based mapping is enough for v0.1.

Full semantic or AST-driven checks are deferred.
