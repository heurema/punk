---
id: goal_verify_brownfield_source_corpus_manifest_model_v0_1
title: "Verify brownfield source corpus manifest model v0.1"
status: ready
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-03
updated_at: 2026-05-03
selected_at: 2026-05-03
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md"
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
  - "Verifies the Brownfield Source Corpus Manifest model stays side-effect-free."
  - "Confirms the model preserves advisory observed-structure authority, repo-relative path rules, no-content defaults, deferred hash/size policy, and claim boundaries."
  - "Confirms no source inventory implementation, repo scan, file walk, content read, manifest writer, runtime storage, or claim extraction is active."
  - "Records verification evidence and selects the next bounded goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is verification-only for a side-effect-free model derived from verified B2 design; external research is not needed unless scope changes toward traversal, scanning, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: work-ledger
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Verification should update only work-ledger artifacts unless it finds drift that requires a narrow docs or model fix."
---

## Context

The Brownfield Source Corpus Manifest model exists as inert Rust model types and
smoke coverage. It is meant to encode the verified design boundary before any
source inventory implementation is selected.

## Intent

Verify that the model remains advisory observed structure only and cannot be
mistaken for a scanner, writer, claim ledger, contract, proof, or project truth.

## Verification focus

- Manifest status and authority remain advisory / observed structure only.
- Paths remain repo-relative and reject absolute, home/user, traversal, and
  escape-shaped inputs.
- Content policy defaults to no reads, no snippets, and no summaries.
- Hash and size policy remain deferred or unset.
- Model capabilities report no scanning, file walking, content reading, hash
  computation, manifest writing, claims, intent inference, network, or remote
  AI.
- Manifest items do not include claim fields such as `claims_created`.
- Smoke coverage and crate status do not imply active source inventory.

## Non-scope

Do not implement repo scanning, file walking, language detection, manifest
writing, hash computation, file content reading, source corpus manifest
generation, claim extraction, claim ledger population, AI summaries, module
maps, architecture recovery, intent recovery, contract generation, gate/proof
runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior.
