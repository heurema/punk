# Module Host Contract Stub v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the Module Host Contract Stub before
PubPunk, DevPunk, or any future module skeleton depends on a host shape.

This spec does not activate Module Host runtime, module loading, manifest
parsing, capability enforcement, module invocation, adapter invocation, event
writing, receipt writing, publishing, metrics collection, gate writing,
proofpack writing, or acceptance claims.

## Status and authority

The contract stub is advisory design evidence until a later implementation
slice promotes part of it.

## Required deterministic eval cases

### MODULE-HOST-CONTRACT-001: input bundle is explicit

A module skeleton proposal must identify the host-scoped input bundle fields it
expects or record an open conformance finding.

### MODULE-HOST-CONTRACT-002: capability envelope is deny-by-default

A module skeleton proposal must deny filesystem, network, environment, secrets,
process/shell, adapter invocation, external publishing, metrics collection,
direct event-log writes, final decision writes, and proofpack writes unless a
later bounded goal grants a scoped capability.

### MODULE-HOST-CONTRACT-003: capability request is not a grant

A side-effect or capability request must not be treated as approval, execution,
or receipt evidence.

### MODULE-HOST-CONTRACT-004: advisory outputs are bounded

Module outputs must stay within module assessment, module receipt, runner aid,
draft ref, plan ref, side-effect request proposal, or conformance finding until
later host/gate paths apply.

### MODULE-HOST-CONTRACT-005: event log writes are host-owned

A module must not mutate event logs directly. Host-approved event writing needs
a later selected boundary.

### MODULE-HOST-CONTRACT-006: side-effect proposal has blockers

A side-effect request proposal must include missing policy, payload, adapter,
receipt, credential, or proof blockers when those are unresolved.

### MODULE-HOST-CONTRACT-007: PubPunk starts without publishing grants

A PubPunk skeleton must keep external publishing, metrics collection, adapter
invocation, credential access, and receipt writing denied unless a later goal
explicitly selects one of those behaviors.

### MODULE-HOST-CONTRACT-008: read-only dry run is process evidence only

A trivial read-only module dry run may test the conformance process, but it must
not create active module runtime, a product module requirement, or acceptance
authority.

### MODULE-HOST-CONTRACT-009: contract stub is not runtime

The contract stub must not imply active module loading, manifest parsing,
capability enforcement, provider orchestration, adapter invocation, CLI
behavior, event writing, receipt writing, publishing, metrics collection, gate
writing, proofpack writing, or acceptance claims.

## Minimal fixture shape

```yaml
module_host_contract_stub_fixture:
  input_bundle:
    module_id: pubpunk
    contract_ref: null
    run_ref: null
    allowed_source_refs: []
    workspace_refs: []
    instruction_refs: []
    operation:
      kind: inventory_assessment
      requested_by_ref: work/goals/example.md
    capability_envelope_ref: module_capability_envelope_example
    non_authority: true
  capability_envelope:
    id: module_capability_envelope_example
    grants: []
    denies:
      - filesystem_read
      - filesystem_write
      - network
      - environment
      - secrets
      - process_shell
      - adapter_invoke
      - external_publish
      - metrics_collect
      - direct_event_log_write
      - final_decision_write
      - proofpack_write
    future_grant_requests: []
    non_authority: true
  allowed_outputs:
    - module_assessment
    - module_receipt
    - runner_aid
    - draft_ref
    - plan_ref
    - side_effect_request_proposal
    - conformance_finding
```

## Non-goals

This spec does not define module code, host code, manifest schema,
deterministic checker implementation, module registry behavior, Module Host
runtime, DevPunk implementation, PubPunk implementation, workspace creation,
instruction generation, adapter behavior, publishing behavior, metrics
collection, gate writing, proofpack writing, or acceptance claims.
