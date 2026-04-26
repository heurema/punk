# Proofpack writer storage/schema boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define future proofpack writer storage and schema/file-layout semantics before any proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the future storage surface to:

```text
explicit proofpack values + deterministic manifest bytes + manifest self-digest metadata + storage policy -> append-only proofpack artifact plan
```

It does not activate proofpack writing.

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not create `.punk/proofs`.

It does not expose CLI behavior.

It does not write proofpacks.

It does not write indexes or `latest` pointers.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer storage/schema boundary means defining how a later writer should think about persisted proofpack artifacts, wrapper metadata, indexes, derived views, and schema evolution without activating them.

For v0.1, storage/schema boundary is not storage and not a schema file.

It can define:

- which future artifacts may exist;
- which artifact should be append-only canonical proof evidence;
- which artifacts are wrappers, indexes, or derived views;
- where manifest self-digest metadata may live without recursive self-reference;
- how mutable `latest` pointers must stay non-canonical;
- what future side effects need explicit selection.

It cannot establish:

- that `.punk/proofs` exists;
- that a proofpack was written;
- that schema validation exists;
- that an index was updated;
- that a `latest` pointer is canonical truth;
- that referenced artifact bytes were verified;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Future storage must preserve that model.

It must not turn proofpack storage into a pre-gate checklist, duplicate receipt store, second tracker, or decision surface.

### Proofpack writer preparation

`evals/specs/proofpack-writer-preparation-boundary.v0.1.md` defines writer-ready conceptual inputs, outputs, preconditions, side-effect boundaries, authority limits, and follow-ups.

This boundary narrows the storage/schema side of that preparation.

It does not broaden writer preparation into runtime behavior.

### Proofpack writer hash-policy integration

`evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md` defines how a future writer should preserve declared artifact digests, structural link/hash integrity, referenced artifact verification outcomes, and manifest self-digest metadata.

Storage/schema boundary must keep those hash surfaces visible after persistence.

It must not let storage success hide missing, mismatched, unreadable, invalid, unsupported, or unverified evidence.

### Manifest self-digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines a manifest self-digest over exact deterministic in-memory renderer bytes.

A future writer may store that digest outside the hashed manifest bytes.

The self-digest must not be embedded into the manifest before hashing unless a later recursive self-reference policy exists.

### Project Memory storage direction

`docs/product/PROJECT-MEMORY.md` separates authority from rebuildable runtime views.

Future proofpack storage should follow the same principle:

```text
append-only canonical proof artifacts -> rebuildable indexes/views -> optional service mirror later
```

A missing or stale index must degrade inspect UX, not change proof truth.

## Future artifact classes

A future proofpack writer may eventually produce or update several artifact classes.

They must not be treated as the same authority surface.

### Canonical proofpack artifact

The canonical proofpack artifact is the future append-only proof evidence artifact.

It should contain or reference the deterministic manifest bytes according to a later selected layout.

It should be identified by a stable proofpack id and content digest metadata.

It should not be mutable in place.

### Manifest bytes

The manifest bytes are the deterministic bytes produced by the proofpack renderer.

They may be stored as the proofpack artifact body or inside a later wrapper format.

If stored inside a wrapper, the exact bytes covered by the manifest self-digest must remain inspectable.

### Manifest self-digest metadata

Manifest self-digest metadata identifies the exact manifest bytes.

It should live outside the hashed manifest bytes or in wrapper/index metadata until a later recursive self-reference policy is defined.

It must not satisfy referenced artifact digest requirements.

### Wrapper metadata

A future wrapper may record:

- storage format version;
- proofpack id;
- manifest self-digest;
- generator or writer version;
- created timestamp;
- privacy/redaction flags;
- boundary notes;
- operation evidence refs.

Wrapper metadata must not become a second proofpack manifest unless a later schema explicitly defines it.

### Indexes and views

A future index may support inspect/query UX.

Indexes and views must be rebuildable from canonical append-only proof artifacts and related canonical refs.

They must not own proof truth.

### Mutable latest pointer

A future `latest` pointer may be a convenience view only.

It must be rebuildable.

