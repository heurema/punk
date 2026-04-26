---
id: docs_product_crate_status
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-26
review_after: 2026-07-20
canonical_for:
  - crate-status
  - workspace-status-vocabulary
  - active-core-crate-boundaries
  - parked-crate-boundaries
  - executor-agnostic-validation-crate-boundary
related_docs:
  - docs/product/START-HERE.md
  - docs/product/ROADMAP.md
  - docs/product/ARCHITECTURE.md
related_adrs:
  - docs/adr/ADR-0014-executor-agnostic-validation-boundary.md
supersedes: []
superseded_by: null
---
# Crate Status

## Status vocabulary

- `active-core` — required for the current stability target
- `incubating` — tested but not default/user-facing
- `parked` — boundary exists; minimal stub/docs only
- `retired` — removed or legacy-only

## Crate ownership target

| Crate | Status | Owns |
|---|---|---|
| `punk-cli` | active-core | CLI routing |
| `punk-domain` | active-core | canonical types |
| `punk-core` | active-core | deterministic helpers, validation, artifact hashing helpers |
| `punk-events` | active-core | append-only event log |
| `punk-flow` | active-core | state machine and transition guards |
| `punk-rules` | active-core | Punk Laws and rule snapshots |
| `punk-eval` | active-core | smoke/local eval harness |
| `punk-contract` | active-core | contract schema and validation |
| `punk-gate` | active-core | gate decision model and future decision writer |
| `punk-proof` | active-core | proofpack manifest, manifest self-digest metadata, artifact hash refs, and future proofpack writer |
| `punk-project` | active-core | project identity and memory roots |
| `punk-module-host` | parked | module manifest and hook routing |
| `punk-adapters` | parked | provider/tool wrappers |
| `punk-mod-dev` | parked | software-development module |
| `punk-mod-pub` | parked | content/publishing module |


## Current implemented subset boundary

The ownership table above names the target crate responsibility.
It is not a claim that every target behavior is active today.

Current implemented behavior remains narrower:

- `punk-cli` routes only the current implemented commands listed below.
- `punk-core` provides artifact digest and repo-relative artifact ref validation helpers, exact-byte artifact hash computation for artifact hash policy v0.1, a narrow file IO artifact hashing helper for one explicit regular file under an explicit repo root and validated repo-relative artifact ref, and a narrow evidence-only referenced artifact verification helper that compares a caller-provided canonical expected digest with that observed file digest. It owns a narrow `sha2` dependency for SHA-256 computation without exposing dependency types in public Punk APIs. It may read one explicit regular file for digest evidence through those helpers. It does not infer artifact refs, scan directories, silently follow symlinks, normalize artifact bytes or hashes, verify proofpack referenced artifact refs, write schemas, write proofpacks, write gate decisions, expose CLI behavior, create acceptance claims, or touch `.punk/` runtime state.
- `punk-rules` and `punk-project` are still minimal skeleton crates.
- `punk-events` provides an append-only event-log kernel and deterministic JSONL behavior, but `.punk/events` runtime storage is not active.
- `punk-flow` provides state-machine and guard evidence kernels, but no persisted runtime flow state.
- `punk-contract` provides a side-effect-free contract lifecycle kernel, but no `.punk/contracts` storage.
- `punk-domain` provides run receipt and validation evidence data models, but no `.punk/runs` writer.
- `punk-eval` provides the local smoke eval harness, including opt-in JSON output, artifact hash policy helper behavior coverage, exact-byte artifact hash computation helper coverage, file IO artifact hashing helper coverage, referenced artifact verification helper coverage, proofpack manifest digest helper coverage, proofpack writer operation evidence model coverage, proofpack writer preflight plan model coverage, proofpack writer file IO plan model coverage, proofpack writer file IO outcome model coverage, proofpack writer file IO error reason model coverage, proofpack writer target path policy model coverage, proofpack writer canonical artifact model coverage, and proofpack writer target artifact ref policy model coverage, but no `.punk/evals` report storage, baseline, waiver system, or runtime eval report writer.
- `punk-gate` provides a side-effect-free gate decision kernel, but no `.punk/decisions` writer, CLI behavior, runtime storage, or acceptance claim writer.
- `punk-proof` provides side-effect-free proofpack provenance, deterministic manifest rendering, proofpack manifest self-digest computation from in-memory renderer bytes through `punk-core` exact-byte hashing, digest metadata, structural link/hash integrity checks, proof readiness helpers, proof artifact hash string-shape validation through `punk-core` artifact hash policy helpers, a side-effect-free proofpack writer operation evidence model for planned, written, idempotent, conflict, preflight-failed, write-failed, partial-write, index-failed, latest-failed, and aborted outcomes, a side-effect-free proofpack writer preflight plan model for intended target refs, manifest self-digests, planned side effects, and missing preconditions, a side-effect-free proofpack writer file IO plan model for explicit storage-root refs, target artifact refs, target path refs, write policy, idempotency basis, temp/atomic policy, planned side effects, and error/rollback visibility, a side-effect-free proofpack writer file IO outcome model that maps explicit target, idempotency, temp/write, partial/cleanup, index/latest, and abort observations to operation evidence without claiming acceptance, a side-effect-free proofpack writer file IO error reason model for stable diagnostics over storage-root, target-path, existing-target, temp/write, flush/sync, atomic move, cleanup, index/latest, operation-evidence persistence, and abort failures, a side-effect-free proofpack writer target path policy model that classifies explicit target path refs and keeps storage-root refs, target artifact refs, target path refs, path injection, traversal, and storage-root escape diagnostics non-authoritative, a side-effect-free proofpack writer canonical artifact model that represents exact deterministic manifest renderer bytes, manifest self-digest coverage, and non-canonical metadata separation without writer side effects, and a side-effect-free proofpack writer target artifact ref policy model that requires `(proofpack_id, manifest_self_digest)` identity, renders `proofpack:<proofpack_id>@<manifest_self_digest>` as logical non-path metadata, and keeps target artifact refs separate from canonical bytes, target paths, storage roots, indexes, `latest` pointers, service mirrors, executor claims, and acceptance authority, but no `.punk/proofs` writer, referenced artifact hash computation, file IO hashing, byte/hash normalization, runtime storage, CLI behavior, or acceptance claim writer.

