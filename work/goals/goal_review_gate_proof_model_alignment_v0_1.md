---
id: goal_review_gate_proof_model_alignment_v0_1
title: "Review gate/proof model alignment v0.1"
status: ready
owner: "vitaly"
module: "process"
priority: P1
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/reports/**"
    - "work/goals/**"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ROADMAP.md"
    - "crates/punk-gate/src/lib.rs"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "The review confirms whether gate outcome, proofpack, proof requirements, proofpack-writer slices, and acceptance claim boundaries align."
  - "The review confirms proofpack remains downstream of gate outcome and does not become a gate input or acceptance authority."
  - "The review recommends the next bounded step without implementing gate writers, proof writers, runtime storage, Writer, or acceptance claim writers."
  - "Replayable Project Memory remains advisory and does not activate Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, code generation, or spec-as-source behavior."
  - "No implementation, runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, or `punk init` behavior is added."
knowledge_refs:
  - "work/STATUS.md"
  - "work/reports/2026-05-01-next-contract-core-phase-after-checkpoint-review.md"
  - "work/reports/2026-05-01-contract-core-checkpoint-post-commit-verification.md"
  - "work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is repo-local review over existing gate/proof/contract/proofpack model boundaries; external research is not needed unless new product/runtime behavior is proposed."
  research_refs:
    - "work/STATUS.md"
    - "work/reports/2026-05-01-next-contract-core-phase-after-checkpoint-review.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "docs/product/CRATE-STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: process-ledger
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The next phase review selected gate/proof model alignment as the next review-only trust-boundary checkpoint."
---

## Context

The contract-core checkpoint is committed and verified.

The side-effect-free contract-core chain is coherent through proof requirements:

```text
raw request
-> intent
-> contract draft
-> user confirmation
-> approved_for_run
-> hard clause mapping
-> receipt requirements
-> gate input policy
-> proof requirements
```

The next trust-critical seam is the lifecycle and authority relationship:

```text
gate outcome -> proofpack -> acceptance claim
```

## Intent

Review consistency between `punk-gate`, `punk-proof`, contract gate input policy, contract proof requirements, proofpack-writer slices, and product docs.

## Non-scope

Do not implement Writer.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime writers, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, agent execution, provider adapters, policy engine integration, runtime side effects, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, or `punk init`.

Do not add new gate/proof model features unless a later bounded goal explicitly selects them.
