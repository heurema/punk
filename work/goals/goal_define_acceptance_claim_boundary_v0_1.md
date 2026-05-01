---
id: goal_define_acceptance_claim_boundary_v0_1
title: "Define acceptance claim boundary v0.1"
status: ready
owner: "vitaly"
module: "process"
priority: P1
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/reports/**"
    - "work/goals/**"
    - "evals/specs/**"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-SCHEMA.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PUNK-LAWS.md"
  exclude:
    - ".punk/**"
    - "schemas/**"
    - "crates/**"
acceptance:
  - "Acceptance claim boundary v0.1 is captured as docs/model/eval-spec guidance before any Writer boundary design."
  - "The boundary defines acceptance claim as downstream of accepting gate decision plus matching proofpack, not as gate decision, proofpack, Writer authority, executor claim, module assessment, adapter output, or generated doc truth."
  - "The boundary records prerequisites, matching proofpack expectations, forbidden behavior, project-memory relationship, and future eval implications."
  - "No acceptance claim writer, Writer, runtime storage, CLI, gate writer, proof writer, proofpack writer expansion, artifact hash runtime expansion, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, or `punk init` behavior is added."
knowledge_refs:
  - "work/STATUS.md"
  - "work/reports/2026-05-01-acceptance-claim-boundary-review-v0-1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-SCHEMA.md"
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
  rationale: "This is a repo-local docs/model/eval-spec boundary definition based on existing proof-before-acceptance, gate/proof, proofpack, Writer-readiness, and acceptance-claim review artifacts; external research is not needed unless runtime or Writer behavior is proposed."
  research_refs:
    - "work/reports/2026-05-01-acceptance-claim-boundary-review-v0-1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/CONTRACT-SCHEMA.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture-boundary
  required_updates:
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The acceptance claim boundary review found the lifecycle concept should be explicitly captured before any Writer boundary design or implementation."
---

## Context

Acceptance claim boundary review found that current docs/specs preserve proof-before-acceptance at a high level, but acceptance claim needs its own explicit boundary before Writer boundary design.

## Intent

Define the acceptance claim boundary as a docs/model/eval-spec artifact.

## Non-scope

Do not implement acceptance claim writer.

Do not implement Writer.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime writers, gate writer, proofpack writer expansion, artifact hash runtime expansion, agent execution, provider adapters, policy engine integration, runtime side effects, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, or `punk init`.

Do not add new acceptance claim or Writer capabilities unless a later bounded goal explicitly selects them.

## Deferred selection note

This remains a valid future prerequisite for Writer boundary design, but it is not selected now.

The current contract-core / gate-proof / proofpack-writer / Writer-readiness review series intentionally stops after acceptance claim boundary review to avoid scope creep.

A future review may select this goal deliberately when Punk is ready for a new architecture boundary phase.
