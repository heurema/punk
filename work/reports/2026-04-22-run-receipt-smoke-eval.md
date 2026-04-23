---
id: report_2026_04_22_run_receipt_smoke_eval
goal_id: goal_add_run_receipt_smoke_eval
actor: vitaly
created_at: 2026-04-23
kind: handoff
---

## Summary

Extend the existing deterministic smoke eval harness to cover the receipt-aware contract+flow path so receipt evidence, denied no-receipt behavior, and pre-gate evidence boundaries stay regression-tested.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a narrow extension of the existing smoke eval harness to cover the already implemented run-receipt-aware contract+flow path. It does not introduce storage, schema changes, gate/proof, or CLI receipt surfaces.
Decision:
Proceed with smoke eval coverage only.

## Files Changed

- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `work/goals/goal_add_run_receipt_smoke_eval.md`
- `work/goals/goal_research_gate_decision_boundary.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-run-receipt-smoke-eval.md`

## Smoke Cases Added

- `eval_contract_receipt_allowed_path_produces_evidence`
- `eval_contract_receipt_draft_denial_produces_no_receipt`
- `eval_contract_receipt_invalid_scope_produces_no_receipt`
- `eval_contract_receipt_remains_pre_gate_evidence`

The existing `SmokeEvalReport`, human renderer, and JSON renderer were reused unchanged. The new receipt-aware cases appear through the existing output surfaces without schema or CLI changes.

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-domain`
- `cargo test -p punk-contract`
- `cargo test -p punk-flow`
- `cargo test -p punk-eval`
- `cargo test -p punk-cli`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `cargo run -q -p punk-cli -- eval run smoke --format json`
- `cargo run -q -p punk-cli -- eval run smoke --format json | python3 -c 'import json,sys; json.load(sys.stdin)'`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-run-receipt-smoke-eval.md --report work/reports/2026-04-22-run-receipt-smoke-eval.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Results

- The smoke harness now covers the allowed contract+flow path that produces receipt evidence.
- Draft and invalid-scope denials still produce no receipts.
- Denied transitions still do not mutate flow state.
- Receipt evidence remains pre-gate and does not imply final acceptance.
- Human and JSON smoke output surface the new receipt-aware cases without schema or CLI changes.

## Scope Boundaries Preserved

- no `.punk/runs`
- no receipt storage
- no receipt hashing or provenance
- no gate/proof
- no schema or spec changes
- no CLI changes
- no validators
- no baseline or waiver changes
- no product-doc changes

## Deferred

- gate decision boundary research
- proofpack boundary research
- `.punk/runs` runtime storage
- receipt hashing and provenance
- later gate/proof implementation

## Next Recommended Action

`work/goals/goal_research_gate_decision_boundary.md`
