# Proofpack writer concrete path/storage policy boundary v0.1

Date: 2026-04-30
Status: proposed boundary
Authority: advisory/design

## Purpose

Define concrete future path/storage policy semantics for the proofpack writer before any active proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, filesystem side effects, operation-evidence persistence, proofpack referenced-ref verification integration, gate/proof orchestration, or acceptance claim wiring begins.

This is a docs/spec boundary only.

It narrows the remaining future path/storage policy surface to:

```text
explicit storage root ref + logical target artifact ref + selected path/storage policies
  -> explicit target path ref policy and host path observation preconditions
```

It does not activate proofpack writing.

## Status and authority

This boundary is not implementation.

It does not add Rust code.

It does not add dependencies.

It does not add schema files.

It does not create `.punk/proofs`.

It does not expose CLI behavior.

It does not read, write, inspect, resolve, canonicalize, normalize, or compare host filesystem paths.

It does not write proofpacks.

It does not write operation evidence.

It does not write indexes or `latest` pointers.

It does not verify proofpack referenced artifact refs.

It does not write gate decisions.

It does not claim acceptance.

It does not add provider/model/agent adapters, automation, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.

Future implementation requires a separate bounded goal.

## Definition

Proofpack writer concrete path/storage policy means the selected future policy set that turns logical proofpack artifact identity into an operational target path ref under an explicit storage root ref.

For v0.1, this boundary is not a path encoder, path resolver, storage implementation, schema, or writer.

It can define:

- which policies must be selected before active writer file IO;
- how target path refs are derived from logical target artifact refs at policy level;
- how parent directory, symlink, canonicalization, traversal, storage-root escape, redaction, and absolute-path handling must fail closed;
- what information may become future operational evidence;
- which outputs remain non-canonical;
- what future side effects require explicit selection.

It cannot establish:

- that `.punk/proofs` exists;
- that a storage root exists on the host filesystem;
- that a target path exists or is writable;
- that a host filesystem path was resolved, inspected, canonicalized, or normalized;
- that a parent directory exists or was created;
- that a proofpack was written;
- that schema validation exists;
- that operation evidence was persisted;
- that referenced artifact bytes were verified;
- that a gate accepted the work;
- that positive acceptance may be claimed.

Only future `gate` writes final decisions.

## Relationship to existing boundaries

### File IO boundary

`evals/specs/proofpack-writer-file-io-boundary.v0.1.md` defines future file IO semantics around explicit storage targets, write policy, idempotency, conflicts, temp/atomic handling, partial writes, indexes, and `latest` pointers.

This boundary defines the path/storage policy preconditions that must exist before that file IO can be selected for active writer behavior.

It must not perform file IO.

### Storage/schema boundary

`evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md` defines append-only canonical proofpack artifact semantics and keeps indexes, `latest` pointers, wrappers, service mirrors, dashboards, operation evidence, and host path observations non-canonical.

This boundary preserves that split.

It must not make target path refs, storage-root-relative paths, host path observations, selected policy refs, redaction flags, indexes, or `latest` pointers canonical proof truth.

### Host path resolution boundary

`evals/specs/proofpack-writer-host-path-resolution-boundary.v0.1.md` defines how a future resolver may map explicit storage root refs and target path refs into host path observations or fail-closed blockers.

This boundary defines which path/storage policies must be selected before such observation can be considered safe input to a writer.

A host path observation remains operational evidence only.

### Preflight integration boundary

`evals/specs/proofpack-writer-preflight-integration-boundary.v0.1.md` defines side-effect-free preflight composition before a future writer may attempt side effects.

This boundary supplies policy-level preconditions that preflight integration should require before writer-ready file IO.

Missing or rejected path/storage policy must be a preflight blocker.

### Target artifact ref policy

`evals/specs/proofpack-writer-target-artifact-ref-policy.v0.1.md` selects logical target artifact identity shaped like:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

This boundary keeps that logical ref separate from path refs.

A future target path ref may be derived from the logical target artifact ref only through a selected path encoding policy.

## Terms

### Storage root ref

A storage root ref is an explicit logical reference to a selected future storage area.

It is not the current working directory, repo root inferred from process state, global config, IDE state, chat state, executor-local memory, mutable index, service mirror, dashboard, or `punk init` side effect.

