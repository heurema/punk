---
id: report_2026_04_26_crate_status_file_io_artifact_hashing_helper
goal_id: goal_reconcile_crate_status_file_io_artifact_hashing_helper
actor: vitaly
created_at: 2026-04-26
kind: handoff
---

## Summary

Reconciled `docs/product/CRATE-STATUS.md` with the current file IO artifact hashing helper implementation state.

The canonical crate-status doc now records that:

- `punk-core` provides artifact digest and repo-relative artifact ref validation helpers, exact-byte artifact hash computation, and a narrow file IO artifact hashing helper;
- the file IO helper may read one explicit regular file under an explicit repo root and validated repo-relative artifact ref for digest evidence;
- the helper remains evidence-only and does not infer refs, scan directories, silently follow symlinks, normalize bytes, verify referenced artifact bytes, write schemas, write proofpacks, write gate decisions, expose CLI behavior, or touch `.punk/` runtime state;
- `punk-eval` smoke coverage includes file IO artifact hashing helper behavior.

This was a docs-currentness update only. It does not activate referenced artifact byte verification, proofpack writer behavior, proofpack writer hash-policy integration, runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init`.

## Research Gate

Classification: R1
Required: yes
Rationale:
This was a docs-currentness reconciliation against repo-tracked implementation evidence and canonical crate status. No external research or architecture change was required.
Decision:
Proceed.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `evals/specs/file-io-artifact-hashing-boundary.v0.1.md`
- `evals/specs/artifact-hash-computation-helper.v0.1.md`
- `evals/specs/artifact-hash-policy.v0.1.md`
- `crates/punk-core/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `work/STATUS.md`
- `work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md`
- `work/reports/2026-04-26-thirtieth-work-ledger-review.md`

## Changed Files

- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_reconcile_crate_status_file_io_artifact_hashing_helper.md`
- `work/goals/goal_run_thirty_first_work_ledger_review.md`
- `work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md`

## What changed

- Updated `docs/product/CRATE-STATUS.md` so `punk-core` no longer says it never reads files.
- Recorded the active file IO artifact hashing helper as narrow, explicit, regular-file-only, and evidence-only.
- Recorded that `punk-eval` smoke coverage includes file IO artifact hashing helper behavior.
- Reworded future/deferred language to keep referenced artifact hash computation or verification for proofpack refs, broader file IO hashing, hash normalization, proofpack writer integration, runtime storage, schemas, CLI behavior, gate decisions, and acceptance claims separate.
- Added the thirty-first advisory Work Ledger Review as the next ready goal.

## Boundary

No runtime/code/schema/CLI authority changed.

This did not:

- change Rust code;
- add CLI behavior;
- write `.punk/` state;
- implement proofpack writer behavior;
- add referenced artifact byte verification;
- add referenced artifact hash computation for proofpack refs;
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
  reason: "Canonical crate-status wording changed to match current file IO artifact hashing helper behavior without promoting runtime/writer/referenced-verification/CLI scope."
  touched_surfaces:
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_reconcile_crate_status_file_io_artifact_hashing_helper.md
    - work/goals/goal_run_thirty_first_work_ledger_review.md
    - work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_reconcile_crate_status_file_io_artifact_hashing_helper.md work/goals/goal_run_thirty_first_work_ledger_review.md work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md --report work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md` - PASS with 0 failures and 2 warnings:
  - `DOC_DUPLICATE_DEFINITION_CANDIDATE` for `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary`.
  - `DOC_DUPLICATE_DEFINITION_CANDIDATE` for `docs/product/CRATE-STATUS.md`: `Current CLI surface`.
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found

## Open follow-ups

- Run the thirty-first advisory Work Ledger Review.
- Decide whether the next branch should be referenced artifact verification policy, proofpack writer preparation, proofpack writer hash-policy integration, another docs/spec guardrail, or another active-core setup step.
- Keep `.punk/` storage, runtime gate/proof behavior, schemas, CLI expansion, adapters, automation, provider/model runners, and `punk init` deferred until separately selected.
