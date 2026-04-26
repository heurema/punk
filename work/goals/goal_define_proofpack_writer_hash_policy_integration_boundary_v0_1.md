---
id: goal_define_proofpack_writer_hash_policy_integration_boundary_v0_1
title: "Define proofpack writer hash-policy integration boundary v0.1"
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
    - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A docs/spec boundary defines how a future proofpack writer may integrate artifact hash policy, declared artifact hashes, manifest self-digest, and referenced artifact verification outcomes."
  - "The boundary keeps proofpack writer hash-policy integration separate from active writer implementation, `.punk/proofs` runtime storage, schemas, CLI behavior, gate decisions, and acceptance claims."
  - "The boundary defines non-authority behavior for verified, mismatched, missing, unreadable, and unverified referenced artifacts."
  - "The boundary explains how existing helpers may be used later without broadening their scope: artifact hash policy helpers, exact-byte hashing, file IO hashing, referenced artifact verification, proofpack manifest digest, and structural link/hash integrity."
  - "Open follow-ups are recorded for implementation, storage/schema, CLI, privacy/redaction, partial proofpacks, and acceptance wiring."
  - "No Rust code, `.punk/` runtime state, schema files, CLI behavior, proofpack writer behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` are added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/ROADMAP.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
  - "evals/specs/proofpack-manifest-digest.v0.1.md"
  - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "crates/punk-proof/src/lib.rs"
  - "crates/punk-core/src/lib.rs"
  - "work/reports/2026-04-26-proofpack-writer-preparation-boundary-v0-1.md"
  - "work/reports/2026-04-26-thirty-fifth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded docs/spec boundary derived from repo-tracked proofpack, hash, file IO, and referenced-artifact verification artifacts; no external research is required unless a major architecture conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ROADMAP.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-preparation-boundary.v0.1.md"
    - "evals/specs/proofpack-manifest-digest.v0.1.md"
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-core/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-proofpack-writer-preparation-boundary-v0-1.md"
    - "work/reports/2026-04-26-thirty-fifth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
    - "work/reports/2026-04-26-proofpack-writer-hash-policy-integration-boundary-v0-1.md"
  rationale: "The selected goal should add docs/spec and work artifacts only while preserving runtime/code/schema/CLI boundaries."
---

## Context

Proofpack writer preparation boundary v0.1 is defined as docs/spec only.

Punk has artifact hash policy helpers, exact-byte hashing, narrow file IO hashing, referenced artifact verification, proofpack manifest digest, and structural link/hash integrity.

Before implementing proofpack writer behavior or proofpack writer hash-policy integration, define how a future writer may compose those surfaces without activating runtime authority.

## Notes

Do not add code.
Do not write `.punk` state.
Do not add schema files.
Do not add CLI commands.
Do not implement proofpack writer behavior.
Do not implement proofpack referenced-ref verification integration.
Do not implement gate decisions or acceptance claims.
Do not add adapters, automation, provider/model runners, or `punk init`.


## Outcome

Completed proofpack writer hash-policy integration boundary v0.1 as docs/spec only.

Added `evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md` to define future integration behavior for declared artifact digests, structural link/hash integrity, referenced artifact verification outcomes, manifest self-digest handling, failure/partial-proofpack policy, privacy/redaction concerns, and setup-neutral authority limits.

No Rust code, `.punk/` runtime state, schema files, CLI behavior, proofpack writer behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` were added.
