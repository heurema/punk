---
id: goal_add_pubpunk_channel_connector_profile_resolution_v0_1
title: "Add PubPunk Channel Connector Profile Resolution v0.1"
status: done
owner: "vitaly"
module: "product"
priority: P1
authority: canonical
created_at: 2026-05-20
updated_at: 2026-05-20
selected_at: 2026-05-20
started_at: 2026-05-20
completed_at: 2026-05-20
blocked_by: []
scope:
  include:
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/pubpunk-channel-connector-profile-resolution.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_channel_connector_profile_resolution_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-channel-connector-profile-resolution-v0-1.md"
  exclude:
    - ".punk/**"
    - ".github/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-module-host/**"
    - "crates/punk-events/**"
    - "publishing/**"
    - "external publishing behavior"
    - "browser automation"
    - "adapter invocation"
    - "module runtime behavior"
acceptance:
  - "Adds a side-effect-free PubPunk channel connector profile resolution packet model with explicit inventory assessment, candidate, channel, connector profile, API availability, browser automation policy, manual handoff, credential signal, and payload refs."
  - "Requires the narrow resolve_connector_profile grant, metadata-only privacy, expected receipt fields, API-first strategy order, and allowed source refs before projecting resolved connector refs."
  - "Selects API first when available, browser automation only when API is unavailable and browser policy allows it, and manual handoff when automated paths are unavailable but manual fallback is allowed."
  - "Blocks readiness when no API, browser, or manual strategy is allowed."
  - "Keeps PubPunk advisory and side-effect-free: no PubPunk runtime, CLI, module invocation, filesystem reads, API calls, browser opens, credential reads, publishing, metrics collection, receipt writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds an eval spec and updates PubPunk docs, crate status, documentation map, report, and work status."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-channel-connector-profile-resolution.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-channel-connector-profile-resolution-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-channel-connector-profile-resolution-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating PubPunk connector-selection boundary before runtime publishing. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, and the provider advisory pass that recommended API-first connector profile resolution over immediate connector implementation."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "work/reports/2026-05-20-pubpunk-channel-connector-profile-resolution-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/modules/**"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds PubPunk channel connector profile resolution evidence without PubPunk runtime activation, browser/API calls, adapter invocation, metrics collection, or external publishing."
---

# Add PubPunk Channel Connector Profile Resolution v0.1

## Context

PubPunk must eventually support multiple project/channel publishing surfaces.
The next safest step is not a real connector, adapter, browser action, or
publish call. It is a side-effect-free model that selects the connector path
from explicit metadata so later publishing work cannot silently skip the
API-first, browser-policy, manual-fallback rule.

## Selected slice

Add one pure PubPunk model plus one smoke eval case:

- require explicit inventory assessment, candidate, channel, connector profile,
  API availability, browser automation policy, manual handoff, credential
  signal, and payload refs;
- require the narrow `resolve_connector_profile` grant;
- require metadata-only privacy and expected receipt fields for the selection
  boundary;
- select API, browser, or manual strategy from explicit signals;
- block when no strategy is allowed;
- project resolved connector refs only when the packet is ready.

## Boundary

This slice changes no PubPunk runtime, Module Host runtime, public CLI,
workspace initialization, filesystem scanning, draft body reads, provider
orchestration, adapter invocation, browser automation, external publishing,
metrics collection, token collection, receipt writing, gate writing, proofpack
writing, or acceptance claims.

## Outcome

Done when the packet model, unit tests, smoke eval case, eval spec, docs,
report, and work status are updated, checks pass, and `selected_next` remains
unchanged.
