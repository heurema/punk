---
id: research_plot_intake_boundary_2026_05_19
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
review_after: 2026-06-19
classification: r2-design-research
canonical_for: []
related_docs:
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/CONTRACT-SCHEMA.md
  - docs/product/MODULES.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/REVIEW-ASSESSMENT.md
  - docs/product/INSTRUCTION-SOURCES.md
related_evals:
  - evals/specs/plot-intake-boundary.v0.1.md
  - evals/specs/plot-intake-routing-recommendation.v0.1.md
  - evals/specs/plot-intake-evidence-plan.v0.1.md
supersedes: []
superseded_by: null
---

# Plot Intake boundary prior art

## Question

How should Punk shape raw user work before domain execution, without turning intake into a module runtime, executor, decision surface, or second source of truth?

## Decision context

Punk has one lifecycle grammar:

```text
plot -> cut -> gate
```

`plot` shapes work and creates a contract. `cut` executes bounded approved work. `gate` verifies evidence and writes the final decision.

The proposed Work Intake layer is therefore better framed as **Plot Intake**: an upstream boundary inside `plot` that turns a raw request into contract-draft readiness.

This research does not activate runtime behavior, public CLI commands, modules, adapters, code execution, publishing, gate decisions, proofpacks, or acceptance claims.

## Source quality table

| Source | Tier | Use |
|---|---:|---|
| Punk README and product docs | A | Source of truth for lifecycle, active scope, current CLI boundary, laws, contract schema, module boundaries, project memory. |
| Existing gstack intake research and boundary docs | A/B | Prior art for forcing questions, review lanes, runner aids, instruction freshness, and advisory assessment boundaries. |
| Shape Up official material | A | Confirms shaping/building separation and rough/solved/bounded work. |
| Kiro Specs official docs | A | Confirms requirements/design/tasks split and clarification before executable tasks. |
| GitHub Spec Kit official docs/repo | A | Confirms clarify/checklist/analyze before implementation, but as agent-facing prior art only. |
| EARS, BDD/Gherkin, Example Mapping | A/B | Useful for ambiguity reduction, examples, acceptance shaping, and downstream clause quality. |
| JTBD, Opportunity Solution Tree, story mapping | A/B | Useful for problem/business-intent shaping and smallest useful slice. |
| DDD, EventStorming, Wardley Mapping | A/B | Useful for domain understanding and route selection, not mandatory workflow. |
| STRIDE / OWASP / threat modeling | A | Useful risk-review lane before side effects or security-sensitive work. |
| Ops runbooks / SRE incident materials | A/B | Useful for operational route framing, receipts, evidence, rollback, and runbook discipline. |
| Goalrail/CoreL pain notes | B/C unless repo-tracked | Useful motivating use case only; do not hardcode into Punk truth. |

## Prior-art comparison matrix

| Source | Mechanism | What Punk can take | What Punk must not take | Failure modes | Punk mapping | Intake outcome |
|---|---|---|---|---|---|---|
| Punk canon | `plot` shapes work; contract separates intent from execution; `gate` decides | Put intake inside `plot` | New lifecycle phase or decision surface | Hidden authority, fake acceptance | Plot Intake | adopt |
| gstack `/office-hours` | Forcing questions and reframing | Raw request clarification, assumptions, unknowns, alternatives | Slash-command authority | Workshop theater, provider lock-in | ClarificationQuestions | adopt |
| gstack product/engineering reviews | Product and architecture lenses | Advisory review lanes | Persona workflow as authority | Scope inflation, overdesign | Intake lenses | adopt |
| gstack `/autoplan` | Serialized review pipeline | Advisory assessment pipeline idea | Auto-decisions or hidden routing | Premature automation | Future assessment pipeline | defer |
| Kiro Specs | Requirements/design/tasks split | Work before executable task generation | IDE runtime behavior | Premature tasks | Intent/scope/evidence before contract | adopt |
| Shape Up | Rough, solved, bounded shaping | Smallest useful slice and shaping/building split | Fixed cycle/process mechanics | Ceremony cargo-culting | SmallestUsefulSlice | adopt |
| GitHub Spec Kit | Clarify/checklist/analyze before implementation | Readiness checks and what/why discipline | Agent workflow as project truth | Over-coupling to agent runtime | Readiness checks | defer |
| EARS / BDD / Example Mapping | Structured requirements and examples | Ambiguity reduction, examples, questions | Executable scenarios too early | False precision | Downstream clauses/evidence | defer/adopt selectively |
| JTBD / OST / Story Mapping | Problem and opportunity framing | Business intent and wedge discovery | Full discovery program | Discovery sprawl | Product shaper lens | adopt selectively |
| DDD / EventStorming / Wardley | Domain and strategic mapping | Domain vocabulary and route hints | Mandatory workshops | Strategy cosplay | Domain expert/router lens | adopt selectively |
| STRIDE / OWASP | Threat/risk review | Risk reviewer and forbidden effects | Mandatory heavyweight security process for trivial work | Checklist theater | Risk reviewer lens | adopt selectively |
| Ops runbooks/SRE | Procedures, evidence, roles | Operational evidence and rollback thinking | ITSM product surface | Ops bias | Ops route hints | adopt selectively |

