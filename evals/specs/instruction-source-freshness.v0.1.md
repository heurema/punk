# Instruction Source Freshness v0.1

Date: 2026-05-18
Status: proposed boundary
Authority: advisory/design

## Purpose

Define boundary checks for `.punk/instructions/` source pages, future generated
instruction views, and future host-specific runner-aid transforms.

This is a design/spec artifact only. It does not activate generated instruction
writers, public instruction CLI behavior, module runtime, provider adapters,
skill installation, gate writing, proofpack writing, or acceptance claims.

## Status and authority

Instruction source pages are advisory local orientation aids.

Generated instruction views are rebuildable and non-authoritative.

## Required deterministic eval cases

### INSTR-FRESH-001: generated view cites source pages

Every generated instruction view item must cite a source instruction page ref.
Items without source refs are invalid.

### INSTR-FRESH-002: missing source page fails freshness

A generated view that cites a deleted or missing source page must be marked
stale.

### INSTR-FRESH-003: source hash mismatch fails freshness when hashes are declared

If a generated view declares source hashes, any mismatch must mark the view
stale or require rebuild. Silent acceptance is invalid.

### INSTR-FRESH-004: generated view cannot own truth

A generated instruction view must not introduce rules, acceptance criteria,
module authority, adapter authority, or active behavior not present in source
refs.

### INSTR-FRESH-005: future capability claims are flagged

Instruction pages or generated views that describe parked/future behavior as
active must be flagged.

### INSTR-FRESH-006: host-specific path leakage fails closed

A host-specific generated runner aid must not contain another host's skill path,
tool name, or setup command when the transform config forbids it.

### INSTR-FRESH-007: transform is deterministic

For the same source refs, config, and host target, generated instruction output
must be deterministic.

### INSTR-FRESH-008: external side effects are not implied

Instruction pages or generated views must not imply browser, network,
publishing, deploy, credential, or provider behavior is active without a
promoted side-effect policy and receipt path.

### INSTR-FRESH-009: non-authority statement is preserved

Generated instruction views and host-specific runner aids must preserve a
non-authority statement or link to the runner-aid boundary.

### INSTR-FRESH-010: source pages remain repo-tracked

Instruction source pages intended as durable local orientation must live in
repo-tracked source paths, not hidden runtime stores.

## Minimal fixture shape

```yaml
instruction_index:
  id: instruction_index_fixture_001
  status: generated_view
  authority: non_authoritative
  source_pages:
    - ref: .punk/instructions/INDEX.md
      hash: sha256:fixture
  nodes:
    - id: instructions.index
      source_ref: .punk/instructions/INDEX.md
      source_hash: sha256:fixture
      title: Index
  generated_for_host: null
```

## Non-goals

This spec does not define `.punk/views` writer behavior, instruction generation,
host installation, skill packaging, provider-specific execution, module
activation, browser automation, gate writing, proofpack writing, or acceptance
claims.
