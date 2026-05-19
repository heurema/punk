# Trivial Read-Only Module Dry Run v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the expected result for a trivial read-only module dry run that tests the
Module Conformance Packet and Module Host Contract Stub before PubPunk
skeleton work.

This spec does not create a product module, module runtime, Module Host
activation, module loading, manifest parser, capability enforcement, workspace
initializer, adapter invocation, publishing, metrics collection, gate writing,
proofpack writing, or acceptance claims.

## Fixture module candidate

Use this candidate only as process evidence:

```yaml
module_candidate:
  module_id: read_only_fixture
  status: dry_run_fixture
  authority: non_authoritative
  purpose: "Validate module conformance process with a no-domain read-only fixture."
  non_goals:
    - "No product module."
    - "No runtime module."
    - "No filesystem read."
    - "No filesystem write."
    - "No event-log mutation."
    - "No external side effects."
    - "No PubPunk or DevPunk implementation."
  lifecycle_target: parked
  workspace_policy: repo_native
  initializer_status: not_selected
  instruction_refs:
    - docs/product/MODULE-AUTHORING.md
    - docs/product/MODULE-CONFORMANCE.md
    - docs/product/MODULE-HOST-CONTRACT.md
  host_contract_ref: docs/product/MODULE-HOST-CONTRACT.md
  non_authority: true
```

## Required deterministic eval cases

### READ-ONLY-DRY-RUN-001: fixture is not a product module

The dry run must identify `read_only_fixture` as process evidence only and must
not add it to product module status as DevPunk, PubPunk, or any future domain
module.

### READ-ONLY-DRY-RUN-002: conformance refs are present

The dry run must cite Module Authoring Baseline, Module Conformance Packet,
Module Host Contract Stub, and their eval/spec refs.

### READ-ONLY-DRY-RUN-003: capability grants are empty

The dry run must grant no capabilities.

### READ-ONLY-DRY-RUN-004: denied capabilities are explicit

The dry run must deny filesystem read/write, network, environment, secrets,
process/shell, adapter invocation, external publishing, metrics collection,
direct event-log writes, final decision writes, and proofpack writes.

### READ-ONLY-DRY-RUN-005: input bundle is scoped

The dry run must use a host-scoped input bundle with explicit source refs and
no hidden project truth.

### READ-ONLY-DRY-RUN-006: outputs are advisory only

The dry run may produce conformance findings and a module assessment summary.
It must not write receipts, events, gate decisions, proofpacks, or acceptance
claims.

### READ-ONLY-DRY-RUN-007: side-effect requests are absent

The dry run must not request external publishing, metrics collection, adapter
invocation, credential reads, browser behavior, comments, PRs, or deploys.

### READ-ONLY-DRY-RUN-008: readiness is split

The dry run must distinguish process readiness from implementation readiness
and runtime activation readiness.

### READ-ONLY-DRY-RUN-009: result points to PubPunk packet

If the dry run passes, the next recommended work should be applying Module
Conformance Packet v0.1 to PubPunk, not implementing runtime host behavior.

### READ-ONLY-DRY-RUN-010: token accounting is honest

The dry run report should include deliberation cost-accounting status when
available. Missing exact token usage must be recorded as unavailable or
unknown, not as zero.

## Expected advisory result

```yaml
read_only_module_dry_run_result:
  status: passed
  process_ready_for_pubpunk_packet: true
  ready_for_fixture_implementation: false
  ready_for_runtime_activation: false
  next_smallest_step: "Apply Module Conformance Packet v0.1 to PubPunk."
  non_authority: true
```

## Non-goals

This spec does not define module code, host code, manifest schema, deterministic
checker implementation, module registry behavior, Module Host runtime,
DevPunk implementation, PubPunk implementation, workspace creation, instruction
generation, adapter behavior, publishing behavior, metrics collection, gate
writing, proofpack writing, or acceptance claims.
