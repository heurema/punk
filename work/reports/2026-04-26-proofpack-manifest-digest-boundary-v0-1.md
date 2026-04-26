---
id: report_2026_04_26_proofpack_manifest_digest_boundary_v0_1
goal_id: goal_define_proofpack_manifest_digest_boundary_v0_1
actor: vitaly
created_at: 2026-04-26
kind: specification
---

## Summary

Defined proofpack manifest digest boundary v0.1 as a docs/spec artifact.

The boundary states that a future helper may hash the exact UTF-8 bytes of the deterministic in-memory proofpack manifest rendering and return canonical `sha256:<64 lowercase hex>` digest metadata.

The boundary keeps proofpack manifest self-digest separate from referenced artifact digests, file IO artifact hashing, proofpack writer behavior, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded docs/spec decision for future proofpack manifest digest behavior, based on existing Punk docs, specs, and code artifacts. No Deep Research was needed because no architecture conflict appeared.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `evals/specs/proofpack-boundary.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `crates/punk-proof/src/lib.rs`
- `crates/punk-core/src/lib.rs`
- `work/STATUS.md`
- `work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md`
- `work/reports/2026-04-26-twenty-fifth-work-ledger-review.md`

## Changed Files

- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_proofpack_manifest_digest_boundary_v0_1.md`
- `work/goals/goal_run_twenty_sixth_work_ledger_review.md`
- `work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md`

## What changed

- Added `evals/specs/proofpack-manifest-digest.v0.1.md`.
- Defined manifest digest bytes as exactly `render_manifest_json().as_bytes()` in current implementation terms.
- Required the future helper to reuse `punk-core` exact-byte hashing rather than adding a new SHA-256 dependency to `punk-proof`.
- Distinguished manifest self-digest from referenced artifact digest entries and file IO artifact hashing.
- Explicitly rejected recursive self-inclusion for v0.1.
- Recorded capability wording for future implementation so manifest digest computation does not imply file IO hashing, hash normalization, proofpack writing, runtime storage, CLI behavior, or acceptance authority.
- Added required future tests/evals for deterministic digest behavior, exact renderer bytes, canonical digest format, no formatting normalization, side-effect-free behavior, no recursive self-inclusion, and no new `punk-proof` hash dependency.
- Added the twenty-sixth advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/code/schema/CLI/`.punk` authority changed.

This did not:

- change Rust code;
- add dependencies;
- compute hashes in code;
- add file IO hashing;
- verify referenced artifact bytes;
- normalize bytes or hashes;
- write gate decisions;
- write proofpacks;
- write runtime storage;
- add schema files;
- add CLI commands;
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
  reason: "Added a new eval/spec boundary for future proofpack manifest digest behavior without promoting implementation, writer, runtime, file-IO-hash, schema, or CLI scope."
  touched_surfaces:
    - evals/specs/proofpack-manifest-digest.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_proofpack_manifest_digest_boundary_v0_1.md
    - work/goals/goal_run_twenty_sixth_work_ledger_review.md
    - work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md
  required_updates:
    - evals/specs/proofpack-manifest-digest.v0.1.md
    - work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files evals/specs/proofpack-manifest-digest.v0.1.md work/STATUS.md work/goals/goal_define_proofpack_manifest_digest_boundary_v0_1.md work/goals/goal_run_twenty_sixth_work_ledger_review.md work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md --report work/reports/2026-04-26-proofpack-manifest-digest-boundary-v0-1.md` - PASS, 0 failures, 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the twenty-sixth advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack manifest digest helper implementation, proofpack writer preparation, file IO hash boundary, gate/eval/proof orchestration, runtime storage, schema work, or another active-core guardrail.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
