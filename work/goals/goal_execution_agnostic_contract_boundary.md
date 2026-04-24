---
id: goal_execution_agnostic_contract_boundary
title: "Adopt initial Contract over Prompt boundary"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-24
updated_at: 2026-04-24
selected_at: 2026-04-24
started_at: 2026-04-24
completed_at: 2026-04-24
blocked_by: []
scope:
  include:
    - "knowledge/research/**"
    - "docs/adr/**"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/CRATE-STATUS.md"
    - "AGENTS.md"
    - ".agents/skills/punk-workflow/SKILL.md"
    - "evals/specs/**"
    - "work/goals/**"
    - "work/reports/**"
    - "work/STATUS.md"
  exclude:
    - "crates/**"
    - "Cargo.toml"
    - "Cargo.lock"
    - ".github/workflows/**"
    - "site/**"
    - "brand/**"
    - ".punk/**"
acceptance:
  - "Executor-agnostic validation boundary is promoted as documentation, ADR, Research Gate, and eval-policy boundary only."
  - "Punk remains provider/model independent and does not gain a runner, provider adapter, prompt manager, skill manager, MCP integration, or public CLI command."
  - "Prompts, skills, playbooks, AGENTS.md, local memories, and model/provider settings are advisory runner aids, not project truth."
  - "Executor output remains evidence only after receipt/eval/gate/proof linkage."
  - "No Rust code, `.punk/`, runtime storage, or provider integration is introduced."
knowledge_refs:
  - "knowledge/research/2026-04-24-contract-over-prompt.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/PROJECT-MEMORY.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-24-execution-agnostic-contract-boundary.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by:
  - "work/goals/goal_refine_executor_agnostic_validation_boundary.md"
research_gate:
  classification: R2
  required: true
  rationale: "This changes core laws, architecture, roadmap phase boundaries, Research Gate policy, project memory, and runner-aid authority boundaries. User-provided research and repo canonical docs are available, so no Deep Research escalation is needed."
  research_refs:
    - "knowledge/research/2026-04-24-contract-over-prompt.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/PROJECT-MEMORY.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/product/CRATE-STATUS.md"
  rationale: "The diff promotes a researched executor/model boundary into canonical product docs and supporting advisory artifacts."
---

## Context

The user explicitly changed scope from the previously selected GoalRail process-shell pilot to adopt the researched `Contract over Prompt` idea now, but only as an executor-agnostic validation boundary.

The corrected boundary is:

```text
Punk is executor-agnostic, not validation-agnostic.
Punk does not own the executor.
Punk owns contract, validation protocol, evidence, gate, proof, and memory.
```

## Notes

This goal is complete:

- research note saved under `knowledge/research/`;
- ADR-0014 created with `Status: Proposed` and later renamed to `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`;
- eval spec created for executor-agnostic validation docs/policy consistency;
- canonical docs updated without changing runtime code;
- AGENTS and punk-workflow skill now state they are advisory runner aids;
- no new active-core crate, CLI command, provider adapter, or `.punk/` runtime state was added.

The interrupted `selected_next` remains `work/goals/goal_extract_goalrail_process_shell_pilot.md` after this manual closure.
