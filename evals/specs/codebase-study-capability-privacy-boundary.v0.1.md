# Codebase Study capability and privacy boundary v0.1

Date: 2026-06-08
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary expectations for Codebase Study capability
requests, capability grants, privacy blockers, and redaction blockers.

This spec is boundary evidence only. It does not activate module execution,
Module Host runtime, capability enforcement, privacy enforcement, redaction
enforcement, request parsing, repo scanning, file walking, source content
reading, source filesystem hashing, size collection, manifest generation from
repository state, source indexing, claim extraction, AI summaries, runtime
`.punk` storage, CLI behavior, lab code import, benchmark execution, gate
decisions, proof writing, or acceptance claims.

## Status and authority

The capability/privacy boundary is non-authoritative design evidence.

Required status:

```text
boundary_status = proposed
authority = non_authoritative
non_authority = true
```

Capability requests may be represented. Capability grants must remain empty in
this boundary.

## Required deterministic eval cases

### CODEBASE-STUDY-CAPABILITY-001: boundary identity is explicit

A capability/privacy boundary packet must include:

```text
schema_version = codebase-study-capability-privacy-boundary.v0.1
module_id = codebase-study
boundary_status = proposed
authority = non_authoritative
non_authority = true
```

Aliases such as `CodePunk`, `SourcePunk`, or `code` must not be used as
canonical module ids.

### CODEBASE-STUDY-CAPABILITY-002: capability requests are not grants

The boundary may list requested capabilities, but it must keep selected grants
empty:

```yaml
capability_requests: [...]
capability_grants: []
selected_capability_grants: []
```

Requested capabilities must not be interpreted as approval to scan, traverse,
read contents, hash files, collect sizes, import lab code, run benchmarks,
write storage, or make authority claims.

### CODEBASE-STUDY-CAPABILITY-003: no grants are selected by default

The v0.1 boundary must declare that no source-observation capability grants are
selected.

Any output that treats path observation, traversal, content reading, hashing,
size collection, manifest generation, storage, lab execution, benchmark
authority, gate decisions, proof, or acceptance as granted must fail.

### CODEBASE-STUDY-CAPABILITY-004: future grant prerequisites are explicit

A future capability grant must require a later bounded goal, explicit
caller-supplied source refs, selected grant refs, privacy and redaction refs or
resolved blockers, receipt/evidence refs, evaluation-route refs, conformance
review, and deterministic eval coverage.

The boundary must not allow an implicit grant from natural-language request
text, current working directory, provider prompt state, local lab state, or
module name.

### CODEBASE-STUDY-CAPABILITY-005: explicit refs only

The boundary must require caller-supplied refs for any future structural source
observation request.

Ambient repository discovery, implicit current-working-directory traversal, and
hidden provider context must be rejected or blocked.

### CODEBASE-STUDY-CAPABILITY-006: structural categories are requestable only

The boundary may allow future requests for structural advisory categories such
as path existence candidate, path kind candidate, basename marker, extension
marker, source-class candidate, generated candidate, vendored candidate,
warning, blocker, or limitation.

Those categories remain requests only. They must not grant scanner, file
walker, reader, indexer, manifest builder, claim extractor, AI summarizer,
runtime storage, or authority behavior.

### CODEBASE-STUDY-CAPABILITY-007: traversal and auto-discovery are denied

Given a request to auto-discover files, walk directories, scan the repository,
expand a glob from ambient state, or enumerate all Rust files, the boundary
must reject or block it.

Traversal requires a later explicit traversal capability boundary and selected
grants before implementation.

### CODEBASE-STUDY-CAPABILITY-008: content and snippets are denied

Given a request to read file contents, summarize code, extract snippets, include
document excerpts, inspect raw values, or infer source intent from contents, the
boundary must reject or block it.

Content reads require a later explicit content-read, privacy, redaction, and
capability boundary before implementation.

### CODEBASE-STUDY-CAPABILITY-009: raw env and secrets are denied

Given a request to inspect raw environment variables, credentials,
secret-like values, private transcripts, or sensitive content, the boundary
must reject or block it.

Privacy and redaction blockers must fail closed.

