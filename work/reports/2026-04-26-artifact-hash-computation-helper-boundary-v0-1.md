---
id: report_2026_04_26_artifact_hash_computation_helper_boundary_v0_1
goal_id: goal_define_artifact_hash_computation_helper_boundary_v0_1
actor: vitaly
created_at: 2026-04-26
kind: specification
---

## Summary

Defined artifact hash computation helper boundary v0.1 as a docs/spec artifact.

The boundary keeps the future helper narrow: exact caller-provided bytes in, canonical `sha256:<64 lowercase hex>` digest metadata out.

It explicitly keeps path/ref validation, file IO, byte normalization, proofpack writing, runtime storage, schemas, CLI behavior, gate decisions, adapters, automation, and `punk init` out of scope.

It also records dependency stance: Rust stdlib has no SHA-256, Punk should not hand-roll crypto, and a future implementation may add one narrow maintained SHA-256 dependency in `punk-core` only if a separate bounded implementation goal selects it.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded docs/spec decision for a future active-core helper and dependency stance. Existing Punk docs, specs, work reports, and current crate manifests were sufficient; no Deep Research was needed because no implementation dependency was added in this patch.
Decision:
Proceed.

Research refs:

- `docs/product/ARCHITECTURE.md`
- `docs/product/CRATE-STATUS.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `work/STATUS.md`
- `work/reports/2026-04-26-twenty-second-work-ledger-review.md`

## Changed Files

- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_artifact_hash_computation_helper_boundary_v0_1.md`
- `work/goals/goal_run_twenty_third_work_ledger_review.md`
- `work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md`

## Specification

- Defined helper responsibility as `exact bytes -> canonical sha256 digest string`.
- Recommended future Rust API shape: `compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest`.
- Required output to reuse canonical artifact digest validation after formatting.
- Kept path/ref validation separate from byte hashing.
- Kept future file hashing helper behavior separate because it introduces filesystem IO, privacy, symlink, path, and missing-file behavior.
- Recorded that a future caller may hash exact UTF-8 bytes from `Proofpack::render_manifest_json()`, but the hash helper must not know proofpack semantics.
- Added test/eval expectations for empty bytes, `abc`, canonical lowercase output, exact-byte preservation, side-effect freedom, and executor-claim separation.

## Dependency stance

- Rust stdlib does not provide SHA-256.
- Punk should not hand-roll cryptographic SHA-256.
- A future implementation may add one small maintained SHA-256 dependency, preferably RustCrypto-family such as `sha2`, scoped to `punk-core` only.
- No dependency was added in this patch.
- If the future implementation cannot keep that dependency narrow, it should stop and select a new decision/ADR goal.

## Boundary

No runtime/code/schema/CLI/`.punk` changes were made.

This patch did not:

- implement hash computation;
- add dependencies;
- add file IO;
- normalize bytes or hashes;
- write gate decisions;
- write proofpacks;
- write runtime storage;
- add schema files;
- add CLI commands;
- add adapters, automation, provider/model runners, or agent integrations;
- implement `punk init`.

The current implemented CLI surface remains:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Added a docs/spec boundary for future artifact hash computation helper behavior and dependency stance."
  touched_surfaces:
    - evals/specs/artifact-hash-computation-helper.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_artifact_hash_computation_helper_boundary_v0_1.md
    - work/goals/goal_run_twenty_third_work_ledger_review.md
    - work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md
  required_updates:
    - evals/specs/artifact-hash-computation-helper.v0.1.md
    - work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/artifact-hash-computation-helper.v0.1.md work/STATUS.md work/goals/goal_define_artifact_hash_computation_helper_boundary_v0_1.md work/goals/goal_run_twenty_third_work_ledger_review.md work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md --report work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md` - PASS, 0 failures, 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the twenty-third advisory Work Ledger Review.
- Decide whether to implement `punk-core` exact-byte hash computation helper next.
- If implementation is selected, keep it side-effect-free and consider a narrow `sha2` dependency only in that bounded goal.
- Keep file IO hashing, proofpack writer, runtime storage, schemas, CLI behavior, gate/eval/proof orchestration, adapters, automation, and `punk init` deferred.

## Validation Notes

An initial docs-governance run failed because `eval-spec` is not an allowed `DocImpact` classification. The report and goal were corrected to `docs-only`, then docs-governance was rerun and passed with 0 failures and 0 warnings.
