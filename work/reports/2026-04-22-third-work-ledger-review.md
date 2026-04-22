---
id: report_2026_04_22_third_work_ledger_review
goal_id: goal_run_third_work_ledger_review
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Summary

Run the third advisory Work Ledger Review over the contract kernel, contract-to-flow guard, contract-flow smoke coverage, run receipt boundary research, and run receipt boundary spec sequence.

This review is advisory.
It does not decide.
It does not create a second live source of truth.
Follow-up work still requires explicit goal selection through the normal Work Ledger flow.

## Research Gate

Classification: R0
Required: no
Rationale:
This is an advisory review over existing repo-tracked evidence. It does not change architecture, runtime, CLI, storage, eval semantics, validators, product docs, specs, research notes, or implementation.
Decision:
Proceed.

## Evidence Reviewed

Review range:
- `536cca9` — Run second work ledger review
- `3163616` — Add minimal contract lifecycle kernel
- `d34040e` — Connect contract approval to flow state
- `117ddf3` — Add contract-flow smoke eval coverage
- `e514246` — Research run receipt boundary
- `fa14827` — Define run receipt boundary v0.1

Repo-tracked evidence:
- `work/STATUS.md`
- `work/goals/goal_run_second_work_ledger_review.md`
- `work/goals/goal_add_contract_lifecycle_minimal.md`
- `work/goals/goal_connect_contract_to_flow_state.md`
- `work/goals/goal_add_contract_flow_smoke_eval.md`
- `work/goals/goal_research_run_receipt_boundary.md`
- `work/goals/goal_define_run_receipt_boundary_v0_1.md`
- `work/reports/2026-04-22-second-work-ledger-review.md`
- `work/reports/2026-04-22-contract-lifecycle-minimal.md`
- `work/reports/2026-04-22-connect-contract-to-flow-state.md`
- `work/reports/2026-04-22-contract-flow-smoke-eval.md`
- `work/reports/2026-04-22-run-receipt-boundary-research.md`
- `work/reports/2026-04-22-run-receipt-boundary-v0-1.md`
- `knowledge/research/2026-04-22-run-receipt-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `crates/punk-contract/src/lib.rs`
- `crates/punk-flow/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`

## Findings

### F-001

```yaml
id: F-001
domain: architecture
finding: "The contract lifecycle kernel remained bounded: `ApprovedForRun` still authorizes bounded work only and does not collapse into final acceptance, gate, or proof semantics."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - docs/product/PUNK-LAWS.md
  - docs/product/CONTRACT-TRACKER.md
  - work/reports/2026-04-22-contract-lifecycle-minimal.md
  - crates/punk-contract/src/lib.rs
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The current contract kernel and its tests already keep approval-for-run separate from acceptance, so adding more process now would not materially reduce risk."
revisit_trigger: "A later contract or CLI diff starts using approval wording as final closure or tries to write decision/proof data from the contract layer."
```

### F-002

```yaml
id: F-002
domain: code
finding: "The contract-to-flow integration stayed guard/evidence-only: contract facts can allow or deny `StartRun`, but denied transitions do not mutate flow state and do not become final decisions."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - docs/product/PUNK-LAWS.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
  - work/reports/2026-04-22-connect-contract-to-flow-state.md
  - crates/punk-flow/src/lib.rs
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The integration already preserves the assessment-vs-decision boundary at the guard level, so the better next move is to extend core lifecycle progress rather than redesign the guard model."
revisit_trigger: "A future flow diff tries to store final decision state, receipts for denials, or other closure semantics directly in flow guards."
```

### F-003

```yaml
id: F-003
domain: validation
finding: "Contract-flow smoke coverage is sufficient for the current risk surface because the smoke harness now protects the authorized run path, denial paths, non-mutating denials, and evidence-only semantics."
severity: medium
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - work/reports/2026-04-22-contract-flow-smoke-eval.md
  - crates/punk-eval/src/lib.rs
  - docs/product/EVAL-PLANE.md
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The current smoke cases already cover the new contract-flow guard branch, so the project does not need another eval/report sub-branch before moving forward."
revisit_trigger: "A future receipt or gate diff changes run authorization behavior without adding matching smoke coverage."
```

### F-004

```yaml
id: F-004
domain: research
finding: "Run receipt research and the boundary spec now define receipt semantics tightly enough to protect the next implementation step: receipt is future evidence, pre-gate, append-only later, distinct from event log, eval report, gate decision, and proofpack."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - knowledge/research/2026-04-22-run-receipt-boundary.md
  - evals/specs/run-receipt-boundary.v0.1.md
  - work/reports/2026-04-22-run-receipt-boundary-research.md
  - work/reports/2026-04-22-run-receipt-boundary-v0-1.md
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_run_receipt_minimal.md
driver: vitaly
rationale_prevents_recurrence: "Using the new boundary/spec as the contract for one narrow receipt implementation step preserves the receipt-vs-decision line and avoids turning storage or gate/proof work into implicit prerequisites."
revisit_trigger: "The next implementation proposal expands into `.punk/runs`, CLI, validators, gate/proof, or another architecture pass instead of staying a minimal pre-gate receipt kernel."
```

### F-005

```yaml
id: F-005
domain: product
finding: "Another research/spec branch for gate decision or proofpack is lower value than minimal run receipt implementation because core decision/proof boundaries are already stated in canonical docs and accepted ADRs, while Phase 3 still lacks the receipt artifact itself."
severity: high
recurrence: first-seen
evidence_strength: mixed
evidence_refs:
  - docs/product/ROADMAP.md
  - docs/product/ARCHITECTURE.md
  - docs/product/CONTRACT-TRACKER.md
  - docs/adr/ADR-0012-minimal-proofpack-provenance.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
  - work/reports/2026-04-22-run-receipt-boundary-v0-1.md
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_run_receipt_minimal.md
driver: vitaly
rationale_prevents_recurrence: "Taking the next honest core-lifecycle artifact now prevents another round of boundary-only work from replacing execution progress when the decision/proof boundary is already sufficiently documented."
revisit_trigger: "Minimal receipt implementation exposes a concrete ambiguity that the existing gate/proof docs or ADRs cannot answer."
```

## Candidate Improvements

### CI-001

```yaml
id: CI-001
based_on_findings:
  - F-004
  - F-005
