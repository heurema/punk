# File IO artifact hashing boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary for future file IO artifact hashing before any implementation reads artifact files, verifies referenced artifact bytes, writes proofpacks, writes `.punk/` runtime state, exposes CLI behavior, writes gate decisions, or creates acceptance claims.

This is a design/spec artifact only.

It does not activate file IO hashing.

It narrows the future behavior to one possible operation:

```text
explicit repo-relative artifact ref -> policy-allowed file read -> exact file bytes -> canonical sha256 digest metadata
```

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not activate proofpack writing, referenced artifact byte verification, hash normalization, runtime storage, provider/model/agent adapters, automation, gate decisions, acceptance claims, or CLI behavior.

It does not write `.punk/` state.

Future implementation requires a separate bounded goal.

## Relationship to existing hash boundaries

### Artifact hash policy

`evals/specs/artifact-hash-policy.v0.1.md` defines canonical digest identity:

```text
sha256:<64 lowercase hex chars>
```

A future file IO hash helper must return digest metadata in that format and must pass artifact digest validation.

It must not introduce a second digest identity format.

### Exact-byte computation helper

`evals/specs/artifact-hash-computation-helper.v0.1.md` defines the existing active helper boundary:

```text
caller-provided exact bytes -> canonical sha256 digest metadata
```

A future file IO hash helper should reuse that exact-byte helper after reading file bytes.

The exact-byte helper remains lower-level and side-effect-free.

File IO behavior must stay in a separate helper because reading files adds runtime, privacy, path, symlink, missing-file, and setup concerns.

### Proofpack manifest digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines manifest self-digest over deterministic in-memory renderer bytes.

Manifest self-digest is not file IO hashing.

A future file IO helper must not change proofpack manifest digest semantics.

