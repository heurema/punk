---
id: goal_connect_flow_transitions_to_event_log
title: "Connect flow transitions to event log"
status: ready
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-21
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-flow/**"
    - "crates/punk-events/**"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-cli/**"
    - "crates/punk-eval/**"
acceptance:
  - "Flow transition attempts can be recorded through the append-only event log kernel."
  - "Allowed and denied flow transitions leave deterministic event evidence in unit tests."
  - "The integration stays library-first and does not activate `.punk/` runtime paths."
  - "cargo check --workspace passes."
knowledge_refs:
  - "docs/product/FLOW.md"
  - "docs/product/TELEMETRY.md"
  - "docs/product/ROADMAP.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

The append-only event log kernel now exists, but flow transitions do not yet emit events through it.

## Notes

This glue goal keeps the next code diff honest before `punk flow inspect` or smoke eval work tries to rely on event evidence.
