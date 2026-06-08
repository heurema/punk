---
id: goal_define_codebase_study_source_observation_request_packet_v0_1
title: "Define Codebase Study source observation request packet v0.1"
status: done
owner: "vitaly"
module: "product"
priority: P2
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: 2026-06-08
completed_at: 2026-06-08
blocked_by: []
scope:
  include:
    - "docs/modules/codebase-study.md"
    - "evals/specs/codebase-study-source-observation-request-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_codebase_study_source_observation_request_packet_v0_1.md"
    - "work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Defines the explicit source observation request packet boundary for future Codebase Study input."
  - "Requires caller-supplied scope/refs, capability requests, privacy/redaction policy refs or blockers, expected output authority, and downstream handoff intent."
  - "Keeps capability requests separate from grants."
  - "Rejects ambient repo discovery, implicit current-working-directory traversal, auto-discovery of all files, content reads, source hashing, size collection, manifest generation, runtime `.punk` writes, lab execution, benchmark authority, gate/proof authority, and acceptance."
  - "Records docs/eval evidence and selects the next bounded checkpoint."
knowledge_refs:
  - "docs/modules/codebase-study.md"
  - "evals/specs/codebase-study-module-boundary.v0.1.md"
  - "evals/specs/codebase-study-conformance-packet.v0.1.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "work/reports/2026-06-08-codebase-study-conformance-packet-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This defines the input packet boundary selected by the Codebase Study conformance packet. It uses repo-tracked module, host contract, conformance, Brownfield, and eval refs. External research is not needed unless scope changes toward implementation, active scanning, traversal, content reads, hashing, source inventory generation, runtime storage, module execution, lab execution, or external benchmark claims."
  research_refs:
    - "docs/modules/codebase-study.md"
    - "evals/specs/codebase-study-module-boundary.v0.1.md"
    - "evals/specs/codebase-study-conformance-packet.v0.1.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "work/reports/2026-06-08-codebase-study-conformance-packet-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/codebase-study.md"
    - "evals/specs/codebase-study-source-observation-request-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Defines future Codebase Study request input before implementation or active source observation behavior."
---

# Define Codebase Study source observation request packet v0.1

## Context

The Codebase Study conformance packet keeps skeleton implementation and runtime
activation blocked until source observation input, capability, privacy,
evidence, receipt, and evaluation-route findings are resolved.

The smallest next step is the input packet boundary. It can define what a
caller must provide before Codebase Study may even request source observation,
without granting scanner, traversal, content read, storage, or authority
behavior.

## Selected slice

Capture the request packet boundary only:

- request packet fields;
- valid and invalid request examples;
- capability-request vs capability-grant split;
- privacy/redaction blockers;
- downstream handoff intent;
- eval/spec cases and work report.

## Outcome

Done in
`work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md`.

The request packet boundary defines caller-supplied input shape, explicit
source refs, capability requests separate from grants, privacy/redaction refs
or blockers, expected advisory output authority, downstream handoff intent,
and explicit invalid request cases. It does not grant scanner, traversal,
content read, source hashing, size collection, manifest generation, runtime
storage, lab execution, benchmark authority, gate/proof authority, or
acceptance behavior.

The next bounded checkpoint is
`work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md`.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no Codebase Study runtime, no module invocation, no provider orchestration, no
adapter invocation, no workspace initializer, no repo scanning, no file
walking, no content reading, no source filesystem hashing, no size collection,
no manifest generation, no runtime `.punk` writes, no lab code import, no
benchmark execution, no gate writer, no proofpack writer, and no acceptance
claim.

## Outcome

Done when the request packet boundary is documented, eval cases cover positive
and negative input behavior, the work ledger records completion, checks pass,
and `selected_next` points to the next bounded checkpoint.
