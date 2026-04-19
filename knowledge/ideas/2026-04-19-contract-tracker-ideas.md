---
kind: idea-bank
status: active
authority: speculative
created: 2026-04-19
related_research: knowledge/research/2026-04-19-contract-tracker-prior-art.md
related_decision: docs/adr/ADR-0009-contract-tracker-core-primitives.md
---

# Contract Tracker Ideas to Preserve

These ideas are useful but should not enter active-core yet.

They remain speculative until promoted through ADR, roadmap, goal/contract, implementation, and eval results.

## Defer / preserve

### Plot inbox / triage queue

Source inspiration: Linear triage and context-routing patterns.

Idea: create an advisory intake layer before contract approval.

Why parked now:

- easy to turn into a second source of truth;
- depends on stable contract semantics first;
- likely needs agent/module boundaries that do not exist yet.

### Advisory AI dedupe and relation suggestions

Idea: suggest `duplicate-of`, `related-to`, `supersedes`, or similar links between plots/contracts.

Why parked now:

- useful only as advisory signal;
- too easy to overclaim as truth before core ledger semantics stabilize.

### Evidence freshness

Idea: detect whether proof evidence is stale relative to the latest contract, code, or knowledge change.

Why deferred:

- valuable for later coherence checks;
- needs stable artifact identity and hashing rules first.

### Agent session transparency

Source inspiration: visible agent-state guidance from Linear.

Idea: keep inspectable agent receipts, tool receipts, and bounded session state without storing private chain-of-thought.

Why parked now:

- agent execution is not active-core;
- needs side-effect and privacy rules.

### Human-owned agent delegation

Idea: allow delegation while keeping a human accountable owner on the contract.

Why parked now:

- ownership and actor model are still immature;
- must be enforced as an invariant, not implied socially.

### Skills as reusable workflow recipes

Idea: a skill can draft, assess, or produce receipts for repeated work patterns.

Why parked now:

- skills would be module/agent-level helpers, not truth owners;
- introducing them too early blurs the core/runtime boundary.

### Customer or external signal refs

Idea: link external requests, tickets, or feedback to plots/contracts as non-canonical evidence.

Why parked now:

- external adapters are explicitly later-scope;
- source-of-truth boundaries are not ready.

### Gate follow-up obligations

Idea: a gate decision could emit explicit follow-up obligations such as docs update, eval addition, or notification.

Why deferred:

- potentially useful for Phase 4-5 coherence;
- should not expand the contract loop before the minimal ledger is proven.

### Public narrative extraction

Idea: derive public-facing stories or artifacts from accepted contracts and proofs.

Why parked now:

- public narrative automation is later-scope;
- manual public artifacts already have a dedicated repo structure.

### Local-first conflict ledger

Idea: preserve conflict or divergence records when local-first state collides across views or runtimes.

Why parked now:

- interesting only once runtime/project-memory sync rules are more concrete.

## Do not promote without a new ADR

- AI triage as active-core behavior.
- Agent-owned work or accountability.
- Skills as decision writers.
- External integrations as tracker truth.
- Public narrative automation from tracker state.
- Configurable workflow-builder features.
