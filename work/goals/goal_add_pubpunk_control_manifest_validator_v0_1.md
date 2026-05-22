---
id: goal_add_pubpunk_control_manifest_validator_v0_1
title: "Add PubPunk control manifest validator v0.1"
status: done
owner: "vitaly"
module: "pubpunk"
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
    - "scripts/check_pubpunk_control_manifest.py"
    - "scripts/check.sh"
    - "docs/modules/pubpunk-control-manifest.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/goals/goal_add_pubpunk_control_manifest_validator_v0_1.md"
    - "work/reports/2026-05-21-pubpunk-control-manifest-validator-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/**"
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
    - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
acceptance:
  - "Adds a read-only stdlib validator for the PubPunk control manifest fixture and eval spec."
  - "Wires the validator through `scripts/check.sh pubpunk-control-manifest`."
  - "Checks required manifest refs, behavior artifact, immutable core, empty grants, denied side effects, disabled reflection, non-current tuning, user-local, hook fixture, and promotion markers."
  - "Rejects obvious activation markers for auto-apply, runtime, manifest parser, resolver, writer, user-local config writer, reflection scheduler, adapter invocation, publishing, metrics, event writing, gate writer, proofpack writer, acceptance claim, selected current behavior, selected user-local overlay/artifact, selected promotion, and selected tuning proposal."
  - "Updates the fixture and eval spec with the validator command."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, adapter invocation, publishing, metrics collector, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/modules/pubpunk-control-manifest.md"
  - "evals/specs/pubpunk-control-manifest.v0.1.md"
report_refs:
  - "work/reports/2026-05-21-pubpunk-control-manifest-validator-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic marker checker for the PubPunk application of the adaptive module control-plane contract. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, or scheduler behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_pubpunk_control_manifest.py"
    - "scripts/check.sh"
    - "docs/modules/pubpunk-control-manifest.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only validator and documents its command without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The PubPunk control manifest fixture now applies the general Module Control
Plane contract to PubPunk. It needs a deterministic check so future edits do
not silently drop empty grants, denied side effects, disabled reflection, or
non-current tuning boundaries.

## Intent

Add a read-only validator:

```text
scripts/check.sh pubpunk-control-manifest
```

## In scope

- New stdlib Python validator.
- `scripts/check.sh` target.
- Fixture/spec validation-command notes.
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

Done in `work/reports/2026-05-21-pubpunk-control-manifest-validator-v0-1.md`.

The patch adds a read-only fixture validator only.