summary: "Create `work/goals/goal_add_run_receipt_minimal.md` as the single next bounded step and keep it pre-gate, in-memory, and storage-free."
priority: high
destination:
  type: work-goal
  ref: work/goals/goal_add_run_receipt_minimal.md
why_now: "The run receipt boundary is now explicit, and Phase 3 core lifecycle progress is bottlenecked on the missing receipt artifact rather than on more boundary work."
why_not_now: "Do not expand this goal into `.punk/runs`, CLI, validators, gate/proof, or contract storage."
driver: vitaly
```

### CI-002

```yaml
id: CI-002
based_on_findings:
  - F-004
summary: "Keep `.punk/runs`, gate/proof implementation, contract storage, and any additional eval/report infrastructure parked until the minimal receipt kernel proves the next lifecycle surface."
priority: medium
destination:
  type: no-action
  ref: ""
why_now: "The boundary/spec sequence already answers the major trust-surface questions, so more adjacent infrastructure would be early."
why_not_now: "Parking this work is not a refusal; it is what keeps the next step narrow and testable."
driver: vitaly
```

## ADR Candidates

| Finding | Why ADR-level | Suggested ADR | Status |
|---|---|---|---|
| F-004 | If a later receipt implementation makes `.punk/runs` layout, hash rules, or canonical receipt storage semantics normative, that would cross from bounded implementation into architecture truth. | Run receipt canonical storage and hash rules | parked |

## Parked Ideas

- `.punk/runs` runtime storage activation
- receipt schema validators
- gate decision implementation
- proofpack implementation
- contract storage boundary work
- another eval/report infrastructure branch
- review automation or coherence scoring

## No Action

- `ApprovedForRun` semantics stay bounded and already have direct code/test evidence.
- The contract-to-flow guard model already preserves evidence-vs-decision semantics.
- Current smoke coverage is sufficient for the present contract-flow risk surface.

## Next Recommended Action

`work/goals/goal_add_run_receipt_minimal.md`

Rationale:
- it is the next missing core lifecycle artifact in Phase 3;
- receipt research and the boundary spec now protect the implementation boundary;
- decision/proof semantics already have stronger canonical coverage than receipt implementation does right now;
- it keeps the project moving forward without activating `.punk/runs`, gate, proof, or CLI surfaces yet.

## Scope Boundaries Preserved

- no Rust code
- no CLI changes
- no `.punk/`
- no validators
- no specs
- no research notes
- no product-doc changes
- no workflow skill or template changes
- no automation
