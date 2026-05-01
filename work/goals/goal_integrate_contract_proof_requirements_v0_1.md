---
id: goal_integrate_contract_proof_requirements_v0_1
title: "Integrate contract proof requirements v0.1"
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
    - "crates/punk-proof/src/lib.rs"
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
  - "Contract proof requirements are modeled side-effect-free as downstream proofpack link/hash obligations after gate outcome."
  - "Proof requirements can reference contract, run receipt, eval/assessment evidence, gate outcome, and output artifacts without creating proofpacks."
  - "Proof requirements do not become gate prerequisites and do not write gate outcomes, proofpacks, or acceptance claims."
  - "No CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime writer, Writer, proofpack writer, gate decision writer, runtime validator execution, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-05-01-contract-receipt-requirements-v0-1.md"
  - "work/reports/2026-05-01-contract-gate-input-policy-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-contract-proof-requirements-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local side-effect-free contract/proof boundary step after hard-clause mapping, receipt requirements, and gate input policy; external research is not needed unless runtime proof writing, storage, adapters, or policy engines are added."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/2026-05-01-contract-receipt-requirements-v0-1.md"
    - "work/reports/2026-05-01-contract-gate-input-policy-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The next trust-chain seam should define downstream proof requirements after gate input policy, without making proofpack a gate prerequisite or activating proof writing."
---

## Context

Hard-clause mapping, receipt requirements, and gate input policy now define the pre-run and pre-gate trust seams.

The next safe seam is modeling downstream proof requirements: what a proofpack must later link/hash after gate outcome.

## Notes

This is a side-effect-free model/eval goal only.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime contract writing, runtime receipt writing, gate decision writing, proofpack writing, Writer activation, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Completion

Completed on 2026-05-01.

Outcome:

- Added a side-effect-free `punk-contract` proof requirements declaration/assessment model with known proof targets, baseline required targets, findings, coverage, duplicate normalization, and missing/unsupported target blockers.
- Added default contract proof requirements to contract drafts and approved-for-run model state without making proof readiness block `approved_for_run`.
- Preserved the lifecycle rule that proofpack is downstream of gate outcome and acceptance claims are downstream of matching proof.
- Added `punk-eval` smoke coverage for required proof targets, missing/unsupported/duplicate targets, no proofpack writing, no `.punk/proofs` storage, no artifact hash runtime, no gate/acceptance/Writer behavior, no proofpack-before-gate behavior, and unchanged `ContractStatus` acceptance boundaries.
- Updated product docs and work-ledger evidence.

No CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime contract writer, runtime receipt writer, runtime gate decision writer, runtime proofpack writer, artifact hash runtime, Writer, agent execution, provider adapters, policy engine, runtime side effects, acceptance claim writer, or `punk init` behavior was added.
