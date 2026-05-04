---
id: goal_prepare_brownfield_source_corpus_manifest_writer_first_slice_v0_1
title: "Prepare brownfield source corpus manifest writer first slice v0.1"
status: ready
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-04
updated_at: 2026-05-04
selected_at: 2026-05-04
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Prepares the smallest source corpus manifest writer slice only after implementation-boundary verification."
  - "Keeps writer input limited to an already-constructed SourceCorpusManifest model, explicit safe target, and successful preflight."
  - "Keeps rendering deterministic and canonical, with no hidden runtime clock, host paths, environment values, local usernames, raw file contents, or authority promotion."
  - "Keeps writing limited to one explicit safe target under .punk/memory/reconstruction/ with no repo scan, file walk, content read, filesystem hash computation, manifest generation from repo state, claims, runtime storage, or gate/proof runtime."
  - "Records implementation evidence and selects a bounded verification follow-up."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-implementation-boundary-verification-v0-1.md"
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
  rationale: "This first writer slice is derived from verified repo-tracked Source Corpus Manifest boundary, model, writer boundary, preflight model, and implementation-boundary evidence. External research is not needed unless scope changes toward repository traversal, content reads, hashing source files from filesystem, claim extraction, runtime storage, or AI summaries."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-implementation-boundary-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-and-status
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md if behavior boundary wording changes"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md if eval coverage changes"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
  rationale: "The next goal may select the first side-effectful manifest writer slice, so it must keep code, eval, docs, and work-ledger evidence aligned if any implementation is added."
---

## Context

The Source Corpus Manifest writer implementation boundary has been verified.

The first writer slice, if implemented, must stay smaller than source inventory
generation. It may write only a manifest that already exists as a model.

## Intent

Prepare the smallest possible source corpus manifest writer slice.

Allowed future shape:

```text
input: already-constructed SourceCorpusManifest
target: explicit safe target
preflight: must pass
render: deterministic canonical bytes
write: one explicit safe target
evidence: non-authoritative operation evidence
```

## Required boundaries

- No repo scan.
- No directory walk.
- No file content read.
- No filesystem hash computation for source files.
- No manifest generation from repository state.
- No claim extraction.
- No claim ledger population.
- No intent inference.
- No AI summaries.
- No contract generation.
- No runtime storage.
- No gate/proof runtime.
- No Punk `Writer` behavior.
- No authority promotion.
- No Conformance Pack runtime.
- No Migration Contract runtime.
- No Regenerative Spec behavior.
- No spec-as-source behavior.

## Non-scope

Do not implement repo scanning, file walking, directory traversal, language
detection, source corpus manifest generation from repo state, hash computation
from source files, file content reading, claim extraction, claim ledger
population, AI summaries, module maps, architecture recovery, intent recovery,
contract generation, gate/proof runtime, Writer behavior, runtime `.punk`
storage, grayfield reconciliation, Conformance Pack runtime, Migration
Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.
