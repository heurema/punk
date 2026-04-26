---
id: goal_add_proofpack_writer_preflight_plan_model_v0_1
title: "Add proofpack writer preflight plan model v0.1"
status: ready
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "`punk-proof` exposes a side-effect-free proofpack writer preflight/plan model aligned with proofpack writer preparation, storage/schema, and operation evidence boundaries."
  - "The model can represent writer-ready preconditions, missing preflight reasons, intended target refs, intended manifest self-digest, and planned side effects without writing proofpacks or runtime state."
  - "The model can derive or hold a `planned_only` or `preflight_failed` operation evidence shape without implying canonical artifact availability, gate decisions, schema validation, receipts, or acceptance claims."
  - "Unit tests cover successful planned preflight, missing precondition visibility, evidence-only authority, setup-neutral behavior, and no writer/storage/CLI/schema side effects."
  - "Smoke eval coverage or capability reporting is updated only if needed to keep current behavior inspectable without exposing new CLI commands."
  - "`docs/product/CRATE-STATUS.md` is reconciled if current `punk-proof` behavior changes."
  - "No `.punk/` runtime state, schema files, CLI behavior, proofpack file writer, gate decisions, acceptance claims, provider/model/agent adapters, automation, or `punk init` are added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "work/reports/2026-04-26-proofpack-writer-operation-evidence-model-v0-1.md"
  - "work/reports/2026-04-26-thirty-ninth-work-ledger-review.md"
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
  rationale: "This is a bounded side-effect-free code/model implementation derived from repo-tracked proofpack writer preparation, storage/schema, operation evidence, and crate-status artifacts; no external research is required unless implementation exposes an architecture conflict."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-proofpack-writer-operation-evidence-model-v0-1.md"
    - "work/reports/2026-04-26-thirty-ninth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-proofpack-writer-preflight-plan-model-v0-1.md"
  rationale: "The selected goal may change current `punk-proof` behavior and should reconcile crate status while preserving runtime/schema/CLI authority boundaries."
---

## Context

Proofpack writer operation evidence model v0.1 is implemented as side-effect-free `punk-proof` behavior.

Before activating a proofpack file writer or `.punk/proofs`, the next safe implementation slice is a preflight/plan model that makes intended writer inputs, missing preconditions, target refs, manifest self-digest, and planned side effects explicit without performing side effects.

## Notes

Do not write `.punk` state.
Do not add schema files.
Do not add CLI commands.
Do not implement proofpack file writing.
Do not implement proofpack referenced-ref verification integration.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
