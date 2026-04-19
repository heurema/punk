---
id: goal_research_module_host_prior_art
title: "Research module/extension host prior art before module host implementation"
status: draft
owner: "vitaly"
module: "core"
priority: P2
authority: canonical
created_at: 2026-04-18
updated_at: 2026-04-18
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
contract_refs: []
latest_proof_ref: null
---

## Context

Do not implement module host before researching stable extension boundaries and known failure modes.
