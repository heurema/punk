# ADR-0014: Execution-agnostic contract boundary

Status: Proposed
Date: 2026-04-24

Research refs:

- `knowledge/research/2026-04-24-contract-over-prompt.md`

Eval refs:

- `evals/specs/execution-agnostic-boundary.v0.1.md`

## Context

Punk is a local-first bounded work kernel. Current docs already make contract, eval, gate, proof, and project memory active trust surfaces, while coding-agent execution and provider adapters remain not active.

The project now needs to prevent prompt/skill/scaffold accumulation from becoming the real architecture.

## Decision

Punk will not own execution.

Punk will own:

- task contracts
- scope
- expected artifacts
- receipts
- validators
- eval reports
- gate decisions
- proofpacks
- event logs
- project-memory links

Executors may be humans, local models, coding agents, scripts, IDEs, future modules, or adapters.

Executor-local prompts, skills, memories, model settings, provider defaults, and workflow rituals are non-authoritative unless captured as scoped evidence.

The operating shorthand is:

```text
Contract over Prompt.
Validate, don't over-instruct.
```

## Consequences

Positive:

- provider/model independence;
- less prompt bloat;
- cleaner trust boundary;
- better fit with `plot / cut / gate`;
- future DevPunk/adapters can plug in without owning truth.

Negative:

- contracts and validators must be better specified;
- some useful skills may be underused if policy is too strict;
- external execution quality may vary;
- receipts/evidence shape becomes more important.

## Out of scope

- model runner;
- provider adapters;
- autonomous coding execution;
- plugin runtime;
- prompt/skill product surface;
- public CLI changes;
- `.punk/` runtime writes;
- Rust code changes.

## Gate note

This ADR is proposed, not accepted.

Only future `gate` writes final acceptance. Level 0 `done` records manual closure with evidence.
