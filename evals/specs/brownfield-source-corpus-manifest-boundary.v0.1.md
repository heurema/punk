# Brownfield Source Corpus Manifest boundary v0.1

Date: 2026-05-03
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary expectations for a future Brownfield Source
Corpus Manifest before implementation exists.

This is an eval/spec artifact only. It does not activate repo scanning, file
walking, language detection, manifest writing, hash computation, file content
reading, claim extraction, AI summaries, contract generation, gate/proof
runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior.

## Required future eval cases

### BSCM-001: manifest uses repo-relative paths

Future manifest items must use repo-relative paths only.

Absolute host paths, home paths, URL-like refs, parent traversal, and
backslash path separators must be rejected or redacted.

### BSCM-002: manifest has no absolute path leakage

Future manifest output must not include `/Users/...`, `/home/...`, drive-rooted
paths, or symlink target paths that expose local host layout.

### BSCM-003: manifest does not store file contents

Future manifest output must not store file contents, code snippets, document
excerpts, raw environment values, raw secret values, or semantic summaries.

### BSCM-004: manifest items do not contain claim fields

Future manifest items must not contain purpose, requirement, intent,
invariant, owner truth, architecture decision, accepted behavior, contract
readiness conclusion, risk, severity, proof status, or gate decision fields.

### BSCM-005: manifest authority is observed structure

Future manifest output must carry:

```text
manifest_status = advisory
authority = observed_structure
```

It must not claim canonical, accepted, contract, decision, proof, or project
truth authority.

### BSCM-006: manifest excludes runtime and cache dirs

Future manifest output must exclude `.punk/runtime`, `.punk/cache`,
`.punk/indexes`, `.git`, `node_modules`, `target`, local caches, and build
outputs by default.

### BSCM-007: generated and vendored stay candidates

Future manifest output must mark generated or vendored material as candidates
when detected.

It must not silently treat generated or vendored material as project-authored
truth.

### BSCM-008: no network and no remote AI

Future manifest generation must be local-only by default.

It must not require network access, remote AI calls, telemetry upload, hidden
sync, or remote indexing.

### BSCM-009: no contract or proof claims

Future manifest output may provide source refs for later reviewed work.

It must not draft contracts, decide contract readiness, write gate decisions,
create proof, or accept claims.

### BSCM-010: hashes are deferred by default

Future first implementation must not require hashes.

Optional hash fields may exist only as deferred/null structural slots until a
later bounded goal enables hashing.

### BSCM-011: size metadata is deferred by default

Future first implementation must not require file sizes.

Optional size fields may exist only as deferred/null structural slots until a
later bounded goal enables size recording.

### BSCM-012: item vocabulary is bounded

Future manifest `source_class` values must stay within the B1 vocabulary:

```text
source_code
docs
tests
ci_config
package_manifest
schema
migration
script
generated_candidate
vendored_candidate
unknown
```

If classification is uncertain, `unknown` must be used.

### BSCM-013: writer_target_is_reconstruction_manifest_path

A future source corpus manifest writer may write only to
`.punk/memory/reconstruction/source-corpus-manifest.md` or to an explicitly
configured repo-relative target under `.punk/memory/reconstruction/`.

It must not infer hidden output targets.

### BSCM-014: writer_rejects_path_escape

A future writer preflight must reject parent traversal, normalized path escape,
and target paths outside `.punk/memory/reconstruction/`.

### BSCM-015: writer_rejects_absolute_target

A future writer preflight must reject absolute host paths, home/user paths,
drive-rooted paths, URL-like targets, and other non repo-relative targets.

### BSCM-016: writer_rejects_symlink_escape

A future writer preflight must reject target parent paths where any symlink
ancestor can redirect the write outside the repository or outside the allowed
reconstruction directory.

### BSCM-017: writer_preflight_before_write

A future writer must validate target path, parent directory, path escape,
symlink ancestor escape, target conflict policy, manifest authority, manifest
status, claim-field absence, content/snippet absence, and absolute-path absence
before writing any bytes.

### BSCM-018: writer_no_partial_write_on_conflict

A future writer must not leave partial files when a target conflict, invalid
manifest, invalid target, or atomic replacement failure is detected.

Missing targets may be written after clean preflight. Identical content may be
idempotent. Different existing content must block unless a later explicit
overwrite boundary is selected.

### BSCM-019: writer_blocks_claim_fields

A future writer must reject manifest data containing claim-like fields such as
intent, requirement, module purpose, architecture decision, accepted behavior,
invariant, contract, proof, `claims_created`, or project truth.

### BSCM-020: writer_blocks_absolute_paths

A future writer must reject or redact absolute paths in both target refs and
manifest data.

Manifest output must remain repo-relative.

### BSCM-021: writer_blocks_content_snippets

A future writer must reject manifest data containing file contents, code
snippets, document excerpts, semantic summaries, raw secret values, or raw
environment values.

### BSCM-022: writer_does_not_activate_runtime_storage

A future source corpus manifest writer must not create or write `.punk/runtime`,
`.punk/events`, `.punk/runs`, `.punk/decisions`, `.punk/proofs`, or other
runtime authority surfaces.

### BSCM-023: writer_operation_evidence_is_not_proof

