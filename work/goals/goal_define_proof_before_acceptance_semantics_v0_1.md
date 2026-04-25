---
id: goal_define_proof_before_acceptance_semantics_v0_1
title: "Define proof-before-acceptance semantics v0.1"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "evals/specs/**"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Proof-before-acceptance semantics v0.1 are defined as a docs/spec-only artifact before runtime gate/proof/proofpack implementation."
  - "Spec clarifies the artifact sequence and authority boundary between gate decision, proof/proofpack, and final acceptance."
  - "Spec reconciles `Only gate writes the final decision` with `Proof comes before acceptance` without making proofpack a second decision surface."
  - "Spec explains how receipts, evals, missing-validator outcomes, and semantic assessor evidence feed closure without becoming proof or acceptance by themselves."
  - "No runtime code, CLI, schema, gate, proofpack writer, provider/model/agent adapter, or `.punk/` state is implemented."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
  - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
  - "evals/specs/gate-decision-boundary.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/minimal-receipt-fields.v0.1.md"
  - "evals/specs/missing-validator-policy.v0.1.md"
  - "evals/specs/semantic-assessor-command-interface.v0.1.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a docs/spec-only reconciliation of existing canonical laws, gate boundary, proofpack boundary, receipt fields, missing-validator policy, and semantic assessor boundary before runtime gate/proof implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
    - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
    - "evals/specs/gate-decision-boundary.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/minimal-receipt-fields.v0.1.md"
    - "evals/specs/missing-validator-policy.v0.1.md"
    - "evals/specs/semantic-assessor-command-interface.v0.1.md"
    - "work/reports/2026-04-25-sixth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; execution will produce the proof-before-acceptance semantics artifact and report."
---

## Context

Punk has separate gate decision and proofpack boundaries, plus the core laws:

- only `gate` writes the final decision;
- proof comes before acceptance.

Before runtime gate/proof/proofpack work, clarify the closure sequence so Punk does not accidentally make proofpack a second decision surface or make a gate decision look accepted before proof exists.

## Notes

Keep this docs/spec-only. Do not implement runtime gate behavior, proofpack writing, schema files, CLI commands, provider/model/agent adapters, automation, or `.punk` state.
