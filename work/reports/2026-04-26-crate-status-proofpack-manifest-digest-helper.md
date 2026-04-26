---
id: report_2026_04_26_crate_status_proofpack_manifest_digest_helper
goal_id: goal_reconcile_crate_status_proofpack_manifest_digest_helper
actor: vitaly
created_at: 2026-04-26
kind: docs-currentness
---

## Summary

Reconciled `docs/product/CRATE-STATUS.md` after proofpack manifest digest helper v0.1 became active.

CRATE-STATUS now records that `punk-proof` owns proofpack manifest self-digest metadata and provides side-effect-free manifest self-digest computation from deterministic in-memory renderer bytes through `punk-core` exact-byte hashing.

The wording keeps all larger proof surfaces deferred: referenced artifact hash computation, referenced artifact byte verification, file IO hashing, hash normalization, proofpack writer behavior, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`.

## Research Gate

Classification: R1
Required: yes
Rationale:
This reconciled canonical crate status against repo-tracked implementation evidence and the selected work goal. No external research was needed because no product-doc conflict was found.
Decision:
Proceed.

## What changed

- Updated the `punk-proof` ownership row to include manifest self-digest metadata.
- Updated current implemented subset wording for `punk-eval` to mention proofpack manifest digest helper coverage.
- Updated current implemented subset wording for `punk-proof` to mention deterministic manifest rendering and proofpack manifest self-digest computation through `punk-core` exact-byte hashing.
- Replaced stale wording that could imply `punk-proof` does not compute any proofpack hash with narrower deferred wording:
  - referenced artifact hash computation remains deferred;
  - referenced artifact byte verification remains deferred;
  - file IO hashing remains deferred;
  - hash normalization remains deferred;
  - proofpack writer behavior remains deferred.
- Updated telemetry and executor-agnostic validation sections to include manifest self-digest metadata without making it authority.

## Boundary

No runtime/code/schema/CLI/`.punk` changes were made.

No new dependency was added.

Current implemented CLI truth remains only:

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

The docs update did not:

- change Rust code;
- add dependencies;
- add schema files;
- add CLI commands;
- write `.punk/` state;
- implement proofpack writer behavior;
- add file IO hashing;
- verify referenced artifact bytes;
- normalize bytes or hashes;
- write gate decisions;
- create acceptance claims;
- add adapters, automation, provider/model runners, or `punk init`.

## Evidence

- `docs/product/CRATE-STATUS.md`
- `crates/punk-proof/src/lib.rs`
- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `evals/specs/proofpack-manifest-digest.v0.1.md`
- `work/reports/2026-04-26-proofpack-manifest-digest-helper-v0-1.md`

## Changed Files

- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_reconcile_crate_status_proofpack_manifest_digest_helper.md`
- `work/goals/goal_run_twenty_eighth_work_ledger_review.md`
- `work/reports/2026-04-26-crate-status-proofpack-manifest-digest-helper.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Canonical crate status wording was updated to match active proofpack manifest digest helper behavior without changing runtime/code/schema/CLI scope."
  touched_surfaces:
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_reconcile_crate_status_proofpack_manifest_digest_helper.md
    - work/goals/goal_run_twenty_eighth_work_ledger_review.md
    - work/reports/2026-04-26-crate-status-proofpack-manifest-digest-helper.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_reconcile_crate_status_proofpack_manifest_digest_helper.md work/goals/goal_run_twenty_eighth_work_ledger_review.md work/reports/2026-04-26-crate-status-proofpack-manifest-digest-helper.md --report work/reports/2026-04-26-crate-status-proofpack-manifest-digest-helper.md` - PASS, 0 failures, 2 warnings (`DOC_DUPLICATE_DEFINITION_CANDIDATE` for existing CRATE-STATUS sections)
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the twenty-eighth advisory Work Ledger Review.
- Keep proofpack writer behavior, file IO hashing, referenced artifact byte verification, runtime proof storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` deferred until separate bounded goals select them.
