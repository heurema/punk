# Proofpack writer first active write slice boundary v0.1

Date: 2026-04-30
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the smallest future active proofpack writer write slice before implementation begins.

This is a docs/spec boundary only.

It narrows the first active writer implementation surface to:

```text
explicit writer-ready inputs + explicit safe test storage target
  -> one canonical artifact write attempt + non-authoritative in-memory outcome evidence
```

It does not activate proofpack writing today.

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not create `.punk` or `.punk/proofs`.

It does not expose CLI behavior.

It does not write proofpacks.

It does not persist operation evidence.

It does not update indexes or `latest` pointers.

It does not verify proofpack referenced artifact refs.

It does not write gate decisions.

It does not create acceptance claims.

It does not add provider/model/agent adapters, automation, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.

Only future `gate` writes final decisions. Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Selected first active slice

The first active slice should be named conceptually:

```text
write_artifact_only_explicit_test_storage_root
```

A later implementation goal may attempt exactly one product-relevant active behavior:

```text
write exact canonical proofpack artifact bytes to one explicit target path under one explicit caller-provided storage root
```

This allowed behavior is narrow:

- the storage root must be explicit and caller-provided;
- the target path must be explicit and policy-accepted before the write;
- the parent directory must already exist unless a later goal explicitly selects parent directory creation;
- the canonical artifact bytes must be supplied by the canonical artifact model or equivalent explicit in-memory value;
- the implementation must write those bytes exactly, without re-rendering or transforming them;
- temp-file creation, atomic promotion, and temp cleanup may occur only as sub-effects of the selected temp/atomic write policy;
- the implementation may return in-memory operation outcome evidence;
- the implementation must not persist operation evidence.

The first slice should be testable in temporary test directories without activating `.punk/proofs` runtime storage.

The target path is operational evidence, not proof truth.

A successful write is operation evidence only. It is not gate acceptance, contract acceptance, schema validation, referenced-ref verification, or positive acceptance.

## Side effects still forbidden

The first active slice must not perform these side effects:

- create `.punk` or `.punk/proofs` as active runtime state;
- infer a storage root from repository discovery, current working directory, global config, IDE state, chat state, executor-local memory, mutable indexes, service mirrors, dashboards, or `punk init`;
- create parent directories;
- expose CLI behavior;
- add schema files;
- write gate decisions;
- create acceptance claims;
- persist operation evidence;
- update indexes;
- update mutable `latest` pointers;
- write service mirrors;
- update dashboards;
- verify proofpack referenced artifact refs;
- hash broad referenced artifact trees;
- normalize artifact bytes or hashes;
- invoke provider/model/agent adapters;
- run automation;
- run or require `punk init`.

If any later implementation needs one of these behaviors, it must be selected by a separate bounded goal.

## Required explicit inputs

A later first-slice implementation must receive explicit inputs.

Required conceptual inputs:

- active writer behavior selection for this attempt;
- writer-ready preflight integration result with status `ready`;
- proofpack id;
- exact canonical artifact bytes;
- manifest self-digest for the exact canonical bytes;
- accepted canonical artifact policy status;
- accepted logical target artifact ref;
- explicit storage root ref;
- explicit target path ref;
- accepted host path observation or explicit host path blocker-free observation, when selected by the implementation goal;
- accepted concrete path/storage policy status;
- accepted idempotency and conflict policy;
- accepted temp/atomic write policy;
- parent directory policy status;
- symlink policy status;
- traversal and storage-root escape policy status;
- redaction policy status for diagnostics;
- selected side effects for this first slice;
- boundary notes.

Inputs must not be silently inferred from hidden process state.

## Required preconditions

A later implementation must fail closed before any canonical artifact write when any required precondition is missing, rejected, ambiguous, or contradictory.

Required preconditions:

| Precondition | Fail-closed requirement |
|---|---|
| Active slice selected | No write unless this first active slice is explicitly selected by a bounded goal or runtime policy. |
| Writer-ready preflight | `blocked`, `not_selected`, missing, or mismatched preflight blocks all write side effects. |
| Canonical bytes | Missing canonical bytes, missing manifest self-digest, or digest/bytes mismatch blocks the write. |
| Logical target artifact ref | Missing or rejected target artifact ref blocks the write. |
| Storage root ref | Missing, disallowed, inferred, or ambiguous storage root blocks the write. |
| Target path ref | Missing, rejected, traversal-producing, absolute, or storage-root-escaping target path blocks the write. |
| Host path observation | Missing required observation, unresolved blocker, redaction failure, or path ambiguity blocks the write. |
| Concrete path/storage policy | Missing or rejected policy readiness blocks the write. |
| Parent directory policy | Missing parent directory, non-directory parent, or unselected parent creation blocks the write. |
| Symlink policy | Symlink ambiguity or disallowed symlink blocks the write. |
| Idempotency/conflict policy | Missing comparison basis blocks overwrite and canonical availability claims. |
| Temp/atomic policy | Missing temp/atomic policy blocks the write. |
| Privacy/redaction policy | Diagnostics that would expose disallowed local paths or secrets must be redacted or blocked. |

