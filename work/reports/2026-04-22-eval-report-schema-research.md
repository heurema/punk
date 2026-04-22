---
id: report_2026_04_22_eval_report_schema_research
goal_id: goal_research_eval_report_schema_before_machine_output
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Research the boundary for future machine-readable smoke eval reports before any implementation.

## Research Gate

Classification: R2
Required: yes
Rationale:
This task concerns eval report schema, machine-readable output boundary, future storage, and baseline or waiver implications.
Research refs:
- `docs/product/RESEARCH-GATE.md`
- `docs/product/EVAL-PLANE.md`
- `docs/product/ARCHITECTURE.md`
- `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md`
Decision:
Proceed with research only. Do not implement machine-readable output, schema files, `.punk/evals`, baseline, or waiver behavior in this diff.

## Files Changed

- `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md`
- `work/goals/goal_research_eval_report_schema_before_machine_output.md`
- `work/goals/goal_define_smoke_eval_report_schema_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-eval-report-schema-research.md`

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md work/reports/2026-04-22-eval-report-schema-research.md --report work/reports/2026-04-22-eval-report-schema-research.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge || true`

## Results

- The research note compared:
  - internal-only report rendering;
  - stable JSON output;
  - JSON Schema 2020-12;
  - JUnit XML export;
  - SARIF export;
  - TAP output;
  - `.punk/evals` storage.
- Recommendation:
  - keep `SmokeEvalReport` as the current canonical in-code shape;
  - do not implement machine-readable output yet;
  - define a schema-only proposal next;
  - treat JUnit, SARIF, and TAP as compatibility formats later rather than canonical internal truth.
- The research explicitly defers storage, baseline, waiver, and stable export work.

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no JSON output implementation
- no schema file implementation
- no `.punk/evals`
- no baseline or waiver implementation
- no report storage
- no gate/proof behavior

## Deferred

- stable machine-readable output
- JSON Schema implementation files
- `.punk/evals` storage
- baseline and waiver discipline
- export adapters for JUnit, SARIF, and TAP

## Next Recommended Action

`work/goals/goal_define_smoke_eval_report_schema_v0_1.md`
