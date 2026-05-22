---
id: goal_add_module_control_validation_runbook_v0_1
title: "Add Module Control validation runbook v0.1"
status: done
owner: "vitaly"
module: "module-control-plane"
priority: P2
authority: canonical
created_at: 2026-05-21
updated_at: 2026-05-21
selected_at: 2026-05-21
started_at: 2026-05-21
completed_at: 2026-05-21
blocked_by: []
scope:
  include:
    - "docs/product/MODULE-CONTROL-VALIDATION.md"
    - "evals/specs/module-control-validation.v0.1.md"
    - "scripts/check_module_control_validation.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/goals/goal_add_module_control_validation_runbook_v0_1.md"
    - "work/reports/2026-05-21-module-control-validation-runbook-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
acceptance:
  - "Adds a canonical Module Control validation runbook for current marker-check command routing."
  - "Adds an eval spec for the validation runbook and suite wiring."
  - "Adds a read-only stdlib validator exposed as `scripts/check.sh module-control-validation`."
  - "Adds the validator to `scripts/check.sh module-control-suite`."
  - "Documents command map, validator meaning, routing rules, failure handling, and current suite membership."
  - "Records that marker checks are not runtime proof, gates, proofpacks, acceptance signals, or substitutes for Research Gate and Work Ledger checks."
  - "Rejects obvious activation markers for runtime, manifest parser, overlay resolver, writer, user-local config writer, scheduler, metrics, external research, adapter invocation, publishing, event writing, gate writer, proofpack writer, and acceptance claim."
  - "Updates documentation map and Module Control Plane related refs."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, article reader, metrics collector, research runner, adapter invocation, publishing, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-VALIDATION.md"
  - "evals/specs/module-control-validation.v0.1.md"
  - "docs/product/MODULE-CONTROL-PLANE.md"
report_refs:
  - "work/reports/2026-05-21-module-control-validation-runbook-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic validation runbook and marker checker for the existing Module Control Plane validation pack. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, metrics, or research execution behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/MODULE-CONTROL-VALIDATION.md"
    - "evals/specs/module-control-validation.v0.1.md"
    - "scripts/check_module_control_validation.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only validation runbook and validator for Module Control Plane marker-check routing without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The Module Control Plane now has several focused marker validators. They need a
single canonical runbook that tells operators which check protects which
boundary and how to interpret check output.

## Intent

Add a validation runbook and validator:

```text
scripts/check.sh module-control-validation
```

Also include it in:

```text
scripts/check.sh module-control-suite
```

## In scope

- New Module Control validation runbook.
- New eval spec.
- New stdlib Python validator.
- `scripts/check.sh` target.
- Aggregate suite wiring.
- Documentation map and related-ref updates.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- PubPunk runtime.
- Manifest parser.
- Overlay resolver.
- Config resolver.
- User-local config writer.
- Behavior-artifact writer.
- Reflection scheduler.
- Article reader.
- Metrics collector.
- External research runner.
- Adapter invocation.
- Browser automation.
- Credential access.
- External publishing.
- Receipt writing.
- Event writing.
- Gate writer.
- Proofpack writer.
- Acceptance claim.

## Outcome

Done in
`work/reports/2026-05-21-module-control-validation-runbook-v0-1.md`.

The patch adds a read-only validation runbook and marker checker only.
