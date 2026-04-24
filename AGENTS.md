# AGENTS.md

## Operator aid, not project truth

This file is a lightweight aid for local coding agents.

It is not a source of Punk product truth, architecture truth, final decisions, or acceptance criteria.

When this file conflicts with canonical docs, the canonical docs win:

1. `docs/product/START-HERE.md`
2. `docs/product/PUNK-LAWS.md`
3. `docs/product/ARCHITECTURE.md`
4. `docs/product/ROADMAP.md`
5. `docs/product/CRATE-STATUS.md`
6. `docs/product/RESEARCH-GATE.md`
7. `docs/product/PROJECT-MEMORY.md`

Do not turn repeated executor failures into new global instructions automatically.
Prefer contract clauses, validators, eval cases, proof requirements, and project-memory artifacts.

## Punk repository rules

Before meaningful repo work:

0. Run Research Gate preflight: classify the task as `R0`, `R1`, `R2`, or `R3` before editing.
1. Read `work/STATUS.md`.
2. Follow `selected_next` unless the user explicitly changes scope.
3. Read the selected goal under `work/goals/`.
4. Keep one bounded diff per task.
5. Do not mix unrelated cleanup with the selected goal.
6. Update the relevant goal, report, and `work/STATUS.md` for meaningful changes.
7. Use repo-relative paths only in repo-tracked artifacts.
8. Do not write `.punk/` runtime state unless explicitly scoped.
9. Do not expose parked or future capabilities as active behavior.
10. Only future `gate` writes final acceptance; Level 0 `done` is manual closure with evidence.
11. If a task is `R1+` and repo-tracked or user-provided research is not available, stop and ask the user for Deep Research instead of silently implementing.

Required checks:

- Always run `python3 scripts/check_research_gate.py`.
- Always run `python3 scripts/check_work_ledger.py`.
- Run `cargo check --workspace` when Rust workspace or code changes.
- Run docs governance checks when product or process docs, reports, or work-ledger artifacts change.

For non-trivial repo work, use `$punk-workflow`.
