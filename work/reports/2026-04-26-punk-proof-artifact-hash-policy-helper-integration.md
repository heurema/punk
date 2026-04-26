---
id: report_2026_04_26_punk_proof_artifact_hash_policy_helper_integration
goal_id: goal_integrate_artifact_hash_policy_helpers_into_punk_proof_v0_1
actor: vitaly
created_at: 2026-04-26
kind: implementation
---

## Summary

Integrated artifact hash policy helper validation into the side-effect-free `punk-proof` kernel.

Proof artifact hashes now validate through the existing `punk-core` artifact hash policy helpers instead of accepting any non-empty string.

This remains structural proof metadata only. The change does not compute hashes, normalize artifact bytes, write proofpacks, write gate decisions, write acceptance claims, write runtime storage, add schema files, add CLI behavior, add adapters, add automation, or implement `punk init`.

## Research Gate

Classification: R0
Required: no
Rationale:
This integrates already-defined and smoke-covered side-effect-free `punk-core` artifact hash policy helpers into the existing side-effect-free proof kernel.
Decision:
Proceed.

## Changed Files

- `Cargo.lock`
- `crates/punk-proof/Cargo.toml`
- `crates/punk-proof/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/goals/goal_integrate_artifact_hash_policy_helpers_into_punk_proof_v0_1.md`
- `work/goals/goal_run_twentieth_work_ledger_review.md`
- `work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md`

## Implementation

- Added `punk-core` as a `punk-proof` dependency.
- Updated `ProofArtifactHash::new` to validate through `validate_artifact_digest` after preserving the existing empty/whitespace error path.
- Added `ProofpackError::InvalidArtifactHash(ArtifactHashPolicyError)` for non-empty non-canonical digest failures.
- Replaced proof and eval placeholder digest fixtures with deterministic canonical-shaped static strings.
- Added proof tests covering canonical digest acceptance and rejection of placeholder, bare, uppercase, short, and unsupported-algorithm digest strings.

## Boundary

Unchanged active-core boundary:

- proofpack link/hash integrity remains structural;
- no hash computation;
- no byte normalization;
- no proofpack writer;
- no gate decision writer;
- no acceptance claim writer;
- no runtime storage;
- no schema files;
- no CLI behavior;
- no provider/model/agent adapters;
- no automation;
- no `punk init`.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Proof kernel validation behavior changed and a work report records the bounded implementation; canonical product docs/specs did not change."
  touched_surfaces:
    - Cargo.lock
    - crates/punk-proof/Cargo.toml
    - crates/punk-proof/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_integrate_artifact_hash_policy_helpers_into_punk_proof_v0_1.md
    - work/goals/goal_run_twentieth_work_ledger_review.md
    - work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `cargo fmt --all` - PASS
- `cargo test -p punk-proof` - PASS
- `cargo test -p punk-eval` - PASS
- `cargo run -p punk-cli -- eval run smoke` - PASS
- `cargo run -p punk-cli -- eval run smoke --format json` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files Cargo.lock crates/punk-proof/Cargo.toml crates/punk-proof/src/lib.rs crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_integrate_artifact_hash_policy_helpers_into_punk_proof_v0_1.md work/goals/goal_run_twentieth_work_ledger_review.md work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md --report work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md` - PASS, 0 failures, 0 warnings
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found
