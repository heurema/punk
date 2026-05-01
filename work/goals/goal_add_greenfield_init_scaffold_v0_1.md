---
id: goal_add_greenfield_init_scaffold_v0_1
title: "Add greenfield init scaffold v0.1"
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
    - "crates/punk-project/**"
    - "crates/punk-cli/**"
    - "crates/punk-eval/**"
    - "scripts/check_docs_governance.py"
    - "evals/cases/docs/active-cli-surface/README.md"
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
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
acceptance:
  - "`punk init <project-id>` creates a greenfield Level 0 project-memory scaffold."
  - "The scaffold records `project_id` and `entry_mode = greenfield` in `work/STATUS.md`."
  - "The scaffold creates `work/STATUS.md`, an initial ready goal, `work/reports/`, `knowledge/ideas/`, `knowledge/research/`, and `docs/adr/`."
  - "The existing `.punk` marker/setup files remain marker-only and no `.punk` runtime stores are created."
  - "Brownfield reconstruction, grayfield reconciliation, repo scanning, AI summaries, network behavior, contracts, gate artifacts, proofpacks, Writer behavior, and acceptance claims remain inactive."
  - "Existing files are not overwritten and mismatched files produce blocking conflicts."
  - "Smoke eval coverage verifies scaffold shape, project id/entry mode, no runtime stores, and overwrite refusal."
knowledge_refs:
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-greenfield-init-scaffold-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This promotes a small active CLI/setup surface for greenfield project memory without runtime storage or reconstruction. Existing Project Memory, Dogfooding, and crate-status docs provide the needed repo-tracked research."
  research_refs:
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/RESEARCH-GATE.md"
    - "work/goals/goal_add_level0_project_init_v0_1.md"
    - "work/goals/goal_add_punk_root_marker_init_v0_1.md"
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
  rationale: "The active init surface now requires a project id and is explicitly greenfield-only."
---

## Context

The maintainer selected the next safe product step for real greenfield dogfooding: make `punk init` explicit about greenfield entry mode and project identity without expanding into brownfield, grayfield, runtime storage, Writer, gate, or proof behavior.

## Intent

Promote the current Level 0 init scaffold into `punk init <project-id>` for greenfield projects only.

## Non-scope

Do not implement brownfield reconstruction.

Do not implement grayfield reconciliation.

Do not implement repo scanning, AI-generated summaries, network analysis, contracts, gate/proof runtime, Writer behavior, adapters, automation, or acceptance claims.

Do not create `.punk/` runtime stores.

## Outcome

`punk init <project-id>` now validates a lowercase slug project id, records `project_id` and `entry_mode = greenfield`, creates `work/goals/goal_initial_project_setup.md`, keeps `.punk` marker/setup files marker-only, and refuses overwrites.

The no-argument `punk init` form now reports usage instead of creating TODO identity scaffolds.
