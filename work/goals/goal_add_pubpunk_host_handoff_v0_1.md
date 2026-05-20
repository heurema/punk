---
id: goal_add_pubpunk_host_handoff_v0_1
title: "Add PubPunk Host Handoff v0.1"
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
    - "crates/punk-eval/src/lib.rs"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/pubpunk-host-handoff.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_host_handoff_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-host-handoff-v0-1.md"
  exclude:
    - ".punk/**"
    - ".github/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-module-host/**"
    - "crates/punk-mod-pubpunk/**"
    - "publishing/**"
    - "external publishing behavior"
    - "module runtime behavior"
acceptance:
  - "Adds one deterministic smoke eval case that starts from PubPunkInventoryInputPacket."
  - "Chains packet readiness, PubPunk inventory assessment, Module Host preflight, advisory envelope, and receipt proposal."
  - "Keeps all models advisory and side-effect-free: no runtime, CLI, module invocation, filesystem reads, publishing, metrics collection, receipt writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds an eval spec and updates PubPunk docs, crate status, documentation map, report, and work status."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
  - "evals/specs/pubpunk-host-handoff.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-host-handoff-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-host-handoff-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating module-to-host eval boundary. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, and the existing PubPunk input-packet spec."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
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
  rationale: "Adds smoke evidence for a PubPunk-to-Module-Host handoff without activating runtime behavior."
---

# Add PubPunk Host Handoff v0.1

## Context

PubPunk now has an explicit inventory input packet and a side-effect-free
inventory assessment model. Module Host already has preflight, advisory
envelope, and receipt proposal models. The missing evidence was a single
deterministic chain that connects these existing pieces.

## Selected slice

Add one pure smoke eval case:

- start from `PubPunkInventoryInputPacket`;
- convert to inventory assessment input only after packet readiness;
- run the existing PubPunk inventory assessment;
- preflight an explicit Module Host invocation envelope;
- wrap the advisory output;
- model a future receipt proposal.

## Boundary

This slice changes no PubPunk runtime, Module Host runtime, public CLI,
workspace initialization, filesystem scanning, draft body reads, provider
orchestration, adapter invocation, external publishing, metrics collection,
token collection, receipt writing, gate writing, proofpack writing, or
acceptance claims.

## Outcome

Done when the smoke eval case, eval spec, docs, report, and work status are
updated, checks pass, and `selected_next` remains unchanged.
