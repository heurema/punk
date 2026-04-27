---
id: goal_align_proofpack_writer_target_ref_models_with_policy_v0_1
title: "Align proofpack writer target-ref models with target artifact policy v0.1"
status: done
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-04-27
updated_at: 2026-04-27
selected_at: 2026-04-27
started_at: 2026-04-27
completed_at: 2026-04-27
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
  - "Existing side-effect-free proofpack writer preflight/file-IO/target-path policy model fixtures or helpers are aligned with the target artifact ref policy model."
  - "Where a model represents a target artifact ref, it uses or can derive the logical `proofpack:<proofpack_id>@<manifest_self_digest>` value from explicit policy-model inputs instead of path-like refs."
  - "Target artifact refs remain distinct from target path refs and storage root refs."
  - "Smoke eval coverage verifies that target artifact refs are logical non-path metadata while target path refs remain separate path-policy inputs."
  - "Boundary/capability flags continue to show no filesystem reads/writes, `.punk/proofs`, runtime storage, schema files, CLI behavior, referenced-ref verification integration, operation-evidence persistence, index/latest writes, service-mirror authority, executor-claim proof authority, gate decision, or acceptance claim side effects."
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
  - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md"
  - "work/reports/2026-04-27-forty-ninth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-27-proofpack-writer-target-ref-model-policy-alignment-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded side-effect-free code/model alignment derived from repo-tracked target artifact ref policy, target artifact ref policy model, writer model, and crate-status artifacts; no external research is required unless implementation exposes a target-ref authority conflict."
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
    - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md"
    - "work/reports/2026-04-27-forty-ninth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-27-proofpack-writer-target-ref-model-policy-alignment-v0-1.md"
  rationale: "The goal should align side-effect-free `punk-proof` target-ref model behavior, smoke eval coverage, and status/docs without runtime/storage/schema/CLI behavior."
---

## Context

Proofpack writer target artifact ref policy model v0.1 is implemented as side-effect-free Rust behavior.

The selected target artifact identity is `(proofpack_id, manifest_self_digest)`, rendered as logical non-path metadata shaped like `proofpack:<proofpack_id>@<manifest_self_digest>`.

Some existing side-effect-free writer models and smoke fixtures predate that policy and still use path-like `ProofpackWriterTargetRef` values in preflight/file-IO/target-path policy contexts.

Before active proofpack writing, `.punk/proofs`, schema files, CLI behavior, or proofpack referenced-ref verification integration, align those target-ref consumers with the logical target artifact ref policy while preserving target path/storage root separation.

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


## Outcome

Completed the side-effect-free target-ref model alignment.

`ProofpackWriterTargetRef` can now be derived from an accepted `ProofpackWriterTargetArtifactRefPolicyModel` logical ref, and exposes helpers that distinguish logical proofpack artifact refs from path-like refs.

Updated proof and smoke fixtures so target artifact refs use logical `proofpack:<proofpack_id>@<manifest_self_digest>` metadata while target path refs remain separate explicit path-policy inputs.

No runtime/storage/schema/CLI/`.punk`, active writer, filesystem write, referenced-ref verification integration, gate decision, acceptance claim, adapter, automation, provider/model runner, or `punk init` was added.
