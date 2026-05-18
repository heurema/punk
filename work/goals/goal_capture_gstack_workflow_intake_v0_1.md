---
id: goal_capture_gstack_workflow_intake_v0_1
title: "Capture gstack workflow intake v0.1"
status: done
owner: "vitaly"
module: "product"
priority: P2
authority: canonical
created_at: 2026-05-18
updated_at: 2026-05-18
selected_at: 2026-05-18
started_at: 2026-05-18
completed_at: 2026-05-18
blocked_by: []
scope:
  include:
    - "knowledge/research/2026-05-18-gstack-workflow-prior-art.md"
    - "knowledge/ideas/2026-05-18-gstack-mechanism-intake.md"
    - "docs/product/RUNNER-AIDS.md"
    - "docs/product/REVIEW-ASSESSMENT.md"
    - "docs/product/INSTRUCTION-SOURCES.md"
    - "evals/specs/runner-aid-boundary.v0.1.md"
    - "evals/specs/review-assessment-receipt.v0.1.md"
    - "evals/specs/contract-intake-questions.v0.1.md"
    - "evals/specs/docs-drift-assessment.v0.1.md"
    - "evals/specs/instruction-source-freshness.v0.1.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "work/STATUS.md"
    - "work/goals/goal_capture_gstack_workflow_intake_v0_1.md"
    - "work/reports/2026-05-18-gstack-workflow-intake-v0-1.md"
  exclude:
    - "crates/**"
    - ".github/**"
    - ".punk/**"
    - "publishing/**"
    - "docs/adr/**"
    - "scripts/**"
acceptance:
  - "A repo-tracked R2 advisory research note classifies gstack mechanisms as adopt/defer/park/avoid."
  - "A companion idea artifact captures the extracted mechanisms and trigger conditions."
  - "Runner aid, review assessment, and instruction source boundaries are documented without activating runtime behavior."
  - "Eval/spec boundary artifacts cover runner aids, review assessment receipts, contract-intake questions, docs drift, and instruction freshness."
  - "Documentation map rows are updated for the new canonical boundary docs."
  - "Work status records the side-track completion while preserving the existing selected_next."
  - "No Rust code, public CLI command, Module Host runtime, DevPunk, browser adapter, provider adapter, external side effect, gate writer, proofpack writer, or acceptance claim is added."
knowledge_refs:
  - "knowledge/research/2026-05-18-gstack-workflow-prior-art.md"
  - "knowledge/ideas/2026-05-18-gstack-mechanism-intake.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/RESEARCH-GATE.md"
  - "docs/product/RESEARCH-INTAKE.md"
  - "docs/product/PROJECT-MEMORY.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-18-gstack-workflow-intake-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This work touches persistent model-control artifacts, runner aids, review assessment boundaries, instruction sources, project memory, eval policy, and future module/adapter boundaries. It is satisfied by the gstack workflow prior-art research note plus existing Punk laws, architecture, research gate, research intake, and project-memory boundaries."
  research_refs:
    - "knowledge/research/2026-05-18-gstack-workflow-prior-art.md"
    - "knowledge/ideas/2026-05-18-gstack-mechanism-intake.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/RESEARCH-INTAKE.md"
    - "docs/product/PROJECT-MEMORY.md"
  external_research_refs:
    - "https://github.com/garrytan/gstack"
  blocked_reason: null
doc_impact:
  classification: docs-eval-spec
  required_updates:
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/RUNNER-AIDS.md"
    - "docs/product/REVIEW-ASSESSMENT.md"
    - "docs/product/INSTRUCTION-SOURCES.md"
    - "evals/specs/**"
    - "knowledge/research/**"
    - "knowledge/ideas/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The patch adds new boundary docs and eval/spec artifacts from external prior-art intake, so the documentation map, work ledger, and report evidence must be updated."
---

## Context

The maintainer requested a detailed study of `garrytan/gstack` to strengthen
Punk.

gstack is useful prior art for staged AI-assisted work, review lanes, browser
QA, safety guardrails, generated skill docs, and agent memory. The risk is
scope drift: copying gstack directly would turn Punk into a provider-specific
agent workflow rather than a local-first bounded work kernel.

## Selected slice

Capture only the docs/eval/spec intake needed for Punk to borrow the useful
mechanisms safely:

- research note;
- idea backlog entry;
- runner-aid boundary doc;
- review-assessment boundary doc;
- instruction-source boundary doc;
- five eval/spec boundary artifacts;
- work report and status note.

## Boundary

This slice is documentation and eval/spec only.

It adds no Rust code, no public CLI behavior, no provider-specific workflow, no
Claude/Codex skill installation, no browser daemon, no cookie import, no
network behavior, no deploy/canary behavior, no Module Host runtime, no DevPunk,
no adapter invocation, no gate/proof writer, and no acceptance claim.

## Outcome

Done when the new artifacts are committed, documentation map rows are added,
`work/STATUS.md` records the side-track completion, and checks pass or any
pre-existing check failure is recorded explicitly.

`selected_next` must remain unchanged.
