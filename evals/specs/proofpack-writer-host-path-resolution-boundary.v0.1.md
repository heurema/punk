# Proofpack writer host path resolution boundary v0.1

Date: 2026-04-29
Status: proposed boundary
Authority: advisory/design

## Purpose

Define how a future proofpack writer may resolve explicit storage root refs and target path refs into host path observations before any active proofpack writer implementation, `.punk/proofs` runtime storage, schema files, CLI behavior, operation-evidence persistence, gate/proof orchestration, or acceptance claim wiring begins.

This is a docs/spec boundary only.

It narrows the future path-resolution surface to:

```text
explicit storage root ref + explicit target path ref + selected path policy
  -> host path observation or fail-closed blocker evidence
```

It does not activate proofpack writing.

## Non-goals

This boundary does not:

- add Rust code;
- add dependencies;
- add schema files;
- add CLI commands;
- activate `.punk` or `.punk/proofs`;
- implement proofpack writing;
- read, write, inspect, resolve, canonicalize, or normalize host filesystem paths in code;
- create parent directories;
- follow symlinks;
- persist operation evidence;
- verify proofpack referenced artifact refs;
- write gate decisions;
- create acceptance claims;
- add adapters, automation, provider/model runners, context compiler, Knowledge Vault implementation, compiled wiki behavior, or `punk init`.

Future implementation requires a separate bounded goal.

## Terms

### Storage root ref

A storage root ref is an explicit logical reference to a future storage root selected by a write policy.

It is not inferred from the current working directory, global config, IDE state, chat state, executor-local memory, mutable indexes, service mirrors, dashboards, or `punk init`.

A storage root ref is not proof authority, gate authority, receipt authority, schema authority, acceptance authority, or project truth by itself.

### Logical target artifact ref

A logical target artifact ref identifies the intended canonical proofpack artifact.

For the current target artifact ref policy, logical refs are shaped like:

```text
proofpack:<proofpack_id>@<manifest_self_digest>
```

A logical target artifact ref is not a filesystem path, storage root, host path, index row, `latest` pointer, service mirror id, proof authority, or acceptance authority.

### Target path ref

A target path ref is an explicit operational ref under a selected storage root.

It is not the same thing as a logical target artifact ref. It may be derived from explicit proofpack identity only through a selected path encoding policy, and it remains operational evidence.

### Host path observation

A host path observation is a future runtime observation that records how an explicit storage root ref and target path ref mapped, or failed to map, to a host filesystem path under selected policy.

It is an observation, not canonical truth. It must not be the only evidence needed to understand proof authority, gate decisions, referenced artifact verification, or acceptance.

## Inputs

A future host path resolution step must receive explicit inputs.

Required conceptual inputs:

- storage root ref;
- target path ref;
- logical target artifact ref, when the path is intended for a proofpack artifact;
- selected path encoding policy;
- selected parent directory policy;
- selected symlink policy;
- selected canonicalization policy;
- selected traversal and storage-root escape policy;
- caller or runtime context allowed by a future bounded goal;
- boundary notes explaining which policies were used.

The resolver must not infer missing inputs from hidden process state.

## Explicit boundaries

The future resolver must keep these distinct:

```text
storage root ref != logical target artifact ref
target artifact ref != target path ref
target path ref != host path observation
host path observation != proof authority
```

Path resolution evidence is operational evidence only.

It is not:

- proof authority;
- gate authority;
- receipt authority;
- schema authority;
- acceptance authority;
- project truth by itself;
- referenced artifact verification;
- proofpack availability by itself.

A future writer must fail closed on ambiguous, missing, contradictory, or unsafe path-resolution inputs.

## Host path observation model

A future host path observation should be inspectable and minimal.

Recommended fields:

```yaml
host_path_observation:
  status: observed | blocked | not_selected
  storage_root_ref: storage_root:project-runtime-proofs
  target_path_ref: target-path:proofpacks/<encoded-target>.json
  logical_target_artifact_ref: proofpack:<id>@<digest>
  policy_refs:
    path_encoding: policy:path-encoding-v0.1
    parent_directory: policy:parent-directory-v0.1
    symlink: policy:symlink-v0.1
    canonicalization: policy:canonicalization-v0.1
    traversal: policy:traversal-v0.1
  host_path_kind: absolute | storage_root_relative | unavailable
  host_path_redacted: true
  blockers: []
  boundary_notes:
    - "observation is operational evidence only"
```

This is a conceptual evidence shape, not an active schema.

A future implementation should prefer repo-relative or storage-root-relative evidence when possible and must avoid leaking sensitive local paths unless a later privacy/redaction policy explicitly permits it.

## Path encoding policy

A future path encoding policy must be explicit before target artifact refs become path components.

The policy should define:

- allowed characters;
- normalization rules, if any;
- escaping rules;
- digest casing;
- path segment limits;
- reversibility or inspectability requirements;
- collision handling;
- rejected characters or sequences.

The future writer must not directly use unencoded `proofpack:<id>@<digest>` as a host path segment unless a later policy explicitly proves that the encoding is safe for all supported host filesystems.

The path encoding policy must preserve the ability to recover or inspect the proofpack id and manifest self-digest without reading mutable indexes, `latest` pointers, service mirrors, dashboards, or hidden runtime state.

## Parent directory policy

A future parent directory policy must be explicit.

This boundary does not create parent directories.

A future writer must know whether parent directories are expected to already exist, may be created by the writer, or block the write when missing.

Recommended fail-closed blockers:

