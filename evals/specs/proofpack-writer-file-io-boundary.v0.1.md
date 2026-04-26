# Proofpack writer file IO boundary v0.1

Date: 2026-04-26
Status: proposed boundary
Authority: advisory/design

## Purpose

Define future proofpack writer file IO semantics before any proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, gate/proof orchestration, or acceptance claim wiring begins.

This is a design/spec artifact only.

It narrows the future file IO surface to:

```text
writer-ready proofpack plan + explicit storage target + explicit write policy -> append-only canonical artifact write attempt evidence
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

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer file IO means the future filesystem behavior needed to persist a canonical proofpack artifact from an already explicit writer-ready plan.

For v0.1, file IO boundary is not a writer.

It can define:

- explicit storage root requirements;
- explicit target ref and target path requirements;
- append-only canonical artifact behavior;
- idempotency and conflict semantics;
- temporary-file and atomic-write expectations;
- partial-write detection and rollback limits;
- index and `latest` pointer non-authority;
- error-reporting expectations;
- operation evidence mapping.

It cannot establish:

- that `.punk/proofs` exists;
- that a proofpack was written;
- that schema validation exists;
- that a gate accepted work;
- that positive acceptance may be claimed;
- that referenced artifact bytes were verified;
- that indexes or `latest` pointers are canonical truth.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### Proofpack boundary

`evals/specs/proofpack-boundary.v0.1.md` defines proofpack as a post-gate provenance bundle over linked evidence.

File IO must preserve that model.

It must not turn proofpack writing into a pre-gate checklist, decision surface, receipt surface, or second tracker.

### Proofpack writer preparation

`evals/specs/proofpack-writer-preparation-boundary.v0.1.md` defines writer-ready conceptual inputs, outputs, preconditions, side-effect boundaries, authority limits, and follow-ups.

File IO must consume an explicit writer-ready plan or equivalent explicit refs.

It must not infer missing proofpack inputs from hidden process state.

### Proofpack writer preflight plan model

Current `punk-proof` can model writer preflight plans as side-effect-free data.

Future file IO should start only after preflight says the plan is writer-ready.

A `preflight_failed` plan must not attempt canonical artifact writes.

A `planned_only` plan may become input to a future write attempt, but planning itself is not a write.

### Proofpack writer storage/schema

`evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md` defines append-only canonical proofpack artifact expectations, wrapper/index/latest pointer limits, schema/file-layout deferral, idempotency/conflict behavior, and storage failure policy.

File IO narrows the write-side behavior of that storage boundary.

It still does not choose a schema file or concrete persisted layout.

### Proofpack writer operation evidence

`evals/specs/proofpack-writer-operation-evidence-boundary.v0.1.md` defines future non-authoritative evidence for writer attempts.

File IO must report outcomes through operation evidence or an equivalent future evidence shape.

Storage success is evidence of a side effect.

It is not gate acceptance.

### Project Memory storage direction

`docs/product/PROJECT-MEMORY.md` separates canonical artifacts from rebuildable runtime views.

Future proofpack file IO must follow the same split:

```text
append-only canonical proofpack artifact -> rebuildable indexes/views -> optional service mirror later
```

A missing or stale index must degrade inspect UX, not change proof truth.

## Explicit future inputs

A future file writer should receive explicit inputs.

Recommended conceptual inputs:

- proofpack values or deterministic manifest bytes;
- proofpack id;
- manifest self-digest;
- writer preflight plan ref or in-memory plan;
- storage root;
- target artifact ref;
- target artifact path resolved from storage root and target ref;
- overwrite/idempotency policy;
- temp/atomic write policy;
- index update policy, if selected;
- latest pointer policy, if selected;
- operation evidence target, if persistence is selected later;
- boundary notes.

Inputs must be explicit.

A future writer must not silently use:

- process current working directory;
- global config;
- editor state;
- chat state;
- executor-local memory;
- mutable indexes;
- mutable `latest` pointers;
- unlinked files;
- inferred repository roots;
- hidden service state.

If a later runtime resolves a storage root or target path, that resolution result must be inspectable before writing.

## Storage root boundary

A storage root is the explicit root under which future proofpack artifacts may be written.

For a future `.punk/proofs` implementation, the storage root might be a repo-local runtime path.

This boundary does not activate that path.

A future writer must treat storage root resolution as policy-sensitive.

Required future storage-root properties:

- explicit, not inferred from chat or executor state;
- policy-allowed;
- inspectable before write;
- not a mutable index or `latest` pointer;
- not a remote sink unless a later remote-storage boundary selects it;
- stable enough for operation evidence to reference.

A missing or disallowed storage root must fail preflight or produce `preflight_failed` evidence before a canonical write is attempted.

## Target ref and target path boundary

The target artifact ref is the future stable reference to the intended canonical proofpack artifact.

The target path is the future filesystem path derived from the explicit storage root and target ref.

A future writer must keep these separate:

```text
target_ref != absolute path authority
target_path != proof truth
```

Target refs should be stable and repo-inspectable.

Target paths should be derived by selected policy.

A future writer must not expose host-specific absolute paths as canonical proof refs unless a later boundary explicitly selects that behavior.

Target path derivation must reject path traversal, absolute injected refs, home-relative refs, URL refs, backslashes when unsupported, and ambiguous `.` segments.

## Canonical artifact boundary

A future canonical proofpack artifact should be append-only proof evidence.

The writer must not mutate it in place.

The writer must not silently overwrite it.

The writer must not treat indexes, latest pointers, operation evidence, schema validation reports, or CLI output as the canonical proofpack artifact.

Canonical artifact availability can be reported only when the selected future write policy determines that a complete canonical artifact exists or an existing matching artifact already exists.

Even then, availability is operation evidence only.

It is not acceptance.

## Temp and atomic write boundary

A future file writer should prefer an atomic write policy where supported.

Recommended future policy:

1. render or receive deterministic manifest/wrapper bytes in memory;
2. compute or preserve manifest self-digest before filesystem side effects;
3. resolve storage root and target path explicitly;
4. check for existing target;
5. write to a temporary sibling or otherwise policy-selected temporary target;
6. flush/sync according to selected durability policy;
7. atomically move/rename into the target when supported;
8. report the canonical artifact status and temp/atomic side-effect status separately.

This boundary does not choose platform-specific fsync behavior.

Future implementation must document platform limits.

If atomic rename is unavailable, the writer must either fail closed or use an explicitly selected non-atomic policy that reports weaker durability.

## Idempotency boundary

A future writer must define how repeated attempts are compared.

Recommended idempotency basis:

- selected canonical bytes match exactly; or
- selected content identity matches exactly.

The idempotency basis must be explicit in operation evidence.

If the target exists and matches, the future outcome should be `already_exists_matching`.

Idempotent evidence must not claim a new write happened.

A future writer must not infer idempotency from:

- mutable indexes;
- mutable `latest` pointers;
- chat state;
- executor-local memory;
- filename alone;
- timestamps alone;
- absence of an error.

## Conflict boundary

If the target canonical artifact already exists and differs from the intended bytes or selected content identity, the future writer must fail closed.

Recommended future outcome:

```text
conflict_existing_different
```

Conflict evidence should identify:

- target ref;
- comparison basis;
- intended content identity or digest;
- existing content identity or digest when safely available;
- that the existing canonical artifact was not overwritten.

Conflict evidence must not expose hidden host-specific paths as proof truth.

Conflict handling must not mutate the existing canonical artifact.

## Partial-write and rollback boundary

A future writer must distinguish these states:

- no canonical write attempted;
- temp write failed before canonical artifact availability;
- canonical move/rename failed;
- canonical target exists but completeness is ambiguous;
- canonical artifact available but index update failed;
- canonical artifact available but latest pointer update failed.

If a temp write fails, the writer should not leave a canonical partial artifact.

If cleanup fails, cleanup failure must be visible as operation evidence.

If a partial or ambiguous canonical artifact state is detected, future evidence should use `partial_write_detected` or an equivalent non-authoritative outcome.

Partial state must not enable positive acceptance claims.

Rollback must be conservative.

A future writer must not delete or rewrite a pre-existing canonical artifact as rollback unless a later selected policy explicitly proves it owns that artifact and can do so safely.

## Index and latest pointer boundary

Indexes and `latest` pointers are future convenience views only.

They are not canonical proof truth.

A future writer may update them only if that side effect is separately selected.

Their statuses must be reported separately:

```text
canonical_artifact_status != index_status != latest_pointer_status
```

If canonical artifact write or idempotent availability succeeds but an index update fails, canonical availability remains separate from index failure.

If a `latest` pointer update fails, proof truth remains anchored in the append-only canonical artifact and gate/proof links.

A missing, stale, inconsistent, or failed index/latest update may affect inspect UX.

It must not change acceptance truth.

## Operation evidence mapping

A future file writer should map file IO results to explicit operation outcomes.

Recommended mapping:

| File IO condition | Future outcome |
|---|---|
| no write selected, plan only | `planned_only` |
| required precondition missing before write | `preflight_failed` |
| canonical artifact write completes | `written` |
| target exists with matching selected bytes or identity | `already_exists_matching` |
| target exists with different selected bytes or identity | `conflict_existing_different` |
| canonical write attempt fails before availability | `write_failed` |
| partial or ambiguous canonical artifact state detected | `partial_write_detected` |
| canonical artifact available but index update fails | `index_update_failed` |
| canonical artifact available but latest pointer update fails | `latest_pointer_update_failed` |
| operation intentionally stopped | `aborted` |

The outcome must not be the only evidence.

Side-effect details should remain visible.

## Error reporting boundary

Future file IO errors should preserve enough detail for debugging without turning host-local state into proof authority.

Recommended future error classes:

- storage root missing;
- storage root disallowed;
- target path invalid;
- parent directory missing;
- parent directory creation denied, if selected;
- target exists matching;
- target exists different;
- temp write denied;
- temp write failed;
- flush or sync failed;
- atomic move unsupported;
- atomic move failed;
- cleanup failed;
- index update failed;
- latest pointer update failed;
- operation evidence persistence failed, if selected later.

Errors should include stable reason codes.

Errors may include diagnostic text.

Errors must not claim acceptance or gate authority.

## Security and privacy boundary

Proofpack writer file IO may expose sensitive refs, paths, hashes, timestamps, event ranges, failure details, and boundary notes.

A future implementation must not write secrets or sensitive local paths into canonical artifacts unless an explicit privacy/redaction policy permits it.

A future writer should prefer repo-relative or storage-root-relative refs for canonical evidence.

Host-specific absolute paths should stay diagnostic-only unless a later boundary selects otherwise.

## Setup-neutral boundary

Future file IO must stay setup-neutral.

It must not require a specific IDE, editor, agent, prompt, local model, provider, skill, or chat surface.

A CLI may later call the writer, but this boundary does not create CLI behavior.

An agent may later call the writer, but executor claims are not proof.

## Non-goals

This boundary does not define or implement:

- proofpack file writer code;
- `.punk/proofs` runtime storage;
- proofpack schema files;
- concrete file layout;
- CLI commands;
- `punk init`;
- referenced artifact verification integration;
- hash or byte normalization;
- broad file IO hashing;
- gate/eval/proof orchestration;
- acceptance claim schema or writer;
- remote attestation;
- signing;
- transparency log entries;
- provider/model/agent adapters;
- automation.

## Acceptance checks for a future implementation

A future implementation should not be selected until it can be checked against these boundary rules:

### PWIO-001: explicit storage target

The writer must receive or derive from explicit inputs a storage root, target ref, and target path before write attempts.

### PWIO-002: no hidden source of truth

The writer must not use chat state, executor-local memory, mutable indexes, latest pointers, or unlinked files as authority for missing inputs.

### PWIO-003: append-only canonical artifact

The writer must not silently overwrite an existing canonical proofpack artifact.

### PWIO-004: idempotency is explicit

A matching existing canonical artifact must produce idempotent evidence rather than claiming a new write.

### PWIO-005: conflict fails closed

A different existing canonical artifact must report conflict and must not be overwritten.

### PWIO-006: temp and atomic policy visible

Temporary write and atomic move behavior must be explicit, including platform limitations and failure reporting.

### PWIO-007: partial writes are non-authoritative

Partial or ambiguous canonical artifact states must be visible and must not enable positive acceptance claims.

### PWIO-008: index/latest are non-canonical

Index and latest pointer updates must remain separate from canonical artifact availability.

### PWIO-009: operation evidence is separate

Writer operation evidence must remain evidence about side effects, not proof authority, gate authority, receipt authority, schema validation authority, or acceptance authority.

### PWIO-010: no setup ritual

The writer must not require a specific IDE, prompt, provider, model, skill, or local runtime ritual beyond explicit selected inputs.

## Open follow-ups

- proofpack writer implementation;
- `.punk/proofs` activation;
- concrete schema/file layout;
- atomic write and fsync policy;
- proofpack index and latest-pointer implementation;
- operation evidence persistence policy;
- referenced artifact verification integration;
- privacy/redaction policy for paths and diagnostics;
- CLI surface, if any;
- `punk init`, if future runtime setup is selected.

## Work Ledger next step

After this proofpack writer file IO boundary, run another advisory Work Ledger Review before selecting proofpack writer implementation, `.punk/proofs` activation, schema files, CLI behavior, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
