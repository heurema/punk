---
id: docs_product_brownfield_inventory
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-06-08
review_after: 2026-07-20
canonical_for:
  - brownfield-inventory-boundary
  - brownfield-source-inventory-observation-packet-boundary
  - source-corpus-manifest-boundary
  - inventory-item-authority
  - brownfield-claim-separation
  - brownfield-inventory-privacy-boundary
related_docs:
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
  - docs/modules/codebase-study.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/GLOSSARY.md
related_evals:
  - evals/specs/brownfield-inventory-boundary.v0.1.md
  - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
supersedes: []
superseded_by: null
---
# Brownfield Inventory

## Purpose

This document defines the B1 boundary for future brownfield inventory work.

It exists because brownfield projects start with existing code, docs, tests, history, and configuration that can be observed but must not automatically become Punk truth.

B0 is already active as an entry scaffold:

```text
punk init <project-id> --mode brownfield
```

B1 is only this boundary/design layer.

Future source corpus inventory implementation remains inactive.

`docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md` owns the B2 manifest
schema and policy design. It is still design/spec only and does not implement
inventory.

## Status and authority

This boundary is canonical product guidance for future brownfield inventory work.

It does not implement a file walker, repository scanner, source corpus manifest writer, claim extractor, module map, AI summary, contract generator, gate/proof runtime, Writer behavior, runtime `.punk` storage, or grayfield reconciliation.

The authority model for any future brownfield inventory output is:

```text
status = advisory
authority = observed_structure
```

Future inventory output must not claim:

```text
status = canonical
authority = accepted
authority = contract
authority = decision
authority = proof
```

## Module ownership boundary

Future codebase study should be owned by a separate Punk module, not by
Brownfield core, `punk-project`, the Source Corpus Manifest writer, or generic
Punk `Writer` behavior.

The Unix-style role for that future module is:

```text
explicit source observation request
  -> codebase study module
  -> advisory source inventory observation packet
```

That module may later produce an observation packet only through explicit input
and narrow capability grants. The packet is a handoff artifact, not final
authority.

The module must not own:

- final Brownfield decisions;
- contract approval;
- gate decisions;
- proof or acceptance;
- Source Corpus Manifest writer behavior;
- runtime `.punk` storage;
- broad Punk orchestration.

Brownfield core may define packet and handoff contracts. It must not silently
absorb codebase study into an implicit core scanner.

## Core rule

Brownfield inventory records observable structure only.

It may say:

```text
file exists
directory exists
package manifest exists
test file exists
CI config exists
README exists
schema file exists
git repository metadata is present
```

It must not say:

```text
this module means X
this feature is intended to do Y
this test proves requirement Z
this README is canonical
this behavior must be preserved
this file defines a contract
```

Existing code, docs, tests, and history are evidence sources. They are not Punk truth until reviewed and promoted through the normal authority path.

## Term boundaries

| Term | Boundary definition | Must not do |
|---|---|---|
| Source Corpus Manifest | Future deterministic inventory of observable source, docs, tests, CI, schema, migration, script, and config artifacts. | Become project memory truth, a claim ledger, a contract, a proof artifact, or a gate decision. |
| Inventory Item | One observable fact about a repo-relative artifact, such as path existence, extension, candidate source class, manifest type, doc/test/config marker, and future explicitly allowed size/hash metadata. | Contain semantic intent, requirements, module purpose, accepted behavior, invariants, ownership truth, or project truth. |
| Claim | An interpretation that needs source refs, confidence, and review before it can influence contracts or accepted project memory. | Be created by inventory. |
| Contract Readiness | Future advisory assessment that enough reviewed information may exist to draft a bounded contract. | Be decided by inventory or treated as gate/proof authority. |

## Allowed inventory outputs

Future inventory may record only low-level observed structure by default:

- repo-relative path exists;
- path kind is file or directory;
- file extension or known basename marker;
- source class candidate;
- known package manifest type;
- doc/test/config/schema/migration/script marker;
- generated, vendored, or unknown candidate marker;
- git repository metadata presence;
- size or hash only if a later bounded goal explicitly allows it.

