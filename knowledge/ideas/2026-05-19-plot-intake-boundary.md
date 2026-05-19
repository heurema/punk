---
id: idea_2026_05_19_plot_intake_boundary
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-19
review_after: 2026-06-19
related_research:
  - knowledge/research/2026-05-19-plot-intake-boundary-prior-art.md
related_docs:
  - docs/product/PLOT-INTAKE.md
  - docs/product/CONTRACT-SCHEMA.md
  - docs/product/ARCHITECTURE.md
related_evals:
  - evals/specs/plot-intake-boundary.v0.1.md
  - evals/specs/plot-intake-routing-recommendation.v0.1.md
  - evals/specs/plot-intake-evidence-plan.v0.1.md
---

# Plot Intake boundary idea

## Intent

Capture the Work Intake direction as a Punk-shaped idea without activating runtime behavior.

The idea is to define **Plot Intake** as the upstream part of `plot` that turns a raw request into contract-draft readiness.

## Extracted primitive

```text
raw request
  -> intent candidate
  -> assumptions / unknowns / contradictions
  -> smallest useful slice
  -> scope / non-scope / forbidden effects
  -> evidence and receipt plan
  -> advisory routing recommendation
  -> contract draft readiness
```

## Adopt

- Canonical name: `Plot Intake`.
- Human alias: `Work Intake`.
- Output shape: advisory intake assessment.
- Lenses: product shaper, domain expert, architect, risk reviewer, evidence planner, module router, project-memory reviewer.
- Routing is recommendation only and must include rationale, alternatives, and confidence.
- Intake must be inspectable and link to project memory when promoted.

## Defer

- Runtime writer.
- Public CLI.
- Generated executor briefs.
- Full task-generation pipeline.
- Module routing implementation.
- EARS/Gherkin as default output formats.
- Contract Context Pack integration beyond docs/spec mapping.

## Park

- standalone intake module packaging.
- branded intake-module naming.
- Autoplan orchestration.
- Plugin/runtime router.
- Browser/deploy/side-effect adapter involvement.

## Avoid

- Intake writes final decisions.
- Intake creates proofpacks or acceptance claims.
- Intake writes code or publishes.
- Intake claims module routing as truth.
- Hidden memory or hidden policy state.
- Slash-command workflow as Punk architecture.

## Trigger condition for implementation

Implementation should wait until:

- Plot/contract loop promotion is explicitly selected;
- boundary docs are accepted;
- eval/spec fixtures exist;
- no active CLI/runtime overclaim exists;
- module routing remains advisory by contract.
