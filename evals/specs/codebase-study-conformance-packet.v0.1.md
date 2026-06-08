# Codebase Study conformance packet v0.1

Date: 2026-06-08
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the advisory Codebase Study Module
Conformance Packet before any implementation exists.

This spec does not activate module execution, Module Host runtime, repo
scanning, file walking, source content reading, source filesystem hashing, size
collection, manifest generation from repository state, source indexing, claim
extraction, AI summaries, runtime `.punk` storage, CLI behavior, lab code
import, benchmark execution, gate decisions, proof writing, or acceptance
claims.

## Status and authority

The Codebase Study conformance packet is advisory readiness evidence only.

It must not promote Codebase Study to active status, skeleton readiness,
runtime activation, source-observation authority, project truth, benchmark
authority, gate decision, proof, or acceptance.

## Required deterministic eval cases

### CODEBASE-STUDY-CONFORMANCE-001: packet cites baseline refs

A Codebase Study conformance packet must cite:

- `docs/product/MODULE-AUTHORING.md`;
- `docs/product/MODULE-CONFORMANCE.md`;
- `docs/product/MODULE-HOST-CONTRACT.md`;
- `evals/specs/module-authoring-baseline.v0.1.md`;
- `evals/specs/module-conformance-packet.v0.1.md`;
- `evals/specs/module-host-contract-stub.v0.1.md`.

### CODEBASE-STUDY-CONFORMANCE-002: module identity is explicit

A packet must use:

```text
module_id = codebase-study
module_ref = docs/modules/codebase-study.md
lifecycle_target = parked
```

It must not use `CodePunk`, `SourcePunk`, or `code` as canonical module ids.

### CODEBASE-STUDY-CONFORMANCE-003: authority is non-authoritative

A packet must declare advisory/non-authoritative status and must not claim
gate, proof, acceptance, benchmark, project-truth, or final-decision authority.

### CODEBASE-STUDY-CONFORMANCE-004: lifecycle cannot self-promote

A packet may recommend docs-only follow-up work. It must not mark Codebase Study
ready for skeleton implementation, runtime activation, or active source
observation while source request, capability, privacy, evidence, receipt, and
evaluation-route findings remain open.

### CODEBASE-STUDY-CONFORMANCE-005: workspace policy blocks execution

A packet must record that no execution workspace, source workspace, or runtime
storage path is selected for Codebase Study.

It may cite repo-tracked docs as source-of-truth refs for this conformance
review, but must not treat the current checkout, ambient current working
directory, local lab state, or `.punk` state as execution input.

### CODEBASE-STUDY-CONFORMANCE-006: instruction refs are explicit

A packet must cite the thin instruction/source refs used for the conformance
review and mark generated views as deferred when absent.

Missing module-specific runner aids must be findings, not hidden context.

### CODEBASE-STUDY-CONFORMANCE-007: deliberation limitations are explicit

A packet must record Deliberation Budget class, source refs, and limitations.

Prior lab/provider passes may inform the packet only as advisory inputs. Missing
new provider passes must not be represented as provider diversity.

### CODEBASE-STUDY-CONFORMANCE-008: capabilities are deny-by-default

A packet must keep grants empty and deny source-observation capabilities unless
a later bounded goal selects them.

Denied capabilities must include repo scanning, file walking, source content
reading, source filesystem hashing, size collection, manifest generation from
repository state, runtime `.punk` storage, module execution, direct event-log
mutation, final decision writing, proof writing, and acceptance claims.

Future capability requests must remain requests, not grants.

### CODEBASE-STUDY-CONFORMANCE-009: privacy and redaction are blockers

A packet must record that content reads, raw snippets, raw environment values,
secret-like values, and sensitive source observations remain blocked until a
later privacy/redaction boundary is selected.

### CODEBASE-STUDY-CONFORMANCE-010: host contract check is scoped

A packet must include a Module Host Contract Stub check with explicit
`allowed_source_refs`, empty capability grants, explicit denies, and advisory
allowed outputs.

The host contract check must not imply Module Host runtime, module invocation,
capability enforcement, event writing, or receipt writing.

### CODEBASE-STUDY-CONFORMANCE-011: outputs remain advisory

A packet's expected outputs must remain advisory module assessments, module
receipts, conformance findings, runner aids, or side-effect request proposals.

It must not output a Source Corpus Manifest, claim ledger, contract, gate
decision, proof, acceptance, benchmark authority, or project truth.

### CODEBASE-STUDY-CONFORMANCE-012: agent-bench-lab route is unselected

A packet must record that future active Codebase Study results need an
`agent-bench-lab` evaluation route before influencing Brownfield decisions.

The packet must not run `agent-bench-lab`, import lab code, or treat benchmark
results as gate, proof, acceptance, project truth, or implementation authority.

### CODEBASE-STUDY-CONFORMANCE-013: readiness is split

A packet must distinguish:

- `ready_for_docs_only_next`;
- `ready_for_skeleton`;
- `ready_for_runtime_activation`.

For this v0.1 packet, docs-only next work may be ready while skeleton and
runtime activation remain false.

### CODEBASE-STUDY-CONFORMANCE-014: next smallest step is explicit

A packet must name the next smallest bounded step.

The next step must not be scanner implementation, file walking, content
reading, source hashing, manifest generation, runtime storage, CLI behavior,
module execution, lab execution, gate/proof behavior, or acceptance.

### CODEBASE-STUDY-CONFORMANCE-015: packet is not runtime

The packet must not imply a parser, checker, module registry, Module Host
activation, CLI command, provider orchestration, adapter invocation, workspace
initializer, source observer, source inventory generator, benchmark runner,
gate writer, proof writer, or acceptance claim.

## Minimal fixture shape

This is illustrative boundary shape only, not an implemented schema.

```yaml
codebase_study_conformance_result:
  status: blocked_by_findings
  module_id: codebase-study
  ready_for_docs_only_next: true
  ready_for_skeleton: false
  ready_for_runtime_activation: false
  next_smallest_step: "Define Codebase Study source observation request packet v0.1."
  non_authority: true
```

## Non-goals

This spec does not define module code, manifest parsing, packet parsing,
deterministic checker implementation, module registry behavior, Module Host
runtime, Codebase Study implementation, workspace creation, instruction
generation, adapter behavior, repo scanning, file walking, content reading,
source filesystem hashing, size collection, source inventory generation,
Source Corpus Manifest generation, runtime `.punk` storage, lab execution,
benchmark execution, gate writing, proof writing, or acceptance claims.
