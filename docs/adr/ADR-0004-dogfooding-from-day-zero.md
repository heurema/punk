# ADR-0004: Dogfooding from day zero

Status: proposed

## Context

`punk` should eventually develop itself.

Waiting until the whole system is complete would delay learning and allow process drift.

Using the system too early for trust-critical self-modification would create false confidence.

## Decision

Dogfooding starts immediately, but in levels.

Initial dogfooding is self-tracking through `work/`, `knowledge/`, and ADRs.

Self-execution is allowed only after flow, eval, contract, gate, and proof layers are stable enough.

## Levels

- Level 0: manual project memory
- Level 1: flow-tracked work
- Level 2: eval-gated development
- Level 3: contract-tracked implementation
- Level 4: bounded self-execution
- Level 5: selective self-improvement

## Consequences

Positive:

- project process is tested on itself from day one
- goals/reports/knowledge evolve with the product
- self-hosting becomes incremental and inspectable

Negative:

- more process overhead early
- trust-surface changes require stricter review
