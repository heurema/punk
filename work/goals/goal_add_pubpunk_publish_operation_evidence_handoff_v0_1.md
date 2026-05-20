---
id: goal_add_pubpunk_publish_operation_evidence_handoff_v0_1
title: "Add PubPunk Publish Operation Evidence Handoff v0.1"
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
    - "evals/specs/pubpunk-publish-operation-evidence-handoff.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pubpunk_publish_operation_evidence_handoff_v0_1.md"
    - "work/reports/2026-05-20-pubpunk-publish-operation-evidence-handoff-v0-1.md"
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
  - "Adds a side-effect-free PubPunk publish operation evidence handoff packet model with explicit receipt writer result, operation-evidence target, operation-evidence bytes, adapter invocation receipt, connector, channel, and payload refs."
  - "Requires the narrow request_operation_evidence_write grant, metadata-only privacy, expected receipt fields, `.punk/runs` operation-evidence target path, target/path match, and allowed source refs before projecting host operation-evidence handoff refs."
  - "Adds unit and smoke coverage that chains the ready packet to the existing Module Host operation-evidence writer for exact-byte writes only to an explicit temporary `.punk/runs` target after a successful receipt write."
  - "Keeps PubPunk advisory and side-effect-free: no PubPunk runtime, CLI, module invocation, filesystem reads, publishing, metrics collection, PubPunk-owned receipt writing, PubPunk-owned operation-evidence writing, adapter invocation, gate/proof writing, or acceptance claims."
  - "Adds an eval spec and updates PubPunk docs, crate status, documentation map, report, and work status."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/modules/pubpunk.md"
  - "docs/modules/pubpunk-workspace-instructions.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "evals/specs/pubpunk-publish-receipt-write-handoff.v0.1.md"
  - "evals/specs/pubpunk-publish-operation-evidence-handoff.v0.1.md"
  - "work/reports/2026-05-20-pubpunk-publish-operation-evidence-handoff-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-20-pubpunk-publish-operation-evidence-handoff-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This changes an incubating PubPunk-to-Module-Host operation-evidence handoff boundary. It is satisfied by existing module authoring/conformance/host docs, PubPunk module docs, the publish receipt write handoff spec, and the existing Module Host operation-evidence writer model."
  research_refs:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/modules/pubpunk.md"
    - "docs/modules/pubpunk-workspace-instructions.md"
    - "evals/specs/pubpunk-publish-receipt-write-handoff.v0.1.md"
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
  rationale: "Adds PubPunk publish operation-evidence handoff evidence without PubPunk runtime activation or external publishing."
---

# Add PubPunk Publish Operation Evidence Handoff v0.1

## Context

PubPunk can prepare a publish receipt write handoff packet and chain it into the
existing Module Host first active local receipt writer. The next missing link
was a PubPunk-owned handoff packet that prepares explicit refs for the existing
host operation-evidence writer without making PubPunk an evidence writer.

## Selected slice

Add one pure PubPunk model plus one smoke eval case:

- require explicit publish receipt write handoff, receipt writer result,
  operation-evidence target path, operation-evidence bytes, adapter invocation
  receipt, connector, channel, and payload refs;
- require the narrow `request_operation_evidence_write` grant;
- require operation-evidence target path refs to stay under `.punk/runs`;
- require operation-evidence target path refs to match the writer result's
  operation-evidence ref;
- project operation-evidence handoff refs only when the packet is ready;
- chain the projected refs into the existing Module Host operation-evidence
  writer against an explicit temporary `.punk/runs` target after a successful
  receipt write.

## Boundary

This slice changes no PubPunk runtime, Module Host runtime, public CLI,
workspace initialization, filesystem scanning, draft body reads, provider
orchestration, adapter invocation, external publishing, metrics collection,
token collection, PubPunk-owned receipt writing, PubPunk-owned
operation-evidence writing, event-log mutation, gate writing, proofpack writing,
or acceptance claims.

## Outcome

Done when the packet model, unit tests, smoke eval case, eval spec, docs,
report, and work status are updated, checks pass, and `selected_next` remains
unchanged.
