---
id: goal_research_task_storage_before_project_memory
title: "Research task/work storage before Project Memory implementation"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: 2026-04-25
completed_at: 2026-04-25
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
  - "docs/product/PROJECT-MEMORY.md"
  - "knowledge/research/2026-04-25-task-work-storage-before-project-memory.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-task-work-storage-research.md"
decision_refs:
  - "docs/adr/ADR-0015-project-memory-storage-direction.md"
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
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/adr/ADR-0003-project-memory-plane.md"
    - "docs/adr/ADR-0009-contract-tracker-core-primitives.md"
    - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
    - "knowledge/research/2026-04-25-task-work-storage-before-project-memory.md"
    - "work/pilots/goalrail-process-shell.md"
    - "work/reports/2026-04-25-goalrail-process-shell-pilot.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: research-promotion
  required_updates:
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/adr/ADR-0015-project-memory-storage-direction.md"
  rationale: "R2 storage research produced a proposed ADR and updated the canonical Project Memory storage direction without implementation."
---

## Context

Before implementing Project Memory storage, run a focused research gate over existing task/issue systems and their failure modes.

## Notes

This goal is ready after the manual Work Ledger semantics survived the selected goal -> report -> status update -> next selected goal cycle during the GoalRail process-shell pilot extraction.


## Outcome

Completed as an R2 research/docs-only task.

Artifacts:

- `knowledge/research/2026-04-25-task-work-storage-before-project-memory.md`
- `docs/adr/ADR-0015-project-memory-storage-direction.md`
- `work/reports/2026-04-25-task-work-storage-research.md`

Direction:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events
  -> rebuildable SQLite indexes/views
  -> optional service-backed adapter/sync
```

No runtime storage, CLI, schema, gate, proofpack, provider/model/agent adapter, or `.punk/` state was implemented.
