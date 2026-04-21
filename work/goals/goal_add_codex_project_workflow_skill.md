---
id: goal_add_codex_project_workflow_skill
title: "Add Codex project workflow skill"
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
    - "AGENTS.md"
    - ".agents/skills/punk-workflow/**"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/**"
acceptance:
  - "Root AGENTS.md exists and stays short."
  - "The repo-scoped `punk-workflow` skill exists under `.agents/skills/`."
  - "The skill is instruction-first and reference-based."
  - "The skill tells Codex to read `work/STATUS.md` first."
  - "The skill tells Codex to follow `selected_next` unless the user explicitly overrides scope."
  - "The skill requires goal, report, and status updates for meaningful work."
  - "The skill requires `python3 scripts/check_work_ledger.py` and conditional repo checks."
  - "The final ledger state restores `selected_next` to `work/goals/goal_add_append_only_event_log.md`."
knowledge_refs:
  - "docs/product/START-HERE.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/DOGFOODING.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-21-codex-project-workflow-skill.md"
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

Add a thin repo-scoped Codex workflow skill before expanding the append-only event log trust surface.

## Notes

This goal is process-only. It temporarily interrupts the event-log path to install developer workflow guidance, then restores `selected_next` to `work/goals/goal_add_append_only_event_log.md` for the next code diff.
