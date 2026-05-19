# Module Authoring Baseline v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary cases before Punk creates or expands domain
modules such as PubPunk, DevPunk, or future modules.

This is a design/spec artifact only. It does not activate Module Host runtime,
module manifest parsing, module registry behavior, provider orchestration,
workspace initialization, adapter invocation, public CLI behavior, runtime
storage, publishing, gate writing, proofpack writing, or acceptance claims.

## Status and authority

Module authoring conformance is advisory until a later validator, module host
path, or gate policy promotes it.

## Required deterministic eval cases

### MODULE-AUTHOR-001: baseline version is explicit

A module proposal must cite a module-authoring baseline version or record an
explicit no-baseline rationale.

### MODULE-AUTHOR-002: module purpose and non-goals are explicit

A module proposal without purpose and non-goal statements is incomplete.

### MODULE-AUTHOR-003: module cannot own truth

A module proposal that makes module state, prompts, generated views, or external
workspace contents authoritative project truth is invalid.

### MODULE-AUTHOR-004: final decisions remain gate-owned

A module proposal must not write final decisions, proofpacks, acceptance claims,
or gate outcomes.

### MODULE-AUTHOR-005: direct event-log mutation is forbidden

A module proposal must not mutate event logs directly. Future host-approved
events must remain host validated.

### MODULE-AUTHOR-006: capabilities are deny-by-default

Filesystem, network, environment, secrets, process, shell, publishing, adapter,
event-log, decision, proof, and project-memory write capabilities must be denied
unless separately granted by a later bounded goal.

### MODULE-AUTHOR-007: side effects require declared policy

Publishing, API calls, browser behavior, credential access, comments, pull
requests, deploys, and metrics collection require explicit side-effect policy,
receipt requirements, and conformance coverage before activation.

### MODULE-AUTHOR-008: workspace policy is explicit

A module proposal must declare `project_punk`, `global_punk`,
`external_workspace`, or `repo_native` workspace policy before implementation.

### MODULE-AUTHOR-009: empty workspace initialization is separate

A module may require a workspace initializer, but initializer behavior must not
be implied by the module-authoring baseline. It needs a later bounded goal.

### MODULE-AUTHOR-010: instruction sources are thin and cited

Module instruction refs must point to source artifacts. They must not define
hidden acceptance criteria, provider-specific authority, or module-owned truth.

### MODULE-AUTHOR-011: generated instruction views are non-authoritative

Generated instruction views or host-specific runner aids must stay rebuildable,
cited, and non-authoritative.

### MODULE-AUTHOR-012: Deliberation Budget is recorded

Module boundary work should record `standard` Deliberation Budget or explicit
smaller-budget rationale.

### MODULE-AUTHOR-013: conformance packet bridges rules to implementation

Before implementation, a module should have a conformance packet or explicit
no-packet rationale with limitations.

### MODULE-AUTHOR-014: DevPunk gets no exemptions

A development module may be proposed, but it must not receive special authority,
direct event-log write access, final-decision authority, or bypass rights.

### MODULE-AUTHOR-015: PubPunk cannot be a one-off

PubPunk implementation must cite the baseline, workspace policy, instruction
plan, Deliberation Budget, side-effect boundary, and conformance refs before a
bounded implementation slice starts.

### MODULE-AUTHOR-016: baseline is not runtime

The baseline must not imply active module loading, registry, manifest parser,
checker, workspace initializer, provider orchestration, adapter invocation, or
public CLI behavior.

### MODULE-AUTHOR-017: module lifecycle is explicit

A module proposal must declare `parked`, `incubating`, or `active` status and
must not self-promote.

### MODULE-AUTHOR-018: module outputs stay advisory before host/gate

Module receipts, module assessments, draft refs, runner aids, hints, and
side-effect request proposals must remain advisory until host validation and
gate/proof paths apply.

## Minimal fixture shape

```yaml
module_authoring_baseline_fixture:
  module:
    id: pubpunk
    status: parked
    baseline_version: module-authoring-baseline.v0.1
    lifecycle_tier: parked
    purpose: "Publishing and editorial workflows."
    non_goals:
      - "No external publishing."
      - "No adapter invocation."
      - "No final decisions."
    workspace_policy: external_workspace
    instruction_refs: []
    deliberation_budget_ref: work/reports/example.md
    capability_grants: []
    denied_capabilities:
      - filesystem_write
      - network
      - secrets
      - external_publishing
      - direct_event_log_write
      - final_decision_write
    output_kinds:
      - module_receipt
      - module_assessment
      - side_effect_request_proposal
    conformance_refs:
      - evals/specs/module-authoring-baseline.v0.1.md
    ready_for_implementation: false
```

## Non-goals

This spec does not define module code, module manifests as active runtime
schema, module registry behavior, plugin packaging, Module Host runtime,
provider routing, DevPunk implementation, PubPunk implementation, workspace
creation, instruction generation, checker implementation, adapter behavior,
publishing behavior, gate writing, proofpack writing, or acceptance claims.
