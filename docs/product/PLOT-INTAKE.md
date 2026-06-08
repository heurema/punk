---
id: docs_product_plot_intake
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-06-08
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

Current code includes only a side-effect-free Plot Intake routing model in
`punk-contract` with smoke coverage in `punk-eval`. It classifies supported
slash-command-shaped request text into advisory phase/module route candidates
without activating a CLI command, harness adapter, module execution, runtime
storage, provider call, publication, receipt, gate decision, proofpack, or
acceptance claim.

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

## Harness slash command bridge candidate

Future harness commands such as `/punk ...` inside Codex CLI, Codex App,
Claude Code, Gemini, `agy`, or another executor shell may act as a thin bridge
into Plot Intake.

The harness command layer should not decide the route. It should package an
explicit request envelope and call a Punk-owned intake entrypoint, such as a
future CLI command:

```text
punk intake route --request-file <path> --harness <harness-id> --format json
```

The request envelope should include:

- raw user text;
- harness id, such as `codex-cli`, `codex-app`, `claude-code`, `gemini`, or
  `agy`;
- current working directory or project root ref;
- available capability declarations;
- explicit attachment or artifact refs;
- redaction and privacy notes;
- no secrets, credentials, private tokens, or hidden provider-local memory.

The route result should separate phase from module:

```yaml
intake_route_result:
  status: advisory
  authority: non_authoritative
  raw_request_ref: null
  intent_candidate: null
  phase_route: plot | cut | gate | blocked | clarify
  module_route: core | pubpunk | devpunk | future_module | external
  route_confidence: low | medium | high
  rationale: []
  alternatives_considered: []
  blockers: []
  required_evidence_before_execution: []
  next_handoff: null
  side_effects_active: false
  module_activated: false
  gate_decision_written: false
```

For example, `/punk draft a blog post about X` may route to:

```yaml
phase_route: plot
module_route: pubpunk
next_handoff: pubpunk.contract_intake
side_effects_active: false
```

This means the request is probably a PubPunk-shaped contract/intake problem. It
does not mean PubPunk module execution through the Punk-owned Module Host is
active, a draft has been written, publishing is allowed, adapters may run, or
the result has been accepted.

Optional shorter harness commands such as `/pub ...` or `/dev ...` may be
introduced later as explicit route hints, but hints are not authority. The Punk
router must still be able to return `clarify`, `blocked`, or a different route
when scope, evidence, side effects, or module status require it.

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
- harness slash command implementation;
- module host integration;
- module routing runtime;
- generated executor briefs;
- domain execution;
- gate/proof behavior.
