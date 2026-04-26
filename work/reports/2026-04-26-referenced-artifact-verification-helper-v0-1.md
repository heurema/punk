---
id: report_2026_04_26_referenced_artifact_verification_helper_v0_1
goal_id: goal_add_referenced_artifact_verification_helper_v0_1
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Added referenced artifact verification helper v0.1 as a bounded active-core helper.

`punk-core` now exposes:

- `ReferencedArtifactVerificationOutcome`;
- `ReferencedArtifactVerification`;
- `ReferencedArtifactVerificationCapabilities`;
- `REFERENCED_ARTIFACT_VERIFICATION_CAPABILITIES`;
- `verify_referenced_artifact_digest(&RepoRoot, &RepoRelativeArtifactRef, &ArtifactDigest)`.

The helper compares a caller-provided expected canonical digest with the observed digest for one explicit regular file under one explicit repo root and validated repo-relative artifact ref.

The helper returns evidence-only outcomes, including `verified`, `digest_mismatch`, `missing`, `not_regular_file`, `symlink`, `read_denied`, `read_error`, `invalid_repo_root`, and `outside_repo_root`.

`punk-eval` smoke coverage now includes referenced artifact verification helper behavior as local assessment only.

This implementation does not add proofpack writer behavior, proofpack writer hash-policy integration, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init`.

## Research Gate

Classification: R1
Required: yes
Rationale:
This was a bounded implementation derived from repo-tracked hash/file-IO/verification boundaries and current `punk-core` helpers. No external research or architecture change was required.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `evals/specs/referenced-artifact-verification-boundary.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md`
- `work/reports/2026-04-26-thirty-second-work-ledger-review.md`

## Changed Files

- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/goals/goal_add_referenced_artifact_verification_helper_v0_1.md`
- `work/goals/goal_run_thirty_third_work_ledger_review.md`
- `work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md`

## What changed

- Added structured referenced artifact verification evidence types and a helper in `punk-core`.
- Reused existing exact-byte file digest computation rather than adding new file IO behavior.
- Kept invalid raw refs and invalid raw digests explicit through existing typed constructors and validators.
- Added unit tests for verified, digest mismatch, missing, non-regular/directory, symlink, invalid ref/digest, and evidence-only capability boundaries.
- Added a smoke eval case for referenced artifact verification helper behavior.
- Updated smoke eval summary, boundary, deferred notes, and test expectations to include the new helper while preserving local-assessment wording.
- Added the thirty-third advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/schema/CLI/`.punk` authority changed.

This did not:

- add CLI behavior;
- write `.punk/` state;
- add schema files;
- implement proofpack writer behavior;
- implement proofpack writer hash-policy integration;
- normalize bytes or hashes;
- implement gate decision writer behavior;
- add acceptance claims;
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
  reason: "Added bounded active-core helper code, smoke eval coverage, work status, goal outcome, next review goal, and implementation report."
  touched_surfaces:
    - crates/punk-core/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_add_referenced_artifact_verification_helper_v0_1.md
    - work/goals/goal_run_thirty_third_work_ledger_review.md
    - work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md
  required_updates:
    - work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test --workspace
```

## Checks run

- `cargo fmt --all` - PASS
- `cargo check --workspace` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files crates/punk-core/src/lib.rs crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_referenced_artifact_verification_helper_v0_1.md work/goals/goal_run_thirty_third_work_ledger_review.md work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md --report work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md` - PASS with 0 failures and 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

Validation note:
- First docs-governance run failed because the report used invalid `doc_impact.classification: code-and-docs`; corrected to the allowed `code-doc` value and reran successfully.

## Open follow-ups

- Run the thirty-third advisory Work Ledger Review.
- Reconcile `docs/product/CRATE-STATUS.md` if the review confirms it is stale for referenced artifact verification helper behavior.
- Decide whether the next branch should be additional smoke eval/docs guardrails, proofpack writer preparation, proofpack writer hash-policy integration, another docs/spec guardrail, or another active-core setup step.
- Keep `.punk/` storage, runtime gate/proof behavior, schemas, CLI expansion, adapters, automation, provider/model runners, and `punk init` deferred until separately selected.
