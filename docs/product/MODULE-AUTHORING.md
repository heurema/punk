---
id: docs_product_module_authoring
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-21
review_after: 2026-06-19
canonical_for:
  - module-authoring-baseline
  - module-authoring-conformance-boundary
  - module-manifest-baseline-boundary
  - module-naming-baseline
  - module-workspace-policy-boundary
  - module-instruction-boundary
  - development-module-strategy-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/MODULES.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/product/MODULE-CONTROL-PLANE.md
  - docs/product/MODULE-HOST.md
  - docs/product/DELIBERATION-BUDGET.md
  - docs/product/INSTRUCTION-SOURCES.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/REVIEW-ASSESSMENT.md
  - docs/product/PLOT-INTAKE.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/module-authoring-baseline.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
  - evals/specs/module-naming-baseline.v0.1.md
  - evals/specs/module-conformance-packet.v0.1.md
supersedes: []
superseded_by: null
---

# Module Authoring Baseline

## Purpose

Define the minimum rules every Punk module must satisfy before module-specific
implementation work starts.

The immediate reason is PubPunk pressure. Publishing hurts now, but a one-off
publishing module would make later modules drift. This baseline keeps PubPunk
and future modules on the same path.

## Baseline rule

Use Module Authoring Baseline v0.1 before creating or expanding a module.

The baseline is not a runtime, SDK, package format, plugin loader, or module
host implementation. It is the shared authoring contract that future module
docs, manifests, evals, and conformance packets must cite.

## Module anatomy

Use the same minimum anatomy for every module:

| Area | Required authoring decision |
|---|---|
| Identity | Module id, status, owner, baseline version, lifecycle tier. |
| Purpose | Domain capability and explicit non-goals. |
| Inputs | Contract refs, run refs, source refs, workspace refs, capability grants. |
| Outputs | Module receipts, module assessments, runner aids, draft refs, side-effect request proposals. |
| Workspace | Global `.punk`, project `.punk`, or external workspace policy. |
| Instructions | Thin instruction source refs and optional generated views. |
| Deliberation | Budget class, trigger reasons, independent pass refs or limitation notes. |
| Receipts and evidence | Required receipt fields, evidence refs, side-effect visibility. |
| Capabilities | Deny-by-default capability list and selected grants. |
| Conformance | Eval/spec refs and baseline conformance packet refs. |

Missing anatomy is a module-authoring finding. It does not activate runtime
failure behavior until a later validator or module host path is promoted.

## Naming policy

Each domain module must declare one canonical root.

Use that root to derive the other names mechanically:

| Surface | Rule | PubPunk example |
|---|---|---|
| Canonical root | stable module root | `pubpunk` |
| Product/module prose name | product-owned PascalCase | `PubPunk` |
| Module id | canonical root in slug form | `pubpunk` |
| Module docs path | `docs/modules/<module-id>.md` | `docs/modules/pubpunk.md` |
| Rust crate/package | `punk-mod-<module-id>` | `punk-mod-pubpunk` |
| Rust import | crate name as snake case | `punk_mod_pubpunk` |
| Short alias | future UX or CLI alias only after selection | `pub` proposed-only |

Infrastructure crates such as `punk-module-host` are not domain modules and do
not need the `punk-mod-<module-id>` pattern.

Do not introduce one-off shortened crate names when the module id is already
selected. For example, the PubPunk crate uses `punk-mod-pubpunk`, not
`punk-mod-pub`.

This naming policy is documentation and eval guidance. It is not a manifest
parser, package registry, crate publisher, CLI surface, or module loader.

## Manifest baseline

Future module manifests should pin the baseline they claim to follow:

```yaml
module:
  id: pubpunk
  status: parked
  baseline_version: module-authoring-baseline.v0.1
  lifecycle_tier: parked | incubating | active
  owner: vitaly
  purpose: "Publishing and editorial workflows."
  workspace_policy_ref: docs/product/MODULE-AUTHORING.md#workspace-policy
  instruction_refs: []
  capability_grants: []
  forbidden_capabilities:
    - final_decision_write
    - direct_event_log_write
    - undeclared_external_publish
  conformance_refs:
    - evals/specs/module-authoring-baseline.v0.1.md
```

This is a documentation boundary. It is not an active manifest schema, parser,
validator, registry, or loader.

## Workspace policy

Modules must declare where their durable artifacts live.

Allowed workspace policies:

| Policy | Use when | Boundary |
|---|---|---|
| `project_punk` | The artifacts belong to one user project and should be repo-tracked with that project. | Use project `.punk/memory/` and `.punk/instructions/` conventions. |
| `global_punk` | The artifacts are user-level operational state shared across projects. | Use a user-level Punk workspace; do not pollute the project repo. |
| `external_workspace` | The artifacts are operational surfaces that should stay outside the product repo, such as publishing workspaces. | Record an explicit pointer and receipts; do not treat the external workspace as hidden truth. |
| `repo_native` | The module owns product/public docs that intentionally live in the repository, such as `publishing/`. | Keep canonical refs and DocImpact explicit. |

A module may initialize an empty workspace when that behavior is explicitly
selected later. Until then, workspace initialization remains an authoring
requirement and boundary, not active behavior.

## Instruction policy

Module instructions must stay thin.

Use instruction source pages to point an executor, model, module, or future host
to relevant artifacts. Do not encode hidden acceptance criteria, module truth,
or provider-specific authority in instruction text.

