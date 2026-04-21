---
id: goal_research_module_host_prior_art
title: "Research module/extension host prior art before module host implementation"
status: done
owner: "vitaly"
module: "core"
priority: P2
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-21
selected_at: null
started_at: null
completed_at: 2026-04-19
blocked_by: []
scope:
  include:
    - "knowledge/research/**"
    - "docs/product/MODULES.md"
    - "docs/adr/**"
  exclude:
    - "crates/punk-module-host/**"
acceptance:
  - "Research note compares browser extensions, editor extensions, MCP/tool adapters, and local plugin models."
  - "Research note distinguishes module API from adapter API."
  - "Required module conformance evals are listed."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "knowledge/research/2026-04-19-wasm-plugin-module-host-r1.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Do not implement module host before researching stable extension boundaries and known failure modes.
