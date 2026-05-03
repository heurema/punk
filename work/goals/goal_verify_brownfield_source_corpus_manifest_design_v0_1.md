---
id: goal_verify_brownfield_source_corpus_manifest_design_v0_1
title: "Verify brownfield source corpus manifest design v0.1"
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
    - "work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Verification confirms the manifest design is schema/policy only and does not implement source inventory."
  - "Verification confirms repo-relative path, no-content, deferred hash/size, source-class, authority, privacy, and claim boundaries are explicit."
  - "Verification confirms no repo scan, file walker, manifest writer, content reading, AI summaries, claims, contracts, gate/proof runtime, Writer, runtime storage, or grayfield behavior was added."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-design-v0-1.md"
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
  rationale: "This is a repo-local verification/status goal after the B2 source corpus manifest design; external research is not needed unless direction changes."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-design-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The verification goal should record evidence only and not change product behavior."
---

## Context

Brownfield Source Corpus Manifest design v0.1 is defined as docs/eval only.

## Intent

Verify the design before selecting any source corpus manifest model or
implementation.

## Non-scope

Do not implement repo scanning, file walking, language detection, manifest
writing, hash computation, file content reading, source corpus manifest
generation, claim extraction, claim ledger population, AI summaries, module
maps, architecture recovery, intent recovery, contract generation, gate/proof
runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior.
