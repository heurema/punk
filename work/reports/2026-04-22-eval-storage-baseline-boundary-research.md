---
id: report_2026_04_22_eval_storage_baseline_boundary_research
goal_id: goal_research_eval_storage_and_baseline_boundary
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Research the boundary for future eval storage, baseline comparison, waiver semantics, and later gate/proof references before any `.punk/evals` implementation.

## Research Gate

Classification: R2
Required: yes
Rationale:
This task concerns eval storage, baseline comparison, waiver semantics, and the boundary between local assessment artifacts and future gate/proof evidence.
Research refs:
- `docs/product/EVAL-PLANE.md`
- `docs/product/ARCHITECTURE.md`
- `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md`
Decision:
Proceed with research-only boundary analysis. Do not implement storage, baseline, waiver, or proof behavior.

## Files Changed

- `knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md`
- `work/goals/goal_research_eval_storage_and_baseline_boundary.md`
- `work/goals/goal_define_eval_storage_boundary_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-eval-storage-baseline-boundary-research.md`

## Sources Reviewed

- `docs/product/EVAL-PLANE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/PUNK-LAWS.md`
- `evals/specs/smoke-eval-report.v0.1.md`
- `knowledge/research/2026-04-22-eval-report-schema-before-machine-output.md`
- `work/reports/2026-04-22-smoke-eval-harness.md`
- `work/reports/2026-04-22-smoke-eval-cli-command.md`
- `work/reports/2026-04-22-smoke-eval-report-artifact-shape.md`
- `work/reports/2026-04-22-smoke-eval-json-output-v0-1.md`
- [GitHub Actions workflow artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [GitHub Checks API](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#create-a-check-run)
- [GitLab unit test reports](https://docs.gitlab.com/ci/testing/unit_test_reports/)
- [Bazel Test Encyclopedia](https://bazel.build/reference/test-encyclopedia)
- [Bazel output directory layout](https://bazel.build/remote/output-directories)
- [cargo-nextest JUnit support](https://nexte.st/docs/machine-readable/junit/)
- [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/)
- [SLSA provenance v1.0](https://slsa.dev/spec/v1.0/provenance)

## Recommendation Summary

- Do **not** implement `.punk/evals` yet.
- Do **not** implement baseline comparison yet.
- Do **not** implement waiver behavior yet.
- If storage is introduced later, prefer append-only run-scoped artifacts with explicit schema version, run identity, and hash/reference fields.
- Treat future baseline selection as an explicit reference, not a mutable `latest` file.
- Treat future waivers as explicit policy artifacts that require scope, reason, owner, and expiry.
- Treat future gate/proof integration as reference-by-id/hash, not proof of acceptance written by eval.
- Safe next step: design/spec-only `goal_define_eval_storage_boundary_v0_1.md`.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds research and work-ledger artifacts only. It does not change canonical product docs or runtime/code surfaces."
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-22-eval-storage-baseline-waiver-boundary.md work/reports/2026-04-22-eval-storage-baseline-boundary-research.md --report work/reports/2026-04-22-eval-storage-baseline-boundary-research.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no JSON output changes
- no `.punk/evals`
- no baseline comparison implementation
- no waiver ledger implementation
- no storage model implementation
- no validators
- no gate/proof implementation

## Deferred

- `.punk/evals` storage implementation
- report persistence and report history implementation
- baseline comparison implementation
- waiver ledger implementation
- future proofpack references to eval report hashes
- export adapters and telemetry integration

## Next Recommended Action

`work/goals/goal_define_eval_storage_boundary_v0_1.md`
