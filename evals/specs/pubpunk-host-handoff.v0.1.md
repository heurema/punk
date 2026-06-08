# PubPunk Host Handoff v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic smoke coverage for the first PubPunk-to-Module-Host
handoff.

The handoff proves that an explicit PubPunk inventory input packet can feed the
existing side-effect-free inventory assessment, then be wrapped by existing
Module Host preflight, advisory envelope, and receipt proposal models.

This spec does not activate PubPunk module execution, Module Host runtime, module
loading, filesystem reads, workspace initialization, publishing, metrics
collection, adapter invocation, receipt writing, gate writing, proofpack
writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-HOST-HANDOFF-001: packet readiness is first

The smoke case must start from `PubPunkInventoryInputPacket`, not directly from
`PubPunkInventoryInput`. The packet must be ready and advisory before any
assessment input conversion is considered.

### PUBPUNK-HOST-HANDOFF-002: packet converts to assessment input only when ready

The handoff may call the inventory assessment model only after
`try_into_inventory_input` succeeds. Blocked packets must not be treated as
module assessment input.

### PUBPUNK-HOST-HANDOFF-003: PubPunk assessment stays advisory

The PubPunk inventory assessment must stay side-effect-free and advisory. It
must not read files, publish, collect metrics, invoke adapters, write receipts,
write gate decisions, write proofpacks, or claim acceptance.

### PUBPUNK-HOST-HANDOFF-004: Module Host preflight validates explicit refs

The Module Host invocation envelope must use explicit safe repo-relative input
refs and the narrow `assess_provided_input` grant. Logical workspace refs such
as `punk-publishing://...` remain PubPunk packet metadata and must not become
Module Host input refs in this slice.

### PUBPUNK-HOST-HANDOFF-005: advisory envelope wraps module output only

The Module Host envelope must wrap an advisory output summary with
side-effect-free boundary flags. It must not invoke modules, load plugins, read
or write files, write receipts, mutate event logs, call APIs, read credentials,
invoke adapters, publish, or write gate/proof authority.

### PUBPUNK-HOST-HANDOFF-006: receipt proposal is local and non-authoritative

The Module Host receipt proposal must cover host validation and side-effect
fields from the advisory envelope without creating or writing a receipt.

### PUBPUNK-HOST-HANDOFF-007: no runtime promotion is implied

Passing this smoke case means only that the local model chain is coherent. It
does not promote PubPunk to active module execution, public CLI, adapter behavior,
external publishing, metrics collection, receipt writing, gate writing,
proofpack writing, or acceptance authority.

## Expected advisory result

```yaml
pubpunk_host_handoff_result:
  status: done
  starts_from_input_packet: true
  packet_ready: true
  inventory_assessment_ready: true
  module_host_preflight_ready: true
  advisory_envelope_ready: true
  receipt_proposal_ready: true
  receipt_proposal_writes_receipt: false
  module_invocation_runtime: false
  publishing_runtime: false
  side_effects: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication planning, receipt writing, adapter
behavior, external publishing, metrics collection, token collection, gate
writing, proofpack writing, or acceptance claims.
