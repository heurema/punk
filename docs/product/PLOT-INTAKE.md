---
id: docs_product_plot_intake
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
review_after: 2026-06-19
canonical_for:
  - plot-intake-boundary-candidate
  - work-intake-alias-candidate
  - plot-intake-output-shape-candidate
  - plot-intake-authority-boundary-candidate
related_docs:
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/CONTRACT-SCHEMA.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/MODULES.md
related_evals:
  - evals/specs/plot-intake-boundary.v0.1.md
  - evals/specs/plot-intake-routing-recommendation.v0.1.md
  - evals/specs/plot-intake-evidence-plan.v0.1.md
supersedes: []
superseded_by: null
---

# Plot Intake

## Status

This document is an active product boundary proposal.

It describes how Punk may shape raw work inside `plot` before domain execution.

It does not activate runtime behavior, CLI commands, modules, adapters, storage writers, gate decisions, proofpacks, acceptance claims, publishing, or code execution.

## Purpose

Plot Intake is the upstream part of `plot` that turns a raw user request into a bounded, inspectable, evidence-aware candidate for contract drafting.

Human-facing alias: **Work Intake**.

Output-facing sub-boundary: **Contract Intake**.

## Placement

Plot Intake lives inside the existing lifecycle:

```text
plot -> cut -> gate
```

It does not create a fourth phase.

```text
raw request
  -> plot intake assessment
  -> contract draft readiness
  -> user confirmation / contract approval path later
  -> cut later
  -> gate later
```

## Non-goals

Plot Intake is not:

- `DevPunk`;
- a standalone module;
- a public CLI command;
- a runtime router;
- a prompt manager;
- a hidden memory store;
- an execution engine;
- a decision surface;
- a proofpack;
- an acceptance authority.

## Required distinctions

Plot Intake must preserve:

```text
raw request != intent candidate
intent candidate != contract draft
contract draft != approved contract
user confirmation != gate acceptance
routing recommendation != module authority
gate decision != proofpack
proofpack != acceptance authority
```

## Intake artifact alignment

This section keeps Plot Intake aligned with the existing owners of contract,
module, and project-memory concepts instead of redefining their schemas.

Before contract drafting, a Plot Intake assessment may surface:

| Area | Plot Intake may surface |
|---|---|
| Request and intent | Raw request reference, intent candidate, problem or business intent, assumptions, unknowns, contradictions, clarification questions. |
| Scope and effects | Smallest useful slice, scope include, scope exclude, forbidden effects. |
| Evidence and readiness | Evidence plan, receipt requirements candidate, eval plan candidate, contract draft readiness, open risks. |
| Advisory routing | Routing recommendation, routing alternatives, routing rationale, routing confidence. |
| Project memory | Project-memory refs, research refs, doc impact candidate, knowledge impact candidate. |

## Intake lenses

Plot Intake can use advisory lenses:

| Lens | Role |
|---|---|
| Product shaper | Clarifies problem, business intent, value, and smallest useful slice. |
| Domain expert | Identifies domain-specific unknowns and vocabulary. |
| Architect | Surfaces feasibility, boundary, integration, and test implications. |
| Risk reviewer | Finds side effects, privacy/security risks, forbidden effects, and trust-boundary risks. |
| Evidence planner | Maps acceptance needs to evidence, receipts, evals, source policy, and proof requirements. |
| Module router | Recommends a later domain lane with rationale, alternatives, and confidence. |
| Project-memory reviewer | Links relevant goals, research, docs, ADRs, reports, stale refs, contradictions, and open risks. |

These lenses are advisory. They do not decide.

## Advisory routing guidance

When Plot Intake recommends a later domain lane, keep the recommendation
explainable and non-authoritative.

The routing note should include:

- recommended route;
- alternative routes considered;
- rationale;
- confidence;
- blockers;
- required evidence before execution;
- statement that final decisions remain with `gate`.

It must not activate a module, invoke an adapter, execute work, publish, or write final decisions.

## Forbidden behavior

Plot Intake must not:

- write code;
- publish;
- invoke adapters;
- run domain execution;
- create receipts;
- write gate decisions;
- write proofpacks;
- claim acceptance;
- approve contracts by itself;
- treat assumptions as confirmed requirements;
- create hidden memory;
- create a second source of truth;
- expose a public CLI surface before roadmap promotion.

## Project-memory boundary

Plot Intake outputs may become project-memory-relevant only through explicit repo-tracked artifacts and authority tags.

They must not live only in chat, hidden executor memory, provider-local state, or prompt text.

## Current documentation boundary

Documented now in this draft:

- boundary terminology;
- output-shape proposal;
- forbidden behavior list;
- eval/spec direction.

Not active now:

- runtime writer;
- storage;
- CLI;
- module host integration;
- module routing runtime;
- generated executor briefs;
- domain execution;
- gate/proof behavior.
