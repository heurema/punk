---
id: goal_define_pubpunk_control_manifest_fixture_v0_1
title: "Define PubPunk control manifest fixture v0.1"
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
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/goals/goal_define_pubpunk_control_manifest_fixture_v0_1.md"
    - "work/reports/2026-05-21-pubpunk-control-manifest-fixture-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-publish-*.md"
    - "scripts/**"
acceptance:
  - "Defines a PubPunk-specific module control manifest fixture."
  - "Uses `docs/modules/pubpunk-workspace-instructions.md` as the only current behavior artifact."
  - "Keeps capability grants empty and denies publishing, credentials, adapters, browser automation, metrics, direct event-log writes, final decisions, and proofpacks."
  - "Keeps reflection trigger `none` and `auto_apply: false`."
  - "Adds deterministic eval/spec coverage without implementing a parser, checker, writer, scheduler, runtime, adapter, or side effect."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
  - "docs/modules/pubpunk-control-manifest.md"
report_refs:
  - "work/reports/2026-05-21-pubpunk-control-manifest-fixture-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "The fixture applies the researched module control plane to PubPunk. The R2 requirement is satisfied by the adaptive module control plane research note."
  research_refs:
    - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  external_research_refs:
    - "https://12factor.net/config"
    - "https://kubernetes.io/docs/concepts/architecture/controller/"
    - "https://kubernetes.io/docs/concepts/extend-kubernetes/api-extension/custom-resources/"
    - "https://www.openpolicyagent.org/docs/latest/management-decision-logs/"
    - "https://opentelemetry.io/docs/what-is-opentelemetry/"
    - "https://openfeature.dev/specification/"
    - "https://code.visualstudio.com/api/references/extension-manifest"
    - "https://docs.anthropic.com/en/docs/claude-code/skills"
    - "https://docs.langchain.com/oss/python/langgraph/memory"
    - "https://arxiv.org/abs/2303.17651"
    - "https://arxiv.org/abs/2305.11738"
    - "https://arxiv.org/abs/2308.00352"
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/pubpunk-control-manifest.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/pubpunk-control-manifest.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a PubPunk-specific advisory control manifest fixture without activating runtime, adapters, reflection automation, behavior-artifact writers, or side effects."
---

## Context

The module-general control manifest and tuning proposal contract now exists.
The next bounded PubPunk step is to instantiate it as a fixture without
changing PubPunk runtime behavior.

## Intent

Define PubPunk's first control manifest fixture:

- one current behavior artifact;
- immutable core boundaries;
- empty grants;
- denied side effects and authority-bearing capabilities;
- reflection disabled;
- no current tuning proposal.

## In scope

- PubPunk control manifest fixture doc.
- PubPunk control manifest eval/spec.
- Documentation map link.
- Work report and status note.

## Out of scope

- Rust code.
- Module Host runtime.
- Manifest parser.
- Deterministic checker implementation.
- Background reflection scheduler.
- Behavior-artifact writer.
- `.punk` runtime or derived state.
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

Done in `work/reports/2026-05-21-pubpunk-control-manifest-fixture-v0-1.md`.

The patch adds a PubPunk fixture and eval/spec only.
