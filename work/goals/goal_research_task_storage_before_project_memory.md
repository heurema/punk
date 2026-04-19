---
id: goal_research_task_storage_before_project_memory
title: "Research task/work storage before Project Memory implementation"
status: draft
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-18
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
latest_proof_ref: null
---

## Context

Before implementing Project Memory storage, run a focused research gate over existing task/issue systems and their failure modes.
