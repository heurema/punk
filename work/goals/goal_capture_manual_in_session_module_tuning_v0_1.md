---
id: goal_capture_manual_in_session_module_tuning_v0_1
title: "Capture manual in-session module tuning v0.1"
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
    - "work/goals/goal_capture_manual_in_session_module_tuning_v0_1.md"
    - "work/reports/2026-05-21-manual-in-session-module-tuning-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
acceptance:
  - "Defines manual in-session tuning as an operator-triggered request during active work."
  - "Keeps manual tuning proposal-only until an explicit selected edit or gate promotion path records provenance."
  - "Requires local evidence refs, metrics refs when used, and external research refs only when the work order grants external research."
  - "Clarifies that operator approval is promotion input, not module self-apply authority."
  - "Updates PubPunk control manifest fixture without enabling reflection, runtime behavior, behavior-artifact writing, publishing, metrics collection, adapter invocation, gate writing, proofpack writing, or acceptance claims."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "docs/modules/pubpunk-control-manifest.md"
report_refs:
  - "work/reports/2026-05-21-manual-in-session-module-tuning-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This refines the module control-plane architecture for behavior-changing manual tuning requests. The R2 requirement is satisfied by the existing adaptive module control plane research note; this slice adds no new runtime behavior."
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
  rationale: "Adds the manual in-session tuning boundary to the module control-plane contract without activating runtime, reflection automation, behavior-artifact writers, publishing, metrics, adapters, gate writers, proofpack writers, or acceptance claims."
---

## Context

The maintainer clarified that some module customization happens during live
work. An operator may notice that a module needs a new hook, style rule, or
workflow preference, then ask the module to analyze recent outputs, metrics,
and optional external research before proposing a change.

## Intent

Capture this as manual in-session tuning:

```text
operator request
  -> explicit evidence/research refs
  -> tuning proposal
  -> explicit approval or promotion path
  -> promoted behavior artifact with provenance
```

## In scope

- Module Control Plane wording for manual in-session tuning.
- Module Control Plane eval cases.
- PubPunk control manifest fixture wording.
- PubPunk control manifest eval case.
- Documentation map note.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- Manifest parser.
- Deterministic checker implementation.
- Background reflection scheduler.
- Behavior-artifact writer.
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

Done in
`work/reports/2026-05-21-manual-in-session-module-tuning-v0-1.md`.

The patch adds docs/eval refinement only.
