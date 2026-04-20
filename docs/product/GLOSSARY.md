---
id: docs_product_glossary
kind: glossary
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - shared-terms
related_docs:
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/DOC-GOVERNANCE.md
supersedes: []
superseded_by: null
---

# Glossary

## Lifecycle terms

### plot

The shaping phase.

It defines intent, scope, constraints, and the contract candidate.

### cut

The bounded execution phase.

It runs work inside approved scope and produces receipts, not final decisions.

### gate

The final decision phase.

`gate` can accept, reject, or request rework after inspecting evidence.

### contract

The bounded work artifact produced from `plot`.

A meaningful contract will eventually carry `DocImpact` when docs truth may change.

### receipt

Inspectable evidence of what a run actually did.

Receipts record actions and outputs; they do not close work.

### proofpack

A minimal provenance bundle linking or hashing the contract, receipts, evals, and gate decision.

### assessment

An advisory result from rules, evals, guards, modules, or adapters.

Assessments may inform `gate`, but they are not final decisions.

### decision

The final closure artifact written only by `gate`.

## Documentation and memory terms

### project memory

Repo-tracked goals, reports, knowledge, docs, ADRs, and public artifacts plus rebuildable runtime views.

### canonical

The current owner of truth for a surface.

Canonical docs may be summarized elsewhere, but they should not be contradicted.

### advisory

Useful evidence or guidance that can support decisions but is not current implementation truth by itself.

### speculative

Idea-level content that must stay out of default implementation truth until promoted.

### DocImpact

The explicit declaration of whether and how a change affects docs truth, ownership, supersession, archives, or public claims.

### superseded

Still inspectable history, but no longer the current truth because another artifact replaced it.

### archived

Preserved historical artifact that should not be treated as current truth.

### retired

Intentionally ended path or capability with an explicit retirement reason.

## Status terms

### active-core

Part of the current stability target.

### incubating

Built and testable, but not the default/operator path.

### parked

Known boundary with minimal stub/docs only.

### raw

Captured but not yet shaped into stable knowledge.

### draft

Proposed but not yet accepted as current truth.

### accepted

Explicitly adopted artifact or decision.

## Research terms

### research note

An advisory snapshot under `knowledge/research/`.

Useful input, but not product truth until promoted.

### idea

A speculative candidate or backlog item under `knowledge/ideas/`.

### promotion

The path by which research or ideas become roadmap, ADR, contract, docs, or implementation truth.

Promotion does not erase the source artifact.
