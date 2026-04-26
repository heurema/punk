---
id: report_2026_04_26_proofpack_manifest_digest_helper_v0_1
goal_id: goal_add_proofpack_manifest_digest_helper_v0_1
actor: vitaly
created_at: 2026-04-26
kind: implementation
---

## Summary

Added the first proofpack manifest digest helper as a side-effect-free active-core primitive.

`punk-proof` now exposes `compute_proofpack_manifest_digest(&Proofpack)`, which hashes exactly the UTF-8 bytes returned by `Proofpack::render_manifest_json()` through `punk-core` exact-byte digest computation.

The helper returns canonical `sha256:<64 lowercase hex>` metadata. It does not write proofpacks, write `.punk/` state, read files, verify referenced artifact bytes, normalize bytes or hashes, write gate decisions, create acceptance claims, add schema files, add CLI behavior, add adapters, add automation, add provider/model runners, or implement `punk init`.

Smoke eval coverage now records the helper as local assessment only.

## Research Gate

Classification: R1
Required: yes
Rationale:
This implemented the accepted repo-tracked proofpack manifest digest boundary using existing `punk-core` exact-byte hashing. No external research was needed because no boundary or dependency conflict was found.
Decision:
Proceed.

## What changed

- Added `compute_proofpack_manifest_digest(&Proofpack) -> ArtifactDigest` in `punk-proof`.
- Reused `punk-core::compute_artifact_digest` rather than adding any hash dependency to `punk-proof`.
- Added narrow proofpack boundary flags:
  - `computes_manifest_digest = true`
  - `computes_referenced_artifact_hashes = false`
- Kept broad proofpack hash boundaries deferred:
  - `computes_hashes = false`
  - `normalizes_hashes = false`
- Added `punk-proof` unit tests for:
  - deterministic repeated manifest digest calls;
  - equivalence to hashing `render_manifest_json().as_bytes()`;
  - exact renderer byte identity, including trailing-newline sensitivity;
  - no recursive self-inclusion;
  - no referenced-artifact integrity side effect;
  - no writer/runtime/CLI/acceptance side effects.
- Added a smoke eval case for manifest digest helper behavior as local assessment only.
- Updated smoke eval summary, boundary notes, deferred notes, human output assertions, JSON output assertions, and expected case count.

## Boundary

No runtime/schema/CLI/`.punk` changes were made.

No new dependency was added to `punk-proof`.

Current implemented CLI truth remains only:

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

The implementation did not:

- write proofpacks;
- write `.punk/proofs` or any `.punk/` state;
- add file IO hashing;
- verify referenced artifact bytes;
- normalize bytes or hashes;
- add schema files;
- add CLI commands;
- write gate decisions;
- create acceptance claims;
- add adapters, automation, provider/model runners, or `punk init`.

## Evidence

- `crates/punk-proof/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`

## Changed Files

- `crates/punk-proof/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/goals/goal_add_proofpack_manifest_digest_helper_v0_1.md`
- `work/goals/goal_run_twenty_seventh_work_ledger_review.md`
- `work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md`

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Implementation updated proof/eval code and work-ledger artifacts only. Canonical product docs remain unchanged in this slice."
  touched_surfaces:
    - crates/punk-proof/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_add_proofpack_manifest_digest_helper_v0_1.md
    - work/goals/goal_run_twenty_seventh_work_ledger_review.md
    - work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required:
    - punk eval run smoke
    - punk eval run smoke --format json
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
- `scripts/check.sh docs-governance --files crates/punk-proof/src/lib.rs crates/punk-eval/src/lib.rs work/STATUS.md work/goals/goal_add_proofpack_manifest_digest_helper_v0_1.md work/goals/goal_run_twenty_seventh_work_ledger_review.md work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md --report work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md` - PASS, 0 failures, 0 warnings
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the twenty-seventh advisory Work Ledger Review.
- Reconcile `docs/product/CRATE-STATUS.md` if the review selects a docs-currentness goal, because `punk-proof` now computes manifest self-digests while still not computing referenced artifact hashes or file IO hashes.
- Keep proofpack writer behavior, file IO hashing, runtime proof storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` deferred until separate bounded goals select them.
