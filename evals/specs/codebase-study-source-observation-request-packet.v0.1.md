# Codebase Study source observation request packet v0.1

Date: 2026-06-08
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary expectations for the future Codebase Study source
observation request packet.

This spec defines request shape only. It does not activate module execution,
Module Host runtime, request parsing, repo scanning, file walking, source
content reading, source filesystem hashing, size collection, manifest
generation from repository state, source indexing, claim extraction, AI
summaries, runtime `.punk` storage, CLI behavior, lab code import, benchmark
execution, gate decisions, proof writing, or acceptance claims.

## Status and authority

A source observation request packet is non-authoritative input intent.

Required authority:

```text
packet_status = requested
authority = request_only
non_authority = true
```

The packet may describe requested capabilities and downstream handoff intent.
It must not grant capabilities, execute Codebase Study, read files, write
storage, produce an observation packet, generate a manifest, decide project
truth, run benchmarks, write gates or proof, or claim acceptance.

## Required deterministic eval cases

### CODEBASE-STUDY-REQUEST-001: packet identity is explicit

A request packet must include:

```text
schema_version = codebase-study-source-observation-request-packet.v0.1
module_id = codebase-study
packet_status = requested
authority = request_only
non_authority = true
```

Aliases such as `CodePunk`, `SourcePunk`, or `code` must not be used as
canonical module ids.

### CODEBASE-STUDY-REQUEST-002: caller-supplied refs are required

A request packet must include explicit caller-supplied source scope refs and
exclude refs.

The packet must not rely on ambient repository discovery, hidden provider
context, local lab state, or implicit current-working-directory traversal.

### CODEBASE-STUDY-REQUEST-003: request is not a capability grant

A request packet may include capability requests. It must keep selected
capability grants empty:

```yaml
capability_requests: [...]
capability_grants: []
```

Capability requests must not be interpreted as permission to scan, traverse,
read contents, hash files, collect sizes, import lab code, run benchmarks, or
write storage.

### CODEBASE-STUDY-REQUEST-004: observation categories are bounded

A request packet may ask for future structural observation categories such as
path existence, path kind candidate, basename or extension marker, source class
candidate, generated/vendored candidate, warning, blocker, or limitation.

It must not ask Codebase Study to infer user intent, requirements, module
purpose, architecture decisions, accepted behavior, contract readiness, project
truth, gate status, proof status, or acceptance.

### CODEBASE-STUDY-REQUEST-005: privacy and redaction refs are required or blocked

A request packet must include privacy policy refs and redaction policy refs, or
record explicit blockers explaining why execution cannot proceed.

Requests that ask for source snippets, document excerpts, raw environment
values, secret-like values, private transcripts, credentials, or sensitive
content must be rejected or blocked until a later privacy/redaction boundary is
selected.

### CODEBASE-STUDY-REQUEST-006: downstream handoff intent is explicit

A request packet must declare downstream handoff intent, such as advisory
source inventory observation packet preparation.

It must not request a Source Corpus Manifest, manifest writer behavior,
runtime `.punk` write, claim ledger, contract, gate decision, proof, acceptance,
or benchmark authority.

### CODEBASE-STUDY-REQUEST-007: output authority remains advisory

Expected output authority must remain:

```text
expected_output = advisory_source_inventory_observation_packet
expected_packet_status = advisory
expected_authority = observed_structure
```

The request must not ask for project truth, final decisions, proof, acceptance,
or benchmark authority.

### CODEBASE-STUDY-REQUEST-008: auto-discovery is invalid

Given a request such as "auto-discover all Rust files", the request packet must
be rejected or blocked.

Autonomous discovery requires a later explicit traversal boundary and selected
capability grants before implementation.

### CODEBASE-STUDY-REQUEST-009: content read is invalid

Given a request to read file contents, summarize code, extract snippets, or
inspect raw values, the request packet must be rejected or blocked.

Content reads require a later explicit privacy, redaction, and capability
boundary before implementation.

### CODEBASE-STUDY-REQUEST-010: hashing and size collection are invalid

Given a request to compute source filesystem hashes or collect file sizes, the
request packet must reject or block those asks.

Hashes and sizes require a later bounded policy and capability boundary.

### CODEBASE-STUDY-REQUEST-011: manifest generation is invalid

Given a request to generate, assemble, or write a Source Corpus Manifest from
repository state, the request packet must reject or block it.

The Source Corpus Manifest model track may consume accepted-for-handoff
advisory observation packets later. The request packet does not create that
handoff, run Codebase Study, or activate writer behavior.

### CODEBASE-STUDY-REQUEST-012: runtime storage writes are invalid

Given a request to cache observations, write `.punk` state, mutate event logs,
or persist module output, the request packet must reject or block it.

Runtime storage requires a later bounded storage goal.

### CODEBASE-STUDY-REQUEST-013: lab execution and benchmark authority are invalid

Given a request to run `code-intel-kernel`, run `agent-bench-lab`, import lab
code, or treat benchmark output as authority, the request packet must reject or
block it.

Future active Codebase Study results need an `agent-bench-lab` evaluation route
before influencing Brownfield decisions, but this packet does not run that
route.

### CODEBASE-STUDY-REQUEST-014: gate proof and acceptance claims are invalid

Given a request to approve contracts, write gate decisions, write proof, claim
acceptance, decide project truth, or promote benchmark results, the request
packet must reject it.

### CODEBASE-STUDY-REQUEST-015: blockers are explicit

Invalid or incomplete request packets must expose blockers, limitations,
missing evidence, refusal, or advisory uncertainty.

Failure output must not become a reason to guess, widen scope, traverse
unrequested paths, read contents, cache results, assemble manifests, create
claims, or promote authority.

## Minimal fixture shape

This is illustrative boundary shape only, not an implemented schema.

```yaml
codebase_study_source_observation_request_packet:
  schema_version: codebase-study-source-observation-request-packet.v0.1
  packet_status: requested
  authority: request_only
  module_id: codebase-study
  requested_by_ref: work/goals/example.md
  operation:
    kind: source_observation_request
  source_scope:
    mode: explicit_refs_only
    include_refs:
      - docs/modules/codebase-study.md
    exclude_refs:
      - .punk/**
  requested_observation_categories:
    - path_exists
    - path_kind_candidate
    - source_class_candidate
    - warning
    - blocker
    - limitation
  capability_requests:
    - scoped_path_observation_over_explicit_refs
  capability_grants: []
  privacy_policy_ref: null
  redaction_policy_ref: null
  privacy_blockers:
    - privacy_policy_not_selected
    - redaction_policy_not_selected
  expected_output:
    kind: advisory_source_inventory_observation_packet
    packet_status: advisory
    authority: observed_structure
  downstream_handoff_intent:
    target: brownfield_observation_packet_boundary
    source_corpus_manifest_writer: not_requested
  evaluation_requirement_refs:
    - agent_bench_lab_route_unselected
  blockers:
    - capability_grants_not_selected
    - privacy_policy_not_selected
    - redaction_policy_not_selected
  non_authority: true
```

## Non-goals

This v0.1 boundary does not define or activate request parsing, Codebase Study
module execution, Module Host runtime, module manifest schema, module SDK,
plugin packaging, repo scanning, file walking, source content reading, source
filesystem hashing, size collection, manifest generation from repository
state, source indexing, claim extraction, AI summaries, runtime `.punk`
storage, CLI behavior, lab code import, benchmark suite execution, gate/proof
authority, or acceptance.
