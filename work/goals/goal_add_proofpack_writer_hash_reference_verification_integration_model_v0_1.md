---
id: goal_add_proofpack_writer_hash_reference_verification_integration_model_v0_1
title: "Add proofpack writer hash/reference verification integration model v0.1"
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
    - "evals/specs/**"
acceptance:
  - "A side-effect-free proofpack writer hash/reference verification integration model is implemented as library behavior."
  - "The model composes explicit proofpack refs, declared artifact digest entries, structural link/hash integrity status, optional referenced artifact verification outcomes, and manifest self-digest readiness without reading files or writing artifacts."
  - "The model keeps declared digest validity, structural link/hash integrity, referenced artifact verification outcomes, and manifest self-digest status as separate evidence surfaces rather than collapsing them into one boolean."
  - "Missing, invalid, unsupported, wrong-kind/ref, mismatched, unreadable, symlink, outside-root, unverified, not-required, and optional verification states remain visible and fail closed where required by the selected policy."
  - "The model distinguishes writer-ready hash/reference evidence from gate acceptance, positive acceptance, schema validation, runtime proof storage, and canonical artifact availability."
  - "Tests or smoke coverage demonstrate ready evidence, missing/invalid declared digest blockers, non-passing verification outcome blockers, optional/unverified visibility, and no forbidden side effects."
  - "The existing first active exact-byte write slice is not broadened and does not start verifying files, hashing broad trees, persisting evidence, writing `.punk/proofs`, creating parent directories, exposing CLI behavior, writing schema files, updating indexes/latest, writing gate decisions, or creating acceptance claims."
knowledge_refs:
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
  - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md"
  - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
  - "work/reports/2026-04-30-proofpack-writer-first-active-write-slice-v0-1.md"
  - "work/reports/2026-04-30-sixty-first-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-proofpack-writer-hash-reference-verification-integration-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The selected model may use repo-tracked hash-policy, referenced artifact verification, proofpack writer, and crate-status artifacts to add side-effect-free integration behavior over explicit inputs only; escalate before active file reads, broad artifact hashing, runtime storage, schema, CLI, or platform filesystem guarantees."
  research_refs:
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md"
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md"
    - "evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-proofpack-writer-first-active-write-slice-v0-1.md"
    - "work/reports/2026-04-30-sixty-first-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/**"
    - "work/STATUS.md"
  rationale: "The next step should change current proofpack writer crate behavior by adding a side-effect-free hash/reference verification integration model and eval smoke coverage."
---

## Context

The first active proofpack writer write slice can now write exact canonical bytes to an explicit test target, but it intentionally does not verify proofpack referenced artifact refs.

Before broader writer orchestration, runtime storage, schema, CLI, `.punk/proofs`, or persisted operation evidence, the writer track needs an inspectable side-effect-free model for hash/reference verification readiness.

## Notes

Do not broaden the first active write slice.
Do not read files or hash artifact trees in this goal.
Do not write `.punk`, `.punk/proofs`, schemas, CLI output, persisted operation evidence, indexes, latest pointers, gate decisions, or acceptance claims.
Do not add adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.

The model may consume explicit verification evidence produced elsewhere, but it must not create that evidence by reading files.


## Outcome

Implemented the side-effect-free proofpack writer hash/reference verification integration model v0.1.

The model composes explicit proofpack refs, declared artifact digest evidence, structural link/hash integrity, optional referenced artifact verification outcomes, and manifest self-digest readiness while keeping those evidence surfaces separate.

It exposes fail-closed blockers for missing/invalid/unsupported/wrong-kind declared digests and non-passing required verification outcomes, while keeping optional or not-required verification evidence visible without turning it into gate acceptance.

No file reads, broad artifact hashing, `.punk`/`.punk/proofs`, runtime storage, schema files, CLI behavior, persisted operation evidence, indexes/latest, active referenced artifact file verification, broader writer orchestration, gate decision writer, acceptance claim writer, provider/model/agent adapter, automation, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init` behavior was added.

Selected next: `work/goals/goal_run_sixty_second_work_ledger_review.md`.
