---
id: goal_prepare_brownfield_inventory_boundary_v0_1
title: "Prepare brownfield inventory boundary v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-03
updated_at: 2026-05-03
selected_at: 2026-05-03
started_at: 2026-05-03
completed_at: 2026-05-03
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_prepare_brownfield_inventory_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "evals/specs/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "The boundary defines what a future brownfield source corpus inventory may record without becoming project truth."
  - "The boundary separates inventory facts from reconstructed claims, intent, contracts, specs, and acceptance authority."
  - "The boundary records privacy/redaction defaults, allowed source classes, forbidden source classes, no-network defaults, and source-ref requirements."
  - "No inventory implementation, repo scanning, AI summaries, claim extraction, contract generation, gate/proof runtime, Writer behavior, runtime storage, or grayfield behavior is added."
knowledge_refs:
  - "work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md"
  - "work/reports/2026-05-03-brownfield-init-scaffold-verification-v0-1.md"
  - "docs/product/PROJECT-MEMORY.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-03-brownfield-inventory-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local boundary/design step selected after the B0 brownfield scaffold verification; external research is not needed unless the inventory authority model changes."
  research_refs:
    - "work/reports/2026-05-03-brownfield-init-scaffold-v0-1.md"
    - "work/reports/2026-05-03-brownfield-init-scaffold-verification-v0-1.md"
    - "docs/product/PROJECT-MEMORY.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "evals/specs/**"
  rationale: "The next step should define the inventory boundary before any source corpus inventory implementation."
---

## Context

B0 brownfield init scaffold is implemented and verified.

## Intent

Define the B1 inventory boundary before implementing any inventory behavior.

## Non-scope

Do not implement repo scanning, file inventory generation, source graph extraction, AI summaries, claim extraction, module inference, contract-readiness generation, contract generation, spec generation, gate/proof runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.

## Outcome

Defined Brownfield Inventory as observable structure only.

Added canonical product docs and an eval/spec boundary for future source corpus manifest work while keeping implementation inactive.

Selected `work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md` as the next verification-only goal.
