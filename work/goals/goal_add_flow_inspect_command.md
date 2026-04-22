---
id: goal_add_flow_inspect_command
title: "Add flow inspect command"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-22
selected_at: 2026-04-22
started_at: 2026-04-22
completed_at: 2026-04-22
blocked_by: []
scope:
  include:
    - "crates/punk-cli/**"
    - "crates/punk-flow/**"
    - "crates/punk-events/**"
    - "docs/product/FLOW.md"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-eval/**"
acceptance:
  - "A bounded `punk flow inspect` surface exists."
  - "The command does not expose parked capability surfaces."
  - "The command makes no dishonest runtime claims."
  - "cargo check --workspace passes."
knowledge_refs:
  - "docs/product/FLOW.md"
  - "docs/product/START-HERE.md"
  - "docs/product/ROADMAP.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-flow-inspect-command.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "Touches public CLI/operator surface and inspectable flow/event views."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/START-HERE.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
  external_research_refs:
    - "work/reports/2026-04-21-flow-state-kernel.md"
    - "work/reports/2026-04-21-append-only-event-log.md"
    - "work/reports/2026-04-22-connect-flow-transitions-to-event-log.md"
    - "work/reports/2026-04-22-research-gate-preflight.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Expose inspectable flow state only after there is real state and event evidence to inspect.

## Notes

Flow transitions now emit event drafts, so this is the next honest inspect surface.
Research Gate preflight was satisfied before implementation because this goal touches a public CLI/operator surface.
This first command stays a limited kernel preview and does not claim `.punk/` runtime persistence.
