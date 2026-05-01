---
id: goal_change_greenfield_init_to_compact_punk_memory_layout_v0_1
title: "Change greenfield init to compact Punk memory layout v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P1
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - ".gitignore"
    - "crates/punk-project/**"
    - "crates/punk-cli/**"
    - "crates/punk-eval/**"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/FLOW.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
    - "work/reports/**"
  exclude:
    - ".punk/runtime/**"
    - ".punk/cache/**"
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
acceptance:
  - "`punk init <project-id>` creates compact tracked durable memory under `.punk/memory/`."
  - "The scaffold creates `.punk/memory/STATUS.md`, `.punk/memory/goals/goal_initial_project_setup.md`, `.punk/memory/reports/`, `.punk/memory/knowledge/ideas/`, `.punk/memory/knowledge/research/`, and `.punk/memory/adr/`."
  - "The scaffold does not create root-level `work/`, `knowledge/`, `docs/adr/`, or `publishing/` directories for user projects."
  - "`.punk/project.toml` records `[memory] layout = \"compact\"` and inactive `[runtime]` metadata."
  - "`.gitignore` does not blanket-ignore `.punk/`; it ignores only future runtime or derived `.punk` dirs."
  - "Brownfield, grayfield, runtime storage, contracts, gate/proof runtime, Writer, agents, adapters, and replayability runtime remain inactive."
  - "Tests, smoke evals, docs, and manual temp init verify the compact layout."
knowledge_refs:
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-greenfield-init-compact-memory-layout-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes:
  - "work/goals/goal_add_greenfield_init_scaffold_v0_1.md"
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This corrects the fresh greenfield init layout while preserving the existing Project Memory and Dogfooding authority split. Existing docs provide the needed repo-tracked context."
  research_refs:
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/RESEARCH-GATE.md"
    - "work/goals/goal_add_greenfield_init_scaffold_v0_1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/FLOW.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "work/STATUS.md"
  rationale: "The default user-project init layout changes from root-level memory dirs to compact `.punk/memory/`."
---

## Context

The first greenfield init implementation created root-level `work/`, `knowledge/`, and `docs/adr/` directories. That matches Punk's own dogfooding repository but is too noisy for normal user projects.

## Intent

Change the default user-project greenfield init layout to compact tracked memory under `.punk/memory/`.

## Non-scope

Do not implement brownfield reconstruction.

Do not implement grayfield reconciliation.

Do not implement runtime storage, contracts, gate/proof runtime, Writer behavior, agents, adapters, replayability runtime, or network behavior.

Do not remove or reorganize the Punk repository's existing root-level dogfooding layout.

## Outcome

`punk init <project-id>` now creates `.punk/memory/` durable memory by default and no longer creates root-level Punk memory directories in user projects.
