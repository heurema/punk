---
id: goal_define_codebase_study_capability_privacy_boundary_v0_1
title: "Define Codebase Study capability and privacy boundary v0.1"
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
    - "evals/specs/codebase-study-capability-privacy-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md"
    - "work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Defines the capability request, capability grant, privacy, and redaction boundary for future Codebase Study source observation."
  - "Keeps request packet capability requests separate from selected grants."
  - "Records which capabilities remain denied by default and which future grants need separate bounded goals."
  - "Rejects content reads, source snippets, raw environment values, secret-like values, source hashing, size collection, runtime `.punk` writes, lab execution, benchmark authority, gate/proof authority, and acceptance."
  - "Records docs/eval evidence and selects the next bounded checkpoint."
knowledge_refs:
  - "docs/modules/codebase-study.md"
  - "evals/specs/codebase-study-module-boundary.v0.1.md"
  - "evals/specs/codebase-study-conformance-packet.v0.1.md"
  - "evals/specs/codebase-study-source-observation-request-packet.v0.1.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This defines the capability/privacy boundary selected by the request packet slice. It uses repo-tracked module, host contract, Brownfield, and eval refs. External research is not needed unless scope changes toward implementation, active scanning, traversal, content reads, hashing, source inventory generation, runtime storage, module execution, lab execution, or external benchmark claims."
  research_refs:
    - "docs/modules/codebase-study.md"
    - "evals/specs/codebase-study-module-boundary.v0.1.md"
    - "evals/specs/codebase-study-conformance-packet.v0.1.md"
    - "evals/specs/codebase-study-source-observation-request-packet.v0.1.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/modules/codebase-study.md"
    - "evals/specs/codebase-study-capability-privacy-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Defines future Codebase Study capability/privacy boundaries before implementation or active source observation behavior."
---

# Define Codebase Study capability and privacy boundary v0.1

## Context

The source observation request packet defines request shape only. Capability
requests are not grants, and privacy/redaction blockers keep execution blocked.

The next smallest boundary is the capability and privacy split: what can be
requested, what remains denied, what a future grant would need, and which
privacy/redaction blockers remain hard stops.

## Selected slice

Capture the capability/privacy boundary only:

- request-vs-grant split;
- denied capabilities;
- future grant prerequisites;
- privacy/redaction blockers;
- invalid examples;
- eval/spec cases and work report.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no Codebase Study runtime, no module invocation, no provider orchestration, no
adapter invocation, no workspace initializer, no repo scanning, no file
walking, no content reading, no source filesystem hashing, no size collection,
no manifest generation, no runtime `.punk` writes, no lab code import, no
benchmark execution, no gate writer, no proofpack writer, and no acceptance
claim.

## Outcome

Done in
`work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md`.

The capability/privacy boundary defines the request-vs-grant split, keeps
`capability_grants` and selected grants empty, records denied-by-default
capabilities, future grant prerequisites, and privacy/redaction blockers. It
does not grant scanner, traversal, content read, source hashing, size
collection, manifest generation, runtime storage, lab execution, benchmark
authority, gate/proof authority, or acceptance behavior.

The next bounded checkpoint is
`work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md`.
