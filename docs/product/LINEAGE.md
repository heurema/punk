---
id: docs_product_lineage
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-29
updated_at: 2026-04-29
review_after: 2026-07-29
canonical_for:
  - project-lineage
  - adjacent-repo-boundary
  - inherited-idea-classification
  - retained-cut-parked-lineage
related_docs:
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/RESEARCH-INTAKE.md
supersedes: []
superseded_by: null
---

# Lineage

## Purpose

This document keeps adjacent-project inspiration from being misread as current `punk` behavior.

`punk` is the current repository and source of truth for this project. Adjacent repositories may inform lineage, vocabulary, and future ideas, but they do not define current `punk` behavior unless current `punk` canonical docs promote that behavior.

This document does not claim a full audit of adjacent repositories. It records a boundary for how to read lineage safely during the v0.1-prep current-truth baseline.

## Current repo authority

Current `punk` truth is owned by this repository's canonical docs, work ledger, goals, reports, ADRs, research notes, and implementation state.

Adjacent-repo language is advisory input. It must pass through Research Gate, ADR/roadmap promotion, goal/contract scope, implementation, eval, gate, proof, and project-memory links before becoming active `punk` truth.

## Adjacent sources

| Source | Retained | Cut / not inherited | Parked / future | Rule |
| --- | --- | --- | --- | --- |
| `specpunk` | contract-first framing, bounded work, proof/evidence vocabulary | older or broader runtime/operator vocabulary, unbounded operator surfaces, hidden authority | any useful legacy pattern after explicit Research Gate classification | Treat as lineage/legacy inspiration, not current `punk` behavior. |
| `signum` | proof/gate/evidence discipline, correctness-before-implementation posture | automatic implementation authority or final-decision ownership outside `gate` | proof-style workflow refinements after explicit promotion | Alignment does not automatically define `punk` implementation. |
| `goalrail` | goal-to-verified-change framing, small bounded work, process-shell discipline | server/control-plane assumptions, broad orchestration, hidden live state | GoalRail runtime pilot only after `punk` gate/proof/storage closure | Do not let `punk` prematurely become a server/control-plane product. |
| `axiom`, `mycel`, `baseline`, `content-ops`, and similar repos | documentation/process patterns, public-build lessons, useful research prompts | product truth, active behavior, provider authority, production-readiness claims | candidate patterns after repo-tracked research or explicit goal scope | Useful patterns are inputs, not canonical `punk` truth. |

## Retained lineage

Current project deliberately retains these ideas:

- bounded work;
- contract-first execution;
- evidence before acceptance;
- gate/proof discipline;
- project memory;
- docs-first governance;
- public build from day zero.

## Cut / not inherited

The current project deliberately does not inherit these as active behavior:

- broad autonomous agent runtime;
- provider-specific authority;
- cloud/SaaS control plane;
- plugin marketplace;
- hidden memory;
- unbounded operator surfaces.

## Parked / future lineage

The following may be useful later, but they remain parked or future until promoted by current `punk` docs, goals, evals, and proof:

- adapters;
- Dev module;
- Pub module;
- module host;
- repo-search adapter;
- publishing automation;
- cloud sync;
- richer `.punk/` runtime storage.

## Conflict rule

When lineage conflicts with current `punk` docs, current `punk` canonical docs win.

Priority:

1. `docs/product/START-HERE.md`
2. `docs/product/PUNK-LAWS.md`
3. `docs/product/ARCHITECTURE.md`
4. `docs/product/ROADMAP.md`
5. `docs/product/CRATE-STATUS.md`
6. `work/STATUS.md`

A lineage source can motivate a question or future research note. It cannot make a feature active, change the CLI, write project truth, bypass `gate`, or override proof-before-acceptance rules.