### CODEBASE-STUDY-CAPABILITY-010: hashing and size collection are denied

Given a request to compute source filesystem hashes, digest source files, or
collect file sizes, the boundary must reject or block it.

Hashes and sizes require a later bounded policy and capability boundary.

### CODEBASE-STUDY-CAPABILITY-011: manifest assembly and storage are denied

Given a request to assemble a Source Corpus Manifest, generate manifest items,
cache observations, write `.punk` state, or mutate event logs, the boundary
must reject or block it.

Observation handoff, manifest assembly, and storage require later separate
boundaries.

### CODEBASE-STUDY-CAPABILITY-012: lab and benchmark authority are denied

Given a request to run `code-intel-kernel`, run `agent-bench-lab`, import lab
code, execute provider passes, or treat benchmark results as authority, the
boundary must reject or block it.

Future active Codebase Study results need an `agent-bench-lab` evaluation route
before influencing Brownfield decisions, but this boundary does not run or
authorize that route.

### CODEBASE-STUDY-CAPABILITY-013: gate proof and acceptance are denied

Given a request to decide project truth, approve contracts, write gate
decisions, write proof, claim acceptance, or promote benchmark results, the
boundary must reject it.

Only later gate/proof paths may make final acceptance claims.

### CODEBASE-STUDY-CAPABILITY-014: privacy and redaction blockers are explicit

Missing privacy policy refs, missing redaction policy refs, unresolved
sensitivity blockers, or requests for raw sensitive values must be represented
as blockers, limitations, refusal, or missing evidence.

Failure output must not become a reason to guess, widen scope, traverse
unrequested paths, read contents, cache results, assemble manifests, create
claims, or promote authority.

### CODEBASE-STUDY-CAPABILITY-015: next smallest step is explicit

The boundary must select a next docs/eval-only checkpoint.

The next step must not be scanner implementation, file walking, content
reading, source hashing, size collection, manifest generation, runtime storage,
CLI behavior, module execution, lab execution, gate/proof behavior, or
acceptance.

## Minimal fixture shape

This is illustrative boundary shape only, not an implemented schema.

```yaml
codebase_study_capability_privacy_boundary:
  schema_version: codebase-study-capability-privacy-boundary.v0.1
  boundary_status: proposed
  authority: non_authoritative
  module_id: codebase-study
  capability_requests:
    - scoped_path_observation_over_explicit_refs
  capability_grants: []
  selected_capability_grants: []
  denied_by_default:
    - ambient_repo_discovery
    - implicit_cwd_traversal
    - repo_scanning
    - file_walking
    - source_content_reading
    - source_snippets
    - raw_environment_values
    - secret_like_values
    - source_filesystem_hashing
    - size_collection
    - manifest_generation_from_repository_state
    - runtime_punk_storage
    - lab_execution
    - benchmark_authority
    - gate_decisions
    - proof_writing
    - acceptance_claims
  future_requestable_only:
    - path_exists_candidate
    - path_kind_candidate
    - basename_marker
    - extension_marker
    - source_class_candidate
    - warning
    - blocker
    - limitation
  future_grant_prerequisites:
    - later_bounded_goal
    - caller_supplied_source_refs
    - selected_capability_grant_refs
    - privacy_policy_ref_or_resolved_blocker
    - redaction_policy_ref_or_resolved_blocker
    - receipt_evidence_refs
    - evaluation_route_refs
    - conformance_review
    - deterministic_eval_coverage
  privacy_blockers:
    - privacy_policy_not_selected
    - redaction_policy_not_selected
  next_smallest_step: define_codebase_study_observation_receipt_evidence_boundary_v0_1
  non_authority: true
```

## Non-goals

This v0.1 boundary does not define or activate request parsing, capability
resolution, privacy enforcement, redaction enforcement, Codebase Study module
execution, Module Host runtime, module manifest schema, module SDK, plugin
packaging, repo scanning, file walking, source content reading, source
filesystem hashing, size collection, manifest generation from repository
state, source indexing, claim extraction, AI summaries, runtime `.punk`
storage, CLI behavior, lab code import, benchmark suite execution, gate/proof
authority, or acceptance.
