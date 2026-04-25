---
id: goal_add_gate_proof_acceptance_smoke_eval
title: "Add gate/proof acceptance-chain smoke eval coverage"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: 2026-04-25
completed_at: 2026-04-25
blocked_by: []
scope:
  include:
    - "Cargo.lock"
    - "crates/punk-eval/Cargo.toml"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - ".punk/**"
acceptance:
  - "Smoke eval covers the gate/proof acceptance chain using existing side-effect-free gate and proofpack kernels."
  - "Smoke cases verify that only gate decision is final authority, proofpack is provenance only, and positive acceptance needs both an accepting decision and a matching proofpack."
  - "Smoke output remains a local assessment and does not write `.punk/evals`, gate decisions, proofpacks, acceptance claims, runtime storage, or CLI state."
  - "No CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "evals/specs/gate-decision-boundary.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md"
  - "work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md"
  - "work/reports/2026-04-25-twelfth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This will add deterministic smoke eval coverage over already-defined and implemented side-effect-free gate/proof kernels without new architecture decisions or runtime activation."
  research_refs:
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "evals/specs/gate-decision-boundary.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md"
    - "work/reports/2026-04-25-proofpack-kernel-minimal-v0-1.md"
    - "work/reports/2026-04-25-twelfth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md"
  rationale: "Execution will change active-core eval behavior and record a work report; no product docs are expected unless active CLI or runtime semantics change."
---

## Context

Punk now has side-effect-free kernels for:

- run receipt evidence;
- gate decision authority;
- proofpack provenance;
- proof-before-acceptance preconditions.

The current smoke eval still covers contract, flow, event, and receipt boundaries, but it does not yet exercise the gate/proof acceptance chain.

## Notes

Keep this as local deterministic eval coverage only.

Do not write `.punk/evals`.
Do not add CLI behavior.
Do not implement gate runtime.
Do not implement proofpack writer.
Do not claim acceptance.
Do not add adapters, automation, provider/model runners, or schema files.

## Outcome

Completed deterministic gate/proof acceptance-chain smoke eval coverage.

Added local smoke cases for:

- accepting gate authority requiring proof before acceptance;
- proofpack staying post-gate provenance and not decision authority;
- positive acceptance requiring both accepting authority and a matching proofpack.

The smoke eval remains a local assessment only.

It still does not:

- write `.punk/evals`;
- write gate decisions;
- write proofpacks;
- claim acceptance;
- require runtime storage;
- expose new CLI behavior;
- add schema files, adapters, automation, or provider/model runners.
