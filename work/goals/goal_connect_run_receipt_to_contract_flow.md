---
id: goal_connect_run_receipt_to_contract_flow
title: "Connect run receipt to contract and flow"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
selected_at: 2026-04-23
started_at: 2026-04-23
completed_at: 2026-04-23
blocked_by: []
scope:
  include:
    - "crates/punk-domain/**"
    - "crates/punk-flow/**"
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
  - "The actual bounded run path can produce or carry a minimal run receipt using existing contract and flow facts."
  - "Denied transitions remain event evidence only and still do not create receipts."
  - "The step stays kernel-only and does not add `.punk/runs`, file storage, CLI, gate/proof, or validators."
  - "Tests cover the integration boundary between contract approval, flow transition, and receipt creation."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/product/PUNK-LAWS.md"
  - "knowledge/research/2026-04-22-run-receipt-boundary.md"
  - "evals/specs/run-receipt-boundary.v0.1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "The minimal receipt kernel now exists. The next honest step is a narrow contract/flow integration so a real bounded run can carry receipt evidence without activating storage, CLI, gate, or proof behavior."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/product/PUNK-LAWS.md"
    - "knowledge/research/2026-04-22-run-receipt-boundary.md"
    - "evals/specs/run-receipt-boundary.v0.1.md"
  external_research_refs:
    - "work/reports/2026-04-22-run-receipt-minimal.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the minimal receipt kernel exists, the next bounded step is to connect it to the existing contract-authorized run path in flow.

## Notes

This goal is complete:
- `crates/punk-flow/src/lib.rs` now exposes a narrow contract-aware helper that returns transition evidence alongside optional receipt evidence for the allowed `Approved -> StartRun` path;
- `punk-domain` still owns receipt types, while `punk-flow` only integrates them;
- denied transitions still remain event evidence only and do not create receipts;
- the next bounded step is to add smoke coverage for the new receipt-aware contract+flow path.
