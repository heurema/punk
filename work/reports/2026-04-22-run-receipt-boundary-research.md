---
id: report_2026_04_22_run_receipt_boundary_research
goal_id: goal_research_run_receipt_boundary
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Research the boundary for future Punk run receipts before any `.punk/runs`, receipt schema, gate/proof, or validator implementation.

## Research Gate

Classification: R2
Required: yes
Rationale:
Run receipts affect runtime evidence, contract/run ordering, future gate/proof references, artifact provenance, and possible storage boundaries.
Research refs:
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/EVAL-PLANE.md`
Decision:
Proceed with research-only boundary analysis. Do not implement receipts, `.punk/runs`, gate/proof, or storage.

## Files Changed

- `knowledge/research/2026-04-22-run-receipt-boundary.md`
- `work/goals/goal_research_run_receipt_boundary.md`
- `work/goals/goal_define_run_receipt_boundary_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-run-receipt-boundary-research.md`

## Sources Reviewed

- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/EVAL-PLANE.md`
- `docs/adr/ADR-0009-contract-tracker-core-primitives.md`
- `work/reports/2026-04-22-contract-lifecycle-minimal.md`
- `work/reports/2026-04-22-connect-contract-to-flow-state.md`
- `work/reports/2026-04-22-contract-flow-smoke-eval.md`
- `evals/specs/smoke-eval-report.v0.1.md`
- `evals/specs/eval-storage-boundary.v0.1.md`
- `evals/specs/eval-baseline-waiver-boundary.v0.1.md`
- [SLSA provenance v1.0](https://slsa.dev/spec/v1.0/provenance)
- [GitHub Actions workflow artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [GitHub Checks API](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#create-a-check-run)
- [GitLab unit test reports](https://docs.gitlab.com/ci/testing/unit_test_reports/)
- [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/)
- [W3C PROV-DM](https://www.w3.org/TR/prov-dm/)
- [Bazel Test Encyclopedia](https://bazel.build/reference/test-encyclopedia)
- [Temporal event history docs](https://docs.temporal.io/workflow-execution/event#event-history)

## Recommendation Summary

- A run receipt should be a future evidence artifact, not a decision artifact.
- A receipt may exist before gate and should later be referenced by gate/proof.
- A denied transition should produce event evidence, not a receipt.
- Receipt, event log, eval report, and proofpack should stay separate artifacts.
- If receipt storage is introduced later, prefer append-only run-scoped artifacts under `.punk/runs/`.
- Safe next step: design/spec-only `work/goals/goal_define_run_receipt_boundary_v0_1.md`.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds research and work-ledger artifacts only. It does not change canonical product docs, specs, or runtime/code surfaces."
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-22-run-receipt-boundary.md work/reports/2026-04-22-run-receipt-boundary-research.md --report work/reports/2026-04-22-run-receipt-boundary-research.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no `.punk/runs`
- no receipt schema implementation
- no validators
- no gate/proof implementation
- no storage activation
- no product doc changes

## Deferred

- `.punk/runs` implementation
- receipt schema implementation
- receipt validators
- gate/proof integration
- proofpack manifests
- actor/provenance hardening

## Next Recommended Action

`work/goals/goal_define_run_receipt_boundary_v0_1.md`