## Boundary recommendation

Canonical name: **Plot Intake**.

Human-facing alias: **Work Intake**.

Output-facing sub-boundary: **Contract Intake**.

Avoid now: branded intake modules, standalone intake module packaging, or treating
DevPunk as the upstream boundary.

## Minimal v0.1 output shape

A future `PlotIntakeAssessment` or equivalent advisory artifact should preserve:

- `raw_request_ref`
- `intent_candidate`
- `problem_or_business_intent`
- `assumptions`
- `unknowns`
- `contradictions`
- `clarification_questions`
- `smallest_useful_slice`
- `scope_include`
- `scope_exclude`
- `forbidden_effects`
- `evidence_plan`
- `receipt_requirements_candidate`
- `eval_plan_candidate`
- `routing_recommendation`
- `routing_alternatives`
- `routing_confidence`
- `contract_draft_readiness`
- `project_memory_refs`
- `research_refs`
- `doc_impact_candidate`
- `knowledge_impact_candidate`
- `open_risks`

This is an output-shape recommendation only. It is not a runtime schema, writer, CLI, or active storage model.

## Forbidden behaviors

Plot Intake must not:

- write code;
- publish;
- invoke adapters;
- create runtime receipts;
- write gate decisions;
- write proofpacks;
- create acceptance claims;
- approve contracts by itself;
- choose modules as truth;
- hide memory or policy state;
- turn generated prompts or runner aids into project truth;
- create a fourth lifecycle phase.

## Adoption map

| Outcome | Items |
|---|---|
| adopt | Plot Intake boundary semantics; raw request to intent distinction; forcing questions; assumptions/unknowns; smallest useful slice; scope include/exclude; forbidden effects; evidence and receipt planning; advisory routing rationale. |
| defer | runtime writer; CLI command; generated executor briefs; full task generation; EARS/Gherkin-first defaults; module routing runtime. |
| park | standalone intake module packaging; autoplan orchestration; plugin/runtime router; browser/deploy side-effect workflows. |
| avoid | intake final authority; hidden memory; AI acceptance; module truth ownership; adapter truth ownership; proof/gate/acceptance from intake. |

## Impact on roadmap

This research does not change the selected implementation target.

If promoted, Plot Intake strengthens Phase 3 contract-loop readiness and Phase 4 project-memory readiness. It should be documented before implementation, with eval/spec boundary coverage before runtime or CLI behavior.

## Required eval/spec direction

- `evals/specs/plot-intake-boundary.v0.1.md`
- `evals/specs/plot-intake-routing-recommendation.v0.1.md`
- `evals/specs/plot-intake-evidence-plan.v0.1.md`

## Required docs / ADRs / contracts if promoted

- `docs/product/PLOT-INTAKE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- small cross-links from `ARCHITECTURE`, `CONTRACT-SCHEMA`, `PROJECT-MEMORY`, and `MODULES` only if kept surgical
- an ADR only if the boundary changes roadmap phase scope, module interface, runtime storage, or CLI behavior
- a goal/report pair for Dogfooding Level 0 documentation work

## Open questions

1. Should the output artifact be called `PlotIntakeAssessment`, `IntakeAssessment`, or remain unnamed until code?
2. Should routing be part of the assessment or a separate `ModuleRoutingAssessment`?
3. Which route labels are canonical now, and which remain future examples?
4. Should `PLOT-INTAKE.md` become a canonical product doc now, or should the research note remain the only repo-tracked artifact until a later ADR?
5. How should Plot Intake relate to Contract Context Pack without creating a second evidence-selection artifact?

## Recommendation

Recommendation: **revise / adopt as documentation boundary**.

Scope label: **active-core-aligned plot boundary candidate**, not runtime behavior.

Why: Punk already needs a boundary between raw request and contract draft readiness. External prior art supports shaping before building, but Punk must preserve gate/proof authority and project-memory discipline.

Risks: premature moduleization, active-surface overclaiming, autoplan cargo-culting, hidden routing authority, unstable citations, and broad docs rewrites.

What stays out of scope: implementation, CLI, runtime storage, modules, adapters, code execution, publishing, gate/proof writers, acceptance claims.

Next smallest safe artifact: documentation-only Plot Intake boundary patch with research note, idea artifact, product boundary doc, eval/spec drafts, and Level 0 work/report evidence.
