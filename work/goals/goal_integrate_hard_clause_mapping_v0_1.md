---
id: goal_integrate_hard_clause_mapping_v0_1
title: "Integrate hard clause mapping v0.1"
status: done
owner: "vitaly"
module: "contract"
priority: P1
authority: canonical
created_at: 2026-04-30
updated_at: 2026-04-30
selected_at: 2026-04-30
started_at: 2026-04-30
completed_at: 2026-04-30
blocked_by: []
scope:
  include:
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Hard contract clauses are integrated into side-effect-free contract model behavior without runtime storage or CLI behavior."
  - "Every hard clause must map to at least one validator ref, required receipt field, proof requirement ref, or explicit human gate review with a non-empty reason."
  - "Unmapped hard clauses block approval/readiness for execution-facing contract state."
  - "Soft and advisory clauses remain allowed without mandatory machine mappings, but do not override hard clauses."
  - "Unsupported hard clauses remain visible as explicit findings rather than silently passing."
  - "Gate review remains a required human review marker only; no gate decision writer is added."
  - "Proof requirement refs remain declarative; no proofpack writer is added."
  - "Writer remains downstream and cannot satisfy hard-clause mapping."
  - "`punk-eval` has smoke coverage for mapped hard clauses, unmapped hard clauses, unsupported-clause findings, soft/advisory behavior, and no gate/proof/Writer activation."
  - "No CLI behavior, `.punk/contracts` storage, runtime contract writer, Writer, proofpack writer, gate decision writer, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
  - "work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-hard-clause-mapping-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The next implementation should use the repo-tracked Contract Schema Blueprint and confirmation boundary to strengthen side-effect-free hard-clause mapping only; external research is not needed unless runtime storage, CLI, adapters, policy engine, Writer, gate writer, or proof writer scope is added."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
    - "work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The selected next step should add side-effect-free crate behavior and smoke coverage for hard-clause mapping semantics, with current-behavior documentation updated only if needed."
---

## Context

The contract draft confirmation boundary now defines how a ready draft can become approved-for-run through explicit user confirmation without gate/proof/Writer authority.

The next trust-surface gap is making hard contract clauses enforceable as mapped requirements before execution-facing contract state can be trusted.

## Notes

Do not add CLI behavior, `.punk/contracts` storage, runtime contract writing, Writer activation, proofpack writer behavior, gate decision writing, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Outcome

Completed on 2026-04-30.

Implemented side-effect-free hard clause mapping integration in `punk-contract` and `punk-eval`. Hard clauses now require validator refs, required receipt fields, proof requirement refs, or explicit human gate review with a non-empty reason before approved-for-run model state. Unsupported hard clauses are surfaced as blocking findings and do not satisfy mapping. Soft/advisory clauses may remain unmapped. Hard rationale clauses and blank human review reasons block approval.

Report: `work/reports/2026-04-30-hard-clause-mapping-v0-1.md`

Selected next: `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md`
