---
id: goal_add_pubpunk_inventory_reader_v0_1
title: "Add PubPunk Inventory Reader v0.1"
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
    - "evals/specs/pubpunk-inventory-reader.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_inventory_reader_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-inventory-reader-v0-1.md"
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
  - "Adds a side-effect-free PubPunk inventory reader model before PubPunkInventoryInputPacket."
  - "Requires canonical pubpunk module id, split explicit refs workspace policy, explicit workspace refs, required instruction refs, allowed source refs, read_workspace_metadata grant, expected receipt fields, metadata-only privacy, and safe optional token-cost refs."
  - "Builds PubPunkInventoryInputPacket only when reader assessment has no blockers."
  - "Allows empty observed inventories for new projects."
  - "Keeps PubPunk advisory and side-effect-free: no runtime, CLI, workspace initialization, filesystem scanning, draft body reads, publishing, metrics collection, receipt writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds smoke and unit coverage for the reader boundary."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
  - "evals/specs/pubpunk-host-handoff.v0.1.md"
  - "evals/specs/pubpunk-inventory-reader.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-inventory-reader-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-inventory-reader-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating PubPunk module model and its eval/docs boundary. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, and the completed PubPunk input-packet and host-handoff specs."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
    - "evals/specs/pubpunk-host-handoff.v0.1.md"
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
  rationale: "Adds an incubating PubPunk inventory reader model and updates docs/eval/work-ledger boundaries without activating runtime behavior."
---

# Add PubPunk Inventory Reader v0.1

## Context

PubPunk can now validate an explicit input packet, assess it, and pass advisory
evidence through Module Host models. The missing boundary is the safe step that
turns explicit observed publishing refs into the input packet.

## Selected slice

Add one pure reader model:

- validates explicit workspace and instruction refs;
- keeps observed item refs inside allowed source refs;
- requires the narrow `read_workspace_metadata` grant;
- allows empty observed inventories for new projects;
- carries optional token-cost refs as metadata only;
- builds `PubPunkInventoryInputPacket` only when reader readiness is unblocked.

## Boundary

This slice changes no public CLI behavior, Module Host runtime behavior, module
activation, provider orchestration, adapter invocation, workspace
initialization, filesystem scanning, draft body reads, publishing behavior,
metrics collection, token collection, receipt writing, gate writing, proofpack
writing, or acceptance claims.

## Outcome

Done when the reader model, smoke coverage, docs, eval spec, report, and work
status are updated, checks pass, and `selected_next` remains unchanged.
