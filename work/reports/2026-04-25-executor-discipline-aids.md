---
id: report_2026_04_25_executor_discipline_aids
goal_id: goal_adopt_executor_discipline_aids
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Adopt useful Karpathy-style executor discipline ideas as Punk advisory documentation, examples, and eval-policy artifacts without adding runtime, schema, CLI, provider, prompt, or skill authority.

## Research Gate

Classification: R1
Required: yes
Rationale:
The patch adapts external prompt/skill prior art and touches eval-policy and active-core/future boundary wording. The sources are bounded and directly relevant, so no Deep Research is required.

Research refs:

- `knowledge/research/2026-04-25-karpathy-skills-review.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`

External refs:

- `https://github.com/forrestchang/andrej-karpathy-skills`
- `https://docs.anthropic.com/en/docs/claude-code/skills`
- `https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills`

Decision:
Adopt ideas, avoid authority model.

## Changed files

- `knowledge/research/2026-04-25-karpathy-skills-review.md`
- `examples/executor-briefs/punk-default.md`
- `examples/contracts/task-transformations.md`
- `examples/punk-workflows/karpathy-style-failures.md`
- `evals/specs/surgical-change-discipline.v0.1.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/DOGFOODING.md`
- `work/goals/goal_adopt_executor_discipline_aids.md`
- `work/reports/2026-04-25-executor-discipline-aids.md`

## What was added

- A research note with source-quality table, adoption classification, and decision to adopt ideas but avoid prompt/skill authority.
- A provider-neutral default executor brief template.
- Task-to-contract transformation examples for vague requests.
- Bad/good workflow examples for common executor failures.
- A surgical-change discipline eval-policy spec marked not runtime-enforced yet.
- Small product-doc notes keeping executor aids advisory and future brief generation deferred.

## What was deliberately not added

- no runtime files;
- no schema files;
- no new CLI command;
- no active `punk brief`;
- no provider/model/tool-specific dependency;
- no prompt manager or skill manager;
- no skill install/export path;
- no model runner;
- no semantic assessor command interface;
- no missing-validator policy;
- no public site copy change;
- no new source of truth.

## Work ledger note

`work/STATUS.md` was intentionally not edited because this contract used strict allowed-path scope.

The work trail is captured in:

- `work/goals/goal_adopt_executor_discipline_aids.md`
- `work/reports/2026-04-25-executor-discipline-aids.md`

`selected_next` was intentionally left unchanged.

## Scope boundaries preserved

- Executor aids are explicitly advisory.
- Executor claims remain non-proof.
- Semantic assessors remain advisory.
- Gate remains final decision authority.
- The new eval spec is docs/eval-policy only and not runtime-enforced yet.
- `andrej-karpathy-skills` is inspiration only, not a dependency or canonical skill.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Adds advisory executor-aid examples and eval-policy guidance while preserving the executor-agnostic validation boundary and active-core/future split."
  touched_surfaces:
    - knowledge/research/2026-04-25-karpathy-skills-review.md
    - examples/executor-briefs/punk-default.md
    - examples/contracts/task-transformations.md
    - examples/punk-workflows/karpathy-style-failures.md
    - evals/specs/surgical-change-discipline.v0.1.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/DOGFOODING.md
  required_updates:
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/DOGFOODING.md
  supersedes: []
  archive_plan: []
  evals_required:
    - evals/specs/surgical-change-discipline.v0.1.md
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next remains `work/goals/goal_extract_goalrail_process_shell_pilot.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next remains `work/goals/goal_extract_goalrail_process_shell_pilot.md`; goals checked: 46.
- `scripts/check.sh docs-governance --files docs/product/ARCHITECTURE.md docs/product/DOGFOODING.md docs/product/RESEARCH-GATE.md docs/product/ROADMAP.md evals/specs/surgical-change-discipline.v0.1.md examples/contracts/task-transformations.md examples/executor-briefs/punk-default.md examples/punk-workflows/karpathy-style-failures.md knowledge/research/2026-04-25-karpathy-skills-review.md work/goals/goal_adopt_executor_discipline_aids.md work/reports/2026-04-25-executor-discipline-aids.md --report work/reports/2026-04-25-executor-discipline-aids.md` - PASS; changed files: 11; canonical docs checked: 4; reports checked: 1; failures: 0; warnings: 5.
- `cargo check --workspace` - PASS; finished `dev` profile.
- `cargo run -q -p punk-cli -- eval run smoke` - PASS; `smoke_result: pass`; no `.punk/evals` runtime state read or written.

## Warnings / legacy warnings

Docs governance emitted 5 warning-level findings and 0 failures:

- 4 pre-existing duplicate-definition candidate warnings in `docs/product/ARCHITECTURE.md`: `Core ownership`, `Workspace activation model`, `Module host boundary`, `Minimal proofpack provenance`.
- 1 pre-existing duplicate-definition candidate warning in `docs/product/RESEARCH-GATE.md`: `Research intake triggers`.

The new `Executor aids` glossary warning from the first pass was resolved by rewording the heading as non-canonical task-scoped advisory handoff language.

## Open follow-ups

- public/site wording patch remains separate;
- missing-validator policy remains separate;
- minimal receipt fields remain separate;
- semantic assessor command interface remains separate;
- executor brief hash policy remains separate;
- formal generated briefs from `plot` remain separate.
