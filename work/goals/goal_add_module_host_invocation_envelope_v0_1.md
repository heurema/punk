---
id: goal_add_module_host_invocation_envelope_v0_1
title: "Add Module Host Invocation Envelope v0.1"
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
    - "Cargo.toml"
    - "Cargo.lock"
    - "crates/punk-module-host/Cargo.toml"
    - "crates/punk-module-host/src/lib.rs"
    - "crates/punk-eval/Cargo.toml"
    - "crates/punk-eval/src/lib.rs"
    - "README.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_module_host_invocation_envelope_v0_1.md"
    - "work/reports/2026-05-16-module-host-invocation-envelope-v0-1.md"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
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
  - "Adds a `punk-module-host` crate for a pure invocation envelope and advisory assessment wrapper."
  - "Preflight requires module, contract, run, project, operation, input refs, and expected receipt fields."
  - "Capabilities stay denied by default except pure assessment of provided input."
  - "Preflight blocks unsupported capabilities, unsafe input refs, and raw/private payload policy."
  - "Assessment wrapping blocks non-advisory module output and output side-effect flags."
  - "Smoke coverage proves the host envelope can wrap side-effect-free PubPunk advisory output without plugin loading, module invocation, CLI behavior, IO, receipts, APIs, credentials, adapters, event-log mutation, gate/proof authority, or acceptance claims."
  - "Docs/status distinguish the incubating envelope from parked Module Host runtime and parked plugin runtime behavior."
research_gate:
  classification: R2
  required: true
  rationale: "This implementation affects module architecture and future runtime/plugin boundaries, but it is a bounded pure local code slice based on existing canonical module-host docs and the PubPunk module boundary."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULES.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/modules/pubpunk.md"
    - "work/STATUS.md"
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
  rationale: "The patch adds an incubating module-host crate and smoke coverage, so current module-host status and work-ledger evidence must reflect it."
---

# Add Module Host Invocation Envelope v0.1

## Context

PubPunk now has a side-effect-free module-owned assessment model, but Punk still
needs a generic boundary for accepting module evidence without routing module
work back into active core or public CLI commands.

The smallest safe next slice is a pure Module Host envelope.

## Selected Slice

Add `punk-module-host` with:

- `ModuleInvocationEnvelope`;
- denied-by-default capability grants;
- metadata-only privacy policy;
- side-effect-free host boundary flags;
- preflight for explicit invocation shape;
- `ModuleOutputSummary`;
- `ModuleAssessmentEnvelope` for advisory output wrapping.

## Boundary

This slice does not activate Module Host runtime, plugin loading, module
execution, module manifests, dynamic dispatch, Wasm/Extism runtime, public CLI,
filesystem IO, receipts, event-log mutation, external APIs, browser behavior,
credential reads, adapter invocation, publishing, gate decisions, proofpacks, or
acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with pure Rust model, unit tests, smoke coverage, docs/status, and
work-report evidence.
