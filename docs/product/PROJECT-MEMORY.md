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

## Link graph

Project memory should preserve explicit links across:

```text
goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative
```

This keeps memory bounded and inspectable. The graph should be built from canonical artifacts, not from one giant prompt or hidden runtime state.

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

docs/
  adr/

public/
```

## Runtime/derived memory

```text
.punk/
  project/
  indexes/
  events/
  contracts/
  runs/
  evals/
  decisions/
  proofs/
  views/
```

## Non-canonical retrieval memory

Repo-search and code-retrieval systems may keep runtime state such as:

- frecency databases
- query history
- file indexes
- symbol indexes
- vector indexes
- language-server caches

This state is retrieval memory, not project memory.

Retrieval memory may help rank or locate evidence, but it must not become a
source of implementation truth. It should be treated as runtime/derived state
and should be inspectable through receipts or explicit index metadata when it
affects a contract run.

If a retrieval result should become durable project knowledge, it must be
promoted through the normal path:

```text
retrieval receipt
  -> report / research note
  -> ADR or knowledge update
  -> contract refs
  -> gate/proof
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

When retrieval grows beyond manual repo inspection, the artifact contract should make source refs, review windows, supersession, contradiction, and proof/decision links explicit instead of hiding them in chat or runtime state.

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

## Knowledge Vault operating boundary

Knowledge Vault is the repo-tracked knowledge surface of the Project Memory Plane.

It is not:

- a runtime memory engine
- a vector DB source of truth
- hidden agent memory
- a remote/shared commons
- a module-owned decision surface

Repo-tracked truth lives in `work/`, `knowledge/`, `docs/adr/`, and `public/`.

`.punk/` may hold derived indexes and views, but derived state is rebuildable and non-authoritative.

Future retrieval must:

- stay advisory-only
- cite repo artifact paths
- exclude speculative knowledge by default
- flag stale or superseded knowledge
- surface contradictions instead of flattening them
- never write final decisions
- never bypass `plot / cut / gate`

Promotion path:

```text
idea/research -> ADR or roadmap decision -> goal/contract -> implementation -> eval result -> proof/knowledge update
```

Promotion is nomination, not move: the source artifact stays intact until a separate decision promotes or supersedes it.

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
