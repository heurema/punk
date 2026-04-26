# Proofpack writer target artifact ref policy v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define how a future proofpack writer should identify the intended canonical proofpack artifact before any proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, gate/proof orchestration, referenced-ref verification integration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the future target artifact surface to:

```text
proofpack id + manifest self-digest + canonical artifact layout -> explicit target artifact ref policy
```

It does not activate proofpack writing.

## Status and authority

This policy is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not create `.punk/proofs`.

It does not expose CLI behavior.

It does not write proofpacks.

It does not write operation evidence.

It does not write indexes or `latest` pointers.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

A proofpack writer target artifact ref is the future stable logical reference to the intended canonical proofpack artifact.

For v0.1, the target artifact ref is not:

- a host absolute path;
- a current working directory derived path;
- a target path ref;
- a storage root ref;
- a filename selected by an executor;
- an index entry;
- a `latest` pointer;
- CLI output;
- a service mirror id;
- a proof of acceptance.

It can define how a future writer names or references the canonical artifact candidate before file IO begins.

It cannot establish:

- that `.punk/proofs` exists;
- that a proofpack was written;
- that schema validation exists;
- that referenced artifact bytes were verified;
- that a gate accepted work;
- that positive acceptance may be claimed;
- that indexes or `latest` pointers are canonical truth.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Target artifact refs must preserve that model.

A target artifact ref is an address for future proof evidence storage.
It is not a decision, receipt, checklist, tracker item, or acceptance surface.

### Proofpack manifest digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines the manifest self-digest over exact deterministic in-memory renderer bytes.

The manifest self-digest is the content identity for those exact manifest bytes.

Target artifact refs may include or be derived from the manifest self-digest, but the ref itself is not the digest computation and must not change digest coverage.

### Proofpack writer canonical artifact layout

`evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md` selects exact deterministic proofpack manifest JSON renderer bytes as the v0.1 canonical artifact body.

Target artifact ref policy must not wrap, reserialize, normalize, compress, encrypt, sign, or otherwise change those bytes.

The target artifact ref is outside canonical artifact bytes.

### Proofpack writer canonical artifact model

Current `punk-proof` exposes a side-effect-free canonical artifact model.

The model ties together:

- proofpack id;
- canonical layout;
- exact canonical body bytes;
- manifest self-digest over those bytes;
- non-canonical surrounding surfaces.

A future target artifact ref should be derived from this explicit model or equivalent explicit values, not from hidden runtime or executor state.

### Proofpack writer target path policy

Current `punk-proof` can evaluate explicit target path refs without filesystem side effects.

Target artifact ref policy is separate from target path policy.

```text
target artifact ref != target path ref
target artifact ref != storage root ref
target path ref != proof truth
```

A later writer may derive a target path from an explicit storage root and a target artifact ref, but that derivation remains future file IO policy.

### Proofpack writer file IO

`evals/specs/proofpack-writer-file-io-boundary.v0.1.md` defines future file IO around explicit storage roots, target refs, target paths, idempotency, conflicts, partial writes, and index/latest handling.

This policy defines the target artifact identity input that future file IO may consume.

It still does not select a concrete directory, filename, extension, temp file, atomic move behavior, or index format.

### Project Memory storage direction

`docs/product/PROJECT-MEMORY.md` separates canonical artifacts from rebuildable indexes/views and optional service mirrors.

Target artifact refs must follow that split.

Indexes, `latest` pointers, service mirrors, dashboards, and CLI output may point to or display a target artifact ref, but they must not own its truth.

## V0.1 target artifact identity decision

The v0.1 canonical target artifact identity is the pair:

```text
(proofpack_id, manifest_self_digest)
```

Both fields are required.

`proofpack_id` provides the stable proofpack object identity and human-inspectable correlation.

`manifest_self_digest` provides exact content identity for the v0.1 canonical artifact body bytes.

Neither field should be used alone as the full target artifact identity.

| Candidate identity | Decision | Rationale |
|---|---|---|
| `proofpack_id` only | Not sufficient | It identifies the proofpack object, but it does not distinguish byte-level content drift, renderer changes, or conflict cases. |
| `manifest_self_digest` only | Not sufficient | It identifies bytes, but it loses the explicit proofpack object identity and weakens inspectable linkage to the proofpack chain. |
| `(proofpack_id, manifest_self_digest)` | Selected | It preserves object identity and exact byte identity without relying on paths, indexes, `latest`, service mirrors, or executor claims. |

The canonical artifact body is still the exact manifest renderer bytes.
The target artifact identity is metadata around those bytes.

## Logical ref vocabulary

A future implementation may render the selected identity as a logical ref.

Recommended v0.1 display vocabulary:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

Example:

```text
proofpack:proofpack_local_001@sha256:0123456789abcdef...
```

