---
id: goal_prepare_brownfield_source_corpus_manifest_design_v0_1
title: "Prepare brownfield source corpus manifest design v0.1"
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
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_design_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Source corpus manifest design stays design/spec only and does not implement inventory."
  - "Design defines manifest schema boundaries before any repository traversal or writer work."
  - "Design preserves observable-structure-only authority and keeps claims, intent, contracts, proof, and project truth out of inventory."
  - "Design records privacy, no-network, repo-relative path, and no-file-contents-by-default constraints."
knowledge_refs:
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "evals/specs/brownfield-inventory-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-inventory-boundary-verification-v0-1.md"
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
  rationale: "This is a repo-local design/spec goal based on the verified B1 brownfield inventory boundary; external research is not needed unless implementation scope changes."
  research_refs:
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-inventory-boundary-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The design goal should prepare the schema and constraints for a future source corpus manifest without adding implementation."
---

## Context

B1 Brownfield Inventory Boundary is verified. Inventory remains observable
structure only.

## Intent

Prepare a design/spec for a future Brownfield Source Corpus Manifest before
any implementation, repository traversal, writer, or CLI behavior is selected.

## Non-scope

Do not implement repo scanning, file walking, language detection, source corpus
manifest generation, inventory writing, claim extraction, claim ledger
population, AI summaries, module maps, architecture recovery, intent recovery,
contract generation, gate/proof runtime, Writer behavior, runtime `.punk`
storage, grayfield reconciliation, Conformance Pack runtime, Migration
Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.
