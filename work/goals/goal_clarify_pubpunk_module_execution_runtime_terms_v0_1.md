---
id: goal_clarify_pubpunk_module_execution_runtime_terms_v0_1
title: "Clarify PubPunk module execution runtime terms v0.1"
status: done
owner: "vitaly"
module: "pubpunk"
priority: P2
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: 2026-06-08
completed_at: 2026-06-08
blocked_by: []
scope:
  include:
    - "docs/product/MODULES.md"
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/modules/pubpunk*.md"
    - "evals/specs/pubpunk-*.md"
    - "work/goals/goal_clarify_pubpunk_module_execution_runtime_terms_v0_1.md"
    - "work/reports/2026-06-08-pubpunk-module-execution-runtime-terms-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/adapters/**"
acceptance:
  - "Clarifies that PubPunk does not own a separate runtime."
  - "Uses PubPunk module execution through the Punk-owned Module Host when describing future active PubPunk behavior."
  - "Keeps Module Host runtime, PubPunk module execution, adapters, publishing, receipts, events, gate, proofpacks, and acceptance behavior inactive."
  - "Updates Work Ledger evidence and runs required governance checks."
knowledge_refs:
  - "docs/product/MODULES.md"
  - "docs/product/MODULE-HOST.md"
  - "docs/modules/pubpunk.md"
  - "docs/product/CRATE-STATUS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-pubpunk-module-execution-runtime-terms-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: false
  rationale: "This slice clarifies user-provided architecture intent against existing repo docs and eval specs. Future Module Host runtime or active PubPunk execution remains a separate R2+ decision."
  research_refs:
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/modules/pubpunk.md"
    - "docs/product/CRATE-STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/MODULES.md"
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/modules/pubpunk*.md"
    - "evals/specs/pubpunk-*.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The slice aligns PubPunk terminology with the single Punk runtime / Module Host architecture."
---

## Context

The maintainer clarified that Punk should have one runtime boundary and that
PubPunk, DevPunk, and future punks are modules run through Punk-owned module
infrastructure, not separate runtimes.

## Intent

Remove wording that makes PubPunk sound like it owns a separate runtime.

## Outcome

Done with docs/spec/work-ledger evidence.

## Non-Scope

Do not implement Module Host runtime, PubPunk module execution, module loading,
plugin loading, public CLI behavior, adapter invocation, browser/API calls,
credential access, publishing, metrics collection, receipt writing, event
writing, gate decisions, proofpacks, or acceptance claims.
