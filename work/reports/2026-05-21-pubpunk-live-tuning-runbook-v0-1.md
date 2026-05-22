---
id: report_2026_05_21_pubpunk_live_tuning_runbook_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
goal_ref: work/goals/goal_define_pubpunk_live_tuning_runbook_v0_1.md
---

# PubPunk Live Tuning Runbook v0.1

## Summary

Added an advisory PubPunk live tuning runbook for operator-triggered behavior
customization during active work.

The runbook defines how a request like article hook tuning moves through
request capture, evidence planning, evidence packet, tuning proposal, approval,
decision routing, and provenance without allowing PubPunk to self-modify.

## Files changed

- `docs/modules/pubpunk-live-tuning-runbook.md`
- `evals/specs/pubpunk-live-tuning-runbook.v0.1.md`
- `docs/modules/pubpunk.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `work/goals/goal_define_pubpunk_live_tuning_runbook_v0_1.md`
- `work/reports/2026-05-21-pubpunk-live-tuning-runbook-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

The R2 requirement is satisfied by
`knowledge/research/2026-05-21-adaptive-module-control-plane.md`.

## What changed

- Added `docs/modules/pubpunk-live-tuning-runbook.md`.
- Added `evals/specs/pubpunk-live-tuning-runbook.v0.1.md`.
- Linked the runbook from the PubPunk module doc.
- Added the runbook and spec to the documentation map.
- Kept validation routed through the existing read-only
  `scripts/check.sh module-control-suite` command.

## Boundary confirmation

- The runbook is advisory only.
- It does not read articles, collect metrics, run external research, write
  config, write behavior artifacts, publish, invoke adapters, open browsers,
  read credentials, write receipts, write events, write gates, write
  proofpacks, or claim acceptance.
- Operator approval is recorded as promotion input only, not auto-apply
  authority.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  canonical_docs:
    - docs/modules/pubpunk-live-tuning-runbook.md
    - docs/modules/pubpunk.md
    - docs/product/DOCUMENTATION-MAP.md
  evals:
    - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
  work_artifacts:
    - work/goals/goal_define_pubpunk_live_tuning_runbook_v0_1.md
    - work/reports/2026-05-21-pubpunk-live-tuning-runbook-v0-1.md
    - work/STATUS.md
  reason: "Adds an advisory runbook and eval boundary for live PubPunk tuning without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, article readers, metrics collectors, research runners, adapter invocation, publishing, event writing, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `scripts/check.sh module-control-suite` -> PASS.
- `python3 scripts/check_research_gate.py` -> PASS.
- `python3 scripts/check_work_ledger.py` -> PASS.
- `scripts/check.sh docs-governance --files docs/modules/pubpunk-live-tuning-runbook.md evals/specs/pubpunk-live-tuning-runbook.v0.1.md docs/modules/pubpunk.md docs/product/DOCUMENTATION-MAP.md work/goals/goal_define_pubpunk_live_tuning_runbook_v0_1.md work/STATUS.md --report work/reports/2026-05-21-pubpunk-live-tuning-runbook-v0-1.md`
  -> PASS.
- `git diff --check` -> PASS.

## Knowledge impact

- Canonical docs changed:
  - `docs/modules/pubpunk-live-tuning-runbook.md`
  - `docs/modules/pubpunk.md`
  - `docs/product/DOCUMENTATION-MAP.md`
- Eval specs changed: `evals/specs/pubpunk-live-tuning-runbook.v0.1.md`.
- Project-memory claims affected: future PubPunk live tuning requests now have
  an advisory runbook.
- Docs / ADRs / evals possibly stale: none identified.
- Active / parked / future scope affected: active runtime scope unchanged;
  manifest parsing, config resolution, user-local config writing, reflection
  scheduling, behavior-artifact writing, article reading, metrics collection,
  external research execution, publishing, event writing, gates, proofpacks,
  and acceptance claims remain parked.
- Public narrative impact: none.
- Follow-up goals or drift findings: future work may add a concrete live
  tuning artifact writer only after a separate selected implementation goal.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, config resolver,
user-local config writer, reflection scheduler, behavior-artifact writer,
article reader, metrics collection, external research execution, Module Host
runtime activation, PubPunk runtime activation, `.punk` runtime state, adapter
invocation, browser automation, credential access, external publishing, receipt
writing, event writing, gate writer, proofpack writer, or acceptance claim was
added.
