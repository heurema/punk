---
id: goal_apply_codebase_study_conformance_packet_v0_1
title: "Apply Codebase Study Conformance Packet v0.1"
status: ready
owner: "vitaly"
module: "product"
priority: P2
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "evals/specs/codebase-study-conformance-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_apply_codebase_study_conformance_packet_v0_1.md"
    - "work/reports/2026-06-08-codebase-study-conformance-packet-v0-1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
    - "docs/product/**"
    - "docs/modules/**"
acceptance:
  - "Applies Module Conformance Packet v0.1 and Module Host Contract Stub v0.1 to Codebase Study as an advisory readiness pass."
  - "Identifies whether Codebase Study is ready for docs-only next work, skeleton implementation, and runtime activation."
  - "Records workspace, instruction, capability, privacy, side-effect, evidence, receipt, and `agent-bench-lab` evaluation-route findings."
  - "Keeps Codebase Study module execution, repo scanning, file walking, content reading, source filesystem hashing, size collection, manifest generation, runtime `.punk` writes, CLI behavior, gate/proof authority, and acceptance inactive."
  - "Records the next smallest bounded step."
knowledge_refs:
  - "docs/modules/codebase-study.md"
  - "evals/specs/codebase-study-module-boundary.v0.1.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/DELIBERATION-BUDGET.md"
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/module-authoring-baseline.v0.1.md"
  - "evals/specs/module-conformance-packet.v0.1.md"
  - "evals/specs/module-host-contract-stub.v0.1.md"
  - "evals/specs/brownfield-inventory-boundary.v0.1.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md"
  - "work/reports/2026-06-08-codebase-study-module-boundary-verification-v0-1.md"
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
  rationale: "This applies existing repo-tracked module conformance and host contract boundaries to the already-defined parked Codebase Study boundary. External research is not needed unless scope changes toward implementation, active scanning, traversal, content reads, hashing, source inventory generation, runtime storage, module execution, lab execution, or external benchmark claims."
  research_refs:
    - "docs/modules/codebase-study.md"
    - "evals/specs/codebase-study-module-boundary.v0.1.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/DELIBERATION-BUDGET.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/module-authoring-baseline.v0.1.md"
    - "evals/specs/module-conformance-packet.v0.1.md"
    - "evals/specs/module-host-contract-stub.v0.1.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md"
    - "work/reports/2026-06-08-codebase-study-module-boundary-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "evals/specs/codebase-study-conformance-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds advisory conformance packet evidence before any Codebase Study implementation or active source observation behavior."
---

# Apply Codebase Study Conformance Packet v0.1

## Context

Codebase Study is a parked future Punk module boundary for bounded source
observation.

The boundary has been verified as docs/eval only. It requires explicit source
observation request input and advisory observation packet output, and it denies
scanner, file walker, content reader, manifest builder, runtime storage,
module execution, gate/proof, and acceptance authority.

## Selected slice

Capture Codebase Study conformance evidence only:

- Codebase Study conformance eval/spec fixture;
- work report with packet result and findings;
- work status note.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no Codebase Study runtime, no module invocation, no provider orchestration, no
adapter invocation, no workspace initializer, no repo scanning, no file
walking, no content reading, no source filesystem hashing, no size collection,
no manifest generation, no runtime `.punk` writes, no lab code import, no
benchmark execution, no gate writer, no proofpack writer, and no acceptance
claim.

## Outcome

Done when Codebase Study has an advisory conformance result, blockers and next
work are recorded, the work ledger records completion, checks pass, and
`selected_next` points to the next bounded checkpoint.
