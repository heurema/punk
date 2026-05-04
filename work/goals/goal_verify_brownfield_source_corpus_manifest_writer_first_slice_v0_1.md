---
id: goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1
title: "Verify brownfield source corpus manifest writer first slice v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-05-04
updated_at: 2026-05-04
selected_at: 2026-05-04
started_at: 2026-05-04
completed_at: 2026-05-04
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "Verifies the first writer slice writes only an already-constructed SourceCorpusManifest model to one explicit safe target after preflight."
  - "Verifies deterministic canonical bytes, idempotent identical target handling, conflict blocking for different content, no partial target on failure, and symlink/path/runtime target blocking."
  - "Verifies no repo scan, file walk, source content read, source filesystem hash computation, manifest generation from repository state, claims, authority promotion, runtime storage, CLI command, gate/proof runtime, or Punk Writer behavior was activated."
  - "Records verification evidence and selects the next bounded goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "docs/product/CRATE-STATUS.md"
  - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-verification-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "Verification is grounded in repo-tracked boundary docs, eval spec, code, smoke eval, and first-slice report. External research is not needed unless scope changes toward repository traversal, content reads, hashing source files from filesystem, claim extraction, runtime storage, or AI summaries."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: verification-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md"
  rationale: "The next goal should verify the first side-effectful Brownfield writer slice before any source inventory generation or broader writer behavior is considered."
---

## Context

The first Source Corpus Manifest writer slice has been implemented.

It is intentionally limited to:

```text
already-constructed SourceCorpusManifest
explicit safe target
successful preflight
deterministic canonical bytes
one no-overwrite target write
in-memory non-authoritative operation evidence
```

## Intent

Verify the first writer slice before selecting any broader Brownfield behavior.

## Required verification

- Confirm the writer accepts only an already-constructed model, explicit safe
  target, and matching successful preflight.
- Confirm canonical rendering is deterministic and has no hidden runtime clock,
  host paths, env values, raw file contents, summaries, or claim fields.
- Confirm safe target write works under `.punk/memory/reconstruction/`.
- Confirm absolute target, path escape, runtime target, and symlink ancestor
  cases block.
- Confirm different existing target content conflicts without overwrite.
- Confirm identical existing target is idempotent without rewriting.
- Confirm failed preflight or missing parent leaves no partial target.
- Confirm operation evidence is not proof, gate decision, acceptance, project
  truth, claim ledger, or contract readiness.
- Confirm no repo scan, file walk, source content read, source filesystem hash
  computation, manifest generation from repository state, claims, authority
  promotion, runtime storage, CLI command, gate/proof runtime, Conformance Pack
  runtime, Migration Contract runtime, Regenerative Spec behavior, or
  spec-as-source behavior was activated.

## Non-scope

Do not implement source inventory generation, repo scanning, file walking,
directory traversal, source file content reading, source filesystem hash
computation, manifest item generation from repository state, claim extraction,
claim ledger population, AI summaries, module maps, architecture recovery,
intent recovery, contract generation, gate/proof runtime, Punk `Writer`
behavior, runtime `.punk` storage, CLI behavior, grayfield reconciliation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior.

## Result

Verification completed cleanly.

The first writer slice remains limited to an already-constructed
`SourceCorpusManifest`, explicit safe target, matching successful preflight,
deterministic canonical bytes, one safe target write, and in-memory
non-authoritative operation evidence.

Verification confirmed idempotent existing targets recheck target bytes before
success, different bytes conflict, unreadable target bytes block, missing
parents leave no partial target, and symlink/path/runtime target cases fail
closed.

Selected next:

```text
work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md
```
