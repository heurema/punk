---
id: report_2026_04_22_smoke_eval_json_output_v0_1
goal_id: goal_add_smoke_eval_json_output_v0_1
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Add opt-in JSON output for the existing smoke eval report while keeping human-readable output as the default CLI behavior.

## Research Gate

Classification: R1
Required: yes
Rationale:
R2 research and the schema-only v0.1 design already exist. This diff only adds opt-in JSON rendering for the existing smoke eval report and does not add storage, baseline/waiver, validators, or eval policy changes.
Research refs:
- `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md`
- `evals/specs/smoke-eval-report.v0.1.md`
Decision:
Proceed with opt-in JSON output only.

## Files Changed

- `crates/punk-eval/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `work/goals/goal_add_smoke_eval_json_output_v0_1.md`
- `work/goals/goal_research_eval_storage_and_baseline_boundary.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-smoke-eval-json-output-v0-1.md`

## JSON Shape

- `punk eval run smoke --format json` now emits the proposed v0.1 report shape:
  - `schema_version`
  - `suite_id`
  - `run_id`
  - `smoke_result`
  - `mode`
  - `runtime_persistence`
  - `report_storage`
  - `case_results`
  - `boundary_notes`
  - `deferred`
- case results emit:
  - `case_id`
  - `status`
  - `summary`
  - `assessment`
- result vocabulary stays bounded to:
  - `pass`
  - `fail`
- `run_id` is emitted as `null` because stable run identity remains deferred.
- human-readable output stays the default for `punk eval run smoke`.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds bounded CLI/library behavior and work-ledger artifacts without changing canonical product docs."
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-eval`
- `cargo test -p punk-cli`
- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `cargo run -q -p punk-cli -- eval run smoke --format json`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-smoke-eval-json-output-v0-1.md --report work/reports/2026-04-22-smoke-eval-json-output-v0-1.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`

## Scope Boundaries Preserved

- no `.punk/evals`
- no file writing
- no report storage
- no baseline comparison
- no waiver ledger
- no JSON Schema validator
- no new dependencies
- no gate/proof or contract-loop behavior
- no adapters or export formats

## Deferred

- `.punk/evals` storage
- report persistence and report history
- baseline and waiver discipline
- JSON Schema validation
- export adapters such as JUnit, SARIF, TAP, or OTel mappings

## Next Recommended Action

`work/goals/goal_research_eval_storage_and_baseline_boundary.md`
