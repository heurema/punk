---
id: goal_document_pubpunk_channel_connector_strategy_v0_1
title: "Document PubPunk Channel Connector Strategy v0.1"
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
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "work/STATUS.md"
    - "work/goals/goal_document_pubpunk_channel_connector_strategy_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-channel-connector-strategy-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "publishing/**"
    - "adapter implementation"
    - "browser automation implementation"
    - "credential handling"
    - "external publishing behavior"
    - "metrics collection behavior"
acceptance:
  - "Records that PubPunk should eventually support explicit connector profiles per project/channel pair."
  - "Records the preferred connector order: official/documented API first, public/read-side metrics API where useful, browser automation only when API paths are unavailable or insufficient, and manual handoff as fallback."
  - "Keeps the note future-only and does not activate connectors, adapters, browser automation, credentials, publishing, metrics collection, receipt writing, gate/proof writing, or acceptance claims."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "work/reports/2026-05-20-pubpunk-channel-connector-strategy-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-channel-connector-strategy-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a docs-only product clarification from the operator about future PubPunk channel connector strategy. It records direction without external research or implementation."
  research_refs:
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Records future PubPunk channel connector direction without activating runtime behavior."
---

# Document PubPunk Channel Connector Strategy v0.1

## Context

PubPunk needs to remember that publication automation is not only article
drafting. Projects may have many publishing channels, and each channel needs a
clear connector path before posting or metrics automation can become reliable.

## Selected slice

Add a docs-only note that PubPunk should eventually use explicit connector
profiles per project/channel pair, preferring official or documented APIs
before browser automation.

## Boundary

This slice adds no Rust code, connector implementation, adapter invocation,
browser automation, credential handling, external publishing, metrics
collection, receipt writer, gate writer, proofpack writer, or acceptance claim.

## Outcome

Done when PubPunk docs and workspace instructions record the strategy, a small
report exists, checks pass, and `selected_next` remains unchanged.
