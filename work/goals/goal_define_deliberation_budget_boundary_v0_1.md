---
id: goal_define_deliberation_budget_boundary_v0_1
title: "Define Deliberation Budget boundary v0.1"
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
    - "docs/product/DELIBERATION-BUDGET.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/deliberation-budget-boundary.v0.1.md"
    - "work/STATUS.md"
    - "work/goals/goal_define_deliberation_budget_boundary_v0_1.md"
    - "work/reports/2026-05-19-deliberation-budget-boundary-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - ".github/**"
    - "publishing/**"
    - "docs/adr/**"
acceptance:
  - "Defines a docs/eval boundary for intentionally spending extra advisory passes on important decisions."
  - "Preserves Punk authority: deliberation, model output, provider output, and synthesis are advisory only."
  - "Connects the boundary to module authoring and future PubPunk work without implementing modules or publishing behavior."
  - "Does not activate provider orchestration, CLI behavior, runtime storage, Module Host runtime, adapters, gate/proof writers, receipts, or acceptance claims."
  - "Preserves selected_next in work/STATUS.md."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/RUNNER-AIDS.md"
  - "docs/product/REVIEW-ASSESSMENT.md"
  - "docs/product/PLOT-INTAKE.md"
  - "docs/product/MODULES.md"
  - "knowledge/research/2026-05-18-gstack-workflow-prior-art.md"
  - "knowledge/research/2026-05-19-plot-intake-boundary-prior-art.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-19-deliberation-budget-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This touches model-control artifacts, review assessment boundaries, module-authoring discipline, future provider diversity, and future PubPunk workflow shape. It is satisfied by existing gstack/Plot Intake research plus canonical Punk laws, architecture, runner-aid, review-assessment, module, and project-memory boundaries."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/RUNNER-AIDS.md"
    - "docs/product/REVIEW-ASSESSMENT.md"
    - "docs/product/PLOT-INTAKE.md"
    - "docs/product/MODULES.md"
    - "knowledge/research/2026-05-18-gstack-workflow-prior-art.md"
    - "knowledge/research/2026-05-19-plot-intake-boundary-prior-art.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/DELIBERATION-BUDGET.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Adds a docs/eval boundary for multi-perspective advisory deliberation before module implementation."
---

# Define Deliberation Budget boundary v0.1

## Context

The maintainer wants Punk to spend more reasoning budget on important decisions
by comparing multiple agents, models, providers, lenses, or reviewers before
settling on a synthesis.

This is needed before the first publishing module so future modules use the
same decision discipline instead of drifting into incompatible local patterns.

## Selected slice

Capture a docs/eval boundary only:

- Deliberation Budget product doc;
- boundary eval/spec;
- documentation-map row;
- work report and status note.

## Boundary

This slice adds no provider orchestration, no CLI behavior, no runtime storage,
no Module Host runtime, no module activation, no adapter invocation, no
publishing behavior, no receipt writer, no gate writer, no proofpack writer,
and no acceptance claim.

## Outcome

The boundary is done when docs governance passes with the new canonical owner
row, the eval/spec exists, and `work/STATUS.md` records the side-track without
changing `selected_next`.
