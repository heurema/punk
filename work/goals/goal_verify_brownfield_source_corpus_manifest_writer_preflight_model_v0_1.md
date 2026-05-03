---
id: goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1
title: "Verify brownfield source corpus manifest writer preflight model v0.1"
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
    - "work/goals/goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md"
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
  - "Verifies the source corpus manifest writer preflight model remains side-effect-free."
  - "Confirms target/path/conflict/symlink/authority/content/claim/runtime findings are explicit and fail closed where required."
  - "Confirms no source inventory implementation, repo scan, file walker, content reads, filesystem hashing, manifest writer, runtime storage, or claim extraction was activated."
  - "Records verification evidence and selects the next bounded goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-verification-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is verification-only for a side-effect-free model derived from verified Brownfield Source Corpus Manifest writer boundary docs; external research is not needed unless scope changes toward implementation, traversal, content reads, claims, or runtime storage."
  research_refs:
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-and-status
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Verification should inspect code/eval/status and update only work-ledger evidence unless it finds narrow drift requiring correction."
---

## Context

The source corpus manifest writer boundary has been defined and verified. The
preflight model now exists as side-effect-free Rust/domain model state.

## Intent

Verify that the preflight model represents future write readiness without
performing writes or collecting filesystem evidence.

## Verification focus

- Allowed target stays under `.punk/memory/reconstruction/`.
- Absolute targets, path escape, runtime storage targets, symlink ancestor
  escape, missing parents, unknown evidence, and different existing targets
  block future writes.
- Manifest status remains advisory.
- Manifest authority remains `observed_structure`.
- Content snippets, summaries, absolute paths, and claim fields including
  `claims_created` block future writes.
- Operation evidence remains evidence only, not proof, gate decision,
  acceptance, or project truth.
- Capabilities report no repo scan, file walk, content read, filesystem hash,
  manifest generation, manifest write, claim creation, intent inference,
  network, remote AI, or runtime storage writes.

## Non-scope

Do not implement repo scanning, file walking, directory traversal, language
detection, manifest writing, hash computation, file content reading, source
corpus manifest generation, claim extraction, claim ledger population, AI
summaries, module maps, architecture recovery, intent recovery, contract
generation, gate/proof runtime, Writer behavior, runtime `.punk` storage,
grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.

## Outcome

Done in `work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-verification-v0-1.md`.

Verification passed. The preflight model remains side-effect-free and evaluates
only explicit caller-provided target, parent, symlink ancestor, conflict,
manifest status, manifest authority, content, summary, absolute-path, and
claim-field inputs.

The model rejects absolute targets, path escape, runtime storage targets,
reported symlink ancestor escape, missing parents, different existing targets,
non-advisory status, non-`observed_structure` authority, manifest absolute
paths, content snippets, summaries, claim-like fields, and `claims_created`.
It allows missing targets and identical existing targets only when the rest of
the preflight is clean.

No source inventory implementation, repo scan, file walker, content reading,
hash computation from filesystem, AI summaries, claim extraction, manifest
writer implementation, contract generation, gate/proof runtime, Writer
behavior, Conformance Pack runtime, Migration Contract runtime, Regenerative
Spec behavior, or spec-as-source behavior was activated.
