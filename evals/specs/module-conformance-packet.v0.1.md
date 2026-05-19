# Module Conformance Packet v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for advisory Module Conformance Packets.

This spec does not activate module loading, manifest parsing, runtime
validation, Module Host behavior, provider orchestration, workspace
initialization, adapter invocation, publishing, gate writing, proofpack writing,
or acceptance claims.

## Status and authority

Module Conformance Packet checks are advisory until a later validator, module
host path, or gate policy promotes them.

## Required deterministic eval cases

### MODULE-CONFORMANCE-001: packet cites baseline

A packet must cite `docs/product/MODULE-AUTHORING.md`,
`evals/specs/module-authoring-baseline.v0.1.md`, and its claimed
`baseline_version`.

### MODULE-CONFORMANCE-002: authority is non-authoritative

A packet must declare advisory/non-authoritative status and must not claim
gate, proof, acceptance, project-truth, or final-decision authority.

### MODULE-CONFORMANCE-003: module identity is explicit

A packet must include module id, module ref, lifecycle target, purpose summary,
and non-goals.

### MODULE-CONFORMANCE-003A: host contract is explicit

A packet that claims skeleton readiness must cite
`docs/product/MODULE-HOST-CONTRACT.md` or record an explicit no-contract
rationale with limitations.

### MODULE-CONFORMANCE-004: lifecycle target cannot self-promote

A packet may recommend readiness, but it must not promote a module to active
status by itself.

### MODULE-CONFORMANCE-005: workspace policy is explicit

A packet must record one selected workspace policy or an open finding that
blocks implementation readiness.

### MODULE-CONFORMANCE-006: empty workspace initializer is separate

A packet must not imply workspace initialization behavior unless a later
bounded goal selects that behavior.

### MODULE-CONFORMANCE-007: instruction policy is explicit

A packet must cite instruction source refs or record an open finding explaining
why they are missing.

### MODULE-CONFORMANCE-008: generated views are deferred or cited

Generated instruction views must be cited when present and marked deferred when
absent.

### MODULE-CONFORMANCE-009: Deliberation Budget is recorded

A packet must include a Deliberation Budget report ref or an explicit
limitation/rationale.

### MODULE-CONFORMANCE-010: capabilities are deny-by-default

A packet must list granted and denied capabilities. Empty grants are valid;
undeclared grants are invalid.

### MODULE-CONFORMANCE-011: side effects are not silently granted

Publishing, API calls, browser behavior, credential access, comments, PRs,
deploys, and metrics collection must be denied or backed by explicit future
policy references.

### MODULE-CONFORMANCE-012: direct event-log writes are denied

A packet must deny direct event-log mutation unless a later promoted host
validation path exists.

### MODULE-CONFORMANCE-013: final decision and proof authority are denied

A packet must deny final decision writing, proofpack writing, and acceptance
claims.

### MODULE-CONFORMANCE-014: outputs remain advisory

A packet's expected outputs must remain module receipts, module assessments,
runner aids, draft refs, or side-effect request proposals until later host/gate
paths apply.

### MODULE-CONFORMANCE-015: findings are explicit

A packet with missing information must record findings with id, severity,
status, and question or resolution.

### MODULE-CONFORMANCE-016: readiness is split

A packet must distinguish implementation readiness from runtime activation
readiness.

### MODULE-CONFORMANCE-017: next smallest step is explicit

A packet must name the smallest next step or record that the module is not
selected.

### MODULE-CONFORMANCE-018: DevPunk has no exemptions

A DevPunk packet must satisfy the same checks as any other module packet.

### MODULE-CONFORMANCE-019: PubPunk urgency is not exemption

A PubPunk packet may record urgency, but it must still satisfy baseline,
workspace, instruction, side-effect, evidence, and conformance requirements.

### MODULE-CONFORMANCE-020: packet is not runtime

The packet must not imply a parser, checker, module registry, Module Host
activation, CLI command, provider orchestration, adapter invocation, workspace
initializer, publishing behavior, gate writer, proofpack writer, or acceptance
claim.

## Minimal fixture shape

```yaml
module_conformance_packet_fixture:
  id: module_conformance_pubpunk_v0_1
  status: advisory
  authority: non_authoritative
  module_id: pubpunk
  module_ref: docs/modules/pubpunk.md
  baseline_version: module-authoring-baseline.v0.1
  baseline_ref: docs/product/MODULE-AUTHORING.md
  host_contract_ref: docs/product/MODULE-HOST-CONTRACT.md
  eval_refs:
    - evals/specs/module-authoring-baseline.v0.1.md
    - evals/specs/module-conformance-packet.v0.1.md
    - evals/specs/module-host-contract-stub.v0.1.md
  lifecycle_target: parked
  purpose_summary: "Publishing and editorial workflows."
  non_goals:
    - "No external publishing."
    - "No adapter invocation."
    - "No final decisions."
  workspace_policy:
    selected: external_workspace
    source_of_truth_refs: []
    initializer_status: not_selected
  instruction_policy:
    source_refs: []
    generated_views: deferred
  deliberation:
    budget_class: standard
    report_ref: null
    limitation: "No attached deliberation report yet."
  capabilities:
    granted: []
    denied:
      - direct_event_log_write
      - final_decision_write
      - proofpack_write
      - external_publish
      - credential_read
      - adapter_invoke
  expected_outputs:
    - module_assessment
    - module_receipt
    - runner_aid
    - side_effect_request_proposal
  findings:
    - id: missing_workspace_source_refs
      severity: medium
      status: open
      question: "Where is the operational workspace source of truth?"
  readiness:
    ready_for_implementation: false
    ready_for_runtime_activation: false
    next_smallest_step: "Resolve open findings."
  non_authority: true
```

## Non-goals

This spec does not define module code, manifest parsing, packet parsing,
deterministic checker implementation, module registry behavior, Module Host
runtime, DevPunk implementation, PubPunk implementation, workspace creation,
instruction generation, adapter behavior, publishing behavior, gate writing,
proofpack writing, or acceptance claims.
