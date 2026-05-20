---
id: goal_add_pubpunk_inventory_input_packet_v0_1
title: "Add PubPunk Inventory Input Packet v0.1"
status: done
owner: "vitaly"
module: "product"
priority: P1
authority: canonical
created_at: 2026-05-19
updated_at: 2026-05-19
selected_at: 2026-05-19
started_at: 2026-05-19
completed_at: 2026-05-19
blocked_by: []
scope:
  include:
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_inventory_input_packet_v0_1.md"
    - "work/reports/2026-05-19-pubpunk-inventory-input-packet-v0-1.md"
  exclude:
    - ".punk/**"
    - ".github/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-module-host/**"
    - "publishing/**"
    - "external publishing behavior"
    - "module runtime behavior"
acceptance:
  - "Adds a PubPunk inventory input packet before the existing inventory assessment model."
  - "Requires canonical pubpunk module id, split explicit refs workspace policy, explicit workspace refs, required instruction refs, allowed source refs, assess_provided_inventory grant, expected receipt fields, metadata-only privacy, and safe optional token-cost refs."
  - "Allows conversion to PubPunkInventoryInput only when the packet assessment has no blockers."
  - "Keeps PubPunk advisory and side-effect-free: no runtime, CLI, workspace scan, file reads, publishing, metrics collection, receipt writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds smoke and unit coverage for the input packet boundary."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
  - "work/reports/2026-05-19-pubpunk-inventory-input-packet-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-pubpunk-inventory-input-packet-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating domain module model and its eval/docs boundary. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, and the PubPunk workspace instruction packet."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-workspace-instruction-packet.v0.1.md"
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
  rationale: "Adds an incubating PubPunk module input-packet model and updates docs/eval/work-ledger boundaries without activating runtime behavior."
---

# Add PubPunk Inventory Input Packet v0.1

## Context

PubPunk already had a side-effect-free inventory assessment model over
caller-provided metadata. The missing step was a deterministic packet boundary
for what a host or agent must pass before the module can assess inventory.

## Selected slice

Add one pure input packet model:

- validates explicit workspace and instruction refs;
- keeps item refs inside allowed source refs;
- requires the narrow `assess_provided_inventory` grant;
- carries optional token-cost refs as metadata only;
- converts to `PubPunkInventoryInput` only when packet readiness is unblocked.

## Boundary

This slice changes no public CLI behavior, Module Host runtime behavior, module
activation, provider orchestration, adapter invocation, workspace
initialization, filesystem scanning, draft body reads, publishing behavior,
metrics collection, token collection, receipt writing, gate writing, proofpack
writing, or acceptance claims.

## Outcome

Done when the input packet model, smoke coverage, docs, eval spec, report, and
work status are updated, checks pass, and `selected_next` remains unchanged.
