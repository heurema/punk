# Proofpack writer canonical artifact layout v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the canonical proofpack artifact byte/layout boundary for a future proofpack writer before any writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the future write surface to:

```text
validated proofpack values -> deterministic manifest JSON bytes -> one append-only canonical proofpack artifact
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

It does not write operation evidence.

It does not write indexes or `latest` pointers.

It does not verify proofpack referenced artifact refs.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer canonical artifact layout means the byte surface that a future writer may treat as the canonical proofpack artifact for append-only storage, idempotency comparison, conflict detection, and manifest self-digest computation.

For v0.1, the canonical artifact layout is:

```text
canonical proofpack artifact v0.1
└─ artifact body: exact deterministic proofpack manifest JSON bytes
```

The artifact body is exactly the UTF-8 bytes returned by the deterministic proofpack manifest renderer.

In current implementation terms, these are the bytes of:

```rust
proofpack.render_manifest_json().as_bytes()
```

The exact function name may change later, but the boundary remains exact renderer bytes.

For v0.1, Punk deliberately does not select wrapper JSON, separate manifest/wrapper files, JSONL records, archives, content-addressed directory trees, or schema-validation reports as canonical proofpack artifact bytes.

Those may be future non-canonical metadata or future v0.2+ layout candidates, but they are not v0.1 canonical proofpack artifacts.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

This layout boundary does not change proofpack authority.

A proofpack artifact can make closure inspectable only when linked to the required gate/proof chain.

It does not decide and does not create acceptance by being written.

### Proofpack manifest digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines the manifest self-digest over exact deterministic in-memory renderer bytes.

This layout boundary chooses those same bytes as the v0.1 canonical artifact body.

Therefore, for v0.1:

```text
manifest self-digest bytes == canonical proofpack artifact body bytes
```

The manifest self-digest still must not be embedded into those bytes before hashing.

### Proofpack writer storage/schema

`evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md` deferred file layout and schema activation.

This boundary narrows that deferral by selecting a minimal canonical byte surface.

It still does not create schema files, activate `.punk/proofs`, select concrete filenames, or write files.

### Proofpack writer file IO

`evals/specs/proofpack-writer-file-io-boundary.v0.1.md` defines future write-side behavior for explicit storage roots, target refs, target paths, idempotency, conflicts, partial writes, and index/latest handling.

This boundary provides the byte identity that future file IO may compare.

File IO remains future and side-effectful behavior that must be selected separately.

### Proofpack writer operation evidence

`evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md` defines evidence for future writer attempts.

Operation evidence may record the manifest self-digest, target ref, target path ref, write outcome, index status, and latest pointer status.

Operation evidence is not the canonical proofpack artifact.

### Project Memory storage direction

`docs/product/PROJECT-MEMORY.md` separates canonical artifacts from rebuildable indexes/views and optional service mirrors.

This boundary follows that split.

The canonical proofpack artifact body is append-only proof evidence.

Indexes, `latest` pointers, service mirrors, dashboards, and CLI output remain views or evidence surfaces, not proof truth.

## V0.1 layout decision

The selected v0.1 layout is manifest-only canonical artifact bytes.

```text
canonical:
  proofpack manifest JSON renderer bytes

non-canonical around it:
  manifest self-digest metadata
  storage root refs
  target artifact refs
  target path refs
  writer operation evidence
  schema validation reports
  indexes/views
  latest pointers
  CLI output
  service mirrors
