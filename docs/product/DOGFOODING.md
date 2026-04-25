---
id: docs_product_dogfooding
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-25
review_after: 2026-07-21
canonical_for:
  - dogfooding-levels
  - manual-work-ledger-discipline
  - self-tracking-boundary
  - trust-surface-classes
related_docs:
  - docs/product/PROJECT-MEMORY.md
  - docs/product/CONTRACT-TRACKER.md
  - docs/product/START-HERE.md
related_adrs:
  - docs/adr/ADR-0004-dogfooding-from-day-zero.md
supersedes: []
superseded_by: null
---

# Dogfooding

## Purpose

`punk` should be developed with `punk`.

Dogfooding starts before `punk` can execute its own implementation work.

The initial form is not self-execution. The initial form is self-tracking.

## Principle

Use `punk` as soon as a capability exists, but only at the trust level it has earned.

## Dogfooding levels

### Level 0 — Manual project memory

Use repo-tracked `work/` and `knowledge/` manually.

The canonical live state is:

```text
work/STATUS.md
```

Allowed:

- create goals in `work/goals/`
- write reports in `work/reports/`
- maintain knowledge docs
- record decisions in ADRs
- keep current state, selected next goal, blockers, and recent evidence in `work/STATUS.md`

Not allowed:

- claim `punk` executed the work
- skip human review
- create a second live-state file beside `work/STATUS.md`
- treat `done` as future `gate` acceptance

Level 0 operating rules:

- `work/STATUS.md` is the only live work-state source of truth;
- `selected_next` points to one `ready` goal;
- set `in_progress` only when work has actually started;
- every meaningful change updates status and/or the selected goal/report;
- `done` means manually closed with evidence, not final acceptance;
- `.punk/` runtime state is not written yet for this purpose.

Exit criteria:

- all new meaningful work has a goal or explicit maintenance reason
- current focus and selected next work are inspectable from `work/STATUS.md`
- accepted decisions have ADRs or knowledge updates when needed

### Level 1 — Flow-tracked work

Use `punk` to inspect flow state and next actions.

Currently implemented subset:

- `punk flow inspect`

Target surfaces after implementation:

- `punk init`
- `punk inspect project`
- flow state tracking

Not allowed:

- autonomous code modification
- self-certification of core trust changes

Exit criteria:

- flow can resume from persisted state
- illegal transitions are denied

### Level 2 — Eval-gated development

Use the currently implemented eval command before and after changes.

Currently implemented subset:

- `punk eval run smoke`
- baseline comparison
- eval reports linked to work goals

Not allowed:

- phase promotion with red hard gates
- replacing human review with eval scores

Exit criteria:

- every active phase has smoke/core eval coverage
- regressions are tracked as goals or explicit accepted waivers

### Level 3 — Contract-tracked implementation

Use `punk` to create and approve contracts for its own work.

Allowed:

- `punk start --from work/goals/<id>.md`
- `punk plot approve`
- manual/deterministic run receipts
- gate/proof over local deterministic checks

Not allowed:

- autonomous modification of gate/proof/eval/flow trust surfaces without stronger review

Exit criteria:

- accepted work links goal -> contract -> run -> decision -> proof

### Level 4 — Bounded self-execution

Use `punk` to execute bounded implementation slices.

Allowed:

- bounded `cut` execution in non-trust-critical surfaces
- normal gate/proof
- reports and knowledge updates

Requires:

- green eval baseline
- contract scope
- rollback path

Not allowed without extra review:

- changes to `punk-flow`
- changes to `punk-gate`
- changes to `punk-proof`
- changes to `punk-eval`
- changes to rule resolution
- changes to module permission model

### Level 5 — Selective self-improvement

Use `punk` to propose improvements to its own rules/modules.

Allowed:

- propose candidate rule/module changes
- run conformance evals
- produce proof and report

Requires:

- independent review for core law changes
- phase gate approval
- explicit promotion decision

## Trust-surface classes

### Ordinary product surface

Examples:

- CLI formatting
- docs
- templates
- non-critical inspect views

May be dogfooded at Level 3+.

### Core trust surface

Examples:

- flow controller
- gate
- proof
- eval runner
- rule resolution
- event log
- module permissions

Requires stronger review and additional evals.

### External side-effect surface

Examples:

- publishing
- provider adapters
- network tools
- secrets

Requires declared side-effect policy and receipts.

## Dogfooding event rule

Every dogfooded work item must leave:

- goal ref or maintenance reason
- flow state
- eval result when relevant
- report
- proof when available

Executor briefs may be used as advisory handoff aids during dogfooding, but they do not replace the dogfood trail. The trail should still record goal, contract when available, executor attempt, receipt/evidence, validators, gate decision when available, proof when available, and project-memory links.

## Research intake dogfooding checks

Each accepted dogfooding contract should answer:

- Which event ids prove the lifecycle path?
- Which guard results allowed or denied transitions?
- Which eval reports ran?
- Which gate decision accepted or rejected the work?
- Which proofpack references the evidence?
- Which project-memory docs were updated or explicitly waived?
- Did the run avoid hidden network behavior?

## First dogfooding action

Before code exists, create the first goal manually:

```text
work/goals/goal_bootstrap_punk_core.md
```

It should track Phase 0: architecture skeleton.
