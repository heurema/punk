# PubPunk Workspace Instruction Packet v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the PubPunk workspace and instruction
packet before selecting the next PubPunk implementation slice.

This spec does not activate PubPunk module execution, Module Host runtime, module
loading, manifest parsing, workspace initialization, filesystem scanning,
instruction generation, adapter invocation, publishing, metrics collection,
receipt writing, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-WORKSPACE-001: split explicit refs are selected

The packet must select `split_explicit_refs` for PubPunk workspace policy.

### PUBPUNK-WORKSPACE-002: repo-native refs stay repo-relative

Repo-native Punk publishing refs must be repo-relative refs under
`publishing/`.

### PUBPUNK-WORKSPACE-003: external workspaces are explicit pointers

Project-specific publishing workspaces must be represented as logical or
caller-provided external workspace refs, not hardcoded local host paths in
repo-tracked artifacts.

### PUBPUNK-WORKSPACE-004: workspace initialization is not implied

The packet may identify future workspace initialization, but it must not create
or imply active workspace creation behavior.

### PUBPUNK-WORKSPACE-005: instruction refs are thin and cited

The packet must cite PubPunk, Module Authoring, Module Conformance, Module Host
Contract, Instruction Sources, and publishing README refs without making the
instruction packet canonical project truth.

### PUBPUNK-WORKSPACE-006: caller-provided metadata is the default

The packet must keep caller-provided metadata as the default input source for
the next PubPunk slice.

### PUBPUNK-WORKSPACE-007: draft body reads are not granted

The packet must not grant filesystem reads or raw draft body reads.

### PUBPUNK-WORKSPACE-008: future filesystem read is a separate request

Any scoped filesystem-read capability must remain a future request, not a
grant.

### PUBPUNK-WORKSPACE-009: assessment fields are selected

The packet must select advisory assessment fields for module id, operation,
workspace policy, workspace ref, allowed source refs, capability grants,
denied capabilities, inventory counts, gap findings, side-effect requests,
token cost ref, and non-authority.

### PUBPUNK-WORKSPACE-010: side effects are proposals only

Publishing, metrics collection, adapter invocation, browser behavior, network
calls, credential reads, receipt writes, gate decisions, proofpacks, and
acceptance claims must remain denied or future proposals only.

### PUBPUNK-WORKSPACE-011: next readiness is bounded

The packet may make PubPunk ready for selecting the next pure metadata or
advisory model slice, but it must not claim runtime activation readiness.

## Expected advisory result

```yaml
pubpunk_workspace_instruction_packet_result:
  status: done
  workspace_policy: split_explicit_refs
  ready_for_docs_only_next: true
  ready_for_next_pure_model_slice_selection: true
  ready_for_runtime_activation: false
  ready_for_side_effects: false
  next_smallest_step: "Select the next PubPunk implementation slice against existing punk-mod-pubpunk boundaries."
  non_authority: true
```

## Non-goals

This spec does not define module code, host code, manifest schema,
deterministic checker implementation, module registry behavior, Module Host
runtime, PubPunk module execution, workspace creation, instruction generation, adapter
behavior, publishing behavior, metrics collection, gate writing, proofpack
writing, or acceptance claims.
