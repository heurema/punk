---
id: report_2026_04_22_connect_contract_to_flow_state
goal_id: goal_connect_contract_to_flow_state
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Connect minimal contract lifecycle state to existing flow transition guards so that `ApprovedForRun` can authorize bounded `StartRun` transitions while draft or invalid contract facts are denied without mutating flow state.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a narrow integration between existing contract lifecycle and flow guard primitives. It does not introduce storage, public CLI, run receipts, gate/proof, LLM drafting, or broad flow redesign.
Research refs:
- `docs/product/ROADMAP.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0009-contract-tracker-core-primitives.md`
Decision:
Proceed with contract-to-flow integration only.

## Files Changed

- `Cargo.lock`
- `crates/punk-flow/Cargo.toml`
- `crates/punk-flow/src/lib.rs`
- `work/goals/goal_connect_contract_to_flow_state.md`
- `work/goals/goal_add_contract_flow_smoke_eval.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-connect-contract-to-flow-state.md`

## Integration Boundary

- `punk-flow` now depends on `punk-contract`, not the other way around.
- `FlowInstance::attempt_transition_with_contract(...)` and `transition_with_contract(...)` accept contract authorization facts through `ContractStatus` plus `scope_valid`.
- `ApprovedForRun` with valid scope allows `StartRun` from `Approved`.
- draft or invalid contract facts deny `StartRun` with guard evidence and leave flow state unchanged.
- the integration remains evidence-only:
  - no gate decision writes
  - no proofpack creation
  - no `.punk/` runtime state

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds bounded flow/contract kernel integration and work-ledger artifacts without changing canonical product docs."
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
- `cargo test -p punk-events`
- `cargo check --workspace`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-connect-contract-to-flow-state.md --report work/reports/2026-04-22-connect-contract-to-flow-state.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Results

- `StartRun` can now be guarded by contract authorization facts without changing the core flow graph.
- `ApprovedForRun` authorizes bounded work only when scope is valid.
- draft/unapproved and invalid-scope contract facts deny the transition with guard evidence.
- denied transitions do not mutate flow state.
- allowed and denied outcomes remain guard results, not final decisions.

## Scope Boundaries Preserved

- no `.punk/contracts`
- no `.punk/flows`
- no file storage
- no CLI commands
- no `punk plot`, `punk cut`, or `punk gate`
- no run receipts
- no proofpacks
- no final decisions
- no LLM drafting
- no module host or adapters
- no eval storage, baseline, or waiver behavior

## Deferred

- contract-flow smoke eval coverage
- `.punk/contracts` or `.punk/flows` storage
- CLI contract commands
- run receipts
- gate/proof implementation
- LLM drafting or agent execution

## Next Recommended Action

`work/goals/goal_add_contract_flow_smoke_eval.md`
