# PubPunk Inventory Reader v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk inventory reader model.

The reader model is the boundary before `PubPunkInventoryInputPacket`. It turns
explicit caller-provided observed refs into an input packet while preserving
allowed-source boundaries and empty new-project inventories.

This spec does not activate PubPunk runtime, Module Host runtime, public CLI
behavior, workspace initialization, filesystem scanning, draft body reads,
provider orchestration, adapter invocation, publishing, metrics collection,
receipt writing, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-READER-001: explicit workspace ref is required

The reader input must carry a safe publishing workspace ref such as
`punk-publishing://project/<project-id>` or a safe repo-relative ref.

### PUBPUNK-READER-002: split explicit refs are required

The reader input must use workspace policy `split_explicit_refs`. Repo-only,
global-only, project `.punk`, or external-only policies are not selected for
this slice.

### PUBPUNK-READER-003: required instruction refs are present

The reader input must include the current thin instruction refs for PubPunk,
workspace instructions, Module Authoring, Module Conformance, Module Host
Contract, Instruction Sources, and the publishing README.

### PUBPUNK-READER-004: read capability is narrow

The reader input must grant only `read_workspace_metadata`. It must block
publishing, metrics collection, adapter invocation, credential access, draft
file reads, filesystem writes, gate writing, proofpack writing, and acceptance
claims.

### PUBPUNK-READER-005: observed refs stay inside allowed refs

Every observed item source ref must be safe and present in the allowed source
refs. The model must reject unsafe, absolute, traversal, URL, and unallowed
observed refs.

### PUBPUNK-READER-006: empty new-project inventory is allowed

An empty observed item set is valid. New projects may have no drafts, receipts,
or metrics snapshots yet.

### PUBPUNK-READER-007: raw bodies remain blocked

The reader model accepts metadata refs only. Raw post bodies, raw external
payloads, credentials, customer data, sensitive code, and raw prompts must be
blocked.

### PUBPUNK-READER-008: reader builds packet only when ready

The reader may build `PubPunkInventoryInputPacket` only when reader assessment
has no blockers. The built packet grants `assess_provided_inventory` for the
next model and carries observed item metadata forward.

### PUBPUNK-READER-009: side-effect boundary stays false

The reader and its smoke eval must expose boundary flags proving no public CLI,
filesystem IO, workspace initialization, publication receipt writing, external
API/browser behavior, credential read, adapter invocation, gate decision
writing, proofpack writing, or acceptance claim creation.

## Expected advisory result

```yaml
pubpunk_inventory_reader_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  requires_instruction_refs: true
  requires_allowed_source_refs_for_observed_items: true
  requires_read_workspace_metadata_grant: true
  expected_receipt_fields_required: true
  empty_inventory_allowed: true
  builds_input_packet_when_ready: true
  ready_for_runtime_activation: false
  ready_for_side_effects: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, real
filesystem readers, manifest parsing, workspace creation, directory walking,
draft body reads, draft generation, publication planning, receipt writing,
adapter behavior, external publishing, metrics collection, token collection,
gate writing, proofpack writing, or acceptance claims.
