---
id: goal_add_pubpunk_publish_receipt_preflight_resolved_connector_refs_v0_1
title: "Add PubPunk Publish Receipt Preflight Resolved Connector Refs v0.1"
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
    - "evals/specs/pubpunk-publish-receipt-preflight.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_publish_receipt_preflight_resolved_connector_refs_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-publish-receipt-preflight-resolved-connector-refs-v0-1.md"
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
    - "API calls"
    - "adapter invocation"
    - "module runtime behavior"
acceptance:
  - "Extends the side-effect-free PubPunk publish receipt preflight packet so readiness requires explicit connector profile resolution, connector profile, and selected connector strategy refs."
  - "Requires those connector refs in allowed source refs before projecting receipt-writer preflight refs."
  - "Requires expected receipt fields for connector profile resolution, connector profile ref, and selected connector strategy."
  - "Keeps the existing Module Host side-effect receipt writer chain preflight-only."
  - "Keeps PubPunk advisory and side-effect-free: no PubPunk runtime, CLI, module invocation, filesystem reads, API calls, browser opens, credential reads, publishing, metrics collection, receipt writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Updates the eval spec, PubPunk docs, crate status, report, and work status."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-channel-connector-profile-resolution.v0.1.md"
  - "evals/specs/pubpunk-publish-request-packet.v0.1.md"
  - "evals/specs/pubpunk-publish-receipt-preflight.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-channel-connector-profile-resolution-v0-1.md"
  - "work/reports/2026-05-20-pubpunk-publish-request-resolved-connector-refs-v0-1.md"
  - "work/reports/2026-05-20-pubpunk-publish-receipt-preflight-resolved-connector-refs-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-publish-receipt-preflight-resolved-connector-refs-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating PubPunk receipt-preflight boundary before runtime publishing. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, the connector profile resolution spec, and the prior resolved-connector publish request slice."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-channel-connector-profile-resolution.v0.1.md"
    - "evals/specs/pubpunk-publish-request-packet.v0.1.md"
    - "evals/specs/pubpunk-publish-receipt-preflight.v0.1.md"
    - "work/reports/2026-05-20-pubpunk-publish-request-resolved-connector-refs-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "docs/modules/**"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Extends PubPunk publish receipt preflight readiness to require resolved connector refs without PubPunk runtime activation, browser/API calls, adapter invocation, metrics collection, receipt writing, or external publishing."
---

# Add PubPunk Publish Receipt Preflight Resolved Connector Refs v0.1

## Context

PubPunk publish requests now require resolved connector refs. The next safe
step is to carry the same connector-resolution evidence into the receipt
preflight boundary so the future receipt writer path cannot lose how the
connector was selected.

## Selected slice

Extend one pure PubPunk model plus the existing smoke eval case:

- require explicit connector profile resolution, connector profile, and
  selected connector strategy refs;
- require those refs in allowed source refs;
- require connector evidence fields in expected receipt fields;
- project those refs through `PubPunkPublishReceiptWriterPreflightRefs`;
- keep the existing Module Host receipt writer chain preflight-only.

## Boundary

This slice changes no PubPunk runtime, Module Host runtime, public CLI,
workspace initialization, filesystem scanning, draft body reads, provider
orchestration, adapter invocation, browser automation, API calls, external
publishing, metrics collection, token collection, receipt writing,
operation-evidence persistence, gate writing, proofpack writing, or acceptance
claims.

## Outcome

Done when the packet model, unit tests, smoke eval case, eval spec, docs,
report, and work status are updated, checks pass, and `selected_next` remains
unchanged.
