# Brownfield Inventory boundary v0.1

Date: 2026-05-03
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic boundary expectations for any future brownfield source corpus inventory before implementation exists.

This is a design/spec artifact only. It does not activate repo scanning, file walking, inventory writing, claim extraction, AI summaries, contract generation, gate/proof runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.

## Status and authority

Future inventory output is advisory observed structure only:

```text
status = advisory
authority = observed_structure
```

It is not canonical project memory truth, accepted behavior, a contract, a gate decision, proof, or claim ledger.

## Required deterministic eval cases

### BFI-001: observable structure only

Given a future inventory item, it may include repo-relative path, path kind, extension, candidate source class, and marker metadata.

It must not include module purpose, requirements, accepted behavior, invariants, non-goals, ownership truth, or intent.

### BFI-002: claims are not created

Given a source artifact that looks meaningful, future inventory must not create a claim such as module purpose, requirement, architecture decision, accepted behavior, or contract clause.

Candidate claims belong to a later reviewed reconstruction layer.

### BFI-003: source classes are bounded

Future inventory source classes must be limited to:

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

If classification is uncertain, `unknown` must be used instead of forced interpretation.

### BFI-004: sensitive classes are not exposed

Future inventory must not expose raw secret values, raw environment values, local cache contents, build output contents, `.git` internals, `.punk/runtime`, `.punk/cache`, `.punk/indexes`, or private agent transcripts.

### BFI-005: repo-relative paths only

Future inventory output must use repo-relative paths.

Absolute host paths such as `/Users/...`, `/home/...`, or drive-rooted local paths must be rejected or redacted.

### BFI-006: no network and no remote AI

Future inventory must be local-only by default.

It must not require network access, remote AI calls, telemetry upload, remote indexing, or hidden sync.

### BFI-007: file contents excluded by default

Future inventory must not store file contents by default.

If a later goal allows hashes or sizes, those fields must remain structural evidence and must not become semantic summaries.

### BFI-008: docs are not canonical by default

Given a README or docs file, future inventory may record that it exists and classify it as `docs`.

It must not state that the doc is canonical, current, accepted, or contract authority.

### BFI-009: tests are not proof

Given test files, future inventory may record that tests exist and classify them as `tests`.

It must not state that tests prove requirements, acceptance, compatibility, or invariants.

### BFI-010: deterministic and atomic output

Future source corpus manifest writing must be deterministic for the same checkout inputs and must preflight output conflicts.

On conflict, it must not leave a partial manifest.

### BFI-011: generated and vendored candidates stay marked

Future inventory must mark generated or vendored material as candidates when detected.

It must not silently treat generated or vendored artifacts as project-authored truth.

### BFI-012: contract readiness remains advisory and future

Future inventory may support later contract-readiness assessment by providing source refs.

It must not decide contract readiness, draft contracts, write gate decisions, create proof, or accept claims.

### BFI-013: observation packet stays advisory observed structure

A future source inventory observation packet must carry:

```text
packet_status = advisory
authority = observed_structure
```

It must not be canonical project memory truth, accepted behavior, a contract,
a gate decision, proof, acceptance, or claim authority.

### BFI-014: observation packet uses explicit scoped inputs

A future observation packet must record explicit include and exclude scope,
source root ref, repo-relative artifact refs, evidence ids, warnings, blockers,
and limitations.

Unbounded include rules, absolute paths, parent traversal, symlink target host
paths, and missing evidence ids must fail closed.

### BFI-015: observation packet does not read or store content

A future observation packet must not contain source contents, code snippets,
document excerpts, raw secret values, raw environment values, semantic
summaries, filesystem hashes, or file sizes by default.

Hashes and sizes require a later bounded boundary and remain structural
evidence only if enabled.

### BFI-016: observation packet preserves uncertainty

Ambiguous source class, sensitivity, generated/vendored status, ignored status,
and missing evidence must remain visible as candidates, warnings, blockers,
limitations, or `unknown`.

The packet must not force classification or convert missing evidence into a
semantic claim.

### BFI-017: local labs are advisory only

`code-intel-kernel` may inform observation-packet shape, warning vocabulary,
evidence-id requirements, selector boundaries, and missing-evidence handling.

It must not import lab code, lab state, lab output, or lab benchmark results as
Punk product truth.

### BFI-018: future active observer results require evaluation route

Future active observer/scanner results must define an `agent-bench-lab`
evaluation route before they influence Brownfield decisions.

The route must cover result validity, evidence coverage, deterministic
repeatability, missing-evidence visibility, policy violations, hidden or
mutation cases when available, and invalid-run handling.

### BFI-019: benchmark results are not authority

An `agent-bench-lab` run result may be evidence for review readiness only.

It must not become a gate decision, proof, acceptance, contract readiness,
project truth, or authority promotion.

### BFI-020: observation packet does not activate implementation

This boundary must not activate source inventory generation, repo scanning,
file walking, source content reading, source filesystem hashing, size
collection, manifest generation from repository state, claim extraction, AI
summaries, runtime storage, CLI behavior, or broader Writer behavior.

### BFI-021: codebase study is a separate future module

Future codebase study must be represented as a separate Unix-style Punk module
when implementation is selected.

The canonical module id for this boundary is:

```text
codebase-study
```

Brownfield core, `punk-project`, Source Corpus Manifest writer behavior, and
generic Punk `Writer` behavior must not silently absorb codebase study as an
implicit scanner.

### BFI-022: codebase study module returns packet not authority

A future codebase-study module may return an advisory observation packet from
explicit source observation input and narrow capability grants.

It must not return final Brownfield decisions, accepted claims, contract
approval, gate decisions, proof, acceptance, runtime storage writes, or Source
Corpus Manifest writer authority.

## Minimal fixture shape

This is illustrative boundary shape only, not an implemented schema.

```yaml
inventory_item:
  path: crates/example/src/lib.rs
  path_kind: file
  source_class: source_code
  markers:
    - extension: rs
  status: advisory
  authority: observed_structure
  semantic_summary: null
  claims_created: []
```

Observation packet fixture shape:

```yaml
observation_packet:
  schema_version: brownfield-source-inventory-observation-packet.v0.1
  packet_status: advisory
  authority: observed_structure
  source_root_ref:
    kind: repo_root
    path: .
  observation_scope:
    include:
      - .
    exclude:
      - .git
      - .punk/runtime
      - .punk/cache
      - .punk/indexes
      - node_modules
      - target
  observations:
    - repo_relative_path: crates/example/src/lib.rs
      observed_kind_candidate: file
      source_class_candidate: source_code
      sensitivity_candidate: normal
      evidence_ids:
        - evidence:explicit-observer-input
  warnings: []
  blockers: []
  limitations:
    - no_content_read
    - no_hashes
    - no_sizes
    - no_claims
```

## Non-goals

This v0.1 boundary does not define runtime inventory storage, `.punk/` inventory writers, CLI commands, repository traversal, language detection, file content extraction, hashing behavior, claim ledger population, contract-readiness scoring, AI summaries, gate/proof runtime, Writer behavior, or grayfield reconciliation.

## Future implementation constraints

Any future implementation must keep inventory as observable structure, keep claims in a later reviewed reconstruction layer, write repo-relative paths, avoid file contents by default, use allowlist/denylist rules, preserve privacy, and remain advisory until reviewed.
