# Contract Tracker

## Status

Proposed.

This document defines a phase-gated product and architecture direction.

It does not promote a new active CLI surface by itself.

## Purpose

`punk` needs an inspectable work ledger for bounded execution.

The Contract Tracker is that ledger.

It is not a separate task tracker, not a Jira clone, and not a replacement lifecycle.

## Core definition

Treat the Contract Tracker as Punk's Work Ledger view over the existing lifecycle:

```text
plot -> cut -> gate
```

Work is not complete because a status says `done`.

Work is complete only when the ledger contains the artifacts required by Punk Laws:

- contract
- run receipt
- proofpack
- gate decision
- append-only events

## Non-goals

Do not treat the Contract Tracker as:

- a new core module;
- a configurable workflow engine;
- a second source of truth above flow state;
- an agent-owned work queue;
- a general project-management product.

## Core primitives

The minimal object model is:

```text
Plot -> Contract -> Cut -> Run -> Receipt -> Proofpack -> GateDecision -> EventLog
```

Definitions:

- `Plot` shapes work and leads to a contract.
- `Contract` defines bounded scope and approval state.
- `Cut` is the bounded executable slice inside approved scope.
- `Run` is one execution attempt.
- `Receipt` records what the run actually did.
- `Proofpack` bundles verifiable evidence.
- `GateDecision` is the only final closure artifact.
- `EventLog` preserves the lifecycle as append-only project memory.

## Truth model

The Contract Tracker must preserve current Punk invariants:

- every executable goal resolves to a contract;
- every transition writes an event;
- `cut` cannot run unapproved work;
- every run writes a receipt;
- only `gate` writes the final decision;
- proof is required before acceptance;
- modules may assess but may not decide;
- adapters may invoke but may not own truth.

This means the tracker is a ledger view over core artifacts, not an independent workflow system.

## Relationship to project memory

The Contract Tracker belongs inside the Project Memory Plane as the Work Ledger surface.

Repo-tracked and runtime data stay split:

- repo-tracked goals, reports, research, ADRs, and related knowledge under `work/` and `knowledge/`;
- runtime and derived contract/run/decision/proof artifacts under `.punk/`.

Accepted summaries may later expose stable inspect views, but those views must not become a second source of truth.

## Phase mapping

### Phase 3 — Contract loop without agents

Prove the minimal lifecycle:

```text
goal -> contract -> approve -> cut -> run -> gate -> proof
```

Required ledger facts:

- accepted contract has scope;
- run writes receipt;
- gate writes decision;
- proofpack links or hashes the verified artifacts.

### Phase 4 — Project Memory minimal

Expose the Contract Tracker as an inspectable project-memory surface over:

- accepted contracts;
- receipts;
- gate decisions;
- proofpacks;
- goal/report links;
- follow-up obligations when they exist.

### Phase 5 — Project coherence gate

Use accepted ledger artifacts to detect drift across many individually valid contracts.

## Evidence model

A proofpack should stay minimal in early phases.

Adopt the principle, not a heavy supply-chain system:

- evidence refs;
- stable hashes where available;
- timestamps;
- actor/process provenance;
- verification outcome.

Do not treat green checks alone as truth.

The proofpack exists to make the gate decision inspectable, not theatrical.

## Assessments vs decisions

Rules, evals, and modules can produce assessments.

Assessments are advisory evidence.

They must not be named or stored as final decisions.

Only `gate` may close the loop.

## CLI design target

The following is a design target only.

It is not an accepted public CLI contract yet.

```text
punk plot new --from work/goals/...
punk plot approve <plot>
punk cut new <contract> --title "..."
punk cut receipt <cut> --from <file-or-command-output>
punk gate decide <contract> --proof <proofpack> --accept|--reject
punk inspect ledger --contract <id>
```

Any future CLI promotion must satisfy the core-first rule from `docs/product/START-HERE.md`.

## Out of scope for active-core now

Park these ideas until later phases or dedicated ADRs:

- AI triage or inbox ranking;
- agent delegation and agent-owned execution surfaces;
- skills as active-core workflow primitives;
- customer or support integrations;
- external adapters as tracker truth;
- advanced dependency graphs;
- automated public narrative extraction.

## Open questions

1. Should `Cut` be a first-class persisted entity or a transition-derived slice?
2. Should accepted proof summaries live only under `.punk/`, or also surface in repo-tracked reports?
3. What is the smallest useful outcome vocabulary for `GateDecision` beyond simple accept/reject?
4. When should dependency tracking enter scope: Phase 3, or only after Project Memory is stable?
5. How should Punk detect stale evidence without turning proofpacks into maintenance theater?
