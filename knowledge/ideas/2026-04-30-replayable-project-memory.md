---
id: idea_2026_04_30_replayable_project_memory
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-04-30
updated_at: 2026-04-30
review_after: 2026-07-30
components:
  - project-memory
  - replayability
  - contracts
  - migration
  - conformance
  - regenerative-specs
related_docs:
  - docs/product/NORTH-STAR.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-GATE.md
related_goals:
  - goal_capture_replayable_project_memory_direction
related_research:
  - knowledge/research/2026-04-30-replayable-project-memory.md
supersedes: []
superseded_by: null
---

# Replayable Project Memory

## Status

This is an advisory idea artifact and strategic direction.

It is not product truth by itself. It does not activate runtime behavior, CLI
behavior, Writer, gate writers, proof writers, agents, adapters, migration
automation, or spec-as-source generation.

## Summary

Replayable Project Memory is the idea that Punk artifacts should preserve enough
intent, contracts, evidence, decisions, examples, invariants, and proof links to
support future migration/reimplementation contracts.

## Research refinement

Related research:

- `knowledge/research/2026-04-30-replayable-project-memory.md`

Refined thesis:

```text
Punk should preserve replayable obligations, not replayable code.
```

Adoption map:

- adopt now: strategic concept, vocabulary, and idea artifact;
- adopt now as discipline: preserve replayability-relevant refs where current workflow already supports it;
- defer: Conformance Pack boundary, Migration Contract Pack, accepted-difference model, conformance/equivalence evals;
- park: Regenerative Spec Pack, full-project replay, spec-as-source authority, automatic cross-stack regeneration;
- avoid now: deterministic replay promises, generated specs as canonical truth, Conformance Pack as gate decision, preserving bugs as requirements.

## Problem

Project memory can become passive history unless it is designed for
replayability.

Future users may need to:

- rewrite a subsystem on another stack;
- replace a framework;
- split a monolith;
- reimplement a service;
- generate a new implementation from reviewed artifacts.

If Punk only records final outcomes, future humans and agents may be unable to
recover why work happened, which behavior mattered, which differences were
accepted, or how to verify a migration.

## Non-goal

This is not a promise of deterministic code regeneration.

This is not active spec-as-source.

This is not automatic migration.

This is not current CLI/runtime behavior.

Punk cannot currently regenerate projects, replay a project into another
language, or guarantee that existing project memory is complete enough for a
migration.

## Strategic thesis

Punk should make software work replayable enough that future
migration/reimplementation can be expressed as contracts and verified through
conformance/equivalence gates.

The goal is not:

```text
old history -> auto-generate new project
```

The goal is:

```text
old implementation + Punk project memory
-> portable intent and behavior baseline
-> migration/reimplementation contract
-> new implementation
-> conformance/equivalence gate
-> proof
```

## Three dimensions

### Project Entry Mode

- greenfield;
- brownfield;
- grayfield.

### Artifact Authority Mode

- code-first discipline;
- spec-anchored work;
- spec-and-intent-native workflow;
- future spec-as-source experiment.

### Replayability Mode

- audit replay;
- intent replay;
- contract replay/transposition;
- behavior replay;
- migration replay;
- regenerative replay.

## Replayability ladder

### RL-0 Audit replay

Preserves:

- goals;
- reports;
- decisions;
- evidence refs;
- proof refs when available.

Enables:

- reconstructing what happened;
- reviewing evidence trails;
- finding missing or stale links.

Status: adopt-now discipline through repo-tracked work ledger, reports, docs, and
proof direction. Runtime storage remains deferred.

### RL-1 Intent replay

Preserves:

- portable intent;
- rationale;
- assumptions;
- unknowns;
- non-goals;
- research refs.

Enables:

- reconstructing why work happened;
- drafting follow-up work with less hidden context;
- separating user intent from executor claims.

Status: adopt-now discipline for side-effect-free intent/contract modeling;
deferred for richer runtime capture.

### RL-2 Contract replay / transposition

Preserves:

- contracts;
- scope include/exclude;
- hard/soft/advisory clauses;
- validator refs;
- required receipt fields;
- proof requirement refs;
- human review markers.

Enables:

- using previous contracts as source-linked input to a future migration or
  reimplementation contract;
- preserving authority boundaries across rewrites.

Status: partially adopt-now discipline as side-effect-free model direction;
deferred for runtime contract storage and migration-contract workflows.

### RL-3 Behavior replay

Preserves:

- behavioral examples;
- tests and fixtures;
- public API contracts;
- data contracts;
- domain invariants;
- state transitions;
- error behavior;
- side-effect model.

Enables:

- checking a new implementation against preserved behavior;
- building future conformance/equivalence gates.

Status: deferred. Existing tests/evals can be linked as evidence, but Punk does
not yet package behavior replay artifacts.

### RL-4 Migration replay

Preserves:

- migration notes;
- stack-specific assumptions;
- accepted differences;
- equivalence hints;
- security/privacy constraints;
- performance expectations;
- known non-goals.

Enables:

- bounded migration/reimplementation contracts;
- explicit compatibility and accepted-difference review;
- conformance/equivalence gate planning.

Status: parked/future. Needs research and stronger contract/receipt/gate/proof
surfaces.

### RL-5 Regenerative replay

