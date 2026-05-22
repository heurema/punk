---
id: goal_add_module_tuning_handoff_template_v0_1
title: "Add Module Tuning Handoff Template v0.1"
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
    - "docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md"
    - "evals/specs/module-tuning-handoff-template.v0.1.md"
    - "scripts/check_module_tuning_handoff_template.py"
    - "scripts/check_module_control_validation.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/MODULE-CONTROL-VALIDATION.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-validation.v0.1.md"
    - "work/goals/goal_add_module_tuning_handoff_template_v0_1.md"
    - "work/reports/2026-05-21-module-tuning-handoff-template-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
acceptance:
  - "Adds a generic proposal-only Module Tuning Handoff Template for text, voice, and scheduled-proposal module tuning sessions."
  - "Adds an eval spec for the generic template."
  - "Adds a read-only stdlib validator exposed as `scripts/check.sh module-tuning-handoff-template`."
  - "Adds the validator to `scripts/check.sh module-control-suite`."
  - "Updates Module Control validation runbook, command wiring, suite membership, and validator."
  - "Updates Module Control Plane related refs and Documentation Map truth-surface refs."
  - "Template captures session, evidence packet, tuning proposal, decision record, handoff packet, completion checklist, and validation commands."
  - "Template records that natural-language requests, generated proposals, and operator approval are not auto-apply authority."
  - "Rejects obvious activation markers for auto-apply, approval-as-auto-apply, capability changes, side-effect changes, runtime, parser, resolver, writer, user-local config writer, scheduler, metrics, external research, adapter invocation, publishing, event writing, gate writer, proofpack writer, acceptance claim, selected current behavior, secrets, private data, and executable code."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, article reader, metrics collector, research runner, adapter invocation, publishing, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md"
  - "evals/specs/module-tuning-handoff-template.v0.1.md"
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "docs/product/MODULE-CONTROL-VALIDATION.md"
report_refs:
  - "work/reports/2026-05-21-module-tuning-handoff-template-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic generic handoff template and marker checker for already researched adaptive module tuning. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, metrics, or research execution behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md"
    - "evals/specs/module-tuning-handoff-template.v0.1.md"
    - "scripts/check_module_tuning_handoff_template.py"
    - "scripts/check_module_control_validation.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/MODULE-CONTROL-VALIDATION.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-validation.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only generic module tuning handoff template and validator without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, adapter invocation, publishing, metrics, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

PubPunk now has a live tuning handoff template, but the underlying pattern is
not PubPunk-specific. Any adjustable module needs a generic proposal-only
handoff shape for text, voice, or scheduled-proposal tuning requests.

## Intent

Add a generic template and validator:

```text
scripts/check.sh module-tuning-handoff-template
```

Also include it in:

```text
scripts/check.sh module-control-suite
```

## In scope

- New generic Module Tuning Handoff Template.
- New eval spec.
- New stdlib Python validator.
- `scripts/check.sh` target.
- Aggregate suite wiring.
- Module Control validation runbook update.
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
`work/reports/2026-05-21-module-tuning-handoff-template-v0-1.md`.

The patch adds a generic proposal-only tuning handoff template and read-only
marker checker only.
