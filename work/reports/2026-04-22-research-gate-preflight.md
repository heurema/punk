---
id: report_2026_04_22_research_gate_preflight
goal_id: goal_add_research_gate_preflight_to_workflow_skill
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Goal

Add Research Gate preflight to the Punk workflow before continuing with `work/goals/goal_add_flow_inspect_command.md`.

## Why this interrupted selected_next

`work/goals/goal_add_flow_inspect_command.md` was already the next implementation goal after the flow/event glue diff.
This process diff temporarily interrupted that path because `flow inspect` touches a public CLI/operator surface and inspectable derived views.
The preflight does not supersede flow inspect.
It installs the guardrail first, then restores `selected_next` to `work/goals/goal_add_flow_inspect_command.md`.

## What changed

- Added Research Gate preflight guidance to `AGENTS.md`.
- Added Step 0 Research Gate preflight to `.agents/skills/punk-workflow/SKILL.md`.
- Extended `.agents/skills/punk-workflow/references/bounded-diff-checklist.md` with explicit `R0`/`R1`/`R2`/`R3` checks.
- Extended `work/_templates/goal.md` with a `research_gate` block.
- Added `scripts/check_research_gate.py` as a stdlib-only checker for the selected goal and any `in_progress` goal.
- Added `work/goals/goal_add_research_gate_preflight_to_workflow_skill.md` as the bounded process goal for this interrupt.
- Updated `work/goals/goal_add_flow_inspect_command.md` with explicit `research_gate` metadata while keeping it as the next implementation goal.
- Updated `work/STATUS.md` to record the completed preflight and keep `selected_next` on flow inspect.

## Research Gate

Classification: R0
Required: no
Rationale:
This diff implements the existing repo-tracked Research Gate policy from `docs/product/RESEARCH-GATE.md`.
No new product truth, external research synthesis, or architecture promotion is introduced.
Research considered:
- `docs/product/RESEARCH-GATE.md`
Adopted now:
- every meaningful repo task is classified as `R0`, `R1`, `R2`, or `R3` before editing;
- `R1+` work must cite repo-tracked or user-provided research or stop for Deep Research;
- the selected goal carries explicit `research_gate` metadata before future CLI/operator-surface work.
Deferred:
- Research Gate workflow automation outside the repo skill and local checker;
- broader backfill of `research_gate` metadata across legacy goals;
- any Rust, CLI, `.punk`, eval, or gate/proof implementation.
Decision:
Proceed with this bounded process diff and restore `selected_next` to `work/goals/goal_add_flow_inspect_command.md`.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Adds Research Gate preflight to the repo workflow before the first CLI/operator-surface implementation diff."
  touched_surfaces:
    - project-memory
  required_updates:
    - AGENTS.md
    - .agents/skills/punk-workflow/SKILL.md
    - .agents/skills/punk-workflow/references/bounded-diff-checklist.md
    - work/_templates/goal.md
    - scripts/check_research_gate.py
    - work/STATUS.md
    - work/goals/goal_add_flow_inspect_command.md
    - work/goals/goal_add_research_gate_preflight_to_workflow_skill.md
    - work/reports/2026-04-22-research-gate-preflight.md
  supersedes: []
  archive_plan: []
  evals_required:
    - research-gate-check
    - work-ledger-checks
    - docs-governance
```

## Checks run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-research-gate-preflight.md --report work/reports/2026-04-22-research-gate-preflight.md`

## Result

- The repo workflow now requires Research Gate classification before meaningful edits.
- The selected goal can carry explicit `research_gate` metadata without changing the Work Ledger SSoT.
- `flow inspect` remains the next implementation goal after the process diff completes.
- No Rust code, `.punk` runtime state, CLI behavior, or eval harness behavior was added.

## Out of scope confirmation

This diff does not implement:

- `punk flow inspect`
- Rust code changes
- `.punk/` runtime activation
- smoke eval harness behavior
- gate or proof runtime
- external research ingestion or product-doc rewrites

## Next recommended goal

`work/goals/goal_add_flow_inspect_command.md`