It must not be the canonical proofpack.

It must not be the source of acceptance truth.

If a `latest` pointer is stale, missing, or inconsistent, the canonical append-only artifact and gate/proof links win.

## Future storage target boundary

A future implementation may choose `.punk/proofs` or another explicit storage root through a separate goal.

This boundary does not activate any directory.

If `.punk/proofs` is later selected, the writer must define:

- the storage root;
- the artifact naming policy;
- overwrite behavior;
- temporary-file and atomic-write behavior;
- index/update behavior;
- cleanup or retention policy;
- privacy/redaction policy;
- cross-platform path handling;
- failure rollback semantics.

The writer must not use the process current working directory, global config, editor state, chat state, or executor-local memory as hidden storage authority.

A caller may resolve a project root or storage root explicitly, but that resolution must be inspectable.

## Future schema/file-layout boundary

This boundary does not create schema files.

A future schema goal should decide whether proofpack storage uses:

- manifest-only JSON;
- wrapper JSON containing manifest bytes or manifest ref;
- separate manifest and wrapper files;
- JSONL records;
- content-addressed paths;
- another explicit layout.

A future schema/file-layout must define:

- stable schema version fields;
- deterministic serialization boundary;
- which bytes are covered by which digest;
- required vs optional fields;
- unknown-field policy;
- migration and supersession behavior;
- how privacy/redaction flags are represented;
- how partial proofpacks are represented if ever allowed.

Until then, the current Rust renderer is the only deterministic manifest byte surface.

No file layout is active.

## Append-only and overwrite boundary

Future canonical proofpack artifacts should be append-only.

A future writer should not overwrite an existing canonical proofpack artifact silently.

Recommended future behavior:

- if the target canonical proofpack artifact already exists with matching bytes, return idempotent evidence;
- if the target exists with different bytes, fail closed and report conflict;
- if a temp/write operation fails, do not leave a canonical partial artifact;
- if an index update fails after canonical artifact write, preserve the canonical artifact and report index failure separately;
- if a `latest` pointer update fails, preserve the canonical artifact and report pointer failure separately.

Idempotency and conflict handling must be explicit in a later implementation.

## Hidden source-of-truth boundary

Future proofpack storage must not create a hidden source of truth.

Not allowed as canonical proof truth:

- chat transcripts;
- executor-local memory;
- IDE state;
- prompts or skills;
- provider/model metadata;
- mutable indexes;
- mutable `latest` pointers;
- service mirrors;
- dashboards;
- unlinked filesystem artifacts.

Project truth must remain in explicit artifacts linked through goal, contract, receipt, eval/assessment, gate decision, and proofpack refs.

## Future writer preconditions

A future proofpack writer should not persist a canonical proofpack unless storage/schema preconditions are explicit.

Recommended future preconditions:

1. proofpack values are structurally valid;
2. canonical post-gate decision ref is present;
3. required linked evidence refs are explicit;
4. required declared digest entries are present or missing status is explicit;
5. structural link/hash integrity report is available;
6. manifest bytes are rendered deterministically in memory;
7. manifest self-digest is computed from exact renderer bytes;
8. storage root and target artifact ref are explicit and policy-allowed;
9. overwrite/idempotency policy is explicit;
10. privacy/redaction policy has been applied upstream or marked unresolved;
11. schema/file-layout version is explicit;
12. index/latest pointer behavior is explicit and non-authoritative.

These are preparation constraints, not active checks today.

## Future writer outputs

A future writer may eventually produce:

- canonical proofpack artifact ref;
- manifest bytes or manifest artifact ref;
- manifest self-digest;
- storage wrapper ref, if any;
- structural link/hash integrity status;
- referenced artifact verification status summary, if supplied;
- writer operation evidence or receipt, if a later boundary defines it;
- index update status, if indexes are later selected;
- latest pointer update status, if a pointer is later selected;
- boundary notes for deferred or unresolved evidence.

For v0.1, these outputs are conceptual only.

This spec does not define a schema or file layout.

## Failure and partial-write policy

Future proofpack writing should fail closed when required storage or schema preconditions are missing.

It must not silently invent:

