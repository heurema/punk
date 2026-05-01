---
id: goal_capture_contract_schema_blueprint_v0_1
title: "Capture Contract Schema Blueprint v0.1"
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
    - "docs/product/CONTRACT-SCHEMA.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "The full high-level Contract Schema Blueprint v0.1 is captured as side-effect-free code and canonical documentation."
  - "The blueprint preserves identity, authority, lifecycle, work, boundaries, clauses, validation, evidence, receipt requirements, gate policy, proof requirements, change control, and memory links."
  - "The blueprint explicitly separates `required_now`, `deferred`, `parked`, and `future` fields."
  - "Contract statuses remain lifecycle-only: draft, approved_for_run, superseded, cancelled. Accepted/rejected/partially_accepted are not ContractStatus values."
  - "Typed clause shape and hard-clause mapping rule are represented without runtime activation."
  - "Gate policy declares required inputs only and does not write gate outcomes."
  - "Proof requirements declare required links/hashes only and do not create proofpacks or authority."
  - "Writer remains downstream and is not upstream contract authority."
  - "`punk-eval` has side-effect-free smoke coverage for the blueprint boundaries."
  - "Product docs register `docs/product/CONTRACT-SCHEMA.md` as the canonical owner without duplicating runtime truth elsewhere."
  - "No runtime `.punk/contracts` storage, schema files, CLI behavior, contract writer, Writer activation, provider/model/agent adapters, automation, gate writer, proof writer, acceptance claim writer, policy engine, PKI/signatures, remote transparency logs, or `punk init` behavior is added."
  - "Level 0 done remains manual closure with evidence, not future gate acceptance."
knowledge_refs:
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/DOCUMENTATION-MAP.md"
  - "docs/product/GLOSSARY.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The task captures a product/architecture blueprint in side-effect-free code and docs using repo-tracked product docs and recent Work Ledger evidence; external research is not needed because no runtime behavior, storage, Writer, adapters, policy engine, signatures, or remote transparency are activated."
  research_refs:
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CONTRACT-SCHEMA.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The selected override adds side-effect-free crate behavior, smoke coverage, and a new canonical product doc for the Contract Schema Blueprint."
---

## Context

The user explicitly asked to preserve the full high-level Contract schema before continuing smaller `draft -> confirmation -> approved_for_run` implementation steps.

This goal temporarily overrides the previously selected sixty-fourth advisory review so the project does not lose the full contract shape while iterating in small slices.

## Notes

Do not add runtime storage, schema files, CLI behavior, `.punk` writes, contract writer behavior, Writer activation, provider/model/agent adapters, automation, gate writing, proof writing, acceptance claim writing, policy engine integration, PKI/signatures, remote transparency logs, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Outcome

Completed Contract Schema Blueprint v0.1 as side-effect-free code, docs, eval coverage, and Work Ledger evidence.

Selected next remains: `work/goals/goal_run_sixty_fourth_work_ledger_review.md`.

The next review should now assess the state after both the user intent-to-contract draft model and Contract Schema Blueprint v0.1 before selecting the next bounded implementation step.