A blocker is operation evidence. It is not a gate decision.

## First active behavior flow

A later implementation should follow this order:

1. receive explicit first-slice inputs;
2. verify that the selected operation is `write_artifact_only_explicit_test_storage_root` or equivalent first-slice behavior;
3. reject missing, `blocked`, `not_selected`, or mismatched preflight before filesystem side effects;
4. verify canonical artifact bytes and manifest self-digest match the explicit canonical artifact input;
5. verify logical target artifact ref, storage root ref, target path ref, host path observation, and concrete path/storage policy readiness;
6. verify parent directory, symlink, traversal, storage-root escape, idempotency/conflict, temp/atomic, and redaction policies;
7. inspect only the explicit target path needed for idempotency/conflict classification;
8. classify existing target as absent, matching, different, unreadable, or ambiguous;
9. return `already_exists_matching` evidence and do not rewrite when existing canonical bytes match;
10. return `conflict_existing_different` evidence and do not overwrite when existing bytes differ;
11. write canonical bytes through the selected temp/atomic policy when the target is absent;
12. report write, temp, atomic promotion, and cleanup statuses separately;
13. return in-memory non-authoritative operation outcome evidence;
14. avoid all acceptance, gate, schema, referenced-ref verification, index, latest-pointer, CLI, `.punk`, and operation-evidence persistence claims.

This order is conceptual.

It does not define a schema, CLI, runtime storage layout, or concrete Rust API.

## Expected outcomes

The first active slice should use or map to existing operation outcome vocabulary:

| Condition | Expected outcome | Required behavior |
|---|---|---|
| Explicit plan only, no write selected | `planned_only` | No filesystem write. |
| Required precondition missing or rejected | `preflight_failed` | No canonical artifact write. |
| Target absent and write policy completes | `written` | Exact canonical bytes become available according to selected write policy. |
| Target exists with matching canonical bytes | `already_exists_matching` | Return idempotent evidence and do not rewrite. |
| Target exists with different canonical bytes | `conflict_existing_different` | Fail closed and do not overwrite. |
| Write fails before artifact availability | `write_failed` | Preserve failure evidence and do not claim availability. |
| Partial or ambiguous artifact detected | `partial_write_detected` | Mark non-authoritative and do not claim acceptance. |
| Operation intentionally stopped | `aborted` | Say whether any sub-effects were attempted. |

Index, `latest`, operation-evidence persistence, schema, gate, and acceptance outcomes are out of scope for this first slice.

## Exact-byte boundary

The future implementation must write the exact supplied canonical bytes.

It must not:

- re-render the manifest during the write;
- change line endings;
- append whitespace;
- wrap bytes in a schema envelope;
- compress, encrypt, sign, or otherwise transform bytes;
- normalize hashes or paths inside canonical bytes;
- add diagnostic host paths to canonical bytes;
- add timestamps unless the canonical artifact model already selected them.

Exact-byte evidence should compare the file bytes read back from the explicit test target with the supplied canonical bytes.

This read-back evidence is operation evidence only.

## Idempotency and conflict boundary

The first active slice must preserve append-only canonical artifact semantics.

Required behavior:

- absent target may be written once through selected policy;
- existing target with exact matching bytes returns `already_exists_matching`;
- existing target with different bytes returns `conflict_existing_different`;
- unreadable or ambiguous target fails closed;
- no existing target is silently overwritten;
- mutable indexes and `latest` pointers must not be used as idempotency evidence;
- target path existence alone must not become proof truth.

## Temp and atomic boundary

The first active slice may use temp/atomic sub-effects only if the selected write policy makes them explicit.

Recommended first-slice default:

```text
write temp sibling under the explicit target parent
flush according to selected policy when available
promote temp to target through selected atomic move policy when supported
cleanup temp file created by this attempt when needed
```

This boundary does not make platform-specific atomicity or durability claims.

If a later implementation claims cross-platform atomic rename, durability after crash, fsync semantics, symlink-safe behavior, canonicalization guarantees, or platform path normalization guarantees, it needs additional external research before coding those claims.

If the later implementation stays narrower and only demonstrates exact-byte behavior in temporary test directories without broad platform guarantees, repo-tracked research may be sufficient.

Any weaker-than-atomic behavior must be explicitly selected and reported as weaker operation evidence.

## Failure evidence boundary

Failures must remain visible.

Required failure evidence:

- which precondition failed before write;
- whether any filesystem sub-effect was attempted;
- whether a temp file was created;
- whether a temp cleanup failed;
- whether target availability is absent, complete, partial, ambiguous, unreadable, matching, or different;
- whether host path diagnostics were redacted;
- that no gate decision, acceptance claim, index, latest pointer, schema file, `.punk` state, or operation-evidence persistence was written.

