---
id: goal_add_smoke_eval_harness
title: "Add smoke eval harness"
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
    - "crates/punk-eval/**"
    - "evals/**"
    - "docs/product/EVAL.md"
    - "docs/product/EVAL-PLANE.md"
    - "work/**"
  exclude:
    - ".punk/**"
acceptance:
  - "Smoke eval harness exists."
  - "Initial smoke cases can exercise flow and event expectations."
  - "cargo check --workspace passes."
  - "No final decision writing is introduced."
knowledge_refs:
  - "docs/product/EVAL.md"
  - "docs/product/EVAL-PLANE.md"
  - "docs/product/ROADMAP.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-smoke-eval-harness.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "Touches the eval operator surface over the current flow/event inspect path."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "docs/product/ROADMAP.md"
  external_research_refs:
    - "work/reports/2026-04-22-research-gate-preflight.md"
    - "work/reports/2026-04-22-flow-inspect-command.md"
    - "work/reports/2026-04-22-smoke-eval-harness.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Start the smoke eval harness only after there is enough deterministic flow/event behavior to validate.

## Notes

This goal is complete once the library-first smoke harness can assess current flow and event kernels without activating `.punk/` runtime state, baseline tracking, waivers, or a broader eval platform.
CLI `punk eval run smoke` remains deferred because the selected goal stayed within `crates/punk-eval/**`.
