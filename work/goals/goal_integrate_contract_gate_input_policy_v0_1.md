---
id: goal_integrate_contract_gate_input_policy_v0_1
title: "Integrate contract gate input policy v0.1"
status: done
owner: "vitaly"
module: "contract"
priority: P1
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Contract gate input policy is modeled side-effect-free as a declaration of what future gate review requires before deciding."
  - "Gate input policy can reference approved contract state, hard-clause mapping, receipt requirements, run receipt/evidence surfaces, eval refs, scope/deviation status, and executor claim status without writing gate outcomes."
  - "Gate input policy does not require an existing proofpack; proofpack remains downstream of gate outcome."
  - "Missing gate input declarations are reported as model findings only; no gate decision writer is implemented."
  - "No CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, runtime writer, Writer, proofpack writer, gate decision writer, runtime validator execution, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-04-30-hard-clause-mapping-v0-1.md"
  - "work/reports/2026-05-01-contract-receipt-requirements-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-contract-gate-input-policy-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local side-effect-free contract/gate boundary step built from current Punk docs, hard-clause mapping, and receipt requirements; external research is not needed unless runtime gate writing, policy engines, adapters, or storage are added."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-hard-clause-mapping-v0-1.md"
    - "work/reports/2026-05-01-contract-receipt-requirements-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The next trust-chain seam should define gate input policy as model/eval behavior only, after hard-clause mapping and receipt requirements are declared."
---

## Context

Hard-clause mapping and receipt requirements now define what a contract must map and what future run receipts must capture before approved-for-run model state.

The next safe seam is modeling which inputs future `gate` review must require before it can decide.

## Notes

This is a side-effect-free model/eval goal only.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, runtime contract writing, runtime receipt writing, gate decision writing, Writer activation, proofpack writer behavior, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Completion

Completed on 2026-05-01.

Outcome:

- Added a side-effect-free `punk-contract` gate input policy/readiness model with requirement sources, known gate inputs, findings, coverage, duplicate normalization, and missing/unsupported input blockers.
- Preserved the lifecycle rule that `approved_for_run` is not `ready_for_gate`.
- Kept proofpack as a post-gate expectation only, not a required gate input.
- Added `punk-eval` smoke coverage for required gate inputs, missing/unsupported inputs, duplicate normalization, no gate/proof/acceptance/Writer/runtime activation, no proofpack-as-input behavior, and unchanged `ContractStatus` acceptance boundaries.
- Updated product docs and work-ledger evidence.

No CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime contract writer, runtime receipt writer, runtime gate decision writer, runtime proofpack writer, Writer, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior was added.
