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
  - documentation-owner-registry
related_docs:
  - docs/product/START-HERE.md
  - docs/product/DOC-GOVERNANCE.md
  - docs/product/GLOSSARY.md
supersedes: []
superseded_by: null
---

# Documentation Map

## Purpose

This file answers one question:

Which file owns which current truth?

Entry docs may summarize.

Canonical owners define the meaning.

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
| Documentation owner registry | `docs/product/DOCUMENTATION-MAP.md` | n/a | This file. |
| Shared term meanings | `docs/product/GLOSSARY.md` | canonical owners above | Prevents term drift across docs. |

## Historical surfaces

| Surface | Location | Rule |
|---|---|---|
| ADR history | `docs/adr/` | Never delete accepted ADRs; mark them superseded instead. |
| Archived docs | `docs/archive/` | Historical-only; must point to replacement or retirement reason. |
| Research snapshots | `knowledge/research/` | Advisory until promoted. |
| Idea backlog | `knowledge/ideas/` | Speculative; never current implementation truth by itself. |
| Reports | `work/reports/` | Evidence/handoff, not canonical product truth. |

## Update rule

When a meaningful change touches one of these surfaces:

1. declare it in `DocImpact`;
2. update the canonical owner, not only an entry doc;
3. mark any replaced truth as superseded, archived, or retired;
4. keep links and glossary terms aligned.
