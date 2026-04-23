---
id: report_2026_04_22_gate_decision_boundary_research
goal_id: goal_research_gate_decision_boundary
actor: vitaly
created_at: 2026-04-23
kind: handoff
---

## Summary

Research the future gate decision boundary before any gate, proof, `.punk/decisions`, or validator implementation starts.

## Research Gate

Classification: R2
Required: yes
Rationale:
Final decision semantics affect closure authority, flow semantics, proof linkage, storage boundaries, and the relationship between contract, event, receipt, eval, and proof artifacts.
Research refs:
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
Decision:
Proceed with research-only boundary analysis. Do not implement gate, proof, storage, or validators.

## Files Changed

- `knowledge/research/2026-04-22-gate-decision-boundary.md`
- `work/goals/goal_research_gate_decision_boundary.md`
- `work/goals/goal_define_gate_decision_boundary_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-gate-decision-boundary-research.md`

## Sources Reviewed

Repo-tracked:

- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/eval-storage-boundary.v0.1.md`
- `evals/specs/eval-baseline-waiver-boundary.v0.1.md`
- recent contract / flow / receipt / eval reports

External prior art:

- [Open Policy Agent decision logs](https://www.openpolicyagent.org/docs/latest/management-decision-logs/)
- [GitHub REST API: check runs](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28)
- [GitHub Actions workflow artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [SLSA provenance v1.0](https://slsa.dev/spec/v1.0/provenance)
- [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/)

## Recommendation Summary

- only `gate` should write the future final closure artifact;
- contract, flow, event, receipt, eval, and later proof surfaces must stay evidence-only;
- a future `DecisionObject` should reference contract, run receipt, eval, event, and proof refs or hashes instead of absorbing their bodies;
- proofpack and gate decision should remain separate artifacts with explicit links later;
- the next honest step is design/spec-only `work/goals/goal_define_gate_decision_boundary_v0_1.md`.

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
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-22-gate-decision-boundary.md work/reports/2026-04-22-gate-decision-boundary-research.md --report work/reports/2026-04-22-gate-decision-boundary-research.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no `.punk/`
- no gate implementation
- no proofpack implementation
- no storage activation
- no validators
- no spec changes
- no product-doc changes

## Deferred

- gate implementation
- proofpack implementation details
- `.punk/decisions` storage implementation
- decision validators and schema enforcement
- signing or transparency-log behavior
- actor identity hardening

## Next Recommended Action

`work/goals/goal_define_gate_decision_boundary_v0_1.md`
