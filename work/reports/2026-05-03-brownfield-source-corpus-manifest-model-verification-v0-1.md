---
id: report_2026_05_03_brownfield_source_corpus_manifest_model_verification_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md
---

# Brownfield Source Corpus Manifest Model Verification v0.1

## Summary

Verified the side-effect-free Brownfield Source Corpus Manifest model after PR
#23 landed in `main`.

Verdict: pass.

The model may represent a future manifest. It does not collect one.

## Files inspected

- `crates/punk-project/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md`
- `evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md`
- `work/reports/2026-05-03-brownfield-source-corpus-manifest-model-v0-1.md`

## Model boundary verification

Pass.

`SourceCorpusManifest` defaults to:

```text
status = advisory
authority = observed_structure
entry_mode = brownfield
```

`SourceCorpusManifestModelCapabilities` reports no repository scanning, file
walking, file content reads, filesystem hash computation, manifest writing,
claim creation, intent inference, network use, or remote AI use.

## Path policy verification

Pass.

`SourceCorpusRepoRelativePath` accepts repo-relative paths and rejects absolute
paths, home/user paths, Windows drive-rooted paths, URL-like paths, parent
traversal, dot segments, empty segments, and backslashes.

Manifest item evidence refs use:

```text
<manifest_id>#<item_id>
```

so item evidence remains manifest-instance scoped.

## Content/hash/size policy verification

Pass.

`SourceCorpusContentPolicy` defaults to no contents, no snippets, and no
summaries.

`SourceCorpusHashPolicy` and `SourceCorpusSizePolicy` default to `deferred` and
do not require filesystem hashing or filesystem metadata reads.

## Source class and sensitivity verification

Pass.

Source classes match the B1/B2 vocabulary:

```text
source_code
docs
tests
ci_config
package_manifest
schema
migration
script
generated_candidate
vendored_candidate
unknown
```

Caution classes match the canonical caution vocabulary:

```text
secrets
env_files
local_cache
build_output
private_agent_transcripts
generated_candidate
vendored_candidate
unknown
```

Path-like excludes remain default excluded paths, not caution classes:

```text
.git
.punk/runtime
.punk/cache
.punk/indexes
node_modules
target
```

Generated/vendored markers remain candidates, not truth.

## Claim-boundary verification

Pass.

Manifest items do not contain claim authority. The forbidden claim-field guard
rejects:

```text
intent
requirement
module_purpose
architecture_decision
accepted_behavior
invariant
contract_ref_as_claim
proof_ref_as_truth
claims_created
contract_readiness
gate_decision
proof_status
risk
severity
```

## No-scan/no-writer verification

Pass.

The model API has no filesystem traversal, no manifest writer, no content read,
no filesystem hash computation, no network, and no remote AI path.

## Eval coverage summary

`punk-project` unit tests cover advisory status, observed-structure authority,
repo-relative path acceptance, absolute/home path rejection, no-content
defaults, deferred hash/size policy, manifest-scoped evidence refs, source and
caution class vocabulary, forbidden claim fields, and no-scan/no-read/no-write
capabilities.

`punk-eval` smoke includes:

```text
eval_source_corpus_manifest_model_is_side_effect_free
```

## Anti-overclaim grep result

Command:

```bash
rg -n "claims_created|project truth|canonical truth|infers intent|accepted behavior|contract generation|AI summaries|repo scan active|source inventory active|manifest writer active|active-core now" \
  crates/punk-project/src/lib.rs \
  crates/punk-eval/src/lib.rs \
  docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md \
  evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md \
  docs/product/CRATE-STATUS.md
```

Result: pass after inspection.

Matches were either explicit prohibitions, non-active boundary text, or tests
for forbidden fields such as `claims_created`. No active source inventory, repo
scan, manifest writer, AI summary, claim extraction, contract generation, or
`active-core now` drift was found.

## Drift observed

No drift requiring correction was observed.

This verification changed only work-ledger artifacts. It did not change
`crates/**`, `.punk/**`, `schemas/**`, CLI behavior, runtime storage, repo
traversal, manifest writing, file content reading, hash computation, AI
behavior, claims, contracts, gate/proof runtime, or Writer behavior.

## Checks run

- `rg -n "claims_created|project truth|canonical truth|infers intent|accepted behavior|contract generation|AI summaries|repo scan active|source inventory active|manifest writer active|active-core now" crates/punk-project/src/lib.rs crates/punk-eval/src/lib.rs docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md docs/product/CRATE-STATUS.md`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo fmt --check`
- `git diff --check`
- `cargo check --workspace`
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-model-verification-v0-1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-model-verification-v0-1.md`
- `cargo test --workspace`
- `~/.local/bin/punk-dev eval run smoke`

## Next selected goal

`work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md`

This next goal must be boundary/design only. It must not implement source
inventory scanning, file walking, content reading, hash computation, manifest
generation/writing, claim extraction, or runtime storage.

## Boundaries preserved

No source inventory implementation, repo scan, file walker, content reading,
hash computation from filesystem, AI summaries, claim extraction, contract
generation, gate/proof runtime, Writer, Conformance Pack runtime, Migration
Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was
activated.

## Doc impact
```yaml
  classification: docs-only
  canonical_docs: []
  eval_specs: []
  work_artifacts:
    - work/goals/goal_verify_brownfield_source_corpus_manifest_model_v0_1.md
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_boundary_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-model-verification-v0-1.md
    - work/STATUS.md
  reason: "Records verification evidence for the side-effect-free Brownfield Source Corpus Manifest model and selects a writer boundary/design-only next goal without changing product behavior."
```
