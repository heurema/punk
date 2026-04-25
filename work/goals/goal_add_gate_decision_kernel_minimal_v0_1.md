---
id: goal_add_gate_decision_kernel_minimal_v0_1
title: "Add minimal gate decision kernel v0.1"
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
    - "crates/punk-gate/Cargo.toml"
    - "crates/punk-gate/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-proof/**"
    - ".punk/**"
acceptance:
  - "Gate decision kernel models a side-effect-free decision object with explicit outcome, contract refs, receipt refs, optional eval/event refs, created-at, and boundary notes."
  - "Gate decision validates required refs without writing `.punk/decisions`, CLI output, proofpacks, acceptance claims, or runtime storage."
  - "Accepting decision remains final decision authority but is not sufficient for acceptance without matching proof."
  - "Receipt/eval/event evidence remains evidence-only and is referenced rather than absorbed."
  - "No CLI command, `.punk/` state, proofpack writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "evals/specs/gate-decision-boundary.v0.1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "evals/specs/minimal-receipt-fields.v0.1.md"
  - "work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md"
  - "work/reports/2026-04-25-tenth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This implements already-defined gate decision and proof-before-acceptance boundaries as a side-effect-free Rust kernel without new architecture decisions."
  research_refs:
    - "evals/specs/gate-decision-boundary.v0.1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "evals/specs/minimal-receipt-fields.v0.1.md"
    - "work/reports/2026-04-25-tenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-gate-decision-kernel-minimal-v0-1.md"
  rationale: "Execution will change active-core Rust gate behavior and record a work report; no product docs are expected unless behavior boundaries change."
---

## Context

Punk now has:

- minimal contract lifecycle kernel;
- flow guard and event evidence;
- run receipt kernel with minimal fields and missing-validator evidence;
- gate decision boundary v0.1;
- proof-before-acceptance semantics v0.1.

The next smallest active-core implementation step is a side-effect-free gate decision kernel that can model final decision authority without writing runtime storage or proofpacks.

## Notes

Keep this as a pure kernel change.

Do not write `.punk/decisions`.
Do not add CLI behavior.
Do not implement proofpack writer.
Do not claim acceptance.
Do not add adapters, automation, provider/model runners, or schema files.

## Outcome

Completed the minimal side-effect-free gate decision kernel v0.1.

Implemented gate decision fields:

- schema version;
- decision id;
- outcome;
- contract refs;
- run receipt refs;
- optional eval refs;
- optional event refs;
- created-at;
- boundary notes.

The gate decision kernel models final decision authority as data, but still does not:

- write `.punk/decisions`;
- expose CLI behavior;
- create proofpacks;
- claim acceptance;
- require runtime storage;
- absorb evidence bodies.

An accepting decision still requires matching proof before acceptance can be claimed.
