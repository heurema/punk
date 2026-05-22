---
id: goal_add_pubpunk_live_tuning_pack_validator_v0_1
title: "Add PubPunk live tuning pack validator v0.1"
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
    - "scripts/check_pubpunk_live_tuning_pack.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/modules/pubpunk-live-tuning-runbook.md"
    - "docs/modules/pubpunk-live-tuning-handoff-template.md"
    - "docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md"
    - "evals/specs/pubpunk-live-tuning-runbook.v0.1.md"
    - "evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md"
    - "evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md"
    - "work/goals/goal_add_pubpunk_live_tuning_pack_validator_v0_1.md"
    - "work/reports/2026-05-21-pubpunk-live-tuning-pack-validator-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
acceptance:
  - "Adds a read-only stdlib validator for the PubPunk live tuning pack."
  - "Wires the validator through `scripts/check.sh pubpunk-live-tuning-pack`."
  - "Adds the validator to `scripts/check.sh module-control-suite`."
  - "Checks required runbook, handoff template, article-hook handoff example, and eval/spec markers."
  - "Rejects obvious activation markers for auto-apply, approval-as-auto-apply, selected current behavior, runtime, parser, resolver, writer, user-local config writer, adapter invocation, publishing, metrics, external research, event writing, gate writer, proofpack writer, acceptance claim, secrets, private data, and executable code."
  - "Updates live tuning docs and specs with the dedicated validator command."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, article reader, metrics collector, research runner, adapter invocation, publishing, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/modules/pubpunk-live-tuning-runbook.md"
  - "docs/modules/pubpunk-live-tuning-handoff-template.md"
  - "docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md"
  - "evals/specs/pubpunk-live-tuning-runbook.v0.1.md"
  - "evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md"
  - "evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md"
report_refs:
  - "work/reports/2026-05-21-pubpunk-live-tuning-pack-validator-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic marker checker for the adaptive PubPunk live tuning pack. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, metrics, or research execution behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_pubpunk_live_tuning_pack.py"
    - "scripts/check_module_control_suite.py"
    - "scripts/check.sh"
    - "docs/modules/pubpunk-live-tuning-runbook.md"
    - "docs/modules/pubpunk-live-tuning-handoff-template.md"
    - "docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md"
    - "evals/specs/pubpunk-live-tuning-runbook.v0.1.md"
    - "evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md"
    - "evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only live tuning pack validator and documents its command without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, article readers, metrics collectors, research runners, adapter invocation, publishing, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The PubPunk live tuning runbook, handoff template, and article-hook handoff
example now exist. They need a deterministic read-only check so future edits do
not silently drop no-auto-apply, no-runtime, example-only, or evidence
limitation markers.

## Intent

Add a read-only validator:

```text
scripts/check.sh pubpunk-live-tuning-pack
```

Also include it in:

```text
scripts/check.sh module-control-suite
```

## In scope

- New stdlib Python validator.
- `scripts/check.sh` target.
- Aggregate suite wiring.
- Live tuning docs/spec validation-command notes.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- PubPunk runtime.
- Manifest parser.
- Config resolver.
- User-local config writer.
- Behavior-artifact writer.
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
`work/reports/2026-05-21-pubpunk-live-tuning-pack-validator-v0-1.md`.

The patch adds a read-only live tuning pack validator only.
