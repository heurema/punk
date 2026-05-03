---
id: goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1
title: "Prepare brownfield source corpus manifest writer implementation boundary v0.1"
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
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Defines the minimal future source corpus manifest writer implementation boundary before any writer implementation."
  - "Specifies target, preflight, canonical bytes, temp/atomic write, conflict, operation evidence, runtime-storage, and claim-boundary rules."
  - "Preserves no source inventory implementation, repo scan, file walker, content reads, filesystem hashing, manifest writer implementation, runtime storage, or claim extraction."
  - "Records docs/eval/work evidence and selects the next bounded verification goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-verification-v0-1.md"
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
  rationale: "This is a boundary/design-only goal derived from verified Brownfield Source Corpus Manifest writer preflight model evidence; external research is not needed unless scope changes toward implementation, traversal, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  rationale: "The goal should define a future writer implementation boundary only and must not add code or active manifest writer behavior."
---

## Context

The Brownfield Source Corpus Manifest design, manifest model, writer boundary,
and writer preflight model have been defined and verified. No source inventory
writer is active.

## Intent

Define the last implementation boundary before any future source corpus
manifest writer code is selected.

This is still boundary/design only. It should specify what a later minimal
writer implementation may do, what it must refuse, and what evidence it may
return without becoming proof, gate decision, acceptance, or project truth.

## Boundary questions

- What exact target may the first writer implementation write?
- What preflight result is required before bytes can be written?
- What bytes are canonical and what metadata must stay non-authoritative?
- What temp/atomic write policy is required?
- How are missing, identical, different, unknown, and partial targets handled?
- What operation evidence can be reported?
- How are runtime storage, claims, content reads, filesystem hashes, repo scan,
  file walking, and AI summaries kept inactive?

## Non-scope

Do not implement repo scanning, file walking, directory traversal, language
detection, manifest writing, hash computation, file content reading, source
corpus manifest generation, claim extraction, claim ledger population, AI
summaries, module maps, architecture recovery, intent recovery, contract
generation, gate/proof runtime, Writer behavior, runtime `.punk` storage,
grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.
