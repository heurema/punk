---
id: goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1
title: "Prepare brownfield source corpus manifest writer preflight model v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-03
updated_at: 2026-05-03
selected_at: 2026-05-03
started_at: 2026-05-03
completed_at: 2026-05-03
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md"
    - "work/reports/**"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Adds a side-effect-free source corpus manifest writer preflight model only."
  - "Represents future target, path escape, symlink ancestor, conflict, authority, content, absolute-path, and claim-field checks without writing files."
  - "Preserves no repo scan, no file walker, no content reads, no filesystem hashing, no manifest generation, no manifest writer implementation, no runtime storage, and no claim extraction."
  - "Adds focused tests/eval coverage and records work-ledger evidence."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-boundary-verification-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a side-effect-free model goal derived from verified Brownfield Source Corpus Manifest writer boundary docs; external research is not needed unless scope changes toward implementation, traversal, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-boundary-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-and-status
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "A side-effect-free model should update crate status and work evidence, but must not activate a writer, scanner, runtime storage, or CLI behavior."
---

## Context

The source corpus manifest writer boundary has been defined and verified as a
future write-policy boundary only.

## Intent

Encode the writer preflight rules as inert Rust/domain model state before any
future writer implementation is selected.

The model may represent:

- target path allowed or blocked;
- parent directory validity as declared evidence;
- path escape and absolute target findings;
- symlink ancestor escape findings as explicit supplied evidence;
- conflict policy readiness;
- manifest status and authority checks;
- content/snippet/summary/absolute-path blockers;
- claim-field blockers.

The model must not collect this evidence from the filesystem.

## Non-scope

Do not implement repo scanning, file walking, directory traversal, language
detection, manifest writing, hash computation, file content reading, source
corpus manifest generation, claim extraction, claim ledger population, AI
summaries, module maps, architecture recovery, intent recovery, contract
generation, gate/proof runtime, Writer behavior, runtime `.punk` storage,
grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.

## Outcome

Done in `work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md`.

The source corpus manifest writer preflight model now represents future target,
path escape, symlink ancestor, parent, conflict, manifest authority, content,
absolute-path, claim-field, runtime-storage, and operation-evidence checks from
explicit caller-provided inputs.

The model is side-effect-free. It does not scan repositories, walk files, read
file contents, compute hashes from filesystem, generate manifests, write
manifests, create claims, infer intent, use network or remote AI, write runtime
storage, create gate decisions, create proof, create acceptance, or create
project truth.
