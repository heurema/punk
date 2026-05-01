---
id: goal_add_proofpack_writer_concrete_path_storage_policy_model_v0_1
title: "Add proofpack writer concrete path/storage policy model v0.1"
status: done
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-04-30
updated_at: 2026-04-30
selected_at: 2026-04-30
started_at: 2026-04-30
completed_at: 2026-04-30
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
  - "A side-effect-free `punk-proof` model represents proofpack writer concrete path/storage policy readiness and blockers derived from `evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md`."
  - "The model requires explicit storage root refs, logical target artifact refs, target path refs or target path derivation inputs, and selected storage/path policy refs before reporting ready/accepted policy evidence."
  - "The model keeps storage root refs, logical target artifact refs, target path refs, host path observations, selected policy refs, and canonical proofpack artifact refs distinct."
  - "The model fails closed for missing or rejected storage root selection, target artifact refs, target path derivation, path encoding, parent directory, symlink, canonicalization, traversal, storage-root escape, redaction, idempotency/conflict, and temp/atomic policies."
  - "The model records selected policies, host path observations, redaction state, and blockers as operational evidence only, not canonical proof, gate, receipt, schema, project-truth, or acceptance authority."
  - "The model exposes capability/boundary flags showing no active writer, `.punk/proofs`, runtime storage, schema files, CLI behavior, filesystem reads/writes/inspection/canonicalization/normalization, operation-evidence persistence, referenced-ref verification integration, gate decision, acceptance claim, adapter, automation, provider/model runner, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init` behavior."
  - "`punk-eval` smoke coverage includes the concrete path/storage policy model and keeps wording as local assessment/evidence, not acceptance."
  - "`docs/product/CRATE-STATUS.md`, the report, and `work/STATUS.md` are updated."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md"
  - "work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-boundary-v0-1.md"
  - "work/reports/2026-04-30-fifty-eighth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is side-effect-free model/eval work derived from repo-tracked concrete path/storage policy, storage/schema, host path resolution, project memory, and crate-status artifacts; no external research is required unless the work attempts to implement platform filesystem behavior."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-boundary-v0-1.md"
    - "work/reports/2026-04-30-fifty-eighth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/**"
    - "work/STATUS.md"
  rationale: "The next step should add side-effect-free model/eval coverage for the concrete path/storage policy boundary without runtime/storage/schema/CLI changes."
---

## Context

Proofpack writer concrete path/storage policy boundary v0.1 is now defined as docs/spec only.

The next safe implementation step is side-effect-free model/eval coverage that makes those policy preconditions inspectable before any active writer, runtime storage, schema, CLI, `.punk`, or filesystem work.

## Notes

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not read, write, inspect, resolve, canonicalize, normalize, or compare host filesystem paths.
Do not implement proofpack referenced-ref verification integration.
Do not persist operation evidence.
Do not write indexes or `latest` pointers.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.


## Outcome

Added a side-effect-free `punk-proof` concrete path/storage policy model v0.1 and `punk-eval` smoke coverage.

The model requires explicit storage root refs, logical target artifact refs, target path refs, selected storage/path policy refs, host path observations, redaction state, idempotency/conflict policy, temp/atomic policy, and index/latest non-authority policy before reporting writer-ready policy evidence.

It records policy refs, host path observations, redaction state, and blockers as operational evidence only. It does not add active writer behavior, `.punk/proofs`, runtime storage, schema files, CLI behavior, filesystem reads/writes/inspection/canonicalization/normalization, operation-evidence persistence, referenced-ref verification integration, gate decisions, acceptance claims, adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.
