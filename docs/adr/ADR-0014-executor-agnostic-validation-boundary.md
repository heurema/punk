# ADR-0014: Executor-agnostic validation boundary

Status: Proposed
Date: 2026-04-24

Research refs:

- `knowledge/research/2026-04-24-contract-over-prompt.md`

Eval refs:

- `evals/specs/executor-agnostic-validation-boundary.v0.1.md`

## Context

Punk is a local-first bounded work kernel. Current docs already make contract, eval, gate, proof, and project memory active trust surfaces, while coding-agent execution and provider adapters remain not active.

The project now needs to prevent prompt/skill/scaffold accumulation from becoming the real architecture without pretending that Punk is detached from local validation.

Two meanings of execution must stay separate:

- execution substrate: the user's repository, filesystem, shell, IDE, toolchains, installed tools, humans, scripts, local models, coding agents, provider CLIs, environment configuration, modules, and adapters;
- executor authority: the question of who is allowed to decide that work is accepted.

Punk runs in the user's environment, but it must not trust the user's executor as truth.

## Decision

Punk is executor-agnostic, not validation-agnostic.

Punk will not own or trust the executor's hidden runtime, prompts, skills, model settings, provider defaults, local memories, workflow rituals, or self-reported success.

Punk will own the validation protocol:

- task contracts
- allowed scope
- expected artifacts
- validator plan
- receipt and evidence shape
- eval and assessment report shape
- gate decision rules
- proofpack refs and hashes
- event-log refs
- project-memory links

Executors may be humans, local models, coding agents, scripts, IDEs, future modules, or adapters.

The user runtime is the substrate, not the authority. Punk may invoke validators inside that runtime, but authority flows through contract, receipt, validator/eval evidence, gate, proof, and project-memory links.

Executor claims are not proof. Claims such as "tests passed", "scope preserved", or "done" must be verified, captured as unverified claims, or rejected/deferred by gate policy.

Semantic or LLM-based assessors may produce clause-specific advisory evidence. They cannot write final decisions, and executor self-review cannot become acceptance.

The operating shorthand is:

```text
Bring your own executor.
Punk brings the contract and the gate.

Contract over Prompt.
Validate, don't over-instruct.
```

## Consequences

Positive:

- provider/model independence;
- less prompt bloat;
- cleaner trust boundary;
- better fit with `plot / cut / gate`;
- local validators can run without making local executors authoritative;
- future DevPunk/adapters can plug in without owning truth.

Negative:

- contracts, validators, receipts, and proof requirements must be better specified;
- missing local validators need explicit defer/reject behavior;
- semantic assessments need careful evidence-vs-decision boundaries;
- some useful skills may be underused if policy is too strict;
- external execution quality may vary.

## Out of scope

- model runner;
- provider adapters;
- autonomous coding execution;
- plugin runtime;
- prompt/skill product surface;
- public CLI changes;
- contract schema changes;
- semantic assessor command interface;
- `.punk/` runtime writes;
- Rust code changes.

## Gate note

This ADR is proposed, not accepted.

Only future `gate` writes final acceptance. Level 0 `done` records manual closure with evidence.
