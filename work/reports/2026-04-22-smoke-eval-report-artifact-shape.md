---
id: report_2026_04_22_smoke_eval_report_artifact_shape
goal_id: goal_add_smoke_eval_report_artifact_shape
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Add a bounded internal smoke eval report artifact shape in `punk-eval` and route the existing smoke CLI output through it.

## Research Gate

Classification: R1
Required: yes
Rationale:
This adds an internal bounded smoke eval report shape over the existing harness. It does not introduce stable JSON/schema, storage, baseline or waiver semantics, or eval policy changes.
Research refs:
- `work/reports/2026-04-22-smoke-eval-harness.md`
- `work/reports/2026-04-22-smoke-eval-cli-command.md`
Decision:
Proceed with internal report shape only.

## Files Changed

- `crates/punk-eval/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `work/goals/goal_add_smoke_eval_report_artifact_shape.md`
- `work/goals/goal_research_eval_report_schema_before_machine_output.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-smoke-eval-report-artifact-shape.md`

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-eval`
- `cargo test -p punk-cli`
- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-smoke-eval-report-artifact-shape.md --report work/reports/2026-04-22-smoke-eval-report-artifact-shape.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md || true`

## Results

- `punk-eval` now exposes an internal report shape built around:
  - `SmokeEvalReport`
  - `SmokeEvalSummary`
  - `SmokeEvalCaseResult`
  - `SmokeEvalStatus`
- The report shape carries:
  - suite id;
  - overall `smoke_result`;
  - summary assessment;
  - mode/runtime/report-storage boundaries;
  - case results;
  - boundary notes;
  - deferred notes.
- `punk eval run smoke` now renders from the internal report shape instead of formatting ad hoc strings in `punk-cli`.
- The report shape remains internal only and is not promoted as a stable machine-readable contract.

## Scope Boundaries Preserved

- no `.punk/evals` runtime storage
- no file writing
- no stable JSON output
- no JSON schema
- no serde dependency
- no baseline comparison
- no waiver ledger
- no remote or provider evals
- no gate/proof or contract-loop behavior

## Deferred

- machine-readable smoke eval output
- stable report schema or JSON contract
- `.punk/evals` report storage
- baseline and waiver discipline
- richer eval platform architecture beyond the current internal report shape

## Next Recommended Action

`work/goals/goal_research_eval_report_schema_before_machine_output.md`