Preserves:

- reviewed spec packs;
- executable specifications;
- conformance packs;
- proof-linked behavior baselines;
- generation constraints and accepted-difference policies.

Enables:

- future bounded regeneration experiments;
- spec-as-source research;
- round-trip reconstruction experiments.

Status: future-only. This is not current Punk behavior.

## Greenfield vs Brownfield vs Grayfield

Greenfield with Punk records replay memory from day zero.

Brownfield without Punk reconstructs replay candidates from existing artifacts.

Grayfield reconciles partial recorded memory with partial reconstruction.

This means:

```text
Greenfield with Punk = recorded replay memory
Brownfield without Punk = reconstructed replay candidates
Grayfield = partial recorded memory + partial reconstruction
```

## Replay vs transposition vs regeneration

Replay means reconstructing what happened from project artifacts.

Intent replay means reconstructing why work happened.

Contract transposition means using previous contracts as source-linked input to
future migration/reimplementation contracts.

Behavior replay means preserving examples, tests, fixtures, invariants, and
schemas to verify future implementations.

Regeneration means generating implementation from a higher-level artifact. This
is future/spec-as-source territory and remains parked.

## Candidate future artifacts

### Conformance Pack

Future/parked artifact idea: source-linked examples, tests, invariants, schemas,
fixtures, accepted-difference notes, and evidence refs used to check
compatibility of a new implementation.

It does not write gate decisions.

### Migration Contract Pack

Future/parked artifact idea: source-linked bundle that helps draft a bounded
migration or reimplementation contract from prior project memory, existing code,
decisions, non-goals, and behavior baselines.

It does not approve migration work.

### Regenerative Spec Pack

Future/parked artifact idea: reviewed artifact set strong enough to help
generate or regenerate implementation for a bounded subsystem.

It is not active behavior and must not become authority without review, gate,
and proof.

## What Punk may need to preserve from day zero

| Item | Conservative status | Note |
|---|---|---|
| `portable_intent` | adopt-now discipline | Present in intent/contract direction, still side-effect-free. |
| `stack_specific_assumptions` | defer | Needed for migration but not fully modeled. |
| `behavioral_examples` | defer | Can be linked today; no conformance pack yet. |
| `public_api_contracts` | defer | Needs product/schema boundaries. |
| `data_contracts` | defer | Needs schema/data-contract work. |
| `domain_invariants` | defer | Can appear as clauses; richer invariant capture deferred. |
| `state_transitions` | adopt-now discipline | Flow/eval kernels exist; broader behavior packaging deferred. |
| `error_behavior` | defer | Needs behavior replay modeling. |
| `side_effect_model` | adopt-now discipline | Present in contract/proof writer boundaries, still partial. |
| `security_privacy_constraints` | defer | Needs explicit policy/security boundary work. |
| `performance_expectations` | defer | Not active-core today. |
| `equivalence_hints` | future | Needed for migration replay. |
| `migration_notes` | future | Needed for migration replay. |
| `known_non_goals` | adopt-now discipline | Present in contract/intent direction. |
| `accepted_differences` | future | Needed for migration/equivalence workflows. |
| `unknowns` | adopt-now discipline | Present in intent/contract direction. |
| `research_refs` | adopt-now discipline | Research Gate and work reports already use refs. |
| `decision_refs` | adopt-now discipline | ADR/report/gate direction already uses refs. |
| `evidence_refs` | adopt-now discipline | Reports/evals/receipts direction use evidence refs. |
| `validator_refs` | adopt-now discipline | Present as declarative refs, not runtime execution. |
| `proof_refs` | adopt-now discipline | Proof direction exists; runtime proof storage remains deferred. |

## Risks

- overclaiming spec-as-source;
- preserving bugs as requirements;
- treating tests as complete behavior;
- treating old implementation details as portable intent;
- turning generated specs into authority;
- flattening history into a giant prompt;
- letting conformance packs act as gate decisions;
- premature runtime/CLI expansion.

## Research needed

Future deep research topic:

```text
Replayable Project Memory / Regenerative Specs / Round-trip Project Reconstruction
```

Research directions:

- characterization testing;
- golden master testing;
- approval testing;
- executable specs;
- property-based testing;
- specification mining;
- invariant inference;
- traceability recovery;
- architecture reconstruction;
- semantic equivalence testing;
- CodeSpeak takeover/mixed mode/intent recovery;
- GitHub Extract/Edit/Apply;
- SpecLang.

## Adoption recommendation

Adopt now:

- strategic concept;
- vocabulary;
- idea artifact;
- advisory research note.

Adopt now as discipline:

- preserve replayability-relevant refs where current contracts, reports, docs, and evals already support them;
- keep portable intent, assumptions, examples, evidence refs, decision refs, validator refs, proof refs, unknowns, and non-goals source-linked when they are already part of bounded work.

Defer:

- Conformance Pack boundary;
- Migration Contract Pack;
- accepted-difference model;
- bug-vs-requirement classification;
- conformance/equivalence evals.

Park:

- Regenerative Spec Pack;
- full-project replay;
- spec-as-source authority;
- automatic cross-stack regeneration.

Avoid now:

- deterministic replay promises;
- generated specs as canonical truth;
- Conformance Pack as gate decision;
- preserving bugs as requirements.
