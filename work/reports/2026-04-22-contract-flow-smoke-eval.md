---
id: report_2026_04_22_contract_flow_smoke_eval
goal_id: goal_add_contract_flow_smoke_eval
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Extend the existing deterministic smoke eval harness to cover the new contract-aware flow guard so that contract authorization, draft denial, invalid-scope denial, state preservation, and guard-evidence boundaries all stay regression-tested.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a narrow extension of the existing smoke eval harness to cover the already implemented contract-to-flow guard integration. It does not change eval policy, schemas, storage, baseline/waiver semantics, gate/proof, or run receipts.
Research refs:
- `docs/product/ROADMAP.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/CONTRACT-TRACKER.md`
- `work/reports/2026-04-22-connect-contract-to-flow-state.md`
Decision:
Proceed with smoke eval coverage only.

## Files Changed

- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `work/goals/goal_add_contract_flow_smoke_eval.md`
- `work/goals/goal_research_run_receipt_boundary.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-contract-flow-smoke-eval.md`

## Smoke Cases Added

- `eval_contract_ready_for_bounded_work_allows_start_run`
- `eval_contract_draft_denies_start_run`
- `eval_contract_invalid_scope_denies_start_run`
- `eval_contract_denial_does_not_mutate_flow_state`
- `eval_contract_guard_result_remains_evidence_not_decision`

The existing `SmokeEvalReport` shape and human/JSON renderers were reused as-is, so `punk eval run smoke` and `punk eval run smoke --format json` automatically surface the new contract-flow cases without schema or CLI changes.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff extends the existing smoke harness and work-ledger artifacts without changing canonical product docs or eval specs."
  touched_surfaces: []
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-contract`
- `cargo test -p punk-flow`
- `cargo test -p punk-eval`
- `cargo test -p punk-cli`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `cargo run -q -p punk-cli -- eval run smoke --format json`
- `cargo run -q -p punk-cli -- eval run smoke --format json | python3 -c 'import json,sys; json.load(sys.stdin)'`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-contract-flow-smoke-eval.md --report work/reports/2026-04-22-contract-flow-smoke-eval.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Results

- The smoke harness now covers the contract-aware `Approved -> StartRun` path with explicit `ApprovedForRun` authorization.
- Draft or unapproved contract facts continue to deny `StartRun`.
- Empty contract scope continues to deny `StartRun`.
- Contract-aware denial keeps the flow state unchanged.
- Contract-aware guard results remain evidence and do not become final decisions.
- Human and JSON smoke output automatically include the new cases through the existing report renderer.

## Scope Boundaries Preserved

- no `.punk/evals`
- no report storage
- no baseline comparison
- no waiver ledger
- no eval policy changes
- no schema or spec changes
- no JSON schema validation
- no gate/proof behavior
- no run receipts
- no contract storage
- no CLI contract commands

## Deferred

- run receipt boundary research
- runtime receipt/storage activation
- gate/proof implementation
- baseline/waiver work
- schema validation and export adapters

## Next Recommended Action

`work/goals/goal_research_run_receipt_boundary.md`
