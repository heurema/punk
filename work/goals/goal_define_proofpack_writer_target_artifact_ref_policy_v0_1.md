---
id: goal_define_proofpack_writer_target_artifact_ref_policy_v0_1
title: "Define proofpack writer target artifact ref policy v0.1"
status: done
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: 2026-04-26
completed_at: 2026-04-26
blocked_by: []
scope:
  include:
    - "evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A docs/spec-only proofpack writer target artifact ref policy v0.1 is defined before active proofpack writer implementation."
  - "The policy separates proofpack id, manifest self-digest, canonical artifact identity, target artifact ref, target path ref, storage root ref, indexes, `latest` pointers, and CLI/service mirrors."
  - "The policy states whether target artifact refs may be derived from proofpack id, manifest self-digest, or both, and records any stable naming/ref vocabulary needed by a future writer."
  - "The policy rejects hidden authority from host paths, current working directory, chat state, executor-local memory, index views, `latest` pointers, service mirrors, or executor claims."
  - "The policy defines what a future implementation must treat as advisory evidence versus canonical identity without implementing file writing, schema files, CLI behavior, runtime storage, referenced-ref verification integration, gate decisions, or acceptance claims."
  - "The report and `work/STATUS.md` are updated."
  - "No Rust code, `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem write, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "work/reports/2026-04-26-proofpack-writer-canonical-artifact-model-v0-1.md"
  - "work/reports/2026-04-26-forty-seventh-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded docs/spec policy definition derived from repo-tracked proofpack writer file IO, canonical artifact layout, canonical artifact model, and crate-status artifacts; no external research is required unless target artifact identity conflicts with canonical proofpack identity."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-canonical-artifact-model-v0-1.md"
    - "work/reports/2026-04-26-forty-seventh-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md"
  rationale: "The goal should define a docs/spec-only target artifact ref policy boundary before code or runtime writer behavior."
---

## Context

Proofpack writer canonical artifact layout and canonical artifact model v0.1 are in place.

Target path policy is also implemented as side-effect-free behavior, but target artifact refs still need a canonical policy boundary before any writer chooses file names, artifact refs, indexes, or `latest` pointers.

The next safe slice is a docs/spec-only policy that defines how a future proofpack writer should identify the target artifact without turning paths, indexes, service mirrors, executor memory, or CLI output into authority.

## Non-goals

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not change Rust code.
Do not read or write the filesystem.
Do not canonicalize or inspect host filesystem paths.
Do not implement proofpack referenced-ref verification integration.
Do not write operation evidence.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.

## Outcome

Defined proofpack writer target artifact ref policy v0.1 as docs/spec-only boundary.

The policy selects `(proofpack_id, manifest_self_digest)` as the v0.1 target artifact identity, keeps logical target artifact refs separate from canonical bytes, storage roots, target paths, indexes, `latest` pointers, CLI output, service mirrors, and executor claims, and records `proofpack:<proofpack_id>@<manifest_self_digest>` as a recommended display ref vocabulary for future implementations.

No Rust code, `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem write, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` was added.