The implementation should prefer stable reason codes over boolean success values.

## Later implementation test plan

A later active implementation goal should include tests or smoke coverage for these cases.

### PWFAW-001: exact-byte write

Given explicit writer-ready inputs, explicit storage root, explicit target path, accepted path/storage policy, existing parent directory, and absent target, the writer writes bytes exactly.

Evidence must prove the target bytes equal the supplied canonical bytes.

### PWFAW-002: no write when preflight is blocked

Given `blocked`, `not_selected`, missing, or mismatched preflight, the writer returns fail-closed evidence and no target file is created.

### PWFAW-003: no write when path/storage policy is blocked

Given missing or rejected storage root, target path, host path observation, concrete path/storage policy, traversal rejection, storage-root escape rejection, symlink policy, parent directory policy, redaction policy, idempotency policy, or temp/atomic policy, the writer returns fail-closed evidence and no target file is created.

### PWFAW-004: idempotent existing-match

Given an existing target with exact matching canonical bytes, the writer returns `already_exists_matching`, does not rewrite silently, and does not claim a new write.

### PWFAW-005: conflict existing-different

Given an existing target with different bytes, the writer returns `conflict_existing_different`, does not overwrite, and preserves the existing bytes.

### PWFAW-006: failure visibility

Given an injected or reproducible write/temp/atomic/cleanup failure where in scope, the writer reports the failed sub-effect separately and does not claim canonical artifact availability unless the selected policy proves it.

### PWFAW-007: forbidden side effects stay absent

The first slice must not create `.punk`, `.punk/proofs`, schema files, CLI behavior, gate decision files, acceptance claim files, indexes, latest pointers, operation-evidence persistence, service mirrors, dashboards, provider/model/agent adapter behavior, automation, context compiler outputs, Knowledge Vault outputs, compiled wiki behavior, or `punk init` state.

### PWFAW-008: write success is not acceptance

A `written` or `already_exists_matching` outcome remains non-authoritative operation evidence and must not mark work accepted.

### PWFAW-009: referenced-ref verification remains separate

The writer must not claim proofpack referenced artifact verification unless explicit verification evidence from a separately selected verifier is supplied.

For this first slice, referenced-ref verification integration remains out of scope.

## Research Gate for later implementation

This boundary itself uses repo-tracked knowledge and does not require external research.

A later implementation goal should classify research as follows:

- R1 may be sufficient for a narrow exact-byte write implementation limited to explicit test storage roots, stdlib file operations, temporary test directories, and no broad platform atomicity or durability claims.
- R2 or stronger is required before claiming cross-platform atomic rename semantics, crash durability, fsync guarantees, symlink-safe behavior, canonicalization guarantees, platform path normalization behavior, or security-sensitive filesystem guarantees.
- R2 or stronger is also required before introducing `.punk/proofs` runtime activation, persistent operation evidence, schema files, CLI behavior, or broader runtime storage behavior if those choices rely on external platform, filesystem, or ecosystem claims.

The later implementation goal must state its classification before coding.

## Relationship to authority

The first active write slice may produce evidence that bytes were written or already existed at an explicit target.

It must not produce or imply:

- gate acceptance;
- contract approval;
- goal closure authority;
- validator success;
- schema validation;
- referenced artifact verification;
- receipt authority;
- proof authority by target path alone;
- project truth by host path alone;
- positive acceptance claims.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Non-goals

This boundary does not define or implement:

- active proofpack writer code;
- `.punk/proofs` runtime storage activation;
- concrete schema files;
- CLI commands;
- operation-evidence persistence;
- proofpack referenced-ref verification integration;
- gate/eval/proof orchestration;
- acceptance claim writer;
- provider/model/agent adapters;
- automation;
- context compiler;
- Knowledge Vault implementation;
- compiled wiki behavior;
- `punk init`;
- broad filesystem hashing;
- signing, transparency log, remote attestation, or service mirrors.

## Open follow-ups

- Run an advisory Work Ledger Review before selecting active implementation.
- If selected, implement the first active writer slice as a separate bounded goal with explicit research classification.
- Define operation-evidence persistence separately after the first active write behavior is known.
- Define proofpack referenced-ref verification integration separately.
- Define `.punk/proofs`, schema files, indexes, latest pointers, CLI, runtime setup, and `punk init` only through later bounded goals.

## Work Ledger next step

After this boundary, run another advisory Work Ledger Review before selecting the first active proofpack writer implementation or any runtime, schema, CLI, `.punk`, operation-evidence persistence, referenced-ref verification integration, gate/eval/proof orchestration, adapter, automation, provider/model runner, context compiler, Knowledge Vault, compiled wiki, or `punk init` work.