A storage root ref is not proof authority, gate authority, receipt authority, schema authority, project-truth authority, or acceptance authority.

### Logical target artifact ref

A logical target artifact ref identifies the intended canonical proofpack artifact.

It is not a filesystem path.

It is not a storage root.

It is not an index row, `latest` pointer, service mirror id, host path observation, proof authority, or acceptance authority.

### Target path ref

A target path ref is an operational ref under a selected storage root.

It is derived only by selected path/storage policy.

It remains operational metadata.

It must not be used as canonical proof truth.

### Host path observation

A host path observation is future operational evidence about how selected refs and policies mapped, or failed to map, to a host filesystem path.

It is not canonical proofpack artifact truth, gate truth, receipt truth, schema truth, project truth, referenced-ref verification, or acceptance evidence.

## Required policy set

A future active writer must not attempt canonical artifact file IO unless the following policy refs are explicit and accepted:

- storage root selection policy;
- logical target artifact ref policy;
- target path derivation policy;
- path encoding policy;
- parent directory policy;
- symlink policy;
- canonicalization policy;
- traversal rejection policy;
- storage-root escape rejection policy;
- absolute-path and local-path redaction policy;
- idempotency and conflict policy;
- temp/atomic write policy;
- index and `latest` pointer non-authority policy, if those side effects are selected.

Policy refs are preconditions and operational evidence.

They are not proof authority, gate authority, receipt authority, schema authority, project-truth authority, or acceptance authority.

## Storage root selection policy

A future storage root selection policy must define which storage roots are allowed.

Recommended v0.1 policy properties:

- storage root refs must be explicit;
- storage root refs must be stable enough to reference in future operation evidence;
- storage root refs must not be mutable indexes or `latest` pointers;
- storage root refs must not be inferred from process current working directory, repo discovery, global config, IDE state, chat state, executor-local memory, or `punk init`;
- storage root refs must not imply that the host directory exists;
- storage root refs must not imply that writes are allowed;
- remote storage roots require a later remote-storage boundary.

A missing, disallowed, ambiguous, or hidden storage root ref must block writer-ready preflight before any filesystem side effect.

## Target path derivation policy

A future target path derivation policy must define how a logical target artifact ref becomes a target path ref under a selected storage root ref.

Recommended v0.1 policy shape:

```text
storage_root_ref: storage-root:project-runtime-proofs
logical_target_artifact_ref: proofpack:<proofpack_id>@<manifest_self_digest>
target_path_ref: target-path:proofpacks/<encoded-proofpack-id>/<encoded-manifest-self-digest>.json
```

This is an example conceptual shape, not an active layout.

The policy must keep these separate:

```text
storage root ref != logical target artifact ref
logical target artifact ref != target path ref
target path ref != host path observation
target path ref != canonical proofpack artifact
target path ref != proof authority
```

A target path ref must not contain host absolute path authority.

A future writer must not derive target path refs from mutable indexes, `latest` pointers, service mirrors, dashboards, executor claims, timestamps, or hidden runtime state.

## Path encoding policy

A future path encoding policy must define how logical artifact identity becomes safe path components.

Recommended v0.1 requirements:

- allowed character set is explicit;
- separator behavior is explicit;
- escaping behavior is explicit;
- digest casing is explicit;
- maximum segment length behavior is explicit;
- collision handling is explicit;
- reversibility or inspectability is explicit;
- decoded output must not contain traversal or separators unless explicitly selected;
- encoded output must not be an absolute path, home-relative path, URL, drive-prefixed path, UNC path, or empty segment.

The future writer must not directly use `proofpack:<proofpack_id>@<manifest_self_digest>` as a host path segment unless a later policy proves that exact shape safe for all selected storage targets.

Encoding success is operational evidence only.

It does not prove write availability, schema validation, referenced-ref verification, gate acceptance, or positive acceptance.

## Parent directory policy

A future parent directory policy must define whether parent directories:

- must already exist;
- may be created by the writer;
- may be created only by a setup command selected separately;
- block the write when missing;
- require cleanup or rollback evidence on failure.

Default boundary: parent directory creation is not selected by this spec.

Recommended fail-closed blockers:

