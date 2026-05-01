---
id: goal_integrate_contract_receipt_requirements_v0_1
title: "Integrate contract receipt requirements v0.1"
status: done
owner: "vitaly"
module: "contract"
priority: P1
authority: canonical
created_at: 2026-04-30
updated_at: 2026-05-01
selected_at: 2026-04-30
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-domain/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Contract receipt requirements are integrated into side-effect-free model behavior without runtime storage or CLI behavior."
  - "Approved-for-run contract model state can preserve required receipt fields for later run evidence."
  - "Receipt requirements remain declarative and do not write `.punk/runs` or runtime receipts."
  - "Executor claims remain unverified unless linked to validator/evidence surfaces."
  - "Gate policy and proof requirements may reference receipt requirements as future inputs only; no gate decision writer or proofpack writer is added."
  - "Writer remains downstream and cannot satisfy receipt requirements."
  - "`punk-eval` has smoke coverage for receipt requirements preservation, missing receipt requirements, and no runtime/gate/proof/Writer activation."
  - "No CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, runtime contract writer, runtime receipt writer, Writer, proofpack writer, gate decision writer, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
  - "work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md"
  - "work/reports/2026-04-30-hard-clause-mapping-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-contract-receipt-requirements-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The next implementation should use repo-tracked Contract Schema Blueprint, confirmation boundary, hard-clause mapping, and existing receipt/domain models; external research is not needed unless runtime storage, CLI, adapters, policy engine, Writer, gate writer, or proof writer scope is added."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
    - "work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md"
    - "work/reports/2026-04-30-hard-clause-mapping-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The selected next step should connect contract receipt requirements to side-effect-free contract/domain/eval behavior without runtime receipt storage or CLI behavior."
---

## Context

The hard-clause mapping integration now blocks approved-for-run model state when hard clauses lack validator refs, required receipt fields, proof requirement refs, or explicit human gate review with reason.

The next trust-surface gap is connecting contract receipt requirements to run receipt model surfaces so later execution evidence can satisfy approved contract requirements without making executor claims authoritative.

## Notes

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, runtime contract writing, runtime receipt writing, Writer activation, proofpack writer behavior, gate decision writing, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Completion

Completed on 2026-05-01.

Outcome:

- Added a side-effect-free `punk-contract` receipt requirements model with known receipt fields, requirement sources, coverage/findings, duplicate normalization, and unsupported/missing-field blockers.
- Wired contract draft confirmation so hard-clause required receipt fields must be declared by contract receipt requirements before `approved_for_run` model state.
- Added `punk-eval` smoke coverage for covered/missing/unknown/duplicate requirements, soft/advisory non-blocking behavior, executor claims as unverified evidence, and no receipt/runtime/gate/proof/Writer activation.
- Updated product docs and work ledger evidence.

No CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, runtime contract writer, runtime receipt writer, Writer, proofpack writer, gate decision writer, agent execution, provider adapters, policy engine, runtime side effects, or `punk init` behavior was added.