Future inventory must avoid file contents by default.

Future inventory must not produce semantic summaries.

## Source inventory observation packet boundary

A future source inventory observation packet is the bounded input envelope
between a future Brownfield observer and the existing Source Corpus Manifest
model track.

The future observer is expected to be a separate Punk codebase-study module
once implementation is selected. This boundary does not name, implement, or
activate that module.

The canonical parked module boundary now lives in
`docs/modules/codebase-study.md`.

It is not a scanner result, not a manifest, not a claim ledger, not project
truth, and not a command to write `.punk/` state.

Required packet authority:

```text
packet_status = advisory
authority = observed_structure
```

An observation packet may contain only explicit observed structure supplied by
the future observer:

- packet id and schema version;
- source root ref with `kind = repo_root` and `path = .`;
- observation scope with explicit include and exclude rules;
- repo-relative artifact refs;
- observed path kind candidates;
- source class candidates from the bounded B1 vocabulary;
- sensitivity candidates such as `normal`, `caution`, `sensitive`,
  `excluded`, or `unknown`;
- generated, vendored, ignored, or unknown candidate markers;
- evidence ids for each observation;
- warnings, blockers, and limitations;
- lab reference notes that remain advisory;
- evaluation requirement refs for future active observer/scanner runs.

It must not contain:

- source file contents;
- code snippets;
- document excerpts;
- raw environment values;
- raw secret values;
- filesystem hashes;
- file sizes;
- semantic summaries;
- user intent;
- requirements;
- module purpose;
- architecture decisions;
- accepted behavior;
- contract readiness conclusions;
- gate decisions;
- proof or acceptance claims.

Hashes and sizes remain excluded from the observation packet until a later
bounded goal explicitly enables them. Even then, they would remain structural
evidence only.

Minimal illustrative packet shape:

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
    - observation_id: obs:crates/example/src/lib.rs:file
      repo_relative_path: crates/example/src/lib.rs
      observed_kind_candidate: file
      source_class_candidate: source_code
      sensitivity_candidate: normal
      generated_or_vendored_candidate: none
      evidence_ids:
        - evidence:explicit-observer-input
      confidence: explicit_observation
  warnings: []
  blockers: []
  limitations:
    - no_content_read
    - no_hashes
    - no_sizes
    - no_claims
