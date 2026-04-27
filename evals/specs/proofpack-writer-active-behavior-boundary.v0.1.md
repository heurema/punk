# Proofpack writer active behavior boundary v0.1

Date: 2026-04-27
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the future active proofpack writer behavior boundary after side-effect-free preflight integration model coverage and before any proofpack writer implementation, `.punk/proofs` runtime storage activation, schema files, CLI behavior, proofpack referenced-ref verification integration, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the future active writer surface to:

```text
writer-ready preflight integration result + explicit storage/path/write policy
  -> selected write attempt behavior + non-authoritative operation evidence
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

Proofpack writer active behavior means the future side-effectful behavior that may attempt to persist a canonical proofpack artifact after a writer-ready preflight integration result.

For v0.1, active behavior is a boundary, not a writer.

It can define:

- the preconditions for any future write attempt;
- the explicit inputs a future writer must consume;
- the side effects a future writer may attempt only when separately selected;
- the required order of checks before side effects;
- idempotency and conflict behavior;
- rollback and failure visibility;
- index and `latest` pointer non-authority;
- operation-evidence production limits;
- evidence-versus-authority boundaries.

It cannot establish:

- that a proofpack was written;
- that `.punk/proofs` exists;
- that schema validation exists;
- that referenced artifact bytes were verified for proofpack refs;
- that operation evidence was persisted;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Active writer behavior must preserve that model.

It must not turn proofpack writing into a pre-gate checklist, a second tracker, a receipt authority, or a decision surface.

### Manifest digest and canonical artifact model

`evals/specs/proofpack-manifest-digest.v0.1.md` defines the manifest self-digest over exact deterministic in-memory renderer bytes.

`evals/specs/proofpack-writer-canonical-artifact-layout.v0.1.md` defines exact deterministic manifest renderer bytes as the v0.1 canonical artifact body.

A future active writer must treat canonical bytes and manifest self-digest as explicit inputs or explicit preflight outputs.

It must not re-render, normalize, wrap, compress, encrypt, sign, or otherwise transform canonical bytes during the write attempt unless a later selected schema/layout boundary explicitly changes the canonical byte surface.

### Target artifact ref policy

`evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md` selects `(proofpack_id, manifest_self_digest)` as the target artifact identity and renders logical refs shaped like:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

A future active writer must keep this target artifact ref logical and non-path.

It must not treat a target artifact ref as a host path, storage root, index row, `latest` pointer, service mirror id, or proof authority.

### Storage root and target path policy

`evals/specs/proofpack-writer-file-io-boundary.v0.1.md` and the target path policy model keep storage root refs, target artifact refs, and target path refs distinct.

A future active writer may attempt file IO only when all three are explicit and policy-accepted:

```text
storage root ref      -> where future write policy allows artifacts
logical artifact ref  -> what canonical proofpack artifact is intended
target path ref       -> where this attempt should write under selected policy
```

The target path ref is operational evidence, not proof truth.

The storage root ref is a future write-policy input, not acceptance authority.

### Preflight integration

`evals/specs/proofpack-writer-preflight-integration-boundary.v0.1.md` defines the side-effect-free preflight integration shape.

Current `punk-proof` has side-effect-free preflight integration model coverage.

A future active writer may attempt side effects only after an explicit `ready` preflight integration result.

`blocked` and `not_selected` results must fail closed and must not attempt canonical artifact writes, index updates, `latest` updates, operation-evidence persistence, or acceptance claims.

### Operation evidence

`evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md` defines future non-authoritative evidence about writer attempts.

Active writer behavior may produce operation evidence in memory or return it to a caller.

Persisting operation evidence is a separate future runtime behavior unless explicitly selected by a later bounded goal.

Operation evidence must not become gate authority, proof authority, receipt authority, schema authority, or acceptance authority.

### Proofpack referenced artifact verification

Current referenced artifact verification helpers are narrow evidence-only helpers for explicit file refs.

Active writer behavior must not imply proofpack referenced-ref verification integration.

If referenced artifact verification evidence is required by a future policy, it must be explicit input to preflight or a visible missing precondition.

A successful proofpack artifact write must not be used as evidence that referenced artifacts were read, hashed, or verified.

### Project Memory storage direction

`docs/product/PROJECT-MEMORY.md` separates append-only canonical artifacts from rebuildable indexes/views and optional service mirrors.

Future active proofpack writer behavior must follow the same split:

```text
append-only canonical proofpack artifact -> rebuildable indexes/views -> optional service mirror later
```

Indexes, `latest` pointers, dashboards, service mirrors, and CLI output may help inspectability.
They must not own proof truth.

## Required active writer precondition

A future active writer must receive or reference an explicit preflight integration result.

Required preflight properties before side effects:

- status is `ready`;
- proofpack id is explicit;
- manifest self-digest is explicit and valid;
- canonical artifact bytes are explicit or referenced by an explicit canonical artifact model;
- target artifact ref policy is accepted;
- logical target artifact ref is present;
- storage root ref is explicit if file IO is selected;
- target path ref is explicit if file IO is selected;
- target path policy is accepted;
- write policy is explicit;
- idempotency basis is explicit;
- temp/atomic policy is explicit;
- planned side effects are explicit;
- rollback/error visibility expectations are explicit;
- boundary notes are present.

If any required property is missing or rejected, the future writer must report `preflight_failed` or equivalent fail-closed evidence and must not attempt side effects.

## Explicit future inputs

A future active writer should receive explicit inputs.

Recommended conceptual inputs:

### Writer selection inputs

- active writer behavior was selected by a bounded goal or runtime policy;
- writer mode, such as `dry_run`, `write_artifact_only`, or `write_artifact_and_views`, if later selected;
- allowed side effects for this attempt;
- operation evidence return/persistence policy.

### Preflight inputs

- preflight integration result ref or in-memory value;
- preflight status;
- preflight blockers and diagnostics;
- boundary notes;
- selected planned side effects.

### Canonical artifact inputs

- proofpack id;
- canonical artifact layout;
- exact canonical artifact bytes;
- manifest self-digest over those bytes;
- non-canonical metadata kept outside canonical bytes.

### Target identity and path inputs

- logical target artifact ref;
- storage root ref;
- target path ref;
- target path policy status;
- path encoding policy, if separately selected later;
- host path resolution observation, if a future file IO implementation performs it.

### Write policy inputs

- overwrite policy;
- idempotency basis;
- existing-target comparison policy;
- temp/atomic policy;
- flush/sync policy, if selected;
- cleanup policy;
- index update policy, if selected;
- `latest` pointer policy, if selected.

### Evidence inputs

- expected operation outcome vocabulary;
- operation evidence shape or return path;
- operation evidence persistence policy, if separately selected later;
- referenced artifact verification evidence refs, if separately required;
- privacy/redaction policy status, if separately required.

Inputs must be explicit.

A future active writer must not silently infer missing values from chat transcripts, executor-local memory, IDE state, prompts or skills, provider/model metadata, mutable indexes, mutable `latest` pointers, service mirrors, dashboards, unlinked filesystem artifacts, current working directory assumptions, inferred `.punk` runtime state, or `punk init` side effects.

## Future active behavior flow

A future active writer should behave in a fail-closed order.

Recommended conceptual order:

1. confirm that active writer behavior is selected for this attempt;
2. receive the explicit preflight integration result;
3. reject `blocked` and `not_selected` preflight statuses before side effects;
4. verify that the `ready` preflight result still matches the explicit canonical artifact and target identity inputs;
5. confirm the selected side effects for this attempt;
6. resolve or receive target path observations only through selected file IO policy;
7. check existing target state only if file IO behavior is selected;
8. classify existing target as absent, matching, different, unreadable, or ambiguous;
9. return idempotent evidence for an existing matching canonical artifact;
10. fail closed for an existing different canonical artifact;
11. write canonical artifact bytes through selected temp/atomic policy;
12. make the canonical artifact available only after the selected write policy says the write is complete;
13. update indexes only if index behavior is explicitly selected;
14. update `latest` only if latest-pointer behavior is explicitly selected;
15. produce operation evidence with attempted side effects, completed side effects, failed side effects, and boundary notes;
16. avoid positive acceptance claims regardless of write success.

This order is conceptual.

It does not define schema, CLI, runtime storage, concrete filenames, host path canonicalization, or active writer implementation.

## Allowed future side effects when separately selected

A future active writer may attempt only side effects that are explicitly selected.

Potential side effects:

- canonical proofpack artifact write;
- temporary file creation for atomic write policy;
- flush/sync behavior, if selected;
- atomic move/rename into target path, if selected;
- cleanup of temporary files created by this attempt;
- index update, if selected;
- `latest` pointer update, if selected;
- operation evidence persistence, if a later goal selects it.

The canonical proofpack artifact write is the only side effect that may create canonical proof evidence.

Index updates, `latest` pointer updates, operation evidence persistence, logs, CLI output, dashboards, and service mirrors are non-canonical.

## Forbidden future side effects by default

A future active writer must not perform these side effects unless a later bounded goal explicitly selects them:

- create `.punk/proofs` as active runtime state;
- create schema files;
- expose CLI commands;
- write gate decisions;
- create acceptance claims;
- verify proofpack referenced artifact refs;
- hash broad referenced artifact trees;
- normalize artifact hashes or bytes;
- write service mirrors;
- update dashboards;
- invoke provider/model/agent adapters;
- run automation;
- run or require `punk init`.

## Idempotency and conflict behavior

A future active writer must make idempotency and conflict outcomes explicit.

Recommended future behavior:

| Existing target state | Future outcome | Required behavior |
|---|---|---|
| absent | `written` or `write_failed` | Attempt selected write policy or fail visibly. |
| exists with matching canonical bytes or selected content identity | `already_exists_matching` | Return idempotent evidence and do not rewrite silently. |
| exists with different canonical bytes or selected content identity | `conflict_existing_different` | Fail closed and do not overwrite silently. |
| unreadable or ambiguous | `aborted` or `write_failed` | Preserve ambiguity and do not claim artifact availability. |
| partial artifact detected | `partial_write_detected` | Mark non-authoritative and do not enable acceptance claims. |

Idempotency evidence is operation evidence only.

It is not proof of gate acceptance.

## Failure and rollback visibility

A future active writer must keep failures visible.

Required fail-closed behavior:

- preflight failure blocks all side effects;
- target path rejection blocks all side effects;
- target state ambiguity blocks overwrite;
- write failure must not claim canonical artifact availability;
- partial writes must be visible and non-authoritative;
- cleanup failure must be visible and must not be hidden by later success wording;
- index update failure must not invalidate an already complete canonical artifact;
- `latest` update failure must not invalidate an already complete canonical artifact;
- operation evidence persistence failure must not turn an unpersisted record into persisted truth;
- any unsupported side effect must be reported as not selected or aborted.

A future implementation should prefer explicit outcomes over lossy boolean success values.

## Operation evidence output

A future active writer should return operation evidence for every attempted behavior, including failed preflight before side effects.

Recommended evidence content:

- operation id;
- writer mode;
- preflight result ref or summary;
- proofpack id;
- manifest self-digest;
- logical target artifact ref;
- storage root ref;
- target path ref;
- selected side effects;
- attempted side effects;
- completed side effects;
- failed side effects;
- outcome;
- blockers or diagnostics;
- idempotency basis;
- existing-target observation summary;
- rollback/cleanup status;
- index/latest status, if selected;
- operation evidence persistence status, if selected later;
- boundary notes.

This evidence may support inspectability.

It must not become acceptance authority.

## Evidence versus authority

An active writer result may state that a future write attempt succeeded, failed, was idempotent, conflicted, or was aborted.

It must not state that work is accepted.

It must not approve a contract.

It must not close a goal.

It must not claim validators passed.

It must not claim referenced artifact bytes were verified unless explicit verification evidence was provided by a separately selected verifier.

It must not turn executor claims into proof.

It must not turn target path existence into proof.

It must not turn mutable indexes, `latest` pointers, dashboards, service mirrors, or CLI output into proof.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Hidden source-of-truth boundary

Active writer behavior must not create or depend on a hidden source of truth.

Not allowed as authority for writer inputs or outcomes:

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
- inferred `.punk` runtime state;
- implicit `punk init` assumptions.

Project truth must remain in explicit artifacts linked through goal, contract, receipt, eval/assessment, gate decision, and proofpack refs.

## Setup-neutral boundary

Active writer behavior must stay setup-neutral.

It must not require:

- a specific IDE;
- a specific local agent;
- a provider/model runner;
- a prompt or skill pack;
- a local daemon;
- a manual CLI ritual;
- `punk init`;
- `.punk/proofs` to exist before a separately selected runtime storage goal.

A future adapter or skill may help submit explicit inputs.
It must not become the source of truth.

## Acceptance checks for a future implementation

These checks define what a future implementation should demonstrate before active writer behavior can be trusted.

### PWPAB-001: explicit ready preflight required

The writer attempts side effects only after receiving an explicit `ready` preflight integration result.

`blocked` and `not_selected` results produce fail-closed evidence and no side effects.

### PWPAB-002: ref separation preserved

The writer keeps storage root refs, logical target artifact refs, and target path refs distinct.

It must not use logical target artifact refs as filesystem paths.

### PWPAB-003: canonical bytes preserved

The writer writes exact canonical artifact bytes selected by preflight and canonical artifact policy.

It must not hiddenly re-render, normalize, wrap, or transform bytes.

### PWPAB-004: selected side effects only

The writer attempts only side effects selected for this operation.

Index, `latest`, operation-evidence persistence, service mirrors, adapters, automation, and CLI behavior remain off unless separately selected.

### PWPAB-005: idempotency and conflict fail closed

Existing matching artifacts produce explicit idempotent evidence.

Existing different artifacts produce conflict evidence and are not overwritten silently.

### PWPAB-006: partial and rollback states visible

Write, cleanup, index, latest, and evidence-persistence failures remain visible and do not collapse into generic success.

### PWPAB-007: operation evidence is non-authoritative

Writer operation evidence records attempted and completed side effects but does not claim gate acceptance, proof authority, receipt authority, or positive acceptance.

### PWPAB-008: referenced-ref verification remains separate

The writer does not claim proofpack referenced artifact verification unless explicit verification evidence is supplied by a separately selected verifier.

### PWPAB-009: setup-neutral operation

The writer works from explicit data and does not require an IDE, provider, prompt, local agent, CLI ritual, `.punk` runtime state, or `punk init`.

## Non-goals

This boundary does not define:

- active proofpack writer implementation;
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

- Run an advisory Work Ledger Review before selecting active writer implementation, side-effect-free active behavior model, runtime storage, schema files, CLI behavior, proofpack referenced-ref verification integration, or additional eval guardrails.
- Define filename/path encoding if `.punk/proofs` or a concrete storage target is selected later.
- Define schema files only through a separate selected goal.
- Define operation evidence persistence separately from operation evidence production.
- Define proofpack referenced-ref verification integration separately from target artifact identity, preflight readiness, and write success.
- Define active proofpack writer implementation only after a separate bounded goal selects it.
- Keep `punk init` deferred until a bounded runtime setup goal selects it.

## Work Ledger next step

After this boundary, run another advisory Work Ledger Review before selecting active proofpack writer implementation, `.punk/proofs` activation, schema files, CLI behavior, proofpack referenced-ref verification integration, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
