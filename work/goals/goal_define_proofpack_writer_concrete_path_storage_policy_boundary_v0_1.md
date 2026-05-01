---
id: goal_define_proofpack_writer_concrete_path_storage_policy_boundary_v0_1
title: "Define proofpack writer concrete path/storage policy boundary v0.1"
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
    - "evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A docs/spec-only boundary defines the concrete future path/storage policies required before any active proofpack writer implementation."
  - "The boundary covers path encoding, parent directory behavior, symlink handling, canonicalization policy, traversal rejection, storage-root escape rejection, redaction/absolute-path handling, and storage root to target path mapping at policy level only."
  - "The boundary keeps storage root refs, logical target artifact refs, target path refs, host path observations, and canonical proofpack artifacts distinct."
  - "The boundary records that selected path/storage policies and host path observations are future preconditions or operational evidence only, not canonical proof, gate, receipt, schema, project-truth, or acceptance authority."
  - "The boundary preserves append-only canonical proofpack artifact semantics and keeps indexes, `latest` pointers, wrappers, service mirrors, dashboards, operation evidence, and host path observations non-canonical."
  - "No Rust code, active proofpack writer, `.punk/proofs`, runtime storage, schema files, CLI behavior, filesystem reads/writes/inspection/canonicalization, operation-evidence persistence, referenced-ref verification integration, gate decisions, acceptance claims, adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init` is added."
  - "The report and `work/STATUS.md` are updated, and a follow-up advisory review is selected if needed."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/ROADMAP.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-preflight-integration-boundary.v0.1.md"
  - "work/reports/2026-04-30-proofpack-writer-storage-schema-host-path-reconciliation-v0-1.md"
  - "work/reports/2026-04-30-fifty-seventh-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded docs/spec policy boundary derived from repo-tracked proofpack writer file IO, storage/schema, host path resolution, and preflight artifacts; external research is not required unless the work attempts to settle cross-platform filesystem behavior beyond repo policy."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-preflight-integration-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-proofpack-writer-storage-schema-host-path-reconciliation-v0-1.md"
    - "work/reports/2026-04-30-fifty-seventh-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md"
    - "work/reports/**"
    - "work/STATUS.md"
  rationale: "The next step should define concrete path/storage policy semantics before active proofpack writer implementation or runtime storage activation."
---

## Context

Proofpack writer storage/schema boundary v0.1 is now reconciled with the side-effect-free host path resolution model.

That reconciliation explicitly requires future file IO storage to make path encoding, parent directory behavior, symlink handling, canonicalization policy, traversal rejection, storage-root escape rejection, and redaction/absolute-path handling explicit before canonical artifact writes.

Active proofpack writer implementation remains too broad until those concrete path/storage policies are defined at boundary level.

## Notes

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not add Rust code.
Do not read, write, inspect, resolve, canonicalize, or normalize host filesystem paths.
Do not implement proofpack referenced-ref verification integration.
Do not persist operation evidence.
Do not write indexes or `latest` pointers.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.


## Outcome

Completed proofpack writer concrete path/storage policy boundary v0.1 as docs/spec only.

Added `evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md`.

Selected `work/goals/goal_run_fifty_eighth_work_ledger_review.md` as the next advisory review goal.

No Rust code, active proofpack writer, `.punk/proofs`, runtime storage, schema file, CLI behavior, filesystem read/write/inspection/canonicalization, operation-evidence persistence, referenced-ref verification integration, gate decision, acceptance claim, adapter, automation, provider/model runner, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init` was added.
