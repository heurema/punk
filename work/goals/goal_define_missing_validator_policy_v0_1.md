---
id: goal_define_missing_validator_policy_v0_1
title: "Define missing-validator policy v0.1"
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
    - "docs/product/**"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Missing-validator policy v0.1 is defined as a docs/spec-only artifact before runtime gate/proof/storage work."
  - "Policy distinguishes unavailable, skipped, failed, unsupported, and deferred validators without treating missing checks as proof."
  - "Policy preserves setup-neutral UX: Punk reports missing prerequisites and policy impact without forcing user setup or silently passing."
  - "Policy defines how reports/evals/future gate should classify missing validators as evidence inputs, not final decisions."
  - "No runtime code, CLI, schema, gate, proofpack, provider/model/agent adapter, or `.punk/` state is implemented."
knowledge_refs:
  - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
  - "evals/specs/project-memory-storage-boundary.v0.1.md"
  - "work/reports/2026-04-25-fifth-work-ledger-review.md"
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
  rationale: "The policy is a docs/spec-only follow-up grounded in existing executor-agnostic validation, receipt, gate, proofpack, and storage-boundary artifacts."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
    - "evals/specs/run-receipt-boundary.v0.1.md"
    - "evals/specs/gate-decision-boundary.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/project-memory-storage-boundary.v0.1.md"
    - "work/reports/2026-04-25-fifth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; execution will produce the missing-validator policy spec and report."
---

## Context

Punk is setup-neutral and executor-agnostic, but validation cannot silently pass when required local validators are missing.

Before runtime gate/proof/storage work, define how unavailable, skipped, failed, unsupported, and deferred validators are represented and how they affect evidence, reports, and future gate behavior.

## Notes

Keep this docs/spec-only. Do not implement validators, runtime gate behavior, CLI commands, schemas, `.punk/` storage, proofpack writing, or adapters.
