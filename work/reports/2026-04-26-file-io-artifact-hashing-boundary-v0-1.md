---
id: report_2026_04_26_file_io_artifact_hashing_boundary_v0_1
goal_id: goal_define_file_io_artifact_hashing_boundary_v0_1
actor: vitaly
created_at: 2026-04-26
kind: docs-spec
---

## Summary

Defined file IO artifact hashing boundary v0.1 before any implementation reads files or verifies referenced artifact bytes.

The spec keeps future file IO hashing separate from existing exact-byte in-memory hashing and proofpack manifest self-digest metadata.

It records that future file IO hashing may mean:

```text
explicit repo-relative artifact ref -> policy-allowed file read -> exact file bytes -> canonical sha256 digest metadata
```

The boundary explicitly keeps referenced artifact byte verification, proofpack writer behavior, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` deferred.

## Research Gate

Classification: R1
Required: yes
Rationale:
This defined a repo-tracked docs/spec boundary using existing artifact hash and proofpack boundary artifacts before any file IO hashing implementation. No external research was needed because no runtime or security boundary conflict was found.
Decision:
Proceed.

## What changed

- Added `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`.
- Defined future helper responsibility without implementing it.
- Distinguished file IO hashing from:
  - caller-provided exact-byte hashing;
  - proofpack manifest self-digest metadata;
  - referenced artifact byte verification;
  - proofpack writer behavior;
  - runtime storage;
  - CLI behavior;
  - gate decisions;
  - acceptance claims.
- Captured boundary requirements for:
  - explicit repo roots;
  - repo-relative refs;
  - regular-file eligibility;
  - missing files;
  - non-regular files;
  - symlinks;
  - directories;
  - generated artifacts;
  - privacy and safety;
  - setup neutrality.
- Added future test/eval expectations for the eventual implementation.

## Boundary

No runtime/code/schema/CLI/`.punk` changes were made.

No new dependency was added.

Current implemented CLI truth remains only:

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

The docs/spec update did not:

- change Rust code;
- add dependencies;
- add schema files;
- add CLI commands;
- write `.punk/` state;
- implement file IO hashing;
- verify referenced artifact bytes;
- implement proofpack writer behavior;
- normalize bytes or hashes;
- write gate decisions;
- create acceptance claims;
- add adapters, automation, provider/model runners, or `punk init`.

## Evidence

- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `docs/product/CRATE-STATUS.md`
- `work/reports/2026-04-26-twenty-eighth-work-ledger-review.md`

## Changed Files

- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_file_io_artifact_hashing_boundary_v0_1.md`
- `work/goals/goal_run_twenty_ninth_work_ledger_review.md`
- `work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Added a docs/spec boundary for future file IO artifact hashing without changing runtime/code/schema/CLI behavior."
  touched_surfaces:
    - evals/specs/file-io-artifact-hashing-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_file_io_artifact_hashing_boundary_v0_1.md
    - work/goals/goal_run_twenty_ninth_work_ledger_review.md
    - work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/file-io-artifact-hashing-boundary.v0.1.md work/STATUS.md work/goals/goal_define_file_io_artifact_hashing_boundary_v0_1.md work/goals/goal_run_twenty_ninth_work_ledger_review.md work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md --report work/reports/2026-04-26-file-io-artifact-hashing-boundary-v0-1.md` - PASS (0 failures, 0 warnings)
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS (no repo-tracked absolute path leaks found)

## Open follow-ups

- Run the twenty-ninth advisory Work Ledger Review.
- Consider file IO hash helper implementation only after the review selects it explicitly.
- Keep referenced artifact byte verification, proofpack writer behavior, runtime proof storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` deferred until separate bounded goals select them.
