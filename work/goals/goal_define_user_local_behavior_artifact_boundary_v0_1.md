---
id: goal_define_user_local_behavior_artifact_boundary_v0_1
title: "Define user-local behavior artifact boundary v0.1"
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
    - "work/goals/goal_define_user_local_behavior_artifact_boundary_v0_1.md"
    - "work/reports/2026-05-21-user-local-behavior-artifact-boundary-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
acceptance:
  - "Defines user-local behavior artifacts as private customization scoped to one operator or installation."
  - "Keeps user-local behavior artifacts outside repo/project truth by default."
  - "Requires explicit refs, storage policy, source request/proposal refs, provenance, rollback, and export status."
  - "Prevents user-local behavior artifacts from containing secrets, sensitive personal data, executable code, adapters, browser automation, credential access, publishing, metrics collection, gates, proofpacks, or acceptance behavior."
  - "Requires a behavior artifact promotion packet before export to project/workspace behavior."
  - "Updates PubPunk control manifest fixture without selecting a current user-local artifact or enabling config writing."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "docs/modules/pubpunk-control-manifest.md"
report_refs:
  - "work/reports/2026-05-21-user-local-behavior-artifact-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This refines module user-local customization architecture. The R2 requirement is satisfied by the existing adaptive module control plane research note; this slice adds no runtime config writer, resolver, or loader."
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
  rationale: "Adds user-local behavior artifact boundaries without activating runtime, config writers, resolvers, behavior-artifact writers, gate writers, proofpack writers, or acceptance claims."
---

## Context

The overlay stack separates packaged, project/workspace, user-local, and
run-local behavior. The next missing contract is the concrete shape of a
user-local behavior artifact created from a user's customization request.

## Intent

Define the private customization artifact:

```text
voice/text request
  -> tuning proposal
  -> local/private behavior artifact
  -> visible provenance in module outputs
  -> optional export through promotion packet
```

## In scope

- Module Control Plane user-local artifact shape.
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
- User-local config writer.
- Behavior-artifact writer.
- Secret store.
- Sync protocol.
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
`work/reports/2026-05-21-user-local-behavior-artifact-boundary-v0-1.md`.

The patch adds docs/eval refinement only.
