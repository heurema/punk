---
id: goal_add_pubpunk_publish_receipt_evidence_event_handoff_v0_1
title: "Add PubPunk Publish Receipt/Evidence Event Handoff v0.1"
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
    - "evals/specs/pubpunk-publish-receipt-evidence-event-handoff.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_publish_receipt_evidence_event_handoff_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-publish-receipt-evidence-event-handoff-v0-1.md"
  exclude:
    - ".punk/**"
    - ".github/**"
    - "crates/punk-cli/**"
    - "crates/punk-project/**"
    - "crates/punk-module-host/**"
    - "crates/punk-events/**"
    - "publishing/**"
    - "external publishing behavior"
    - "adapter invocation"
    - "module runtime behavior"
acceptance:
  - "Adds a side-effect-free PubPunk publish receipt/evidence event handoff packet model with explicit publish operation evidence handoff, receipt writer result, operation evidence write result, receipt, operation evidence, event source, event correlation, adapter invocation receipt, connector, channel, and payload refs."
  - "Requires the narrow request_receipt_evidence_event_handoff grant, metadata-only privacy, expected receipt fields, `.punk/runs` receipt/evidence refs, fixed `.punk/events/flow.jsonl` event log ref, separated receipt/evidence refs, and allowed source refs before projecting local event handoff refs."
  - "Adds unit and smoke coverage that chains the ready packet to the existing `punk-events` receipt/evidence event helper for one bounded event append only to an explicit temporary `.punk/events/flow.jsonl` log."
  - "Keeps PubPunk advisory and side-effect-free: no PubPunk runtime, CLI, module invocation, filesystem reads, publishing, metrics collection, PubPunk-owned receipt writing, PubPunk-owned operation-evidence writing, PubPunk-owned event-log writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds an eval spec and updates PubPunk docs, crate status, documentation map, report, and work status."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-publish-operation-evidence-handoff.v0.1.md"
  - "evals/specs/pubpunk-publish-receipt-evidence-event-handoff.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-publish-receipt-evidence-event-handoff-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-publish-receipt-evidence-event-handoff-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating PubPunk-to-events receipt/evidence event handoff boundary. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, the publish operation evidence handoff spec, and the existing punk-events local receipt/evidence handoff helper."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-publish-operation-evidence-handoff.v0.1.md"
    - "work/goals/goal_add_local_receipt_evidence_event_handoff_v0_1.md"
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
  rationale: "Adds PubPunk publish receipt/evidence event handoff evidence without PubPunk runtime activation, adapter invocation, or external publishing."
---

# Add PubPunk Publish Receipt/Evidence Event Handoff v0.1

## Context

PubPunk can prepare a publish operation evidence handoff packet and chain it
into the existing Module Host operation-evidence writer after a successful
receipt write. The next missing link was a PubPunk-owned handoff packet that
prepares explicit refs for the existing local receipt/evidence event writer
without making PubPunk an event-log writer.

## Selected slice

Add one pure PubPunk model plus one smoke eval case:

- require explicit publish operation evidence handoff, receipt writer result,
  operation evidence write result, receipt, operation evidence, event source,
  event correlation, adapter invocation receipt, connector, channel, and
  payload refs;
- require the narrow `request_receipt_evidence_event_handoff` grant;
- require receipt and operation evidence refs to stay under `.punk/runs`;
- require receipt and operation evidence refs to remain separated;
- require event log ref to be `.punk/events/flow.jsonl`;
- project event handoff refs only when the packet is ready;
- chain the projected refs into the existing `punk-events` receipt/evidence
  handoff helper against an explicit temporary `.punk/events/flow.jsonl` log.

## Boundary

This slice changes no PubPunk runtime, Module Host runtime, public CLI,
workspace initialization, filesystem scanning, draft body reads, provider
orchestration, adapter invocation, external publishing, metrics collection,
token collection, PubPunk-owned receipt writing, PubPunk-owned
operation-evidence writing, PubPunk-owned event-log writing, gate writing,
proofpack writing, or acceptance claims.

## Outcome

Done when the packet model, unit tests, smoke eval case, eval spec, docs,
report, and work status are updated, checks pass, and `selected_next` remains
unchanged.