Future writer, storage, referenced artifact hash computation or verification for proofpack refs, actual proofpack file IO, broader referenced artifact verification beyond the narrow explicit-file helper, hash normalization, and proofpack writer hash-integration behavior must be added only through separate bounded goals.

## Current CLI surface

The implemented CLI surface is currently limited to:

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

`punk init` is a future setup target, not current CLI behavior.

## Research intake crate stance

Do not add a standalone `punk-telemetry` product crate yet.

Target telemetry ownership is distributed across active-core crates.
Current implemented behavior remains bounded by the section above.

- `punk-events`: event schema, append-only writer, replay support
- `punk-flow`: transition and guard events
- `punk-eval`: eval run events, smoke assessments, and helper-behavior coverage
- `punk-gate`: future final decision events
- `punk-proof`: proofpack manifest, manifest self-digest metadata, and artifact hash refs
- `punk-project`: project identity and memory-link boundaries

Future export adapters, dashboards, or remote sinks are parked and must not be listed as active-core.

## Executor-agnostic validation stance

Do not add an active-core executor, prompt manager, skill manager, provider runner, or agent runtime crate in the current phase.

The current active-core responsibility is:

- contract shape
- scope validation
- validator plan and deterministic helpers
- receipt/evidence shape
- eval report shape
- gate decision exclusivity
- proofpack links, manifest self-digest metadata, digest metadata, structural link/hash integrity, validation-only artifact hash policy shape checks, exact-byte artifact hash computation, narrow evidence-only file artifact digest computation, narrow evidence-only referenced artifact digest comparison for one explicit file, a side-effect-free proofpack writer operation evidence model, a side-effect-free proofpack writer preflight plan model, a side-effect-free proofpack writer file IO plan model, a side-effect-free proofpack writer file IO outcome model, a side-effect-free proofpack writer file IO error reason model, a side-effect-free proofpack writer target path policy model, and a side-effect-free proofpack writer canonical artifact model

This does not mean referenced artifact hash computation for proofpack refs,
proofpack-referenced artifact verification integration, broad referenced
artifact verification beyond explicit typed file refs, actual proofpack file IO,
broad file IO hashing, hash normalization, proofpack writing, proofpack writer
hash-integration, or gate decision writing.

Punk may run validators in the user's environment, but the user's executor, provider, prompt, skill, or local model setup must not become active-core authority.

Executor-specific behavior and semantic assessor command interfaces remain parked until the roadmap explicitly promotes DevPunk, adapters, or an assessor boundary.

## Parked module docs

Parked modules may exist as documentation folders before they become workspace crates.

Initial parked module docs:

- `docs/modules/pubpunk.md`

Future crate name when promoted:

- `punk-mod-pub`

## Parked adapter docs

Parked adapters may exist as documentation folders before they become active
workspace behavior.

Initial parked adapter docs:

- `docs/adapters/repo-search.md`

Future implementation location when promoted:

- `punk-adapters::repo_search`

Promotion requires a completed research note, receipt schema, conformance
evals, and proof that the adapter cannot own project truth or write final
decisions.

## Parked module host note

`punk-module-host` is not a plugin runtime while parked.

The parked boundary may document module manifests, hook types, permissions, and conformance expectations. It must not add a Wasm runtime dependency, plugin installer, plugin marketplace, or public `punk plugin` command until a later ADR promotes that scope.