- parent directory policy missing;
- parent directory missing and creation not selected;
- parent path exists but is not a directory;
- parent directory status ambiguous;
- parent directory creation would escape the selected storage root;
- parent directory creation would require unselected side effects;
- parent directory cleanup status is ambiguous after failure.

Parent directory evidence is operational evidence only.

It does not imply canonical proofpack availability or acceptance.

## Symlink policy

A future symlink policy must be explicit before active file IO.

Default boundary: do not silently follow symlinks.

Recommended fail-closed blockers:

- symlink policy missing;
- selected storage root involves a symlink and policy forbids it;
- target path parent contains a symlink in a policy-sensitive position;
- final target exists as a symlink;
- symlink status cannot be observed safely;
- following a symlink would escape the selected storage root;
- policy allows symlinks but evidence cannot explain exactly what was allowed.

If a later policy allows symlink behavior, that policy must be narrow, inspectable, and fail closed on ambiguity.

Symlink allowance must not become proof authority.

## Canonicalization policy

A future canonicalization policy must be explicit.

This boundary does not canonicalize host paths.

A future canonicalization policy should define:

- whether canonicalization is required before write;
- whether canonicalization may require existing filesystem entries;
- how platform-specific normalization is handled;
- how canonicalization interacts with symlink policy;
- how canonicalization result is compared with the selected storage root;
- how redaction applies to canonicalized local paths.

Recommended fail-closed blockers:

- canonicalization policy missing;
- canonicalization required but unavailable;
- canonicalization depends on a missing filesystem entry when policy requires it;
- canonicalization changes effective storage root unexpectedly;
- canonicalization changes effective target path outside policy;
- canonicalization result cannot be compared to storage root safely;
- canonicalized host path would be recorded unredacted without explicit privacy policy.

Canonicalized local paths are sensitive operational observations.

They must not become canonical proof truth.

## Traversal and storage-root escape policy

Traversal and storage-root escape rejection must be explicit and fail closed.

A future writer must reject target path refs or decoded path components that contain or resolve to:

- `..` parent traversal;
- absolute paths;
- platform root prefixes;
- home-directory expansion;
- drive-prefix escape;
- UNC or network-share escape when relevant;
- empty path segments that alter meaning;
- unselected path separators;
- encoded traversal that decodes to an escape;
- symlink-assisted escape;
- canonicalization-assisted escape;
- storage-root-relative paths that leave the selected root.

The writer must not write when it cannot prove, under selected policy, that the effective target path remains under the selected storage root.

Escape blocker evidence is not acceptance evidence.

## Absolute-path and redaction policy

A future absolute-path and redaction policy must define whether any host local path may be recorded.

Default boundary: unredacted absolute local paths are disallowed.

A future implementation should prefer:

- storage-root-relative path summaries;
- redacted host path observations;
- stable storage root refs;
- target path refs;
- policy refs;
- blocker reason codes.

A later policy may allow absolute path diagnostics only when:

- the user explicitly selects it;
- redaction and retention behavior are explicit;
- the path is not written into canonical proofpack manifest bytes by default;
- the path remains operational evidence only;
- secret, credential, home-directory, workspace-private, and environment-derived segments are protected.

Redaction success or failure must be visible.

Redaction must not alter proofpack canonical bytes unless a later schema explicitly selects such fields.

## Idempotency and conflict interaction

Concrete path/storage policy must not weaken append-only canonical artifact semantics.

If a future target path already exists, active writer behavior must rely on the selected idempotency/conflict policy before any write or overwrite.

Recommended boundary:

- matching canonical bytes or selected content identity may produce idempotent evidence;
- different canonical bytes or selected content identity must produce conflict evidence;
- missing comparison evidence must block or report ambiguity;
- indexes and `latest` pointers must not be used as the comparison source of truth;
- host path observations must not be used as the comparison source of truth;
- conflict handling must not mutate the existing canonical artifact.

## Precondition order

Recommended future order before active file IO:

1. receive structurally valid proofpack and explicit post-gate refs;
2. receive deterministic manifest bytes and manifest self-digest;
3. receive logical target artifact ref from accepted target artifact ref policy;
4. select explicit storage root ref;
5. derive target path ref from selected path encoding and target path derivation policies;
6. verify traversal and storage-root escape rejection at policy/ref level;
7. select parent directory, symlink, canonicalization, and redaction policies;
8. produce or explicitly defer host path observation under selected policy;
9. verify no path/storage blockers exist;
10. select idempotency, temp/atomic, index, and `latest` pointer policies;
11. only then may a later separately selected writer attempt side effects.

