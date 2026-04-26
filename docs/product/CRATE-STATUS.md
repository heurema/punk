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
- `punk-core` provides side-effect-free artifact digest and repo-relative artifact ref validation helpers plus exact-byte artifact hash computation for artifact hash policy v0.1. It owns a narrow `sha2` dependency for SHA-256 computation without exposing dependency types in public Punk APIs. It does not read files, infer artifact refs, normalize artifact bytes, write schemas, write proofpacks, write gate decisions, expose CLI behavior, or touch `.punk/` runtime state.
- `punk-rules` and `punk-project` are still minimal skeleton crates.
- `punk-events` provides an append-only event-log kernel and deterministic JSONL behavior, but `.punk/events` runtime storage is not active.
- `punk-flow` provides state-machine and guard evidence kernels, but no persisted runtime flow state.
- `punk-contract` provides a side-effect-free contract lifecycle kernel, but no `.punk/contracts` storage.
- `punk-domain` provides run receipt and validation evidence data models, but no `.punk/runs` writer.
- `punk-eval` provides the local smoke eval harness, including opt-in JSON output, artifact hash policy helper behavior coverage, exact-byte artifact hash computation helper coverage, and proofpack manifest digest helper coverage, but no `.punk/evals` report storage, baseline, waiver system, or runtime eval report writer.
- `punk-gate` provides a side-effect-free gate decision kernel, but no `.punk/decisions` writer, CLI behavior, runtime storage, or acceptance claim writer.
- `punk-proof` provides side-effect-free proofpack provenance, deterministic manifest rendering, proofpack manifest self-digest computation from in-memory renderer bytes through `punk-core` exact-byte hashing, digest metadata, structural link/hash integrity checks, proof readiness helpers, and proof artifact hash string-shape validation through `punk-core` artifact hash policy helpers, but no `.punk/proofs` writer, referenced artifact hash computation, file IO hashing, byte/hash normalization, runtime storage, CLI behavior, or acceptance claim writer.

Future writer, storage, referenced artifact hash computation or verification, file IO hashing, hash normalization, and proofpack writer hash-integration behavior must be added only through separate bounded goals.

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
- proofpack links, manifest self-digest metadata, digest metadata, structural link/hash integrity, validation-only artifact hash policy shape checks, and side-effect-free exact-byte artifact hash computation

This does not mean referenced artifact hash computation, referenced artifact
byte verification, file IO hashing, hash normalization, proofpack writing,
proofpack writer hash-integration, or gate decision writing.

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
