---
id: report_brownfield_source_corpus_manifest_writer_first_slice_v0_1
kind: implementation-report
status: completed
authority: advisory
owner: vitaly
created_at: 2026-05-04
updated_at: 2026-05-04
goal_ref: work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
selected_next: work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
---

# Brownfield Source Corpus Manifest Writer First Slice v0.1

## 1. Summary

Implemented the first narrow Source Corpus Manifest writer slice.

The slice writes only an already-constructed `SourceCorpusManifest` model to one
explicit safe target after matching preflight passes.

## 2. Files changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md`
- `evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md`
- `work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md`
- `work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md`
- `work/STATUS.md`
- `work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-v0-1.md`

## 3. Writer behavior added

`punk-project` now includes:

- deterministic canonical rendering from `SourceCorpusManifest`;
- first-slice writer result and operation-evidence status vocabulary;
- first-slice fail-closed blockers;
- first-slice capabilities recording the active and inactive behavior boundary;
- a writer function that requires a matching successful preflight result;
- one safe target write under `.punk/memory/reconstruction/`;
- same-directory temporary file plus no-overwrite target creation;
- idempotent handling for preflight-identical existing targets;
- conflict handling for different existing target content;
- in-memory operation evidence only.

No CLI command was added.

## 4. Target/preflight policy

Allowed target remains:

```text
.punk/memory/reconstruction/source-corpus-manifest.md
```

or a repo-relative target under:

```text
.punk/memory/reconstruction/
```

The writer requires a preflight result for the same manifest and target. A
blocking preflight finding prevents file IO.

Blocked target cases include absolute paths, path escape, runtime storage
targets, reported symlink ancestor escape, missing target parent, and actual
intermediate parent symlink.

## 5. Render/canonical bytes policy

Rendering is deterministic and uses stable field order.

The first slice has no modeled timestamp input, so rendered bytes do not include
`created_at` or `generated_at`. They include:

```text
generated_at_policy: no_runtime_clock
```

The rendered bytes include no absolute host paths, home paths, env values, raw
file contents, summaries, or claim fields.

## 6. Atomicity/conflict behavior

Missing target:

```text
write complete bytes to same-directory temp file
flush temp file
create target with no-overwrite link
remove temp file after successful target creation
```

Identical existing target:

```text
idempotent
no write attempted
existing target bytes rechecked against canonical render
```

Different existing target:

```text
conflict
no overwrite
```

Failure before target creation leaves no target. The writer does not create
parent directories.

## 7. Operation evidence boundary

Operation evidence status may be:

```text
attempted
blocked
written
idempotent
conflict
error
```

Operation evidence remains in memory only. It is not proof, not a gate
decision, not acceptance, not project truth, not a claim ledger, and not
contract readiness.

## 8. Non-scope preserved

Preserved inactive:

- source inventory implementation;
- repo scan;
- file walker;
- directory traversal;
- source file content reading;
- source filesystem hash computation;
- manifest generation from repository state;
- claim extraction;
- claim ledger population;
- AI summaries;
- LLM calls;
- module map;
- architecture recovery;
- intent recovery;
- contract generation;
- gate/proof runtime;
- Punk `Writer` behavior;
- runtime `.punk` storage;
- CLI behavior;
- grayfield reconciliation;
- Conformance Pack runtime;
- Migration Contract runtime;
- Regenerative Spec behavior;
- spec-as-source behavior.

## 9. Tests/evals added

Added `punk-project` unit coverage for:

- `writer_writes_canonical_manifest_to_safe_target`
- `writer_requires_preflight_pass`
- `writer_blocks_on_failed_preflight`
- `writer_rejects_absolute_target`
- `writer_rejects_path_escape`
- `writer_rejects_runtime_storage_target`
- `writer_rejects_symlink_escape_when_reported`
- `writer_blocks_actual_intermediate_parent_symlink`
- `writer_blocks_different_existing_content`
- `writer_is_idempotent_on_identical_content`
- `writer_rechecks_identical_target_bytes_before_idempotent`
- `writer_no_partial_target_on_failure`
- `writer_does_not_scan_repo`
- `writer_does_not_read_file_contents`
- `writer_does_not_compute_file_hashes`
- `writer_does_not_create_claims`
- `writer_does_not_promote_manifest_authority`
- `writer_operation_evidence_is_not_proof`
- `writer_operation_evidence_is_not_gate_decision`
- `writer_operation_evidence_is_not_acceptance`

Added smoke eval:

```text
eval_source_corpus_manifest_writer_first_slice_writes_prepared_manifest
```

Updated eval spec with active first-slice coverage cases `BSCM-041` through
`BSCM-057`.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The first narrow Source Corpus Manifest writer slice changed code behavior and required crate-status, product boundary, eval-spec, work-ledger, goal, and report updates."
  touched_surfaces:
    - crates/punk-project/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CRATE-STATUS.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
    - work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-v0-1.md
  required_updates:
    - docs/product/CRATE-STATUS.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - python3 scripts/check_research_gate.py
    - python3 scripts/check_work_ledger.py
    - cargo fmt --check
    - cargo check --workspace
    - cargo test --workspace
    - cargo run -q -p punk-cli -- eval run smoke
    - ~/.local/bin/punk-dev eval run smoke
```

## 10. Checks run

Checks passed:

```bash
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
cargo fmt --check
cargo test -p punk-project
cargo test -p punk-eval
cargo check --workspace
cargo test --workspace
cargo run -q -p punk-cli -- eval run smoke
~/.local/bin/punk-dev eval run smoke
git diff --check
scripts/check.sh docs-governance --files $(git diff --name-only) $(git ls-files --others --exclude-standard) --report work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-v0-1.md
```

## 11. Next selected goal

Selected next:

```text
work/goals/goal_verify_brownfield_source_corpus_manifest_writer_first_slice_v0_1.md
```
