---
id: report_2026_04_26_crate_status_exact_byte_hash_computation
goal_id: goal_reconcile_crate_status_exact_byte_hash_computation
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Reconciled `docs/product/CRATE-STATUS.md` with the current exact-byte artifact hash computation implementation state.

The canonical crate-status doc now records that:

- `punk-core` provides side-effect-free exact-byte artifact hash computation plus artifact digest and repo-relative artifact ref validation helpers;
- `punk-core` owns a narrow `sha2` dependency for SHA-256 computation without exposing dependency types in public Punk APIs;
- `punk-eval` smoke coverage includes exact-byte artifact hash computation helper behavior;
- `punk-proof` still does not own proofpack/file IO hash computation, byte/hash normalization, runtime storage, CLI behavior, or acceptance claims.

This was a docs-currentness update only. It does not activate file IO hashing, proofpack writer behavior, byte/hash normalization, `.punk/` storage, schema files, gate acceptance, adapters, automation, extra CLI commands, provider/model runners, or `punk init`.

## Research Gate

Classification: R0
Required: no
Rationale:
This was a docs-currentness update against repo-tracked implementation state and canonical docs. No external research or architecture change was required.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `work/STATUS.md`
- `work/reports/2026-04-26-artifact-hash-computation-helper-v0-1.md`
- `work/reports/2026-04-26-twenty-fourth-work-ledger-review.md`

## Changed Files

- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md`
- `work/goals/goal_run_twenty_fifth_work_ledger_review.md`
- `work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md`

## What changed

- Updated `docs/product/CRATE-STATUS.md` to stop saying `punk-core` is dependency-free and does not compute hashes.
- Recorded that exact-byte artifact hash computation is active only as a side-effect-free helper over caller-provided bytes.
- Recorded that `sha2` is a narrow `punk-core` dependency and is not exposed through public Punk APIs.
- Recorded that `punk-eval` smoke coverage includes exact-byte artifact hash computation helper behavior.
- Reworded deferred boundaries from broad "active hash computation" language to file IO hashing, hash normalization, proofpack hash-integration, writer, storage, schema, CLI, and gate decision writing.
- Added the twenty-fifth advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/code/schema/CLI authority changed.

This did not:

- change Rust code;
- add CLI behavior;
- write `.punk/` state;
- implement proofpack writer behavior;
- add file IO hashing;
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
  reason: "Canonical crate-status wording changed to match current exact-byte hash computation helper behavior without promoting runtime/writer/file-IO-hash/CLI scope."
  touched_surfaces:
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md
    - work/goals/goal_run_twenty_fifth_work_ledger_review.md
    - work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_reconcile_crate_status_exact_byte_hash_computation.md work/goals/goal_run_twenty_fifth_work_ledger_review.md work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md --report work/reports/2026-04-26-crate-status-exact-byte-hash-computation.md` - PASS with 0 failures and 2 warnings:
  - `DOC_DUPLICATE_DEFINITION_CANDIDATE` for `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary`.
  - `DOC_DUPLICATE_DEFINITION_CANDIDATE` for `docs/product/CRATE-STATUS.md`: `Current CLI surface`.
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the twenty-fifth advisory Work Ledger Review.
- Decide whether the next branch should be proofpack writer preparation, file IO hash boundary, proofpack hash integration, gate/eval/proof orchestration, runtime storage, schema work, or another active-core guardrail.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
