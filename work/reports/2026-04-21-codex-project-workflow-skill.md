---
id: report_2026_04_21_codex_project_workflow_skill
goal_id: goal_add_codex_project_workflow_skill
actor: vitaly
created_at: 2026-04-21
kind: handoff
---

## Goal

Add a thin repo-scoped Codex workflow skill before extending the append-only event log trust surface.

## What changed

- Added a short root `AGENTS.md` with the always-on Punk repo rules for bounded work.
- Added `.agents/skills/punk-workflow/SKILL.md` as a thin workflow skill for meaningful repo changes.
- Added reference files for canonical docs, bounded diff checks, and current phase scope.
- Added `agents/openai.yaml` metadata for explicit skill invocation.
- Added `work/goals/goal_add_codex_project_workflow_skill.md` as a temporary process override before the event log work, then restored the final ledger state to `work/goals/goal_add_append_only_event_log.md`.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Adds a thin repo-scoped Codex workflow skill and records it in the Work Ledger before the next trust-surface code diff."
  touched_surfaces:
    - project-memory
  required_updates:
    - AGENTS.md
    - .agents/skills/punk-workflow/SKILL.md
    - .agents/skills/punk-workflow/agents/openai.yaml
    - .agents/skills/punk-workflow/references/canonical-docs.md
    - .agents/skills/punk-workflow/references/bounded-diff-checklist.md
    - .agents/skills/punk-workflow/references/phase-scope.md
    - work/STATUS.md
    - work/goals/goal_add_codex_project_workflow_skill.md
    - work/reports/2026-04-21-codex-project-workflow-skill.md
  supersedes: []
  archive_plan: []
  evals_required:
    - work-ledger-checks
    - docs-governance
```

## Checks run

- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files work/reports/2026-04-21-codex-project-workflow-skill.md --report work/reports/2026-04-21-codex-project-workflow-skill.md`

## Result

- The repo now has an always-on `AGENTS.md` reminder to read `work/STATUS.md`, follow `selected_next`, and keep one bounded diff.
- The repo now has an explicit `punk-workflow` skill for future meaningful changes.
- The skill stays thin and points back to canonical docs instead of duplicating architecture.
- `selected_next` remains `work/goals/goal_add_append_only_event_log.md` after the skill diff completes.

## Out of scope confirmation

This diff does not implement:

- append-only event writing
- `.punk/` runtime state
- Rust code changes
- plugin, adapter, or module-host runtime
- product doc rewrites

## Next recommended goal

`work/goals/goal_add_append_only_event_log.md`
