---
id: report_2026_05_03_brownfield_source_corpus_manifest_writer_preflight_model_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md
---

# Brownfield Source Corpus Manifest Writer Preflight Model v0.1

## Summary

Added a side-effect-free Brownfield Source Corpus Manifest writer preflight
model.

The model represents whether a future manifest write would be allowed. It does
not write a manifest, scan a repository, walk files, read contents, compute
filesystem hashes, generate inventory, create claims, or activate runtime
storage.

## Files changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md`
- `work/goals/goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md`
- `work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md`
- `work/STATUS.md`

## Preflight model types added

Added inert model types for future source corpus manifest writer preflight:

- `SourceCorpusManifestWriterPreflight`
- `SourceCorpusManifestWriterTarget`
- `SourceCorpusManifestWriterPreflightInput`
- `SourceCorpusManifestWriterPreflightResult`
- `SourceCorpusManifestWriterPreflightFinding`
- `SourceCorpusManifestWriterConflictPolicy`
- `SourceCorpusManifestWriterTargetPolicy`
- explicit parent, symlink ancestor, manifest status, manifest authority, and
  manifest inspection evidence types

The model consumes explicit caller-provided inputs. It does not inspect the
filesystem.

## Target path policy

The default allowed target is:

```text
.punk/memory/reconstruction/source-corpus-manifest.md
```

The model can also represent a future configured repo-relative target under:

```text
.punk/memory/reconstruction/
```

Absolute targets, home/user-style targets, URL-like targets, parent traversal,
current-directory segments, empty segments, backslashes, and targets outside
the reconstruction directory are represented as blocking findings.

## Conflict policy

The model represents future conflict states without reading target files:

- missing target: allowed after clean preflight;
- identical existing target: idempotent allowed after clean preflight;
- different existing target: blocked;
- unknown conflict state: blocked.

No overwrite flag was added.

## Manifest authority checks

The model requires caller-provided manifest evidence to remain:

```text
status = advisory
authority = observed_structure
```

Non-advisory status or non-observed authority becomes a blocking finding. The
existing manifest model still has no accepted, canonical, or project-truth
authority variant.

## Claim-boundary checks

The model blocks manifest-like content with claim fields, including:

```text
intent
requirement
module_purpose
architecture_decision
accepted_behavior
invariant
contract
proof
claims_created
project_truth
```

`claims_created` receives an explicit blocker in addition to the general
claim-field blocker.

## Runtime storage boundary

The model rejects future targets under:

```text
.punk/runtime
.punk/events
.punk/runs
.punk/decisions
.punk/proofs
.punk/cache
.punk/indexes
```

No `.punk/runtime` storage, events, runs, decisions, proofs, cache, or indexes
were activated.

## Operation evidence boundary

The model records that future operation evidence is not proof, not a gate
decision, not acceptance, and not project truth.

## Tests/evals added

Added focused unit tests for:

- allowed reconstruction manifest target;
- absolute target rejection;
- path escape rejection;
- runtime storage target rejection;
- symlink ancestor escape rejection;
- missing parent rejection;
- different existing target blocking;
- missing target allowed;
- identical existing target idempotency;
- non-advisory manifest status rejection;
- non-observed manifest authority rejection;
- manifest absolute-path rejection;
- manifest content-snippet rejection;
- claim-field rejection;
- `claims_created` rejection;
- no manifest write capability;
- no repo scan/file walk/network/remote AI capability;
- no content read/filesystem hash/claim/intent capability;
- operation evidence is not proof, gate decision, acceptance, or project truth.

Added one smoke eval case:

```text
eval_source_corpus_manifest_writer_preflight_model_is_side_effect_free
```

## Non-scope preserved

No source inventory implementation, repo scan, file walker, directory traversal,
content reading, filesystem hash computation, AI summaries, claim extraction,
manifest writer implementation, contract generation, gate/proof runtime,
Writer behavior, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior was activated.

## Checks run

- `cargo fmt`
- `cargo check --workspace`
- `cargo test --workspace`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo fmt --check`
- `cargo build -p punk-cli`
- `~/.local/bin/punk-dev eval run smoke`
- `git diff --check`
- `scripts/check.sh docs-governance --files crates/punk-project/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md work/goals/goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md`
- `rg -n "writer active|manifest writer active|source inventory active|repo scan active|file walker active|content reading active|claims_created|project truth|canonical truth|infers intent|accepted behavior|contract generation|AI summaries|runtime storage active|active-core now" crates/punk-project/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md work/goals/goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md`

The grep matches were explicit blockers, tests, non-scope statements, or older
approved status text. No active source inventory, repo scan, file walker,
content reading, manifest writer implementation, AI summary, claim extraction,
runtime storage, project-truth, or `active-core now` drift was found in this
slice.

## Next selected goal

`work/goals/goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md`

This next goal is verification-only. It must not implement a writer, scanner,
file walker, content reader, filesystem hashing, manifest generation, runtime
storage, claim extraction, gate/proof runtime, Writer behavior, Conformance
Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or
spec-as-source behavior.

## Doc impact

```yaml
  classification: code-doc
  canonical_docs:
    - docs/product/CRATE-STATUS.md
  eval_specs: []
  work_artifacts:
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_writer_preflight_model_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-preflight-model-v0-1.md
    - work/STATUS.md
  reason: "Adds side-effect-free Brownfield Source Corpus Manifest writer preflight model behavior and smoke coverage while documenting that manifest writing, source inventory scanning, file walking, content reads, filesystem hash computation, runtime storage, and claim extraction remain inactive."
```
