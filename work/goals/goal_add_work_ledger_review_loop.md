---
id: goal_add_work_ledger_review_loop
title: "Add Work Ledger review loop"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
selected_at: 2026-04-22
started_at: 2026-04-22
completed_at: 2026-04-22
blocked_by: []
scope:
  include:
    - "work/**"
    - "AGENTS.md"
    - ".agents/skills/punk-workflow/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
acceptance:
  - "A reusable Work Ledger review template exists."
  - "The follow-up remains advisory and does not create a second live tracker."
  - "The follow-up does not add automation, a summarizer script, or `.punk/` review state."
knowledge_refs:
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/DOGFOODING.md"
  - "docs/product/CONTRACT-TRACKER.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-work-ledger-review-loop.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "Touches workflow guidance and review artifacts after the first advisory Work Ledger Review."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOGFOODING.md"
  external_research_refs:
    - "work/reports/2026-04-22-work-ledger-review.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Turn the first advisory Work Ledger Review into a reusable bounded review template and workflow note.

## Notes

Keep this follow-up narrow:
- no automation
- no summarizer script yet
- no `.punk/` review state
- no second live tracker
