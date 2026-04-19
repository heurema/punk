# ADR-0006: Research gate before major decisions

Status: proposed

## Context

The project should not make important architecture decisions only from internal brainstorming.

Adjacent systems have already discovered failure modes in task tracking, evals, workflow persistence, local-first sync, knowledge retrieval, and agent coordination.

## Decision

Introduce a Research Gate for important architecture/product decisions.

A research note is required before implementing changes that affect:

- core laws
- flow semantics
- eval policy
- storage model
- module interface
- project memory
- knowledge retrieval
- adapter boundaries
- external side effects
- roadmap phase boundaries
- public CLI contract

## Source policy

Prefer:

1. official docs and standards
2. mature project docs and repositories
3. papers and benchmark methodology docs
4. concrete issue/PR/postmortem evidence

Do not rely on weak commentary for core decisions.

## Consequences

Positive:

- fewer avoidable architecture mistakes
- better contracts
- better eval cases
- reusable knowledge base

Negative:

- slower major decisions
- research notes must be maintained
- requires discipline to avoid research sprawl
