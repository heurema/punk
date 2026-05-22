---
id: docs_product_module_control_validation
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
canonical_for:
  - module-control-validation-runbook
  - module-control-marker-suite-boundary
  - module-control-validator-routing
related_docs:
  - docs/product/MODULE-CONTROL-PLANE.md
  - docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/modules/pubpunk-control-manifest.md
  - docs/modules/pubpunk-article-hook-tuning-fixture.md
  - docs/modules/pubpunk-live-tuning-runbook.md
  - docs/modules/pubpunk-live-tuning-handoff-template.md
  - docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md
related_evals:
  - evals/specs/module-control-validation.v0.1.md
  - evals/specs/module-tuning-handoff-template.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
  - evals/specs/pubpunk-control-manifest.v0.1.md
  - evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md
  - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
  - evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md
  - evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md
supersedes: []
superseded_by: null
---

# Module Control Validation

## Purpose

Define the operator runbook for the current Module Control Plane marker checks.

The goal is to make adaptive module work repeatable without relying on the
operator or executor remembering which validator protects which boundary.

## Boundary

This document defines validation routing only.

It is not a runtime schema, manifest parser, config resolver, overlay resolver,
user-local config writer, reflection scheduler, behavior-artifact writer,
article reader, metrics collector, research runner, adapter invoker, publisher,
event writer, gate writer, proofpack writer, or acceptance authority.

## Command map

Use these commands for the current Module Control Plane validation pack:

| Command | Primary surface | Use when |
|---|---|---|
| `scripts/check.sh module-control-plane` | Module Control Plane contract and eval spec | Editing the core control manifest, manual tuning, reflection, provenance, overlay, user-local artifact, tuning proposal, or promotion packet contract. |
| `scripts/check.sh module-behavior-boundaries` | Overlay, user-local artifact, tuning proposal, and promotion packet markers | Editing behavior artifact boundaries or tightening the no-auto-apply, no-writer, no-capability-change, or no-side-effect-change rules. |
| `scripts/check.sh module-tuning-handoff-template` | Generic Module Tuning Handoff Template | Editing the generic proposal-only handoff template for text, voice, or scheduled-proposal module tuning. |
| `scripts/check.sh pubpunk-control-manifest` | PubPunk advisory control manifest fixture | Editing PubPunk control manifest docs/specs or changing which PubPunk behavior artifacts are advisory fixtures. |
| `scripts/check.sh pubpunk-hook-tuning-fixture` | PubPunk article hook tuning fixture | Editing the example operator request, evidence packet, proposal, user-local artifact, promotion packet, or resolved behavior fixture. |
| `scripts/check.sh pubpunk-live-tuning-pack` | PubPunk live tuning runbook, handoff template, and example | Editing operator-triggered live tuning docs, template fields, evidence limitations, or example-only handoff content. |
| `scripts/check.sh module-control-validation` | This validation runbook and command wiring | Editing the validation map, suite membership, or check target names. |
| `scripts/check.sh module-control-suite` | Current aggregate marker suite | Before considering the Module Control Plane validation pack internally consistent. |

All commands in this table dispatch through one consolidated marker checker:

```sh
scripts/check_module_control_markers.py
```

## Tooling policy

Use Python only for static repo-governance and docs/spec marker checks.

Runtime behavior checks, manifest parsing checks, resolver checks, writer
checks, module behavior checks, and implementation-state checks belong in Rust
or a future Rust `xtask` path.

Run these repo-level checks after any repo-tracked Module Control Plane slice:

```sh
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
```

Run docs governance when the slice changes product docs, module docs, eval
specs, goals, reports, or work status:

```sh
scripts/check.sh docs-governance --files <changed-files> --report <work-report>
```

## Validator meaning

Passing marker checks mean the expected boundary markers are still present and
obvious activation markers are absent.

Passing marker checks do not mean:

- manifest parsing is active;
- overlays are resolved at runtime;
- behavior artifacts can write themselves;
- user-local config can be written;
- reflection can run in the background;
- metrics were collected;
- external research was run;
- adapters or browsers were invoked;
- publishing happened;
- events, gates, proofpacks, or acceptance claims were written.

## Routing rules

Use the smallest check that covers the edited surface while developing.

Use `scripts/check.sh module-control-suite` before closing a Module Control
Plane or PubPunk control-plane slice.

Use `scripts/check.sh module-control-validation` whenever a command name,
validator file name, suite member, or validation runbook wording changes.

Do not treat a passing focused check as a substitute for the repo-level Research
Gate and Work Ledger checks.

## Failure handling

If a marker check fails:

- inspect the missing or forbidden marker;
- decide whether the document drifted or the checker is stale;
- update docs, specs, and checker together when the boundary intentionally
  changes;
- record the rationale in the selected goal/report;
- do not remove a no-runtime, no-auto-apply, no-writer, no-capability-change,
  no-side-effect-change, no-secret, no-private-data, or no-executable-code
  marker merely to make the check pass.

If a desired change needs runtime behavior, stop this validation path and create
a separate selected goal for that runtime boundary.

## Suite script

Use this script as the current aggregate suite implementation:

- `scripts/check_module_control_markers.py`

The suite is a read-only marker suite. It is not a gate, proofpack, runtime
acceptance signal, or substitute for product review.

## Non-goals

This runbook does not define:

- Rust code;
- CLI behavior;
- active manifest parsing;
- runtime overlay resolution;
- background reflection;
- behavior-artifact writing;
- user-local config writing;
- article reading;
- metrics collection;
- external research execution;
- adapter invocation;
- browser automation;
- credential access;
- external publishing;
- event writing;
- gate writing;
- proofpack writing;
- acceptance claims.

## Validation

Run the read-only marker check for this runbook and its command wiring:

```sh
scripts/check.sh module-control-validation
```

Run the full current Module Control Plane marker suite:

```sh
scripts/check.sh module-control-suite
```
