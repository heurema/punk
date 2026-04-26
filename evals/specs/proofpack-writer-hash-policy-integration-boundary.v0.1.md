# Proofpack writer hash-policy integration boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define how a future proofpack writer may integrate Punk's artifact hash policy, declared artifact hashes, proofpack manifest self-digest, and referenced artifact verification outcomes without becoming runtime authority.

This is a design/spec artifact only.

It narrows the future integration surface to:

```text
explicit proofpack refs + declared artifact digest entries + optional verification evidence + deterministic manifest bytes -> inspectable writer hash-policy evidence
```

It does not activate proofpack writing.

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not activate `.punk/proofs`.

It does not expose CLI behavior.

It does not write proofpacks.

It does not verify proofpack referenced artifact refs.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer hash-policy integration means defining how a later writer should preserve hash-related evidence boundaries while preparing or writing a proofpack artifact.

For v0.1, integration is not a writer and not verification.

It can define:

- how declared artifact digest entries should be validated and preserved;
- how structural link/hash integrity should be checked before writing;
- how optional referenced artifact verification outcomes should be attached or summarized;
- how a manifest self-digest should identify the manifest bytes;
- how missing, invalid, mismatched, unreadable, and unverified evidence stays visible.

It cannot establish:

- that referenced artifact bytes were verified;
- that every required artifact exists;
- that a proofpack was written;
- that `.punk/proofs` exists;
- that schema validation exists;
- that validators passed;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Hash-policy integration must preserve that model.

It must not turn proofpack into a pre-gate checklist, duplicate receipt surface, second tracker, or decision artifact.

### Proofpack writer preparation

`evals/specs/proofpack-writer-preparation-boundary.v0.1.md` defines writer-ready conceptual inputs, outputs, preconditions, side effects, authority limits, and referenced artifact verification handoff.

This boundary narrows one part of that preparation:

```text
how future writer behavior should handle hashes and verification outcomes
```

It does not broaden writer preparation into runtime behavior.

### Artifact hash policy

`evals/specs/artifact-hash-policy.v0.1.md` defines canonical digest identity as:

```text
sha256:<64 lowercase hex chars>
```

Future writer integration should require declared digest entries to satisfy this shape.

Unsupported algorithms, uppercase hex, bare hashes, placeholder values, missing values, and wrong kind/ref entries must remain visible and must not satisfy required proof evidence.

### Exact-byte hashing and file IO hashing

`evals/specs/artifact-hash-computation-helper.v0.1.md` defines exact-byte hashing for caller-provided bytes.

`evals/specs/file-io-artifact-hashing-boundary.v0.1.md` defines a narrow helper for hashing one explicit regular file under one explicit repo root.

Future writer integration may consume digest evidence produced by these helpers.

It must not silently hash files, infer refs, scan directories, follow symlinks, normalize bytes, or read hidden local state.

### Referenced artifact verification

`evals/specs/referenced-artifact-verification-boundary.v0.1.md` defines evidence-only comparison between an expected digest and the observed digest for one explicit file.

Future writer integration may link or summarize that verification evidence.

It must not reinterpret, downgrade, or hide non-passing outcomes.

It must not make verification outcomes gate decisions or acceptance claims.

### Proofpack manifest digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines a manifest self-digest over exact deterministic in-memory renderer bytes.

Future writer integration may return or store that self-digest as metadata for the manifest bytes.

The manifest self-digest is not a substitute for declared artifact digest entries.

It does not verify referenced artifacts.

It must not be embedded into the manifest bytes before hashing unless a later recursive self-reference policy exists.

### Structural link/hash integrity

Current `punk-proof` can check whether proofpack refs have matching declared digest entries by kind and ref.

Future writer integration should treat structural link/hash integrity as a pre-write readiness check.

Structural integrity does not read files and does not prove referenced artifact bytes.

## Integration model

A future proofpack writer should separate four hash-related surfaces:

1. **Declared artifact digests** identify expected bytes for linked artifacts.
2. **Structural link/hash integrity** checks whether required refs have matching declared digest entries by kind and ref.
3. **Referenced artifact verification outcomes** record optional observed local byte checks against declared digests.
4. **Manifest self-digest** identifies the rendered proofpack manifest bytes.

