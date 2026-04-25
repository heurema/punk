---
id: goal_define_minimal_receipt_fields_v0_1
title: "Define minimal receipt fields v0.1"
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
    - "evals/specs/**"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Minimal receipt fields v0.1 are defined as a docs/spec-only artifact before receipt schema/runtime work."
  - "Spec covers contract refs, run refs, scope refs, validator outcome refs, missing-validator evidence gaps, produced artifact refs, and boundary notes."
  - "Spec preserves receipt as run evidence, not gate decision or proofpack."
  - "No runtime code, CLI, schema, gate, proofpack, provider/model/agent adapter, or `.punk/` state is implemented."
knowledge_refs:
  - "evals/specs/run-receipt-boundary.v0.1.md"
  - "evals/specs/missing-validator-policy.v0.1.md"
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
  rationale: "The receipt fields spec is a docs/spec-only follow-up grounded in the run receipt boundary and missing-validator policy."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "evals/specs/run-receipt-boundary.v0.1.md"
    - "evals/specs/missing-validator-policy.v0.1.md"
    - "work/reports/2026-04-25-missing-validator-policy-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; execution will produce the minimal receipt fields spec and report."
---

## Context

The missing-validator policy defines how unavailable/skipped/failed/unsupported/deferred validators remain visible evidence inputs.

Before runtime receipt schema or `.punk/runs` storage work, define the minimal receipt fields needed to carry bounded run evidence and validator outcome refs without turning receipts into gate decisions or proofpacks.

## Notes

Keep this docs/spec-only. Do not implement receipt schema, validators, runtime storage, CLI commands, gate behavior, proofpack writing, or adapters.
