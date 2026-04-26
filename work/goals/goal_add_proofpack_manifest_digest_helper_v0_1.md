---
id: goal_add_proofpack_manifest_digest_helper_v0_1
title: "Add proofpack manifest digest helper v0.1"
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
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "`punk-proof` exposes a side-effect-free helper that computes a proofpack manifest self-digest from exact UTF-8 bytes of deterministic in-memory manifest rendering."
  - "The helper uses `punk-core` exact-byte SHA-256 computation and does not add a new hash dependency to `punk-proof`."
  - "The returned digest is canonical `sha256:<64 lowercase hex>` metadata and passes existing artifact digest validation."
  - "Tests cover deterministic repeated calls, equivalence to hashing `render_manifest_json().as_bytes()`, no recursive self-inclusion, and no referenced-artifact verification side effect."
  - "Smoke eval coverage records proofpack manifest digest helper behavior as local assessment only."
  - "Capability/boundary flags distinguish manifest self-digest computation from referenced artifact hash computation, file IO hashing, hash normalization, proofpack writer behavior, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`."
  - "No `.punk/` state, runtime writer, schema file, CLI command, provider/model/agent adapter, automation, file IO hashing, proofpack writer, gate decision writer, or acceptance claim is added."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
knowledge_refs:
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "crates/punk-proof/src/lib.rs"
  - "crates/punk-core/src/lib.rs"
  - "crates/punk-eval/src/lib.rs"
  - "work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md"
  - "work/reports/2026-04-26-twenty-sixth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded implementation of the accepted repo-tracked proofpack manifest digest boundary using existing `punk-core` exact-byte hashing; no external research is required unless implementation discovers a boundary or dependency conflict."
  research_refs:
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md"
    - "work/reports/2026-04-26-twenty-sixth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md"
  rationale: "Implementation should update proof/eval code and record the bounded active-core behavior without changing canonical product docs in the same slice."
---

## Context

Proofpack manifest digest boundary v0.1 is defined.

`punk-proof` already renders deterministic in-memory proofpack manifests.
`punk-core` already computes canonical SHA-256 digests from exact caller-provided bytes.

The next bounded implementation can combine those two existing capabilities to compute a manifest self-digest without adding file IO, proofpack writing, runtime storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, or `punk init`.

## Notes

Do not add a new SHA-256 dependency to `punk-proof`.
Do not write proofpacks.
Do not write `.punk` state.
Do not add file IO hashing.
Do not verify referenced artifact bytes.
Do not normalize bytes or hashes.
Do not add schema files.
Do not add CLI behavior.
Do not write gate decisions.
Do not create acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.


## Outcome

Implemented a side-effect-free `compute_proofpack_manifest_digest(&Proofpack)` helper in `punk-proof`.

The helper hashes the exact UTF-8 bytes returned by `Proofpack::render_manifest_json()` through `punk-core` exact-byte hash computation and returns canonical `sha256:<64 lowercase hex>` metadata.

Smoke eval coverage now records the manifest digest helper as local assessment only. Boundary flags distinguish manifest self-digest computation from referenced artifact hash computation, file IO hashing, hash normalization, proofpack writer behavior, runtime storage, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`.

No `.punk/` state, runtime writer, schema file, CLI command, provider/model/agent adapter, automation, file IO hashing, proofpack writer, gate decision writer, or acceptance claim was added.
