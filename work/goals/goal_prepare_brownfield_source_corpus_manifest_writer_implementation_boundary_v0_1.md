---
id: goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1
title: "Prepare brownfield source corpus manifest writer implementation boundary v0.1"
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
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
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
report_refs:
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md"
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
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
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

## Outcome

Done in `work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md`.

The boundary defines the smallest future writer implementation slice:

- accept an already-constructed `SourceCorpusManifest` model;
- accept an explicit target or default safe target;
- require a successful preflight result;
- render deterministic canonical bytes;
- write exactly one target under `.punk/memory/reconstruction/`;
- emit non-authoritative operation evidence only.

It also records that the future writer must not accept repo roots, directory
walk inputs, raw source files, AI prompts, or claim ledger input. It must block
absolute targets, path escape, symlink escape, runtime storage targets,
different existing target content, non-advisory manifest status,
non-`observed_structure` authority, claim-like fields, absolute paths, content
snippets, summaries, and hidden runtime-clock metadata.

The next selected goal is
`work/goals/goal_verify_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md`.

No manifest writer implementation, source inventory implementation, repo scan,
file walker, content reading, hash computation from filesystem, AI summaries,
claim extraction, contract generation, gate/proof runtime, Writer, Conformance
Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or
spec-as-source behavior was activated.
