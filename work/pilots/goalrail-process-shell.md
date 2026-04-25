---
id: work_pilot_goalrail_process_shell
kind: process-pilot
status: advisory
authority: advisory
owner: vitaly
created_at: 2026-04-25
updated_at: 2026-04-25
source_goal: work/goals/goal_extract_goalrail_process_shell_pilot.md
source_report: work/reports/2026-04-25-goalrail-process-shell-pilot.md
---

# GoalRail Process Shell Pilot

## Purpose

This artifact extracts the reusable part of Punk's current dogfooding discipline for a narrow GoalRail process-only pilot.

It is an advisory handoff, not product truth, not runtime state, and not a second live tracker.

## Definition

A GoalRail process shell is a manual, repo-tracked work loop that helps one project keep bounded work inspectable before runtime automation exists.

It reuses Punk's Level 0 discipline:

```text
goal -> research gate -> bounded plan -> manual/executor run -> report/evidence -> work-ledger review -> select next
```

The shell owns process shape only. It does not port Punk's CLI, runtime state, event ledger, gate, proofpack, modules, adapters, provider runner, or model behavior.

It is setup-neutral and ambient: the user should keep working where they already work, while the shell quietly adds boundaries, evidence capture, and review memory around that workflow.

Ambient does not mean hidden. Low-friction UX must not create hidden authority, hidden state, or an uninspectable source of truth.

## Source evidence

The pilot is based on repo-tracked Punk evidence:

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

## Reusable minimum set

### Advisory project guidance

GoalRail may use an `AGENTS.md`-style local guidance file to orient human or model executors.

Rules:

- advisory only;
- no hidden authority;
- no acceptance criteria that bypass the selected goal;
- no provider/model-specific behavior as project truth;
- stale or repeated failures become goal clauses, validators, report evidence, eval cases, or review findings before they become global instructions.

### One live status file

GoalRail may use one project-local status file equivalent to Punk's `work/STATUS.md`.

Minimum fields:

- current focus;
- selected next goal;
- why it is next;
- blockers;
- recently completed work with evidence refs;
- latest validation state.

Guardrail: one project gets one live ledger. Do not mirror GoalRail's live state in Punk, and do not create a second live status file inside GoalRail.

### Goals directory

GoalRail may use a `work/goals/`-style directory for durable work intent.

Each goal should record:

- owner;
- status;
- scope include/exclude lists;
- acceptance criteria;
- Research Gate classification;
- refs to source evidence;
- report refs after completion.

### Reports directory

GoalRail may use a `work/reports/`-style directory for durable outcome and handoff artifacts.

Each report should record:

- goal;
- Research Gate result;
- changed files or changed artifacts;
- evidence produced;
- checks run;
- boundaries preserved;
- open follow-ups;
- next-step recommendation when relevant.

### Research Gate

GoalRail may reuse Punk's R0/R1/R2/R3 preflight shape:

- R0: no research required;
- R1: bounded quick scan;
- R2: design research;
- R3: deep research.

Research is advisory until promoted through the project's own goal/report/decision path.

If an R1+ GoalRail task lacks repo-tracked or user-provided research, stop and request research instead of inventing architecture from executor memory.

### Work Ledger Review loop

GoalRail may run advisory Work Ledger Reviews after several meaningful diffs, repeated corrections, blocked branches, or phase-promotion pressure.

A review may recommend one conservative next goal, but it does not decide and does not create a second live source of truth.

### Executor discipline aids

GoalRail may adapt Punk's executor discipline aids as examples and checklists:

- make assumptions explicit before edits;
- keep diffs surgical;
- prefer simple solutions;
- record checks and evidence;
- separate executor claims from verified evidence.

These aids remain runner aids. They are not contracts, validators, gate decisions, proofpacks, or product truth.

### Evidence vs authority boundary

GoalRail should preserve Punk's boundary:

- executor output is evidence;
- executor self-report is a claim;
- claims such as "done", "tests passed", or "scope preserved" are not proof;
- project authority comes from the selected goal, recorded evidence, review, and later any real gate/proof mechanism that GoalRail explicitly implements.

