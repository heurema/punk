---
id: report_2026_04_26_artifact_hash_computation_helper_v0_1
goal_id: goal_add_artifact_hash_computation_helper_v0_1
actor: vitaly
created_at: 2026-04-26
kind: implementation
---

## Summary

Added artifact hash computation helper v0.1 to `punk-core`.

`compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest` now computes a canonical `sha256:<64 lowercase hex>` digest from exact caller-provided bytes.

The helper is side-effect-free. It does not read files, map refs or runtime ids, normalize bytes, write `.punk/` state, write proofpacks, write gate decisions, add schema files, add CLI behavior, create acceptance claims, add adapters, add automation, or implement `punk init`.

Smoke eval coverage now includes exact-byte hash computation known vectors and exact-byte preservation.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded implementation of the accepted repo-tracked helper boundary. It adds one narrow maintained SHA-256 dependency to `punk-core` and keeps implementation side-effect-free. No Deep Research was needed because dependency scope stayed within the accepted boundary.
Decision:
Proceed.

Research refs:

- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/reports/2026-04-26-artifact-hash-computation-helper-boundary-v0-1.md`
- `work/reports/2026-04-26-twenty-third-work-ledger-review.md`

## Changed Files

- `Cargo.lock`
- `crates/punk-core/Cargo.toml`
- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/goals/goal_add_artifact_hash_computation_helper_v0_1.md`
- `work/goals/goal_run_twenty_fourth_work_ledger_review.md`
- `work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md`

## Implementation

- Added `sha2 = { version = "0.10", default-features = false }` to `punk-core`.
- Added `compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest`.
- Formatted computed bytes as lowercase `sha256:<64 hex>` without exposing `sha2` types in the public Punk API.
- Updated `ARTIFACT_HASH_POLICY_CAPABILITIES.computes_hashes` to `true` while keeping:
  - `normalizes_artifact_bytes = false`;
  - `writes_runtime_state = false`.
- Added `punk-core` tests for:
  - empty bytes SHA-256 vector;
  - `abc` SHA-256 vector;
  - canonical digest validation;
  - exact-byte preservation across newline/whitespace differences;
  - capability flags.
- Added `punk-eval` smoke cases for:
  - known SHA-256 vectors;
  - exact-byte preservation without normalization or runtime writes.
- Updated smoke report wording, boundary notes, deferred notes, human output assertions, and JSON output assertions.

## Dependency stance

The added dependency is narrow and owned by `punk-core` only:

- no async runtime;
- no networking;
- no provider/model/agent adapter;
- no OS service;
- no CLI dependency;
- no storage dependency;
- no dependency types exposed through public Punk APIs.

Rust stdlib has no SHA-256 and Punk should not hand-roll cryptographic SHA-256.

## Boundary

No runtime/schema/CLI/`.punk` behavior was added.

This patch did not:

- add file IO hashing;
- read artifact paths;
- map runtime ids;
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
  classification: code-doc
  reason: "Added active-core exact-byte hash computation helper behavior in `punk-core` and smoke eval coverage; work report records the bounded implementation."
  touched_surfaces:
    - Cargo.lock
    - crates/punk-core/Cargo.toml
    - crates/punk-core/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_add_artifact_hash_computation_helper_v0_1.md
    - work/goals/goal_run_twenty_fourth_work_ledger_review.md
    - work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md
  required_updates:
    - work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md
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
- `scripts/check.sh docs-governance --files Cargo.lock crates/punk-core/Cargo.toml crates/punk-core/src/lib.rs crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_artifact_hash_computation_helper_v0_1.md work/goals/goal_run_twenty_fourth_work_ledger_review.md work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md --report work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md` - PASS, 0 failures, 0 warnings
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the twenty-fourth advisory Work Ledger Review.
- Reconcile `docs/product/CRATE-STATUS.md` current implemented subset wording for active exact-byte hash computation, unless the review selects a different higher-priority guardrail.
- Keep file IO hashing, proofpack writer, runtime storage, schemas, CLI behavior, gate/eval/proof orchestration, adapters, automation, and `punk init` deferred.
