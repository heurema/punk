---
id: docs_product_research_intake
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - research-intake-classification
  - adopt-defer-park-avoid-policy
  - research-idea-artifact-flow
  - research-intake-storage
related_docs:
  - docs/product/RESEARCH-GATE.md
  - docs/product/ROADMAP.md
  - docs/product/TELEMETRY.md
related_adrs:
  - docs/adr/ADR-0011-local-first-trust-telemetry.md
  - docs/adr/ADR-0012-minimal-proofpack-provenance.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
supersedes: []
superseded_by: null
---
# Research Intake

Status: active-core process doc
Last updated: 2026-04-19

Related artifacts:

- `knowledge/research/2026-04-19-project-ideas-intake.md`
- `knowledge/ideas/2026-04-19-research-idea-backlog.md`
- `docs/product/TELEMETRY.md`
- `docs/adr/ADR-0011-local-first-trust-telemetry.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`

## Purpose

Research Intake defines how external project research becomes Punk work without turning the roadmap into a grab bag.

`punk` uses research to extract mechanisms, not to copy whole products. Every idea must be classified before it touches architecture, roadmap, evals, or implementation.

## Intake rule

Every non-trivial external idea must be placed into exactly one of these outcomes:

| Outcome | Meaning | Allowed next action |
|---|---|---|
| adopt | Belongs in active-core now | Add ADR/docs/evals before or with implementation |
| defer | Valuable, but not needed for the current phase | Keep in idea backlog with trigger condition |
| park | Out of scope until roadmap explicitly promotes it | Keep out of operator path and CLI docs |
| avoid | Conflicts with Punk laws or trust model | Record as anti-pattern |

## Classification fields

Each idea should record:

- source project or work
- primitive extracted
- Punk mapping
- status: active-core, incubating, parked, future
- risk
- recommendation: adopt, defer, park, avoid
- required docs
- required evals
- score: core fit, trust impact, local-first compatibility, implementation cost, prematurity risk, differentiation

## Adopt criteria

An idea can be adopted into active-core only if it strengthens at least one of these surfaces:

- project identity
- flow state machine
- append-only event log
- local eval harness
- contract lifecycle
- gate decision
- proofpack
- inspectable state
- project memory

Adopted ideas must not introduce hidden network behavior, hidden analytics, unbounded agents, or a second source of truth.

## Defer criteria

Defer ideas that are useful but not needed for current core trust. Typical defer examples:

- richer DAG scheduling
- action caching
- remote provenance publication
- OpenTelemetry export
- richer policy language
- UI views
- collaboration/sync
- agent execution

Deferred ideas need a trigger condition, not a vague "later".

## Park criteria

Park ideas that would distort the project if introduced early:

- marketplace
- SaaS control plane
- remote orchestration
- public registry
- provider marketplace
- autonomous agents as the default operator path
- plugin ecosystem before core trust is proven

Parked ideas must not appear in CLI examples or current operator docs.

## Avoid criteria

Avoid ideas that violate Punk's trust posture:

- hidden automation
- hidden network calls
- AI writes final decisions
- modules deciding instead of assessing
- adapters owning truth
- raw prompt/source telemetry
- dashboards without local canonical state
- memory as one giant prompt
- opaque evals
- proofpacks without artifact hashes

## Required artifact flow

```text
external source
  -> research note in knowledge/research/
  -> idea or anti-pattern in knowledge/ideas/
  -> ADR candidate if architecture changes
  -> eval spec if it touches trust surfaces
  -> roadmap/status patch only after classification
```

## Canonical storage

Research notes live in `knowledge/research/`.

Idea backlog lives in `knowledge/ideas/`.

Accepted project work lives in `work/` and relevant product docs.

Runtime or derived evidence lives under `.punk/`.

Public narrative derived from accepted work lives in `public/`.

## Review cadence

Research intake should be reviewed when:

- a phase boundary is crossed
- an architecture ADR is proposed
- an idea touches event log, gate, proof, eval, project memory, or sandbox permissions
- public narrative changes
- dogfooding exposes drift

## Current adopted research mechanisms

As of 2026-04-19, the following mechanisms are recommended for active-core adoption:

1. local append-only event ledger
2. event replay for inspectable state
3. minimal proofpack manifest with artifact hashes
4. rule explanation for guard denials
5. local eval receipt with machine JSON and human Markdown report
6. no-network default with explicit grants later
7. redaction layer for telemetry and receipts
8. derived inspect views that are regenerable from canonical evidence
9. project-memory link graph across goals, contracts, reports, decisions, proofs, and public narrative

## Current parked mechanisms

The following remain parked until explicit roadmap promotion and Research Gate:

1. remote telemetry export
2. remote transparency log integration
3. cloud sync
4. SaaS workspace
5. marketplace
6. rich plugin API
7. autonomous agent execution
8. team collaboration primitives
9. UI-first workflow
