---
id: goal_define_behavior_overlay_stack_v0_1
title: "Define behavior overlay stack v0.1"
status: done
owner: "vitaly"
module: "modules"
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
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/goals/goal_define_behavior_overlay_stack_v0_1.md"
    - "work/reports/2026-05-21-behavior-overlay-stack-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
acceptance:
  - "Defines behavior overlay stack layers for packaged defaults, project/workspace artifacts, user-local artifacts, and run-local overrides."
  - "Keeps immutable core outside overlays."
  - "Marks user-local overlays as not project truth unless explicitly exported and promoted."
  - "Prevents overlays from granting capabilities, expanding side effects, reading credentials, invoking adapters, publishing, writing gates, writing proofpacks, or claiming acceptance."
  - "Requires resolved behavior artifact set provenance for module outputs shaped by overlays."
  - "Updates PubPunk control manifest fixture without selecting a current user-local overlay or enabling behavior-artifact writing."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "docs/modules/pubpunk-control-manifest.md"
report_refs:
  - "work/reports/2026-05-21-behavior-overlay-stack-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This refines module behavior artifact layering architecture. The R2 requirement is satisfied by the existing adaptive module control plane research note; this slice adds no runtime resolver, config writer, or loader."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds behavior overlay stack boundaries without activating runtime, config resolvers, behavior-artifact writers, gate writers, proofpack writers, or acceptance claims."
---

## Context

Manual tuning and promotion packets define how a proposed behavior change is
reviewed. The remaining customization boundary is how installed modules can
separate packaged defaults, project/workspace guidance, user-local preferences,
and run-local overrides.

## Intent

Define the overlay stack:

```text
immutable core
  -> packaged defaults
  -> project/workspace behavior artifacts
  -> user-local behavior artifacts
  -> run-local override
```

## In scope

- Module Control Plane overlay stack shape.
- Module Control Plane eval cases.
- PubPunk control manifest fixture wording.
- PubPunk control manifest eval case.
- Documentation map note.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- Manifest parser.
- Config resolver.
- Behavior-artifact writer.
- User-local config writer.
- `.punk/` runtime or derived state.
- PubPunk runtime activation.
- Adapter invocation.
- Browser automation.
- Credential access.
- External publishing.
- Metrics collection implementation.
- Gate writer.
- Proofpack writer.
- Acceptance claim.

## Outcome

Done in `work/reports/2026-05-21-behavior-overlay-stack-v0-1.md`.

The patch adds docs/eval refinement only.
