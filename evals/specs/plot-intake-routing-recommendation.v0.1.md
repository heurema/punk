---
id: eval_spec_plot_intake_routing_recommendation_v0_1
kind: eval-spec
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
review_after: 2026-06-19
related_docs:
  - docs/product/PLOT-INTAKE.md
  - docs/product/MODULES.md
  - docs/product/MODULE-HOST.md
---

# Eval spec: Plot Intake routing recommendation v0.1

## Purpose

Ensure module/domain routing remains advisory, explainable, and non-authoritative.

## Status

Design/advisory only. This spec does not implement module routing.

## Cases

### Case 1 — recommendation has rationale

Every routing recommendation must include:

- recommended route;
- rationale;
- alternatives considered;
- confidence;
- blockers or prerequisites;
- statement that the recommendation does not activate a module or decide acceptance.

### Case 2 — no module authority

A routing recommendation must not claim that a module decided truth, accepted scope, or approved execution.

Forbidden claims:

- `module_decision: accepted`
- `route_is_final: true`
- `gate_decision: accepted`
- `module_authority: final`

### Case 3 — future route labels are marked future

If a recommendation mentions route labels that are not active current repo surfaces, the labels must be marked `future`, `candidate`, `parked`, or `incubating` as appropriate.

### Case 4 — no adapter invocation

Routing must not invoke adapters, publish, call APIs, read credentials, create PRs/issues, or perform external side effects.
