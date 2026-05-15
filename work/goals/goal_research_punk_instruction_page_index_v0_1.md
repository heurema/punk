---
id: goal_research_punk_instruction_page_index_v0_1
title: "Research Punk Instruction Page Index v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-15
updated_at: 2026-05-15
selected_at: 2026-05-15
started_at: 2026-05-15
completed_at: 2026-05-15
blocked_by: []
scope:
  include:
    - "knowledge/research/2026-05-15-page-index-instruction-navigation.md"
    - "knowledge/ideas/2026-05-15-punk-instruction-page-index.md"
    - "work/goals/goal_research_punk_instruction_page_index_v0_1.md"
    - "work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
    - "docs/adr/**"
    - "evals/**"
    - "publishing/**"
    - "scripts/**"
acceptance:
  - "Records PageIndex as an external reference for instruction navigation."
  - "Selects a PageIndex-style hierarchy for `.punk` instructions and module instructions."
  - "Keeps source instruction pages separate from derived indexes and thin HTML views."
  - "States generated indexes/views are advisory and rebuildable, not source of truth."
  - "Adds no dependency, runtime behavior, CLI command, module host, PubPunk activation, adapter behavior, vector DB, LLM summarization, OCR, MCP, cloud sync, or external side effect."
knowledge_refs:
  - "README.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "work/STATUS.md"
research_refs:
  - "knowledge/research/2026-05-15-page-index-instruction-navigation.md"
idea_refs:
  - "knowledge/ideas/2026-05-15-punk-instruction-page-index.md"
report_refs:
  - "work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "Instruction and module navigation affect project memory layout, future module interfaces, generated views, and user-facing guidance. The patch records advisory research/design only and uses PageIndex as an external reference without adding dependencies or runtime behavior."
  research_refs:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
  external_research_refs:
    - "https://github.com/VectifyAI/PageIndex"
  blocked_reason: null
doc_impact:
  classification: research-promotion
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The change adds advisory research and idea artifacts for a future `.punk` instruction page index without changing canonical product docs or runtime behavior."
---

## Context

Punk needs local project instructions under `.punk`, and future modules need
their own instruction pages.

The instruction surface should stay navigable without becoming a giant bundled
manual or generated HTML blob.

## Selected direction

Use a PageIndex-style hierarchy:

- focused source instruction pages;
- thin entrypoint pages;
- derived page index tree;
- optional thin HTML view;
- explicit module subtrees.

## Boundary

This goal records research and design only.

It does not change `punk init`, active CLI behavior, module host behavior,
runtime storage, generated views, PageIndex dependencies, LLM calls, OCR, MCP,
vector DB, PubPunk, adapters, or external side effects.

`selected_next` remains unchanged.
