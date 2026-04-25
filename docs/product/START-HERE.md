---
id: docs_product_start_here
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-25
review_after: 2026-07-20
canonical_for:
  - product-entry-path
  - internal-orientation
  - active-operator-surface
  - current-build-posture
  - not-active-now-boundary
related_docs:
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
  - docs/product/PUNK-LAWS.md
related_adrs:
  - docs/adr/ADR-0004-dogfooding-from-day-zero.md
supersedes: []
superseded_by: null
---

# Start Here

## Purpose

This page is the internal entry point for working on `punk`.

It tells you what is active now, what is not active yet, and which documents own each kind of project truth.

## What Punk is

`punk` is a local-first bounded work kernel.

It gives every module the same lifecycle grammar:

```text
plot -> cut -> gate
```

- `plot` shapes work and creates a contract.
- `cut` executes bounded approved work.
- `gate` verifies evidence and writes the final decision.
- proof/proofpack makes the decision inspectable before acceptance.

Only `gate` writes final decisions.

## Current build posture

Core first. Modules later.

Do not promote a capability into the active surface until:

1. it has a flow transition
2. it has at least one eval case
3. it writes inspectable artifacts
4. it does not bypass `gate`
5. it does not create a second source of truth

## Status vocabulary

Use these terms exactly:

- `active-core` — part of the stability target; must stay green
- `incubating` — exists and is tested, but not default/user-facing
- `parked` — boundary exists; minimal stub/docs only
- `retired` — removed or legacy-only

See `docs/product/DOCUMENTATION-MAP.md` and `docs/product/GLOSSARY.md`.

## Active now

The active target is the stable core:

- `punk init`
- `punk flow inspect`
- `punk eval run smoke`
- core flow state
- append-only events
- minimal proof-bearing contract loop after flow is stable

The active trust surfaces are:

- project identity
- flow state machine
- event log
- eval harness
- contract lifecycle
- gate decision
- proofpack
- inspectable state
- project-memory links

## Not active now

Do not build or describe these as current operator paths:

- autonomous coding agent execution
- PubPunk publishing automation
- provider adapters
- MCP integration
- knowledge embeddings as project truth
- plugin marketplace
- SaaS workspace
- cloud sync
- UI-first workflow

Parked/future ideas may exist in docs, research, or idea backlog, but must stay clearly labelled.

## Documentation system of record

Read in this order for architecture/product work:

1. `docs/product/PUNK-LAWS.md` — hard laws
2. `docs/product/ARCHITECTURE.md` — current structural boundaries
3. `docs/product/ROADMAP.md` — phase gates and promotion criteria
4. `docs/product/CRATE-STATUS.md` — crate/folder status
5. `docs/product/RESEARCH-GATE.md` — when research is required
6. `docs/product/RESEARCH-INTAKE.md` — how external ideas are classified
7. `docs/product/TELEMETRY.md` — local trust telemetry
8. `docs/product/EVAL-PLANE.md` — eval semantics
9. `docs/product/PROJECT-MEMORY.md` — repo-tracked memory and authority
10. `docs/product/PUBLIC-NARRATIVE.md` — public-build artifacts
11. `docs/product/DOC-GOVERNANCE.md` — documentation lifecycle and `DocImpact`
12. `docs/product/DOCUMENTATION-MAP.md` — canonical owner registry
13. `docs/product/GLOSSARY.md` — shared term authority

Use `docs/product/DOCUMENTATION-MAP.md` when editing docs or resolving conflicts.

## Documentation integrity

Documentation is part of project memory.

Meaningful changes should declare `DocImpact`, update the canonical truth owner, and move replaced truth into `superseded`, `archived`, or `retired` state instead of silently deleting it.

See:

- `docs/product/DOC-GOVERNANCE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `evals/specs/docs-consistency.v0.1.md`

## Research intake rule

Before adopting an idea from another project, classify it as exactly one of:

- `adopt`
- `defer`
- `park`
- `avoid`

Adopt only if it strengthens active-core trust surfaces.

Research is advisory until promoted through ADR, roadmap, goal/contract, implementation, eval, and proof.

See:

- `docs/product/RESEARCH-GATE.md`
- `docs/product/RESEARCH-INTAKE.md`
- `knowledge/research/`
- `knowledge/ideas/`

## Dogfooding from day zero

Before `punk` can execute work, it can still track work:

- create goals under `work/goals/`
- write reports under `work/reports/`
- keep knowledge under `knowledge/`
- record architectural decisions under `docs/adr/`

Do not claim self-execution until the required dogfooding level is reached.

See `docs/product/DOGFOODING.md`.

## Public build from day zero

The public narrative starts before code automation.

Use `publishing/` to preserve stories, posts, manual publication receipts, and metrics snapshots.

PubPunk automation may later adopt this structure.

See `docs/product/PUBLIC-NARRATIVE.md`.

## First working rule

If a change touches core laws, flow semantics, eval policy, storage, module interfaces, project memory, knowledge retrieval, adapter boundaries, or public CLI contract, check Research Gate before implementation.

If a change promotes a future or parked capability, update roadmap/status docs and add eval implications.
