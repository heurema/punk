---
id: eval_spec_plot_intake_routing_recommendation_v0_1
kind: eval-spec
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-06-08
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

### Case 5 — harness command bridge is thin

A future harness command such as `/punk ...` must be treated as a runner aid
bridge into Punk-owned intake routing. The harness command may package raw user
text, harness id, cwd/project root ref, explicit attachments, capability
declarations, and privacy notes.

It must not decide the route, activate a module, execute work, call providers,
invoke adapters, publish, write receipts, write events, write gate decisions,
write proofpacks, or claim acceptance.

### Case 6 — route separates phase and module

Routing output must separate the lifecycle phase from the domain module route.

Required fields:

- `phase_route`, such as `plot`, `cut`, `gate`, `blocked`, or `clarify`;
- `module_route`, such as `core`, `pubpunk`, `devpunk`, `future_module`, or
  `external`;
- rationale;
- blockers;
- required evidence before execution;
- side-effect status.

A request like `/punk draft a blog post about X` may recommend
`phase_route: plot` and `module_route: pubpunk`, but it must not imply that
PubPunk runtime, publishing, adapters, or draft writing are active.

### Case 7 — explicit Punk entrypoint owns route semantics

Harness-specific commands should call a Punk-owned intake entrypoint, such as a
future `punk intake route --request-file <path> --harness <harness-id>
--format json`.

Provider-specific slash-command files, prompts, skills, or local model settings
are runner aids only. They must not become project truth, route authority,
module authority, gate authority, or acceptance authority.

### Case 8 — route hints remain non-authoritative

Optional shorthand commands such as `/pub ...` or `/dev ...` may provide route
hints in a later design, but route hints must stay advisory. The router must be
able to return `clarify`, `blocked`, or a different module route when the
request conflicts with scope, evidence, side-effect policy, module status, or
Research Gate requirements.
