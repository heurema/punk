---
id: report_2026_04_22_run_receipt_minimal
goal_id: goal_add_run_receipt_minimal
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Add the smallest useful run receipt kernel as an in-code, pre-gate evidence model without activating `.punk/runs`, CLI behavior, gate/proof, or runtime persistence.

## Research Gate

Classification: R1
Required: yes
Rationale:
R2 research and run receipt boundary spec already exist. This diff only adds a minimal in-code receipt model/kernel and does not implement storage, CLI, gate/proof, runtime persistence, or receipt hashing.
Research refs:
- `knowledge/research/2026-04-22-run-receipt-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
Decision:
Proceed with minimal receipt kernel only.

## Files Changed

- `crates/punk-domain/src/lib.rs`
- `work/goals/goal_add_run_receipt_minimal.md`
- `work/goals/goal_connect_run_receipt_to_contract_flow.md`
- `work/STATUS.md`
- `work/reports/2026-04-22-run-receipt-minimal.md`

## Receipt Model

Owner crate choice:
- `punk-domain` now owns the first receipt kernel because `docs/product/CRATE-STATUS.md` defines it as the active-core owner for canonical types.
- No new crate was introduced.
- No dependency on `.punk/`, CLI, gate, or proof surfaces was added.

Minimal receipt surface:
- `RunReceiptId`
- `ContractRef`
- `RunId`
- `RunScopeRef`
- `RunArtifactRef`
- `EventRef`
- `EvalReportRef`
- `RunReceipt`
- `RunReceiptError`
- `RunReceiptBoundary`
- `run_receipt_boundary()`

Boundary semantics:
- receipt is evidence-only;
- receipt does not imply final acceptance;
- receipt does not write gate decisions;
- receipt does not create proofpacks;
- denied transitions do not become receipts;
- no runtime storage is required.

## Checks Run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-domain`
- `cargo test --workspace`
- `cargo check --workspace`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-run-receipt-minimal.md --report work/reports/2026-04-22-run-receipt-minimal.md`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

## Results

- The workspace now has a minimal canonical run receipt model.
- Receipts require non-empty receipt id, contract ref, run id, and run scope ref.
- Receipts can optionally carry artifact refs, event refs, and eval report refs.
- Explicit boundary flags keep receipt separate from final acceptance, gate decision, proofpack creation, and runtime storage.
- Denied transitions remain event evidence only.

## Scope Boundaries Preserved

- no `.punk/runs`
- no file storage
- no runtime persistence
- no CLI commands
- no `punk run`, `punk cut`, or `punk gate`
- no gate decision implementation
- no proofpack implementation
- no receipt hashing
- no actor/identity model
- no denied-transition receipts
- no eval storage, baseline, or waiver changes
- no product-doc changes

## Deferred

- connecting receipt creation to the actual contract/flow run path
- receipt smoke eval coverage
- `.punk/runs` layout and file writes
- receipt hashing and provenance hardening
- gate/proof integration
- validators and schema conformance

## Next Recommended Action

`work/goals/goal_connect_run_receipt_to_contract_flow.md`
