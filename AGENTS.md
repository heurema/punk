# AGENTS.md

## Punk repository rules

Before meaningful repo work:

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

Required checks:

- Always run `python3 scripts/check_work_ledger.py`.
- Run `cargo check --workspace` when Rust workspace or code changes.
- Run docs governance checks when product or process docs, reports, or work-ledger artifacts change.

For non-trivial repo work, use `$punk-workflow`.
