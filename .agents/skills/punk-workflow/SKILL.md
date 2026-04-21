---
name: punk-workflow
description: "Use when making a meaningful change in the Punk repository. Guides Codex through Punk bounded work discipline: read work/STATUS.md, follow selected_next, preserve scope, update goal/report/status, and run checks. Do not use for explanation-only questions."
---

# Punk Workflow Skill

## Purpose

Help Codex make safe, bounded changes in the Punk repository.

This skill is a workflow guide, not a source of product truth.

## Start every meaningful task

1. Read `work/STATUS.md`.
2. Identify `selected_next`, current focus, blocked items, and recent evidence.
3. Read the selected goal under `work/goals/`.
4. Load canonical docs only as needed.
5. State the intended bounded diff before editing.

Use `references/canonical-docs.md` when you need to know which repo document owns a truth.
Use `references/phase-scope.md` when the selected goal touches phase boundaries or future surfaces.
Use `references/bounded-diff-checklist.md` before staging and again before finishing.

## Hard boundaries

Do not:

- create a second live work-state surface;
- add `work/NEXT.md`;
- use absolute local paths in repo artifacts;
- mix unrelated cleanup with the selected goal;
- write `.punk/` runtime state unless explicitly scoped;
- expose parked or future capabilities as active behavior;
- implement plugins, adapters, module host, Knowledge Vault, or PubPunk unless the selected goal explicitly scopes it;
- treat Level 0 `done` as future `gate` acceptance.

## Required artifacts

For meaningful work, update:

- the selected goal;
- `work/STATUS.md`;
- a report under `work/reports/`.

Keep the skill thin: reference canonical docs instead of copying architecture into the skill.

## Required checks

Always run:

```bash
python3 scripts/check_work_ledger.py
```

Run when Rust workspace or code changed:

```bash
cargo check --workspace
```

Run docs governance checks when product or process docs, reports, or work-ledger artifacts changed.

## Completion response

Report:

- files changed;
- checks run and result;
- scope boundaries preserved;
- ledger or report updates;
- next selected goal;
- anything intentionally deferred.
