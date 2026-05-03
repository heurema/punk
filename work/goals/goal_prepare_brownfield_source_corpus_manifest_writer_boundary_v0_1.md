---
id: goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1
title: "Prepare brownfield source corpus manifest writer boundary v0.1"
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
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Defines the future writer boundary before any manifest writer implementation."
  - "Separates writer preflight, inventory selection, model construction, atomic write policy, and report evidence."
  - "Preserves no repo scan implementation, no file walker, no content reads, no filesystem hashing, no manifest generation, no runtime storage, and no claim extraction."
  - "Records docs/eval/work evidence and selects the next bounded verification goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-model-verification-v0-1.md"
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
  rationale: "This is a boundary/design-only goal derived from the verified manifest design and model; external research is not needed unless scope changes toward implementation, traversal, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-model-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  rationale: "The goal should define a future writer boundary only and must not add code or active manifest writer behavior."
---

## Context

The Brownfield Source Corpus Manifest design and side-effect-free model are
verified. A future writer is still not selected or implemented.

## Intent

Define the writer boundary before any source inventory writer, repository
traversal, manifest generation, or filesystem write is implemented.

## Boundary questions

- What preflight must pass before a future writer may write a manifest?
- How does a future writer consume already-approved inventory inputs without
  scanning by implication?
- What atomic/no-partial-write behavior is required?
- What paths are allowed for output, and how are repo-relative refs preserved?
- What runtime directories remain forbidden?
- What evidence is reported without becoming claims, contracts, proof, or
  project truth?

## Non-scope

Do not implement repo scanning, file walking, language detection, manifest
writing, hash computation, file content reading, source corpus manifest
generation, claim extraction, claim ledger population, AI summaries, module
maps, architecture recovery, intent recovery, contract generation, gate/proof
runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior.
