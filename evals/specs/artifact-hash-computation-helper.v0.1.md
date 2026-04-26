# Artifact hash computation helper boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the first implementation boundary for future Punk artifact hash computation before any code, dependency, proofpack writer, `.punk/` storage, schema, gate/proof runtime, or CLI behavior depends on computed hashes.

This is a design/spec artifact only.

It narrows active hash computation to a small deterministic helper over exact caller-provided bytes.

## Status and authority

This boundary is not implementation.

It does not activate hash computation.

It does not add Rust dependencies.

It does not activate hash normalization, proofpack writing, gate decisions, runtime storage, schemas, provider/model/agent adapters, automation, or CLI behavior.

It does not write `.punk/` state.

Future implementation requires a separate bounded goal.

## Relationship to artifact hash policy v0.1

`evals/specs/artifact-hash-policy.v0.1.md` defines digest identity and artifact ref policy.

This boundary defines the narrow helper that may later compute such digest identity from bytes.

The output digest remains:

```text
sha256:<64 lowercase hex chars>
```

The helper must reuse the canonical digest validation model after formatting the digest.

A computed digest is evidence metadata.

It is not acceptance.

Positive acceptance still requires an accepting gate decision and matching proofpack under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Dependency stance

Rust stdlib does not provide SHA-256.

Punk should not hand-roll cryptographic SHA-256.

For the future implementation goal, `punk-core` may add one small, maintained SHA-256 implementation dependency if needed.

Preferred stance:

- use a narrow RustCrypto-family digest implementation such as `sha2` if implementation requires a crate;
- keep the dependency owned by `punk-core` only;
- do not expose the dependency type in public Punk APIs;
- do not add async runtime, networking, provider, OS service, CLI, or storage dependencies as part of hashing;
- keep default features minimal where the chosen crate supports feature control;
- record the dependency in the implementation report and re-run workspace checks;
- do not add a custom or copied SHA-256 implementation to active-core.

If a future implementation cannot satisfy this stance, it must stop and select a new decision/ADR goal instead of silently changing active-core dependency policy.

## Helper responsibility

The first helper should do one thing:

```text
exact bytes -> canonical sha256 digest string
```

Recommended future Rust API shape:

```rust
pub fn compute_artifact_digest(bytes: &[u8]) -> ArtifactDigest
```

The exact function name may change during implementation, but the boundary must remain:

- input is caller-provided bytes;
- output is a canonical `ArtifactDigest` that formats as `sha256:<64 lowercase hex chars>`;
- the helper is deterministic for identical byte slices;
- the helper is side-effect-free;
- the helper does not read files;
- the helper does not infer artifact refs;
- the helper does not normalize bytes;
- the helper does not write runtime state.

A later verification helper may compare caller-provided bytes to an expected digest, but mismatch policy for gate/proof stays deferred.

## Exact bytes boundary

The helper hashes exactly the bytes it receives.

It must not perform:

- Markdown normalization;
- JSON normalization;
- YAML normalization;
- Unicode normalization;
- whitespace normalization;
- newline conversion;
- path normalization;
- archive canonicalization;
- semantic hashing.

Examples:

```text
bytes: []
sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

bytes: "abc"
sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
```

The strings `"line\n"` and `"line\r\n"` are different byte sequences and must produce different digests.

If a future schema defines deterministic serialization, callers may pass those serialized bytes to this helper.

The helper itself does not define serialization.

## Path and ref boundary

Path/ref validation stays separate from byte hashing.

The computation helper must not:

- accept a filesystem path as its first implementation surface;
- open files;
- map runtime ids to files;
- resolve symlinks;
- canonicalize paths;
- decide artifact kind/ref policy.

Existing artifact ref validation remains responsible for repo-relative ref shape.

A future file-hashing helper, if needed, must be a separate bounded goal because it introduces filesystem IO, privacy, symlink, path, and missing-file behavior.

## Proofpack manifest boundary

`punk-proof` can now render a deterministic in-memory proofpack manifest.

A future caller may hash the exact UTF-8 bytes of that rendered string.

The hash helper must not know proofpack semantics.

The proofpack renderer must not become a writer just because its output can be hashed.

Proofpack writer behavior remains deferred.

## Evidence and authority boundary

A digest computed by Punk from caller-provided bytes can establish byte identity for those bytes.

It does not prove:

- that the bytes came from the right artifact;
- that the artifact is complete;
- that validators passed;
- that scope was preserved;
- that a gate accepted the work;
- that a proofpack exists.

Executor-provided digest claims remain unverified until Punk or a trusted local validator recomputes the digest under policy.

Hash metadata remains evidence, not authority.

Only future `gate` writes final decisions.

## Privacy boundary

A hash is metadata that can leak information when source content is small, guessable, proprietary, or sensitive.

The helper should not inspect or redact content.

Callers and future policy must decide whether a given artifact is allowed to be hash-referenced.

The helper must not encourage hashing secrets, raw provider payloads, environment dumps, credentials, private user text, or sensitive generated artifacts by default.

## Capability boundary

Before implementation, `punk-core` capabilities remain validation-only:

```text
validates_digest_format = true
validates_repo_relative_refs = true
computes_hashes = false
normalizes_artifact_bytes = false
writes_runtime_state = false
```

After a future bounded implementation, it may be valid for `computes_hashes` to become `true` while these stay `false`:

```text
normalizes_artifact_bytes = false
writes_runtime_state = false
```

That capability change must happen only in the implementation goal and must be covered by tests and smoke eval updates.

## Required future tests/evals

A future implementation should add tests or smoke eval coverage for:

### AHCH-001: empty bytes digest

`[]` hashes to:

```text
sha256:e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
```

### AHCH-002: known bytes digest

`"abc"` hashes to:

```text
sha256:ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad
```

### AHCH-003: output is canonical digest metadata

Computed output must pass artifact hash policy validation and use lowercase `sha256:` plus 64 lowercase hex chars.

### AHCH-004: exact bytes are preserved

Different newline or whitespace byte sequences must produce different digests; no normalization is allowed.

### AHCH-005: side-effect-free helper

The helper must not write files, write `.punk/` state, add CLI behavior, write proofpacks, write gate decisions, or touch runtime storage.

### AHCH-006: executor claim remains unverified

A digest supplied by an executor or adapter must not become verified proof without recomputation by Punk or a trusted local validator.

## Non-goals

This boundary does not define or activate:

- implementation code;
- Cargo dependency changes;
- file IO hashing helpers;
- path normalization;
- byte normalization;
- schema files;
- proofpack writer behavior;
- `.punk/proofs` storage;
- `.punk/events`, `.punk/runs`, `.punk/evals`, or `.punk/decisions` storage;
- gate decision writer behavior;
- proofpack acceptance claims;
- CLI commands;
- `punk init`;
- provider/model/agent adapters;
- automation;
- signing;
- transparency logs;
- archive canonicalization;
- SLSA or in-toto mapping.

## Future implementation constraints

Any later implementation step should explicitly record:

- the chosen dependency, if any;
- why stdlib is insufficient;
- why hand-rolled crypto was avoided;
- exact public API shape;
- test vectors used;
- capability flag changes;
- smoke eval changes;
- proof that no file IO/runtime/CLI/proofpack writer behavior was added.

## Deferred for later specs or goals

Still deferred after this boundary:

- active hash computation implementation;
- file hashing helper behavior;
- hash/reference verification result shape;
- hash mismatch policy for gate/proof;
- proofpack writer implementation;
- decision/proof schema files;
- real `.punk` runtime storage;
- gate/eval/proof orchestration;
- semantic assessor implementation;
- possible future `punk init`.
