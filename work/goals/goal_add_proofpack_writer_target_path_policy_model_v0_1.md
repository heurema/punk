---
id: goal_add_proofpack_writer_target_path_policy_model_v0_1
title: "Add proofpack writer target path policy model v0.1"
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
  - "`punk-proof` exposes a side-effect-free proofpack writer target path policy model aligned with `evals/specs/proofpack-writer-file-io-boundary.v0.1.md`."
  - "The model keeps storage-root refs, target artifact refs, and target path refs separate and does not derive authority from host-local paths, current working directory, chat state, executor-local memory, indexes, or `latest` pointers."
  - "The model can classify explicit target path refs as policy-accepted or policy-rejected without reading, canonicalizing, creating, or writing filesystem paths."
  - "Rejected target path refs preserve stable reasons for absolute paths, home-relative paths, URLs, path traversal, ambiguous dot segments, empty segments, unsupported backslashes, and storage-root escape attempts."
  - "Target path policy diagnostics map to or remain compatible with existing proofpack writer file IO error/reason diagnostics without creating acceptance claims, gate decisions, writer persistence, schema files, CLI output, adapters, automation, or `.punk` runtime state."
  - "Tests cover accepted repo-runtime-style target refs, rejected path injection/escape cases, setup neutrality, no hidden authority, and no side effects."
  - "Smoke eval coverage and `docs/product/CRATE-STATUS.md` are updated if behavior changes."
  - "No `.punk` runtime state, schema file, CLI command, proofpack writer, filesystem write, gate decision, acceptance claim, provider/model runner, adapter, automation, or `punk init` is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
  - "work/reports/2026-04-26-proofpack-writer-file-io-error-reason-model-v0-1.md"
  - "work/reports/2026-04-26-forty-fourth-work-ledger-review.md"
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
  rationale: "This is a bounded side-effect-free code/model implementation derived from repo-tracked proofpack writer file IO, storage/schema, operation evidence, and crate-status artifacts; no external research is required unless implementation exposes an architecture conflict."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-file-io-boundary.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-file-io-error-reason-model-v0-1.md"
    - "work/reports/2026-04-26-forty-fourth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-proofpack-writer-target-path-policy-model-v0-1.md"
  rationale: "A new side-effect-free `punk-proof` target path policy model would change current crate behavior and smoke eval coverage."
---

## Context

Proofpack writer file IO boundary, plan model, outcome model, and error reason model are in place.

However, `ProofpackWriterTargetPathRef` is still only a non-empty ref wrapper.
A real proofpack writer must not begin filesystem side effects while target path safety, storage-root escape handling, unsupported path syntax, and hidden-authority boundaries are still implicit.

The next safe active-core slice is a side-effect-free target path policy model.
It should make path policy explicit without resolving host paths, touching the filesystem, writing `.punk` state, adding schemas, exposing CLI behavior, or claiming acceptance.

## Non-goals

Do not implement proofpack file writing.
Do not create `.punk/proofs`.
Do not add schema files.
Do not add CLI commands.
Do not resolve the current working directory as authority.
Do not canonicalize or inspect host filesystem paths.
Do not implement proofpack referenced-ref verification integration.
Do not write writer operation evidence.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
