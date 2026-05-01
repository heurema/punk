---
id: goal_reconcile_roadmap_after_greenfield_init_v0_1
title: "Reconcile roadmap/status after greenfield init v0.1"
status: ready
owner: "vitaly"
module: "docs"
priority: P2
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/NORTH-STAR.md"
    - "work/STATUS.md"
    - "work/reports/**"
    - "work/goals/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Canonical docs no longer understate that minimal greenfield `punk init <project-id>` is active."
  - "`docs/product/NORTH-STAR.md` current boundary is reconciled with the active CLI surface without expanding North Star scope."
  - "The docs continue to state that brownfield, grayfield, runtime storage, contracts, gate/proof runtime, Writer, agents/adapters, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, and spec-as-source behavior remain inactive or deferred."
  - "No product behavior, runtime behavior, CLI expansion, storage, code changes, Writer, gate writer, proof writer, proofpack writer expansion, or acceptance claim writer is added."
knowledge_refs:
  - "work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md"
  - "work/reports/2026-05-01-greenfield-init-checkpoint-status-v0-1.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local docs/status reconciliation over already implemented and verified greenfield init behavior; external research is not needed unless product meaning changes."
  research_refs:
    - "work/reports/2026-05-01-greenfield-init-compact-layout-verification-v0-1.md"
    - "work/reports/2026-05-01-greenfield-init-checkpoint-status-v0-1.md"
    - "docs/product/NORTH-STAR.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/NORTH-STAR.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "A current-boundary CLI list still omits the verified active greenfield init command."
---

## Context

The greenfield init checkpoint status confirmed minimal `punk init <project-id>` as active and found one remaining canonical-doc drift in `docs/product/NORTH-STAR.md`.

## Intent

Reconcile status wording so canonical docs consistently reflect the active greenfield init milestone without promoting any deferred behavior.

## Non-scope

Do not implement brownfield init, grayfield reconcile, runtime storage, Writer, gate writer, proof writer, proofpack writer expansion, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, spec-as-source behavior, CLI expansion, or code changes.
