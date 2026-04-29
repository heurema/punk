---
id: goal_truth_baseline_docs_v0_1
title: "Align current repository truth baseline v0.1"
status: done
owner: "vitaly"
module: "docs"
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
    - "README.md"
    - "CHANGELOG.md"
    - "docs/product/LINEAGE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/goals/goal_truth_baseline_docs_v0_1.md"
    - "work/reports/2026-04-29-truth-baseline-docs-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
    - "evals/specs/**"
acceptance:
  - "README clearly states the current implemented CLI surface."
  - "README clearly distinguishes current behavior from target architecture, phase-gated design, parked scope, and future scope."
  - "LINEAGE documents how adjacent repositories inform Punk without defining current Punk behavior."
  - "CHANGELOG exists and records the truth-baseline documentation pass."
  - "work/STATUS.md records the maintainer-approved scope override and v0.1-prep truth-baseline stage."
  - "No Rust code, CLI behavior, runtime storage, schema files, proofpack writer, gate decision writer, adapters, automation, provider/model runner, or `punk init` is added."
  - "Required checks are run and recorded in the report."
knowledge_refs:
  - "README.md"
  - "AGENTS.md"
  - "work/STATUS.md"
  - "docs/product/START-HERE.md"
  - "docs/product/DOCUMENTATION-MAP.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/DOGFOODING.md"
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/PUBLIC-NARRATIVE.md"
  - "CONTRIBUTING.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-29-truth-baseline-docs-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded documentation/work-ledger truth-alignment pass based on repo-tracked canonical docs and a maintainer/user-approved audit synthesis. It does not change architecture, runtime behavior, storage, CLI surface, gate/proof semantics, adapters, modules, or external side-effect policy."
  research_refs:
    - "README.md"
    - "AGENTS.md"
    - "work/STATUS.md"
    - "docs/product/START-HERE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "CONTRIBUTING.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "README.md"
    - "CHANGELOG.md"
    - "docs/product/LINEAGE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-29-truth-baseline-docs-v0-1.md"
  rationale: "The task aligns repo entry, lineage, changelog, and live work ledger with the current implemented surface and v0.1-prep truth-baseline stage."
---

## Context

A deep repository audit found that Punk's core idea is strong and consistent, but the repository needs a current-truth alignment pass so readers can clearly distinguish current implementation from target architecture, parked scope, and future surfaces.

This goal intentionally pauses the previously selected proofpack-writer boundary track to align public repo truth first.

## Non-goals

Do not implement runtime behavior.
Do not change Rust code.
Do not add CLI commands.
Do not activate `.punk/` storage.
Do not implement proofpack writing.
Do not add gate decision writing.
Do not add adapters, modules, provider/model runners, automation, cloud sync, SaaS control-plane behavior, or `punk init`.

## Outcome

Completed as a docs/work-ledger truth-baseline pass.

The selected next work returns to `work/goals/goal_define_proofpack_writer_host_path_resolution_boundary_v0_1.md` after this override is recorded.
