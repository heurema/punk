---
id: goal_define_proofpack_writer_canonical_artifact_layout_v0_1
title: "Define proofpack writer canonical artifact layout v0.1"
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
    - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A docs/spec boundary defines the future proofpack writer canonical artifact layout before any writer implementation or `.punk/proofs` activation."
  - "The boundary chooses or explicitly narrows the v0.1 canonical artifact byte surface, including whether it is manifest-only JSON, wrapper JSON, separate manifest/wrapper files, or another explicit layout."
  - "The boundary records which bytes are covered by manifest self-digest and which metadata, if any, stays outside those bytes."
  - "The boundary keeps mutable indexes, `latest` pointers, operation evidence, schema validation reports, CLI output, and service mirrors non-canonical."
  - "The boundary preserves append-only/idempotent/conflict behavior and target path policy without implementing filesystem writes."
  - "The boundary records privacy/redaction and unknown-field/migration questions as deferred unless resolved."
  - "No Rust code, `.punk` runtime state, schema file, CLI command, proofpack writer, filesystem write, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "work/reports/2026-04-26-proofpack-writer-target-path-policy-model-v0-1.md"
  - "work/reports/2026-04-26-forty-fifth-work-ledger-review.md"
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
  rationale: "This is a bounded docs/spec boundary derived from repo-tracked proofpack writer storage/schema, file IO, target path policy, and crate-status artifacts; no external research is required unless a layout/authority conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-target-path-policy-model-v0-1.md"
    - "work/reports/2026-04-26-forty-fifth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-canonical-artifact-layout-v0-1.md"
  rationale: "The goal should define the canonical artifact byte/layout boundary before implementation."
---

## Context

Proofpack writer preparation, storage/schema, operation evidence, file IO, error/reason diagnostics, and target path policy boundaries/models are in place.

A future proofpack writer still must not write files until Punk defines what the canonical proofpack artifact bytes are and which surrounding metadata is non-canonical.

The next safe slice is docs/spec-only.
It should choose or narrow the v0.1 artifact layout boundary without adding a schema file or activating `.punk/proofs`.

## Non-goals

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not canonicalize or inspect host filesystem paths.
Do not implement proofpack referenced-ref verification integration.
Do not write writer operation evidence.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
