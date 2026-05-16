---
id: goal_add_module_side_effect_request_proposal_model_v0_1
title: "Add Module Side-Effect Request Proposal Model v0.1"
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
    - "work/goals/goal_add_module_side_effect_request_proposal_model_v0_1.md"
    - "work/reports/2026-05-16-module-side-effect-request-proposal-model-v0-1.md"
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
    - "comment behavior"
    - "pull request automation"
    - "gate decision writer"
    - "proofpack writer"
acceptance:
  - "Adds a pure/no-IO `punk-module-host` model for future module side-effect request proposals."
  - "The model consumes an invocation envelope, ready receipt proposal, and explicit request draft without invoking modules or adapters."
  - "The model reports required preconditions for future external actions, including policy refs, receipt proposal readiness, side-effect receipt coverage, adapter invocation receipt, safe payload refs, and gate-or-policy approval."
  - "The model blocks blocked or mismatched receipt proposals, missing side-effect receipt coverage, missing request refs, unsafe request refs, and request drafts that report performed side effects."
  - "Smoke coverage proves the side-effect request proposal model is side-effect-free and not an adapter invoker, publisher, commenter, PR creator, receipt writer, event writer, gate writer, proof writer, or acceptance authority."
  - "Docs/status distinguish the side-effect request proposal model from parked Module Host runtime, plugin runtime, PubPunk runtime, adapters, and external publishing/comment/PR behavior."
  - "Adds no external side effects, GitHub/API/browser/credential behavior, bot, adapter, automatic issue/PR creation, publication receipt writer, DAO, token, or funding behavior."
research_gate:
  classification: R2
  required: true
  rationale: "This implementation affects module architecture and future external side-effect boundaries, but it is a bounded pure local code slice based on existing canonical module-host docs and the completed receipt proposal model."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_module_assessment_receipt_proposal_model_v0_1.md"
    - "work/reports/2026-05-16-module-assessment-receipt-proposal-model-v0-1.md"
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
  rationale: "The patch adds an incubating module-host side-effect request proposal model and smoke coverage, so current module-host status and work-ledger evidence must reflect it."
---

# Add Module Side-Effect Request Proposal Model v0.1

## Context

The previous Module Host slice can model future module receipt field coverage.
The next safest step is not adapter invocation or publishing automation. It is
a pure proposal model that states what a future external action would need
before any side effect can be considered.

## Selected Slice

Add a local-only module side-effect request proposal model in
`punk-module-host`.

The model accepts:

- a `ModuleInvocationEnvelope`;
- a ready `ModuleAssessmentReceiptProposal`;
- a `ModuleSideEffectRequestDraft` with explicit request, target, intent,
  policy, receipt-proposal, adapter, and payload refs.

It returns advisory proposal evidence:

- required preconditions;
- covered preconditions when all inputs are ready;
- fail-closed findings for blocked or unsafe inputs;
- no-side-effect boundary flags.

## Boundary

This slice does not activate Module Host runtime, plugin loading, module
execution, module manifests, dynamic dispatch, Wasm/Extism runtime, public CLI,
filesystem IO, receipt writing, event-log mutation, external APIs, browser
behavior, credential reads, adapter invocation, publishing, commenting, pull
request creation, gate decisions, proofpacks, or acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with pure Rust model, unit tests, smoke coverage, docs/status, and
work-report evidence.
