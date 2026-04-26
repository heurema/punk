---
id: goal_define_referenced_artifact_verification_boundary_v0_1
title: "Define referenced artifact verification boundary v0.1"
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
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "A docs/spec boundary defines what referenced artifact verification means for proofpack refs and hashes without implementing it."
  - "The boundary distinguishes referenced artifact verification from file IO digest computation, proofpack manifest self-digest, structural link/hash integrity checks, gate decisions, proofpack writer behavior, and acceptance claims."
  - "The boundary records allowed v0.1 inputs, path/root/ref constraints, exact-byte comparison semantics, mismatch/missing/unreadable/symlink/directory outcomes, and evidence vs authority limits."
  - "The boundary keeps `.punk/` runtime storage, schema files, CLI behavior, proofpack writer behavior, gate/eval/proof orchestration, adapters, automation, provider/model runners, and `punk init` deferred."
  - "No Rust code, CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "crates/punk-core/src/lib.rs"
  - "crates/punk-proof/src/lib.rs"
  - "work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md"
  - "work/reports/2026-04-26-thirty-first-work-ledger-review.md"
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
  rationale: "This is a docs/spec boundary derived from repo-tracked Punk proof/hash/file-IO artifacts; no external research is required unless an architecture conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-proof/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md"
    - "work/reports/2026-04-26-thirty-first-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs/spec-only
  required_updates:
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md"
  rationale: "The selected goal should define verification semantics before implementation while preserving runtime/proof/CLI boundaries."
---

## Context

Punk now has:

- artifact hash policy helpers;
- exact-byte hash computation;
- proofpack structural link/hash integrity checks;
- proofpack manifest self-digest metadata;
- a narrow file IO artifact hashing helper for one explicit regular file.

`docs/product/CRATE-STATUS.md` still correctly defers referenced artifact byte verification and referenced artifact hash computation for proofpack refs.

Before implementing verification or proofpack writer behavior, define the boundary for comparing a proofpack artifact ref/hash claim against explicit file bytes.

## Notes

Do not change Rust code.
Do not add schema files.
Do not add CLI commands.
Do not write `.punk` state.
Do not implement proofpack writer behavior.
Do not implement referenced artifact byte verification.
Do not imply gate decisions or acceptance authority.
Do not add adapters, automation, provider/model runners, or `punk init`.
