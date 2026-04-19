# ADR-0002: Flow and eval before features

Status: proposed

## Context

New features can make an agentic system look more powerful while reducing reliability.

GoalRail/Punk needs a way to know whether changes improve or regress the product.

## Decision

Add two foundation layers before feature expansion:

1. Flow Controller
2. Eval Plane

## Flow Controller

Commands are validated transitions over persisted state.

The LLM may suggest a step, but runtime state decides whether the step is allowed.

## Eval Plane

Every capability must have eval coverage or explicit no-eval rationale.

The initial eval system is local, repo-stored, deterministic-first, and small.

## Consequences

Positive:

- prevents skipped process steps
- creates before/after product signal
- makes roadmap phase gates enforceable

Negative:

- slower feature activation
- requires maintaining eval fixtures from the beginning
