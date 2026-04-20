---
id: docs_product_doc_governance
kind: process-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - documentation-governance
  - doc-impact-policy
  - docs-lifecycle
related_docs:
  - docs/product/PROJECT-MEMORY.md
  - docs/product/CONTRACT-TRACKER.md
  - docs/product/ROADMAP.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
related_evals:
  - evals/specs/docs-consistency.v0.1.md
supersedes: []
superseded_by: null
---

# Doc Governance

## Purpose

Documentation in `punk` is a project-memory artifact, not side text.

A meaningful change must declare doc impact, update the right truth surfaces, preserve superseded history, and leave assessable evidence for `gate`.

## Rule boundary

Documentation follows the same precedence stack as the rest of Punk:

```text
Core Rules
  -> Project Doc Rules
  -> Module Doc Rules
  -> Profile Rules
  -> Run Contract / DocImpact
```

Docs checks can assess and report.

Only `gate` can accept, reject, or request rework.

## Artifact classes and lifecycle

Current truth should have one owner.

History should remain inspectable.

| Artifact class | Primary path | Authority | Normal lifecycle |
|---|---|---|---|
| Product/process docs | `docs/product/` | canonical | `draft -> active -> superseded|retired|archived` |
| ADRs | `docs/adr/` | canonical decision history | `draft -> accepted -> superseded` |
| Research notes | `knowledge/research/` | advisory | `draft -> accepted -> superseded|archived` |
| Ideas | `knowledge/ideas/` | speculative | `raw -> draft -> accepted|superseded|archived` |
| Reports | `work/reports/` | handoff/evidence | historical; append new reports instead of rewriting old truth |
| Public docs/receipts | `public/` | public memory | `draft -> published -> archived` via receipt |

`superseded` means the artifact is still part of history but is no longer current truth.

`archived` means it is kept only as historical evidence.

`retired` means the capability or doc path is intentionally ended and must say why.

## DocImpact policy

Every meaningful change must carry a `DocImpact` block.

`DocImpact` is the reusable schema in `docs/_schema/doc-impact.v0.1.schema.yaml`.

Current adoption rule:

1. now: store the block in reports or manual contract drafts for any meaningful change;
2. Phase 3: make the same block a required top-level field in the accepted contract schema;
3. later: let proof and coherence views consume the same normalized shape.

That keeps one shape across the lifecycle without pretending the full contract loop already exists.

A valid `classification: none` is allowed, but it still needs a reason.

Minimal shape:

```yaml
doc_impact:
  classification: none|docs-only|code-doc|architecture|dependency|public-claim|research-promotion
  reason: "why docs are or are not affected"
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

Meaningful changes include:

- architecture or lifecycle changes;
- dependency choices that affect current docs or ADR truth;
- new or removed public/operator surfaces;
- public claims or status changes;
- research promoted into roadmap, ADR, contract, or canonical docs;
- docs-only changes that alter current truth ownership.

## Canonical owner and term authority

Canonical owner declarations live in the frontmatter of canonical docs.

`docs/product/DOCUMENTATION-MAP.md` is the human-readable registry/view that summarizes those owners for readers and defines reading order plus documentation organization policy.

Shared term meanings live in `docs/product/GLOSSARY.md`.

Rules:

- every current truth surface has one canonical owner declared in canonical doc frontmatter;
- `DOCUMENTATION-MAP.md` may summarize owner declarations, but it must not become a competing source of owner truth;
- entry docs may summarize but must not redefine the owner;
- `README.md` is the repo/public entry surface and is intentionally outside canonical product-doc frontmatter migration for now;
- `README.md` may summarize identity, active target, non-active warnings, and links, but it must not own canonical product truth;
- glossary terms may be narrowed by module docs but not contradicted;
- superseded docs must not remain the default inbound reference.

## Supersede, archive, retire

Do not silently delete current truth.

Use one of these paths:

1. **Supersede** — keep the old artifact, mark it `status: superseded`, and set `superseded_by`.
2. **Archive** — move historical-only docs under `docs/archive/<date>/` and record `archived_reason` plus replacement if one exists.
3. **Retire** — keep a short tombstone or replacement note with `retired_reason` when there is no direct successor.

Deletion is acceptable only for drafts or duplicates with no inbound references and a recorded reason.

## Docs consistency eval family

See `evals/specs/docs-consistency.v0.1.md`.

Adoption in v0.1 is intentionally staged:

- hard-gate checks apply to new docs and touched canonical docs named by `DocImpact`;
- repo-wide legacy docs can emit warnings until migrated;
- project coherence later turns stale/current-truth drift into a gate input.

Target hard checks:

- frontmatter schema on touched canonical docs;
- no broken local links;
- one canonical owner per truth surface;
- parked/future capability not described as active;
- research and ideas not treated as implementation truth without promotion;
- superseded or archived docs not referenced as current;
- meaningful code/doc changes must carry `DocImpact`.

Target warnings:

- duplicate term definitions;
- overdue review windows;
- TODOs inside canonical docs;
- weak source strength in research-heavy claims;
- missing glossary entries for new terms.

## Receipts, reports, and proof

Docs work should leave inspectable evidence:

- report or contract draft containing `DocImpact`;
- updated canonical docs, ADRs, or archive markers;
- docs consistency eval output;
- gate decision linking the evidence when the contract loop is active.

Docs checks assess the change.

They do not decide it.

## Adopt now

Adopt now as active-core process:

- `DocImpact` as a reusable schema and required report field for meaningful changes;
- frontmatter for new/touched canonical docs;
- documentation owner registry;
- glossary as shared term authority;
- no silent deletion;
- explicit supersede/archive/retire discipline;
- docs consistency eval spec.

Defer for later:

- public `punk docs ...` CLI;
- automatic doc rewriting;
- LLM-only semantic judges as hard gates;
- full AST/doc synchronization.
