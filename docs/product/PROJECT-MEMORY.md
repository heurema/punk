# Project Memory

## Purpose

Single-contract correctness is not enough.

`punk` must also preserve project trajectory across many goals, contracts, reports, and decisions.

## Components

```text
Project Memory Plane
├─ Work Ledger / Contract Tracker
├─ Knowledge Vault
└─ Project Coherence Gate
```

## Repo-tracked memory

```text
work/
  goals/
  reports/
  views/

knowledge/
  product/
  architecture/
  decisions/
  research/
  experiments/
  ops/
  ideas/
  code/
```

## Runtime/derived memory

```text
.punk/
  project/
  indexes/
  contracts/
  runs/
  decisions/
  proofs/
  views/
```

## Knowledge authority

Every knowledge artifact has:

- `status`
- `authority`

Recommended authorities:

- `canonical`
- `advisory`
- `speculative`

Implementation retrieval excludes speculative knowledge by default.

## Project coherence

The project-level gate asks:

Are all accepted contracts still moving the project in the intended direction?

It checks:

- accepted contracts since last review
- touched components
- project scope expansion
- missing knowledge updates
- stale docs
- unresolved cleanup obligations
- blocked/escalated work


## Research as input to project memory

Research notes live under `knowledge/research/`.

They are advisory by default.

They can support:

- ADRs
- roadmap decisions
- contract constraints
- eval case design
- knowledge updates

Research is not canonical truth until promoted.

## Public memory

Public narrative is part of project memory.

Public-facing artifacts live under `public/`.

They are separate from internal `knowledge/`, but can link to canonical knowledge and goals.

The future PubPunk module must use this structure.
