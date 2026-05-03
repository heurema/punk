---
id: report_2026_05_03_brownfield_source_corpus_manifest_writer_boundary_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md
---

# Brownfield Source Corpus Manifest Writer Boundary v0.1

## Summary

Defined the future source corpus manifest writer boundary as docs/eval/work
policy only.

This is not a writer implementation.

## Why writer boundary comes before writer implementation

A future source corpus manifest writer would be the first Brownfield operation
that persists the manifest artifact. That creates target-path, preflight,
partial-write, conflict, privacy, and false-authority risks.

The boundary now defines the rules before any code can write
`.punk/memory/reconstruction/source-corpus-manifest.md`.

## Target path policy

Future writer target is limited to:

```text
.punk/memory/reconstruction/source-corpus-manifest.md
```

or an explicitly configured repo-relative target under:

```text
.punk/memory/reconstruction/
```

The future writer must not write outside the repository, use absolute targets,
follow symlinks out of the repository, or infer hidden output paths.

## Preflight policy

Future writer preflight must validate:

- target path is repo-relative and under `.punk/memory/reconstruction/`;
- parent directory is valid;
- no path escape or parent traversal is present;
- no symlink ancestor can redirect the write outside the repository;
- target conflict policy is known before bytes are written;
- manifest status is `advisory`;
- manifest authority is `observed_structure`;
- manifest data contains no claim fields;
- manifest data contains no file contents, snippets, summaries, raw secrets, or
  raw environment values;
- manifest data contains no absolute host paths.

Any failed preflight must fail closed before target modification.

## Atomicity policy

Future writer behavior must avoid partial files:

- write complete bytes to a temporary file in the same target directory;
- flush or fsync only if a later implementation goal selects durability
  semantics;
- atomically rename into place when platform-compatible;
- fail closed without modifying the target when atomic replacement cannot be
  guaranteed.

No atomic write implementation was added in this goal.

## Conflict policy

The future conflict policy is:

- missing target may be written after clean preflight;
- identical content may be idempotent;
- different existing content must block unless a later bounded goal adds an
  explicit overwrite flag and review boundary.

No overwrite flag is active.

## Content/privacy policy

The future writer may write only manifest data allowed by the existing manifest
design and side-effect-free model.

It must not read file contents, store snippets, store summaries, store secrets,
store raw environment values, store absolute paths, use network access, call
remote AI, or import private agent transcripts.

## Claim boundary

The future writer must reject manifest data containing claim-like fields such
as:

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

The manifest writer may persist observed structure only. It must not create a
claim ledger, decide contract readiness, accept behavior, or promote source
corpus observations into project truth.

## Operation evidence boundary

Future writer operation evidence may describe write attempt and outcome only.

It is not proof, not a gate decision, not acceptance, not a contract, and not
project truth.

## Runtime storage boundary

A future source corpus manifest writer must not activate or write:

```text
.punk/runtime
.punk/events
.punk/runs
.punk/decisions
.punk/proofs
```

Persisting a tracked manifest under `.punk/memory/reconstruction/` must not be
treated as runtime storage activation.

## Eval spec updates

Added future eval-spec coverage for:

- `writer_target_is_reconstruction_manifest_path`;
- `writer_rejects_path_escape`;
- `writer_rejects_absolute_target`;
- `writer_rejects_symlink_escape`;
- `writer_preflight_before_write`;
- `writer_no_partial_write_on_conflict`;
- `writer_blocks_claim_fields`;
- `writer_blocks_absolute_paths`;
- `writer_blocks_content_snippets`;
- `writer_does_not_activate_runtime_storage`;
- `writer_operation_evidence_is_not_proof`;
- `writer_does_not_create_gate_decision`;
- `writer_does_not_create_claim_ledger`.

These are eval/spec cases only. No writer tests or runtime behavior were added.

## Drift observed

No drift requiring correction was observed while preparing the boundary.

The term is recorded as `source corpus manifest writer boundary` to avoid
confusing this future file writer with Punk `Writer` behavior.

## Checks run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo check --workspace`
- `cargo test --workspace`
- `~/.local/bin/punk-dev eval run smoke`
- `git diff --check`
- `scripts/check.sh docs-governance --files work/goals/goal_verify_brownfield_source_corpus_manifest_writer_boundary_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-boundary-v0-1.md docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md work/STATUS.md work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-boundary-v0-1.md`

## Next selected goal

`work/goals/goal_verify_brownfield_source_corpus_manifest_writer_boundary_v0_1.md`

This next goal must be verification-only.

## Boundaries preserved

No source inventory implementation, repo scan, file walker, content reading,
hash computation from filesystem, AI summaries, claim extraction, manifest
writer implementation, contract generation, gate/proof runtime, Writer,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior was activated.

## Doc impact
```yaml
  classification: docs-only
  canonical_docs:
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
  eval_specs:
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
  work_artifacts:
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_writer_boundary_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-boundary-v0-1.md
    - work/STATUS.md
  reason: "Defines the future source corpus manifest writer boundary before any manifest writer implementation, repo scan, file walk, content read, hash computation, claims, runtime storage, gate/proof runtime, or Writer behavior is selected."
```