### Setup-neutral and ambient operation

GoalRail may use the shell without changing the user's preferred work setup.

The shell must not prescribe:

- IDE;
- terminal ritual;
- model;
- provider;
- prompt system;
- skill system;
- local runtime shape;
- repository-specific execution workflow.

The expected feel is: work stays natural, while scope, evidence, checks, reports, and next-step selection become explicit.

## Not included

The GoalRail process shell does not include:

- `.punk/` runtime state;
- Punk CLI commands;
- event ledger implementation;
- gate implementation;
- proofpack writer;
- modules or adapters;
- provider/model runner;
- required IDE, CLI ritual, prompt, skill, model, provider, or local runtime setup;
- semantic assessor command interface;
- schema changes;
- automation;
- public product claims;
- a second tracker;
- GoalRail productization.

## Manual pilot flow

### 1. Create goal

Create one GoalRail-local goal artifact with explicit scope, exclusions, acceptance criteria, and source refs.

Reject vague work until it has inputs, constraints, and acceptance criteria.

### 2. Classify Research Gate

Classify the goal as R0, R1, R2, or R3 before editing.

Use existing repo-tracked or user-provided research for R1+ work. If adequate research is missing, stop.

### 3. Produce bounded plan

Write a short plan that names:

- the single intended diff;
- files or artifacts expected to change;
- checks required;
- non-goals;
- rollback or deferral path for risky items.

### 4. Execute manually or with an executor

A human, script, local tool, model, coding agent, or other executor may attempt the scoped work.

The executor does not own authority. It must not silently widen scope, create hidden state, or convert self-review into acceptance.

### 5. Record report and evidence

Write a GoalRail-local report with changed artifacts, evidence, checks, observed failures, and follow-ups.

If a check is unavailable, record it as unavailable instead of passed.

### 6. Review work ledger

Periodically run an advisory Work Ledger Review over the project-local status, goals, reports, and recent diffs.

The review may identify repeated process failures, stale blockers, missing evidence, or one conservative next goal.

### 7. Select next

Update the single live status file so `selected_next` points to one ready goal.

Do not create a parallel roadmap, hidden backlog, or second live tracker to manage pilot state.

## Guardrails

- One project, one live ledger.
- No hidden source of truth.
- No required change to the user's existing setup.
- No runtime state without explicit future scope.
- No `.punk/` port.
- No CLI port.
- No prescribed IDE, CLI ritual, model, provider, prompt system, skill system, or local runtime shape.
- No gate/proof terms used as if runtime gate/proof exists.
- No executor claim treated as proof.
- No model/provider settings treated as architecture.
- No advisory review treated as final decision.
- No GoalRail productization from this pilot.

## Stop conditions

Stop the pilot and write a report if any of these happen:

- a second live tracker appears;
- GoalRail pilot state starts living in Punk instead of GoalRail;
- executor instructions override goal acceptance criteria;
- the pilot requires users to change IDE, provider, model, prompt, skill, CLI ritual, or local runtime setup;
- runtime, CLI, `.punk/`, gate, proofpack, adapter, or automation work becomes necessary;
- evidence cannot be recorded in repo-tracked artifacts;
- the pilot needs external claims that are not captured in research.

## Open follow-ups

These are intentionally outside this pilot:

- missing-validator policy;
- minimal receipt fields;
- semantic assessor command interface;
- real gate runtime;
- real proofpack runtime;
- runtime Event Ledger;
- possible future `punk init` command;
- reconciliation of proofpack minimum metadata and `proof before acceptance` semantics before gate/proof runtime work.

## `punk init` mismatch note

`docs/product/START-HERE.md` lists `punk init` under the active target, but the current CLI implementation only exposes:

- `punk flow inspect`;
- `punk eval run smoke`;
- `punk eval run smoke --format json`.

This pilot does not fix the mismatch because `docs/product/**` and CLI changes are outside the selected goal scope.

Record it as a separate docs/CLI mismatch follow-up.
