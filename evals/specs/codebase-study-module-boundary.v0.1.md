# Codebase Study module boundary v0.1

Date: 2026-06-08
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary expectations for the future Codebase Study Punk
module before any implementation exists.

This is a design/spec artifact only. It does not activate module execution,
Module Host runtime, repo scanning, file walking, source content reading,
source filesystem hashing, size collection, manifest generation from
repository state, source indexing, claim extraction, AI summaries, runtime
`.punk` storage, CLI behavior, lab code import, benchmark execution, gate
decisions, proof writing, or acceptance claims.

## Status and authority

Codebase Study is a future parked domain module.

Its future output is advisory observed structure only:

```text
packet_status = advisory
authority = observed_structure
```

It is not project truth, a Source Corpus Manifest, a claim ledger, a contract,
a gate decision, proof, acceptance, or benchmark authority.

## Required deterministic eval cases

### CODEBASE-STUDY-001: canonical module identity

A Codebase Study boundary must use:

```text
module_id = codebase-study
prose_name = Codebase Study
docs_path = docs/modules/codebase-study.md
```

Aliases such as `CodePunk`, `SourcePunk`, or `code` are not canonical in this
boundary.

### CODEBASE-STUDY-002: module-owned, not core-owned

Codebase study behavior must be modeled as domain module work.

A design that adds source observation behavior to Brownfield core,
`punk-project`, Source Corpus Manifest writer behavior, generic Punk `Writer`
behavior, or active core before Module Host promotion must fail.

### CODEBASE-STUDY-003: explicit source observation request required

A future invocation must require an explicit source observation request with
caller-supplied scope, refs, selected capability grants, privacy policy,
expected output authority, and downstream handoff intent.

Ambient repo discovery, hidden provider context, implicit current-working-
directory traversal, and local lab state are invalid inputs.

### CODEBASE-STUDY-004: advisory packet output only

A future invocation may return only an advisory source inventory observation
packet.

It must not return a Source Corpus Manifest, claim ledger, contract, final
decision, proof, acceptance, project truth, or benchmark authority.

### CODEBASE-STUDY-005: bounded explicit request can produce packet boundary

Given a bounded explicit source observation request, the module boundary may
describe readiness to produce an advisory observation packet.

It must preserve `packet_status = advisory` and
`authority = observed_structure`.

### CODEBASE-STUDY-006: auto-discovery request is rejected

Given a request such as "auto-discover all Rust files", the module boundary
must reject or defer it.

Autonomous discovery requires a later explicit traversal boundary before
implementation.

### CODEBASE-STUDY-007: content read request is rejected or deferred

Given a request to read file contents, source snippets, document excerpts, raw
environment values, or secret-like values, the module boundary must reject or
defer it.

Content reads require a later explicit privacy and capability boundary before
implementation.

### CODEBASE-STUDY-008: manifest assembly is rejected

Given a request to generate, assemble, or write a Source Corpus Manifest from
repository state, Codebase Study must reject or defer it.

The Source Corpus Manifest model track may consume accepted-for-handoff
advisory packets, but Codebase Study does not own manifest generation or writer
behavior.

### CODEBASE-STUDY-009: runtime storage writes are rejected

Given a request to cache observations, write `.punk` state, mutate event logs,
or persist module output, Codebase Study must reject or defer it.

Runtime storage requires a later bounded storage goal.

### CODEBASE-STUDY-010: authority claims are rejected

Given a request to approve contracts, write gate decisions, write proof,
claim acceptance, decide project truth, or promote benchmark results, Codebase
Study must reject it.

Only later host/gate/proof paths may assign those authorities.

### CODEBASE-STUDY-011: failure modes are explicit

Failure output must use blockers, limitations, missing evidence, refusal, or
advisory uncertainty.

Failure output must not become a reason to guess, widen scope, traverse
unrequested paths, read contents, cache results, assemble manifests, create
claims, or promote authority.

### CODEBASE-STUDY-012: lab and provider passes are advisory only

`code-intel-kernel`, `agent-bench-lab`, Claude, agy, and vibe passes may inform
boundary questions and eval requirements.

They must not become product truth, implementation authority, gate decisions,
proof, acceptance, or benchmark authority.

## Minimal fixture shape

This is illustrative boundary shape only, not an implemented schema.

```yaml
codebase_study_module_boundary:
  module_id: codebase-study
  prose_name: Codebase Study
  status: parked
  baseline_version: module-authoring-baseline.v0.1
  input_boundary: explicit_source_observation_request
  output_boundary: advisory_source_inventory_observation_packet
  denied_capabilities:
    - repo_scanning
    - file_walking
    - source_content_reading
    - source_filesystem_hashing
    - size_collection
    - manifest_generation_from_repository_state
    - runtime_punk_storage
    - gate_decision_writing
    - proof_writing
    - acceptance_claims
```

## Non-goals

This v0.1 boundary does not define or activate Codebase Study module execution,
Module Host runtime, module manifest schema, module SDK, plugin packaging, repo
scanning, file walking, source content reading, source filesystem hashing, size
collection, manifest generation from repository state, source indexing, claim
extraction, AI summaries, runtime `.punk` storage, CLI behavior, lab code
import, benchmark suite execution, gate/proof authority, or acceptance.
