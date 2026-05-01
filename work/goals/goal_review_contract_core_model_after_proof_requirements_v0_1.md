---
id: goal_review_contract_core_model_after_proof_requirements_v0_1
title: "Review contract core model after proof requirements v0.1"
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
    - "docs/product/CONTRACT-SCHEMA.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "The accumulated side-effect-free contract-core model is reviewed end-to-end from request/intent/draft through confirmation, hard-clause mapping, receipt requirements, gate input policy, and proof requirements."
  - "The review identifies any lifecycle, naming, authority, or eval drift before selecting another implementation goal."
  - "The review preserves that `approved_for_run`, `ready_for_gate`, gate outcome, proofpack, and acceptance claim are separate lifecycle surfaces."
  - "No runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, or `punk init` behavior is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-SCHEMA.md"
  - "work/reports/2026-04-30-contract-schema-blueprint-v0-1.md"
  - "work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md"
  - "work/reports/2026-04-30-hard-clause-mapping-v0-1.md"
  - "work/reports/2026-05-01-contract-receipt-requirements-v0-1.md"
  - "work/reports/2026-05-01-contract-gate-input-policy-v0-1.md"
  - "work/reports/2026-05-01-contract-proof-requirements-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local checkpoint over existing side-effect-free contract-core model and docs; external research is not needed unless new runtime, storage, adapters, policy engines, or Writer behavior are proposed."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "work/STATUS.md"
    - "work/reports/2026-05-01-contract-proof-requirements-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: review-ledger
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "After proof requirements, the accumulated contract-core trust seams need a checkpoint before selecting another implementation or writer/runtime step."
---

## Context

The contract-core model now covers:

```text
request -> intent -> draft
contract draft -> explicit confirmation -> approved_for_run
hard clauses -> mapping
hard clauses -> receipt requirements
contract -> gate input policy
contract -> proof requirements
```

Before selecting another implementation step, review the accumulated model for lifecycle, authority, naming, and eval drift.

## Notes

This is a review/checkpoint goal only.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime writers, Writer activation, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Completion

Completed on 2026-05-01.

Outcome:

- Reviewed the accumulated side-effect-free contract-core model after proof requirements.
- Confirmed the request/intent/draft -> confirmation -> approved_for_run -> hard-clause mapping -> receipt requirements -> gate input policy -> proof requirements chain is coherent.
- Confirmed the core lifecycle boundaries remain separate: `approved_for_run` is not `ready_for_gate`, gate outcome precedes proofpack, and proofpack precedes acceptance claim.
- Confirmed Replayable Project Memory remains advisory and does not activate Conformance Pack, Migration Contract Pack, Regenerative Spec, code generation, or spec-as-source behavior.
- Selected uncommitted tree stabilization next because the local tree is large and docs-governance warnings remain non-blocking but visible.

No runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, or `punk init` behavior was added.
