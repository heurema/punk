---
id: goal_consolidate_module_control_marker_validators_v0_1
title: "Consolidate Module Control marker validators v0.1"
status: done
owner: "vitaly"
module: "module-control-plane"
priority: P2
authority: canonical
created_at: 2026-05-22
updated_at: 2026-05-22
selected_at: 2026-05-22
started_at: 2026-05-22
completed_at: 2026-05-22
blocked_by: []
scope:
  include:
    - "scripts/check_module_control_markers.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-VALIDATION.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-validation.v0.1.md"
    - "work/goals/goal_consolidate_module_control_marker_validators_v0_1.md"
    - "work/reports/2026-05-22-module-control-marker-validator-consolidation-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
acceptance:
  - "Consolidates the new Module Control marker validators into one read-only stdlib Python checker."
  - "Keeps existing `scripts/check.sh` targets for focused validation ergonomics."
  - "Routes all Module Control marker targets through `scripts/check_module_control_markers.py`."
  - "Routes `scripts/check.sh module-control-suite` through the same consolidated checker."
  - "Removes the separate untracked `check_module_*` and `check_pubpunk_*` marker validator files from the working tree."
  - "Documents the policy that Python is allowed only for static repo-governance and docs/spec marker checks."
  - "Documents that runtime, parser, resolver, writer, module behavior, and implementation-state checks belong in Rust or future Rust `xtask`."
  - "Adds no new validation semantics beyond consolidating existing marker checks and policy wording."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, article reader, metrics collector, research runner, adapter invocation, publishing, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-VALIDATION.md"
  - "evals/specs/module-control-validation.v0.1.md"
report_refs:
  - "work/reports/2026-05-22-module-control-marker-validator-consolidation-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes:
  - "work/goals/goal_add_module_control_plane_validator_v0_1.md"
  - "work/goals/goal_add_module_behavior_boundary_validator_v0_1.md"
  - "work/goals/goal_add_module_control_suite_validator_target_v0_1.md"
  - "work/goals/goal_add_module_control_validation_runbook_v0_1.md"
  - "work/goals/goal_add_module_tuning_handoff_template_v0_1.md"
  - "work/goals/goal_add_pubpunk_control_manifest_validator_v0_1.md"
  - "work/goals/goal_add_pubpunk_article_hook_tuning_fixture_validator_v0_1.md"
  - "work/goals/goal_add_pubpunk_live_tuning_pack_validator_v0_1.md"
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This consolidates marker-check tooling for the already researched adaptive module control-plane validation pack. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, metrics, or research execution behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_module_control_markers.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-VALIDATION.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-validation.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Consolidates read-only Module Control marker validators and documents Python/Rust validation boundaries without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The Module Control Plane side-track introduced several focused read-only marker
validators. That preserved validation but created Python file sprawl inside a
Rust repository.

Existing project tooling already uses Python for repo-governance checks, so the
right hygiene step is consolidation and an explicit language boundary, not a
Rust rewrite or blind removal.

## Intent

Consolidate the new marker validators into:

```text
scripts/check_module_control_markers.py
```

Keep existing user-facing check targets:

```text
scripts/check.sh module-control-plane
scripts/check.sh module-behavior-boundaries
scripts/check.sh module-tuning-handoff-template
scripts/check.sh pubpunk-control-manifest
scripts/check.sh pubpunk-hook-tuning-fixture
scripts/check.sh pubpunk-live-tuning-pack
scripts/check.sh module-control-validation
scripts/check.sh module-control-suite
```

## In scope

- One consolidated stdlib Python marker checker.
- `scripts/check.sh` dispatch changes.
- Module Control validation runbook policy update.
- Module Control validation eval/spec update.
- Documentation Map wording update.
- Work report and status note.

## Out of scope

- Rust code.
- Rust `xtask` implementation.
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
`work/reports/2026-05-22-module-control-marker-validator-consolidation-v0-1.md`.

The patch consolidates marker tooling only.
