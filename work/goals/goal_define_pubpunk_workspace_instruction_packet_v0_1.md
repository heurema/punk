---
id: goal_define_pubpunk_workspace_instruction_packet_v0_1
title: "Define PubPunk Workspace Instruction Packet v0.1"
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
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-workspace-instruction-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_pubpunk_workspace_instruction_packet_v0_1.md"
    - "work/reports/2026-05-19-pubpunk-workspace-instruction-packet-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/**"
    - "docs/product/**"
    - "docs/adr/**"
acceptance:
  - "Selects PubPunk workspace policy for the next bounded PubPunk work."
  - "Defines thin PubPunk instruction refs without creating generated instruction views."
  - "Keeps repo-native Punk publishing artifacts separate from external project-specific publishing workspaces."
  - "Selects advisory assessment fields for the next PubPunk metadata-only slice."
  - "Keeps PubPunk runtime, Module Host runtime, filesystem reads, workspace initialization, adapter invocation, external publishing, metrics collection, receipt writing, gate/proof writers, and acceptance claims inactive."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "publishing/README.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/INSTRUCTION-SOURCES.md"
  - "docs/product/DELIBERATION-BUDGET.md"
  - "evals/specs/pubpunk-conformance-packet.v0.1.md"
  - "evals/specs/pubpunk-workspace-instruction-packet.v0.1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-pubpunk-workspace-instruction-packet-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This resolves PubPunk module workspace and instruction boundaries before implementation, touching module authoring, future workspace policy, instruction policy, side-effect policy, and publishing/metrics boundaries. It is satisfied by existing PubPunk boundary docs, Module Authoring Baseline, Module Conformance Packet, Module Host Contract Stub, Instruction Sources, publishing README, and the previous PubPunk conformance packet."
  research_refs:
    - "docs/modules/pubpunk.md"
    - "publishing/README.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/INSTRUCTION-SOURCES.md"
    - "work/reports/2026-05-19-pubpunk-conformance-packet-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/**"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds PubPunk workspace and instruction packet before selecting the next PubPunk implementation slice."
---

# Define PubPunk Workspace Instruction Packet v0.1

## Context

The PubPunk conformance packet found that PubPunk needs a selected workspace
policy, module-specific instruction refs, and advisory assessment fields before
the next implementation slice is selected.

## Selected slice

Capture the PubPunk workspace and instruction packet only:

- selected workspace policy;
- thin instruction refs;
- advisory assessment fields;
- eval/spec fixture;
- work report and status note.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no PubPunk runtime, no module invocation, no provider orchestration, no
adapter invocation, no workspace initializer, no filesystem reads, no
publishing behavior, no metrics collection, no receipt writer, no gate writer,
no proofpack writer, and no acceptance claim.

## Outcome

Done when the workspace/instruction packet is recorded, prior conformance
findings are resolved or bounded, the work ledger records completion, checks
pass, and `selected_next` remains unchanged.
