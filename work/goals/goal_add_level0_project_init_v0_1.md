---
id: goal_add_level0_project_init_v0_1
title: "Add Level 0 project init v0.1"
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
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/FLOW.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/cases/docs/active-cli-surface/README.md"
    - "evals/cases/docs/active-cli-surface/*.yaml"
    - "scripts/check_docs_governance.py"
    - "work/STATUS.md"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
    - "crates/punk-flow/**"
    - "crates/punk-gate/**"
    - "crates/punk-proof/**"
acceptance:
  - "`punk init` creates a Dogfooding Level 0 manual project-memory scaffold in the current project root."
  - "The scaffold is limited to repo-tracked `work/`, `docs/adr/`, and `knowledge/` starter files and does not create `.punk/` runtime state."
  - "Existing files are not overwritten; mismatched existing files produce a blocking conflict."
  - "Smoke eval coverage and CLI tests cover creation and no-overwrite behavior."
  - "Product docs and the work ledger describe the active surface honestly."
knowledge_refs:
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-level0-project-init-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This promotes a small public CLI setup surface and project-memory scaffold. Existing Dogfooding and Project Memory docs provide the needed repo-tracked research; no external research is needed because runtime storage, flow persistence, and authority boundaries remain unchanged."
  research_refs:
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/START-HERE.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: active-cli-surface-update
  required_updates:
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/FLOW.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "evals/cases/docs/active-cli-surface/README.md"
    - "work/STATUS.md"
  rationale: "The implemented active CLI surface now includes a bounded Level 0 `punk init` command."
---

## Context

The maintainer wants to start using Punk on a real greenfield weekend project now, instead of waiting for full runtime authority surfaces.

Existing docs already allow Dogfooding Level 0 manual project memory before self-execution. The missing first product affordance is a small `punk init` that creates that scaffold.

## Intent

Implement the smallest useful `punk init`: initialize manual Level 0 project memory for a project that already exists on disk.

## Non-scope

Do not implement `.punk/` runtime state.

Do not implement flow/event persistence.

Do not implement contract writer, run receipt writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, adapters, automation, provider/model execution, or project storage indexes.

## Outcome

`punk init` now creates a Dogfooding Level 0 manual project-memory scaffold in the current project root.

The command writes only repo-tracked starter artifacts, reports conflicts without overwriting existing files, and keeps `.punk/` runtime state, flow persistence, contracts, receipts, gate/proof artifacts, proofpacks, and acceptance claims out of scope.

Smoke eval coverage, CLI tests, docs-governance fixtures, product docs, and this work ledger were updated.
