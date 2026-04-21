---
id: goal_research_knowledge_vault
title: "Research and bound Knowledge Vault"
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
    - "knowledge/research/**"
    - "knowledge/ideas/**"
    - "docs/adr/**"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "work/goals/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Knowledge Vault boundary docs are repo-tracked."
  - "Research note distinguishes adopt, defer, park, and avoid decisions."
  - "Ideas are preserved separately from implementation truth."
  - "Required evals are listed before retrieval implementation."
knowledge_refs:
  - "knowledge/research/2026-04-19-knowledge-vault-prior-art-summary.md"
  - "docs/adr/ADR-0008-knowledge-vault-boundaries.md"
contract_refs: []
report_refs: []
decision_refs:
  - "docs/adr/ADR-0008-knowledge-vault-boundaries.md"
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

Define what `Knowledge Vault` should and should not mean for `punk` before any retrieval implementation starts.

## Non-goals

- implement embeddings
- implement MCP memory
- implement remote/shared commons
- add autonomous memory capture
- change `plot / cut / gate`

## Next contract candidate

Create a bounded docs-only contract to accept, revise, or reject ADR-0008 and keep retrieval out of active-core until eval fixtures exist.