Instruction sources may later produce host-specific runner aids. Generated
views remain rebuildable and non-authoritative.

## Adaptive control plane direction

Future modules may separate a stable implementation from editable behavior
artifacts, such as instruction pages, skills, playbooks, profiles, templates,
routing rules, and recurring workflow preferences.

That separation is an editability boundary. It is not a trust boundary.

Editable behavior artifacts can change module behavior. They must not be
treated as safety-light config merely because they are text. A behavior
artifact change that affects module behavior should move through explicit
evidence, eval/check refs, gate rationale, and provenance before promotion.

Future module control manifests should declare:

- behavior artifact refs;
- immutable core boundaries;
- capability requests and denies;
- behavior-provenance fields;
- reflection trigger status;
- tuning proposal refs, when present;
- latest promoted gate or decision refs, when present.

The canonical control manifest and tuning proposal contract lives in
`docs/product/MODULE-CONTROL-PLANE.md`.

Reflection may generate advisory tuning proposals. It must not apply changes,
grant capabilities, rewrite immutable core boundaries, mutate event logs, write
final decisions, write proofpacks, or make acceptance claims.

This direction is documentation and eval guidance only. It does not add a
module control manifest schema, parser, registry, background reflection job,
Module Host runtime, adapter invocation, or behavior-artifact writer.

## Deliberation policy

Module boundary work defaults to `standard` Deliberation Budget.

The goal/report for module-authoring work should record:

- budget class;
- trigger reasons;
- source refs;
- independent advisory pass summaries;
- tension map;
- synthesis;
- rejected alternatives;
- unresolved questions.

If fewer independent passes are used, the report should record why. Missing
provider diversity is a limitation, not a reason to pretend independence.

## Conformance packet

Use a conformance packet as the reviewable bridge between rules and future
implementation. The canonical packet boundary lives in
`docs/product/MODULE-CONFORMANCE.md`.

```yaml
module_conformance_packet:
  id: module_conformance_pubpunk_v0_1
  status: advisory
  authority: non_authoritative
  module_ref: docs/modules/pubpunk.md
  baseline_version: module-authoring-baseline.v0.1
  manifest_ref: null
  workspace_policy: external_workspace
  instruction_refs: []
  deliberation_budget_ref: null
  eval_refs:
    - evals/specs/module-authoring-baseline.v0.1.md
  findings:
    - id: missing_manifest
      severity: medium
      status: unverified
  ready_for_implementation: false
  non_authority: true
```

The packet is advisory until a later validator, module host path, or gate policy
promotes it.

## Enforcement direction

Do not rely on future agents remembering this document.

The durable enforcement path is:

```text
baseline doc
  -> eval/spec cases
  -> conformance packet
  -> deterministic checker or smoke eval
  -> module host validation later
  -> gate/proof later, when applicable
```

Current slice defines the first two links. It does not add a checker, runtime
validator, CLI command, registry, or module host behavior.

When a later module work order starts, it should fail review if it does not
cite:

- `docs/product/MODULE-AUTHORING.md`;
- `evals/specs/module-authoring-baseline.v0.1.md`;
- the module's conformance packet or explicit no-packet rationale;
- Deliberation Budget result or explicit smaller-budget rationale.

## Development module strategy

There are two separate ideas:

1. module-authoring discipline;
2. a future development module, often called DevPunk.

This baseline implements the first idea only.

A future DevPunk may become an advisory module for software development,
module scaffolding, code review, eval execution, and patch preparation. It must
not receive special exemptions. It may propose changes, collect evidence, and
emit assessments, but it cannot own truth, bypass contracts, mutate event logs,
write final decisions, or claim acceptance.

DevPunk is not required to become active before this baseline exists. A later
bounded goal should decide whether to create a DevPunk advisory skeleton before
or alongside PubPunk.

## PubPunk path

Before PubPunk implementation work starts, require:

- Module Authoring Baseline v0.1 cited by the work goal;
- PubPunk module manifest or explicit draft manifest;
- workspace policy selected;
- instruction refs or instruction plan;
- side-effect boundary kept denied by default;
- Deliberation Budget recorded;
- conformance packet drafted;
- eval/spec coverage linked.

PubPunk remains parked until those artifacts exist and a later goal explicitly
selects a bounded implementation slice.

## Forbidden behavior

Module authoring must not:

- activate Module Host runtime;
- create plugin runtime behavior;
- create active module registry behavior;
- add provider orchestration;
- add public CLI commands;
- write `.punk/` runtime state;
- invoke modules or adapters;
- publish externally;
- read credentials;
- mutate event logs directly;
- write gate decisions;
- write proofpacks;
- claim acceptance;
- let any module own project truth;
- create special exemptions for DevPunk or PubPunk.

## Current documentation boundary

Documented now:

- common module anatomy;
- module naming policy;
- baseline manifest expectations;
- workspace policy choices;
- instruction policy;
- Deliberation Budget usage;
- conformance packet shape;
- enforcement direction;
- DevPunk/PubPunk sequencing boundary.

Not active now:

- module manifest parser;
- module registry;
- conformance checker;
- Module Host runtime;
- DevPunk;
- PubPunk implementation;
- adapter invocation;
- workspace initializer;
- instruction generator;
- receipt writer expansion;
- gate/proof behavior.
