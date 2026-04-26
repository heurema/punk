---
id: report_2026_04_26_proofpack_writer_target_artifact_ref_policy_model_v0_1
goal_id: goal_add_proofpack_writer_target_artifact_ref_policy_model_v0_1
actor: vitaly
created_at: 2026-04-26
kind: execution
---

## Summary

Implemented proofpack writer target artifact ref policy model v0.1 as side-effect-free `punk-proof` behavior.

What changed:

- added `PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION`;
- added stable status/reason vocabulary for accepted/rejected target artifact ref policy outcomes;
- added `ProofpackWriterTargetArtifactIdentity` for the required `(proofpack_id, manifest_self_digest, layout)` tuple;
- added `ProofpackWriterTargetArtifactRef` for the logical display ref `proofpack:<proofpack_id>@<manifest_self_digest>`;
- added `ProofpackWriterTargetArtifactRefPolicyModel` that can derive from the side-effect-free canonical artifact model or evaluate explicit raw identity fields;
- made missing `proofpack_id`, missing `manifest_self_digest`, and invalid `manifest_self_digest` visible instead of inventing a ref;
- added `ProofpackWriterTargetArtifactRefPolicyModelBoundary` to keep filesystem, `.punk/proofs`, runtime storage, schema, CLI, proofpack writing, writer-operation evidence persistence, referenced-ref verification, index/latest writes, gate decision, service mirrors, executor claims, and acceptance side effects false;
- added smoke eval coverage for identity pair requirements, logical ref rendering, missing/invalid identity visibility, evidence-vs-authority boundaries, and no runtime side effects;
- reconciled `docs/product/CRATE-STATUS.md`.

The model remains in-memory, diagnostics-only, and evidence-only.
It does not read or write proofpacks, touch the filesystem, create `.punk/` runtime state, create schema files, add CLI behavior, verify referenced refs, write operation evidence, write indexes or `latest` pointers, write gate decisions, create acceptance claims, add adapters, add automation, add provider/model runners, or implement `punk init`.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded side-effect-free code/model implementation derived from repo-tracked proofpack writer target artifact ref policy, canonical artifact model, manifest digest, and crate-status artifacts.
No external research was needed.
Decision:
Proceed.

Research refs used:

- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md`
- `evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md`
- `evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md`
- `evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md`
- `evals/specs/proofpack-writer-file-io-boundary.v0.1.md`
- `work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-v0-1.md`
- `work/reports/2026-04-26-forty-eighth-work-ledger-review.md`

## Implementation Notes

`ProofpackWriterTargetArtifactRefPolicyModel::from_canonical_artifact_model` uses only the explicit in-memory canonical artifact model values:

- proofpack id;
- manifest self-digest;
- canonical artifact layout.

The accepted logical display ref is compact human-readable metadata only:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

It is not a target path, storage root, filename, index entry, `latest` pointer, service mirror id, CLI output, proofpack write receipt, or acceptance proof.

`ProofpackWriterTargetArtifactRefPolicyModel::evaluate_raw` keeps missing and invalid identity parts visible before a logical ref can exist.
This prevents a future writer from inventing target artifact refs from host paths, current working directory, editor state, indexes, service mirrors, or executor claims.

## Acceptance Evidence

- Stable schema version is represented by `PROOFPACK_WRITER_TARGET_ARTIFACT_REF_POLICY_MODEL_SCHEMA_VERSION`.
- Stable status vocabulary is represented by `ProofpackWriterTargetArtifactRefPolicyStatus`.
- Stable reason vocabulary is represented by `ProofpackWriterTargetArtifactRefPolicyReason`.
- `ProofpackWriterTargetArtifactIdentity` keeps `proofpack_id`, `manifest_self_digest`, and canonical artifact layout structured.
- `ProofpackWriterTargetArtifactRef` renders `proofpack:<proofpack_id>@<manifest_self_digest>` as a logical display ref.
- Missing or invalid identity parts produce rejected policy models without logical refs.
- `ProofpackWriterTargetArtifactRefPolicyModelBoundary` keeps filesystem, `.punk/proofs`, runtime storage, schema, CLI, proofpack writing, writer-operation evidence persistence, referenced-ref verification, index/latest writes or authority, service-mirror authority, executor-claim proof authority, gate decision, and acceptance side effects false.
- `punk-eval` smoke coverage includes `eval_proofpack_writer_target_artifact_ref_policy_model_is_side_effect_free`.

## Boundary

No runtime/storage/schema/CLI/`.punk` changes were made.

This change did not:

- write `.punk/proofs`;
- touch the filesystem for proofpack writes;
- canonicalize or inspect host filesystem paths;
- add schema files;
- add CLI commands;
- implement proofpack file writing;
- implement proofpack referenced-ref verification integration;
- write operation evidence;
- write indexes or `latest` pointers;
- implement gate decisions;
- create acceptance claims;
- add adapters, automation, provider/model runners, or agent integrations;
- implement `punk init`.

## Changed Files

- `crates/punk-proof/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_proofpack_writer_target_artifact_ref_policy_model_v0_1.md`
- `work/goals/goal_run_forty_ninth_work_ledger_review.md`
- `work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md`

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Side-effect-free `punk-proof` and `punk-eval` behavior changed; CRATE-STATUS and work-ledger artifacts were updated."
  touched_surfaces:
    - crates/punk-proof/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_add_proofpack_writer_target_artifact_ref_policy_model_v0_1.md
    - work/goals/goal_run_forty_ninth_work_ledger_review.md
    - work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test --workspace
```

## Checks run

- `cargo test -p punk-proof -p punk-eval` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files crates/punk-proof/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_add_proofpack_writer_target_artifact_ref_policy_model_v0_1.md work/goals/goal_run_forty_ninth_work_ledger_review.md work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md --report work/reports/2026-04-26-proofpack-writer-target-artifact-ref-policy-model-v0-1.md` - PASS
  - Warnings: two existing `docs/product/CRATE-STATUS.md` duplicate-definition candidate warnings for `Current implemented subset boundary` and `Current CLI surface`.
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS
