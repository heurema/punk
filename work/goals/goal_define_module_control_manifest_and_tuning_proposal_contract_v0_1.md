---
id: goal_define_module_control_manifest_and_tuning_proposal_contract_v0_1
title: "Define module control manifest and tuning proposal contract v0.1"
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
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/goals/goal_define_module_control_manifest_and_tuning_proposal_contract_v0_1.md"
    - "work/reports/2026-05-21-module-control-manifest-and-tuning-proposal-contract-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/modules/**"
    - "evals/specs/pubpunk-*.md"
    - "scripts/**"
acceptance:
  - "Defines a module-general control manifest contract with PubPunk as fixture."
  - "Defines a tuning proposal contract for behavior artifact changes."
  - "Keeps reflection propose-only and auto-apply false."
  - "Keeps capability grants, immutable core, side effects, event mutation, gate decisions, proofpacks, and acceptance outside behavior artifacts."
  - "Adds deterministic eval/spec cases without implementing a parser, checker, scheduler, writer, or runtime behavior."
  - "Updates work status only as an explicit side-track while preserving selected_next."
knowledge_refs:
  - "knowledge/research/2026-05-21-adaptive-module-control-plane.md"
  - "knowledge/ideas/2026-05-21-adaptive-module-control-plane.md"
contract_refs:
  - "docs/product/MODULE-CONTROL-PLANE.md"
report_refs:
  - "work/reports/2026-05-21-module-control-manifest-and-tuning-proposal-contract-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "The contract defines a module control manifest and tuning proposal boundary for behavior artifacts. The R2 research requirement is satisfied by the adaptive module control plane research note."
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
  classification: architecture
  required_updates:
    - "docs/product/MODULE-CONTROL-PLANE.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-control-plane.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds the canonical docs/eval contract for module control manifests and tuning proposals without activating runtime, adapters, reflection automation, or side effects."
---

## Context

The previous adaptive module control plane slice recorded the research and
direction. The next selected side-track is to define the concrete docs/eval
contract before any runtime reflection or behavior-artifact writer exists.

## Intent

Define:

- module control manifest contract;
- behavior artifact boundary;
- tuning proposal contract;
- PubPunk fixture;
- deterministic eval/spec cases.

## In scope

- New canonical product doc for the control plane.
- New eval/spec for deterministic review cases.
- Cross-links from module authoring, conformance, and documentation map.
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
`work/reports/2026-05-21-module-control-manifest-and-tuning-proposal-contract-v0-1.md`.

The patch adds docs/eval contracts only.
