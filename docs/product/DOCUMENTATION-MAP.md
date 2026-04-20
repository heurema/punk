---
id: docs_product_documentation_map
kind: documentation-map
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - documentation-reading-order
  - documentation-organization
related_docs:
  - docs/product/START-HERE.md
  - docs/product/DOC-GOVERNANCE.md
  - docs/product/GLOSSARY.md
supersedes: []
superseded_by: null
---

# Documentation Map

## Purpose

This document defines how Punk documentation is organized so the project can stay coherent as docs, ADRs, research notes, ideas, evals, and public narrative grow.

The goal is to avoid duplicate truth, drift, and conflicting claims.

## Principle

Define once. Link elsewhere.

A document should own one kind of truth. Other documents may summarize it briefly, but must point back to the canonical owner.

Authoritative owner declarations belong in the frontmatter of canonical docs.

This map is the human-readable registry/view over those declarations, plus the canonical reading-order and documentation-organization policy.

## Canonical reading order

For product and architecture work, read in this order:

1. `README.md`
2. `docs/product/START-HERE.md`
3. `docs/product/DOCUMENTATION-MAP.md`
4. `docs/product/GLOSSARY.md`
5. `docs/product/PUNK-LAWS.md`
6. `docs/product/ARCHITECTURE.md`
7. `docs/product/ROADMAP.md`
8. `docs/product/CRATE-STATUS.md`
9. `docs/product/RESEARCH-GATE.md`
10. `docs/product/RESEARCH-INTAKE.md`
11. `docs/product/TELEMETRY.md`
12. `docs/product/EVAL-PLANE.md`
13. `docs/product/PROJECT-MEMORY.md`
14. `docs/product/DOC-GOVERNANCE.md`
15. `docs/product/PUBLIC-NARRATIVE.md`
16. `docs/adr/`
17. `knowledge/research/`
18. `knowledge/ideas/`
19. `work/goals/` and `work/reports/`
20. `public/`

## Canonical owners

| Truth surface | Canonical owner | Supporting/history refs | Notes |
|---|---|---|---|
| Product entry path | `docs/product/START-HERE.md` | `README.md` | `README.md` is the repo entry, not the deep canonical owner for every surface. |
| Core laws | `docs/product/PUNK-LAWS.md` | `docs/adr/` | Law statements outrank lower-level docs. |
| Architecture boundaries | `docs/product/ARCHITECTURE.md` | `docs/adr/`, `docs/modules/`, `docs/adapters/` | Defines planes, boundaries, and truth ownership. |
| Flow semantics | `docs/product/FLOW.md` | `docs/adr/ADR-0002-flow-and-eval-before-features.md` | `plot / cut / gate` lifecycle source. |
| Eval operator contract | `docs/product/EVAL.md` | `docs/product/EVAL-PLANE.md`, `evals/` | Product direction for eval behavior. |
| Eval runtime/report shape | `docs/product/EVAL-PLANE.md` | `evals/specs/`, `evals/cases/` | Architecture and report expectations. |
| Contract/work ledger direction | `docs/product/CONTRACT-TRACKER.md` | `docs/adr/ADR-0009-contract-tracker-core-primitives.md` | Phase-gated contract loop and ledger view. |
| Roadmap and phase gates | `docs/product/ROADMAP.md` | `work/goals/`, `docs/adr/` | Current phase order and exit criteria. |
| Crate maturity/status | `docs/product/CRATE-STATUS.md` | `docs/product/ROADMAP.md` | Current crate boundary vocabulary. |
| Project memory model | `docs/product/PROJECT-MEMORY.md` | `docs/adr/ADR-0003-project-memory-plane.md`, `docs/adr/ADR-0008-knowledge-vault-boundaries.md` | Repo-tracked memory surfaces and authority rules. |
| Research gate policy | `docs/product/RESEARCH-GATE.md` | `knowledge/research/` | When research is required before implementation. |
| Research intake classification | `docs/product/RESEARCH-INTAKE.md` | `knowledge/research/`, `knowledge/ideas/` | `adopt / defer / park / avoid` intake discipline. |
| Telemetry/privacy and audit evidence | `docs/product/TELEMETRY.md` | `docs/adr/ADR-0011-local-first-trust-telemetry.md` | Event/redaction/no-network truth. |
| Public narrative/public memory | `docs/product/PUBLIC-NARRATIVE.md` | `public/`, `docs/adr/ADR-0007-public-narrative-plane.md` | Public receipts and publication boundaries. |
| Module boundaries | `docs/product/MODULES.md` | `docs/product/MODULE-HOST.md`, `docs/modules/` | Product-level module status and non-goals. |
| Documentation governance | `docs/product/DOC-GOVERNANCE.md` | `evals/specs/docs-consistency.v0.1.md` | Lifecycle, ownership, supersession, `DocImpact`. |
| Documentation reading order and organization policy | `docs/product/DOCUMENTATION-MAP.md` | n/a | This file is the reader-facing registry/view; authoritative owner declarations live in canonical doc frontmatter. |
| Shared term meanings | `docs/product/GLOSSARY.md` | canonical owners above | Prevents term drift across docs. |

