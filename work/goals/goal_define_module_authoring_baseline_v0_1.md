---
id: goal_define_module_authoring_baseline_v0_1
title: "Define Module Authoring Baseline v0.1"
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
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULES.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/module-authoring-baseline.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_module_authoring_baseline_v0_1.md"
    - "work/reports/2026-05-19-module-authoring-baseline-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/**"
    - "docs/adr/**"
acceptance:
  - "Defines a docs/eval baseline for module authoring before PubPunk or DevPunk implementation."
  - "Includes rules and deterministic enforcement direction so future module work cannot rely on agents remembering prose."
  - "Records three-provider advisory deliberation and synthesis."
  - "Preserves Module Host runtime, module activation, provider orchestration, adapter invocation, publishing, gate/proof writers, receipts, and acceptance claims as inactive."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/MODULES.md"
  - "docs/product/MODULE-HOST.md"
  - "docs/product/DELIBERATION-BUDGET.md"
  - "docs/product/INSTRUCTION-SOURCES.md"
  - "docs/product/RUNNER-AIDS.md"
  - "docs/product/REVIEW-ASSESSMENT.md"
  - "docs/product/PLOT-INTAKE.md"
  - "docs/modules/pubpunk.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-module-authoring-baseline-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This touches module boundaries, future module interfaces, future development-module strategy, instruction sources, workspace policy, eval policy, and PubPunk sequencing. It is satisfied by canonical module/host docs, Deliberation Budget, existing gstack/Plot Intake research, and three provider advisory passes recorded in the report."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-HOST.md"
    - "docs/product/DELIBERATION-BUDGET.md"
    - "docs/product/INSTRUCTION-SOURCES.md"
    - "docs/product/RUNNER-AIDS.md"
    - "docs/product/REVIEW-ASSESSMENT.md"
    - "docs/product/PLOT-INTAKE.md"
    - "docs/modules/pubpunk.md"
    - "knowledge/research/2026-05-18-gstack-workflow-prior-art.md"
    - "knowledge/research/2026-05-19-plot-intake-boundary-prior-art.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULES.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a module-authoring baseline and eval/spec before module implementation."
---

# Define Module Authoring Baseline v0.1

## Context

PubPunk is urgent because publishing work is already operationally painful.
However, implementing PubPunk before shared module-authoring rules risks making
it a one-off and causing later modules to drift.

The maintainer also raised the development-module question: Punk should develop
Punk and future modules through Punk, not through hidden process.

## Selected slice

Capture the docs/eval baseline only:

- module-authoring product doc;
- module-authoring eval/spec;
- documentation-map owner row;
- small Modules cross-link;
- work report and status note.

## Boundary

This slice adds no Rust code, no public CLI behavior, no Module Host runtime,
no module activation, no provider orchestration, no adapter invocation, no
publishing behavior, no receipt writer, no gate writer, no proofpack writer,
and no acceptance claim.

## Outcome

Done when the new baseline and eval/spec exist, the documentation map points to
the canonical owner, the work ledger records completion, checks pass, and
`selected_next` remains unchanged.
