---
id: report_2026_05_21_pubpunk_control_manifest_validator_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
goal_ref: work/goals/goal_add_pubpunk_control_manifest_validator_v0_1.md
---

# PubPunk Control Manifest Validator v0.1

## Summary

Added a read-only validator for the PubPunk control manifest fixture and eval
spec.

The validator checks that PubPunk's control manifest fixture keeps required
refs, behavior artifact, immutable core, empty grants, denied side effects,
disabled reflection, non-current tuning, user-local, hook fixture, and
promotion boundaries visible and does not contain obvious activation markers.

## Files changed

- `scripts/check_pubpunk_control_manifest.py`
- `scripts/check.sh`
- `docs/modules/pubpunk-control-manifest.md`
- `evals/specs/pubpunk-control-manifest.v0.1.md`
- `work/goals/goal_add_pubpunk_control_manifest_validator_v0_1.md`
- `work/reports/2026-05-21-pubpunk-control-manifest-validator-v0-1.md`
- `work/STATUS.md`

## Research Gate

Classification: R2.

The R2 requirement is satisfied by
`knowledge/research/2026-05-21-adaptive-module-control-plane.md`.

## What changed

- Added `scripts/check_pubpunk_control_manifest.py`.
- Added `scripts/check.sh pubpunk-control-manifest`.
- The checker validates required fixture/spec markers.
- The checker rejects activation markers for auto-apply, runtime, manifest
  parser, resolver, writer, user-local config writer, reflection scheduler,
  adapter invocation, publishing, metrics, event writing, gate writer,
  proofpack writer, acceptance claim, selected current behavior, selected
  user-local overlay/artifact, selected promotion, and selected tuning
  proposal.
- Added validator-command notes to the fixture and eval spec.

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
    - docs/modules/pubpunk-control-manifest.md
  evals:
    - evals/specs/pubpunk-control-manifest.v0.1.md
  scripts:
    - scripts/check_pubpunk_control_manifest.py
    - scripts/check.sh
  work_artifacts:
    - work/goals/goal_add_pubpunk_control_manifest_validator_v0_1.md
    - work/reports/2026-05-21-pubpunk-control-manifest-validator-v0-1.md
    - work/STATUS.md
  reason: "Adds a read-only validator and documents its command without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
```

## Validation run

- `scripts/check.sh pubpunk-control-manifest` -> PASS.
- `python3 -m py_compile scripts/check_pubpunk_control_manifest.py` -> PASS.
- `bash -n scripts/check.sh` -> PASS.
- `python3 scripts/check_research_gate.py` -> PASS.
- `python3 scripts/check_work_ledger.py` -> PASS.
- `scripts/check.sh docs-governance --files scripts/check_pubpunk_control_manifest.py scripts/check.sh docs/modules/pubpunk-control-manifest.md evals/specs/pubpunk-control-manifest.v0.1.md work/goals/goal_add_pubpunk_control_manifest_validator_v0_1.md work/STATUS.md --report work/reports/2026-05-21-pubpunk-control-manifest-validator-v0-1.md`
  -> PASS.
- `git diff --check` -> PASS.

## Knowledge impact

- Canonical docs changed: `docs/modules/pubpunk-control-manifest.md`.
- Eval specs changed: `evals/specs/pubpunk-control-manifest.v0.1.md`.
- Project-memory claims affected: future PubPunk control-manifest fixture edits
  now have a deterministic read-only completeness check.
- Docs / ADRs / evals possibly stale: none identified.
- Active / parked / future scope affected: active runtime scope unchanged;
  manifest parsing, config resolution, user-local config writing, reflection
  scheduling, behavior-artifact writing, metrics collection, external
  publishing, event writing, gates, proofpacks, and acceptance claims remain
  parked.
- Public narrative impact: none.
- Follow-up goals or drift findings: a future slice may add an aggregate
  module-control validator target after all current control-plane validators
  are stable.

## Out of scope

No Rust code, CLI behavior, module loader, manifest parser, config resolver,
user-local config writer, reflection scheduler, behavior-artifact writer,
article reader, metrics collection, external research execution, Module Host
runtime activation, PubPunk runtime activation, `.punk` runtime state, adapter
invocation, browser automation, credential access, external publishing, receipt
writing, event writing, gate writer, proofpack writer, or acceptance claim was
added.
