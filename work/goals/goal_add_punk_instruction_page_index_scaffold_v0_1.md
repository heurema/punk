---
id: goal_add_punk_instruction_page_index_scaffold_v0_1
title: "Add Punk Instruction Page Index Scaffold v0.1"
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
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "crates/punk-cli/src/main.rs"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_punk_instruction_page_index_scaffold_v0_1.md"
    - "work/reports/2026-05-15-punk-instruction-page-index-scaffold-v0-1.md"
  exclude:
    - ".github/**"
    - ".punk/**"
    - "crates/**/adapters/**"
    - "docs/adr/**"
    - "evals/**"
    - "publishing/**"
    - "scripts/**"
acceptance:
  - "`punk init <project-id>` creates thin local instruction entrypoints under `.punk/instructions/`."
  - "`punk init <project-id> --mode brownfield` creates the same thin local instruction entrypoints without scanning or reconstructing the repo."
  - "The instruction page-index model is deterministic, advisory, source-ref based, and side-effect-free."
  - "Generated `.punk/views/instructions/page-index.json` is only a future rebuildable view path and is not written by init."
  - "Smoke and CLI coverage prove the bounded init scaffold and absent generated views."
  - "Docs/status distinguish source instruction pages from generated views, runtime state, module host behavior, and project truth."
  - "No PageIndex dependency, Python runtime, LLM call, OCR, MCP, vector DB, hidden truth store, module host activation, PubPunk activation, provider adapter, external side effect, gate writer, proofpack writer, or acceptance claim behavior is added."
knowledge_refs:
  - "knowledge/research/2026-05-15-page-index-instruction-navigation.md"
  - "knowledge/ideas/2026-05-15-punk-instruction-page-index.md"
  - "README.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "work/STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-15-punk-instruction-page-index-scaffold-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This change affects `.punk` project layout, public init behavior, instruction navigation, and future module instruction boundaries. It is satisfied by the repo-tracked PageIndex research note plus existing product laws, architecture, roadmap, crate status, and project-memory boundaries."
  research_refs:
    - "knowledge/research/2026-05-15-page-index-instruction-navigation.md"
    - "knowledge/ideas/2026-05-15-punk-instruction-page-index.md"
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
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The patch changes `punk init` output and smoke coverage, so active-surface docs and work-ledger evidence were updated."
---

## Context

Punk needs a local instruction surface under `.punk` before module-specific
instructions, publishing workflow instructions, or generated instruction views
can be useful.

The prior research goal selected a PageIndex-style hierarchy as an advisory
pattern, not a dependency.

## Selected Slice

Implemented the smallest active project slice:

- thin source instruction pages under `.punk/instructions/`;
- a deterministic advisory instruction page-index model in `punk-project`;
- smoke and CLI coverage that the scaffold exists and generated views do not.

## Boundary

The source instruction pages are repo-tracked setup guidance.

The generated page-index view path is named only as future rebuildable advisory
state. Init does not create `.punk/views/`.

This does not activate a module host, plugin runtime, generated HTML app,
PageIndex package, LLM summarization, vector DB, PubPunk, adapters, external
side effects, gate writer, proofpack writer, or acceptance claims.

`selected_next` remains unchanged in `work/STATUS.md`.

## Outcome

Done with code, smoke coverage, CLI tests, docs/status, and work-report
evidence.
