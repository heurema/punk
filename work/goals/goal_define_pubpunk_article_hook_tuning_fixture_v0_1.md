---
id: goal_define_pubpunk_article_hook_tuning_fixture_v0_1
title: "Define PubPunk article hook tuning fixture v0.1"
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
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
    - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
    - "docs/modules/pubpunk-control-manifest.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "docs/modules/pubpunk.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/goals/goal_define_pubpunk_article_hook_tuning_fixture_v0_1.md"
    - "work/reports/2026-05-21-pubpunk-article-hook-tuning-fixture-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
acceptance:
  - "Defines a concrete non-applying PubPunk article hook tuning fixture."
  - "Models operator request, evidence refs, tuning proposal, user-local hook profile, promotion packet, and resolved behavior set."
  - "Marks all non-doc refs as example-only and does not claim real article reads, metrics collection, or external research."
  - "Keeps the fixture outside current PubPunk behavior and project truth."
  - "Adds deterministic eval/spec cases for fixture completeness and safety."
  - "Adds no runtime, config writer, resolver, article reader, metrics collector, external research runner, publishing, adapter invocation, browser automation, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
  - "docs/modules/pubpunk-control-manifest.md"
report_refs:
  - "work/reports/2026-05-21-pubpunk-article-hook-tuning-fixture-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This fixture demonstrates behavior-changing PubPunk customization through the module control-plane contract. The R2 requirement is satisfied by the existing adaptive module control plane research note; this slice adds no runtime or writer behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/modules/pubpunk.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a concrete non-applying PubPunk tuning fixture and eval cases without activating runtime, user-local config writing, metrics, external research, publishing, adapters, gate writers, proofpack writers, or acceptance claims."
---

## Context

The Module Control Plane now defines manual tuning, user-local behavior
artifacts, promotion packets, and overlay stacks. PubPunk needs one concrete
fixture that shows how those pieces compose for article hook tuning.

## Intent

Model this chain:

```text
operator request
  -> evidence refs
  -> tuning proposal
  -> user-local hook profile
  -> promotion packet
  -> resolved behavior set
```

## In scope

- PubPunk article hook tuning fixture doc.
- PubPunk article hook tuning fixture eval/spec.
- Links from PubPunk control manifest, PubPunk doc, and documentation map.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- PubPunk runtime.
- Manifest parser.
- Config resolver.
- User-local config writer.
- Behavior-artifact writer.
- Article reading.
- Real metrics collection.
- External research execution.
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
`work/reports/2026-05-21-pubpunk-article-hook-tuning-fixture-v0-1.md`.

The patch adds docs/eval fixture coverage only.
