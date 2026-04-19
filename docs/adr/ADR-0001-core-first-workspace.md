# ADR-0001: Core-first workspace

Status: proposed

## Context

The previous SpecPunk line grew functionality before the core was stable.

The new `punk` line should avoid repeating that mistake.

## Decision

Create the workspace and architectural boundaries early, but activate behavior slowly.

A crate may exist without being part of the active operator surface.

## Consequences

Positive:

- less future rewrite pressure
- clearer ownership boundaries
- easier agent-driven implementation slices

Negative:

- more empty/parked structure at the beginning
- stronger discipline required so parked crates do not become implied product promises

## Rule

Scaffold architecture early. Promote capability only through phase gates.
