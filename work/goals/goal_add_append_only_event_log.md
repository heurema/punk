---
id: goal_add_append_only_event_log
title: "Add append-only event log"
status: done
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-21
selected_at: 2026-04-21
started_at: 2026-04-21
completed_at: 2026-04-21
blocked_by: []
scope:
  include:
    - "crates/punk-events/**"
    - "docs/product/TELEMETRY.md"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-eval/**"
acceptance:
  - "Event schema fixture exists."
  - "Append-only writer appends without mutating prior entries."
  - "Event writer leaves deterministic test evidence."
  - "cargo check --workspace passes."
  - "No .punk writes occur until explicitly scoped."
knowledge_refs:
  - "docs/product/TELEMETRY.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/ARCHITECTURE.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-21-append-only-event-log.md"
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

Add the minimal append-only event log only after the pure flow-state kernel exists.

## Notes

This goal is complete once the event schema fixture, append-only writer semantics, and deterministic test evidence exist without activating `.punk/` runtime paths.

Flow transitions still need a separate glue step before inspect or eval surfaces can rely on emitted event evidence.
