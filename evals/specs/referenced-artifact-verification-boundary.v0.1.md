# Referenced artifact verification boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define what Punk means by future referenced artifact verification before any implementation compares proofpack artifact refs and declared hashes against persisted artifact bytes.

This is a design/spec artifact only.

It does not activate referenced artifact verification.

It narrows the future behavior to one possible operation:

```text
proofpack artifact ref + expected digest + explicit repo root -> policy-allowed file read -> exact file bytes -> canonical digest -> comparison evidence
```

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not activate proofpack writing, runtime storage, provider/model/agent adapters, automation, gate decisions, acceptance claims, or CLI behavior.

It does not write `.punk/` state.

Future implementation requires a separate bounded goal.

## Definition

Referenced artifact verification means checking whether a declared artifact digest for a declared artifact ref matches the exact bytes of the explicit file that policy allows Punk to inspect.

For v0.1, verification is evidence only.

It can establish:

```text
this explicit ref, under this explicit root, produced this observed digest, and it matches or does not match the expected digest
```

It does not establish:

- that the artifact should have existed;
- that the artifact is semantically correct;
- that validators passed;
- that scope was preserved;
- that the proofpack is complete;
- that a gate accepted the work;
- that final acceptance can be claimed.

Only future `gate` writes final decisions.

## Relationship to existing hash and proof boundaries

### Artifact hash policy

`evals/specs/artifact-hash-policy.v0.1.md` defines canonical digest identity:

```text
sha256:<64 lowercase hex chars>
```

Referenced artifact verification must require expected digests to satisfy that policy.

Uppercase hex, bare hashes, placeholder values, unsupported algorithms, and empty values do not satisfy v0.1 verification.

### Exact-byte computation helper

`evals/specs/artifact-hash-computation-helper.v0.1.md` defines exact-byte digest computation over caller-provided bytes.

Referenced artifact verification may use exact-byte digest computation after an eligible file has been read.

Verification adds comparison and outcome semantics.

It must not change exact-byte hashing semantics.

### File IO artifact hashing helper

`evals/specs/file-io-artifact-hashing-boundary.v0.1.md` defines the file IO boundary for reading one explicit regular file under one explicit repo root and hashing exact file bytes.

Referenced artifact verification may use that helper to compute the observed digest.

It must not expand file IO behavior beyond that helper.

It must not infer refs, scan directories, silently follow symlinks, normalize bytes, or read outside the explicit root.

### Proofpack manifest digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines a manifest self-digest over deterministic in-memory proofpack manifest bytes.

Manifest self-digest is not referenced artifact verification.

A proofpack manifest may have a valid self-digest while a referenced artifact is missing, unreadable, mismatched, or unverified.

### Structural proofpack link/hash integrity

Current `punk-proof` behavior can check whether declared proofpack refs have matching digest entries by kind and ref.

That is structural integrity only.

It does not read referenced artifact bytes.

It does not prove that a declared digest matches persisted artifact content.

