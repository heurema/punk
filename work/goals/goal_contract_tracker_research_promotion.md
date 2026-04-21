---
id: goal_contract_tracker_research_promotion
title: "Promote contract tracker research into proposed product docs"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-19
updated_at: 2026-04-21
selected_at: null
started_at: null
completed_at: 2026-04-19
blocked_by: []
scope:
  include:
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
    - "knowledge/research/2026-04-19-contract-tracker-prior-art.md"
    - "knowledge/ideas/2026-04-19-contract-tracker-ideas.md"
  exclude:
    - "docs/product/ROADMAP.md"
    - "docs/product/START-HERE.md"
    - "new crates/**"
acceptance:
  - "Contract Tracker is documented as Punk's Work Ledger, not as a separate module or task tracker."
  - "ADR remains Proposed and does not overstate active CLI surface."
  - "Research and ideas are preserved as advisory and speculative artifacts with clear boundaries."
  - "No roadmap or public CLI truth is promoted without later review/gate."
knowledge_refs:
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
  - "knowledge/research/2026-04-19-contract-tracker-prior-art.md"
contract_refs: []
report_refs: []
decision_refs:
  - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Promote the first research pass on Contract Tracker into repo-tracked documents without expanding Punk beyond its current core-first scope.

## Notes

Keep the change documentation-only.

Do not add a new crate, do not update roadmap wording yet, and do not imply that the proposed CLI examples are already part of the active public surface.