This string is a logical ref, not a filesystem path.

A future filename or path encoding may be derived from it only through an explicit target path policy.

A future implementation may choose a structured representation instead:

```yaml
kind: proofpack_canonical_artifact
proofpack_id: proofpack_local_001
manifest_self_digest: sha256:0123456789abcdef...
layout: manifest_only_json
```

The structured representation is preferred for APIs and operation evidence.
The display string is only a compact human-readable rendering.

## Derivation rules

A future writer may derive target artifact identity from explicit inputs only.

Allowed derivation inputs:

- proofpack id from the explicit proofpack value;
- manifest self-digest computed over exact deterministic manifest renderer bytes;
- canonical artifact layout version;
- explicit boundary notes;
- explicit writer preflight plan refs, if already selected by a future goal.

Disallowed derivation inputs:

- host absolute paths;
- current working directory;
- editor or IDE state;
- chat state;
- executor-local memory;
- mutable indexes;
- mutable `latest` pointers;
- service mirror state;
- CLI output;
- unlinked files;
- inferred repository roots;
- provider/model/agent identity;
- executor claims.

If an implementation cannot compute or receive both `proofpack_id` and `manifest_self_digest`, it must report a missing precondition instead of inventing a target artifact ref.

## Relationship to storage root and target path

The target artifact ref is a logical artifact identity.

A storage root is a future explicit place where artifacts may be written.

A target path ref is a future policy-derived path under a storage root.

For v0.1:

```text
canonical identity: (proofpack_id, manifest_self_digest)
logical ref:        proofpack:<proofpack_id>@<manifest_self_digest>
storage root ref:   non-canonical future write target root
target path ref:    non-canonical future path under storage root
```

The target artifact ref must not contain host-specific absolute path authority.

If a future writer encodes the logical ref into a path, the path encoding must be reversible or inspectable enough to recover the proofpack id and manifest self-digest without reading mutable indexes or `latest` pointers.

## Idempotency and conflict policy

A future writer should use the selected identity for idempotency and conflict checks.

Recommended future behavior:

- same `proofpack_id` and same `manifest_self_digest` means idempotent match for the canonical artifact identity;
- same `proofpack_id` and different `manifest_self_digest` means conflict or version drift, not overwrite;
- missing target artifact ref means preflight failure before filesystem writes;
- target path exists with different canonical bytes means conflict, not silent replacement;
- index or `latest` mismatch must not override canonical artifact identity.

A future policy may define append-only versioning for repeated proofpack ids, but it must do so explicitly and must not mutate existing canonical artifacts in place.

## Evidence versus authority

A target artifact ref may appear in future operation evidence.

Operation evidence may record:

- selected target artifact ref;
- proofpack id;
- manifest self-digest;
- target path ref;
- storage root ref;
- write outcome;
- idempotency or conflict outcome;
- index/latest update outcome.

That evidence is useful for inspection.

It is not final acceptance.

A target artifact ref does not prove that referenced artifacts were verified.

A target artifact ref does not prove that a proofpack was written.

A target artifact ref does not prove that a gate accepted work.

## Indexes, latest pointers, and service mirrors

Indexes, `latest` pointers, and service mirrors are views.

They may help find target artifact refs.

They must be rebuildable from canonical artifacts and explicit evidence.

They must not be used to create target artifact refs when the selected identity tuple is missing.

If a `latest` pointer says one thing and the canonical artifact identity says another, the identity tuple and gate/proof links win.

If a service mirror is stale, missing, or inconsistent, local canonical artifacts and repo-tracked evidence win.

## Future implementation requirements

A future Rust model or writer should expose boundary/capability flags showing that target artifact ref policy itself does not:

- read the filesystem;
- write the filesystem;
- create `.punk/proofs`;
- write schema files;
- write CLI output;
- verify referenced artifacts;
- write operation evidence;
- write indexes or `latest` pointers;
- write gate decisions;
- create acceptance claims.

Future side-effectful writer behavior must be selected separately.

## Non-goals

This policy does not define:

- `.punk/proofs` activation;
- a concrete filename or extension;
- a concrete directory layout;
- schema files;
- CLI commands;
- active writer behavior;
- operation evidence persistence;
- index or `latest` pointer format;
- service mirror behavior;
- proofpack referenced-ref verification integration;
- gate decisions;
- acceptance claims;
- provider/model/agent adapters;
- `punk init`.

## Open follow-ups

- Implement a side-effect-free `punk-proof` target artifact ref policy model.
- Define filename/path encoding if `.punk/proofs` is selected later.
- Define active proofpack writer preflight integration after target artifact ref policy is modeled.
- Define schema files only through a separate selected goal.
- Define proofpack referenced-ref verification integration separately from target artifact identity.
- Keep `punk init` deferred until a bounded runtime setup goal selects it.
