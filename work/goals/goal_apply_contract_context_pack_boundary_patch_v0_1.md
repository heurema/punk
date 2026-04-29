---
id: goal_apply_contract_context_pack_boundary_patch_v0_1
title: "Apply Contract Context Pack boundary patch v0.1"
status: done
owner: "vitaly"
module: "contract"
priority: P1
authority: canonical
created_at: 2026-04-29
updated_at: 2026-04-29
selected_at: 2026-04-29
started_at: 2026-04-29
completed_at: 2026-04-29
blocked_by: []
scope:
  include:
    - "crates/punk-contract/src/lib.rs"
    - "docs/adr/ADR-0016-contract-context-pack-boundary.md"
    - "docs/adapters/repo-search.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "evals/specs/context-pack-boundary.v0.1.md"
    - "knowledge/ideas/2026-04-29-context-pack-future-work.md"
    - "knowledge/research/2026-04-28-context-pack-boundary.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "The user-provided Contract Context Pack boundary patch is applied to the current repository state, including clean manual integration of rejected hunks."
  - "Contract Context Pack boundary research, ADR, eval/spec, and parked future-work artifacts are present and authority-scoped."
  - "Product docs define Contract Context Pack as advisory contract-linked evidence, not project truth, not a lifecycle phase, not a retrieval engine, and not executor brief authority."
  - "`punk-contract` exposes side-effect-free Contract Context Pack model and deterministic validation helpers for no orphan context, required clause coverage, bidirectional support/coverage mapping, speculative-source exclusion, and advisory-only boundary flags."
  - "No runtime storage, `.punk` state, schemas, CLI behavior, retrieval adapter activation, executor brief generator, gate decision, proofpack authority, or acceptance claim is added."
  - "Work ledger, goal, and report are updated; previous selected next goal remains ready."
knowledge_refs:
  - "knowledge/research/2026-04-28-context-pack-boundary.md"
  - "docs/adr/ADR-0016-contract-context-pack-boundary.md"
  - "evals/specs/context-pack-boundary.v0.1.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/product/GLOSSARY.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-29-contract-context-pack-boundary-patch-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This applies a user-provided research-backed patch that changes architecture, project-memory, and contract boundary semantics; the patch includes the research note and ADR needed for the boundary, so no additional external research is required in this application step."
  research_refs:
    - "knowledge/research/2026-04-28-context-pack-boundary.md"
    - "docs/adr/ADR-0016-contract-context-pack-boundary.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "evals/specs/context-pack-boundary.v0.1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/product/GLOSSARY.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/adapters/repo-search.md"
    - "work/reports/2026-04-29-contract-context-pack-boundary-patch-v0-1.md"
  rationale: "The patch introduces a Contract Context Pack architecture boundary and side-effect-free contract model helpers."
---

## Context

The user provided a Contract Context Pack boundary patch and asked to study and apply it to this repository.

The patch introduces a new context-selection boundary for `plot` and a side-effect-free `punk-contract` model/validator. It also updates product docs so the context pack stays below contract, gate, proof, and project-truth authority.

## Non-goals

Do not implement runtime context-pack storage.
Do not write `.punk` state.
Do not add schemas.
Do not add CLI commands.
Do not activate repo-search or retrieval adapters.
Do not add executor brief generation.
Do not make retrieval, prompts, executor memory, or context packs project truth.
Do not write gate decisions, proofpacks, or acceptance claims.

## Outcome

Applied and revised the patch against the current repository state.

The original patch had a malformed hunk count and several hunks that no longer matched current files, so clean hunks were applied with recount/reject mode and the rejected hunks were manually integrated without changing the intended boundary.

A review found that the first integrated validator accepted one-way
support/coverage mappings. The revision makes the validator fail closed when
`covered_by` points to a missing context item, when a coverage row cites an item
that does not support that clause, or when an included item support claim is
absent from clause coverage.

This is Level 0 manual closure with evidence. It is not future `gate`
acceptance.

`work/STATUS.md.selected_next` remains `work/goals/goal_define_proofpack_writer_host_path_resolution_boundary_v0_1.md` because this was an explicit user interruption, not a replacement for the next ready proofpack writer goal.
