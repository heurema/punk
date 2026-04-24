---
name: punk-workflow
description: "Use when making a meaningful change in the Punk repository. Guides Codex through Punk bounded work discipline: run Research Gate preflight, read work/STATUS.md, follow selected_next, preserve scope, update goal/report/status, and run checks. Do not use for explanation-only questions."
---

# Punk Workflow Skill

## Purpose

Help Codex make safe, bounded changes in the Punk repository.

This skill is a workflow guide, not a source of product truth.

## Authority boundary

This skill is an advisory runner aid.

It may help an executor navigate Punk's workflow, but it must not:

- define product truth
- override Punk Laws
- write final decisions
- bypass gate
- promote parked capabilities
- treat model-specific behavior as architecture
- accumulate fixes without eval/failure evidence

Canonical truth lives in repo-tracked docs, ADRs, contracts, evals, decisions, proofpacks, and project memory.

## Step 0 — Research Gate preflight

Before editing, classify the task as exactly one of:

- `R0` — no research required
- `R1` — quick scan required
- `R2` — design research required
- `R3` — deep research required

Use `docs/product/RESEARCH-GATE.md` as the canonical policy.

If the task is `R1+`:

- use repo-tracked research or user-provided research when it already exists;
- cite that research in the goal or report;
- do not silently promote research to canonical product truth.

If the task is `R1+` and no adequate research is available:

- stop and ask the user for Deep Research before implementation;
- do not silently continue.

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

## Work Ledger Reviews

After several meaningful diffs, before phase promotion, after repeated human corrections, or after a process violation that was supposedly fixed, propose a Work Ledger Review.
Use `work/_templates/work-ledger-review.md`.
Keep the review advisory and choose at most one conservative next goal through the normal Work Ledger flow.

## Required checks

Always run:

```bash
python3 scripts/check_research_gate.py
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