- proofpack ids;
- storage roots;
- artifact paths;
- schema versions;
- manifest bytes;
- manifest self-digests;
- gate decision refs;
- contract refs;
- receipt refs;
- eval refs;
- event refs;
- output refs;
- declared digests;
- verification outcomes;
- timestamps;
- acceptance claims.

If partial proofpack artifacts are later allowed, partial state must be explicit and must not enable positive acceptance claims.

## Privacy and retention boundary

Proofpack storage can expose sensitive refs, hashes, timestamps, event ranges, and boundary notes.

A future storage/schema implementation must not include by default:

- secrets;
- credentials;
- raw provider payloads;
- private prompts;
- environment dumps;
- private user text;
- large generated artifact bodies;
- unredacted logs;
- hidden absolute local paths.

Retention, redaction, deletion, and publication policies remain future explicit boundaries.

## Setup-neutral boundary

Proofpack storage/schema behavior must stay setup-neutral.

It must not require:

- a specific IDE;
- a specific executor;
- a specific provider or model;
- a prompt or skill setup;
- background services;
- network access;
- `punk init`;
- global config;
- hidden local runtime state.

A future implementation may provide setup helpers, but they must not be required or implied by this boundary.

## Authority boundary

A future proofpack writer may persist provenance artifacts.

It must not decide.

It must not approve work.

It must not close a contract.

It must not transform failed or missing validators into passing evidence.

It must not turn executor claims into proof.

It must not make a mutable `latest` pointer canonical truth.

It must not create positive acceptance claims by itself.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Non-goals

This v0.1 boundary does not define or implement:

- proofpack writer implementation;
- `.punk/proofs` directory creation;
- `.punk/proofs` storage layout activation;
- schema files;
- CLI commands;
- runtime proof storage;
- proofpack referenced-ref verification integration implementation;
- hash normalization;
- broader file IO hashing;
- directory, archive, recursive, or semantic hashing;
- signing;
- transparency logs;
- remote attestation exports;
- final acceptance claim schema;
- gate implementation;
- provider/model/agent adapters;
- automation;
- `punk init`.

## Required future tests/evals

A future implementation should add tests or smoke eval coverage for:

### PWSS-001: no storage active before selected implementation

Docs and reports that reference this boundary must not describe `.punk/proofs`, schemas, CLI commands, or proofpack writing as active behavior.

### PWSS-002: append-only canonical artifact behavior

A future writer must not silently overwrite a canonical proofpack artifact; matching bytes may be idempotent, different bytes must report conflict.

### PWSS-003: manifest self-digest stored outside hashed manifest bytes

The manifest self-digest must identify exact manifest bytes and must not require recursive self-inclusion in v0.1.

### PWSS-004: indexes and latest pointers are non-canonical

Missing, stale, or inconsistent indexes/latest pointers must not change canonical proof truth.

### PWSS-005: schema/file-layout version is explicit

Future persisted proofpack artifacts must carry an explicit version and deterministic byte boundary before schema validation or migration claims are made.

### PWSS-006: no acceptance authority side effects

Writing proof storage must not write gate decisions, create acceptance claims, run validators, call providers, expose CLI behavior by itself, or mutate unrelated `.punk` surfaces.

### PWSS-007: setup-neutral behavior

Future storage/schema behavior must not require `punk init`, global config, background services, network access, providers, models, agents, skills, or IDE setup.

## Open follow-ups

- proofpack writer implementation;
- actual `.punk/proofs` storage activation;
- proofpack schema files;
- proofpack writer CLI surface, if any;
- atomic write and rollback policy;
- proofpack index and latest-pointer policy;
- proofpack referenced-ref verification integration implementation;
- privacy/redaction and retention policy for proof artifacts;
- partial proofpack policy;
- writer receipt or operation evidence shape;
- acceptance claim wiring after gate and proof are linked;
- remote attestation, signing, or transparency-log policy;
- possible future `punk init` interaction with proof storage.

## Next bounded step

After this proofpack writer storage/schema boundary, run another advisory Work Ledger Review before selecting proofpack writer implementation, `.punk/proofs` activation, schema files, CLI, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
