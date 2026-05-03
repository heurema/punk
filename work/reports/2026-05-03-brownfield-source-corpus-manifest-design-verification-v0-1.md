---
id: report_2026_05_03_brownfield_source_corpus_manifest_design_verification_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md
---

# Brownfield Source Corpus Manifest Design Verification v0.1

## Summary

Verified the B2 Brownfield Source Corpus Manifest design after PR #21 landed in
`main`.

Verdict: pass.

The design remains schema/policy only. It does not implement source inventory,
repository scanning, file walking, content reading, manifest writing, AI
summaries, claim extraction, contracts, gate/proof runtime, Writer behavior, or
runtime `.punk` storage.

## Files inspected

- `docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md`
- `evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md`
- `docs/product/BROWNFIELD-INVENTORY.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `work/STATUS.md`
- `work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md`
- `work/reports/2026-05-03-brownfield-source-corpus-manifest-design-v0-1.md`
- `knowledge/research/2026-04-30-replayable-project-memory.md`

## Path policy verification

Pass.

The design requires future manifest output to use repo-relative paths only,
reject or redact absolute paths, reject or redact home/user paths, reject parent
traversal, reject backslash separators in portable manifest refs, avoid symlink
target expansion, fail closed on path escape, and keep `.punk/runtime`,
`.punk/cache`, and `.punk/indexes` out of source corpus items.

The eval spec covers:

- repo-relative paths only;
- no absolute host path leakage;
- no `/Users/...`, `/home/...`, drive-rooted, or symlink target path leakage.

## Content and hash policy verification

Pass.

The design preserves the default content policy:

```text
read_contents = false
store_snippets = false
summarize_contents = false
```

It forbids storing file contents, code snippets, environment values,
secret-like values, doc excerpts, and semantic summaries.

Hashes and sizes remain deferred for the first manifest implementation:

```text
hash_policy.status = deferred
hash_policy.value = null
size_policy.status = deferred
size_policy.value = null
```

The design permits optional future hash/size slots only as structural metadata
after a separate bounded goal.

## Source class and exclusion verification

Pass.

The design keeps the B1 source class vocabulary:

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

It requires `unknown` when classification is uncertain.

Default excluded paths are explicit:

```text
.git
.punk/runtime
.punk/cache
.punk/indexes
node_modules
target
```

Default caution categories are explicit:

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

Generated and vendored material stays candidate-marked. Candidate marking is
not authorship proof.

## Authority and privacy verification

Pass.

The future manifest authority model is:

```text
manifest_status = advisory
authority = observed_structure
```

The design states that the manifest is not project memory truth, a claim ledger,
a contract, a decision, proof, a semantic summary, an architecture map, intent
recovery, or contract readiness.

Privacy/no-network rules are explicit:

- local-only by default;
- no network access;
- no remote AI calls;
- no telemetry upload;
- no hidden sync;
- no remote indexing;
- no raw secrets;
- no raw environment values;
- no absolute host paths;
- no private transcript import.

## Inventory-vs-claim verification

Pass.

The design keeps manifest items separate from claims.

Manifest items must not include:

- purpose;
- requirement;
- intent;
- invariant;
- owner truth;
- architecture decision;
- accepted behavior;
- contract readiness conclusion;
- canonical doc status;
- proof status;
- gate decision;
- risk or severity.

Candidate claims belong to a later reviewed reconstruction layer. The manifest
may provide source refs for that layer, but it must not create claims.

The prior review finding about `claims_created` is resolved: the current eval
fixture does not contain that field.

## Verification questions

| Question | Result |
|---|---|
| Does the manifest design use repo-relative paths only? | Pass. |
| Does it forbid absolute paths and home/user paths? | Pass. |
| Does it avoid file contents by default? | Pass. |
| Does it avoid snippets and summaries? | Pass. |
| Are hashes/sizes deferred or optional, not required? | Pass. |
| Does it preserve `status = advisory` and `authority = observed_structure`? | Pass. |
| Does it keep manifest items separate from claims? | Pass. |
| Does it forbid intent, requirements, module purpose, accepted behavior, contracts, proof, and project truth? | Pass. |
| Does it define default excluded paths such as `.git`, `.punk/runtime`, caches, `node_modules`, `target`, build outputs? | Pass. |
| Does it preserve no-network / no-remote-AI / no-secret-output rules? | Pass. |
| Does eval spec cover false-authority risks? | Pass. |
| Does documentation avoid implying source inventory implementation is active? | Pass. |

## Anti-overclaim grep result

Ran:

```bash
rg -n "reconstructs|understands|infers intent|accepted behavior|project truth|canonical truth|contract generation|AI summaries|repo scan active|source inventory active|source corpus manifest active|claims_created|active-core now" \
  docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md \
  evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md \
  docs/product/BROWNFIELD-INVENTORY.md \
  docs/product/PROJECT-MEMORY.md \
  docs/product/ROADMAP.md \
  knowledge/research/2026-04-30-replayable-project-memory.md
