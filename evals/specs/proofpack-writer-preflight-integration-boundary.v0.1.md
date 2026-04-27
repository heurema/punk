# Proofpack writer preflight integration boundary v0.1

Date: 2026-04-27
Status: proposed boundary
Authority: advisory/design

## Purpose

Define how a future proofpack writer preflight should integrate the existing side-effect-free writer models before any proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, proofpack referenced-ref verification integration, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the next future surface to preflight integration semantics:

```text
explicit proofpack + canonical artifact model + target artifact ref policy
+ target path policy + file IO plan policy inputs
  -> integrated writer preflight result
```

It does not activate proofpack writing.

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not activate `.punk/proofs`.

It does not expose CLI behavior.

It does not read the filesystem.

It does not write proofpacks.

It does not write operation evidence.

It does not verify proofpack referenced artifact refs.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer preflight integration means a side-effect-free composition step that evaluates whether all explicit model-level inputs required by a future writer are present, coherent, and policy-allowed before any filesystem side effect is attempted.

For v0.1, preflight integration is not a writer.

It can establish:

```text
these explicit inputs and policy models are sufficient for a future write attempt to be considered writer-ready
```

It cannot establish:

- that a proofpack was written;
- that `.punk/proofs` exists;
- that a target path exists;
- that target path parent directories exist;
- that schema validation exists;
- that referenced artifact bytes were verified for proofpack refs;
- that operation evidence was persisted;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Preflight integration must preserve that model.

It must not make proofpack a pre-gate checklist, duplicate receipt surface, second tracker, or second decision artifact.

### Proofpack manifest renderer and digest

Current `punk-proof` can render deterministic in-memory proofpack manifest bytes and compute a manifest self-digest over exact renderer bytes.

Preflight integration may require a canonical artifact model that exposes those bytes and digest metadata.

The manifest self-digest remains metadata for canonical proofpack artifact bytes.

It does not verify referenced artifacts and does not satisfy required linked artifact digest entries.

### Canonical artifact layout and model

`evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md` defines the future canonical artifact byte/layout boundary.

Current `punk-proof` can model canonical artifact bytes side-effect-free.

Preflight integration may consume that model as the canonical artifact input for target artifact identity and future idempotency checks.

It must not re-render or canonicalize through a second hidden format.

### Target artifact ref policy

`evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md` selects `(proofpack_id, manifest_self_digest)` as the target artifact identity and renders logical refs shaped like:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

Preflight integration must keep target artifact refs logical and non-path.

A missing target artifact ref is a preflight blocker before any filesystem side effect.

### Target path policy

Current `punk-proof` can model explicit target path policy without filesystem side effects.

Preflight integration may include a target path policy result.

It must keep:

```text
target artifact ref != target path ref
storage root ref != target path ref
target path ref != proof authority
```

A target path policy rejection is a preflight blocker before any write attempt.

### File IO boundary and plan model

`evals/specs/proofpack-writer-file-io-boundary.v0.1.md` defines future file IO semantics around explicit storage roots, target refs, target paths, write policy, idempotency, conflicts, temp/atomic handling, partial writes, indexes, and `latest` pointers.

Current `punk-proof` can model file IO plans side-effect-free.

Preflight integration may consume or produce a file IO plan model.

It must not perform file IO.

It must not inspect the host filesystem.

It must not turn a file IO plan into proof of artifact availability.

### Operation evidence boundary

`evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md` defines future writer operation evidence vocabulary and authority limits.

Preflight integration may specify the operation evidence that a future writer would produce on `planned_only` or `preflight_failed` outcomes.

It must not persist that evidence.

It must not make operation evidence a gate decision, receipt authority, schema validation authority, proof authority, or acceptance authority.

### Proofpack referenced artifact verification

Current referenced artifact verification helpers are narrow evidence-only helpers for explicit file refs.

Preflight integration must not claim proofpack referenced-ref verification integration.

If referenced artifact verification evidence is required by a future writer, it must be explicit input or a visible missing precondition.

It must not be inferred from target artifact refs, target paths, indexes, service mirrors, or executor claims.

## Conceptual preflight integration flow

A future integrated preflight should be ordered so later steps cannot invent earlier missing identity.

Recommended order:

1. validate proofpack structure and required linked refs;
2. validate declared digest entries and digest string policy;
3. render or receive canonical artifact model from exact in-memory manifest bytes;
4. compute or receive manifest self-digest from canonical artifact bytes;
5. derive target artifact ref from explicit `(proofpack_id, manifest_self_digest)` policy inputs;
6. receive explicit storage root ref, if a future file IO plan is selected;
7. receive explicit target path ref, if a future file IO plan is selected;
8. classify target path policy without filesystem access;
9. assemble write policy, idempotency basis, temp/atomic policy, planned side effects, and error/rollback visibility;
10. produce an integrated preflight result with status, blockers, and boundary notes;
11. only a later separately selected writer may attempt side effects after a writer-ready result.

This order is conceptual.

It does not define schema, CLI, runtime storage, or active writer behavior.

## Explicit future inputs

A future preflight integration should receive explicit inputs.

Recommended input groups:

### Proofpack identity and content

- proofpack value or proofpack id plus linked refs;
- gate decision ref;
- contract refs;
- run receipt refs;
- optional eval refs;
- optional event refs;
- optional output artifact refs;
- declared digest entries by kind and ref;
- created-at metadata;
- boundary notes.

### Canonical artifact inputs

- canonical artifact layout;
- canonical manifest bytes or canonical artifact model;
- manifest self-digest;
- non-canonical metadata, if any, kept outside canonical bytes.

### Target artifact identity inputs

- proofpack id;
- manifest self-digest;
- target artifact ref policy status;
- logical target artifact ref, if accepted.

### Future storage and path inputs

- storage root ref;
- target path ref;
- target path policy status;
- target path diagnostics, if rejected;
- optional storage policy flags.

### Future file IO policy inputs

- write policy;
- idempotency basis;
- temp/atomic policy;
- planned side effects;
- error/rollback visibility expectations.

### Optional verification inputs

- referenced artifact verification evidence refs or summaries, if a future goal selects them;
- privacy/redaction policy status;
- missing-verification explanation, if verification is intentionally deferred.

Inputs must be explicit.

A future preflight integration must not silently infer missing contract, receipt, eval, event, output, digest, storage, target artifact, target path, verification, privacy, or side-effect policy inputs from hidden process state.

## Integrated preflight result

A future integrated preflight result should be inspectable.

Recommended output fields:

- preflight status;
- proofpack id;
- manifest self-digest;
- target artifact ref;
- storage root ref, if selected;
- target path ref, if selected;
- target path policy status;
- selected write policy;
- selected idempotency basis;
- selected temp/atomic policy;
- selected planned side effects;
- missing preconditions;
- blockers;
- diagnostics;
- boundary notes;
- expected operation outcome if no write is attempted;
- expected operation evidence kind, without persistence.

For v0.1, this result is conceptual only.

It does not define a schema.

## Preflight statuses

Recommended status vocabulary for a future model:

| Status | Meaning | May a writer attempt filesystem side effects? |
|---|---|---|
| `ready` | All required explicit inputs and side-effect-free policy checks are satisfied. | Only if a separate active writer goal exists. |
| `blocked` | One or more required explicit inputs or policy checks are missing or rejected. | No. |
| `not_selected` | No active writer/storage behavior was selected. | No. |

These statuses are evidence about readiness.

They are not gate decisions.

They are not acceptance claims.

## Preflight blockers

A future preflight integration should keep blockers explicit.

Recommended blocker categories:

- missing proofpack id;
- invalid proofpack structure;
- missing required linked refs;
- missing declared digest entries;
- invalid digest shape;
- missing canonical artifact model;
- missing manifest self-digest;
- rejected target artifact ref policy;
- missing logical target artifact ref;
- missing storage root ref;
- missing target path ref;
- rejected target path policy;
- missing write policy;
- missing idempotency basis;
- missing temp/atomic policy;
- missing canonical artifact write selection;
- missing error/rollback visibility;
- missing boundary notes;
- unresolved privacy/redaction policy;
- missing referenced artifact verification evidence, if a future goal requires it.

Blockers must be visible.

A blocker must not be converted into a warning if it would make a future writer invent identity, paths, storage, hashes, verification, or authority.

## Evidence versus authority

Preflight integration may produce evidence that a future writer could use.

It must not decide whether work is accepted.

It must not approve a contract.

It must not close a goal.

It must not claim that validators passed.

It must not claim that referenced artifact bytes were verified unless explicit verification evidence was provided.

It must not turn executor claims into proof.

It must not turn target path availability into proof.

