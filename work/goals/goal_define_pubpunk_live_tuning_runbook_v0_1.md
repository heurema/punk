---
id: goal_define_pubpunk_live_tuning_runbook_v0_1
title: "Define PubPunk live tuning runbook v0.1"
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
    - "docs/modules/pubpunk-live-tuning-runbook.md"
    - "evals/specs/pubpunk-live-tuning-runbook.v0.1.md"
    - "docs/modules/pubpunk.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/goals/goal_define_pubpunk_live_tuning_runbook_v0_1.md"
    - "work/reports/2026-05-21-pubpunk-live-tuning-runbook-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
acceptance:
  - "Adds an advisory PubPunk live tuning runbook for operator-triggered customization during active work."
  - "Defines the request, evidence plan, evidence packet, tuning proposal, approval, decision routing, provenance, and handoff packet steps."
  - "Records stop conditions for missing evidence, unretained metrics/research refs, capability grants, side-effect expansion, immutable-core changes, secrets, private data, executable code, and silent self-modification."
  - "Adds deterministic eval cases for the runbook boundary."
  - "Links the runbook from the PubPunk module doc and documentation map."
  - "Adds no runtime, parser, resolver, config writer, behavior-artifact writer, scheduler, article reader, metrics collector, research runner, adapter invocation, publishing, event writer, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/modules/pubpunk-live-tuning-runbook.md"
  - "evals/specs/pubpunk-live-tuning-runbook.v0.1.md"
  - "docs/modules/pubpunk.md"
  - "docs/product/DOCUMENTATION-MAP.md"
report_refs:
  - "work/reports/2026-05-21-pubpunk-live-tuning-runbook-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This defines a practical runbook for adaptive PubPunk behavior tuning. The R2 requirement is satisfied by the existing adaptive module control-plane research note; this slice adds no runtime, writer, resolver, scheduler, metrics, or research execution behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/pubpunk-live-tuning-runbook.md"
    - "evals/specs/pubpunk-live-tuning-runbook.v0.1.md"
    - "docs/modules/pubpunk.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds an advisory runbook and eval boundary for live PubPunk tuning without activating runtime, parsers, config writers, resolvers, behavior-artifact writers, schedulers, article readers, metrics collectors, research runners, adapter invocation, publishing, event writing, gate writers, proofpack writers, or acceptance claims."
---

## Context

The Module Control Plane and PubPunk fixtures define the contract, but operators
need a practical path for live tuning requests such as article hook adjustment.

## Intent

Add a runbook for manual PubPunk live tuning.

## In scope

- PubPunk live tuning runbook.
- Eval/spec cases for the runbook boundary.
- PubPunk module doc and documentation map links.
- Work report and status note.

## Out of scope

- Rust code.
- Scripts or validators.
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

Done in `work/reports/2026-05-21-pubpunk-live-tuning-runbook-v0-1.md`.

The patch adds advisory process documentation only.
