---
id: report_2026_04_22_smoke_eval_report_schema_v0_1
goal_id: goal_define_smoke_eval_report_schema_v0_1
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Define a schema-only `smoke-eval-report.v0.1` proposal for future machine-readable smoke eval output without implementing JSON output, storage, validators, or baseline/waiver behavior.

## Research Gate

Classification: R1
Required: yes
Rationale:
R2 research has already been completed. This diff only creates a proposed schema/design artifact and does not implement machine-readable output, storage, validators, baseline/waiver semantics, or eval policy changes.
Research refs:
- `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md`
Decision:
Proceed with schema-only proposal.

## Files Changed

- `evals/specs/smoke-eval-report.v0.1.md`
- `work/goals/goal_define_smoke_eval_report_schema_v0_1.md`
- `work/goals/goal_add_smoke_eval_json_output_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-smoke-eval-report-schema-v0-1.md`

## Spec Summary

- defines `smoke-eval-report.v0.1` as a proposed schema/design artifact;
- keeps the current internal `SmokeEvalReport` as the canonical in-code shape for now;
- defines bounded top-level fields:
  - `schema_version`
  - `suite_id`
  - `run_id` as optional/deferred-from-required
  - `smoke_result`
  - `mode`
  - `runtime_persistence`
  - `report_storage`
  - `case_results`
  - `boundary_notes`
  - `deferred`
  - optional future provenance block;
- defines bounded case-result fields:
  - `case_id`
  - `status`
  - `summary`
  - `assessment`
  - optional `evidence_refs`;
- keeps result vocabulary intentionally narrow:
  - `pass`
  - `fail`;
- omits `skip` from v0.1 because the current smoke harness does not yet model partial execution, waiver, or skipped-case semantics.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds a schema-only eval spec proposal and work-ledger updates without changing canonical product docs or active runtime/code surfaces."
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files evals/specs/smoke-eval-report.v0.1.md work/reports/2026-04-22-smoke-eval-report-schema-v0-1.md --report work/reports/2026-04-22-smoke-eval-report-schema-v0-1.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code
- no CLI output changes
- no JSON output implementation
- no JSON Schema file
- no schema validator
- no `.punk/evals`
- no baseline comparison
- no waiver ledger
- no report persistence
- no gate/proof changes
- no adapters or export formats

## Deferred

- bounded JSON output implementation over the proposed v0.1 schema
- stable public machine-readable contract hardening
- `.punk/evals` report storage
- baseline and waiver behavior
- schema validation files or validation engine
- export adapters such as JUnit, SARIF, TAP, or OTel mappings

## Next Recommended Action

`work/goals/goal_add_smoke_eval_json_output_v0_1.md`
