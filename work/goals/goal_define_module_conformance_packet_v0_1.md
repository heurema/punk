---
id: goal_define_module_conformance_packet_v0_1
title: "Define Module Conformance Packet v0.1"
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
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-conformance-packet.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_module_conformance_packet_v0_1.md"
    - "work/reports/2026-05-19-module-conformance-packet-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/**"
    - "docs/adr/**"
acceptance:
  - "Defines a docs/eval Module Conformance Packet boundary as the reviewable bridge from Module Authoring Baseline to module-specific implementation."
  - "Keeps the packet advisory and non-authoritative."
  - "Splits implementation readiness from runtime activation readiness."
  - "Keeps DevPunk and PubPunk subject to the same packet rules."
  - "Preserves Module Host runtime, module activation, provider orchestration, adapter invocation, workspace initialization, publishing, gate/proof writers, receipts, and acceptance claims as inactive."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/MODULES.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-HOST.md"
  - "docs/product/DELIBERATION-BUDGET.md"
  - "docs/product/INSTRUCTION-SOURCES.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-module-conformance-packet-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This defines the module conformance boundary, future module review shape, readiness semantics, and DevPunk/PubPunk application notes. It is satisfied by the Module Authoring Baseline, existing module docs, and the prior three-provider deliberation recorded in the Module Authoring Baseline report."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/DELIBERATION-BUDGET.md"
    - "docs/product/INSTRUCTION-SOURCES.md"
    - "work/reports/2026-05-19-module-authoring-baseline-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a conformance packet boundary and eval/spec before module implementation."
---

# Define Module Conformance Packet v0.1

## Context

Module Authoring Baseline v0.1 defines shared rules. The next gap is the
reviewable packet that applies those rules to a concrete module candidate
before implementation starts.

## Selected slice

Capture the docs/eval packet boundary only:

- module-conformance product doc;
- module-conformance eval/spec;
- documentation-map owner row;
- Module Authoring Baseline cross-link;
- work report and status note.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no module activation, no provider orchestration, no adapter invocation, no
workspace initializer, no publishing behavior, no receipt writer, no gate
writer, no proofpack writer, and no acceptance claim.

## Outcome

Done when the packet boundary and eval/spec exist, the documentation map points
to the canonical owner, the work ledger records completion, checks pass, and
`selected_next` remains unchanged.
