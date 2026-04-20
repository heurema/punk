# Start Here

## Purpose

This packet defines the first build direction for `punk`.

The goal is to avoid the failure mode where the product grows faster than the core stabilizes.

## Product thesis

`punk` is a bounded work kernel.

It gives every module the same operational grammar:

- `plot` — shape work and create a contract
- `cut` — execute bounded work
- `gate` — verify, decide, and prove

## Core-first rule

Do not promote a capability into the active surface until:

1. it has a flow transition
2. it has at least one eval case
3. it writes inspectable artifacts
4. it does not bypass `gate`
5. it does not create a second source of truth

## Workspace rule

Scaffold architecture early. Activate behavior slowly.

Status vocabulary:

- `active-core` — part of the stability target; must stay green
- `incubating` — exists and is tested, but not default/user-facing
- `parked` — boundary exists, minimal stub/docs only
- `retired` — removed or legacy-only

## First build slice

Only build:

- `punk init`
- `punk flow inspect`
- `punk eval run smoke`
- core flow state
- append-only events
- minimal proof-bearing contract loop after flow is stable

Do not build modules before the flow and eval layers are stable.


## Dogfooding from day zero

The project must use its own project memory immediately.

Before `punk` can execute work, it can still track work:

- create goals under `work/goals/`
- write reports under `work/reports/`
- keep knowledge under `knowledge/`
- record architectural decisions under `docs/adr/`

Do not claim self-execution until the required dogfooding level is reached.


## Research before major decisions

Important architecture/product decisions must pass the Research Gate before implementation.

This does not mean every small edit needs research.

Research is required when changing:

- core laws
- flow semantics
- eval policy
- storage model
- module interfaces
- project memory
- knowledge retrieval
- adapter boundaries
- public CLI contract

See `docs/product/RESEARCH-GATE.md`.

## Research intake rule

Before adopting an idea from another project, classify it as `adopt`, `defer`, `park`, or `avoid`.

Adopt only if it strengthens active-core trust surfaces:

- project identity
- flow state
- event log
- eval harness
- contract lifecycle
- gate decision
- proofpack
- inspectable state
- project memory

Parked and future-only ideas must not appear as the current operator path.

## Documentation integrity

Documentation is part of project memory.

Meaningful changes should declare `DocImpact`, update the canonical truth owner, and move replaced truth into `superseded`, `archived`, or `retired` state instead of silently deleting it.

Documentation refs:

- `docs/product/DOC-GOVERNANCE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`

Research intake refs:

- `docs/product/RESEARCH-INTAKE.md`
- `knowledge/research/2026-04-19-project-ideas-intake.md`
- `knowledge/ideas/2026-04-19-research-idea-backlog.md`

## Public build from day zero

The public narrative starts before code automation.

Use `public/` to preserve stories, posts, manual publication receipts, and metrics snapshots.

PubPunk automation will later adopt this structure.

See `docs/product/PUBLIC-NARRATIVE.md`.
