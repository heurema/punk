---
id: report_2026_04_24_public_site_executor_neutral_copy
goal_id: goal_public_site_executor_neutral_copy
actor: vitaly
created_at: 2026-04-24
kind: handoff
---

## Goal

Make public site copy align with the executor-agnostic validation boundary without changing runtime, schema, receipt, validator, or semantic assessor behavior.

## Research Gate

Classification: R1
Required: yes
Rationale:
Public copy affects external claims, but the needed canonical context is already available in the public narrative docs, ADR-0014, and the executor-boundary eval spec.

Research refs:

- `docs/product/PUBLIC-NARRATIVE.md`
- `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`
- `evals/specs/executor-agnostic-validation-boundary.v0.1.md`

Decision:
Proceed with a bounded site-copy patch only.

## What changed

- Replaced hero copy `Agents do the work` with `Work happens in your runtime`.
- Replaced lifecycle `run` subtitle `agents on a leash` with `executor under contract`.
- Replaced DevPunk copy `Agents on your repo...` with `Bring-your-own executor on your repo...`.
- Replaced guide example `run an agent under it` with `run your chosen executor under it`.
- Replaced a receipt mock label from `agents` to `executors`.
- Updated the default site title from `bounded work kernel for agents` to `bounded work kernel for local work`.
- Adjusted the site law copy so gate records decisions from validated evidence instead of saying deterministic checks decide.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no `.punk/` runtime state written;
- no runtime behavior changed;
- no contract schema, receipt schema, missing-validator policy, or semantic assessor interface added;
- no provider adapter, model runner, prompt manager, or skill manager added;
- agents remain allowed as a use case but not as required architecture.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Public site copy was aligned with existing executor-agnostic validation boundary docs without changing canonical product docs."
  touched_surfaces:
    - site/src/components/Hero.astro
    - site/src/components/HowSection.astro
    - site/src/components/ModulesSection.astro
    - site/src/data/content.ts
    - site/src/layouts/Layout.astro
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files site/src/components/Hero.astro site/src/components/HowSection.astro site/src/components/ModulesSection.astro site/src/data/content.ts site/src/layouts/Layout.astro work/goals/goal_public_site_executor_neutral_copy.md work/reports/2026-04-24-public-site-executor-neutral-copy.md work/STATUS.md --report work/reports/2026-04-24-public-site-executor-neutral-copy.md`
- `npm --prefix site run build`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true`
- `git diff --name-only`

Result: PASS.

Docs governance emitted 0 failures and 0 warnings.
