---
id: report_2026_04_25_crate_status_current_vs_target_scope
goal_id: goal_reconcile_crate_status_current_vs_target_scope
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Reconcile `docs/product/CRATE-STATUS.md` so it distinguishes target crate ownership from current implemented behavior where gate/proof/hash wording could overclaim runtime behavior.

## Research Gate

Classification: R0
Required: no
Rationale:
This is a docs honesty update against repo-tracked implementation state and canonical docs. No external research or architecture change was required.

Research refs:

- `docs/product/CRATE-STATUS.md`
- `docs/product/ROADMAP.md`
- `work/STATUS.md`
- `work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md`
- `work/reports/2026-04-25-fifteenth-work-ledger-review.md`

Decision:
Proceed with docs-only current-vs-target scope reconciliation.

## Changed files

- `docs/product/CRATE-STATUS.md`
- `work/goals/goal_reconcile_crate_status_current_vs_target_scope.md`
- `work/goals/goal_run_sixteenth_work_ledger_review.md`
- `work/reports/2026-04-25-crate-status-current-vs-target-scope.md`
- `work/STATUS.md`

## What changed

- Updated `docs/product/CRATE-STATUS.md` frontmatter `updated_at`.
- Renamed the crate table heading from initial target status to crate ownership target.
- Adjusted target ownership wording for:
  - `punk-core` as future hashing helpers;
  - `punk-gate` as gate decision model and future decision writer;
  - `punk-proof` as proofpack manifest/hash refs and future proofpack writer.
- Added a current implemented subset boundary that states the ownership table is not a claim that every target behavior is active today.
- Recorded current narrow behavior for active-core crates, including gate/proof/eval/storage/hash limitations.
- Explicitly deferred writer/storage/hash behavior to separate bounded goals.
- Added the sixteenth advisory Work Ledger Review as the next ready goal.

## Authority boundary

This was a docs-only honesty update.

It did not:

- change Rust code;
- add CLI behavior;
- write `.punk/` state;
- implement gate or proofpack writers;
- compute hashes;
- normalize hashes;
- add schema files;
- add provider/model/agent adapters;
- add automation;
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
  classification: docs-only
  reason: "Canonical crate-status wording changed to distinguish target ownership from current implemented behavior and avoid overclaiming gate/proof/hash runtime scope."
  touched_surfaces:
    - docs/product/CRATE-STATUS.md
    - work/goals/goal_reconcile_crate_status_current_vs_target_scope.md
    - work/goals/goal_run_sixteenth_work_ledger_review.md
    - work/reports/2026-04-25-crate-status-current-vs-target-scope.md
    - work/STATUS.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - work/reports/2026-04-25-crate-status-current-vs-target-scope.md
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_reconcile_crate_status_current_vs_target_scope.md work/goals/goal_run_sixteenth_work_ledger_review.md work/reports/2026-04-25-crate-status-current-vs-target-scope.md --report work/reports/2026-04-25-crate-status-current-vs-target-scope.md` - PASS with warnings:
  - `DOC_DUPLICATE_DEFINITION_CANDIDATE` for `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary`.
  - `DOC_DUPLICATE_DEFINITION_CANDIDATE` for `docs/product/CRATE-STATUS.md`: `Current CLI surface`.
- `cargo test --workspace` - PASS.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found.

## Open follow-ups

- Run the sixteenth advisory Work Ledger Review.
- Decide whether the next implementation branch should be proofpack writer, runtime storage, gate/eval/proof orchestration, schema/hash work, semantic assessor integration, or another active-core setup step.
- Keep `.punk/` storage and `punk init` deferred until separately selected.
