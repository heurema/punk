---
id: report_2026_04_22_smoke_eval_harness
goal_id: goal_add_smoke_eval_harness
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Add the smallest useful smoke eval harness as a library-first deterministic suite over the current `punk-flow` and `punk-events` kernels.

## Research Gate

Classification: R1
Required: yes
Rationale:
This diff touches the eval operator surface, but it stays a narrow deterministic harness over already accepted flow/event/inspect foundations instead of changing eval policy or runner architecture.
Research refs:
- `docs/product/RESEARCH-GATE.md`
- `docs/product/EVAL-PLANE.md`
- `docs/product/ROADMAP.md`
External research refs:
- `work/reports/2026-04-22-research-gate-preflight.md`
- `work/reports/2026-04-22-flow-inspect-command.md`
- `work/reports/2026-04-22-connect-flow-transitions-to-event-log.md`
Decision:
Proceed with a library-first smoke harness in `punk-eval` only. Defer CLI `punk eval run smoke`, baseline comparison, waivers, and `.punk/evals` runtime activation.

## Files Changed

- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `work/goals/goal_add_smoke_eval_harness.md`
- `work/goals/goal_run_first_work_ledger_review.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-smoke-eval-harness.md`

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-eval`
- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo test -p punk-cli`
- `cargo check --workspace`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-smoke-eval-harness.md --report work/reports/2026-04-22-smoke-eval-harness.md`

## Results

- `punk-eval` now contains a deterministic `smoke.v0` suite over current flow and event kernels.
- The smoke harness checks:
  - allowed flow transition behavior;
  - denied transition behavior;
  - denied transition preserving state;
  - transition attempts producing event evidence;
  - append-only event-log monotonicity.
- The harness uses `smoke_result` and `assessment` wording only.
- No final decision writing is introduced.
- `goal_add_smoke_eval_harness` is complete.

## Scope Boundaries Preserved

- no `.punk/` runtime activation
- no `punk eval run smoke` CLI command
- no baseline comparison
- no waiver ledger
- no rich eval report storage model
- no gate/proof or contract-loop behavior
- no plugin/module/adapter or Knowledge Vault work

## Deferred

- `punk eval run smoke` as a public CLI command
- machine-readable eval report artifacts
- human Markdown eval report export beyond this handoff report
- baseline comparison and waiver semantics
- `.punk/evals` runtime activation
- the first Work Ledger review implementation itself

## Next Recommended Action

`work/goals/goal_run_first_work_ledger_review.md`

After this smoke eval harness, run the first Work Ledger review over the bounded cycle from manual Work Ledger bootstrap through smoke eval harness completion.
That review should stay advisory and must not create a second live tracker.
