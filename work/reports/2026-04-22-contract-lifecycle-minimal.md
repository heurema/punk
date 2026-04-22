---
id: report_2026_04_22_contract_lifecycle_minimal
goal_id: goal_add_contract_lifecycle_minimal
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Add the smallest useful contract lifecycle kernel in `punk-contract` with explicit scope, minimal status, validation, and approval-for-run semantics.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a narrow implementation of minimal contract lifecycle primitives already described by the current roadmap and architecture. It does not introduce storage, public CLI, gate/proof, run receipts, LLM drafting, or final contract schema architecture.
Research refs:
- `docs/product/ROADMAP.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0009-contract-tracker-core-primitives.md`
Decision:
Proceed with minimal contract lifecycle kernel only.

## Files Changed

- `crates/punk-contract/src/lib.rs`
- `work/goals/goal_add_contract_lifecycle_minimal.md`
- `work/goals/goal_connect_contract_to_flow_state.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-contract-lifecycle-minimal.md`

## Contract Model

- `ContractId` validates non-empty ids.
- `ContractScope` carries explicit scope refs and rejects empty effective scope.
- `ContractStatus` stays narrow:
  - `Draft`
  - `ApprovedForRun`
- `ContractDraft` holds id, title, and scope before approval.
- `Contract` holds id, title, scope, and bounded run-authorization status.
- `validate_contract` requires non-empty title and non-empty scope.
- `approve_contract` upgrades a valid draft into `ApprovedForRun`.
- `contract_lifecycle_boundary()` makes the kernel boundary explicit:
  - no final acceptance implied
  - no gate decision writes
  - no proofpack creation
  - no runtime storage required

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds bounded contract kernel code and work-ledger artifacts without changing canonical product docs."
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
- `cargo check --workspace`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-contract-lifecycle-minimal.md --report work/reports/2026-04-22-contract-lifecycle-minimal.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Results

- `punk-contract` now exposes a bounded contract kernel with explicit scope and minimal lifecycle semantics.
- Executable contracts require non-empty scope.
- Approval authorizes bounded work only through `ApprovedForRun`.
- The model does not imply final acceptance and does not write gate decisions.
- The kernel does not create proofpacks and does not require `.punk/` storage.

## Scope Boundaries Preserved

- no `.punk/contracts`
- no file storage
- no CLI commands
- no `punk plot`, `punk cut`, or `punk gate`
- no run receipts
- no proofpacks
- no LLM drafting
- no module host or adapters
- no eval storage, baseline, or waiver behavior

## Deferred

- contract-to-flow integration
- contract storage/runtime activation
- CLI contract commands
- run receipts
- gate/proof implementation
- LLM drafting or agent execution

## Next Recommended Action

`work/goals/goal_connect_contract_to_flow_state.md`