These surfaces must not collapse into one boolean.

Illustrative conceptual flow:

```text
explicit proofpack refs
  -> required digest refs
  -> declared digest entries validated by artifact hash policy
  -> structural link/hash integrity report
  -> optional referenced artifact verification evidence
  -> deterministic manifest render
  -> manifest self-digest metadata
  -> future writer result with visible gaps
```

This is not a schema and not implementation.

## Declared artifact digest handling

A future writer should accept only explicit declared digest entries.

For each required ref, it should preserve:

- artifact kind;
- artifact ref;
- declared digest string;
- digest policy status;
- whether the declared digest satisfies the required kind/ref;
- whether the artifact is required or optional, if a later policy supplies that fact.

Recommended future status vocabulary:

- `declared_valid` - digest is canonical and matches the required kind/ref;
- `declared_missing` - required digest entry is absent;
- `declared_invalid_format` - digest-like value is present but not canonical v0.1;
- `declared_unsupported_algorithm` - digest uses a non-v0.1 algorithm;
- `declared_wrong_kind_or_ref` - digest exists for another kind/ref;
- `declared_unverified` - digest was provided by an executor, adapter, or external tool and was not recomputed by Punk or an allowed local validator.

Only `declared_valid` can satisfy the structural declaration requirement.

`declared_valid` still does not prove the referenced artifact bytes.

## Referenced artifact verification outcome handling

A future writer may receive verification evidence for declared artifact refs.

It should preserve the original outcome and expected/observed digest relationship when safe.

Recommended handling:

| Outcome | Future writer integration behavior | Authority boundary |
|---|---|---|
| `verified` | May link or summarize as matching observed local byte evidence. | Evidence only; not a gate decision or acceptance claim. |
| `digest_mismatch` | Must remain visible with expected and observed digest when safe. | Non-passing for required verification unless later gate policy says otherwise. |
| `missing` | Must remain visible as absent local artifact evidence. | Must not be converted to pass or hidden by manifest self-digest. |
| `not_regular_file` | Must remain visible as ineligible local artifact evidence. | Must not trigger recursive hashing or directory hashing. |
| `symlink` | Must remain visible under v0.1 symlink refusal. | Must not silently follow the link. |
| `read_denied` | Must remain visible as unreadable evidence. | Must not invent observed digest. |
| `read_error` | Must remain visible as unreadable or unavailable evidence. | Must not invent observed digest. |
| `invalid_repo_root` | Must remain visible as invalid local verification setup. | Must not use current working directory as hidden authority. |
| `outside_repo_root` | Must remain visible as rejected path escape. | Must not hash outside the explicit root. |
| `invalid_ref` | Must remain visible if a later verifier reports it. | Must not normalize into another ref. |
| `invalid_expected_digest` | Must remain visible if a later verifier reports it. | Must not reinterpret malformed digest as verified. |
| `unsupported_ref` | Must remain visible for runtime ids, URLs, or unmapped refs. | Must not fetch network resources or infer mapping. |
| `not_required` | May be recorded as policy-not-required when explicitly supplied. | Must not imply required verification passed. |
| `unverified` | Must remain visible when no local recomputation evidence exists. | Executor claims are not proof. |

Only `verified` means the expected digest matched the observed digest for an explicit eligible local file under an explicit root.

Even `verified` remains evidence, not authority.

## Manifest self-digest handling

A future writer may compute or return a manifest self-digest after deterministic manifest rendering.

The self-digest should identify exactly the manifest bytes returned by the renderer.

It should be stored outside the hashed manifest bytes or in a wrapper/index artifact until a later policy defines recursive self-reference.

The self-digest must not satisfy missing digest entries for:

- gate decision;
- contract;
- run receipt;
- eval report;
- event range;
- output artifact;
- rule set;
- work report;
- other referenced artifacts.

A proofpack can have a valid manifest self-digest while referenced artifacts are missing, mismatched, unreadable, or unverified.

## Future writer preconditions

A future proofpack writer should not write a canonical proofpack unless hash-policy integration preconditions are explicit.

