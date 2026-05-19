# PubPunk Conformance Packet v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the expected advisory result for applying Module Conformance Packet v0.1
and Module Host Contract Stub v0.1 to PubPunk before any PubPunk skeleton work.

This spec does not activate PubPunk runtime, Module Host runtime, module
loading, manifest parsing, workspace initialization, adapter invocation,
publishing, metrics collection, receipt writing, gate writing, proofpack
writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-CONFORMANCE-001: baseline refs are present

The PubPunk packet must cite Module Authoring Baseline, Module Conformance
Packet, Module Host Contract Stub, PubPunk module boundary, and their eval/spec
refs.

### PUBPUNK-CONFORMANCE-002: PubPunk remains parked

The packet must keep PubPunk parked or blocked for skeleton work unless all
implementation-readiness blockers are resolved.

### PUBPUNK-CONFORMANCE-003: workspace policy separates surfaces

The packet must distinguish repo-native public narrative artifacts under
`publishing/` from external project-specific operational publishing workspaces.

### PUBPUNK-CONFORMANCE-004: workspace initializer is not implied

The packet may identify workspace initialization as needed later, but it must
not imply active workspace creation behavior.

### PUBPUNK-CONFORMANCE-005: instruction refs are incomplete until selected

The packet must record missing module-specific instruction refs as an open
finding unless they already exist and are cited.

### PUBPUNK-CONFORMANCE-006: capabilities are denied by default

The packet must grant no active capabilities and must explicitly deny
filesystem read/write, network, environment, secrets, process/shell, adapter
invocation, external publishing, metrics collection, direct event-log writes,
final decision writes, and proofpack writes.

### PUBPUNK-CONFORMANCE-007: side effects are proposals only

Publishing, metrics collection, adapter invocation, credential reads, browser
behavior, comments, PRs, and deploys must remain denied or future request
proposals only.

### PUBPUNK-CONFORMANCE-008: outputs stay advisory

Expected PubPunk outputs may include module assessment, module receipt,
inventory gap note, draft/plan refs, and side-effect request proposals. They
must not be final decisions, proofpacks, project truth, or publication receipts.

### PUBPUNK-CONFORMANCE-009: implementation readiness is separate from runtime activation

The packet must separately report readiness for docs-only next work, skeleton
implementation, and runtime activation.

### PUBPUNK-CONFORMANCE-010: next step is smallest safe work

If the packet is not ready for skeleton work, it must name the smallest next
step that resolves one blocker without un-parking runtime behavior.

### PUBPUNK-CONFORMANCE-011: token accounting is honest

The packet report should include deliberation cost-accounting status when
available. Missing exact token usage must be recorded as unavailable or
unknown, not as zero.

## Expected advisory result

```yaml
pubpunk_conformance_result:
  status: blocked_by_findings
  ready_for_docs_only_next: true
  ready_for_skeleton: false
  ready_for_runtime_activation: false
  next_smallest_step: "Define PubPunk workspace and instruction packet v0.1."
  non_authority: true
```

## Non-goals

This spec does not define module code, host code, manifest schema,
deterministic checker implementation, module registry behavior, Module Host
runtime, PubPunk implementation, workspace creation, instruction generation,
adapter behavior, publishing behavior, metrics collection, gate writing,
proofpack writing, or acceptance claims.
