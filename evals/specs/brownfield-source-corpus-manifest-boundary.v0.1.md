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
      claims_created: []
```

## Non-goals

This v0.1 boundary does not define runtime inventory storage, `.punk/`
manifest writers, CLI commands, repository traversal, language detection, file
content extraction, hashing behavior, size recording, claim ledger population,
contract-readiness scoring, AI summaries, gate/proof runtime, Writer behavior,
or grayfield reconciliation.
