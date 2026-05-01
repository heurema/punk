---
id: goal_define_contract_draft_confirmation_boundary_v0_1
title: "Define contract draft confirmation boundary v0.1"
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
  - "A side-effect-free boundary defines how a ready contract draft can be explicitly confirmed by the user into an approved-for-run contract or equivalent approved model."
  - "A ready draft can become approved-for-run only through explicit user confirmation."
  - "User confirmation does not equal gate acceptance and cannot write final decisions."
  - "`clarification_required`, `refused_or_deferred`, and `blocked` drafts cannot become approved contracts."
  - "Unresolved unknowns block approval unless explicitly converted to assumptions."
  - "Approved contract state preserves scope, non-scope, evidence plan, side-effect boundaries, and downstream closure expectations."
  - "Writer remains downstream and cannot be selected as an upstream closure mechanism."
  - "`punk-eval` has smoke coverage for ready confirmation, blocked/clarification/refusal denial, unknown handling, non-gate acceptance, no final-decision writing, and Writer-not-upstream behavior."
  - "No CLI behavior, `.punk/contracts` storage, runtime contract writer, Writer, proofpack writer, gate decision writer, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md"
  - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
  - "work/reports/2026-04-30-sixty-fourth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The next implementation should use repo-tracked intent-to-contract and Contract Schema Blueprint evidence to add a side-effect-free confirmation boundary only; external research is not needed unless runtime UX, CLI, storage, adapters, policy engine, Writer, gate writer, or proof writer scope is added."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md"
    - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
    - "work/reports/2026-04-30-sixty-fourth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The selected next step should add side-effect-free crate behavior and smoke coverage for draft confirmation semantics, with current-behavior documentation updated only if needed."
---

## Context

The intent-to-contract draft model now classifies drafts as `ready_for_user_confirmation`, `clarification_required`, `refused_or_deferred`, or `blocked`.

Contract Schema Blueprint v0.1 now preserves the full target contract shape without runtime activation.

The next unresolved upstream seam is how an explicitly ready draft becomes an approved-for-run contract without confusing user confirmation with gate acceptance.

## Notes

Do not add CLI behavior, `.punk/contracts` storage, runtime contract writing, Writer activation, proofpack writer behavior, gate decision writing, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Outcome

Completed the side-effect-free contract draft confirmation boundary v0.1.

Added `ContractDraftConfirmation`, `ContractDraftConfirmationOutcome`, `ContractDraftApprovalEvidence`, unknown-handling types, and approved-for-run model state in `punk-contract`.

Added `punk-eval` smoke coverage for explicit confirmation, non-ready denial states, unresolved unknown handling, preservation of scope/evidence/side-effect boundaries, and confirmation not creating gate/proof/Writer authority.

Selected next: `work/goals/goal_integrate_hard_clause_mapping_v0_1.md`.

No CLI behavior, `.punk/contracts` storage, runtime contract writer, Writer, proofpack writer, gate decision writer, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior was added.