This order is conceptual.

It does not define schema, CLI, runtime storage, or active writer behavior.

## Operational evidence boundary

Future concrete path/storage policy may produce or reference operational evidence such as:

- selected policy refs;
- storage root ref;
- logical target artifact ref;
- target path ref;
- target path derivation summary;
- storage-root-relative path summary;
- host path observation summary;
- redaction status;
- blocker reasons;
- boundary notes.

This evidence must not claim:

- `.punk/proofs` exists;
- proofpack was written;
- schema validation happened;
- referenced proofpack artifacts were verified;
- operation evidence was persisted;
- gate accepted the work;
- positive acceptance can be claimed.

A blocker is evidence that a future writer must not proceed.

It is not a gate decision.

## Recommended blocker vocabulary

Recommended blocker reason vocabulary for future models or evals:

| Reason | Meaning |
|---|---|
| `storage_root_policy_missing` | No storage root selection policy was selected. |
| `storage_root_ref_missing` | No explicit storage root ref was provided. |
| `storage_root_ref_disallowed` | Storage root ref was not allowed by selected policy. |
| `target_artifact_ref_missing` | Logical target artifact ref was missing. |
| `target_path_derivation_policy_missing` | No policy was selected for deriving target path refs. |
| `target_path_ref_missing` | No target path ref was produced or provided. |
| `path_encoding_policy_missing` | No path encoding policy was selected. |
| `path_encoding_rejected` | Encoding rejected proofpack id, digest, or path component. |
| `path_encoding_collision_ambiguous` | Encoding could collide or did collide without selected collision policy. |
| `parent_directory_policy_missing` | No parent directory policy was selected. |
| `parent_directory_missing` | Required parent directory was absent or unselected for creation. |
| `parent_directory_not_directory` | Parent path exists but is not a directory. |
| `parent_directory_side_effect_not_selected` | Creating or repairing parent directories would require unselected side effects. |
| `symlink_policy_missing` | No symlink policy was selected. |
| `symlink_disallowed` | Symlink was encountered where policy forbids it. |
| `canonicalization_policy_missing` | No canonicalization policy was selected. |
| `canonicalization_unavailable` | Required canonicalization observation could not be produced. |
| `traversal_detected` | Traversal was detected before or after decoding. |
| `storage_root_escape_detected` | Effective target escaped selected storage root. |
| `host_path_observation_required` | Host path observation was required but not available. |
| `host_path_ambiguous` | Host path observation could not be made unambiguous. |
| `host_path_redaction_required` | Observation could not be safely recorded without redaction. |
| `absolute_path_policy_missing` | Absolute-path or redaction policy was missing. |
| `idempotency_policy_missing` | Idempotency/conflict policy was missing. |
| `temp_atomic_policy_missing` | Temp/atomic write policy was missing. |
| `index_latest_policy_missing` | Index/latest side-effect policy was missing when those side effects were selected. |

This vocabulary is advisory/design.

It does not add Rust enums or schemas in this patch.

## Authority boundary

A future concrete path/storage policy may authorize a writer to consider a target path ref safe enough for a later write attempt.

It must not decide.

It must not approve work.

It must not close a contract.

It must not transform missing or failed validators into passing evidence.

It must not turn executor claims into proof.

It must not make storage root refs, target path refs, host path observations, redaction flags, policy refs, indexes, `latest` pointers, service mirrors, or dashboards canonical proof truth.

It must not create positive acceptance claims.

Positive acceptance still requires an accepting gate decision and matching proof under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Security and privacy boundary

Path/storage policy can expose sensitive refs, local path summaries, hashes, timestamps, event ranges, policy choices, blocker reasons, and boundary notes.

Future implementations must not write secrets, credentials, private prompts, environment dumps, private user text, raw provider payloads, or unredacted local paths into canonical artifacts by default.

Future implementations should prefer storage-root-relative and redacted evidence.

Retention, deletion, publication, and redaction policies remain future explicit boundaries.

## Setup-neutral boundary

