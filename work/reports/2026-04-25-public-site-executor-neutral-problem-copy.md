---
id: report_2026_04_25_public_site_executor_neutral_problem_copy
goal_id: goal_public_site_executor_neutral_problem_copy
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Finish the small public landing copy cleanup by replacing the remaining agent-first problem-section wording with executor-neutral language.

## Research Gate

Classification: R1
Required: yes
Rationale:
Public copy affects external trust claims, but this is a bounded follow-up using existing canonical public narrative and executor-boundary docs. No Deep Research is required.

Research refs:

- `docs/product/PUBLIC-NARRATIVE.md`
- `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`
- `evals/specs/executor-agnostic-validation-boundary.v0.1.md`
- `work/reports/2026-04-24-public-site-executor-neutral-copy.md`

Decision:
Proceed with a narrow landing problem-section copy patch only.

## Changed files

- `site/src/components/Problem.astro`
- `work/goals/goal_public_site_executor_neutral_problem_copy.md`
- `work/reports/2026-04-25-public-site-executor-neutral-problem-copy.md`
- `work/STATUS.md`

## What changed

- Replaced `A model can write the patch...` with executor-neutral copy.
- Replaced `AI agents are powerful` with executor-neutral copy.
- Replaced the problem checklist phrase `gate automated` with `gate checked` to avoid implying active gate automation beyond current public-copy scope.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no `.punk/` runtime state written;
- no runtime behavior changed;
- no schema, receipt field, missing-validator policy, semantic assessor interface, provider adapter, model runner, prompt manager, or skill manager added;
- historical journal entries remain unchanged;
- agents remain allowed as a use case but not as required architecture.

## Work ledger note

`work/STATUS.md` was updated only to record this user-requested public-copy correction and the latest validation command.

`selected_next` intentionally remains `work/goals/goal_extract_goalrail_process_shell_pilot.md`.

## Doc impact

```yaml
doc_impact:
  classification: public-claim
  reason: "Public landing copy was aligned with existing executor-agnostic validation boundary docs without changing canonical product docs."
  touched_surfaces:
    - site/src/components/Problem.astro
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next remains `work/goals/goal_extract_goalrail_process_shell_pilot.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next remains `work/goals/goal_extract_goalrail_process_shell_pilot.md`; goals checked: 47.
- `scripts/check.sh docs-governance --files site/src/components/Problem.astro work/goals/goal_public_site_executor_neutral_problem_copy.md work/reports/2026-04-25-public-site-executor-neutral-problem-copy.md work/STATUS.md --report work/reports/2026-04-25-public-site-executor-neutral-problem-copy.md` - PASS; changed files: 4; canonical docs checked: 0; reports checked: 1; failures: 0; warnings: 0.
- `npm --prefix site run build` - PASS; 2 pages built.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute repo path leaks.

`cargo check --workspace` was not run because this patch does not touch Rust workspace files.

## Warnings / legacy warnings

Docs governance emitted 0 failures and 0 warnings for this patch.
