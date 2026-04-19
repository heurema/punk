# ADR-0003: Project memory plane

Status: proposed

## Context

Single-contract correctness does not guarantee long-term project coherence.

Many individually scoped accepted contracts can still produce project drift.

## Decision

Add a Project Memory Plane:

- Work Ledger / Contract Tracker
- Knowledge Vault
- Project Coherence Gate

## Storage rule

Repo-tracked memory lives in:

- `work/`
- `knowledge/`

Runtime and derived artifacts live in `.punk/`.

## Consequences

Positive:

- long-term development becomes inspectable
- accepted contracts can be reviewed as a trajectory
- project drift becomes visible

Negative:

- adds governance overhead
- requires authority/status discipline for knowledge
