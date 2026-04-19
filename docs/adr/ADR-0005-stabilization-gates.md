# ADR-0005: Stabilization gates before expansion

Status: proposed

## Context

The main project risk is feature expansion before core stability.

## Decision

Each roadmap phase has explicit stabilization gates.

A phase cannot promote if:

- smoke evals fail
- hard gates regress
- flow transitions are missing
- docs do not match active status
- new runtime truth bypasses the event/artifact model
- dogfooding level is overstated

## Consequences

Positive:

- protects the core from feature pressure
- makes scope creep visible
- gives agents a clear stop condition

Negative:

- slows down early feature delivery
- requires maintaining docs/evals with code
