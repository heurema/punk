---
id: goal_add_run_receipt_minimal
title: "Add minimal run receipt"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
selected_at: null
started_at: null
completed_at: null
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
  - "A minimal in-memory run receipt kernel exists for an actual bounded run."
  - "The receipt references contract and run context without activating `.punk/runs`, file storage, or CLI behavior."
  - "The step stays pre-gate: receipts remain evidence and do not imply final acceptance or write decisions/proofs."
  - "Tests cover the boundary between an actual run and denied transitions, which must remain event-only."
knowledge_refs:
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CONTRACT-TRACKER.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
  - "docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md"
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
  rationale: "Contract-flow behavior is now bounded and smoke-covered, and the run receipt boundary/spec work is complete. The next honest step is a minimal receipt kernel without storage, CLI, gate, or proof activation."
  research_refs:
    - "docs/product/ROADMAP.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/product/PUNK-LAWS.md"
    - "knowledge/research/2026-04-22-run-receipt-boundary.md"
    - "evals/specs/run-receipt-boundary.v0.1.md"
  external_research_refs:
    - "work/reports/2026-04-22-third-work-ledger-review.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the contract lifecycle kernel, contract-to-flow guard integration, smoke coverage, and run receipt boundary sequence, the next core-lifecycle step is to add the smallest honest run receipt kernel.

## Notes

Keep this goal narrow:
- no `.punk/runs`;
- no file storage;
- no CLI receipt surface;
- no gate/proof implementation;
- denied transitions remain event evidence only, not receipts.
