# ADR-0009: Contract Tracker as core work ledger primitives

Status: Proposed
Date: 2026-04-19

## Context

`punk` is an experimental, early-stage, local-first bounded work kernel.

The active-core target is already constrained around project identity, flow state machine, append-only event log, eval harness, contract lifecycle, gate decision, proofpack, and inspectable state.

Current product docs already establish that:

- the universal grammar is `plot / cut / gate`;
- new capabilities cannot bypass flow, eval, inspectable artifacts, or `gate`;
- every executable goal resolves to a contract;
- every transition writes an event;
- every run writes a receipt;
- only `gate` writes final decisions;
- project memory includes a Work Ledger / Contract Tracker surface.

The open question is how to introduce a Contract Tracker without accidentally building a second workflow system, a Jira clone, or a new module that competes with the core lifecycle.

## Decision

Treat the Contract Tracker as Punk's core Work Ledger view over lifecycle artifacts.

Do not treat it as:

- a new module;
- a replacement lifecycle;
- a generic task tracker;
- a second source of truth above flow state.

Adopt the minimal object model:

```text
Plot -> Contract -> Cut -> Run -> Receipt -> Proofpack -> GateDecision -> EventLog
```

Adopt now:

- Contract Tracker as a documentation-level concept;
- `Cut` as the bounded execution slice in the lifecycle;
- gate-only closure semantics;
- proofpack as verifiable evidence bundle;
- assessments from rules/evals/modules as advisory evidence only.

Defer:

- full inspect views beyond minimal ledger output;
- dependency graphs;
- evidence freshness automation;
- project coherence scoring details.

Park:

- AI triage;
- agent delegation;
- skills as core primitives;
- customer or support integrations;
- adapter-backed tracker surfaces.

Avoid:

- a separate `punk-tracker` crate at this stage;
- status-driven closure as final truth;
- agent-owned accountability;
- module or adapter decision writes;
- workflow-builder semantics.

## Consequences

Positive:

- reinforces existing Punk Laws instead of diluting them;
- gives Project Memory a concrete ledger shape;
- keeps future tracker work inside Phase 3-5 instead of creating a parallel product;
- preserves gate as the only final decision writer.

Negative / costs:

- the tracker remains intentionally narrow in early phases;
- inspect ergonomics may lag behind the conceptual model;
- `Cut` still needs a more precise storage boundary.

## Scope classification

- Active-core: contract, cut, run, receipt, proofpack, gate decision, event log, minimal inspectable ledger view.
- Incubating: richer ledger projections, follow-up obligations, dependency views.
- Parked: AI triage, agent delegation, skills, external signal ingestion, adapters.
- Avoided for now: dedicated tracker module/crate and configurable workflow engine.

## Touched laws / phases / risks

Laws: 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18.

Phases:

- Phase 3: prove the contract loop without agents;
- Phase 4: expose ledger artifacts through project memory;
- Phase 5: use accepted ledger history for coherence/drift checks.

Risks:

- a proposed tracker CLI could be mistaken for active public surface;
- proofpacks could devolve into checklists without real evidence value;
- teams may reintroduce `done` semantics next to gate decisions.

Mitigation:

- mark tracker CLI as design target only;
- keep `gate` as the only final closure writer;
- keep proofpack requirements evidence-linked and inspectable;
- block any second source of truth in roadmap and implementation review.

## Required project updates

Add now:

- `docs/product/CONTRACT-TRACKER.md`
- `knowledge/research/2026-04-19-contract-tracker-prior-art.md`
- `knowledge/ideas/2026-04-19-contract-tracker-ideas.md`
- `work/goals/goal_contract_tracker_research_promotion.md`

Defer until review/gate:

- wording updates in `docs/product/ROADMAP.md`
- any cross-links from `docs/product/START-HERE.md`
- any public CLI exposure
- any new crate or runtime storage changes

## Open questions

1. Should `Cut` be persisted explicitly or derived from contract/run/event state?
2. Where should accepted proof summaries surface: `.punk/` only, or also repo-tracked reports?
3. What gate outcome vocabulary is minimal but sufficient for Phase 3?
4. When should dependency relationships enter scope?
5. How should stale evidence be detected without heavy automation?
