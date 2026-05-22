---
id: goal_add_module_control_suite_validator_target_v0_1
title: "Add Module Control Suite validator target v0.1"
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
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/goals/goal_add_module_control_suite_validator_target_v0_1.md"
    - "work/reports/2026-05-21-module-control-suite-validator-target-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
acceptance:
  - "Adds a read-only stdlib aggregate validator target for the current Module Control Plane validation pack."
  - "Wires the target through `scripts/check.sh module-control-suite`."
  - "Runs the existing Module Control Plane, PubPunk control manifest, and PubPunk hook tuning fixture validators."
  - "Does not add new contract semantics beyond invoking the existing validators."
  - "Updates the Module Control Plane contract and eval spec with the aggregate command."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, adapter invocation, publishing, metrics collector, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "evals/specs/module-control-plane.v0.1.md"
  - "docs/modules/pubpunk-control-manifest.md"
  - "evals/specs/pubpunk-control-manifest.v0.1.md"
  - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
  - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
report_refs:
  - "work/reports/2026-05-21-module-control-suite-validator-target-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds an aggregate checker for the existing adaptive module control-plane validation pack. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, or new policy behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only aggregate validator target and documents its command without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The control-plane work now has three focused read-only validators:

- `scripts/check.sh module-control-plane`
- `scripts/check.sh pubpunk-control-manifest`
- `scripts/check.sh pubpunk-hook-tuning-fixture`

They need one aggregate target for routine validation.

## Intent

Add a read-only aggregate validator:

```text
scripts/check.sh module-control-suite
```

## In scope

- New stdlib Python aggregate runner.
- `scripts/check.sh` target.
- Contract/spec validation-command notes.
- Work report and status note.

## Out of scope

- Rust code.
- New control-plane semantics.
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
`work/reports/2026-05-21-module-control-suite-validator-target-v0-1.md`.

The patch adds a read-only aggregate check target only.