Any future writer operation evidence may describe write attempt and outcome
only.

It must not become proof, gate decision, acceptance, contract, project truth,
or claim authority.

### BSCM-024: writer_does_not_create_gate_decision

A future writer must not create gate decisions, acceptance outcomes, proofpack
authority, or final project decisions.

### BSCM-025: writer_does_not_create_claim_ledger

A future writer must not create or populate a claim ledger and must not promote
manifest observations into reconstructed claims.

## Future writer implementation-boundary eval cases

These cases define future eval requirements only. They do not implement writer
tests or writer behavior.

### BSCM-026: writer_accepts_only_preflight_pass

A future writer may proceed only when supplied an already-constructed manifest
model, an explicit safe target, and a preflight result with no blocking
findings for that same model and target.

### BSCM-027: writer_rejects_blocking_preflight

A future writer must refuse to write when preflight contains any blocking
finding, including absolute target, path escape, symlink escape, different
existing target, unknown target state, missing or unknown parent,
non-advisory manifest status, non-`observed_structure` authority, claim-like
fields, absolute paths, content snippets, summaries, or runtime storage target.

### BSCM-028: writer_input_is_manifest_model_not_repo_scan

A future writer input must be an already-constructed `SourceCorpusManifest`
model plus explicit target and preflight result.

It must not take a repository root to scan, directory list to walk, raw source
files to inspect, AI prompt, or claim ledger input.

### BSCM-029: writer_render_is_deterministic

Future rendering must produce deterministic canonical bytes from the manifest
model with stable field order and no hidden runtime metadata.

The same model input must produce the same bytes.

### BSCM-030: writer_render_has_no_host_paths

Future rendered bytes must not contain absolute paths, home/user paths,
drive-rooted paths, symlink target host paths, local usernames, or environment
paths.

### BSCM-031: writer_render_has_no_hidden_runtime_clock

Future rendering must not read the runtime clock implicitly.

Any timestamp in rendered output must come from manifest model input unless a
later bounded goal explicitly allows clock injection.

### BSCM-032: writer_writes_only_safe_target

A future writer may write only
`.punk/memory/reconstruction/source-corpus-manifest.md` or a configured
repo-relative target under `.punk/memory/reconstruction/`.

It must not write absolute targets, escaped targets, symlink-escaped targets,
hidden inferred targets, runtime storage, cache, index, proof, decision, run,
or event paths.

### BSCM-033: writer_blocks_different_existing_target

A future writer must block when the existing target content differs from the
canonical bytes unless a later reviewed overwrite boundary is selected.

No overwrite flag is selected by this boundary.

### BSCM-034: writer_idempotent_on_identical_target

A future writer may treat identical existing target content as idempotent when
preflight passes and the canonical bytes match exactly.

### BSCM-035: writer_no_partial_target_on_failure

A future writer must leave no partial target when preflight fails, canonical
rendering fails, temporary write fails, flush/fsync policy fails, atomic rename
is unavailable or fails, or conflict detection blocks the operation.

### BSCM-036: writer_operation_evidence_is_not_proof

Future operation evidence may report attempted, blocked, written, idempotent,
conflict, or error outcomes only.

It must not be proof or proofpack authority.

### BSCM-037: writer_operation_evidence_is_not_acceptance

Future operation evidence must not be acceptance, gate decision, final project
decision, contract approval, or project truth.

### BSCM-038: writer_does_not_promote_manifest_authority

A written manifest must remain:

```text
manifest_status = advisory
authority = observed_structure
```

Writing the file must not promote the manifest to canonical truth, accepted
behavior, contract, gate decision, proof, or project memory truth.

### BSCM-039: writer_does_not_create_claims

A future writer must not create claims, claim ledger entries, contract
readiness conclusions, requirements, invariants, intent recovery, architecture
decisions, accepted behavior, or AI summaries.

### BSCM-040: writer_does_not_activate_runtime_storage

A future writer must not activate or write `.punk/runtime`, `.punk/events`,
`.punk/runs`, `.punk/decisions`, `.punk/proofs`, `.punk/cache`, `.punk/indexes`,
gate/proof runtime, Punk `Writer` behavior, Conformance Pack runtime,
Migration Contract runtime, Regenerative Spec behavior, or spec-as-source
behavior.

## Minimal fixture shape

This is illustrative boundary shape only, not an implemented schema.

```yaml
manifest:
  schema_version: brownfield-source-corpus-manifest.v0.1
  manifest_status: advisory
  authority: observed_structure
  source_root_ref:
    kind: repo_root
    path: .
  items:
    - item_id: item:crates/example/src/lib.rs:file
      repo_relative_path: crates/example/src/lib.rs
      observed_kind: file
      source_class: source_code
      source_markers:
        - extension: rs
      content_policy:
        read_contents: false
        store_snippets: false
        summarize_contents: false
      hash_policy:
        status: deferred
        value: null
      size_policy:
        status: deferred
        value: null
```

## Non-goals

This v0.1 boundary does not implement runtime inventory storage, `.punk/`
manifest writers, CLI commands, repository traversal, language detection, file
content extraction, hashing behavior, size recording, claim ledger population,
contract-readiness scoring, AI summaries, gate/proof runtime, Writer behavior,
or grayfield reconciliation.
