---
id: goal_add_proofpack_writer_host_path_resolution_model_v0_1
title: "Add proofpack writer host path resolution model v0.1"
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
  - "`punk-proof` exposes a side-effect-free proofpack writer host path resolution model aligned with `evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md`."
  - "The model requires explicit storage root ref, target path ref, logical target artifact ref when proofpack-targeted, and selected policy refs; it does not infer missing inputs from current working directory, global config, IDE state, chat state, executor-local memory, mutable indexes, service mirrors, dashboards, or `punk init`."
  - "The model keeps storage root ref, logical target artifact ref, target path ref, and host path observation distinct."
  - "The model fails closed for missing policy, missing/invalid refs, unsafe path encoding, parent directory ambiguity, symlink-disallowed observations, canonicalization ambiguity, traversal, storage-root escape, ambiguous host path observation, and required redaction blockers."
  - "Host path observations are represented as operational evidence only and do not imply proofpack availability, referenced artifact verification, schema validation, operation-evidence persistence, gate acceptance, or positive acceptance claims."
  - "Smoke eval coverage verifies observed, blocked, and not-selected cases plus no-side-effect/no-authority boundary flags."
  - "Boundary/capability flags continue to show no filesystem reads/writes/inspection/canonicalization, `.punk/proofs`, runtime storage, schema files, CLI behavior, active proofpack writer, operation-evidence persistence, referenced-ref verification integration, gate decision, acceptance claim, adapter, automation, provider/model runner, or `punk init` side effects."
  - "`docs/product/CRATE-STATUS.md`, the goal, the report, and `work/STATUS.md` are updated."
  - "No `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem read/write/inspection/canonicalization, operation-evidence persistence, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
  - "evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md"
  - "evals/specs/proofpack-writer-preflight-integration-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-active-behavior-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md"
  - "work/reports/2026-04-27-proofpack-writer-active-behavior-model-v0-1.md"
  - "work/reports/2026-04-29-proofpack-writer-host-path-resolution-boundary-v0-1.md"
  - "work/reports/2026-04-30-fifty-fifth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-proofpack-writer-host-path-resolution-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded side-effect-free code/model implementation derived from repo-tracked proofpack writer host path resolution boundary and existing proofpack writer file-IO/preflight/active behavior models; no external research is required unless implementation exposes active filesystem, authority, storage, schema, or CLI behavior."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md"
    - "evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md"
    - "evals/specs/proofpack-writer-preflight-integration-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-active-behavior-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md"
    - "work/reports/2026-04-27-proofpack-writer-active-behavior-model-v0-1.md"
    - "work/reports/2026-04-29-proofpack-writer-host-path-resolution-boundary-v0-1.md"
    - "work/reports/2026-04-30-fifty-fifth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/**"
    - "work/STATUS.md"
  rationale: "The goal should add side-effect-free `punk-proof` model behavior, smoke eval coverage, and status/docs without runtime/storage/schema/CLI/filesystem side effects."
---

## Context

Proofpack writer host path resolution boundary v0.1 is defined as docs/spec only.

The boundary makes future path resolution explicit and keeps these surfaces separate:

```text
storage root ref != logical target artifact ref
target artifact ref != target path ref
target path ref != host path observation
host path observation != proof authority
```

The next smallest implementation step is to add a side-effect-free host path resolution model in `punk-proof` and cover it in smoke evals while keeping actual filesystem IO, host path canonicalization, `.punk/proofs`, runtime storage, schema files, CLI behavior, operation-evidence persistence, referenced-ref verification integration, gate decisions, and acceptance claims deferred.

The model should make future host path observations inspectable without becoming a writer, gate, proof authority, receipt authority, schema authority, project truth, or acceptance authority.

## Non-goals

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not read, write, inspect, resolve, canonicalize, or normalize host filesystem paths.
Do not create parent directories.
Do not follow symlinks.
Do not implement proofpack referenced-ref verification integration.
Do not persist operation evidence.
Do not write indexes or `latest` pointers.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.


## Outcome

Completed the side-effect-free proofpack writer host path resolution model v0.1.

`punk-proof` now exposes `ProofpackWriterHostPathResolutionModel`, selected host path policy refs, stable blocker vocabulary, observation statuses, host path kinds, and boundary flags for explicit host path observations.

`punk-eval` smoke coverage now checks observed, blocked, and not-selected cases, plus vocabulary stability and no-side-effect/no-authority boundaries.

No `.punk` runtime state, schema file, CLI command, active proofpack writer, filesystem read/write/inspection/canonicalization, operation-evidence persistence, referenced-ref verification integration, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` was added.
