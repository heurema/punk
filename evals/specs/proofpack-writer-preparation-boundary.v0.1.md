# Proofpack writer preparation boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define what Punk means by proofpack writer preparation before any proofpack writer implementation, `.punk/proofs` storage, schema files, CLI behavior, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the next future surface to preparation semantics:

```text
existing proofpack model + linked evidence refs + declared hashes + optional verification evidence -> writer-ready plan/boundary
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

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer preparation means defining the minimum conditions and explicit handoff shape that a later writer implementation must satisfy before it can safely persist a proofpack artifact.

For v0.1, preparation is not a writer.

It can establish:

```text
these inputs, refs, digest expectations, ordering rules, and authority boundaries are ready to guide a future writer implementation
```

It cannot establish:

- that a proofpack was written;
- that `.punk/proofs` exists;
- that schema validation exists;
- that referenced artifact bytes were verified for proofpack refs;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Writer preparation must preserve that model.

It must not make proofpack a pre-gate checklist, duplicate receipt surface, or second decision artifact.

### Proofpack manifest renderer

Current `punk-proof` can render a deterministic in-memory proofpack manifest.

Writer preparation may treat that renderer as the future source of manifest bytes.

It must not require a writer to parse and re-serialize the manifest in a second canonical format.

### Proofpack manifest digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines a self-digest over exact in-memory renderer bytes.

Writer preparation may require a future writer to record or return that self-digest.

The manifest self-digest remains metadata for the manifest bytes.

It does not verify referenced artifacts and does not satisfy required artifact digest entries.

### Structural link/hash integrity

Current `punk-proof` can check whether declared proofpack refs have matching digest entries by kind and ref.

Writer preparation may require this structural check to be complete before writing.

Structural link/hash integrity does not read files and does not prove referenced artifact bytes.

### Artifact hash policy and exact-byte hashing

`evals/specs/artifact-hash-policy.v0.1.md` defines canonical digest shape.

`evals/specs/artifact-hash-computation-helper.v0.1.md` defines exact-byte hashing.

Writer preparation may require all declared artifact hashes and manifest digests to use canonical digest metadata.

It must not normalize hashes or bytes.

### File IO artifact hashing

`evals/specs/file-io-artifact-hashing-boundary.v0.1.md` defines a narrow helper for hashing one explicit regular file under one explicit repo root.

Writer preparation may refer to this as a possible future source of observed digest evidence.

It must not broaden file IO behavior, infer refs, scan directories, follow symlinks, or write runtime state.

### Referenced artifact verification

`evals/specs/referenced-artifact-verification-boundary.v0.1.md` defines evidence-only comparison between an expected digest and the observed digest for one explicit file.

Writer preparation may require that future writer integration preserve referenced artifact verification outcomes separately from proofpack writing.

Referenced artifact verification evidence may be linked or summarized later.

It must not become writer authority, gate authority, or acceptance authority.

## Writer-ready conceptual inputs

A future proofpack writer should receive explicit inputs.

Recommended conceptual inputs:

- `proofpack_id`;
- `schema_version` or boundary version;
- `gate_decision_ref`;
- one or more `contract_refs`;
- zero or more `run_receipt_refs`;
- zero or more `eval_refs`;
- zero or more `event_refs` or an event range/root concept;
- zero or more `output_artifact_refs`;
- declared artifact digest entries by kind and ref;
- `created_at`;
- `boundary_notes`;
- optional privacy/redaction flags;
- optional referenced artifact verification outcome refs or summaries.

Inputs must be explicit.

A future writer must not silently infer missing contract, receipt, eval, event, output, or digest refs from hidden process state.

If a later runtime uses Project Memory or `.punk/` state to discover inputs, that discovery must be a separate explicit step whose result is inspectable before writing.

## Writer-ready conceptual outputs

A future writer may eventually produce explicit outputs.

Recommended conceptual outputs:

- proofpack manifest bytes or a proofpack artifact ref;
- proofpack manifest self-digest;
- structural link/hash integrity report or status;
- referenced artifact verification evidence refs or status summary, if supplied;
- writer receipt or operation evidence, if a later boundary defines it;
- storage location, if `.punk/proofs` or another store is later activated;
- boundary notes explaining what was not verified.

For v0.1 preparation, these outputs are conceptual only.

This spec does not define a schema or file layout.

## Minimum writer preconditions

A future writer implementation should not write a canonical proofpack unless all required preconditions are explicit.

Recommended future preconditions:

1. proofpack values are structurally valid;
2. required refs are present;
3. required declared digest entries exist for linked artifacts;
4. declared digest strings satisfy artifact hash policy;
5. manifest bytes are rendered deterministically in memory;
6. manifest self-digest is computed from exact renderer bytes;
7. gate decision ref is present for canonical post-gate proofpacks;
8. boundary notes record missing or intentionally deferred verification;
9. privacy/redaction policy has been applied by an explicit upstream step or marked unresolved;
10. storage target is explicit and policy-allowed.

These are preparation constraints, not active checks today.

## Authority boundary

A future proofpack writer may create provenance artifacts.

It must not decide.

It must not approve work.

It must not close a contract.

It must not transform failed or missing validators into passing evidence.

It must not turn executor claims into proof.

It must not create positive acceptance claims by itself.

It must not make a mutable `latest` pointer canonical truth.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Side-effect boundary

This preparation boundary does not permit side effects.

Future writer implementation must specify side effects explicitly before activation.

Deferred side effects include:

- writing `.punk/proofs`;
- creating or updating proofpack indexes;
- writing `latest` pointers;
- emitting CLI output;
- writing schemas;
- exporting remote attestations;
- signing proofpacks;
- writing transparency log entries;
- updating acceptance claims.

Until those are separately selected, writer preparation stays docs/spec-only.

## Referenced artifact verification handoff

A future writer may need to include referenced artifact verification evidence.

For v0.1 preparation, the safe handoff is:

```text
declared artifact ref + declared digest + optional observed verification outcome -> proofpack-linked evidence
```

The writer should not recompute or reinterpret verification unless a later integration boundary explicitly selects that behavior.

If verification evidence is absent, unavailable, stale, or failed, the writer must preserve that fact rather than silently passing it.

Recommended future vocabulary alignment:

- `verified` remains evidence that expected and observed digests matched;
- `digest_mismatch` remains visible;
- `missing`, `not_regular_file`, `symlink`, `read_denied`, `read_error`, `invalid_repo_root`, and `outside_repo_root` remain non-passing evidence states;
- unverified optional artifacts must be distinguishable from verified artifacts.

## Privacy and redaction boundary

Proofpack writing can expose sensitive metadata.

Writer preparation must assume that refs, hashes, manifest bytes, event ranges, and boundary notes may leak information.

A future writer must not include by default:

- secrets;
- credentials;
- raw provider payloads;
- private prompts;
- environment dumps;
- private user text;
- large generated artifact bodies;
- unredacted logs.

Privacy/redaction policy remains a future explicit boundary.

## Failure and missing-data policy

A future writer should fail closed or produce explicit non-authoritative evidence when required inputs are missing.

It must not silently invent:

- contract refs;
- decision refs;
- receipt refs;
- eval refs;
- event refs;
- output artifact refs;
- declared digests;
- verification outcomes;
- timestamps;
- storage paths.

If a future writer supports partial proofpacks, the partial state must be explicit and must not enable positive acceptance claims.

## Non-goals

This v0.1 boundary does not define:

- proofpack writer implementation;
- `.punk/proofs` storage layout;
- schema files;
- CLI commands;
- runtime proof storage;
- proofpack referenced-ref verification integration;
- hash normalization;
- signing;
- transparency logs;
- remote attestation exports;
- final acceptance claim schema;
- gate implementation;
- provider/model/agent adapters;
- automation;
- `punk init`.

## Open follow-ups

- proofpack writer schema or manifest artifact format;
- `.punk/proofs` storage boundary;
- proofpack writer CLI surface, if any;
- proofpack referenced-ref verification integration;
- hash normalization policy, if ever needed;
- privacy/redaction policy for proof artifacts;
- partial proofpack policy;
- writer receipt or operation evidence shape;
- acceptance claim wiring after gate and proof are linked;
- remote attestation, signing, or transparency-log policy;
- possible future `punk init` interaction with proof storage.

## Next bounded step

After this proofpack writer preparation boundary, run another advisory Work Ledger Review before selecting implementation, writer hash-policy integration, runtime storage, schema, CLI, or gate/eval/proof orchestration work.