A manifest self-digest does not verify referenced artifacts.

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` keeps proofpack as post-gate provenance and not decision authority.

A future file IO hash helper may later help compute hashes for artifacts that a proofpack references.

It must not make proofpack the gate, writer, storage surface, or acceptance authority.

## Future helper responsibility

Recommended future Rust API shape:

```rust
pub fn compute_artifact_file_digest(
    repo_root: &RepoRoot,
    artifact_ref: &RepoRelativeArtifactRef,
) -> Result<ArtifactDigest, FileArtifactHashError>
```

The exact names may differ during implementation.

The boundary must remain:

- caller supplies an explicit repo root or equivalent already-authorized root;
- caller supplies an explicit validated repo-relative artifact ref;
- helper reads one regular file if policy allows it;
- helper hashes exactly the file bytes read;
- helper returns canonical `sha256:<64 lowercase hex>` digest metadata;
- helper does not infer refs from paths;
- helper does not scan directories;
- helper does not write files;
- helper does not write `.punk/` state;
- helper does not update proofpacks;
- helper does not write gate decisions;
- helper does not claim acceptance;
- helper does not expose CLI behavior by itself.

## Repo root and ref boundary

Future file IO hashing must be rooted explicitly.

The helper must not use the process current working directory as hidden authority unless a caller has explicitly resolved it into a repo root under policy.

Repo-relative artifact refs must stay separate from filesystem paths.

The helper must not accept absolute paths as its public artifact identity.

Allowed ref shape should follow current artifact ref validation:

- slash-separated;
- no leading slash;
- no `.` or `..` path segments;
- no empty path segments;
- no home-directory shorthand such as `~`;
- no URL scheme;
- no platform-specific backslashes;
- no absolute filesystem paths.

Examples of refs that may be eligible after future policy approval:

```text
work/reports/2026-04-26-example.md
evals/specs/file-io-artifact-hashing-boundary.v0.1.md
target/artifact/example.bin
```

Non-examples:

```text
/absolute/path/report.md
../work/report.md
work//report.md
work/./report.md
https://example.com/report.md
~/project/report.md
work\report.md
```

## File eligibility boundary

The first future implementation should hash only regular files.

It should reject or defer:

- directories;
- missing files;
- device files;
- sockets;
- pipes;
- platform-specific special files;
- unresolved refs;
- refs outside the explicit repo root;
- refs that fail repo-relative validation.

Directory hashing is out of scope for v0.1.

Archive canonicalization is out of scope for v0.1.

Recursive hashing is out of scope for v0.1.

Glob expansion is out of scope for v0.1.

## Symlink boundary

Symlink behavior must be explicit before implementation.

Default v0.1 stance:

- do not silently follow symlinks;
- do not silently hash symlink targets;
- do not silently hash symlink path text;
- report a symlink or non-regular-file status if encountered;
- require a later policy if symlink support becomes necessary.

This prevents path escape, workspace surprise, and cross-platform drift.

## Exact bytes boundary

A future file IO hash helper hashes exactly the bytes read from the eligible file.

It must not perform:

- newline normalization;
- Unicode normalization;
- Markdown normalization;
- JSON normalization;
- YAML normalization;
- whitespace normalization;
- path normalization inside file content;
- archive canonicalization;
- semantic hashing;
- redaction before hashing.

If a future artifact type has deterministic serialization, a separate writer or serializer policy may produce the file bytes.

The file IO hash helper only hashes bytes that already exist at the resolved file.

## Generated artifact boundary

Generated artifacts may be hashable only when the caller supplies an explicit artifact ref and future policy allows the path.

The helper must not:

- run generators;
- build targets;
- infer expected outputs;
- mutate build directories;
- decide whether generated output is complete;
- decide whether output should be committed.

Generated artifact completeness remains separate evidence and gate policy.

## Missing and error status boundary

Future implementation should return explicit errors or statuses.

Recommended future vocabulary:

- `invalid_ref` - ref fails repo-relative policy;
- `outside_repo_root` - resolved path would escape the explicit root;
- `missing` - file does not exist;
- `not_regular_file` - path exists but is not a regular file;
- `symlink_not_supported` - path is a symlink and v0.1 policy rejects it;
- `read_denied` - OS denied read access;
- `read_error` - file could not be read for another explicit reason;
- `hashed` - exact bytes were read and digest metadata was computed.

Missing or unreadable artifacts must not be represented as passing.

A digest supplied by an executor or external tool remains `unverified` until Punk or an allowed local validator recomputes it under policy.

## Referenced artifact verification boundary

File IO hashing and referenced artifact verification are separate.

A future file IO helper may compute the digest for one file.

It does not by itself decide that a proofpack ref is verified.

Referenced artifact verification requires additional policy to compare:

```text
expected kind + expected ref + expected digest
against
observed kind + observed ref + observed digest
```

That comparison can still be evidence only until future gate/proof policy uses it.

The v0.1 file IO boundary does not implement referenced artifact verification.

## Proofpack writer boundary

A future file IO hash helper must not write proofpack manifests or proofpack indexes.

It must not add digest entries to proofpacks.

It must not write `.punk/proofs`.

It must not create wrapper artifacts for manifest self-digests.

Proofpack writer hash integration remains a separate future goal.

## Runtime storage and CLI boundary

A future file IO hash helper may be a library primitive.

It must not imply:

- `.punk/` runtime storage;
- `.punk/proofs` storage;
- `.punk/evals` storage;
- new CLI commands;
- `punk init` behavior;
- background scanning;
- watch mode;
- automation;
- provider/model/agent adapters.

If a later CLI or runtime path needs file IO hashing, that must be selected through a separate bounded goal.

## Evidence and authority boundary

A file digest can establish byte identity for the bytes read from a policy-allowed file at a specific time.

It does not prove:

- that the file is the right artifact;
- that the file is complete;
- that the file was produced by the claimed run;
- that validators passed;
- that scope was preserved;
- that a proofpack was written;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Hash metadata remains evidence, not authority.

Only future `gate` writes final decisions.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Privacy and safety boundary

Reading a file to hash it can expose sensitive metadata and may touch sensitive content.

A digest can leak information when source content is small, guessable, proprietary, or sensitive.

The helper should not inspect, classify, redact, upload, or publish content.

Future policy must decide whether a path is safe to hash-reference.

The helper must not encourage hashing:

- secrets;
- credentials;
- raw provider payloads;
- environment dumps;
- private user text;
- sensitive generated artifacts;
- dependency caches;
- local machine state;
- files outside the explicit repo root.

## User setup neutrality

File IO hashing must stay compatible with Punk's setup-neutral stance.

A future helper must not require:

- a specific IDE;
- a specific executor;
- a specific provider;
- a prompt or skill setup;
- background services;
- network access;
- `punk init`;
- a global config file;
- hidden local state.

The caller supplies explicit inputs and receives explicit evidence or error status.

## Capability boundary

Current behavior after manifest digest helper remains:

```text
punk_core.computes_hashes = true
punk_core.normalizes_artifact_bytes = false
punk_core.writes_runtime_state = false
punk_proof.computes_manifest_digest = true
punk_proof.computes_referenced_artifact_hashes = false
punk_proof.writes_proofpack = false
punk_proof.requires_runtime_storage = false
punk_proof.writes_cli_output = false
punk_proof.creates_acceptance_claim = false
```

A future file IO implementation may add a narrow capability such as:

```text
computes_file_artifact_digests = true
verifies_referenced_artifact_bytes = false
writes_runtime_state = false
writes_proofpack = false
writes_cli_output = false
creates_acceptance_claim = false
```

If implementation uses broader `computes_hashes` wording, tests and docs must state whether it means exact in-memory bytes, manifest self-digests, file artifact digests, or referenced artifact verification.

## Required future tests/evals

A future implementation should add tests or smoke eval coverage for:

### FIAH-001: repo-relative ref required

Absolute paths, parent traversal, URL refs, home refs, backslashes, empty path segments, and dot segments are rejected before file reads.

### FIAH-002: regular file hashing uses exact bytes

A regular file with known bytes hashes to the same digest as `compute_artifact_digest(bytes)`.

### FIAH-003: no byte normalization

Files that differ only by newline or trailing whitespace produce different digests.

### FIAH-004: missing file is explicit

Missing files return an explicit missing/error status and do not produce a passing digest.

### FIAH-005: directories are rejected

Directories are not recursively hashed and do not produce digest metadata.

### FIAH-006: symlink stance is enforced

Symlinks are rejected or reported explicitly according to the v0.1 policy; they are not silently followed.

### FIAH-007: no proofpack writer side effect

Computing a file digest does not write proofpacks, `.punk/` state, schemas, gate decisions, CLI output, or acceptance claims.

### FIAH-008: no referenced artifact verification side effect

Computing a file digest does not make proofpack missing digest refs pass and does not mark a referenced artifact verified.

### FIAH-009: setup-neutral behavior

The helper does not require `punk init`, global config, background services, network access, providers, models, agents, or IDE setup.
