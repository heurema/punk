---
id: goal_add_smoke_eval_cli_command
title: "Add smoke eval CLI command"
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
    - "crates/punk-cli/**"
    - "crates/punk-eval/**"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-gate/**"
    - "crates/punk-proof/**"
acceptance:
  - "A bounded `punk eval run smoke` command exists."
  - "The command exposes the existing smoke harness honestly as a local check."
  - "The command does not activate `.punk/evals` runtime state."
  - "The command does not introduce baseline or waiver behavior."
  - "`cargo check --workspace` passes."
knowledge_refs:
  - "docs/product/EVAL.md"
  - "docs/product/EVAL-PLANE.md"
  - "docs/product/ROADMAP.md"
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
  rationale: "Adds a new eval CLI/operator surface over the existing library-first smoke harness."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "docs/product/EVAL.md"
  external_research_refs:
    - "work/reports/2026-04-22-smoke-eval-harness.md"
    - "work/reports/2026-04-22-work-ledger-review.md"
    - "work/reports/2026-04-22-work-ledger-review-loop.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Expose the existing `punk-eval` smoke harness through a narrow honest CLI command.

## Notes

Keep this goal narrow:
- no `.punk/evals` persistence
- no baseline/waiver platform
- no full eval operator surface
- no gate/proof integration
