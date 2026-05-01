---
id: docs_product_crate_status
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-05-01
review_after: 2026-07-20
canonical_for:
  - crate-status
  - workspace-status-vocabulary
  - active-core-crate-boundaries
  - parked-crate-boundaries
  - executor-agnostic-validation-crate-boundary
related_docs:
  - docs/product/START-HERE.md
  - docs/product/CONTRACT-SCHEMA.md
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
- `punk-rules` is still a minimal skeleton crate.
- `punk-project` provides a greenfield Level 0 manual project-memory init scaffold that requires a slug-safe `project_id`, records `entry_mode = greenfield`, writes repo-tracked `work/STATUS.md`, an initial ready goal, `work/reports/`, `docs/adr/`, `knowledge/`, `.punk/README.md`, and `.punk/project.toml` starter files with create-new/no-overwrite behavior. It reports conflicts fail-closed and creates `.punk` only as a project root marker, without brownfield reconstruction, grayfield reconciliation, repo scanning, network behavior, `.punk/events`, `.punk/contracts`, `.punk/runs`, `.punk/evals`, `.punk/decisions`, `.punk/proofs`, flow persistence, contracts, run receipts, gate artifacts, proofpacks, or acceptance claims.
- `punk-events` provides an append-only event-log kernel and deterministic JSONL behavior, but `.punk/events` runtime storage is not active.
- `punk-flow` provides state-machine and guard evidence kernels, but no persisted runtime flow state.
- `punk-contract` provides a side-effect-free contract lifecycle kernel, side-effect-free Contract Context Pack boundary model/validation helpers, a side-effect-free user intent-to-contract draft model that classifies raw request, intent, scope, acceptance criteria, evidence plan, research refs, downstream closure expectations, clarification/refusal, and readiness for user confirmation before execution, a side-effect-free Contract Schema Blueprint v0.1 model that preserves identity, authority, lifecycle, work, boundaries, clauses, validation, evidence, receipt requirements, gate policy, proof requirements, change control, memory links, and the `required_now`/`deferred`/`parked`/`future` field split, a side-effect-free contract draft confirmation boundary that requires explicit user confirmation before producing approved-for-run model state from a ready draft while blocking clarification-required, refused/deferred, blocked, or unresolved-unknown drafts, a side-effect-free hard-clause mapping model that requires hard clauses to map to validator refs, required receipt fields, proof requirement refs, or explicit human gate review with reason before approved-for-run model state while unsupported or invalid hard clauses block approval, a side-effect-free contract receipt requirements model that connects hard-clause required receipt fields to declared future run receipt fields and blocks approved-for-run model state when required fields are missing or unsupported, a side-effect-free contract gate input policy model that declares future gate inputs and separates `approved_for_run` from `ready_for_gate` without requiring an existing proofpack, and a side-effect-free contract proof requirements model that declares future proofpack links and hashes after gate outcome without creating proofpacks, computing artifact hashes, writing gate outcomes, or creating acceptance claims. It has no `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime contract writer, runtime receipt writer, runtime gate writer, runtime proofpack writer, artifact hash computation from filesystem, context-pack writer, retrieval integration, executor brief generator, CLI behavior, Writer activation, runtime validator execution, gate decision writer, proofpack writer, acceptance claim writer, or gate/proof authority.
- `punk-domain` provides run receipt and validation evidence data models, but no `.punk/runs` writer.
- `punk-eval` provides the local smoke eval harness, including opt-in JSON output, Level 0 project init scaffold coverage, artifact hash policy helper behavior coverage, exact-byte artifact hash computation helper coverage, file IO artifact hashing helper coverage, referenced artifact verification helper coverage, proofpack manifest digest helper coverage, proofpack writer operation evidence model coverage, proofpack writer preflight plan model coverage, proofpack writer file IO plan model coverage, proofpack writer file IO outcome model coverage, proofpack writer file IO error reason model coverage, proofpack writer target path policy model coverage, proofpack writer canonical artifact model coverage, proofpack writer target artifact ref policy model coverage, proofpack writer preflight integration model coverage, proofpack writer active behavior model coverage, proofpack writer host path resolution model coverage, proofpack writer concrete path/storage policy model coverage, proofpack writer first active write slice exact-byte smoke coverage, proofpack writer hash/reference integration model coverage, user intent-to-contract draft model smoke coverage, Contract Schema Blueprint v0.1 boundary coverage, contract draft confirmation boundary smoke coverage, hard-clause mapping smoke coverage, contract receipt requirements smoke coverage, contract gate input policy smoke coverage, contract proof requirements smoke coverage, and target-ref alignment coverage for logical artifact refs versus path-policy refs, but no `.punk/evals` report storage, baseline, waiver system, or runtime eval report writer.
- `punk-gate` provides a side-effect-free gate decision kernel, but no `.punk/decisions` writer, CLI behavior, runtime storage, or acceptance claim writer.
- `punk-proof` provides side-effect-free proofpack provenance, deterministic manifest rendering, proofpack manifest self-digest computation from in-memory renderer bytes through `punk-core` exact-byte hashing, digest metadata, structural link/hash integrity checks, proof readiness helpers, proof artifact hash string-shape validation through `punk-core` artifact hash policy helpers, a side-effect-free proofpack writer operation evidence model for planned, written, idempotent, conflict, preflight-failed, write-failed, partial-write, index-failed, latest-failed, and aborted outcomes, a side-effect-free proofpack writer preflight plan model for logical target artifact refs derived from the target artifact ref policy model, manifest self-digests, planned side effects, and missing preconditions, a side-effect-free proofpack writer file IO plan model for explicit storage-root refs, logical target artifact refs, target path refs, write policy, idempotency basis, temp/atomic policy, planned side effects, and error/rollback visibility, a side-effect-free proofpack writer file IO outcome model that maps explicit target, idempotency, temp/write, partial/cleanup, index/latest, and abort observations to operation evidence without claiming acceptance, a side-effect-free proofpack writer file IO error reason model for stable diagnostics over storage-root, target-path, existing-target, temp/write, flush/sync, atomic move, cleanup, index/latest, operation-evidence persistence, and abort failures, a side-effect-free proofpack writer target path policy model that classifies explicit target path refs and keeps storage-root refs, logical target artifact refs, target path refs, path injection, traversal, and storage-root escape diagnostics non-authoritative, a side-effect-free proofpack writer canonical artifact model that represents exact deterministic manifest renderer bytes, manifest self-digest coverage, and non-canonical metadata separation without writer side effects, a side-effect-free proofpack writer target artifact ref policy model that requires `(proofpack_id, manifest_self_digest)` identity, renders `proofpack:<proofpack_id>@<manifest_self_digest>` as logical non-path metadata, lets writer target refs be derived from accepted policy-model inputs, and keeps target artifact refs separate from canonical bytes, target paths, storage roots, indexes, `latest` pointers, service mirrors, executor claims, and acceptance authority, and a side-effect-free proofpack writer preflight integration model that composes explicit proofpack, canonical artifact, target artifact ref policy, preflight plan, file IO plan, and target path policy inputs into ready/blocked/not-selected evidence while keeping blockers fail-closed and storage root refs, logical target artifact refs, and target path refs distinct, and a side-effect-free proofpack writer active behavior model that requires explicit writer-ready preflight, consumes explicit observation data, tracks selected/attempted/completed/failed side effects, models planned-only, preflight-failed, written, idempotent, conflict, write-failed, partial-write, index-failed, latest-failed, and aborted outcomes, and keeps operation-evidence persistence failure visible without persisting evidence, a side-effect-free proofpack writer host path resolution model that requires explicit storage root refs, logical target artifact refs, target path refs, selected path policy refs, redacted host path observations, separated refs, and fail-closed blockers without filesystem resolution, a side-effect-free proofpack writer concrete path/storage policy model that requires explicit storage root refs, logical target artifact refs, target path refs, selected storage/path policy refs, host path observations, redaction state, idempotency/conflict policy, temp/atomic policy, and index/latest non-authority policy before reporting writer-ready policy evidence, a minimal first active proofpack writer write slice that writes exact canonical artifact bytes only to one explicit caller-provided storage root path plus explicit target-relative path when writer preflight and concrete path/storage policy are ready, uses create-new/no-overwrite behavior, reports in-memory non-authoritative outcome evidence for written, already-existing matching, conflict, preflight-failed, write-failed, and partial/ambiguous outcomes, and fails closed for unsafe or unready inputs, and a side-effect-free proofpack writer hash/reference integration model that composes explicit proofpack refs, declared artifact digest evidence, structural link/hash integrity, optional referenced artifact verification outcomes, and manifest self-digest readiness while keeping blockers fail-closed and evidence surfaces separate without reading files or writing artifacts. It still has no broader active proofpack writer orchestration, `.punk/proofs` writer, referenced artifact hash computation, active proofpack referenced artifact file verification, broad file IO hashing, host filesystem path resolution/canonicalization, byte/hash normalization, runtime storage, CLI behavior, schema writer, persisted operation-evidence writer, gate decision writer, or acceptance claim writer.

Future broader writer implementation, storage, referenced artifact hash computation or active verification for proofpack refs, runtime proofpack file IO, active host path resolution/canonicalization, broader referenced artifact verification beyond the narrow explicit-file helper, hash normalization, operation-evidence persistence, and runtime proofpack hash/reference authority must be added only through separate bounded goals.

The proofpack writer preflight integration, active behavior, host path resolution, concrete path/storage policy, first active write slice, and hash-policy integration boundaries remain docs/spec authorities for future writer behavior. Current crate behavior includes side-effect-free preflight integration, active behavior, host path resolution, concrete path/storage policy, and hash/reference integration models plus the first narrow active exact-byte write slice for explicit caller-provided test targets; it does not add `.punk/proofs`, runtime storage, schema files, CLI behavior, parent directory creation, persisted operation-evidence writing, indexes, latest pointers, active proofpack referenced artifact file verification, active host path resolution/canonicalization, gate decisions, or acceptance claims.

## Current CLI surface

The implemented CLI surface is currently limited to:

- `punk init <project-id>`
- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

`punk init <project-id>` is current CLI behavior only for greenfield Dogfooding Level 0 manual project-memory scaffolding.
It records `project_id` and `entry_mode = greenfield` and creates `.punk/README.md` and `.punk/project.toml` as marker/setup files, but does not activate brownfield reconstruction, grayfield reconciliation, `.punk/` runtime storage, flow/event persistence, contract writing, receipt writing, gate writing, proof writing, or proofpack writing.

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
- Contract Schema Blueprint v0.1 shape and `required_now`/`deferred`/`parked`/`future` split
- user intent-to-contract draft model shape and readiness classification
- contract draft confirmation boundary shape for explicit confirmation into approved-for-run model state without gate/proof/Writer authority
- hard-clause mapping shape for validator refs, required receipt fields, proof requirement refs, and explicit human gate review with reason, where unsupported hard clauses block approved-for-run model state without runtime validator/gate/proof/Writer authority
- contract receipt requirements shape for future run receipt fields, where hard-clause receipt-field mappings must be covered before approved-for-run model state and missing or unsupported fields block without runtime receipt writing, `.punk/runs` storage, validator execution, gate/proof writer authority, or Writer activation
- contract gate input policy shape for future gate inputs, where `ready_for_gate` requires contract refs, `approved_for_run` status, run receipt refs, receipt requirement coverage, hard-clause mapping assessment, eval/assessment refs, scope/deviation status, and executor claim status without proofpack-as-input, runtime gate writing, `.punk/decisions` storage, `.punk/proofs` storage, proofpack creation, acceptance claims, validator execution, or Writer activation
- contract proof requirements shape for future proofpack links/hashes after gate outcome, where required targets include contract refs, run receipt refs, gate outcome refs, eval report refs/hashes when applicable, and output artifact refs/hashes when expected, without proofpack writing, artifact hash runtime, `.punk/proofs` storage, gate-writing behavior, acceptance claims, or Writer activation
- scope validation
- Contract Context Pack boundary shape and deterministic validation of no orphan context, material clause coverage, bidirectional support/coverage mapping, speculative-source exclusion, and advisory-only authority
- validator plan and deterministic helpers
- receipt/evidence shape
- eval report shape
- gate decision exclusivity
- proofpack links, manifest self-digest metadata, digest metadata, structural link/hash integrity, validation-only artifact hash policy shape checks, exact-byte artifact hash computation, narrow evidence-only file artifact digest computation, narrow evidence-only referenced artifact digest comparison for one explicit file, a side-effect-free proofpack writer operation evidence model, a side-effect-free proofpack writer preflight plan model, a side-effect-free proofpack writer file IO plan model, a side-effect-free proofpack writer file IO outcome model, a side-effect-free proofpack writer file IO error reason model, a side-effect-free proofpack writer target path policy model, a side-effect-free proofpack writer canonical artifact model, a side-effect-free proofpack writer target artifact ref policy model/alignment helper, a docs/spec proofpack writer preflight integration boundary, a side-effect-free proofpack writer preflight integration model, a docs/spec proofpack writer active behavior boundary, a side-effect-free proofpack writer active behavior model, a docs/spec proofpack writer host path resolution boundary, a side-effect-free proofpack writer host path resolution model, a docs/spec proofpack writer concrete path/storage policy boundary, a side-effect-free proofpack writer concrete path/storage policy model, a docs/spec proofpack writer first active write slice boundary, a first narrow exact-byte proofpack writer write slice for explicit caller-provided test targets, and a side-effect-free proofpack writer hash/reference integration model

This does not mean referenced artifact hash computation for proofpack refs,
active proofpack-referenced artifact file verification, broad referenced
artifact verification beyond explicit typed file refs, broader proofpack file IO,
broad file IO hashing, hash normalization, broader active proofpack writing beyond
the first explicit-target write slice, active host path
resolution/canonicalization, operation-evidence persistence, runtime proofpack
hash/reference authority, or gate decision writing.

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