Recommended future preconditions:

1. required proofpack refs are explicit;
2. required digest refs are derived from explicit proofpack refs;
3. declared digest entries are present for required refs or missing status is explicit;
4. declared digest strings are validated against artifact hash policy;
5. structural link/hash integrity report is produced;
6. optional referenced artifact verification outcomes are linked or summarized without reinterpretation;
7. manifest bytes are rendered deterministically in memory;
8. manifest self-digest is computed from exact renderer bytes;
9. privacy/redaction policy has been applied upstream or marked unresolved;
10. missing, mismatched, unreadable, unverified, and optional evidence states are preserved.

These are preparation constraints, not active checks today.

## Failure and partial-proofpack policy

Required hash gaps should fail closed unless a later partial-proofpack policy explicitly permits a non-authoritative partial artifact.

A future writer must not silently invent:

- artifact refs;
- artifact kinds;
- declared digests;
- observed digests;
- verification outcomes;
- manifest self-digest values;
- storage paths;
- timestamps;
- policy approvals.

If partial proofpacks are later allowed, the partial state must be explicit and must not enable positive acceptance claims.

## Privacy and redaction boundary

Hashes and verification metadata can leak information.

Future writer integration must not include by default:

- secrets;
- credentials;
- raw provider payloads;
- private prompts;
- environment dumps;
- private user text;
- large generated artifact bodies;
- unredacted logs;
- hidden absolute local paths.

A future privacy/redaction boundary must decide which refs, hashes, verification outcomes, and manifest metadata are safe to persist or publish.

## Setup-neutral boundary

Proofpack writer hash-policy integration must stay setup-neutral.

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

Future integrations may consume explicit evidence produced in the user's environment, but that environment is substrate, not authority.

## Authority boundary

A future writer may create provenance artifacts.

It must not decide.

It must not approve work.

It must not close a contract.

It must not transform failed or missing validators into passing evidence.

It must not turn executor claims into proof.

It must not turn a valid manifest self-digest into referenced artifact verification.

It must not create positive acceptance claims by itself.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Non-goals

This v0.1 boundary does not define or implement:

- proofpack writer implementation;
- `.punk/proofs` storage layout;
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

### PWHI-001: declared digest policy preserved

Canonical `sha256:<64 lowercase hex>` declared digests satisfy declaration format; uppercase, bare, placeholder, unsupported, missing, and wrong kind/ref entries remain visible and non-passing for required declarations.

### PWHI-002: structural link/hash integrity remains separate from byte verification

A proofpack with complete declared digests can pass structural link/hash integrity while still having missing or unverified referenced artifact byte evidence.

### PWHI-003: verification outcome propagation

`verified`, `digest_mismatch`, `missing`, `not_regular_file`, `symlink`, `read_denied`, `read_error`, `outside_repo_root`, and `unverified` outcomes are preserved without reinterpretation.

### PWHI-004: manifest self-digest is not artifact verification

Computing a manifest self-digest must not satisfy missing declared artifact digests or turn unverified referenced artifacts into verified ones.

### PWHI-005: no hidden file IO or setup dependency

Writer hash-policy integration must not scan directories, infer refs, follow symlinks, use current working directory as hidden authority, require `punk init`, require network, or depend on provider/model/agent setup.

### PWHI-006: no runtime authority side effects

Integration must not write `.punk` state, schema files, gate decisions, acceptance claims, proofpacks, CLI output, adapters, automation, or provider/model runners unless separately selected.

## Open follow-ups

- proofpack writer implementation boundary;
- `.punk/proofs` storage and schema boundary;
- proofpack writer CLI surface, if any;
- proofpack referenced-ref verification integration implementation;
- privacy/redaction policy for proof artifacts and verification metadata;
- partial proofpack policy;
- writer receipt or operation evidence shape;
- acceptance claim wiring after gate and proof are linked;
- remote attestation, signing, or transparency-log policy;
- possible future `punk init` interaction with proof storage.

## Next bounded step

After this boundary, run another advisory Work Ledger Review before selecting proofpack writer implementation, runtime storage, schema, CLI, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
