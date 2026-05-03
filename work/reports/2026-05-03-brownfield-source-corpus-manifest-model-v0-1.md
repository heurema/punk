---
id: report_2026_05_03_brownfield_source_corpus_manifest_model_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md
---

# Brownfield Source Corpus Manifest Model v0.1

## Summary

Added a side-effect-free Rust model for the future Brownfield Source Corpus
Manifest. The model represents advisory observed structure only. It does not
scan repositories, walk files, read file contents, compute filesystem hashes,
write manifests, create claims, infer intent, use network, or use remote AI.

## Files changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md`
- `work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md`

## Model types added

`punk-project` now exposes inert model types for the manifest boundary:

- `SourceCorpusManifest`
- `SourceCorpusItem`
- `SourceCorpusManifestId`
- `SourceCorpusItemId`
- `SourceCorpusRepoRelativePath`
- `SourceCorpusObservedKind`
- `SourceCorpusSourceClass`
- `SourceCorpusSourceMarker`
- `SourceCorpusSensitivityClass`
- `SourceCorpusGeneratedOrVendoredCandidate`
- `SourceCorpusManifestStatus`
- `SourceCorpusManifestAuthority`
- `SourceCorpusContentPolicy`
- `SourceCorpusHashPolicy`
- `SourceCorpusSizePolicy`
- `SourceCorpusExclusionPolicy`
- `SourceCorpusManifestModelCapabilities`

## Invariants represented/enforced

- Default manifest status is `Advisory`.
- Default authority is `ObservedStructure`.
- Manifest items have no project-truth or claim authority.
- Manifest item evidence refs use `<manifest_id>#<item_id>` so item evidence
  remains manifest-instance scoped.
- Model capabilities default to no scanning, walking, reading, filesystem
  hashing, manifest writing, claim creation, intent inference, network, or
  remote AI.
- `claims_created` and other claim-shaped fields are forbidden by the manifest
  claim-field guard.
- Default caution classes match the canonical caution vocabulary. Path-like
  excludes such as `.git`, `.punk/runtime`, `node_modules`, and `target` remain
  default excluded paths, not caution classes.

## Path/content/hash policy

- Manifest item paths must be repo-relative.
- Absolute paths, home paths, Windows drive paths, URL-like paths, parent
  traversal, dot segments, empty segments, and backslashes are rejected.
- Content policy defaults to contents not read, snippets not stored, and
  summaries not generated.
- Hash and size policies default to deferred.
- The model does not compute file hashes or read filesystem metadata.

## Claim boundary

The model does not contain fields for intent, requirements, module purpose,
architecture decisions, accepted behavior, invariants, contracts as claims,
proof as truth, or `claims_created`.

Evidence refs remain source/observation refs only, not claim acceptance.

## Tests/evals added

`punk-project` tests cover:

- default advisory status and observed-structure authority;
- absolute/home path rejection and repo-relative path acceptance;
- no-content defaults;
- deferred hash/size defaults;
- forbidden claim fields including `claims_created`;
- manifest-scoped evidence refs;
- source and caution class coverage;
- no scan/read/write capabilities.

`punk-eval` adds:

- `eval_source_corpus_manifest_model_is_side_effect_free`
- smoke report summary and human-output coverage for the new boundary note.

## Non-scope preserved

No source inventory implementation, repo scan, file walker, content reading,
filesystem hash computation, manifest writer, AI summaries, claim extraction,
contract generation, gate/proof runtime, Writer behavior, Conformance Pack
runtime, Migration Contract runtime, Regenerative Spec behavior, or
spec-as-source behavior was activated.

## Checks run

- `cargo test -p punk-project`
- `cargo test -p punk-eval`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `cargo build -p punk-cli`
- `~/.local/bin/punk-dev eval run smoke`
- `git diff --check`
- `scripts/check.sh docs-governance --files crates/punk-eval/src/lib.rs crates/punk-project/src/lib.rs docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md`

## Next selected goal

`work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md`

## Doc impact
```yaml
  classification: code-doc
  canonical_docs:
    - docs/product/CRATE-STATUS.md
  eval_specs: []
  work_artifacts:
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md
    - work/STATUS.md
  reason: "Adds side-effect-free Brownfield Source Corpus Manifest model behavior and smoke coverage while documenting that source inventory scanning, manifest writing, content reads, and claim extraction remain inactive."
```
