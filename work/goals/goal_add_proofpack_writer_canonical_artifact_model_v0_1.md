---
id: goal_add_proofpack_writer_canonical_artifact_model_v0_1
title: "Add proofpack writer canonical artifact model v0.1"
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
  - "`punk-proof` adds a side-effect-free canonical artifact model that represents the v0.1 canonical proofpack artifact body as exact deterministic manifest renderer bytes."
  - "The model records or exposes the manifest self-digest as content identity for those exact bytes without embedding it into hashed manifest bytes."
  - "The model keeps wrapper metadata, storage roots, target refs, target paths, operation evidence, schema validation reports, indexes, `latest` pointers, CLI output, and service mirrors outside canonical artifact bytes."
  - "The model exposes boundary/capability flags showing no filesystem writes, `.punk/proofs`, runtime storage, schema files, CLI behavior, referenced-ref verification, gate decision, or acceptance claim side effects."
  - "Smoke eval coverage is added for canonical artifact byte identity, digest coverage, non-canonical metadata separation, and no runtime side effects."
  - "`docs/product/CRATE-STATUS.md`, the goal, the report, and `work/STATUS.md` are updated."
  - "No `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem write, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "work/reports/2026-04-26-proofpack-writer-canonical-artifact-layout-v0-1.md"
  - "work/reports/2026-04-26-forty-sixth-work-ledger-review.md"
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
  rationale: "This is a bounded side-effect-free code/model implementation derived from repo-tracked proofpack manifest digest and canonical artifact layout specs; no external research is required unless a layout/authority conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-canonical-artifact-layout-v0-1.md"
    - "work/reports/2026-04-26-forty-sixth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-proofpack-writer-canonical-artifact-model-v0-1.md"
  rationale: "The goal should add a side-effect-free `punk-proof` model, smoke eval coverage, and status/docs updates without runtime/storage/schema/CLI behavior."
---

## Context

Proofpack writer canonical artifact layout v0.1 is defined as docs/spec only.

The selected v0.1 canonical proofpack artifact body is exact deterministic proofpack manifest JSON renderer bytes, and manifest self-digest covers exactly those bytes.

Before active proofpack writing, `.punk/proofs`, schema files, CLI behavior, or referenced-ref verification integration, add a side-effect-free Rust model that makes this byte/layout boundary inspectable and testable.

## Non-goals

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not read or write the filesystem.
Do not canonicalize or inspect host filesystem paths.
Do not implement proofpack referenced-ref verification integration.
Do not write operation evidence.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
