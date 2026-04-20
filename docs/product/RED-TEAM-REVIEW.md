---
id: docs_product_red_team_review
kind: product-doc
status: active
authority: advisory
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - architectural-risk-review
  - failure-mode-review
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/MODULES.md
related_adrs: []
supersedes: []
superseded_by: null
---

# Red-Team Review

## Purpose

This document captures architectural risks in the `punk` starter architecture.

It should be updated when a new risk changes the roadmap, phase gate, or core boundary.

## Review stance

Assume the product will fail by becoming too broad before the core is stable.

Therefore, every proposal must answer:

- does it strengthen the core?
- does it require a new flow transition?
- does it require an eval case?
- does it create a second source of truth?
- does it make dogfooding safer or noisier?

## Top risks

### R1 — Workspace sprawl

Risk: creating many crates early makes the project feel more complete than it is.

Mitigation:

- maintain `active-core / incubating / parked / retired`
- parked crates are not workspace members unless needed
- public CLI only exposes active capabilities

### R2 — Eval theatre

Risk: evals exist, but do not catch the regressions that matter.

Mitigation:

- smoke evals first
- hard gates before scores
- add cases from real failures
- keep a shadow pool of candidate evals
- require baseline comparison for phase promotion

### R3 — Flow bypass

Risk: users or agents call mid-pipeline commands and accidentally skip required state.

Mitigation:

- commands are transitions over persisted state
- illegal transitions are denied by runtime
- expert override requires reason and ledger event
- override appears in inspect/proof context

### R4 — Dogfooding recursion

Risk: `punk` modifies itself and falsely certifies its own trust surfaces.

Mitigation:

- dogfooding levels
- stronger review for core/gate/proof/eval/flow changes
- self-hosted work can propose and execute, but core trust changes require extra evidence

### R5 — Knowledge swamp

Risk: memory grows but retrieval gets worse.

Mitigation:

- status + authority required
- raw ideas excluded from implementation truth
- reports are not automatically promoted to canonical knowledge
- project coherence gate checks missing/stale knowledge

### R6 — Module drift

Risk: modules become separate products with separate vocabularies and truth models.

Mitigation:

- modules inherit Punk Laws
- modules cannot write final decisions
- modules must pass conformance evals
- module-specific payloads remain inside universal contract/run/proof lifecycle

### R7 — Adapter capture

Risk: provider-specific adapters start shaping the kernel.

Mitigation:

- wrap provider capabilities
- adapters never own truth
- adapters freeze outputs into run context
- no provider-zoo UX in core

### R8 — Roadmap bypass by good ideas

Risk: attractive ideas enter active work before their phase.

Mitigation:

- classify every idea: core-strengthening, future capability, or destabilizing
- core-strengthening may enter current phase only with eval/flow impact
- future capability goes to roadmap/backlog
- destabilizing idea becomes research note only

## Required review before phase promotion

Before any phase promotion, answer:

1. What became active?
2. What stayed parked?
3. Which evals were added or updated?
4. Which flow transitions changed?
5. Which docs changed?
6. Which risks changed?
7. Can `punk` dogfood this phase safely?


### R9 — Research sprawl

Risk: research becomes a new form of procrastination or a dumping ground.

Mitigation:

- use R0/R1/R2/R3 depth levels
- require recommendation and out-of-scope section
- require eval implications
- keep research notes concise and curated
- do not require research for small implementation steps

### R10 — Weak-source contamination

Risk: low-quality commentary shapes core decisions.

Mitigation:

- source quality tiers
- Tier C cannot justify core decisions alone
- prefer official docs, mature project docs, standards, papers, issue/PR evidence, and postmortems


### R11 — Public narrative drift

Risk: public story diverges from actual product state.

Mitigation:

- first posts must not overclaim implementation readiness
- public claims should link to canonical docs or be framed as opinion
- publication receipts distinguish draft from published
- project coherence gate may later check stale/incorrect public claims

### R12 — Hidden PubPunk store

Risk: future PubPunk creates a separate content database and ignores early public artifacts.

Mitigation:

- `publishing/` is the repo-tracked source for public narrative
- PubPunk must adopt `publishing/`
- metrics indexes may be derived, not canonical

### R13 - Plugin sandbox theatre

Risk: a future Wasm/plugin host looks safe because it is sandboxed, but still becomes a second kernel through hidden state, ambient capabilities, direct event writes, or unbounded host calls.

Mitigation:

- keep plugin runtime work parked until Module Host phase
- require R2 before choosing Wasm, Extism, native modules, or another runtime
- keep capabilities deny-by-default
- host validates plugin-produced receipts and assessments
- only `gate` writes final decisions
- conformance evals must prove scope, capability, resource, receipt, and decision boundaries
