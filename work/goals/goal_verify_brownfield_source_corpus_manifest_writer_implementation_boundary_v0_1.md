---
id: goal_verify_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1
title: "Verify brownfield source corpus manifest writer implementation boundary v0.1"
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
    - "work/goals/goal_verify_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md"
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
  - "Verifies the source corpus manifest writer implementation boundary before any writer implementation."
  - "Confirms the future writer input, target, preflight, render, atomic write, operation evidence, authority, privacy, and runtime boundaries are explicit."
  - "Confirms no source inventory implementation, repo scan, file walker, content reads, filesystem hashing, manifest writer implementation, runtime storage, or claim extraction was activated."
  - "Records verification evidence and selects the next bounded goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md"
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
  rationale: "This is verification-only for docs/eval implementation-boundary text derived from verified Brownfield Source Corpus Manifest writer preflight model evidence; external research is not needed unless scope changes toward implementation, traversal, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-verification
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Verification should inspect docs/eval/work-ledger evidence and update only work-ledger artifacts unless it finds narrow drift requiring correction."
---

## Context

The source corpus manifest writer implementation boundary has been defined as
docs/eval/spec only. It selects no writer implementation.

## Intent

Verify that the implementation boundary is the last guardrail before any future
source corpus manifest writer slice is selected.

## Verification focus

- Future writer input is an already-constructed `SourceCorpusManifest` model,
  explicit target, and successful preflight result.
- Future writer input is not repo root, directory walk input, raw source file
  input, AI prompt, or claim ledger input.
- Future target is the default manifest path or a configured repo-relative
  target under `.punk/memory/reconstruction/`.
- Future preflight blocks absolute targets, path escape, symlink escape,
  runtime storage target, different existing target, non-advisory manifest,
  non-`observed_structure` authority, claim-like fields, absolute paths,
  content snippets, and summaries.
- Future render is deterministic canonical bytes with no hidden runtime clock,
  host paths, environment values, local usernames, or raw file contents.
- Future atomic write avoids partial targets and keeps identical existing
  content idempotent while blocking different existing content.
- Operation evidence is not proof, gate decision, acceptance, project truth,
  claim ledger, or contract readiness.
- Written manifest remains advisory `observed_structure` and does not activate
  runtime storage, Punk `Writer` behavior, Conformance Pack runtime, Migration
  Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.

## Non-scope

Do not implement repo scanning, file walking, directory traversal, language
detection, manifest writing, hash computation, file content reading, source
corpus manifest generation, claim extraction, claim ledger population, AI
summaries, module maps, architecture recovery, intent recovery, contract
generation, gate/proof runtime, Writer behavior, runtime `.punk` storage,
grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.
