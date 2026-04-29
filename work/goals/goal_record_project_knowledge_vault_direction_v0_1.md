---
id: goal_record_project_knowledge_vault_direction_v0_1
title: "Record Project Knowledge Vault direction v0.1"
status: done
owner: "vitaly"
module: "project-memory"
priority: P0
authority: canonical
created_at: 2026-04-29
updated_at: 2026-04-29
selected_at: 2026-04-29
started_at: 2026-04-29
completed_at: 2026-04-29
blocked_by: []
scope:
  include:
    - "knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md"
    - "knowledge/ideas/2026-04-29-compiled-project-wiki-and-claim-graph.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/goals/goal_record_project_knowledge_vault_direction_v0_1.md"
    - "work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
    - "docs/adr/**"
    - "evals/specs/**"
acceptance:
  - "Research note records Knowledge Vault != RAG direction."
  - "Idea note captures compiled wiki / claim graph / context compiler future work."
  - "Product docs lightly reference direction without claiming implementation."
  - "Work ledger/report updated."
  - "No runtime/code/CLI/storage/adapter implementation added."
  - "Checks pass."
knowledge_refs:
  - "knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md"
  - "knowledge/ideas/2026-04-29-compiled-project-wiki-and-claim-graph.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/DOCUMENTATION-MAP.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This records architecture/product direction for Project Memory and Knowledge Vault without implementation."
  research_refs:
    - "README.md"
    - "AGENTS.md"
    - "work/STATUS.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/LINEAGE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/adr/ADR-0016-contract-context-pack-boundary.md"
    - "evals/specs/context-pack-boundary.v0.1.md"
    - "knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md"
  external_research_refs:
    - "https://www.sciencestack.ai/paper/2512.13564v1"
    - "https://gist.github.com/karpathy"
    - "https://proceedings.iclr.cc/paper_files/paper/2025/hash/4a4a3c197deac042461c677219efd36c-Abstract-Conference.html"
    - "https://www.researchgate.net/publication/403308415_Codebase-Memory_Tree-Sitter-Based_Knowledge_Graphs_for_LLM_Code_Exploration_via_MCP"
    - "https://huggingface.co/papers/2507.07957"
    - "https://github.com/mem0ai/mem0"
    - "https://huggingface.co/papers/2502.12110"
    - "https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents"
    - "https://www.langchain.com/blog/context-engineering-for-agents"
    - "https://direct.mit.edu/tacl/article/doi/10.1162/tacl_a_00638/119630/Lost-in-the-Middle-How-Language-Models-Use-Long"
    - "https://www.trychroma.com/research/context-rot"
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-29-project-knowledge-vault-direction-v0-1.md"
  rationale: "Records Knowledge Vault as repo-first artifact ledger plus derived claims/graphs/views/context packs, not RAG-first memory."
---

## Context

Punk is moving into Memory-Safe Core Development v0.1. Contract Context Pack is the first practical brick. This goal records the larger Project Knowledge Vault direction before returning to proofpack writer work.

## Non-goals

Do not write code.
Do not change Rust.
Do not add CLI commands.
Do not activate `.punk`.
Do not implement Knowledge Vault.
Do not implement graph DB, embeddings, vector DB, RAG pipeline, compiled wiki generator, context compiler, daemon, MCP runtime, or retrieval adapter.
Do not add Knowledge Impact Report convention yet.
Do not continue proofpack writer work in this patch.

## Outcome

Completed as a docs/research-only direction patch.

The direction recorded is: Knowledge Vault is not RAG. It is a repo-first artifact ledger with typed claims/facts/relations, receipts/provenance, code graph and project graph, compiled derived views, context compiler direction, and stale/conflict/unknown handling.

`work/STATUS.md.selected_next` remains restored to `work/goals/goal_define_proofpack_writer_host_path_resolution_boundary_v0_1.md` because this was a maintainer-approved scope interruption before returning to proofpack writer boundary work.
