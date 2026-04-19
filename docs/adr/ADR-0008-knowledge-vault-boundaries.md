# ADR-0008: Knowledge Vault boundaries

Status: proposed

Research refs:

- `knowledge/research/2026-04-19-knowledge-vault-prior-art-summary.md`

## Context

`punk` needs project memory from day zero, but active-core must not grow faster than flow, eval, contract, `gate`, and proof.

Prior art around shared agent knowledge contains useful patterns, but it also shows recurring risks:

- hidden sources of truth
- stale or contradictory memory
- poisoned or injected retrieval inputs
- confidence scores treated as authority
- retrieval paths that bypass decision boundaries

## Decision

Treat Knowledge Vault as a documentation and research boundary now, not as approval for a retrieval implementation.

If accepted, Knowledge Vault means:

> repo-tracked, authority-tagged project knowledge artifacts plus reconstructable derived indexes

Knowledge Vault is not:

- a vector DB source of truth
- hidden agent memory
- a remote/shared commons
- a module-owned decision surface
- an MCP integration
- a substitute for `gate` or proof

## Rules

1. Repo artifacts under `work/`, `knowledge/`, `docs/adr/`, and `public/` are truth surfaces.
2. Runtime indexes and views under `.punk/` are derived and disposable.
3. Retrieval is advisory evidence only.
4. Implementation retrieval excludes `authority: speculative` by default.
5. Stale, superseded, and contradictory knowledge must be representable before retrieval is promoted.
6. Knowledge promotion is proof-linked.
7. Promotion is nomination, not move: the source artifact remains intact until a separate decision promotes or supersedes it.
8. Confidence or reputation may inform review, but cannot reduce review or bypass `gate`.
9. No embeddings, MCP memory, remote commons, or autonomous memory promotion are active-core.

## Consequences

Positive:

- keeps `punk` local-first and inspectable
- keeps repo artifacts as the source of truth
- gives retrieval a bounded future path without changing core laws
- preserves room for richer derived indexes later

Negative:

- slows down memory feature work
- requires explicit metadata and eval fixtures before retrieval is useful
- keeps manual review in the loop

## Required follow-up before implementation promotion

- define retrieval-relevant metadata in repo-tracked artifacts
- add evals for authority filtering, stale/superseded handling, contradiction surfacing, proof-link integrity, and no-hidden-truth rebuilds
- add security evals for prompt injection and secret/PII leakage
- keep retrieval local-first and non-authoritative

## Out of scope now

- vector DB as source of truth
- MCP integration
- global/shared commons
- autonomous memory capture
- auto-promotion of agent-learned knowledge
- reputation-based bypass of review
