---
id: goal_add_run_receipt_smoke_eval
title: "Add run receipt smoke eval"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-23
updated_at: 2026-04-23
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-eval/**"
    - "crates/punk-flow/**"
    - "crates/punk-domain/**"
    - "crates/punk-contract/**"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-cli/**"
    - "crates/punk-gate/**"
    - "crates/punk-proof/**"
    - "docs/product/**"
    - "knowledge/research/**"
    - "evals/specs/**"
acceptance:
  - "Smoke eval covers the allowed contract+flow path that now produces receipt evidence."
  - "Smoke eval confirms denied transitions still produce event evidence only and do not create receipts."
  - "The step stays eval-only and does not add `.punk/runs`, storage, CLI, gate/proof, or broader lifecycle activation."
  - "Existing smoke report and JSON output stay within the current report boundary unless a new bounded goal is selected."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "knowledge/research/2026-04-22-run-receipt-boundary.md"
  - "evals/specs/run-receipt-boundary.v0.1.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "The receipt-aware contract/flow path now exists, so the next honest step is deterministic smoke coverage before any storage, gate, or proof work continues."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "knowledge/research/2026-04-22-run-receipt-boundary.md"
    - "evals/specs/run-receipt-boundary.v0.1.md"
  external_research_refs:
    - "work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the receipt-aware contract/flow path exists, the next bounded step is to add deterministic smoke coverage for allowed receipt creation and denied no-receipt behavior.

## Notes

Keep this goal narrow:
- no `.punk/runs`;
- no file storage;
- no CLI changes;
- no gate/proof implementation;
- no report schema changes unless explicitly re-selected later.
