# Proofpack manifest digest boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the first boundary for a future proofpack manifest digest helper before any implementation, proofpack writer, `.punk/` storage, schema, gate/proof runtime, CLI behavior, or file IO hashing depends on proofpack manifest hashes.

This is a design/spec artifact only.

It narrows the future helper to one operation:

```text
deterministic in-memory proofpack manifest string -> exact UTF-8 bytes -> canonical sha256 digest metadata
```

## Status and authority

This boundary is not implementation.

It does not activate proofpack manifest digest computation.

It does not add Rust dependencies.

It does not activate proofpack writing, file IO hashing, referenced artifact verification, hash normalization, runtime storage, schemas, provider/model/agent adapters, automation, gate decisions, acceptance claims, or CLI behavior.

It does not write `.punk/` state.

Future implementation requires a separate bounded goal.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle that references evidence without deciding.

A proofpack manifest digest may identify the bytes of that manifest.

It does not make proofpack the closure authority.

`gate` remains the future decision authority.

### Artifact hash policy

`evals/specs/artifact-hash-policy.v0.1.md` defines digest identity as:

```text
sha256:<64 lowercase hex chars>
```

A future proofpack manifest digest must use that canonical identity and pass artifact digest validation.

### Artifact hash computation helper

`evals/specs/artifact-hash-computation-helper.v0.1.md` defines the exact-byte hashing helper.

A future proofpack manifest digest helper should reuse `punk-core` exact-byte SHA-256 computation over caller-provided bytes.

It should not add a new SHA-256 dependency to `punk-proof`.

It should not expose `sha2` dependency types through public Punk APIs.

## Manifest bytes boundary

The bytes to hash are exactly the UTF-8 bytes of the deterministic in-memory string returned by the proofpack manifest renderer.

In current implementation terms, the future helper may use the bytes of:

```rust
proofpack.render_manifest_json().as_bytes()
```

The exact future function name may differ, but the boundary must remain:

- render the manifest in memory;
- encode the rendered string as UTF-8;
- hash exactly those bytes;
- return canonical `sha256:<64 lowercase hex>` digest metadata;
- do not read from the filesystem;
- do not write the manifest;
- do not add a trailing newline unless the renderer returned one;
- do not add a byte-order mark;
- do not parse and re-serialize JSON;
- do not minify, pretty-print, sort, normalize, or canonicalize outside the renderer;
- do not normalize Unicode, whitespace, line endings, paths, refs, or hashes.

If the deterministic renderer output changes, the manifest digest changes.

That is expected because the digest identifies exact bytes, not semantic equality.

## Manifest self-digest vs referenced artifact digests

A proofpack manifest self-digest identifies the proofpack manifest bytes.

It is not the same as the artifact digests listed inside the manifest.

The manifest self-digest does not prove that referenced artifacts are present, complete, valid, or unchanged.

Referenced artifact digest integrity remains separate:

```text
manifest self-digest -> identifies proofpack manifest bytes
artifact digest entries -> identify referenced evidence bytes when separately verified
link/hash integrity report -> checks declared refs have matching digest entries
future file IO hash helper -> may later hash persisted artifact files
future gate/proof policy -> may later decide which evidence is sufficient
```

The self-digest must not satisfy missing required artifact digests for:

- gate decision;
- contract;
- run receipt;
- eval report;
- event range;
- output artifact.

It is metadata for the manifest itself.

## Recursion boundary

The v0.1 manifest self-digest must not be embedded into the manifest before hashing.

Otherwise the manifest would need recursive self-reference semantics that are out of scope.

If a future proofpack writer stores the manifest digest, it should store it outside the hashed manifest bytes or in a separate wrapper/index artifact until a later spec defines self-referential packaging.

## Future helper responsibility

Recommended future Rust API shape:

```rust
pub fn compute_proofpack_manifest_digest(proofpack: &Proofpack) -> ArtifactDigest
```

The exact function name may change during implementation, but the helper should:

- call the deterministic in-memory manifest renderer;
- hash the exact UTF-8 bytes of the rendered string with `punk-core` exact-byte hashing;
- return canonical artifact digest metadata;
- be deterministic for identical proofpack values and renderer output;
- be side-effect-free;
- avoid adding a new hash dependency to `punk-proof`;
- avoid exposing hash dependency types in public Punk APIs.

The helper must not:

- write proofpacks;
- write `.punk/proofs`;
- create schema files;
- read artifact files;
- verify referenced artifact bytes;
- normalize hashes or bytes;
- write gate decisions;
- claim acceptance;
- expose CLI behavior;
- run validators;
- call provider/model/agent adapters;
- require runtime storage.

## Capability boundary

Current `punk-proof` behavior remains:

```text
models_proofpack = true
writes_proofpack = false
checks_structural_link_hash_integrity = true
computes_hashes = false
normalizes_hashes = false
requires_runtime_storage = false
writes_cli_output = false
creates_acceptance_claim = false
```

A future implementation may add a narrow manifest digest capability.

Preferred future capability wording should distinguish:

```text
computes_manifest_digest = true
computes_referenced_artifact_hashes = false
normalizes_hashes = false
writes_proofpack = false
requires_runtime_storage = false
writes_cli_output = false
creates_acceptance_claim = false
```

If the implementation reuses an existing broader `computes_hashes` flag, tests and docs must state that the flag means proofpack manifest self-digest only.

It must not imply file IO hashing or referenced artifact hash verification.

## Evidence and authority boundary

A manifest digest can establish byte identity for the rendered proofpack manifest bytes.

It does not prove:

- that a proofpack was written to disk;
- that `.punk/proofs` exists;
- that referenced artifacts are available;
- that referenced artifact bytes match their declared digests;
- that validators passed;
- that scope was preserved;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Hash metadata remains evidence, not authority.

Only future `gate` writes final decisions.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Privacy boundary

A manifest digest is metadata.

It may leak information when manifest content is small, guessable, proprietary, or sensitive.

The helper should not inspect, redact, or classify content.

Future policy must decide whether a proofpack manifest is safe to hash-reference or publish.

The helper must not encourage hashing secrets, raw provider payloads, environment dumps, credentials, private user text, or sensitive generated artifacts by default.

## Required future tests/evals

A future implementation should add tests or smoke eval coverage for:

### PMD-001: deterministic manifest digest

The same proofpack value and renderer output produce the same digest across repeated calls.

### PMD-002: digest uses exact renderer bytes

The digest equals the result of hashing `render_manifest_json().as_bytes()` with `punk-core` exact-byte hashing.

### PMD-003: output is canonical digest metadata

The returned digest must use lowercase `sha256:` plus 64 lowercase hex chars and pass artifact hash policy validation.

### PMD-004: no formatting normalization

Adding or removing a trailing newline, changing JSON spacing, or changing renderer output must produce a different digest.

### PMD-005: self-digest is not referenced artifact verification

Computing a proofpack manifest digest must not make missing referenced artifact digests pass link/hash integrity.

### PMD-006: no recursive self-inclusion

The v0.1 helper must hash the manifest bytes without embedding the digest into those bytes first.

### PMD-007: side-effect-free helper

The helper must not write files, write `.punk/` state, add CLI behavior, write proofpacks, write gate decisions, create schemas, run validators, or touch runtime storage.

### PMD-008: no new hash dependency in `punk-proof`

Implementation should reuse `punk-core` exact-byte hashing rather than adding a second SHA-256 dependency to `punk-proof`.

## Non-goals

- proofpack writer implementation;
- `.punk/proofs` storage;
- file IO artifact hashing;
- referenced artifact byte verification;
- hash normalization;
- JSON canonicalization beyond the existing renderer output;
- schema files;
- CLI commands;
- gate decision writing;
- acceptance claim writing;
- provider/model/agent adapters;
- automation;
- `punk init`.

## Open follow-ups

- future proofpack manifest digest helper implementation;
- future smoke eval coverage for manifest digest behavior;
- future proofpack writer boundary and implementation;
- future file IO artifact hashing boundary;
- future referenced artifact hash verification policy;
- future runtime gate/proof/storage work;
- possible future `punk init`.
