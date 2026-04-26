---
id: report_2026_04_25_artifact_hash_policy_helpers_v0_1
goal_id: goal_add_artifact_hash_policy_helpers_v0_1
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Add side-effect-free artifact hash policy helpers v0.1 in `punk-core` for digest format validation and repo-relative artifact ref validation.

## Research Gate

Classification: R0
Required: no
Rationale:
This is a narrow implementation of the already-defined artifact hash policy v0.1. No external research or architecture change was required.

Research refs:

- `evals/specs/artifact-hash-policy.v0.1.md`
- `docs/product/CRATE-STATUS.md`
- `work/reports/2026-04-25-artifact-hash-policy-v0-1.md`
- `work/reports/2026-04-25-seventeenth-work-ledger-review.md`

Decision:
Proceed with side-effect-free validation helpers only.

## Changed files

- `crates/punk-core/src/lib.rs`
- `work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md`
- `work/goals/goal_run_eighteenth_work_ledger_review.md`
- `work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md`
- `work/STATUS.md`

## What changed

- Added artifact hash policy constants:
  - `ARTIFACT_HASH_POLICY_VERSION`;
  - `CANONICAL_SHA256_DIGEST_PREFIX`;
  - `CANONICAL_SHA256_DIGEST_HEX_LEN`.
- Added `ArtifactDigest` and `RepoRelativeArtifactRef` wrappers.
- Added `ArtifactHashPolicyError` variants for digest/ref validation failures.
- Added `validate_artifact_digest` and `is_canonical_artifact_digest`.
- Added `validate_repo_relative_artifact_ref` and `is_valid_repo_relative_artifact_ref`.
- Added `ARTIFACT_HASH_POLICY_CAPABILITIES` to make non-runtime boundaries explicit.
- Added unit coverage for canonical digests, unsupported/bare algorithms, uppercase hex, invalid lengths, placeholders, valid repo-relative refs, invalid absolute/home/URL/backslash refs, invalid empty/dot/parent segments, and capability boundaries.
- Added the eighteenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

This was a side-effect-free helper implementation.

It did not:

- compute hashes;
- normalize artifact bytes;
- add dependencies;
- add schema files;
- change `punk-proof`;
- add CLI behavior;
- write `.punk/` state;
- implement proofpack writer behavior;
- implement gate decision writer behavior;
- add provider/model/agent adapters;
- add automation;
- implement `punk init`;
- claim acceptance.

The current implemented CLI surface remains:

```text
punk flow inspect
punk eval run smoke
punk eval run smoke --format json
```

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Implemented side-effect-free artifact hash policy helpers in `punk-core` and recorded the work report; no product docs or specs changed."
  touched_surfaces:
    - crates/punk-core/src/lib.rs
    - work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md
    - work/goals/goal_run_eighteenth_work_ledger_review.md
    - work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md
    - work/STATUS.md
  required_updates:
    - work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `cargo fmt --all` - PASS.
- `cargo test -p punk-core` - PASS.
- `git diff --check` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-core/src/lib.rs work/STATUS.md work/goals/goal_add_artifact_hash_policy_helpers_v0_1.md work/goals/goal_run_eighteenth_work_ledger_review.md work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md --report work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found.

## Open follow-ups

- Run the eighteenth advisory Work Ledger Review.
- Decide whether the next branch should integrate hash policy helpers into `punk-proof`, add smoke eval coverage for AHP cases, or keep runtime writer/storage work deferred.
- Keep proofpack writer, active hash computation, `.punk/` storage, schema files, gate/eval/proof orchestration, adapters, automation, and `punk init` deferred until separately selected.
