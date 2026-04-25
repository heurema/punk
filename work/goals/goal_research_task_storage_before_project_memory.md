---
id: goal_research_task_storage_before_project_memory
title: "Research task/work storage before Project Memory implementation"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "knowledge/research/**"
    - "docs/adr/**"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/goals/**"
  exclude:
    - "crates/**"
acceptance:
  - "Research note compares repo Markdown, JSONL, SQLite mirror, and service-backed options."
  - "Research note extracts failure modes from Beads/minibeads/adjacent tools."
  - "ADR or roadmap update records chosen storage direction."
  - "Required eval cases are listed."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "Task/work storage affects the future Project Memory storage model and must compare prior art and failure modes before implementation."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/pilots/goalrail-process-shell.md"
    - "work/reports/2026-04-25-goalrail-process-shell-pilot.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; future execution will produce the research and any required docs/ADR updates."
---

## Context

Before implementing Project Memory storage, run a focused research gate over existing task/issue systems and their failure modes.

## Notes

This goal is ready after the manual Work Ledger semantics survived the selected goal -> report -> status update -> next selected goal cycle during the GoalRail process-shell pilot extraction.
