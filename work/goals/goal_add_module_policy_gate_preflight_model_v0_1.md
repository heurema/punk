---
id: goal_add_module_policy_gate_preflight_model_v0_1
title: "Add Module Policy Gate Preflight Model v0.1"
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
    - "work/goals/goal_add_module_policy_gate_preflight_model_v0_1.md"
    - "work/reports/2026-05-16-module-policy-gate-preflight-model-v0-1.md"
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
    - "policy engine invocation"
    - "gate invocation"
    - "adapters"
    - "external API behavior"
    - "publishing behavior"
    - "comment behavior"
    - "pull request automation"
    - "gate decision writer"
    - "proofpack writer"
acceptance:
  - "Adds a pure/no-IO `punk-module-host` model for future module policy gate preflight evidence."
  - "The model consumes a ready side-effect request proposal and explicit policy, gate-input, side-effect receipt proposal, adapter invocation receipt, payload, and proof requirement refs."
  - "The model reports required preflight evidence only when inputs are ready and refs are safe."
  - "The model blocks blocked side-effect request proposals, missing or unsafe refs, mismatched policy/receipt/payload refs, missing side-effect request precondition coverage, and preflight drafts that report performed side effects."
  - "Smoke coverage proves the policy gate preflight model is side-effect-free and not a policy engine invoker, gate invoker, adapter invoker, publisher, commenter, PR creator, receipt writer, event writer, gate writer, proof writer, or acceptance authority."
  - "Docs/status distinguish the policy gate preflight model from parked Module Host runtime, plugin runtime, policy runtime, gate decision writing, PubPunk runtime, adapters, and external publishing/comment/PR behavior."
  - "Adds no external side effects, GitHub/API/browser/credential behavior, bot, adapter, automatic issue/PR creation, publication receipt writer, DAO, token, or funding behavior."
research_gate:
  classification: R2
  required: true
  rationale: "This implementation affects module architecture and future external side-effect policy boundaries, but it is a bounded pure local code slice based on existing canonical module-host docs and the completed side-effect request proposal model."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_module_side_effect_request_proposal_model_v0_1.md"
    - "work/reports/2026-05-16-module-side-effect-request-proposal-model-v0-1.md"
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
  rationale: "The patch adds an incubating module-host policy gate preflight model and smoke coverage, so current module-host status and work-ledger evidence must reflect it."
---

# Add Module Policy Gate Preflight Model v0.1

## Context

The previous Module Host slice can model future side-effect request
preconditions. The next safest step is not adapter invocation, policy engine
execution, gate invocation, or publishing automation. It is a pure preflight
model that checks whether explicit policy evidence refs exist before any future
side effect can move toward execution.

## Selected Slice

Add a local-only module policy gate preflight model in `punk-module-host`.

The model accepts:

- a ready `ModuleSideEffectRequestProposal`;
- a `ModulePolicyGatePreflightDraft` with explicit policy, gate-input,
  side-effect receipt proposal, adapter invocation receipt, payload, and proof
  requirement refs.

It returns advisory preflight evidence:

- required evidence refs;
- covered evidence refs when all inputs are ready;
- fail-closed findings for blocked or unsafe inputs;
- no-side-effect boundary flags.

## Boundary

This slice does not activate Module Host runtime, plugin loading, module
execution, module manifests, dynamic dispatch, Wasm/Extism runtime, public CLI,
filesystem IO, receipt writing, event-log mutation, policy engine invocation,
gate invocation, external APIs, browser behavior, credential reads, adapter
invocation, publishing, commenting, pull request creation, gate decisions,
proofpacks, or acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with pure Rust model, unit tests, smoke coverage, docs/status, and
work-report evidence.
