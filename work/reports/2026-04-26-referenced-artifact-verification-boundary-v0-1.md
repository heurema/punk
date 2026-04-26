---
id: report_2026_04_26_referenced_artifact_verification_boundary_v0_1
goal_id: goal_define_referenced_artifact_verification_boundary_v0_1
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Defined referenced artifact verification boundary v0.1 as a docs/spec artifact.

The new boundary specifies what future referenced artifact verification means for proofpack refs and declared artifact hashes:

- explicit artifact ref, expected digest, and repo root inputs;
- repo-relative ref and explicit root constraints;
- regular-file-only eligibility for v0.1;
- symlink refusal;
- exact-byte digest comparison semantics;
- outcome vocabulary for `verified`, `digest_mismatch`, `missing`, `not_regular_file`, `symlink`, `unreadable`, `invalid_ref`, `invalid_expected_digest`, `unsupported_ref`, and `not_required`;
- evidence record boundaries;
- required vs optional artifact separation;
- privacy and sensitivity boundaries;
- future test/eval cases.

This was a docs/spec-only boundary. It does not implement referenced artifact verification, proofpack writer behavior, proofpack writer hash-policy integration, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init`.

## Research Gate

Classification: R1
Required: yes
Rationale:
This was a docs/spec boundary derived from repo-tracked Punk proof/hash/file-IO artifacts. No external research or architecture change was required.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-proof/src/lib.rs`
- `work/STATUS.md`
- `work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md`
- `work/reports/2026-04-26-thirty-first-work-ledger-review.md`

## Changed Files

- `evals/specs/referenced-artifact-verification-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_referenced_artifact_verification_boundary_v0_1.md`
- `work/goals/goal_run_thirty_second_work_ledger_review.md`
- `work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md`

## What changed

- Added `evals/specs/referenced-artifact-verification-boundary.v0.1.md`.
- Defined verification as comparing a declared proofpack artifact ref/hash against exact bytes of one explicit eligible file under one explicit repo root.
- Distinguished verification from file IO digest computation, proofpack manifest self-digest, structural proofpack link/hash integrity, proofpack writer behavior, gate decisions, and acceptance claims.
- Recorded non-passing outcomes for mismatch, missing, unreadable, invalid ref/digest, symlink, directory, special file, and unsupported ref cases.
- Added the thirty-second advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/code/schema/CLI authority changed.

This did not:

- change Rust code;
- add dependencies;
- add CLI behavior;
- write `.punk/` state;
- implement proofpack writer behavior;
- implement referenced artifact byte verification;
- implement referenced artifact hash computation for proofpack refs;
- broaden file IO hashing beyond the narrow helper;
- normalize bytes or hashes;
- implement gate decision writer behavior;
- add schema files;
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
  classification: docs-only
  reason: "Added a docs/spec boundary for future referenced artifact verification without implementing runtime/writer/CLI behavior."
  touched_surfaces:
    - evals/specs/referenced-artifact-verification-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_referenced_artifact_verification_boundary_v0_1.md
    - work/goals/goal_run_thirty_second_work_ledger_review.md
    - work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md
  required_updates:
    - evals/specs/referenced-artifact-verification-boundary.v0.1.md
    - work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/referenced-artifact-verification-boundary.v0.1.md work/STATUS.md work/goals/goal_define_referenced_artifact_verification_boundary_v0_1.md work/goals/goal_run_thirty_second_work_ledger_review.md work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md --report work/reports/2026-04-26-referenced-artifact-verification-boundary-v0-1.md` - PASS with 0 failures and 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the thirty-second advisory Work Ledger Review.
- Decide whether the next branch should be referenced artifact verification implementation, smoke eval coverage for the new boundary, proofpack writer preparation, proofpack writer hash-policy integration, another docs/spec guardrail, or another active-core setup step.
- Keep `.punk/` storage, runtime gate/proof behavior, schemas, CLI expansion, adapters, automation, provider/model runners, and `punk init` deferred until separately selected.
