---
id: goal_reconcile_crate_status_exact_byte_hash_computation
title: "Reconcile CRATE-STATUS exact-byte hash computation status"
status: done
owner: "vitaly"
module: "docs"
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
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "`docs/product/CRATE-STATUS.md` accurately describes `punk-core` as owning side-effect-free exact-byte artifact hash computation plus artifact digest and repo-relative artifact ref validation helpers."
  - "`docs/product/CRATE-STATUS.md` records that `punk-core` has a narrow `sha2` dependency for SHA-256 computation and does not expose dependency types in public Punk APIs."
  - "`docs/product/CRATE-STATUS.md` records that `punk-eval` smoke coverage includes exact-byte artifact hash computation helper behavior."
  - "`docs/product/CRATE-STATUS.md` keeps file IO hashing, byte normalization, schema files, proofpack writer behavior, `.punk/` runtime storage, gate decision writing, CLI behavior, adapters, automation, provider/model runners, and `punk init` deferred."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
  - "No Rust code, CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md"
  - "work/reports/2026-04-26-twenty-fourth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This is a docs-currentness update against repo-tracked implementation state and canonical docs; no external research or architecture change is required."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md"
    - "work/reports/2026-04-26-twenty-fourth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md"
  rationale: "Execution should update a canonical product doc for current exact-byte hash computation behavior and record a work report."
---

## Context

`punk-core` now exposes `compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest` and computes canonical `sha256:<64 lowercase hex>` digests from exact caller-provided bytes.

`docs/product/CRATE-STATUS.md` still says `punk-core` is dependency-free and does not compute hashes. That wording no longer matches the implemented helper subset.

## Notes

Keep this as a docs-currentness update only.

Do not change Rust code.
Do not add CLI behavior.
Do not write `.punk/` state.
Do not implement gate or proofpack writers.
Do not add file IO hashing.
Do not normalize bytes or hashes.
Do not add schema files.
Do not add adapters, automation, provider/model runners, or `punk init`.


## Outcome

Completed the CRATE-STATUS exact-byte hash computation reconciliation.

`docs/product/CRATE-STATUS.md` now records that:

- `punk-core` owns side-effect-free exact-byte artifact hash computation plus artifact digest and repo-relative artifact ref validation helpers;
- `punk-core` owns a narrow `sha2` dependency for SHA-256 computation without exposing dependency types in public Punk APIs;
- `punk-eval` smoke coverage includes exact-byte artifact hash computation helper behavior;
- file IO hashing, byte/hash normalization, proofpack writer behavior, `.punk/` runtime storage, schema files, gate decision writing, CLI behavior, adapters, automation, provider/model runners, and `punk init` remain deferred.

No runtime/code/schema/CLI/`.punk` changes were made.
