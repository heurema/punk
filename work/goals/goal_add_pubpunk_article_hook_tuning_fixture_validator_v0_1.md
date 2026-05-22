---
id: goal_add_pubpunk_article_hook_tuning_fixture_validator_v0_1
title: "Add PubPunk article hook tuning fixture validator v0.1"
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
    - "scripts/check_pubpunk_hook_tuning_fixture.py"
    - "scripts/check.sh"
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
    - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
    - "work/goals/goal_add_pubpunk_article_hook_tuning_fixture_validator_v0_1.md"
    - "work/reports/2026-05-21-pubpunk-article-hook-tuning-fixture-validator-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/**"
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/modules/pubpunk.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
acceptance:
  - "Adds a read-only stdlib validator for the PubPunk article hook tuning fixture."
  - "Wires the validator through `scripts/check.sh pubpunk-hook-tuning-fixture`."
  - "Checks required request, evidence, proposal, user-local artifact, promotion packet, and resolved behavior set markers."
  - "Rejects obvious activation markers for auto-apply, selected current behavior, runtime, writer, publishing, metrics, external research, secrets, private data, and executable code."
  - "Updates fixture and eval spec with the validator command."
  - "Adds no runtime, config writer, resolver, article reader, metrics collector, external research runner, publishing, adapter invocation, browser automation, gate, proofpack, or acceptance behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
  - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
report_refs:
  - "work/reports/2026-05-21-pubpunk-article-hook-tuning-fixture-validator-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This adds a deterministic checker for behavior-changing PubPunk customization fixture completeness. The R2 requirement is satisfied by the existing adaptive module control plane research note; this slice adds no runtime, writer, or resolver behavior."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "scripts/check_pubpunk_hook_tuning_fixture.py"
    - "scripts/check.sh"
    - "docs/modules/pubpunk-article-hook-tuning-fixture.md"
    - "evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a read-only validator and documents its command without activating runtime, config writers, resolvers, behavior-artifact writers, gate writers, proofpack writers, or acceptance claims."
---

## Context

The PubPunk article hook tuning fixture is now documented. It needs a small
deterministic check so future edits do not accidentally turn the example into
active behavior or drop required chain markers.

## Intent

Add a read-only validator:

```text
scripts/check.sh pubpunk-hook-tuning-fixture
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
`work/reports/2026-05-21-pubpunk-article-hook-tuning-fixture-validator-v0-1.md`.

The patch adds a read-only fixture validator only.