```

Fail closed when a packet or future observer output includes:

- absolute host paths;
- parent traversal or normalized path escape;
- symlink target expansion outside repo-relative refs;
- unbounded include rules;
- missing exclude coverage for default runtime/cache/vendor/build paths;
- observations without evidence ids;
- private local paths or unredacted sensitive path segments;
- raw source contents, snippets, docs excerpts, secrets, or environment values;
- hashes or sizes without a later accepted boundary;
- semantic claims, intent, requirements, or contract-readiness conclusions;
- lab results or benchmark results presented as product authority;
- active observer/scanner results without an `agent-bench-lab` evaluation plan.

`code-intel-kernel` may be used only as an advisory local lab for packet-shape
ideas such as explicit selectors, evidence ids, warnings, limitations,
candidate ranking, and fail-closed missing-evidence handling. Its local checkout
state must be verified before reuse and must not become Punk product truth by
reference.

Future active observer/scanner results must define an `agent-bench-lab`
evaluation route before they influence Brownfield decisions. The evaluation
requirement should cover result validity, evidence coverage, deterministic
repeatability, missing-evidence visibility, policy violations, hidden or
mutation cases when available, and invalid-run handling. Benchmark or lab
results are not gate decisions, proof, acceptance, contract readiness, or
project truth.

## Forbidden inferences

Future inventory must not infer:

- user intent;
- requirements;
- module purpose;
- architecture decisions;
- canonical docs;
- accepted behavior;
- invariants;
- non-goals;
- ownership truth;
- project memory truth;
- contract readiness;
- bug versus feature classification;
- risk or severity;
- compatibility or conformance.

Tests are not proof.

Observed behavior is not intent.

Generated summaries are not contracts.

README files and docs are source refs, not canonical truth by default.

## Source classes

Allowed future inventory source classes:

| Source class | Meaning | Boundary |
|---|---|---|
| `source_code` | Source-like code artifact. | File identity only; no module purpose or behavior claim. |
| `docs` | Documentation-like artifact. | Presence only; not canonical by default. |
| `tests` | Test-like artifact. | Presence only; does not prove requirements. |
| `ci_config` | CI or workflow configuration. | Presence only; does not prove release policy. |
| `package_manifest` | Package, dependency, or workspace manifest. | Manifest type marker only by default. |
| `schema` | Schema-like artifact. | Presence/type marker only; not a contract by default. |
| `migration` | Migration-like artifact. | Presence/type marker only; no data model intent claim. |
| `script` | Script or automation-like artifact. | Presence/type marker only; no operational authority claim. |
| `generated_candidate` | Artifact that appears generated. | Must be marked candidate, not ignored or trusted silently. |
| `vendored_candidate` | Artifact that appears vendored. | Must be marked candidate, not treated as project-authored truth. |
| `unknown` | Artifact not safely classified. | Prefer `unknown` over forced classification. |

Forbidden or sensitive classes that require caution:

| Class | Rule |
|---|---|
| `secrets` | Do not record raw secret values or secret-like snippets. |
| `env_files` | Do not record raw environment values. |
| `local_cache` | Exclude or mark non-authoritative cache material. |
| `build_output` | Exclude or mark generated output. |
| `node_modules` | Exclude by default as dependency/vendor material. |
| `target` | Exclude by default as build output. |
| `.git` | Do not inventory internals by default; metadata presence is enough. |
| `.punk/runtime` | Exclude runtime state from source corpus inventory. |
| `.punk/cache` | Exclude cache state from source corpus inventory. |
| `.punk/indexes` | Exclude derived indexes from source corpus inventory. |
| `private_agent_transcripts` | Exclude unless a future explicit import boundary is accepted. |

Future implementation may add more denylisted paths, but it must not remove these cautions without a separate boundary update.

## Privacy and no-network rules

Future inventory must be local-only by default.

Required rules:

- no network access;
- no remote AI calls;
- no raw secret values in reports;
- no raw environment values in reports;
- no local absolute path leakage;
- repo-relative paths only;
- redaction for sensitive path segments;
- no public report output that includes private local content;
- no hidden upload, sync, telemetry, or remote indexing.

If a future inventory path needs secret detection, it may record only safe diagnostic categories, never secret values.

## Future B2 implementation constraints

Any future source corpus manifest implementation must:

- use explicit allowlist and denylist rules;
- write repo-relative paths only;
- avoid file contents by default;
- avoid semantic summaries;
- avoid hidden project truth;
- be deterministic for the same checkout inputs;
- preflight writes and avoid partial manifests on conflict;
- mark generated, vendored, and unknown candidates explicitly;
- keep inventory output separate from claim ledger output;
- keep `claim-ledger.md` empty unless a later reconstruction goal creates reviewed claim candidates;
- preserve `status = advisory` and `authority = observed_structure`.

Future B2 may produce a source corpus manifest only after this boundary is verified.

## How inventory may support later work

The safe future path is:

```text
inventory item
  -> source ref
  -> candidate claim
  -> review
  -> accepted baseline or explicit unknown
  -> bounded contract input
```

Inventory cannot skip directly to claim, contract, gate, proof, or project truth.

## Non-goals

This boundary does not implement:

- repo scanning;
- file walking;
- language detection;
- inventory writing;
- source corpus manifest generation;
- claim extraction;
- claim ledger population;
- AI summaries;
- LLM calls;
- module maps;
- architecture recovery;
- intent recovery;
- contract generation;
- Contract Context Pack generation;
- Conformance Pack runtime;
- Migration Contract runtime;
- Regenerative Spec behavior;
- gate/proof runtime;
- Writer behavior;
- runtime `.punk` storage;
- grayfield reconciliation.