## Historical surfaces

| Surface | Location | Rule |
|---|---|---|
| ADR history | `docs/adr/` | Never delete accepted ADRs; mark them superseded instead. |
| Archived docs | `docs/archive/` | Historical-only; must point to replacement or retirement reason. |
| Research snapshots | `knowledge/research/` | Advisory until promoted. |
| Idea backlog | `knowledge/ideas/` | Speculative; never current implementation truth by itself. |
| Reports | `work/reports/` | Evidence/handoff, not canonical product truth. |
| Runtime evidence | `.punk/` | Inspectable runtime/derived state, never hand-edited product truth. |

## Precedence for conflicts

When two docs disagree, resolve in this order:

1. `docs/product/PUNK-LAWS.md`
2. Accepted ADRs in `docs/adr/`
3. Product docs by canonical ownership, summarized here for readers
4. `docs/product/START-HERE.md`
5. `README.md`
6. Accepted research notes in `knowledge/research/`
7. Ideas in `knowledge/ideas/`
8. Work reports in `work/reports/`
9. Public narrative in `public/`

Research and ideas do not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Update rules

### README

Use `README.md` as the public entry.

Allowed:

- short identity
- current active target
- short non-active warning
- links to canonical docs

Avoid:

- schema details
- roadmap phase details
- research rationale
- future module/operator examples

### START-HERE

Use `START-HERE.md` as the internal orientation page.

Allowed:

- reading path
- active operator surface
- short rules for what not to build now
- links to canonical docs

Avoid:

- duplicating full laws
- duplicating full roadmap
- defining schemas
- storing research conclusions directly

### Product docs

Each product doc should own one surface.

If a section explains another surface, it should be short and link to the owner.

### ADRs

ADRs explain accepted decisions and their rationale.

Do not use ADRs as the only place where the current operator path is documented. Product docs should point to ADRs and reflect accepted decisions.

### Research notes

Research notes are advisory by default. They should include adoption maps and eval implications, but they are not implementation truth until promoted.

### Idea backlog

Ideas may be useful, speculative, parked, or anti-patterns. Ideas must not appear in current operator docs unless explicitly promoted.

## Duplication policy

Allowed duplication:

- one-sentence summaries
- short active/not-active lists in entry docs
- links to canonical owners
- status labels repeated for clarity

Not allowed:

- two different definitions of the same lifecycle object
- schemas copied into multiple docs
- roadmap status repeated as if active in future/module docs
- research conclusions presented as accepted architecture without ADR/roadmap promotion
- public claims that exceed canonical docs

## Documentation consolidation checklist

Before editing docs, check:

1. Which document owns this truth?
2. Is this active-core, incubating, parked, future, or retired?
3. Does the edit change a law, roadmap phase, storage model, module boundary, adapter boundary, eval policy, project memory, or public CLI contract?
4. If yes, is Research Gate required?
5. Does the edit introduce duplicate schema text?
6. Does it imply a future capability is active?
7. Does it create or remove a source of truth?
8. Are ADR/research/idea/work refs updated?

## Documentation integrity rule

When a meaningful change touches one of these surfaces:

1. declare it in `DocImpact`;
2. update the canonical owner, not only an entry doc;
3. mark any replaced truth as superseded, archived, or retired;
4. keep links and glossary terms aligned.
