---
id: report_2026_04_22_smoke_eval_cli_command
goal_id: goal_add_smoke_eval_cli_command
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Add the first honest `punk eval run smoke` command as a narrow CLI wrapper over the existing library-first smoke harness.

## Research Gate

Classification: R1
Required: yes
Rationale:
This adds a narrow public CLI wrapper over the existing smoke harness. It does not change eval policy, storage, baseline or waiver semantics, or runtime persistence.
Research refs:
- `work/reports/2026-04-22-smoke-eval-harness.md`
- `work/reports/2026-04-22-work-ledger-review-loop.md`
Decision:
Proceed with a minimal CLI wrapper only.

## Files Changed

- `crates/punk-cli/Cargo.toml`
- `crates/punk-cli/src/main.rs`
- `Cargo.lock`
- `work/goals/goal_add_smoke_eval_cli_command.md`
- `work/goals/goal_add_smoke_eval_report_artifact_shape.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-smoke-eval-cli-command.md`

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-eval`
- `cargo test -p punk-cli`
- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-smoke-eval-cli-command.md --report work/reports/2026-04-22-smoke-eval-cli-command.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md || true`

## Results

- `punk-cli` now depends on `punk-eval` and exposes `punk eval run smoke`.
- The command runs the existing deterministic smoke harness and prints a human-readable local assessment.
- The command returns exit code `0` when the smoke suite passes and a non-zero exit code when it fails.
- Output stays honest about current boundaries:
  - `runtime_persistence: inactive`
  - `report_storage: inactive`
  - local assessment only
- Eval wording stays assessment-only:
  - `smoke_result`
  - `assessment`
  - `case_results`
- No final decision writing was introduced.

## Scope Boundaries Preserved

- no `.punk/evals` runtime activation
- no baseline comparison
- no waiver ledger
- no rich eval report storage model
- no machine-readable report contract
- no remote or provider evals
- no gate/proof or contract-loop behavior
- no Work Ledger Review automation

## Deferred

- machine-readable smoke eval output
- eval report artifact shape beyond this CLI text output
- baseline and waiver discipline
- `.punk/evals` runtime storage
- any future coherence or gate integration over eval outputs

## Next Recommended Action

`work/goals/goal_add_smoke_eval_report_artifact_shape.md`
