---
id: report_2026_05_22_module_control_marker_validator_consolidation_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-22
updated_at: 2026-05-22
goal_ref: work/goals/goal_consolidate_module_control_marker_validators_v0_1.md
---

# Module Control Marker Validator Consolidation v0.1

## Summary

Consolidated the Module Control Plane marker validators into one read-only
stdlib Python checker.

The public check targets remain stable through `scripts/check.sh`, but the
implementation now dispatches through `scripts/check_module_control_markers.py`
instead of separate `check_module_*` and `check_pubpunk_*` Python files.

## Files changed

- `scripts/check_module_control_markers.py`
- `scripts/check.sh`
- `docs/product/MODULE-CONTROL-VALIDATION.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `evals/specs/module-control-validation.v0.1.md`
- `work/goals/goal_consolidate_module_control_marker_validators_v0_1.md`
- `work/reports/2026-05-22-module-control-marker-validator-consolidation-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

The R2 requirement is satisfied by
`knowledge/research/2026-05-21-adaptive-module-control-plane.md`.

## What changed

- Added `scripts/check_module_control_markers.py` as the single consolidated
  marker checker.
- Updated `scripts/check.sh` so existing targets call the consolidated checker:
  - `module-control-plane`
  - `module-behavior-boundaries`
  - `module-tuning-handoff-template`
  - `pubpunk-control-manifest`
  - `pubpunk-hook-tuning-fixture`
  - `pubpunk-live-tuning-pack`
  - `module-control-validation`
  - `module-control-suite`
- Removed the separate untracked marker validator files from the working tree.
- Updated Module Control validation runbook and spec to record the one-script
  suite implementation.
- Added the explicit validation-language policy:
  - Python is allowed only for static repo-governance and docs/spec marker
    checks.
  - Runtime, parser, resolver, writer, module behavior, and implementation-state
    checks belong in Rust or a future Rust `xtask` path.
- Updated Documentation Map wording for Module Control Validation.

## Boundary confirmation

- The consolidated checker is read-only.
- It does not parse manifests or handoff YAML as runtime state.
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
    - docs/product/DOCUMENTATION-MAP.md
  evals:
    - evals/specs/module-control-validation.v0.1.md
  scripts:
    - scripts/check_module_control_markers.py
    - scripts/check.sh
  work_artifacts:
    - work/goals/goal_consolidate_module_control_marker_validators_v0_1.md
    - work/reports/2026-05-22-module-control-marker-validator-consolidation-v0-1.md
    - work/STATUS.md
  reason: "Consolidates read-only Module Control marker validators and documents Python/Rust validation boundaries without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `scripts/check.sh module-control-plane` -> PASS.
- `scripts/check.sh module-behavior-boundaries` -> PASS.
- `scripts/check.sh module-tuning-handoff-template` -> PASS.
- `scripts/check.sh pubpunk-control-manifest` -> PASS.
- `scripts/check.sh pubpunk-hook-tuning-fixture` -> PASS.
- `scripts/check.sh pubpunk-live-tuning-pack` -> PASS.
- `scripts/check.sh module-control-validation` -> PASS.
- `scripts/check.sh module-control-suite` -> PASS.
- `python3 -m py_compile scripts/check_module_control_markers.py` -> PASS.
- `bash -n scripts/check.sh` -> PASS.
- `python3 scripts/check_research_gate.py` -> PASS.
- `python3 scripts/check_work_ledger.py` -> PASS.
- `scripts/check.sh docs-governance --files scripts/check_module_control_markers.py scripts/check.sh docs/product/MODULE-CONTROL-VALIDATION.md docs/product/DOCUMENTATION-MAP.md evals/specs/module-control-validation.v0.1.md work/goals/goal_consolidate_module_control_marker_validators_v0_1.md work/STATUS.md --report work/reports/2026-05-22-module-control-marker-validator-consolidation-v0-1.md`
  -> PASS.
- `git diff --check` -> PASS.

## Knowledge impact

- Canonical docs changed:
  - `docs/product/MODULE-CONTROL-VALIDATION.md`
  - `docs/product/DOCUMENTATION-MAP.md`
- Eval specs changed: `evals/specs/module-control-validation.v0.1.md`.
- Project-memory claims affected: Module Control Plane marker checks now use one
  consolidated checker, not several separate Python files.
- Docs / ADRs / evals possibly stale: older work reports describe the original
  focused validator slices; this report supersedes that implementation shape
  while preserving the check target names.
- Active / parked / future scope affected: active runtime scope unchanged;
  manifest parsing, overlay resolution, config resolution, user-local config
  writing, reflection scheduling, behavior-artifact writing, metrics
  collection, external research execution, publishing, event writing, gates,
  proofpacks, and acceptance claims remain parked.
- Public narrative impact: none.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, overlay resolver,
config resolver, user-local config writer, reflection scheduler,
behavior-artifact writer, article reader, metrics collection, external research
execution, Module Host runtime activation, PubPunk runtime activation, `.punk`
runtime state, adapter invocation, browser automation, credential access,
external publishing, receipt writing, event writing, gate writer, proofpack
writer, or acceptance claim was added.
