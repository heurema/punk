---
id: goal_add_artifact_hash_computation_helper_v0_1
title: "Add artifact hash computation helper v0.1"
status: done
owner: "vitaly"
module: "core"
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
    - "Cargo.lock"
    - "crates/punk-core/Cargo.toml"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/Cargo.toml"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "evals/specs/**"
    - "crates/punk-proof/**"
acceptance:
  - "`punk-core` exposes a side-effect-free helper that computes canonical artifact digests from exact caller-provided bytes."
  - "The helper output is canonical `sha256:<64 lowercase hex>` and passes existing artifact digest validation."
  - "The helper matches known SHA-256 vectors for empty bytes and `abc`."
  - "Different byte sequences, including different newline forms, produce different digests; no byte normalization is performed."
  - "If a SHA-256 dependency is added, it is narrow, owned by `punk-core`, and no dependency types leak into public Punk APIs."
  - "`punk-core` capability flags may set `computes_hashes = true`, while `normalizes_artifact_bytes = false` and `writes_runtime_state = false` remain false."
  - "Smoke eval coverage records helper behavior as local assessment only, not proof or final acceptance."
  - "No file IO helper, path/ref mapping, proofpack writer, `.punk` runtime storage, schema file, CLI behavior, gate decision writer, acceptance claim, adapter, automation, provider/model runner, or `punk init` is added."
knowledge_refs:
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "docs/product/CRATE-STATUS.md"
  - "work/reports/2026-04-26-twenty-third-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded implementation of an accepted repo-tracked helper boundary and may add one narrow SHA-256 dependency to `punk-core`; no Deep Research is needed unless dependency scope or crypto boundary conflicts appear."
  research_refs:
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md"
    - "work/reports/2026-04-26-twenty-third-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md"
  rationale: "Implementation will change active-core helper behavior and smoke eval coverage; the work report records the bounded implementation while canonical docs/specs remain unchanged unless implementation discovers mismatch."
---

## Context

Artifact hash policy v0.1 and artifact hash computation helper boundary v0.1 are defined.

`punk-core` currently validates digest/ref shape only and explicitly does not compute hashes.

The next narrow implementation is exact-byte SHA-256 computation over caller-provided bytes, with no file IO or runtime behavior.

## Notes

Keep this as a side-effect-free helper only.

Do not add file IO.
Do not read artifact paths.
Do not map runtime ids.
Do not normalize bytes.
Do not write `.punk/` state.
Do not add CLI behavior.
Do not add schema files.
Do not implement proofpack writer behavior.
Do not write gate decisions.
Do not create acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.

## Outcome

Completed artifact hash computation helper v0.1.

`punk-core` now exposes `compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest`, computing canonical SHA-256 digest metadata from exact caller-provided bytes.

The implementation added a narrow `sha2` dependency to `punk-core`, updated capability flags to `computes_hashes = true`, and added smoke eval coverage without file IO, byte normalization, runtime storage, schema, CLI, proofpack writer, gate decision, adapter, automation, provider/model runner, or `punk init` behavior.
