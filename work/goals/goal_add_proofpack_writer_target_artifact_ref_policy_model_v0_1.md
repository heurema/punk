---
id: goal_add_proofpack_writer_target_artifact_ref_policy_model_v0_1
title: "Add proofpack writer target artifact ref policy model v0.1"
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
  - "`punk-proof` adds a side-effect-free proofpack writer target artifact ref policy model aligned with `evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md`."
  - "The model represents v0.1 target artifact identity as both `proofpack_id` and `manifest_self_digest`, rejecting or making missing/invalid identity parts visible instead of inventing refs."
  - "The model exposes structured identity fields and a stable logical display ref shaped like `proofpack:<proofpack_id>@<manifest_self_digest>` without treating that display ref as a filesystem path."
  - "The model keeps target artifact refs separate from canonical artifact bytes, target path refs, storage root refs, indexes, `latest` pointers, CLI output, service mirrors, and executor claims."
  - "Boundary/capability flags show no filesystem reads/writes, `.punk/proofs`, runtime storage, schema files, CLI behavior, referenced-ref verification, operation-evidence persistence, index/latest writes, gate decision, or acceptance claim side effects."
  - "Smoke eval coverage is added for identity pair requirements, logical ref rendering, evidence-vs-authority boundaries, and no runtime side effects."
  - "`docs/product/CRATE-STATUS.md`, the goal, the report, and `work/STATUS.md` are updated."
  - "No `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem write, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
  - "evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md"
  - "work/reports/2026-04-26-forty-eighth-work-ledger-review.md"
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
  rationale: "This is a bounded side-effect-free code/model implementation derived from repo-tracked proofpack writer target artifact ref policy, canonical artifact model, manifest digest, and crate-status artifacts; no external research is required unless implementation exposes an identity/authority conflict."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
    - "evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md"
    - "work/reports/2026-04-26-forty-eighth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md"
  rationale: "The goal should add a side-effect-free `punk-proof` target artifact ref policy model, smoke eval coverage, and status/docs updates without runtime/storage/schema/CLI behavior."
---

## Context

Proofpack writer target artifact ref policy v0.1 is defined as docs/spec only.

The selected v0.1 target artifact identity is `(proofpack_id, manifest_self_digest)`, with `proofpack:<proofpack_id>@<manifest_self_digest>` as a logical display ref that is not a filesystem path.

Before active proofpack writing, `.punk/proofs`, schema files, CLI behavior, or referenced-ref verification integration, add a side-effect-free Rust model that makes this identity/ref boundary inspectable and smoke-covered.

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