- parent directory missing and creation not selected;
- parent path exists but is not a directory;
- parent directory status ambiguous;
- parent directory creation would escape the selected storage root;
- parent directory creation would require unselected side effects.

Parent directory observations are operational evidence only. They do not imply proofpack availability or acceptance.

## Symlink policy

A future symlink policy must be explicit before any filesystem write attempt.

Default boundary: do not silently follow symlinks.

A future writer must treat symlink encounters as blockers unless a later explicit policy allows a narrow behavior with inspectable evidence.

Recommended fail-closed blockers:

- storage root resolves through a symlink and the policy forbids it;
- target path segment resolves through a symlink and the policy forbids it;
- parent directory contains a symlink in a policy-sensitive position;
- final target exists as a symlink;
- symlink status cannot be observed safely.

If a later policy allows any symlink behavior, the resulting observation must record what was allowed and why. It still must not turn symlink resolution into proof authority.

## Canonicalization policy

A future canonicalization policy must be explicit.

This boundary does not canonicalize host paths in code.

A future resolver may need a canonicalization observation to prove a target remains under the selected storage root after path normalization, symlink policy, and platform-specific path behavior are considered.

Canonicalization observations must not silently rewrite user intent.

Recommended fail-closed blockers:

- canonicalization unavailable;
- canonicalization changes the effective storage root unexpectedly;
- canonicalization changes the effective target path outside policy;
- canonicalization depends on a missing filesystem entry when the selected policy requires an existing entry;
- canonicalization result cannot be compared to the storage root safely.

Canonicalized host paths, if recorded, are sensitive operational observations and should be redacted or represented storage-root-relative unless a later policy allows otherwise.

## Traversal and storage-root escape policy

Traversal and storage-root escape must be blockers.

A future writer must reject target path refs that contain or resolve to:

- `..` parent traversal;
- absolute paths;
- platform root prefixes;
- home-directory expansion;
- drive-prefix escape;
- UNC or network-share escape, when relevant;
- empty path segments that alter meaning;
- path separators not selected by policy;
- encoded traversal that decodes to an escape;
- symlink-assisted escape;
- canonicalization-assisted escape.

A future writer must not write when it cannot prove, under selected policy, that the effective target path remains under the selected storage root.

Storage-root escape evidence is blocker evidence, not acceptance evidence.

## Failure / blocker reasons

Recommended blocker reason vocabulary for future models or evals:

| Reason | Meaning |
|---|---|
| `storage_root_ref_missing` | No explicit storage root ref was provided. |
| `storage_root_ref_disallowed` | Storage root ref was not allowed by selected policy. |
| `target_path_ref_missing` | No explicit target path ref was provided. |
| `target_path_ref_invalid` | Target path ref failed selected path policy. |
| `logical_target_artifact_ref_missing` | Intended logical artifact identity was missing when required. |
| `path_encoding_missing` | No selected encoding policy was available. |
| `path_encoding_rejected` | Encoding policy rejected the logical identity or path component. |
| `parent_directory_missing` | Required parent directory was absent or not selected for creation. |
| `parent_directory_not_directory` | Parent path exists but is not a directory. |
| `symlink_disallowed` | Symlink encountered where policy forbids it. |
| `canonicalization_unavailable` | Required canonicalization observation could not be produced. |
| `traversal_detected` | Traversal was detected before or after decoding. |
| `storage_root_escape_detected` | Effective target escaped selected storage root. |
| `host_path_ambiguous` | Host path observation could not be made unambiguous. |
| `host_path_redaction_required` | Observation cannot be safely recorded without redaction policy. |
| `policy_not_selected` | Required host path policy was not selected. |

This vocabulary is advisory/design. It does not add Rust enums or schemas in this patch.

## Evidence model

Host path resolution may produce operational evidence such as:

- selected input refs;
- selected policy refs;
- observation status;
- storage-root-relative target path summary;
- redaction status;
- blockers;
- boundary notes.

Evidence must not claim:

- the proofpack was written;
- `.punk/proofs` exists;
- referenced proofpack artifacts were verified;
- schema validation happened;
- operation evidence was persisted;
- gate accepted the work;
- positive acceptance can be claimed.

A blocker is evidence that a future writer must not proceed. It is not a failure of gate, proof, or acceptance by itself.

## What remains future work

Future bounded goals may define or implement:

- concrete path encoding;
- concrete storage root selection;
- concrete parent directory behavior;
- concrete symlink policy;
- concrete canonicalization behavior;
- concrete traversal and escape checks;
- schema fields for host path observations;
- side-effect-free host path resolution model;
- active writer implementation;
- `.punk/proofs` runtime storage activation;
- operation-evidence persistence;
- proofpack referenced-ref verification integration;
- CLI behavior;
- gate/eval/proof orchestration;
- privacy/redaction policy for local path observations.

Each requires a separate bounded goal.

## Eval implications

Future eval/spec cases should cover:

- storage root ref is required and explicit;
- target path ref is required and explicit;
- logical target artifact ref is not treated as a path;
- host path observation is operational evidence only;
- missing policy fails closed;
- path encoding rejects unsafe characters and collisions;
- parent directory ambiguity blocks writes;
- symlink encounters block by default;
- traversal and encoded traversal block writes;
- storage-root escape blocks writes;
- canonicalization ambiguity blocks writes;
- blocker evidence cannot become acceptance;
- path resolution success cannot imply referenced artifact verification;
- no `.punk/proofs` directory is created by boundary/spec work.