It must not turn mutable indexes, `latest` pointers, dashboards, service mirrors, or CLI output into proof.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Storage and path separation

Preflight integration must preserve three distinct refs:

```text
storage root ref:      explicit future storage policy input
target artifact ref:  logical proofpack identity metadata
target path ref:      explicit path-policy input under a storage root
```

The target artifact ref should be derived from proofpack id and manifest self-digest.

The target path ref should not define target artifact identity.

The storage root ref should not define proof truth.

A future implementation may encode a logical target artifact ref into a target path only through a separate explicit target path policy.

## Hidden source-of-truth boundary

Preflight integration must not create or depend on a hidden source of truth.

Not allowed as authority for missing preflight inputs:

- chat transcripts;
- executor-local memory;
- IDE state;
- prompts or skills;
- provider/model metadata;
- mutable indexes;
- mutable `latest` pointers;
- service mirrors;
- dashboards;
- unlinked filesystem artifacts;
- current working directory assumptions;
- inferred `.punk` runtime state.

Project truth must remain in explicit artifacts linked through goal, contract, receipt, eval/assessment, gate decision, and proofpack refs.

## Setup-neutral boundary

Preflight integration must stay setup-neutral.

It must not require:

- a specific IDE;
- a specific local agent;
- a provider/model runner;
- a prompt or skill pack;
- a local daemon;
- a manual CLI ritual;
- `punk init`;
- `.punk/proofs` to exist.

A future adapter or skill may help submit explicit inputs, but it must not become the source of truth.

## Side-effect boundary

This preflight integration boundary does not permit side effects.

Deferred side effects include:

- writing `.punk/proofs`;
- creating or updating proofpack indexes;
- writing `latest` pointers;
- writing operation evidence;
- writing schema files;
- writing gate decisions;
- creating acceptance claims;
- reading filesystem paths to determine target availability;
- hashing proofpack referenced artifact files as part of writer integration.

Any future side-effectful behavior must be selected through a separate bounded goal.

## Acceptance checks for a future implementation

These checks define what a future implementation should demonstrate before preflight integration can be trusted.

### PWPIF-001: explicit model inputs

The integration receives explicit proofpack, canonical artifact, target artifact ref policy, target path policy, and file IO policy inputs.

It must not infer them from hidden local state.

### PWPIF-002: target artifact ref remains logical

The integration keeps `proofpack:<proofpack_id>@<manifest_self_digest>` as logical non-path metadata.

It must not treat that value as a filesystem path.

### PWPIF-003: target path policy remains separate

The integration keeps target path refs separate from target artifact refs and storage root refs.

Rejected target path policy blocks future writes.

### PWPIF-004: blockers fail closed

Missing required preconditions produce visible blockers.

They must not be converted into implicit defaults.

### PWPIF-005: preflight evidence is not authority

A `ready` result may authorize a later writer attempt only if a separate active writer exists.

It must not claim gate acceptance or positive acceptance.

### PWPIF-006: no filesystem side effects

The integration does not read, write, canonicalize, or inspect host filesystem paths.

### PWPIF-007: setup-neutral operation

The integration works from explicit data and does not require an IDE, provider, prompt, local agent, CLI ritual, `.punk` runtime state, or `punk init`.

## Non-goals

This boundary does not define:

- active proofpack writer behavior;
- `.punk/proofs` activation;
- concrete filenames or path encoding;
- schema files;
- CLI commands;
- host filesystem path canonicalization;
- runtime storage;
- proofpack referenced-ref verification integration;
- operation evidence persistence;
- index or `latest` pointer format;
- service mirror behavior;
- gate decisions;
- acceptance claims;
- provider/model/agent adapters;
- automation;
- `punk init`.

## Open follow-ups

- Implement a side-effect-free preflight integration model only after this boundary is accepted.
- Define filename/path encoding if `.punk/proofs` is selected later.
- Define schema files only through a separate selected goal.
- Define proofpack referenced-ref verification integration separately from target artifact identity and preflight readiness.
- Define active proofpack writer behavior only after preflight integration model coverage exists.
- Keep `punk init` deferred until a bounded runtime setup goal selects it.

## Work Ledger next step

After this boundary, run another advisory Work Ledger Review before selecting preflight integration model implementation, active proofpack writer implementation, `.punk/proofs` activation, schema files, CLI behavior, proofpack referenced-ref verification integration, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
