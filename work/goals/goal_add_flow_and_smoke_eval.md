---
id: goal_add_flow_and_smoke_eval
title: "Add flow controller and smoke eval foundation"
status: draft
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-18
scope:
  include:
    - "crates/punk-flow/**"
    - "crates/punk-eval/**"
    - "flows/**"
    - "evals/**"
    - "docs/product/FLOW.md"
    - "docs/product/EVAL.md"
  exclude:
    - "crates/punk-mod-dev/**"
    - "crates/punk-mod-pub/**"
    - "crates/punk-adapters/**"
acceptance:
  - "Illegal transitions are denied."
  - "Next allowed command is inspectable."
  - "Smoke eval suite can run."
  - "Every transition writes an event."
knowledge_refs:
  - "docs/product/FLOW.md"
  - "docs/product/EVAL.md"
contract_refs: []
latest_proof_ref: null
---

## Context

Build Phase 1 and Phase 2 foundation after Phase 0 skeleton is stable.