Concrete path/storage policy must stay setup-neutral.

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

A future setup helper may prepare directories only if a later bounded goal selects that side effect.

This boundary does not select it.

## Non-goals

This v0.1 boundary does not define or implement:

- proofpack writer implementation;
- `.punk/proofs` directory creation;
- `.punk/proofs` storage layout activation;
- schema files;
- CLI commands;
- runtime proof storage;
- host filesystem path resolution, inspection, canonicalization, or normalization;
- concrete Rust path encoder;
- concrete parent directory creation behavior;
- concrete symlink inspection behavior;
- concrete platform-specific path normalization behavior;
- concrete path traversal checker code;
- concrete storage-root escape checker code;
- operation-evidence persistence;
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
- context compiler;
- Knowledge Vault implementation;
- compiled wiki behavior;
- `punk init`.

## Acceptance checks for a future implementation

A future implementation should not be selected until it can be checked against these boundary rules.

### PWCP-001: selected policies are explicit

The writer must require explicit storage root, target artifact ref, target path derivation, path encoding, parent directory, symlink, canonicalization, traversal, storage-root escape, redaction, idempotency, and temp/atomic policies before active file IO.

### PWCP-002: refs remain distinct

The writer must keep storage root refs, logical target artifact refs, target path refs, host path observations, and canonical proofpack artifacts distinct.

### PWCP-003: no hidden path authority

The writer must not infer storage roots or target paths from current working directory, repo discovery, global config, IDE state, chat state, executor-local memory, mutable indexes, `latest` pointers, service mirrors, or dashboards.

### PWCP-004: path encoding fails closed

Unsafe, ambiguous, colliding, traversal-producing, absolute, URL, home-relative, drive-prefixed, UNC, empty, or separator-injected path encodings must block before write attempts.

### PWCP-005: parent directory behavior is selected

Missing, non-directory, ambiguous, or unselected parent directory side effects must block before canonical artifact writes.

### PWCP-006: symlinks do not silently pass

Symlink behavior must be explicit, inspectable, and fail closed on ambiguity.

### PWCP-007: canonicalization does not rewrite authority

Canonicalization must not silently rewrite user intent or turn canonicalized local paths into proof truth.

### PWCP-008: storage-root escape blocks writes

Traversal, encoded traversal, symlink-assisted escape, canonicalization-assisted escape, and storage-root-relative escape must block before writes.

### PWCP-009: redaction is explicit

Unredacted absolute local paths must not be written into canonical artifacts or operation evidence unless a later explicit privacy/redaction policy permits it.

### PWCP-010: operation evidence is non-authoritative

Selected policies, target path refs, host path observations, and blockers must remain operational evidence, not proof authority, gate authority, receipt authority, schema authority, project-truth authority, or acceptance authority.

### PWCP-011: append-only semantics are preserved

Concrete path/storage policy must not allow silent overwrite of canonical proofpack artifacts.

### PWCP-012: no setup ritual

Concrete path/storage policy must not require a specific IDE, prompt, provider, model, skill, local runtime ritual, global config, or `punk init`.

## Open follow-ups

- side-effect-free concrete path/storage policy model;
- proofpack writer implementation;
- actual `.punk/proofs` storage activation;
- proofpack schema files;
- proofpack writer CLI surface, if any;
- parent directory creation and cleanup policy implementation;
- concrete symlink inspection policy implementation;
- concrete canonicalization and platform path policy implementation;
- concrete storage-root escape checker implementation;
- proofpack index and latest-pointer implementation;
- operation-evidence persistence policy;
- proofpack referenced-ref verification integration implementation;
- privacy/redaction and retention policy for proof artifacts;
- partial proofpack policy;
- writer receipt or operation evidence shape;
- acceptance claim wiring after gate and proof are linked;
- remote attestation, signing, or transparency-log policy;
- possible future `punk init` interaction with proof storage.

## Next bounded step

After this concrete path/storage policy boundary, run another advisory Work Ledger Review before selecting active proofpack writer implementation, side-effect-free model coverage, `.punk/proofs` activation, schema files, CLI, host filesystem path resolution/canonicalization implementation, operation-evidence persistence, referenced-ref verification integration, gate/eval/proof orchestration, adapters, automation, provider/model runners, or `punk init` work.
