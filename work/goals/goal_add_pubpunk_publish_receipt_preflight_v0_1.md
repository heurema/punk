---
id: goal_add_pubpunk_publish_receipt_preflight_v0_1
title: "Add PubPunk Publish Receipt Preflight v0.1"
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
    - "evals/specs/pubpunk-publish-receipt-preflight.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_publish_receipt_preflight_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-publish-receipt-preflight-v0-1.md"
  exclude:
    - ".punk/**"
    - ".github/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-module-host/**"
    - "publishing/**"
    - "external publishing behavior"
    - "adapter invocation"
    - "module runtime behavior"
acceptance:
  - "Adds a side-effect-free PubPunk publish receipt preflight packet model with explicit receipt target, storage, operation-evidence, idempotency, rollback, error, adapter invocation receipt, connector, channel, and payload refs."
  - "Requires the narrow request_external_publish grant, metadata-only privacy, expected receipt fields, and allowed source refs before projecting host receipt-writer preflight refs."
  - "Adds unit and smoke coverage that chains the ready packet to the existing Module Host side-effect receipt writer preflight model."
  - "Keeps all models advisory and side-effect-free: no runtime, CLI, module invocation, filesystem reads, publishing, metrics collection, receipt writing, operation-evidence persistence, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds an eval spec and updates PubPunk docs, crate status, documentation map, report, and work status."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-publish-request-packet.v0.1.md"
  - "evals/specs/pubpunk-publish-receipt-preflight.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-publish-receipt-preflight-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-publish-receipt-preflight-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating PubPunk-to-Module-Host receipt preflight boundary. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, the publish request packet spec, and the existing Module Host receipt writer preflight model."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-publish-request-packet.v0.1.md"
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
  rationale: "Adds PubPunk publish receipt preflight evidence without runtime activation or receipt writing."
---

# Add PubPunk Publish Receipt Preflight v0.1

## Context

PubPunk can prepare a publish request packet and chain it into existing Module
Host side-effect request and policy-gate preflight models. The next missing
link was a PubPunk-owned preflight packet that prepares explicit refs for the
existing host receipt-writer preflight model without writing the receipt.

## Selected slice

Add one pure PubPunk model plus one smoke eval case:

- require explicit receipt target, storage, operation-evidence, idempotency,
  rollback, error, adapter invocation receipt, connector, channel, and payload
  refs;
- require the narrow `request_external_publish` grant;
- project receipt-writer preflight refs only when the packet is ready;
- chain the projected refs into the existing Module Host side-effect receipt
  writer preflight model.

## Boundary

This slice changes no PubPunk runtime, Module Host runtime, public CLI,
workspace initialization, filesystem scanning, draft body reads, provider
orchestration, adapter invocation, external publishing, metrics collection,
token collection, receipt writing, operation-evidence persistence, gate
writing, proofpack writing, or acceptance claims.

## Outcome

Done when the packet model, unit tests, smoke eval case, eval spec, docs,
report, and work status are updated, checks pass, and `selected_next` remains
unchanged.
