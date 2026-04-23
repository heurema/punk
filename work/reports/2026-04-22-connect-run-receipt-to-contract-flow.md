---
id: report_2026_04_22_connect_run_receipt_to_contract_flow
goal_id: goal_connect_run_receipt_to_contract_flow
actor: vitaly
created_at: 2026-04-23
kind: handoff
---

## Summary

Connect the minimal run receipt kernel to the existing contract-aware flow path so the allowed `Approved -> StartRun` transition can carry receipt evidence without activating `.punk/runs`, CLI behavior, gate/proof, or runtime persistence.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a narrow integration between existing receipt, contract, and flow primitives. It does not introduce storage, public CLI, gate/proof, runtime persistence, or broader flow redesign.
Decision:
Proceed with receipt integration only.

## Files Changed

- `crates/punk-flow/src/lib.rs`
- `crates/punk-flow/Cargo.toml`
- `Cargo.lock`
- `work/goals/goal_connect_run_receipt_to_contract_flow.md`
- `work/goals/goal_add_run_receipt_smoke_eval.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md`

## Integration Boundary

- `punk-domain` still owns canonical receipt types.
- `punk-flow` now exposes a narrow helper that returns transition evidence alongside optional receipt evidence.
- Receipt evidence is created only for the allowed contract-authorized `StartRun` path.
- Denied transitions still return denial evidence only and do not create receipts.
- No storage, `.punk/`, CLI, gate, or proof behavior was introduced.

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-domain`
- `cargo test -p punk-contract`
- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo test -p punk-eval`
- `cargo check --workspace`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md --report work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Results

- The allowed contract+flow path can now produce receipt evidence alongside transition evidence.
- Draft or invalid-scope contract facts still deny `StartRun`.
- Denied transitions still do not mutate flow state.
- Denied transitions still do not produce receipts.
- Receipt evidence remains pre-gate and does not imply final acceptance.

## Scope Boundaries Preserved

- no `.punk/runs`
- no file writes
- no append-only receipt storage
- no CLI commands
- no `punk run`, `punk cut`, or `punk gate`
- no gate decisions
- no proofpacks
- no receipt hashing or provenance hardening
- no validators or schema changes
- no product-doc changes

## Deferred

- smoke coverage for the new receipt-aware path
- `.punk/runs` layout and runtime storage
- receipt hashing and provenance
- gate/proof integration
- broader flow redesign

## Next Recommended Action

`work/goals/goal_add_run_receipt_smoke_eval.md`
