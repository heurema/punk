---
id: goal_capture_harness_intake_routing_boundary_v0_1
title: "Capture harness intake routing boundary v0.1"
status: done
owner: "vitaly"
module: "project"
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
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
    - "work/goals/goal_capture_harness_intake_routing_boundary_v0_1.md"
    - "work/reports/2026-06-08-harness-intake-routing-boundary-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "scripts/**"
    - "docs/modules/**"
    - "docs/adapters/**"
acceptance:
  - "Captures the proposed `/punk ...` harness slash-command bridge as a thin runner-aid layer into Punk-owned Plot Intake routing."
  - "Records that route output must separate lifecycle phase from module route before execution."
  - "Keeps harness commands, provider settings, route hints, module routing, PubPunk, adapters, publishing, receipts, events, gate, proofpack, and acceptance behavior non-active."
  - "Updates Work Ledger evidence and runs required governance checks."
knowledge_refs:
  - "docs/product/PLOT-INTAKE.md"
  - "docs/product/RUNNER-AIDS.md"
  - "docs/product/MODULES.md"
  - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-harness-intake-routing-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: false
  rationale: "This slice captures user-provided design intent against existing repo docs and eval specs. Future implementation of runtime routing, provider adapters, or slash-command integration remains a separate R2+ decision."
  research_refs:
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/RUNNER-AIDS.md"
    - "docs/product/MODULES.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/plot-intake-routing-recommendation.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The slice records a new advisory Plot Intake routing boundary candidate for harness slash commands and phase/module route separation."
---

## Context

The maintainer wants Punk to support a convenient command shape inside multiple
executor harnesses, such as `/punk ...` in Codex CLI, Codex App, Claude Code,
Gemini, `agy`, or another shell.

The goal is not to make each harness decide routing. The goal is to let the
harness command collect a raw request and metadata, then pass it to Punk-owned
Plot Intake routing.

## Intent

Capture the boundary before implementation so the idea does not become hidden
chat context.

## Outcome

Done with docs/spec/work-ledger evidence.

## Non-Scope

Do not implement CLI behavior, harness slash-command files, provider adapters,
module routing runtime, Module Host runtime, PubPunk runtime, draft writing,
publishing, browser/API calls, credential access, receipt writing, event
writing, gate decisions, proofpacks, or acceptance claims.
