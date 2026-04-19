# Roadmap

This is a phase-gated roadmap, not a feature wishlist.

A phase can be promoted only when its exit criteria are green.

## Phase -1 — Public narrative seed

Goal: preserve the first public-build story before implementation.

Deliverables:

- `public/` structure
- first story artifact
- first post draft
- manual channel descriptor
- publication receipt template

Exit criteria:

- first post can be manually published
- receipt location is defined
- no automation is implied

This phase can happen before Phase 0.

## Phase 0 — Architecture skeleton

Goal: create the workspace and active/incubating/parked boundaries.

Active target:

- workspace compiles
- docs define core boundaries
- parked crates/folders do not expose public behavior

Exit criteria:

- `cargo check --workspace` passes
- status vocabulary is documented
- no parked capability appears in the public CLI

Out of scope:

- LLM execution
- modules
- adapters
- publishing
- semantic memory

## Phase 1 — Flow + event kernel

Goal: commands become validated transitions over persisted state.

Deliverables:

- `FlowInstance`
- `flows/core-workflow.v1.yaml`
- transition guard
- append-only event log
- `punk flow inspect`

Exit criteria:

- cannot `cut` before approval
- cannot write proof before decision
- every transition writes an event
- flow can be resumed from persisted state

Required eval direction:

- create/keep flow cases under `evals/cases/flow/`
- ensure each allowed transition has event expectations
- ensure illegal transitions leave no forbidden artifacts

## Phase 2 — Eval smoke harness

Goal: create regression signal before adding features.

Deliverables:

- `punk eval list`
- `punk eval run smoke`
- deterministic-first local runner
- machine JSON report
- human Markdown report
- repo-stored eval case schema
- baseline comparison
- explicit waiver ledger

Exit criteria:

- smoke suite is green
- failing hard-gate smoke eval blocks phase promotion
- baseline regression blocks phase promotion unless an accepted waiver exists
- every future phase includes eval additions
- eval reports are inspectable and do not write final decisions

Initial smoke coverage:

- illegal transition denied
- allowed transition writes event
- cut requires approval and scope
- run writes receipt before gate
- only gate writes final decision
- proof requires decision
- proof hashes contract, receipt, and decision
- report emits machine JSON and human Markdown
- parked capability not exposed through public CLI
- waiver requires explicit reason and ledger entry

See `docs/product/EVAL-PLANE.md`.

## Phase 3 — Contract loop without agents

Goal: prove the core loop without LLM-driven execution.

Flow:

```text
goal -> contract -> approve -> run -> gate -> proof
```

Exit criteria:

- accepted contract has scope
- run writes receipt
- gate writes decision
- proof hashes contract, receipt, and decision

Required eval additions:

- contract schema conformance
- receipt schema conformance
- decision schema conformance
- proofpack schema conformance
- proof link/hash integrity

## Phase 4 — Project Memory minimal

Goal: add long-term project continuity without building a Jira clone.

Deliverables:

- `work/goals/`
- `work/reports/`
- `knowledge/`
- goal-to-contract/proof links
- knowledge status/authority model

Exit criteria:

- accepted contract links to goal or maintenance reason
- speculative knowledge is excluded from implementation truth
- reports can be inspected as handoff artifacts

Required eval additions:

- knowledge authority tags validated
- speculative knowledge excluded from implementation truth
- reports link goal/contract/eval/proof refs when relevant

## Phase 5 — Project coherence gate

Goal: detect drift across many individually valid contracts.

Deliverables:

- project scope envelope
- accepted-contract summary
- touched-component summary
- missing-knowledge-update detection

Exit criteria:

- system can report project drift
- system can identify missing docs/decision updates
- system can recommend next corrective contract

Required eval additions:

- missing knowledge update detection
- stale docs detection
- drift report shape

## Phase 6 — Module host

Goal: prepare installable modules without enabling product modules yet.

Deliverables:

- module manifest
- hook interface types
- module conformance suite

Exit criteria:

- module cannot write final decision
- module cannot bypass scope
- module must emit receipt/assessment

Required eval additions:

- module can assess but cannot decide
- module hooks cannot bypass scope
- module assessment is advisory evidence only

## Phase 7 — Dev module

Goal: first active domain module for software work.

Exit criteria:

- uses shared flow/rules/contracts
- emits module receipt/assessment
- gate remains final decision writer

Required research before implementation:

- coding-agent benchmark scan, including SWE-bench family and contamination risks
- sandbox/reproducibility design

## Phase 8 — Pub module

Goal: prove the architecture is not code-only.

Initial scope:

- draft-only
- no external publishing side effects
- style/fact-check receipts

Exit criteria:

- PubPunk follows the same lifecycle
- no hidden external side effects

Required eval additions:

- manual publication requires receipt
- public claims cite canonical knowledge or are marked as opinion
- no automation is implied before side-effect policy exists

## Phase 9 — Adapters

Goal: wrap provider/tool capabilities only after core stability.

Rules:

- wrap, not rebuild
- no provider zoo
- adapters never own truth

Required research before implementation:

- adapter boundary research
- MCP/tool security benchmark review
- side-effect receipt policy

## Cross-phase gates

Every phase promotion requires:

1. active status table updated
2. smoke evals green
3. phase-specific evals green
4. flow transitions documented
5. docs updated
6. dogfooding level updated honestly
7. no parked capability exposed as active
8. no new source of truth outside event/artifact model
9. waivers, if any, have explicit reason and ledger entry

## Dogfooding roadmap

- Phase 0: Dogfooding Level 0
- Phase 1: Dogfooding Level 1
- Phase 2: Dogfooding Level 2
- Phase 3: Dogfooding Level 3
- Phase 7+: Dogfooding Level 4 for non-trust-critical surfaces only
- Later: Level 5 after module/eval/gate hardening

## Research gates by phase

Before implementation, run focused research for these phase decisions:

| Phase | Research requirement |
|---|---|
| Phase 0 | no additional research; use starter docs |
| Phase 1 | R1/R2 if flow persistence design changes |
| Phase 2 | use eval research; R1 before choosing runner integration details |
| Phase 3 | R1 before final contract schema |
| Phase 4 | R2 before Project Memory storage model |
| Phase 5 | R1/R2 before project drift/coherence scoring |
| Phase 6 | R2 before module host API |
| Phase 7 | R1 before dev module scope/check heuristics |
| Phase 8 | R1/R2 before publishing side-effect model |
| Phase 9 | R1/R2 per provider/adapter boundary |

Research output must link to required eval cases.