Referenced artifact verification is a later boundary layered after structural ref/hash integrity.

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` keeps proofpack as a post-gate provenance bundle that references evidence without deciding.

Referenced artifact verification may later provide evidence that proofpack artifact refs still match their declared hashes.

It must not make proofpack the gate, writer, storage surface, or acceptance authority.

## Allowed v0.1 inputs

A future verification operation should require explicit inputs:

- `artifact_kind` or equivalent context for the declared ref;
- `artifact_ref` as a validated repo-relative file ref;
- `expected_digest` as canonical `sha256:<64 lowercase hex chars>` metadata;
- `repo_root` as an explicit root already authorized by the caller or policy;
- optional policy context stating whether the artifact is required or optional.

Illustrative shape only:

```yaml
kind: output_artifact
ref: work/reports/2026-04-26-example.md
expected_digest: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
repo_root_ref: explicit_current_repo
required: true
```

This is not a schema file.

A future implementation must not derive `repo_root` from hidden process state unless a caller has explicitly resolved that process state into an authorized root.

## Ref and root boundary

For v0.1, locally verifiable referenced artifacts are repo-relative file refs only.

Allowed ref shape follows current artifact ref validation:

- slash-separated;
- no leading slash;
- no `.` or `..` path segments;
- no empty path segments;
- no home-directory shorthand such as `~`;
- no URL scheme;
- no platform-specific backslashes;
- no absolute filesystem paths;
- case-sensitive.

Runtime id refs such as `contract_123`, `run_123`, or `decision_123` are not locally file-verifiable unless a separate mapping artifact explicitly maps the id to an eligible repo-relative file ref.

URL refs are not locally file-verifiable in v0.1.

The verifier must not fetch network resources.

## File eligibility boundary

The first future implementation should verify only regular files.

It should report non-passing outcomes for:

- missing files;
- directories;
- symlinks;
- device files;
- sockets;
- pipes;
- platform-specific special files;
- unreadable files;
- refs outside the explicit repo root;
- refs that fail repo-relative validation.

Directory verification is out of scope for v0.1.

Archive canonicalization is out of scope for v0.1.

Recursive hashing is out of scope for v0.1.

Glob expansion is out of scope for v0.1.

## Symlink boundary

The v0.1 default is to reject symlinks as not verified.

A verifier must not silently follow symlinks.

It must not silently hash symlink target bytes.

It must not silently hash symlink path text.

If symlink support becomes necessary, it requires a later policy revision.

## Exact-byte comparison semantics

Verification compares canonical digest identity, not semantic content.

The verifier should:

1. validate the expected digest format;
2. validate the repo-relative artifact ref;
3. resolve the ref under the explicit repo root without escaping the root;
4. require an eligible regular file;
5. hash exactly the file bytes read;
6. compare the observed canonical digest string to the expected canonical digest string.

The verifier must not perform:

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

If the exact file bytes differ, the digest must differ.

That mismatch must remain visible.

## Outcome vocabulary

Recommended future outcome vocabulary:

- `verified` - expected digest is canonical, ref is valid, file is eligible, and observed digest matches expected digest;
- `digest_mismatch` - expected digest is canonical and file is eligible, but observed digest differs;
- `missing` - ref is valid but no file exists at the resolved location;
- `not_regular_file` - resolved location exists but is not a regular file;
- `symlink` - resolved location is a symlink and v0.1 policy refuses to follow it;
- `unreadable` - file appears eligible but cannot be read under local OS permissions or IO conditions;
- `invalid_ref` - artifact ref does not satisfy repo-relative ref policy;
- `invalid_expected_digest` - expected digest is missing, placeholder, unsupported, uppercase, malformed, or otherwise non-canonical;
- `unsupported_ref` - ref is a runtime id, URL, or other non-file ref without an explicit mapping artifact;
- `not_required` - policy says verification is not required for this artifact.

Only `verified` means byte identity matched for the local explicit file under the explicit root.

Every other outcome is non-passing for required artifact verification.

This vocabulary is not a schema file.

## Evidence record boundary

A future verifier may emit evidence like:

```yaml
kind: output_artifact
ref: work/reports/2026-04-26-example.md
expected_digest: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
observed_digest: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
outcome: verified
authority: evidence_only
```

For mismatch, the evidence should include both expected and observed digests when safe:

```yaml
outcome: digest_mismatch
expected_digest: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
observed_digest: sha256:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
```

It should not embed artifact bytes by default.

It should not embed raw file contents, secrets, provider payloads, environment dumps, credentials, or hidden local paths.

It should use repo-relative refs in repo-tracked artifacts.

## Required vs optional artifacts

Requiredness is a policy decision outside the verifier.

The verifier can report an outcome for a given artifact.

A future gate or proof policy may decide whether `missing`, `digest_mismatch`, or `not_required` blocks closure.

This boundary does not implement that decision.

It must not represent absent required verification as passing.

It must not represent executor-provided digest claims as verified unless Punk recomputes the digest under this policy or a trusted local validator supplies equivalent evidence.

## Privacy and sensitivity boundary

Hash metadata can leak information when content is small, guessable, proprietary, or sensitive.

Referenced artifact verification should avoid reading or hashing sensitive artifacts by default unless policy explicitly allows them.

The verifier must not read secrets, raw provider payloads, environment dumps, credentials, or private user text as a side effect of broad scanning.

The verifier must not scan directories to discover artifacts.

The caller must provide exact refs.

## Future helper responsibility

Recommended future Rust API shape:

```rust
pub fn verify_referenced_artifact_digest(
    repo_root: &RepoRoot,
    artifact_ref: &RepoRelativeArtifactRef,
    expected_digest: &ArtifactDigest,
) -> ReferencedArtifactVerification
```

The exact names may differ during implementation.

The boundary must remain:

- explicit root in;
- validated repo-relative ref in;
- canonical expected digest in;
- one explicit regular file read at most;
- exact file bytes hashed;
- expected vs observed digest compared;
- structured evidence out;
- no proofpack writing;
- no `.punk` runtime writes;
- no gate decisions;
- no acceptance claims;
- no CLI behavior by itself.

## Required future tests/evals

A future implementation should add tests or smoke eval coverage for:

### RAV-001: verified regular file

A regular file under an explicit repo root with matching expected digest returns `verified` and records the observed digest.

### RAV-002: digest mismatch

A regular file under an explicit repo root with non-matching expected digest returns `digest_mismatch` and records expected and observed digests.

### RAV-003: missing file

A valid repo-relative ref whose file is absent returns `missing`, not `verified`.

### RAV-004: directory and symlink refusal

Directories and symlinks return non-passing outcomes without reading nested files or following symlinks.

### RAV-005: invalid ref and invalid digest

Invalid repo-relative refs and malformed expected digests return explicit non-passing outcomes.

### RAV-006: exact byte identity

Files differing only by newline, whitespace, or Unicode byte representation produce different observed digests when their bytes differ.

### RAV-007: no authority side effects

Verification does not write proofpacks, `.punk` state, gate decisions, acceptance claims, schema files, or CLI output.

## Non-goals

This boundary does not define or implement:

- referenced artifact verification code;
- proofpack writer behavior;
- proofpack writer hash-policy integration;
- `.punk/proofs` storage;
- `.punk/decisions` storage;
- schema files;
- CLI commands;
- gate/eval/proof orchestration;
- acceptance claim generation;
- directory, archive, recursive, or semantic hashing;
- network fetching;
- provider/model/agent adapters;
- automation;
- `punk init`.

## Open follow-ups

- Implement a side-effect-bounded referenced artifact verification helper after this boundary is reviewed.
- Add smoke eval coverage for the future helper.
- Decide how proofpack writer preparation should reference verified artifact outcomes.
- Decide how future gate policy treats required verification outcomes.
- Decide whether runtime id refs need explicit mapping artifacts before local verification.
