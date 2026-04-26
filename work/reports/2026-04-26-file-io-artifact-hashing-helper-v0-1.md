---
id: report_2026_04_26_file_io_artifact_hashing_helper_v0_1
goal_id: goal_add_file_io_artifact_hashing_helper_v0_1
actor: vitaly
created_at: 2026-04-26
kind: implementation
---

## Summary

Added file IO artifact hashing helper v0.1 to `punk-core`.

`compute_artifact_file_digest(repo_root, artifact_ref)` now computes canonical `sha256:<64 lowercase hex>` digest metadata from one explicit repo-relative artifact ref under one explicit repo root.

The helper reads only one regular file, then delegates digest computation to the existing exact-byte hash helper.

It is evidence only.
It does not verify referenced artifact bytes, write proofpacks, write runtime state, expose CLI behavior, write gate decisions, or create acceptance claims.

Smoke eval coverage now includes file IO artifact hashing helper behavior as local assessment only.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded implementation of the accepted repo-tracked file IO artifact hashing boundary. It uses existing `punk-core` exact-byte hashing and Rust stdlib file metadata/read APIs. No Deep Research was needed because no path, symlink, dependency, or platform boundary conflict was found.
Decision:
Proceed.

Research refs:

- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `docs/product/CRATE-STATUS.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/reports/2026-04-26-twenty-ninth-work-ledger-review.md`

## Changed Files

- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/goals/goal_add_file_io_artifact_hashing_helper_v0_1.md`
- `work/goals/goal_run_thirtieth_work_ledger_review.md`
- `work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md`

## Implementation

- Added `RepoRoot` as an explicit absolute repo-root wrapper.
- Added `FileArtifactHashError` with explicit non-passing outcomes for:
  - empty repo root;
  - relative repo root;
  - outside repo root;
  - missing file;
  - non-regular file;
  - symlink not supported;
  - read denied;
  - read error.
- Added `FileArtifactHashingCapabilities` and `FILE_ARTIFACT_HASHING_CAPABILITIES` to distinguish file artifact digest computation from:
  - referenced artifact byte verification;
  - byte normalization;
  - runtime storage;
  - proofpack writing;
  - CLI output;
  - acceptance claims.
- Added `compute_artifact_file_digest(&RepoRoot, &RepoRelativeArtifactRef) -> Result<ArtifactDigest, FileArtifactHashError>`.
- The helper:
  - requires an explicit absolute repo root;
  - requires an already validated repo-relative artifact ref;
  - rejects symlinks through `symlink_metadata` before reading;
  - rejects directories and other non-regular files;
  - reads exact file bytes only;
  - delegates digest computation to `compute_artifact_digest(bytes)`.
- Added `punk-core` tests for:
  - regular file exact-byte hashing;
  - newline/whitespace non-normalization;
  - missing files;
  - directories;
  - explicit absolute repo root requirement;
  - invalid ref validation boundary;
  - non-authoritative capability flags;
  - symlink rejection on Unix.
- Added `punk-eval` smoke coverage for:
  - explicit repo-root plus repo-relative file hashing;
  - missing file status;
  - directory status;
  - invalid ref and relative-root rejection;
  - non-authoritative capability flags.

## Boundary

No runtime storage/schema/CLI/`.punk` behavior was added.

This patch did not:

- add proofpack writer behavior;
- verify referenced artifact bytes;
- mark proofpack refs verified;
- scan directories;
- follow symlinks silently;
- normalize bytes or hashes;
- write `.punk/` state;
- write runtime storage;
- add schema files;
- add CLI commands;
- write gate decisions;
- create acceptance claims;
- add dependencies;
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
  classification: code-doc
  reason: "Added active-core file IO artifact hashing helper behavior in `punk-core` and smoke eval coverage; work report records the bounded implementation."
  touched_surfaces:
    - crates/punk-core/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_add_file_io_artifact_hashing_helper_v0_1.md
    - work/goals/goal_run_thirtieth_work_ledger_review.md
    - work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md
  required_updates:
    - work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `cargo fmt --all` - PASS
- `cargo test -p punk-core` - PASS
- `cargo test -p punk-eval` - PASS
- `cargo run -p punk-cli -- eval run smoke` - PASS
- `cargo run -p punk-cli -- eval run smoke --format json` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files crates/punk-core/src/lib.rs crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_file_io_artifact_hashing_helper_v0_1.md work/goals/goal_run_thirtieth_work_ledger_review.md work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md --report work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md` - PASS (0 failures, 0 warnings)
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS (no repo-tracked absolute path leaks found)

## Open follow-ups

- Run the thirtieth advisory Work Ledger Review.
- Reconcile `docs/product/CRATE-STATUS.md` current implemented subset wording for active file IO artifact hashing helper, unless the review selects a different higher-priority guardrail.
- Keep referenced artifact byte verification, proofpack writer behavior, proofpack writer hash-policy integration, runtime proof storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` deferred until separate bounded goals select them.
