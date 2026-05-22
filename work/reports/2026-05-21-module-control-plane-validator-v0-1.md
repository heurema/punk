---
id: report_2026_05_21_module_control_plane_validator_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
goal_ref: work/goals/goal_add_module_control_plane_validator_v0_1.md
---

# Module Control Plane Validator v0.1

## Summary

Added a read-only validator for the Module Control Plane contract and eval
spec.

The validator checks that the general control-plane contract keeps the
manifest, manual tuning, behavior overlay stack, user-local artifact, tuning
proposal, and promotion packet boundaries visible and does not contain obvious
activation markers.

## Files changed

- `scripts/check_module_control_plane.py`
- `scripts/check.sh`
- `docs/product/MODULE-CONTROL-PLANE.md`
- `evals/specs/module-control-plane.v0.1.md`
- `work/goals/goal_add_module_control_plane_validator_v0_1.md`
- `work/reports/2026-05-21-module-control-plane-validator-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

The R2 requirement is satisfied by
`knowledge/research/2026-05-21-adaptive-module-control-plane.md`.

## What changed

- Added `scripts/check_module_control_plane.py`.
- Added `scripts/check.sh module-control-plane`.
- The checker validates required contract/spec markers.
- The checker rejects activation markers for auto-apply, runtime, manifest
  parser, resolver, writer, user-local config writer, reflection scheduler,
  adapter invocation, publishing, metrics, gate writer, proofpack writer,
  acceptance claim, secrets, private data, and executable code.
- Added validator-command notes to the contract and eval spec.

## Boundary confirmation

- The checker is read-only.
- It does not parse or load module manifests.
- It does not resolve behavior overlays.
- It does not write user-local artifacts.
- It does not run reflection, publish, collect metrics, invoke adapters, open
  browsers, read credentials, write receipts, write events, write gates, write
  proofpacks, or claim acceptance.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  canonical_docs:
    - docs/product/MODULE-CONTROL-PLANE.md
  evals:
    - evals/specs/module-control-plane.v0.1.md
  scripts:
    - scripts/check_module_control_plane.py
    - scripts/check.sh
  work_artifacts:
    - work/goals/goal_add_module_control_plane_validator_v0_1.md
    - work/reports/2026-05-21-module-control-plane-validator-v0-1.md
    - work/STATUS.md
  reason: "Adds a read-only validator and documents its command without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `scripts/check.sh module-control-plane` -> PASS.
- `python3 -m py_compile scripts/check_module_control_plane.py` -> PASS.
- `bash -n scripts/check.sh` -> PASS.
- `python3 scripts/check_research_gate.py` -> PASS.
- `python3 scripts/check_work_ledger.py` -> PASS.
- `scripts/check.sh docs-governance --files scripts/check_module_control_plane.py scripts/check.sh docs/product/MODULE-CONTROL-PLANE.md evals/specs/module-control-plane.v0.1.md work/goals/goal_add_module_control_plane_validator_v0_1.md work/STATUS.md --report work/reports/2026-05-21-module-control-plane-validator-v0-1.md`
  -> PASS.
- `git diff --check` -> PASS.

## Knowledge impact

- Canonical docs changed: `docs/product/MODULE-CONTROL-PLANE.md`.
- Eval specs changed: `evals/specs/module-control-plane.v0.1.md`.
- Project-memory claims affected: future module-control-plane edits now have a
  deterministic read-only completeness check.
- Docs / ADRs / evals possibly stale: none identified.
- Active / parked / future scope affected: active runtime scope unchanged;
  manifest parsing, config resolution, user-local config writing, reflection
  scheduling, behavior-artifact writing, metrics collection, external
  publishing, gates, proofpacks, and acceptance claims remain parked.
- Public narrative impact: none.
- Follow-up goals or drift findings: a future slice may unify related
  fixture-specific validators after more module fixtures exist.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, config resolver,
user-local config writer, reflection scheduler, behavior-artifact writer,
article reader, metrics collection, external research execution, Module Host
runtime activation, PubPunk runtime activation, `.punk` runtime state, adapter
invocation, browser automation, credential access, external publishing, receipt
writing, event writing, gate writer, proofpack writer, or acceptance claim was
added.
