# PubPunk Inventory Input Packet v0.1

Date: 2026-05-19
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk inventory input packet.

The packet is the boundary before the existing inventory assessment model. It
keeps PubPunk's first implementation step explicit and repeatable without
activating PubPunk module execution, Module Host runtime, module loading, workspace
initialization, filesystem scanning, draft body reads, provider orchestration,
adapter invocation, publishing, metrics collection, receipt writing, gate
writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-INPUT-PACKET-001: canonical module id is required

The packet must use module id `pubpunk`. Missing or non-canonical module ids
block readiness.

### PUBPUNK-INPUT-PACKET-002: split explicit refs are required

The packet must use workspace policy `split_explicit_refs`. Repo-only,
global-only, project `.punk`, or external-only policies are not selected for
this slice.

### PUBPUNK-INPUT-PACKET-003: workspace ref is explicit

The packet must include a safe explicit publishing workspace ref such as
`punk-publishing://project/<project-id>` or a safe repo-relative ref.

### PUBPUNK-INPUT-PACKET-004: required instruction refs are present

The packet must include the current thin instruction refs for PubPunk, workspace
instructions, Module Authoring, Module Conformance, Module Host Contract,
Instruction Sources, and the publishing README.

### PUBPUNK-INPUT-PACKET-005: source refs bound item metadata

Each inventory item source ref must be safe and present in the packet's allowed
source refs. Empty item sets are allowed for new projects with no publications.

### PUBPUNK-INPUT-PACKET-006: capability grant is explicit and narrow

The packet must grant `assess_provided_inventory` and must block publishing,
metrics, adapter, credential, gate, proof, acceptance, filesystem write, and
other unsupported capabilities.

### PUBPUNK-INPUT-PACKET-007: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, customer data, sensitive
code, and raw prompts must be blocked.

### PUBPUNK-INPUT-PACKET-008: expected receipt fields are required

The packet must require expected receipt fields even though this slice does not
write receipts.

### PUBPUNK-INPUT-PACKET-009: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-INPUT-PACKET-010: conversion only follows ready packet

The packet may convert into `PubPunkInventoryInput` only when packet readiness
has no blockers.

### PUBPUNK-INPUT-PACKET-011: side-effect boundary stays false

The packet and its smoke eval must expose boundary flags proving no public CLI,
filesystem IO, publication receipt writing, external API/browser behavior,
credential read, adapter invocation, gate decision writing, proofpack writing,
or acceptance claim creation.

## Expected advisory result

```yaml
pubpunk_inventory_input_packet_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  requires_instruction_refs: true
  requires_allowed_source_refs_for_items: true
  requires_assess_provided_inventory_grant: true
  expected_receipt_fields_required: true
  token_cost_ref: metadata_only
  ready_for_inventory_assessment_input_conversion: true
  ready_for_runtime_activation: false
  ready_for_side_effects: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, manifest
parsing, workspace creation, filesystem scanning, draft generation, publication
planning, receipt writing, adapter behavior, external publishing, metrics
collection, token collection, gate writing, proofpack writing, or acceptance
claims.
