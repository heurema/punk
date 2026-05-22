---
id: report_2026_05_21_module_control_validation_runbook_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
goal_ref: work/goals/goal_add_module_control_validation_runbook_v0_1.md
---

# Module Control Validation Runbook v0.1

## Summary

Added a canonical validation runbook for the current Module Control Plane
marker-check pack.

The runbook maps each focused check to its protected surface, explains what a
passing marker check does and does not prove, defines routing and failure
handling, and lists the current aggregate suite membership.

## Files changed

- `docs/product/MODULE-CONTROL-VALIDATION.md`
- `evals/specs/module-control-validation.v0.1.md`
- `scripts/check_module_control_validation.py`
- `scripts/check_module_control_suite.py`
- `scripts/check.sh`
- `docs/product/MODULE-CONTROL-PLANE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `work/goals/goal_add_module_control_validation_runbook_v0_1.md`
- `work/reports/2026-05-21-module-control-validation-runbook-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

The R2 requirement is satisfied by
`knowledge/research/2026-05-21-adaptive-module-control-plane.md`.

## What changed

- Added `docs/product/MODULE-CONTROL-VALIDATION.md`.
- Added `evals/specs/module-control-validation.v0.1.md`.
- Added `scripts/check_module_control_validation.py`.
- Added `scripts/check.sh module-control-validation`.
- Added the validation runbook checker to
  `scripts/check.sh module-control-suite`.
- Updated `docs/product/MODULE-CONTROL-PLANE.md` related refs.
- Updated `docs/product/DOCUMENTATION-MAP.md` with a Module Control Validation
  truth-surface row.
- The checker validates command map, repo-level check reminders, docs
  governance routing, marker-check meaning, failure handling, suite
  membership, and no-runtime boundaries.

## Boundary confirmation

- The checker is read-only.
- It does not parse manifests or validation docs as runtime state.
- It does not resolve overlays.
- It does not write user-local artifacts.
- It does not promote behavior artifacts.
- It does not run reflection, publish, collect metrics, run external research,
  invoke adapters, open browsers, read credentials, write receipts, write
  events, write gates, write proofpacks, or claim acceptance.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  canonical_docs:
    - docs/product/MODULE-CONTROL-VALIDATION.md
    - docs/product/MODULE-CONTROL-PLANE.md
    - docs/product/DOCUMENTATION-MAP.md
  evals:
    - evals/specs/module-control-validation.v0.1.md
  scripts:
    - scripts/check_module_control_validation.py
    - scripts/check_module_control_suite.py
    - scripts/check.sh
  work_artifacts:
    - work/goals/goal_add_module_control_validation_runbook_v0_1.md
    - work/reports/2026-05-21-module-control-validation-runbook-v0-1.md
    - work/STATUS.md
  reason: "Adds a read-only validation runbook and validator for Module Control Plane marker-check routing without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `scripts/check.sh module-control-validation` -> PASS.
- `scripts/check.sh module-control-suite` -> PASS.
- `python3 -m py_compile scripts/check_module_control_validation.py scripts/check_module_control_suite.py`
  -> PASS.
- `bash -n scripts/check.sh` -> PASS.
- `python3 scripts/check_research_gate.py` -> PASS.
- `python3 scripts/check_work_ledger.py` -> PASS.
- `scripts/check.sh docs-governance --files docs/product/MODULE-CONTROL-VALIDATION.md evals/specs/module-control-validation.v0.1.md scripts/check_module_control_validation.py scripts/check_module_control_suite.py scripts/check.sh docs/product/MODULE-CONTROL-PLANE.md docs/product/DOCUMENTATION-MAP.md work/goals/goal_add_module_control_validation_runbook_v0_1.md work/STATUS.md --report work/reports/2026-05-21-module-control-validation-runbook-v0-1.md`
  -> PASS.
- `git diff --check` -> PASS.

## Knowledge impact

- Canonical docs changed:
  - `docs/product/MODULE-CONTROL-VALIDATION.md`
  - `docs/product/MODULE-CONTROL-PLANE.md`
  - `docs/product/DOCUMENTATION-MAP.md`
- Eval specs changed: `evals/specs/module-control-validation.v0.1.md`.
- Project-memory claims affected: current Module Control Plane marker checks now
  have a canonical validation runbook and read-only checker.
- Docs / ADRs / evals possibly stale: none identified.
- Active / parked / future scope affected: active runtime scope unchanged;
  manifest parsing, overlay resolution, config resolution, user-local config
  writing, reflection scheduling, behavior-artifact writing, metrics
  collection, external research execution, publishing, event writing, gates,
  proofpacks, and acceptance claims remain parked.
- Public narrative impact: none.
- Follow-up goals or drift findings: future validators should update
  `docs/product/MODULE-CONTROL-VALIDATION.md`,
  `evals/specs/module-control-validation.v0.1.md`, and
  `scripts/check_module_control_validation.py` in the same bounded slice.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, overlay resolver,
config resolver, user-local config writer, reflection scheduler,
behavior-artifact writer, article reader, metrics collection, external research
execution, Module Host runtime activation, PubPunk runtime activation, `.punk`
runtime state, adapter invocation, browser automation, credential access,
external publishing, receipt writing, event writing, gate writer, proofpack
writer, or acceptance claim was added.
