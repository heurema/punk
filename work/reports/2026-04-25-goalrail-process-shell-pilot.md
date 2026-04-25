---
id: report_2026_04_25_goalrail_process_shell_pilot
goal_id: goal_extract_goalrail_process_shell_pilot
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Goal

Extract a reusable process-only shell from Punk's Dogfooding Level 0 work-ledger discipline for a narrow GoalRail pilot, while protecting Punk's active-core runtime scope.

## Research Gate

Classification: R1
Required: yes
Rationale:
The pilot reuses existing repo-tracked process evidence and canonical Punk docs. It does not change architecture, runtime, CLI, schemas, active-core crate scope, or product truth, so no Deep Research was required.

Research refs:

- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/DOGFOODING.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/PUBLIC-NARRATIVE.md`
- `work/STATUS.md`
- `work/goals/goal_extract_goalrail_process_shell_pilot.md`
- `work/reports/2026-04-22-fourth-work-ledger-review.md`
- `work/reports/2026-04-24-execution-agnostic-contract-boundary.md`
- `work/reports/2026-04-24-executor-agnostic-validation-boundary-refinement.md`
- `work/reports/2026-04-25-executor-discipline-aids.md`

Decision:
Proceed with a process-only work artifact and work-ledger updates.

## Changed files

- `work/pilots/goalrail-process-shell.md`
- `work/goals/goal_extract_goalrail_process_shell_pilot.md`
- `work/goals/goal_research_task_storage_before_project_memory.md`
- `work/reports/2026-04-25-goalrail-process-shell-pilot.md`
- `work/STATUS.md`

## What changed

- Added `work/pilots/goalrail-process-shell.md` as the reusable GoalRail process-shell pilot artifact.
- Added the setup-neutral / ambient-tool guardrail: the shell must not require users to change IDE, CLI ritual, model, provider, prompt, skill, or local runtime setup.
- Closed `work/goals/goal_extract_goalrail_process_shell_pilot.md` with a report ref and outcome note.
- Promoted `work/goals/goal_research_task_storage_before_project_memory.md` from `draft` to `ready` and added a Research Gate block so it can be selected safely.
- Updated `work/STATUS.md` so the single live ledger now selects the task/work storage research goal.

## GoalRail process shell summary

The GoalRail process shell means a manual, repo-tracked work loop:

```text
goal -> research gate -> bounded plan -> manual/executor run -> report/evidence -> work-ledger review -> select next
```

Reusable Punk process artifacts:

- `AGENTS.md`-style advisory guidance;
- one `work/STATUS.md`-style live status file;
- `work/goals/`-style durable goal artifacts;
- `work/reports/`-style handoff and evidence artifacts;
- Research Gate classification;
- Work Ledger Review loop;
- executor discipline aids;
- evidence-vs-authority boundary;
- setup-neutral ambient operation.

Explicitly not included:

- `.punk/` runtime;
- Punk CLI;
- Event Ledger implementation;
- gate implementation;
- proofpack writer;
- modules/adapters;
- provider/model runner;
- required IDE, CLI ritual, prompt, skill, model, provider, or local runtime setup;
- semantic assessor command interface;
- schema changes;
- automation;
- GoalRail productization;
- a second tracker.

## Manual pilot flow

1. Create one GoalRail-local goal with scope, exclusions, acceptance criteria, and source refs.
2. Classify Research Gate as R0/R1/R2/R3 before editing.
3. Produce a bounded plan naming one intended diff, non-goals, checks, and rollback/deferral path.
4. Execute manually or with an executor while treating the executor as substrate, not authority.
5. Record a report with changed artifacts, evidence, checks, failures, and open follow-ups.
6. Run advisory Work Ledger Review when enough evidence accumulates or process drift appears.
7. Select the next ready goal in the single live status file.

## Guardrails

- One project, one live ledger.
- No hidden source of truth.
- Ambient UX does not mean hidden authority or hidden state.
- Executor claims are not proof.
- Advisory reviews do not decide.
- Gate/proof terms must not imply runtime gate/proof implementation.
- No required change to the user's IDE, CLI ritual, model, provider, prompt, skill, or local runtime setup.
- No `.punk/`, CLI, storage, schema, adapter, module, provider, model runner, or automation work is included.
- GoalRail pilot state must live in GoalRail, not Punk, if/when the pilot is applied there.

## `punk init` mismatch

Observed: `docs/product/START-HERE.md` lists `punk init` under the active target.

Current CLI source exposes only:

- `punk flow inspect`;
- `punk eval run smoke`;
- `punk eval run smoke --format json`.

This patch does not fix the mismatch because `docs/product/**` and CLI changes are outside the selected goal scope.

Record as separate docs/CLI mismatch follow-up.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no schema changed;
- no `.punk/` runtime state written;
- no public CLI behavior changed;
- no user setup, IDE, model, provider, prompt, skill, or runtime workflow requirement added;
- no `punk init` command added;
- no gate implementation added;
- no proofpack writer added;
- no runtime Event Ledger added;
- no provider/model/agent adapter added;
- no automation added;
- no second tracker added;
- no product-doc promotion made.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "Process-only work artifacts and work-ledger metadata changed; no canonical product docs, schemas, runtime code, public claims, or architecture truth changed."
  touched_surfaces:
    - work/pilots/goalrail-process-shell.md
    - work/goals/goal_extract_goalrail_process_shell_pilot.md
    - work/goals/goal_research_task_storage_before_project_memory.md
    - work/reports/2026-04-25-goalrail-process-shell-pilot.md
    - work/STATUS.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS, no output.
- `python3 scripts/check_research_gate.py` - PASS; selected next is `work/goals/goal_research_task_storage_before_project_memory.md`.
- `python3 scripts/check_work_ledger.py` - PASS; selected next is `work/goals/goal_research_task_storage_before_project_memory.md`; goals checked: 47.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_extract_goalrail_process_shell_pilot.md work/goals/goal_research_task_storage_before_project_memory.md work/pilots/goalrail-process-shell.md work/reports/2026-04-25-goalrail-process-shell-pilot.md --report work/reports/2026-04-25-goalrail-process-shell-pilot.md` - PASS; changed files: 5; canonical docs checked: 0; reports checked: 1; failures: 0; warnings: 0.
- `cargo test --workspace` - PASS; 53 Rust tests passed.
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS; no absolute local path refs found.

Additional mismatch probe:

- `cargo run -q -p punk-cli -- init` - observed usage error, exit 2; confirms `punk init` is not currently implemented.

## Open follow-ups

- Resolve `punk init` docs/CLI mismatch in a separate bounded goal.
- Define missing-validator policy.
- Define minimal receipt fields.
- Define semantic assessor command interface.
- Implement real gate runtime only through a later bounded goal.
- Implement real proofpack runtime only through a later bounded goal.
- Reconcile proofpack minimum metadata and `proof before acceptance` semantics before gate/proof runtime work.
- Keep runtime Event Ledger deferred until repeated inspectability evidence justifies it.

## What remains

The next selected goal is `work/goals/goal_research_task_storage_before_project_memory.md`.

## Risks

- GoalRail could accidentally treat the process shell as product/runtime truth; the pilot artifact explicitly blocks that.
- A second tracker could appear if GoalRail mirrors state in multiple places; the guardrail is one project, one live ledger.
- Gate/proof vocabulary could overclaim current implementation; the pilot keeps those terms future/runtime-only unless separately implemented.

## Knowledge updates needed

- Save a project memory observation that the GoalRail shell is process-only and excludes runtime/CLI/storage/gate/proof/adapters/automation.
