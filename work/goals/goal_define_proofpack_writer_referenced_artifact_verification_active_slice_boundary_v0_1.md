---
id: goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1
title: "Define proofpack writer referenced artifact verification active slice boundary v0.1"
status: blocked
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-04-30
updated_at: 2026-04-30
selected_at: 2026-04-30
started_at: null
completed_at: null
blocked_by:
  - "work/goals/goal_run_sixty_fourth_work_ledger_review.md"
scope:
  include:
    - "evals/specs/proofpack-writer-referenced-artifact-verification-active-slice-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
    - "docs/product/CRATE-STATUS.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A docs/spec-only boundary defines the first future active proofpack writer referenced artifact verification slice before implementation."
  - "The boundary requires explicit proofpack refs, declared artifact digests, explicit repo root, explicit repo-relative artifact ref mapping, explicit verification policy, and explicit caller-provided verification intent."
  - "The boundary forbids hidden authority from current working directory, repository discovery, global config, IDE state, executor memory, mutable indexes, service mirrors, dashboards, URLs, network fetches, or `punk init`."
  - "The boundary keeps referenced artifact verification evidence separate from writer readiness, canonical artifact availability, manifest self-digest, persisted operation evidence, schema validation, gate decisions, and acceptance claims."
  - "The boundary records fail-closed treatment for missing, invalid, unsupported, wrong-kind/ref, mismatched, unreadable, symlink, not-regular-file, outside-root, invalid-root, invalid-ref, invalid-expected-digest, unverified, not-required, and optional states."
  - "The boundary does not implement file reads, broad artifact-tree hashing, proofpack writing, `.punk/proofs`, runtime storage, schema files, CLI behavior, persisted operation evidence, indexes/latest, gate decisions, acceptance claims, adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`."
  - "Exactly one next ready goal is selected after the boundary work."
knowledge_refs:
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md"
  - "work/reports/2026-04-30-proofpack-writer-hash-reference-verification-integration-model-v0-1.md"
  - "work/reports/2026-04-30-sixty-second-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-proofpack-writer-track-pause-for-user-intent-alignment-v0-1.md"
  - "work/reports/2026-04-30-sixty-third-work-ledger-review.md"
  - "work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The selected boundary may use repo-tracked file IO hashing, referenced artifact verification, hash-policy integration, first active write slice, and crate-status artifacts to define a docs/spec-only future active verification slice; escalate before active file reads, platform filesystem guarantees, broad hashing, runtime storage, schema, or CLI behavior."
  research_refs:
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-proofpack-writer-hash-reference-verification-integration-model-v0-1.md"
    - "work/reports/2026-04-30-sixty-second-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/proofpack-writer-referenced-artifact-verification-active-slice-boundary.v0.1.md"
    - "work/reports/**"
    - "work/STATUS.md"
  rationale: "The next step should define a boundary only, without changing crate/runtime behavior."
---

## Context

The proofpack writer now has a first active exact-byte write slice and a side-effect-free hash/reference integration model.

The next narrow gap is not broader writer orchestration or runtime storage. It is defining exactly how a future writer may actively verify referenced artifacts using explicit local file evidence without turning file reads, hashes, or executor claims into hidden authority.

## Notes

Do not implement active file reads in this goal.
Do not change `crates/**`, `.punk/**`, `schemas/**`, CLI behavior, runtime storage, broader active writer orchestration, operation-evidence persistence, broad artifact-tree hashing, gate decisions, acceptance claims, adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.
## Pause note

This goal is parked, not abandoned. It was selected by the sixty-second advisory Work Ledger Review, but no work started on it before the maintainer raised the upstream UX concern.

The upstream user request -> intent -> contract UX boundary and side-effect-free intent/contract draft model are now in place. Resume this Writer boundary only after the sixty-fourth advisory Work Ledger Review or a later review explicitly selects Writer again.
