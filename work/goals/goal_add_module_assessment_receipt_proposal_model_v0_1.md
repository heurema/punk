---
id: goal_add_module_assessment_receipt_proposal_model_v0_1
title: "Add Module Assessment Receipt Proposal Model v0.1"
status: done
owner: "vitaly"
module: "module-host"
priority: P1
authority: canonical
created_at: 2026-05-16
updated_at: 2026-05-16
selected_at: 2026-05-16
started_at: 2026-05-16
completed_at: 2026-05-16
blocked_by: []
scope:
  include:
    - "crates/punk-module-host/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "README.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_module_assessment_receipt_proposal_model_v0_1.md"
    - "work/reports/2026-05-16-module-assessment-receipt-proposal-model-v0-1.md"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-events/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/publications/**"
    - "plugin runtime"
    - "Wasm or Extism runtime"
    - "dynamic module execution"
    - "module manifests"
    - "receipt writer"
    - "event-log writer"
    - "adapters"
    - "external API behavior"
    - "publishing behavior"
    - "gate decision writer"
    - "proofpack writer"
acceptance:
  - "Adds a pure/no-IO `punk-module-host` model for future module assessment receipt proposals."
  - "The model consumes an invocation envelope plus advisory assessment envelope and does not invoke modules or adapters."
  - "The model parses known expected receipt fields and reports advisory covered fields only when host inputs are ready."
  - "The model blocks blocked assessment envelopes, mismatched envelope refs, unsafe output refs, output side-effect flags, and unknown expected receipt fields."
  - "Smoke coverage proves the receipt proposal model is side-effect-free and not a receipt writer, event writer, CLI surface, adapter invoker, gate writer, proof writer, publisher, or acceptance authority."
  - "Docs/status distinguish the receipt proposal model from parked Module Host runtime, plugin runtime, PubPunk runtime, adapters, and receipt writing."
  - "Adds no external side effects, GitHub/API/browser/credential behavior, bot, adapter, automatic issue/PR creation, publication receipt writer, DAO, token, or funding behavior."
research_gate:
  classification: R2
  required: true
  rationale: "This implementation affects module architecture and future receipt boundaries, but it is a bounded pure local code slice based on existing canonical module-host docs and the completed invocation envelope slice."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_module_host_invocation_envelope_v0_1.md"
    - "work/reports/2026-05-16-module-host-invocation-envelope-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The patch adds an incubating module-host receipt proposal model and smoke coverage, so current module-host status and work-ledger evidence must reflect it."
---

# Add Module Assessment Receipt Proposal Model v0.1

## Context

The previous Module Host slice can preflight a pure invocation envelope and
wrap already-produced advisory module output. The next safest step is not a
receipt writer. It is a pure model that says what a future host-approved module
receipt would need to cover.

## Selected Slice

Add a local-only module assessment receipt proposal model in
`punk-module-host`.

The model accepts:

- a `ModuleInvocationEnvelope`;
- a `ModuleAssessmentEnvelope`;
- declared expected receipt fields from the invocation.

It returns advisory proposal evidence:

- required known receipt fields;
- covered fields when all inputs are ready;
- fail-closed findings for blocked or mismatched host inputs;
- no-side-effect boundary flags.

## Boundary

This slice does not activate Module Host runtime, plugin loading, module
execution, module manifests, dynamic dispatch, Wasm/Extism runtime, public CLI,
filesystem IO, receipt writing, event-log mutation, external APIs, browser
behavior, credential reads, adapter invocation, publishing, gate decisions,
proofpacks, or acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with pure Rust model, unit tests, smoke coverage, docs/status, and
work-report evidence.
