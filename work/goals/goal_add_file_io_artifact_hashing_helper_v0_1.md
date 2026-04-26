---
id: goal_add_file_io_artifact_hashing_helper_v0_1
title: "Add file IO artifact hashing helper v0.1"
status: ready
owner: "vitaly"
module: "core"
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
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "docs/product/**"
    - "evals/specs/**"
    - "crates/punk-proof/**"
    - "crates/punk-cli/**"
acceptance:
  - "`punk-core` exposes a narrow file IO helper that computes canonical artifact digests from one explicit repo-relative artifact ref under one explicit repo root."
  - "The helper reuses existing exact-byte digest computation after reading bytes and returns canonical `sha256:<64 lowercase hex>` metadata."
  - "The helper rejects or reports invalid refs, refs outside the explicit repo root, missing files, directories, symlinks, and non-regular files without representing them as passing."
  - "Tests cover known file bytes, no normalization, missing files, directories, symlink stance where supported by the platform, and no `.punk`/proofpack/gate/CLI side effects."
  - "Smoke eval coverage records file IO artifact hashing helper behavior as local assessment only, not referenced artifact verification, proof, or final acceptance."
  - "Capability/boundary flags distinguish file artifact digest computation from referenced artifact byte verification, proofpack writer behavior, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`."
  - "No proofpack writer, referenced artifact byte verification, `.punk` runtime storage, schema file, CLI command, gate decision writer, acceptance claim, provider/model/agent adapter, automation, or `punk init` is added."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
knowledge_refs:
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "docs/product/CRATE-STATUS.md"
  - "crates/punk-core/src/lib.rs"
  - "crates/punk-eval/src/lib.rs"
  - "work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md"
  - "work/reports/2026-04-26-twenty-ninth-work-ledger-review.md"
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
  rationale: "This is a bounded implementation of the accepted repo-tracked file IO artifact hashing boundary using existing `punk-core` exact-byte hashing; no external research is required unless implementation discovers a path, symlink, or platform boundary conflict."
  research_refs:
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md"
    - "work/reports/2026-04-26-twenty-ninth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md"
  rationale: "Implementation should update active-core helper behavior and smoke eval coverage while recording the bounded behavior in the work report; canonical product docs remain unchanged unless implementation discovers a mismatch."
---

## Context

File IO artifact hashing boundary v0.1 is defined.

`punk-core` already validates repo-relative artifact refs and computes canonical SHA-256 digests from exact caller-provided bytes.

The next narrow implementation can add a file IO helper that reads one explicit regular file under one explicit repo root and delegates byte hashing to the existing exact-byte helper.

## Notes

Keep this as a library helper only.

Do not add CLI behavior.
Do not write `.punk` state.
Do not write proofpacks.
Do not verify referenced artifact bytes.
Do not scan directories.
Do not follow symlinks silently.
Do not normalize file bytes.
Do not add schema files.
Do not write gate decisions.
Do not create acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.
