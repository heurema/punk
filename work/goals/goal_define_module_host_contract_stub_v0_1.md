---
id: goal_define_module_host_contract_stub_v0_1
title: "Define Module Host Contract Stub v0.1"
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
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/DELIBERATION-BUDGET.md"
    - "evals/specs/module-host-contract-stub.v0.1.md"
    - "evals/specs/deliberation-budget-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_module_host_contract_stub_v0_1.md"
    - "work/reports/2026-05-19-module-host-contract-stub-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/**"
    - "docs/adr/**"
acceptance:
  - "Defines a docs/eval Module Host Contract Stub before PubPunk or DevPunk skeleton work."
  - "Defines host-scoped input bundle, deny-by-default capability envelope, advisory outputs, side-effect request proposal, and event-log boundary."
  - "Documents Deliberation Budget token cost accounting fields for accepted, rejected, discarded, and unknown token spend."
  - "Preserves Module Host runtime, module invocation, provider orchestration, token accounting implementation, adapter invocation, workspace initialization, publishing, gate/proof writers, receipts, and acceptance claims as inactive."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/MODULES.md"
  - "docs/product/MODULE-HOST.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-CONFORMANCE.md"
  - "docs/product/DELIBERATION-BUDGET.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-module-host-contract-stub-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This defines the future Module Host contract shape, capability envelope, side-effect request boundary, and deliberation token-cost accounting direction before module skeleton work. It is satisfied by canonical module/module-host docs, the Module Authoring Baseline, the Module Conformance Packet, Deliberation Budget, and the three-provider advisory review summarized in the report."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/DELIBERATION-BUDGET.md"
    - "work/reports/2026-05-19-module-authoring-baseline-v0-1.md"
    - "work/reports/2026-05-19-module-conformance-packet-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/MODULE-CONFORMANCE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/DELIBERATION-BUDGET.md"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a host contract stub and deliberation cost-accounting boundary before module implementation."
---

# Define Module Host Contract Stub v0.1

## Context

Independent advisory review found that PubPunk can be the first business
module, but a host contract stub should exist first so PubPunk does not invent
its own input, capability, side-effect, or output boundary.

The maintainer also requested token spend accounting for accepted, rejected,
discarded, and unknown decision work during multi-pass deliberation.

## Selected slice

Capture the docs/eval boundary only:

- Module Host Contract Stub product doc;
- Module Host Contract Stub eval/spec;
- Deliberation Budget token-cost accounting fields;
- documentation-map owner row;
- Module Conformance cross-link;
- work report and status note.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no module invocation, no provider orchestration, no token accounting
implementation, no adapter invocation, no workspace initializer, no publishing
behavior, no receipt writer, no gate writer, no proofpack writer, and no
acceptance claim.

## Outcome

Done when the contract stub and eval/spec exist, token cost accounting is
documented in Deliberation Budget, the documentation map points to the
canonical owner, the work ledger records completion, checks pass, and
`selected_next` remains unchanged.
