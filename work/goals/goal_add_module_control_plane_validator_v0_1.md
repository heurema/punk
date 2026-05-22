---
id: goal_add_module_control_plane_validator_v0_1
title: "Add Module Control Plane validator v0.1"
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
    - "scripts/check_module_control_plane.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/goals/goal_add_module_control_plane_validator_v0_1.md"
    - "work/reports/2026-05-21-module-control-plane-validator-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
acceptance:
  - "Adds a read-only stdlib validator for the Module Control Plane contract and eval spec."
  - "Wires the validator through `scripts/check.sh module-control-plane`."
  - "Checks required manifest, manual tuning, overlay stack, user-local artifact, tuning proposal, and promotion packet markers."
  - "Rejects obvious activation markers for auto-apply, runtime, manifest parser, resolver, writer, scheduler, adapter invocation, publishing, metrics, gate writer, proofpack writer, acceptance claim, secrets, private data, and executable code."
  - "Updates the contract and eval spec with the validator command."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, adapter invocation, publishing, metrics collector, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "evals/specs/module-control-plane.v0.1.md"
report_refs:
  - "work/reports/2026-05-21-module-control-plane-validator-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic marker checker for the adaptive module control-plane contract. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, or scheduler behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_module_control_plane.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only validator and documents its command without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, gate writers, proofpack writers, or acceptance claims."
---

## Context

The Module Control Plane contract now defines the general adaptive-module
boundary. It needs a small deterministic check so future edits do not silently
drop the proposal-only, overlay, user-local, or promotion boundaries.

## Intent

Add a read-only validator:

```text
scripts/check.sh module-control-plane
```

## In scope

- New stdlib Python validator.
- `scripts/check.sh` target.
- Contract/spec validation-command notes.
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

Done in `work/reports/2026-05-21-module-control-plane-validator-v0-1.md`.

The patch adds a read-only contract validator only.