```

This decision is intentionally narrow.

It avoids a wrapper-as-truth problem before Punk has schema, migration, privacy, retention, and index policies.

A future writer may still attach or persist metadata around the canonical artifact, but that metadata must not become the v0.1 canonical proofpack artifact unless a later boundary explicitly promotes it.

## Canonical byte rules

The v0.1 canonical artifact body must be exactly the renderer bytes.

A future writer must not change those bytes by:

- adding a trailing newline unless the renderer returned one;
- adding a byte-order mark;
- parsing and re-serializing JSON;
- minifying or pretty-printing outside the renderer;
- sorting fields outside the renderer;
- normalizing Unicode;
- normalizing whitespace;
- normalizing line endings;
- normalizing paths, refs, hashes, or timestamps;
- injecting manifest self-digest metadata into the manifest before hashing;
- adding wrapper fields to the canonical body;
- compressing, encrypting, signing, or archiving the canonical body without a later selected layout boundary.

If renderer output changes, canonical artifact bytes and manifest self-digest change.

That is expected because v0.1 byte identity is exact, not semantic.

## Manifest self-digest coverage

The manifest self-digest covers exactly the canonical artifact body bytes.

It does not cover:

- storage root refs;
- target artifact refs;
- target path refs;
- filenames;
- temp paths;
- operation ids;
- attempted timestamps;
- writer versions;
- operation evidence records;
- index entries;
- `latest` pointers;
- schema validation reports;
- CLI output;
- service mirrors;
- dashboards;
- host-specific absolute paths;
- future wrapper metadata.

The manifest self-digest may be used by future writer logic as content identity for idempotency and conflict checks.

It is not a referenced artifact digest and does not satisfy missing digest requirements for contract, run receipt, eval, event, output artifact, or gate decision refs.

## Metadata outside canonical bytes

A future wrapper or operation evidence record may carry metadata such as:

- layout version;
- proofpack id;
- manifest self-digest;
- writer version;
- created or attempted timestamp;
- target artifact ref;
- target path ref;
- privacy/redaction flags;
- referenced artifact verification summary;
- structural link/hash integrity status;
- boundary notes.

For v0.1, this metadata is outside the canonical artifact body.

It may help inspectability.

It must not become a second proofpack manifest, gate decision, receipt, or acceptance surface.

If a later goal chooses wrapper JSON as canonical, that goal must define a new canonical byte boundary, digest coverage, unknown-field policy, migration policy, and compatibility tests.

## Target ref and path relationship

This boundary does not activate any storage root, target path, filename, directory, or `.punk/proofs` layout.

A future writer may derive a target artifact ref from explicit inputs such as proofpack id and manifest self-digest.

A future writer may derive a target path from an explicit storage root and a target artifact ref using the target path policy model.

Those refs and paths remain outside the canonical artifact body.

Host-specific absolute paths must not become proof truth.

Mutable indexes and `latest` pointers must not be used to infer the canonical artifact body.

## Append-only, idempotency, and conflict behavior

A future writer should use exact canonical artifact bytes or manifest self-digest as the idempotency basis.

Recommended future behavior:

- if no write is selected, report `planned_only` or equivalent;
- if preconditions are missing before writing, report `preflight_failed` or equivalent;
- if the target canonical artifact is absent and write succeeds, report `written` or equivalent;
- if the target exists with exactly matching canonical bytes or matching manifest self-digest, report `already_exists_matching` or equivalent;
- if the target exists with different canonical bytes or a different manifest self-digest, fail closed and report `conflict_existing_different` or equivalent;
- if a write attempt fails before complete canonical availability, report `write_failed` or equivalent;
- if canonical availability is ambiguous, report `partial_write_detected` or equivalent;
- if index or `latest` pointer updates fail after canonical availability, report those side effects separately.

The canonical artifact must not be overwritten silently.

Idempotent evidence must not claim a new write occurred.

Conflict evidence must not mutate the existing canonical artifact.

## Privacy and redaction boundary

Canonical proofpack artifact bytes may expose refs, digest strings, timestamps, and boundary notes.

Privacy and redaction must be applied before manifest rendering or recorded as unresolved boundary notes.

A future writer must not include by default:

- secrets;
- credentials;
- raw provider payloads;
- private prompts;
- executor-local memory;
- private user text;
- environment dumps;
- unredacted logs;
- large generated artifact bodies;
- hidden absolute local paths.

If redaction changes manifest values, it changes canonical artifact bytes and the manifest self-digest.

Retention, deletion, publication, signing, encryption, and transparency-log policies remain future boundaries.

## Unknown-field and migration boundary

V0.1 canonical bytes are renderer-defined manifest JSON bytes.

A future writer must not accept arbitrary additional JSON fields as v0.1 canonical output unless the renderer produced them.

Unknown fields in a future wrapper are non-canonical unless a later schema/layout boundary says otherwise.

Migration and compatibility behavior remain deferred.

A later layout version must explicitly define:

- version fields;
- required and optional fields;
- unknown-field policy;
- deterministic serialization rules;
- digest coverage;
- migration and supersession behavior;
- privacy/redaction representation;
- compatibility with existing manifest self-digests.

## Setup-neutral boundary

Canonical artifact layout must stay setup-neutral.

It must not require:

- a specific IDE;
- a specific executor;
- a specific provider or model;
- prompts or skills;
- network access;
- background services;
- global config;
- `punk init`;
- hidden local runtime state;
- mutable indexes or service state.

A future implementation may provide setup helpers, but they must not be required or implied by this boundary.

## Authority boundary

A canonical proofpack artifact is provenance evidence.

It is not:

- a gate decision;
- a contract approval;
- a run receipt;
- an eval report;
- a validator pass;
- referenced artifact verification;
- writer operation evidence;
- acceptance claim;
- mutable project status;
- executor claim.

Writing or finding a matching canonical proofpack artifact must not decide work, approve work, close work, hide missing validators, or transform executor claims into proof.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Future writer preconditions

A future writer should not persist the canonical proofpack artifact unless all required preconditions are explicit.

Recommended future preconditions:

1. proofpack values are structurally valid;
2. post-gate decision ref is present;
3. contract refs and required evidence refs are explicit;
4. declared artifact digest entries are present or missing status is explicit;
5. structural link/hash integrity status is available;
6. referenced artifact verification status is linked or explicitly not supplied;
7. manifest bytes are rendered deterministically in memory;
8. manifest self-digest is computed from exact renderer bytes;
9. canonical artifact layout version is explicit;
10. storage root, target artifact ref, and target path ref are explicit and policy-allowed;
11. idempotency and conflict policy is explicit;
12. privacy/redaction policy has been applied or recorded as unresolved;
13. index/latest behavior is explicit and non-authoritative;
14. operation evidence behavior is explicit and separate.

These are preparation constraints, not active checks today.

## Required future tests/evals

A future implementation should add tests or smoke eval coverage for:

### PWCA-001: manifest-only canonical bytes

The future writer writes or compares exact deterministic manifest renderer bytes as the v0.1 canonical artifact body.

### PWCA-002: manifest self-digest covers canonical body only

The manifest self-digest covers exact canonical artifact body bytes and excludes wrapper metadata, operation evidence, indexes, latest pointers, paths, and CLI output.

### PWCA-003: wrapper metadata is non-canonical

Wrapper metadata, schema validation reports, operation evidence, indexes, latest pointers, and service mirrors do not become canonical proofpack artifacts.

### PWCA-004: idempotency uses exact bytes or manifest self-digest

Existing matching canonical bytes are idempotent; different bytes fail closed as conflict.

### PWCA-005: no hidden source of truth

The future writer must not infer canonical bytes from chat state, executor memory, IDE state, service mirrors, indexes, `latest` pointers, global config, or `punk init`.

### PWCA-006: privacy and redaction before render

Redaction-sensitive values must be applied before manifest rendering or recorded as unresolved; redaction changes canonical bytes and manifest self-digest.

### PWCA-007: no acceptance authority side effects

Canonical artifact write or idempotent availability must not write gate decisions, claim acceptance, run validators, verify referenced artifacts by itself, or change work status.

### PWCA-008: no runtime activation implied

Docs and reports referencing this boundary must not describe `.punk/proofs`, schema files, proofpack writing, CLI behavior, or runtime storage as active.

## Non-goals

This v0.1 boundary does not define or implement:

- proofpack writer implementation;
- `.punk/proofs` directory creation;
- concrete proofpack filenames or path layout;
- schema files;
- wrapper JSON as canonical artifact;
- separate manifest/wrapper file layout;
- JSONL proofpack storage;
- archive, compression, signing, encryption, or transparency-log formats;
- operation evidence persistence;
- indexes or `latest` pointer writes;
- CLI commands;
- runtime proof storage;
- proofpack referenced-ref verification integration implementation;
- hash or byte normalization;
- gate implementation;
- acceptance claim schema;
- provider/model/agent adapters;
- automation;
- `punk init`.

## Open follow-ups

- proofpack writer implementation;
- actual `.punk/proofs` storage activation;
- concrete target artifact ref and filename policy;
- proofpack schema files, if selected;
- wrapper metadata policy, if needed;
- unknown-field and migration policy;
- privacy/redaction and retention policy for proof artifacts;
- operation evidence persistence policy;
- index and latest-pointer policy;
- proofpack referenced-ref verification integration implementation;
- gate/eval/proof orchestration;
- acceptance claim wiring after gate and proof are linked;
- signing, encryption, remote attestation, or transparency-log policy;
- possible future `punk init` interaction with proof storage.

## Work Ledger next step

After this canonical artifact layout boundary, run another advisory Work Ledger Review before selecting proofpack writer implementation, `.punk/proofs` activation, schema files, CLI behavior, referenced-ref verification integration, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
