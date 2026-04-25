---
id: goal_define_project_memory_storage_boundary_v0_1
title: "Define Project Memory storage boundary v0.1"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/specs/**"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Project Memory storage boundary v0.1 is defined as a docs/spec-only artifact before implementation."
  - "Spec covers repo Markdown authority, JSONL runtime-event boundary, SQLite derived-index boundary, and service-backed non-authority boundary."
  - "Spec includes deterministic eval cases for malformed JSONL, stale/missing SQLite indexes, one live ledger, and no hidden service truth."
  - "No runtime storage, CLI, schema, gate, proofpack, provider/model/agent adapter, or `.punk/` state is implemented."
knowledge_refs:
  - "docs/product/PROJECT-MEMORY.md"
  - "knowledge/research/2026-04-25-task-work-storage-before-project-memory.md"
contract_refs: []
report_refs: []
decision_refs:
  - "docs/adr/ADR-0015-project-memory-storage-direction.md"
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The storage boundary is a design/spec-only follow-up grounded in the completed R2 storage research and existing canonical Project Memory docs."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "knowledge/research/2026-04-25-task-work-storage-before-project-memory.md"
    - "docs/adr/ADR-0015-project-memory-storage-direction.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; execution of this goal will produce the storage-boundary spec and report."
---

## Context

The R2 task/work storage research chose a layered direction:

```text
repo-tracked Markdown authority
  -> append-only JSONL runtime events
  -> rebuildable SQLite indexes/views
  -> optional service-backed adapter/sync
```

Before implementation, define the v0.1 boundary and eval cases so future storage work cannot create a second tracker or hidden source of truth.

## Notes

Keep this as docs/spec-only. Do not write `.punk/` state, runtime storage, CLI commands, schema changes, gate implementation, proofpack writer, or adapters.
