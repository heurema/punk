---
id: goal_add_pubpunk_post_draft_request_packet_v0_1
title: "Add PubPunk post draft request packet v0.1"
status: ready
owner: "vitaly"
module: "pubpunk"
priority: P1
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/modules/pubpunk.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/pubpunk-post-draft-request-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_post_draft_request_packet_v0_1.md"
    - "work/reports/2026-06-08-pubpunk-post-draft-request-packet-v0-1.md"
  exclude:
    - ".punk/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-module-host/**"
    - "publishing/**"
    - "external publishing behavior"
    - "adapter invocation"
    - "provider/model calls"
    - "module runtime behavior"
acceptance:
  - "Adds a side-effect-free PubPunk post draft request packet model for explicit content intent, target resource/channel refs, voice/style refs, constraints, and expected draft outputs."
  - "Supports multiple target resources as explicit refs without hard-coding provider, harness, or publication protocol behavior."
  - "Keeps the packet request-only and advisory: no draft text generation, no provider call, no filesystem read, no workspace scan, no publishing, no adapter invocation, no receipt writing, no gate/proof behavior, and no acceptance claim."
  - "Adds unit and smoke coverage for ready and blocked request packets."
  - "Adds an eval spec and updates PubPunk docs, crate status, report, and work status."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/PLOT-INTAKE.md"
  - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
  - "evals/specs/pubpunk-module-boundary.v0.1.md"
  - "evals/specs/pubpunk-workspace-instruction-packet.v0.1.md"
  - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
  - "evals/specs/pubpunk-publish-request-packet.v0.1.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This adds the first side-effect-free PubPunk draft-request packet selected by an explicit user scope override. It is satisfied by repo-tracked PubPunk, Plot Intake, module authoring, and host contract refs. External channel research is not needed unless scope changes toward channel-specific content rules, provider calls, publication protocols, adapters, external publishing, or factual public claims."
  research_refs:
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/PLOT-INTAKE.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
    - "evals/specs/pubpunk-module-boundary.v0.1.md"
    - "evals/specs/pubpunk-workspace-instruction-packet.v0.1.md"
    - "evals/specs/pubpunk-inventory-input-packet.v0.1.md"
    - "evals/specs/pubpunk-publish-request-packet.v0.1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/modules/pubpunk.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/pubpunk-post-draft-request-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a side-effect-free PubPunk post draft request packet model before any active drafting, provider, adapter, CLI, or publishing behavior."
---

# Add PubPunk post draft request packet v0.1

## Context

The maintainer explicitly selected PubPunk urgency after the Codebase Study
capability/privacy checkpoint landed on `main`.

PubPunk already has side-effect-free inventory, connector, publish request,
receipt preflight, receipt handoff, operation-evidence handoff, and
receipt/evidence event handoff models. The missing front-of-pipeline piece for
writing posts is a request packet for post drafting.

## Selected slice

Add one pure PubPunk model plus smoke coverage:

- capture explicit content intent;
- capture target resource/channel refs;
- capture voice/style refs;
- capture constraints and expected draft outputs;
- support multiple target resources as refs;
- produce request-only readiness/blockers;
- do not generate draft text or call providers.

## Boundary

This slice changes no PubPunk module execution, Module Host runtime, public
CLI, workspace initialization, filesystem scanning, file reads, draft body
generation, provider/model calls, adapter invocation, external publishing,
metrics collection, token collection, receipt writing, event writing, gate
writing, proofpack writing, or acceptance claims.

## Outcome

Done when the packet model, unit tests, smoke eval case, eval spec, docs,
report, and work status are updated, checks pass, and `selected_next` points
to the next bounded PubPunk drafting checkpoint.