```

Result:

- B2 docs/eval matches were prohibitions, non-goals, or inactive-boundary
  wording.
- `claims_created` was not found.
- `active-core now` was not found in the checked B2 docs/eval surfaces.
- Replayable Project Memory research matches are historical/advisory research
  text about accepted behavior and canonical-truth risks; they do not change the
  selected B2 path.

No overclaim requiring correction was found.

## Eval spec coverage

The eval spec covers future checks for:

- repo-relative paths;
- no absolute path leakage;
- no file contents;
- no claim fields;
- advisory observed-structure authority;
- runtime/cache/default excluded dirs;
- generated/vendored candidate marking;
- no network/no remote AI;
- no contract/proof claims;
- deferred hashes;
- deferred sizes;
- bounded source class vocabulary.

This is sufficient for B2 design verification.

## Drift observed

No implementation drift was observed.

This verification changed only work-ledger artifacts. It did not change
`crates/**`, `.punk/**`, `schemas/**`, CLI behavior, runtime storage, repo
traversal, manifest writing, file content reading, hash computation, or Writer
behavior.

One test anomaly was observed: the first `cargo test --workspace` run failed in
`punk-proof` while cleaning up a temporary first-active-write test storage root.
The focused test passed immediately afterward, and the full workspace test
passed on rerun. No code was changed for that anomaly because this verification
goal is docs/work-only and the failure did not reproduce.

## Checks run

- `rg -n "reconstructs|understands|infers intent|accepted behavior|project truth|canonical truth|contract generation|AI summaries|repo scan active|source inventory active|source corpus manifest active|claims_created|active-core now" docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md docs/product/BROWNFIELD-INVENTORY.md docs/product/PROJECT-MEMORY.md docs/product/ROADMAP.md knowledge/research/2026-04-30-replayable-project-memory.md || true` - PASS after inspection.
- `rg -n "claims_created|purpose:|requirement:|intent:|accepted_behavior|contract_readiness|proof_status|gate_decision|risk:|severity:" docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md || true` - PASS; no matches.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-design-verification-v0-1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-design-verification-v0-1.md` - PASS with 0 failures and 0 warnings.
- `cargo check --workspace` - PASS.
- `cargo test -p punk-proof proofpack_writer_first_active_write_slice_blocks_unsafe_or_unready_inputs -- --nocapture` - PASS after the initial full-workspace test anomaly.
- `cargo test --workspace` - PASS on rerun.
- `~/.local/bin/punk-dev eval run smoke` - PASS.

## Next selected goal

`work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md`

This next goal must be side-effect-free model only.

It must not implement source inventory scanning, file walking, content reading,
manifest generation, claim extraction, or runtime storage.

## Boundaries preserved

No source inventory implementation, repo scan, file walker, content reading, AI
summaries, claim extraction, contract generation, gate/proof runtime, Writer,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, or spec-as-source behavior was activated.

## Doc impact
```yaml
  classification: docs-only
  canonical_docs: []
  eval_specs: []
  work_artifacts:
    - work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_model_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-design-verification-v0-1.md
    - work/STATUS.md
  reason: "Records verification evidence for the B2 source corpus manifest design and selects a side-effect-free model-only next goal without changing product behavior."
```
