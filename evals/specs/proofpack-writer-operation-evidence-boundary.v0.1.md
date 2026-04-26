# Proofpack writer operation evidence boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the future proofpack writer operation evidence boundary before any proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the future writer evidence surface to:

```text
writer-ready inputs + storage/hash preconditions + attempted side effects -> explicit non-authoritative operation evidence
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

It does not write indexes or `latest` pointers.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer operation evidence means future structured evidence about a writer attempt.

It records what the writer was asked to do, which inputs and target were explicit, which side effects were attempted, which side effects completed, which side effects failed, and which outcome was produced.

It is evidence about the writer operation.

It is not proof authority.

It cannot decide work, approve work, validate scope by itself, satisfy a gate decision, or create positive acceptance claims.

For v0.1, the boundary may define:

- operation outcome vocabulary;
- conceptual evidence fields;
- preconditions that must be explicit before writing;
- side-effect reporting expectations;
- idempotency and conflict reporting expectations;
- partial-write and failed-update reporting expectations;
- hidden-source-of-truth and setup-neutral limits.

It cannot establish:

- that a proofpack writer exists;
- that `.punk/proofs` exists;
- that a proofpack was written;
- that a schema exists or validated successfully;
- that referenced artifact bytes were verified;
- that an index or `latest` pointer is canonical truth;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

Operation evidence must not turn proofpack writing into a decision surface.

A future operation evidence record may say that a writer wrote or attempted to write a proofpack artifact.

It must not say the work is accepted.

### Proofpack writer preparation

`evals/specs/proofpack-writer-preparation-boundary.v0.1.md` defines future writer-ready inputs, conceptual outputs, preconditions, side effects, referenced artifact verification handoff, and authority limits.

This boundary narrows the `writer receipt or operation evidence` follow-up from that preparation boundary.

It does not broaden preparation into runtime behavior.

### Proofpack writer hash-policy integration

`evals/specs/proofpack-writer-hash-policy-integration-boundary.v0.1.md` defines how a future writer should preserve declared artifact digests, structural link/hash integrity, referenced artifact verification outcomes, and manifest self-digest metadata.

Operation evidence must preserve hash-policy status instead of hiding it behind storage success.

Missing, mismatched, unreadable, invalid, unsupported, unverified, or optional referenced evidence states must remain visible.

### Proofpack writer storage/schema

`evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md` defines append-only canonical proofpack artifact expectations, wrapper/index/latest pointer limits, schema/file-layout deferral, idempotency/conflict behavior, and storage failure policy.

This operation evidence boundary defines how a future writer should report those storage results without making storage results authoritative.

Storage success is evidence of a side effect.

It is not gate acceptance.

### Manifest self-digest

`evals/specs/proofpack-manifest-digest.v0.1.md` defines a manifest self-digest over exact deterministic in-memory renderer bytes.

Operation evidence may carry or reference the manifest self-digest used for the write attempt.

That digest must not be embedded into hashed manifest bytes unless a later recursive self-reference policy exists.

The self-digest also must not satisfy referenced artifact digest requirements.

### Project Memory storage direction

`docs/product/PROJECT-MEMORY.md` separates repo-tracked authority from runtime events and rebuildable indexes/views.

Future operation evidence should follow that authority split.

If operation evidence is later persisted, it must not become a hidden live tracker or a second source of truth.

## Operation evidence is distinct from nearby artifacts

Operation evidence is not the same as:

- proofpack artifact;
- proofpack manifest bytes;
- proofpack manifest self-digest;
- gate decision;
- run receipt;
- eval report;
- acceptance claim;
- schema validation report;
- proofpack index;
- mutable `latest` pointer;
- executor claim;
- chat transcript;
- IDE state.

A future writer may link these artifacts or statuses, but it must keep their authority separate.

## Future outcome vocabulary

Recommended future outcome values:

```text
planned_only
written
already_exists_matching
conflict_existing_different
preflight_failed
write_failed
partial_write_detected
index_update_failed
latest_pointer_update_failed
aborted
```

### `planned_only`

The writer prepared a plan but did not attempt a write.

This can be useful for dry-run or preflight UX, but it does not create a proofpack artifact.

### `written`

The canonical proofpack artifact write completed according to the selected future write policy.

This means artifact availability evidence exists.

It still does not mean gate acceptance or positive acceptance.

### `already_exists_matching`

The target canonical artifact already existed with matching bytes or matching selected content identity.

This is idempotent evidence.

It must not hide that no new artifact was written during this attempt.

### `conflict_existing_different`

The target canonical artifact already existed but differed from the bytes or content identity the writer intended to write.

This must fail closed and must not overwrite the canonical artifact silently.

### `preflight_failed`

A required precondition was missing or failed before any canonical write was attempted.

The evidence must identify the missing or failed precondition without inventing values.

### `write_failed`

A canonical artifact write was attempted and failed before a complete canonical artifact was available.

The evidence must preserve the failure and must not claim artifact availability.

### `partial_write_detected`

A partial or ambiguous write state was detected.

The evidence must mark the state non-authoritative and must not enable positive acceptance claims.

### `index_update_failed`

The canonical artifact write or idempotent availability succeeded, but an index/view update failed.

The canonical artifact status and index status must be reported separately.

A failed index update must degrade inspect UX only.

### `latest_pointer_update_failed`

The canonical artifact write or idempotent availability succeeded, but a mutable `latest` pointer update failed.

The pointer must remain non-canonical.

A failed pointer update must not change proof truth.

### `aborted`

The operation was intentionally aborted after planning or preflight.

The evidence must say whether any side effects had already been attempted.

## Authority of outcomes

Only `written` and `already_exists_matching` can represent future canonical artifact availability.

Even those outcomes remain operation evidence only.

They do not prove:

- referenced artifact bytes match declared digests;
- validators passed;
- scope was preserved;
- a gate accepted the work;
- positive acceptance may be claimed.

Outcomes such as `index_update_failed` and `latest_pointer_update_failed` may coexist with canonical artifact availability if the future evidence shape includes a primary artifact status and separate side-effect statuses.

A later schema may model this as one primary outcome plus side-effect details, or as separate fields.

The selected schema must preserve the distinction.

## Recommended future evidence fields

A future operation evidence record should be explicit and structured.

Recommended conceptual fields:

```text
operation_id
operation_kind
proofpack_id
attempted_at
input_refs
target_ref
manifest_self_digest
structural_link_hash_status
referenced_artifact_verification_summary
storage_action
outcome
side_effects_attempted
side_effects_completed
side_effects_failed
idempotency_key
conflict_ref
boundary_notes
authority
```

Where:

- `operation_id` identifies the writer attempt or dry-run;
- `operation_kind` distinguishes dry-run, write, rewrite-denied, repair, index-update, or other future operation kinds;
- `proofpack_id` identifies the intended proofpack value or artifact identity;
- `attempted_at` is explicit future evidence metadata and must not be invented;
- `input_refs` list explicit contract, receipt, eval, decision, event, output, digest, and verification refs used by the writer;
- `target_ref` identifies the intended canonical artifact target;
- `manifest_self_digest` records or references the digest for exact rendered manifest bytes;
- `structural_link_hash_status` records or references structural link/hash integrity status;
- `referenced_artifact_verification_summary` is optional and must preserve missing, failed, mismatched, unverified, or optional states;
- `storage_action` distinguishes no-write, create, idempotent-match, conflict, repair, or future selected actions;
- `outcome` uses selected vocabulary;
- `side_effects_attempted` lists attempted canonical artifact, temp artifact, index, latest pointer, operation-evidence, event/log, or remote side effects;
- `side_effects_completed` lists completed side effects;
- `side_effects_failed` lists failed side effects with reason codes or refs;
- `idempotency_key` or an equivalent field explains how repeated attempts are compared;
- `conflict_ref` identifies conflicting existing artifact state when present;
- `boundary_notes` record unresolved or intentionally deferred evidence;
- `authority` should be `evidence_only` or an equivalent non-authority marker.

These fields are conceptual only.

This boundary does not create a schema file.

## Future writer preconditions

A future writer should not produce `written` or `already_exists_matching` evidence unless required preconditions are explicit.

Recommended future preconditions:

1. writer-ready proofpack inputs are explicit;
2. required proofpack refs are present;
3. required declared digest entries exist or missing status is explicit;
4. declared digest strings satisfy artifact hash policy;
5. structural link/hash integrity status is available;
6. optional referenced artifact verification evidence is represented without reinterpretation;
7. manifest bytes are rendered deterministically in memory;
8. manifest self-digest is computed from exact renderer bytes;
9. storage root and target artifact ref are explicit and policy-allowed;
10. overwrite/idempotency policy is explicit;
11. failure and rollback policy is explicit;
12. index/latest pointer behavior is explicit and non-authoritative, if selected;
13. privacy/redaction policy has been applied upstream or marked unresolved;
14. operation evidence target is explicit, if operation evidence is persisted later.

These are preparation constraints, not active checks today.

## Side-effect reporting boundary

Future writer operation evidence should report side effects separately.

Possible future side effects include:

- canonical proofpack artifact write;
- temporary artifact write;
- temporary artifact cleanup;
- wrapper metadata write;
- proofpack index update;
- mutable `latest` pointer update;
- operation evidence write;
- event or log write, if later selected;
- remote attestation, signing, or transparency-log export, if later selected.

Each side effect should be visible as attempted, completed, failed, skipped, or not selected.

A future writer must not let a successful side effect hide a failed side effect.

A future writer must not let a failed index, `latest` pointer, operation evidence, or export side effect invalidate an already written canonical append-only proofpack artifact unless a later selected policy explicitly says so.

## Idempotency and conflict boundary

Future canonical proofpack artifacts should be append-only.

A future writer should not overwrite an existing canonical proofpack artifact silently.

Recommended future behavior:

- matching existing canonical artifact bytes or selected content identity produce `already_exists_matching` evidence;
- different existing canonical artifact bytes or selected content identity produce `conflict_existing_different` evidence;
- conflict evidence must identify the target and comparison basis without exposing hidden local paths;
- conflict evidence must not mutate the existing canonical artifact;
- idempotent evidence must not claim a new write happened.

The comparison basis must be explicit.

A future implementation must not infer idempotency from mutable indexes, `latest` pointers, chat state, or executor-local memory.

## Partial-write and failure boundary

Future proofpack writing should fail closed when required operation preconditions are missing.

A future writer must not silently invent:

- operation ids;
- proofpack ids;
- storage roots;
- artifact paths;
- schema versions;
- manifest bytes;
- manifest self-digests;
- gate decision refs;
- contract refs;
- receipt refs;
- eval refs;
- event refs;
- output refs;
- declared digests;
- verification outcomes;
- timestamps;
- acceptance claims.

If a canonical artifact write fails, operation evidence must not claim artifact availability.

If a partial artifact is visible, operation evidence must mark it as partial or ambiguous and non-authoritative.

If partial proofpack artifacts are later allowed, partial state must be explicit and must not enable positive acceptance claims.

## Index and latest pointer boundary

Indexes and `latest` pointers are future convenience views only.

They must be rebuildable.

They must not own proof truth.

Operation evidence should distinguish:

```text
canonical artifact status != index status != latest pointer status
```

A missing, stale, inconsistent, or failed index/latest pointer update may affect inspect UX.

It must not change gate decision truth, proofpack artifact truth, or acceptance truth.

## Hidden source-of-truth boundary

Future operation evidence must not create a hidden source of truth.

Not allowed as operation authority:

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
- unstructured log text.

If a future writer uses runtime state to discover inputs, that discovery result must be an explicit input ref or evidence ref before writing.

## Privacy and retention boundary

Operation evidence can expose sensitive refs, storage targets, hashes, timestamps, event ranges, failure details, and boundary notes.

A future implementation must not include by default:

- secrets;
- credentials;
- raw provider payloads;
- private prompts;
- environment dumps;
- private user text;
- large generated artifact bodies;
- unredacted logs;
- hidden absolute local paths.

Retention, redaction, deletion, signing, transparency-log, and publication policies remain future explicit boundaries.

## Setup-neutral boundary

Proofpack writer operation evidence must stay setup-neutral.

It must not require:

- a specific IDE;
- a specific executor;
- a specific provider or model;
- prompt or skill setup;
- global config;
- background services;
- network access;
- `.punk/proofs` storage layout activation;
- schema files;
- CLI commands;
- `punk init`.

A future local or service-backed implementation may exist later, but it must not make a user's setup the authority.

## Non-goals

This v0.1 boundary does not define:

- proofpack writer implementation;
- `.punk/proofs` storage activation;
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

### PWOE-001: preflight failure stays visible

Missing required inputs must produce explicit non-authoritative evidence and must not write a canonical proofpack artifact.

### PWOE-002: write success is not acceptance

A successful canonical artifact write must not write a gate decision, create an acceptance claim, or mark work accepted.

### PWOE-003: idempotent existing artifact is explicit

A matching existing canonical artifact must produce idempotent evidence rather than silently passing as a new write.

### PWOE-004: conflicting existing artifact fails closed

A different existing canonical artifact must report conflict and must not be overwritten silently.

### PWOE-005: partial and side-effect failures remain visible

Partial writes, index update failures, and `latest` pointer failures must be represented separately from canonical artifact status.

### PWOE-006: no hidden source of truth

Operation evidence must be derived from explicit refs and selected runtime state, not chat state, executor-local memory, mutable indexes, or unlinked files.

### PWOE-007: no runtime activation implied

Docs and reports that reference this boundary must not describe `.punk/proofs`, schemas, CLI commands, or proofpack writing as active behavior.

## Open follow-ups

- proofpack writer implementation;
- actual `.punk/proofs` storage activation;
- proofpack schema files;
- proofpack writer CLI surface, if any;
- operation evidence schema and persistence policy;
- atomic write and rollback policy;
- proofpack index and latest-pointer policy;
- proofpack referenced-ref verification integration implementation;
- privacy/redaction and retention policy for operation evidence;
- partial proofpack policy;
- signing, transparency-log, and remote attestation policy;
- acceptance claim wiring after gate and proof are linked;
- possible future `punk init` interaction with proof storage.

## Next bounded step

After this proofpack writer operation evidence boundary, run another advisory Work Ledger Review before selecting proofpack writer implementation, `.punk/proofs` activation, schema files, CLI behavior, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
