---
id: goal_verify_brownfield_inventory_boundary_v0_1
title: "Verify brownfield inventory boundary v0.1"
status: ready
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-03
updated_at: 2026-05-03
selected_at: 2026-05-03
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Brownfield inventory boundary docs and eval spec are checked for observable-structure-only wording."
  - "Verification confirms inventory remains separate from claims, intent, contracts, proof, gate decisions, and project truth."
  - "Verification confirms no source inventory implementation, repo scanning, AI summaries, claim extraction, contract generation, gate/proof runtime, Writer behavior, runtime storage, or grayfield behavior was added."
knowledge_refs:
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "evals/specs/brownfield-inventory-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-inventory-boundary-v0-1.md"
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
  rationale: "This is a repo-local verification/status goal after the B1 brownfield inventory boundary; external research is not needed unless direction changes."
  research_refs:
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-inventory-boundary-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The verification goal should record evidence only and not change product behavior."
---

## Context

Brownfield inventory boundary v0.1 is defined as docs/eval only.

## Intent

Verify the boundary before selecting any source corpus manifest implementation.

## Non-scope

Do not implement repo scanning, file walking, language detection, source corpus manifest generation, claim extraction, claim ledger population, AI summaries, module maps, architecture recovery, intent recovery, contract generation, gate/proof runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.
