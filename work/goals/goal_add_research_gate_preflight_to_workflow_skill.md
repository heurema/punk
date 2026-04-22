---
id: goal_add_research_gate_preflight_to_workflow_skill
title: "Add Research Gate preflight to workflow skill"
status: done
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
selected_at: 2026-04-22
started_at: 2026-04-22
completed_at: 2026-04-22
blocked_by: []
scope:
  include:
    - "AGENTS.md"
    - ".agents/skills/punk-workflow/**"
    - "scripts/check_research_gate.py"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/**"
acceptance:
  - "AGENTS.md includes Research Gate preflight before meaningful repo work."
  - "The `punk-workflow` skill includes Step 0 Research Gate preflight."
  - "The goal template includes `research_gate` fields."
  - "A stdlib-only `scripts/check_research_gate.py` passes."
  - "The final ledger state restores `selected_next` to `work/goals/goal_add_flow_inspect_command.md`."
knowledge_refs:
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PROJECT-MEMORY.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-22-research-gate-preflight.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R0
  required: false
  rationale: "Implements the existing Research Gate policy as a workflow guardrail before the next CLI/operator-surface diff."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

Add Research Gate preflight to the repo workflow before continuing with `work/goals/goal_add_flow_inspect_command.md`.

## Notes

This goal temporarily interrupts the next implementation step to install a process guardrail, then restores `selected_next` to `work/goals/goal_add_flow_inspect_command.md`.
It does not supersede flow inspect.
