---
id: goal_add_module_behavior_boundary_validator_v0_1
title: "Add module behavior boundary validator v0.1"
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
    - "scripts/check_module_behavior_boundaries.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/goals/goal_add_module_behavior_boundary_validator_v0_1.md"
    - "work/reports/2026-05-21-module-behavior-boundary-validator-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
acceptance:
  - "Adds a read-only stdlib validator for the Module Control Plane behavior boundary group."
  - "Wires the validator through `scripts/check.sh module-behavior-boundaries`."
  - "Adds the validator to `scripts/check.sh module-control-suite`."
  - "Checks behavior overlay stack, user-local behavior artifact, tuning proposal, and behavior artifact promotion packet markers."
  - "Rejects obvious activation markers for auto-apply, writable overlays, capability changes, side-effect changes, runtime, resolver, writer, user-local config writer, scheduler, adapter invocation, publishing, metrics, gate writer, proofpack writer, acceptance claim, selected current behavior, secrets, private data, and executable code."
  - "Updates the Module Control Plane contract and eval spec with the focused validator command."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, adapter invocation, publishing, metrics collector, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "evals/specs/module-control-plane.v0.1.md"
report_refs:
  - "work/reports/2026-05-21-module-behavior-boundary-validator-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic marker checker for the already documented Module Control Plane behavior boundary group. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, metrics, or research execution behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_module_behavior_boundaries.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only focused validator for overlay, user-local artifact, tuning proposal, and promotion boundaries without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The Module Control Plane contract now contains several behavior-changing
artifact boundaries:

- behavior overlay stack;
- user-local behavior artifact;
- tuning proposal;
- behavior artifact promotion packet.

They are covered by the broad `module-control-plane` marker check, but the
behavior boundary group needs a focused validator so future edits do not
silently weaken no-auto-apply, no-writer, no-capability-change, no-side-effect,
and non-authority markers.

## Intent

Add a read-only validator:

```text
scripts/check.sh module-behavior-boundaries
```

Also include it in:

```text
scripts/check.sh module-control-suite
```

## In scope

- New stdlib Python validator.
- `scripts/check.sh` target.
- Aggregate suite wiring.
- Module Control Plane contract/spec validation-command notes.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- PubPunk runtime.
- Manifest parser.
- Config resolver.
- User-local config writer.
- Behavior-artifact writer.
- Reflection scheduler.
- Adapter invocation.
- Browser automation.
- Credential access.
- External publishing.
- Metrics collection.
- Receipt writing.
- Event writing.
- Gate writer.
- Proofpack writer.
- Acceptance claim.

## Outcome

Done in
`work/reports/2026-05-21-module-behavior-boundary-validator-v0-1.md`.

The patch adds a read-only focused behavior boundary check only.
