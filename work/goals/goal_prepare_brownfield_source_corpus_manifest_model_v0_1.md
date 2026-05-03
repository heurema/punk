---
id: goal_prepare_brownfield_source_corpus_manifest_model_v0_1
title: "Prepare brownfield source corpus manifest model v0.1"
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
    - "work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Adds only side-effect-free model types for the Brownfield Source Corpus Manifest boundary."
  - "Model preserves repo-relative path, no-content, deferred hash/size, source-class, authority, privacy, and claim boundaries."
  - "Model code does not scan repositories, walk files, read file contents, generate manifests, write runtime storage, or infer claims."
  - "Smoke/eval coverage confirms the model remains side-effect-free and advisory observed structure only."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-design-verification-v0-1.md"
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
  rationale: "This is a side-effect-free model/eval goal derived from the verified B2 source corpus manifest design; external research is not needed unless implementation scope changes toward traversal, scanning, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-design-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-and-docs
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/CRATE-STATUS.md"
  rationale: "The next goal may add side-effect-free model behavior and eval coverage, but must not activate source inventory scanning or manifest writing."
---

## Context

B2 Brownfield Source Corpus Manifest design is defined and verified.

The verified boundary says the manifest is advisory observed structure only. It
is not project truth, not a claim ledger, not a contract, not a decision, not
proof, not a summary, not an architecture map, and not intent recovery.

## Intent

Prepare a side-effect-free model for the manifest boundary before any source
inventory writer, repository traversal, scanner, or CLI behavior is selected.

Potential model surface:

```text
SourceCorpusManifest
SourceCorpusItem
SourceClass
ObservedKind
TrackingStatus
Sensitivity
GeneratedOrVendoredCandidate
PathPolicy
ContentPolicy
HashPolicy
SizePolicy
ManifestAuthority
```

The exact type names should follow existing crate conventions.

## Non-scope

Do not implement repo scanning, file walking, language detection, manifest
writing, hash computation, file content reading, source corpus manifest
generation, claim extraction, claim ledger population, AI summaries, module
maps, architecture recovery, intent recovery, contract generation, gate/proof
runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior.
