---
id: goal_add_punk_root_marker_init_v0_1
title: "Add Punk root marker init v0.1"
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
  - "`punk init` creates `.punk/README.md` and `.punk/project.toml` as project root marker/setup files."
  - "The `.punk` bootstrap does not create runtime stores such as `.punk/events`, `.punk/contracts`, `.punk/runs`, `.punk/evals`, `.punk/decisions`, `.punk/proofs`, `.punk/indexes`, or `.punk/views`."
  - "Existing files are not overwritten; mismatched marker files produce blocking conflicts through the existing init writer."
  - "Smoke eval coverage and CLI tests distinguish marker files from inactive runtime stores."
  - "Product docs and the work ledger describe the active surface honestly."
knowledge_refs:
  - "docs/product/DOGFOODING.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-punk-root-marker-init-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This promotes a small setup marker under `.punk` without changing runtime storage or authority. Existing Project Memory and Dogfooding docs provide the needed repo-tracked research."
  research_refs:
    - "docs/product/DOGFOODING.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/RESEARCH-GATE.md"
    - "work/goals/goal_add_level0_project_init_v0_1.md"
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
    - "work/STATUS.md"
  rationale: "The Level 0 init scaffold now includes `.punk` marker/setup files while keeping runtime stores inactive."
---

## Context

The first Level 0 `punk init` slice intentionally avoided `.punk` to keep the authority model narrow.

The maintainer selected a smaller follow-up step: create `.punk` now, but only as a marker/setup root.

## Intent

Add `.punk/README.md` and `.punk/project.toml` to the `punk init` scaffold so real projects can start with the eventual Punk root in place.

## Non-scope

Do not implement `.punk` runtime stores.

Do not implement flow/event persistence.

Do not implement contract writer, run receipt writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, adapters, automation, provider/model execution, or project storage indexes.

## Outcome

`punk init` now creates `.punk/README.md` and `.punk/project.toml` as marker/setup files.

The init scaffold still does not create `.punk/events`, `.punk/contracts`, `.punk/runs`, `.punk/evals`, `.punk/decisions`, `.punk/proofs`, `.punk/indexes`, `.punk/views`, flow persistence, contracts, receipts, gate artifacts, proofpacks, or acceptance claims.

Tests, smoke eval coverage, docs, and work-ledger evidence were updated.
