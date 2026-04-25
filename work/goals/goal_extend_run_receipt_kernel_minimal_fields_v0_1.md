---
id: goal_extend_run_receipt_kernel_minimal_fields_v0_1
title: "Extend run receipt kernel with minimal fields v0.1"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-domain/src/lib.rs"
    - "crates/punk-flow/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-gate/**"
    - "crates/punk-proof/**"
    - ".punk/**"
acceptance:
  - "Run receipt kernel carries explicit schema/version/time and boundary notes without requiring runtime storage."
  - "Run receipt kernel can carry validator outcome evidence using the missing-validator policy vocabulary."
  - "Contract/run/scope refs remain required, while event/eval/artifact/validator evidence remains reference-oriented."
  - "Receipts still do not imply final acceptance, write gate decisions, create proofpacks, or hide missing validator gaps."
  - "No CLI command, `.punk/` state, gate/proof/proofpack behavior, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "evals/specs/minimal-receipt-fields.v0.1.md"
  - "evals/specs/missing-validator-policy.v0.1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "work/reports/2026-04-25-ninth-work-ledger-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This implements already-defined receipt field and missing-validator policy boundaries in the side-effect-free Rust receipt kernel without new architecture decisions."
  research_refs:
    - "evals/specs/minimal-receipt-fields.v0.1.md"
    - "evals/specs/missing-validator-policy.v0.1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "work/reports/2026-04-25-ninth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-run-receipt-kernel-minimal-fields-v0-1.md"
  rationale: "Execution will change active-core Rust receipt behavior and record a work report; no product docs are expected unless behavior boundaries change."
---

## Context

Punk already has:

- a minimal side-effect-free run receipt kernel;
- minimal receipt fields v0.1 spec;
- missing-validator policy v0.1 spec;
- proof-before-acceptance semantics v0.1 spec.

The next smallest active-core implementation step is to extend the existing receipt kernel so future gate/proof work has the evidence shape it needs.

## Notes

Keep this as a side-effect-free kernel change.

Do not write `.punk/` state.
Do not add CLI behavior.
Do not implement gate/proof/proofpack.
Do not add adapters, automation, provider/model runners, or schema files.
