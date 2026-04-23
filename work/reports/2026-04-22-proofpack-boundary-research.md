---
id: report_2026_04_22_proofpack_boundary_research
goal_id: goal_research_proofpack_boundary
actor: vitaly
created_at: 2026-04-23
kind: handoff
---

## Summary

Research the future proofpack boundary before any proofpack, `.punk/proofs`, gate/proof linkage, or validator implementation starts.

## Research Gate

Classification: R2
Required: yes
Rationale:
Proofpack semantics affect provenance boundaries, artifact linking, hash/reference expectations, storage boundaries, and the relationship between run receipts, eval reports, gate decisions, events, and later proof artifacts.
Research refs:
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
Decision:
Proceed with research-only boundary analysis. Do not implement proofpack, storage, validators, or CLI surfaces.

## Files Changed

- `knowledge/research/2026-04-22-proofpack-boundary.md`
- `work/goals/goal_research_proofpack_boundary.md`
- `work/goals/goal_define_proofpack_boundary_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-proofpack-boundary-research.md`

## Sources Reviewed

Repo-tracked:

- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- recent contract / flow / receipt / eval / gate-boundary reports

External prior art:

- [SLSA provenance v1.0](https://slsa.dev/spec/v1.0/provenance)
- [in-toto Statement v1](https://github.com/in-toto/attestation/blob/main/spec/v1/statement.md)
- [GitHub Actions workflow artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [Open Policy Agent decision logs](https://www.openpolicyagent.org/docs/latest/management-decision-logs/)
- [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/)

## Recommendation Summary

- proofpack should remain a separate evidence bundle, not a decision artifact;
- gate decision remains the closure authority;
- proofpack should reference contract, receipt, eval, event, decision, and output artifact refs or hashes instead of absorbing their bodies;
- the safest first boundary treats canonical proofpack as post-gate and keeps any earlier draft notion out of scope;
- the next honest step is design/spec-only `work/goals/goal_define_proofpack_boundary_v0_1.md`.

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
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-22-proofpack-boundary.md work/reports/2026-04-22-proofpack-boundary-research.md --report work/reports/2026-04-22-proofpack-boundary-research.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no `.punk/`
- no proofpack implementation
- no storage activation
- no validators
- no spec changes
- no product-doc changes

## Deferred

- proofpack implementation
- `.punk/proofs` storage layout
- proofpack validators and schema enforcement
- signing or transparency-log behavior
- draft-vs-final proofpack modeling
- hash normalization details

## Next Recommended Action

`work/goals/goal_define_proofpack_boundary_v0_1.md`
