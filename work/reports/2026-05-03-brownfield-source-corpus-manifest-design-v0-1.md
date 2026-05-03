---
id: report_2026_05_03_brownfield_source_corpus_manifest_design_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_prepare_brownfield_source_corpus_manifest_design_v0_1.md
---

# Brownfield Source Corpus Manifest Design v0.1

## Summary

Prepared the B2 Brownfield Source Corpus Manifest design as docs/eval/spec
only.

The design defines a future manifest schema and policies before any repository
traversal, scanner, writer, content reading, hash computation, or manifest
generation is selected.

## Why manifest design comes before implementation

B1 established that brownfield inventory may record only observable structure.
The Source Corpus Manifest is the first concrete artifact shape that could
accidentally turn inventory into claims, project truth, contracts, or source
analysis.

Design comes first so future implementation has explicit limits for paths,
contents, hashes, source classes, exclusions, authority, privacy, and claim
boundaries.

## Manifest schema

The future manifest top-level shape includes:

```text
manifest_id
schema_version
project_id
entry_mode
manifest_status
authority
created_at
generated_at_policy
source_root_ref
inventory_scope
excluded_paths
items[]
```

The selected authority is:

```text
manifest_status = advisory
authority = observed_structure
```

## Item schema

Each item represents one observable structural fact about one repo-relative
path.

The future item shape includes:

```text
item_id
repo_relative_path
observed_kind
source_class
source_markers
tracking_status
sensitivity
generated_or_vendored_candidate
size_policy
hash_policy
content_policy
evidence_ref
notes
```

Allowed `observed_kind` values:

```text
file
directory
symlink_candidate
unknown
```

Allowed `tracking_status` values:

```text
observed
excluded
sensitive_redacted
unknown
```

Allowed `sensitivity` values:

```text
normal
caution
sensitive
excluded
unknown
```

## Path, content, and hash policies

Path policy:

- repo-relative paths only;
- no absolute host paths;
- no home/user paths;
- no parent traversal;
- no backslash path separators in portable manifest refs;
- no symlink target expansion in the manifest;
- fail closed on path escape.

Content policy:

- do not read file contents by default;
- do not store snippets;
- do not summarize contents;
- do not store raw environment values or secret-like values.

Hash policy:

- hashes are deferred for the first manifest implementation;
- optional future hash fields may exist only as deferred/null structural slots;
- hashes must not imply accepted behavior, contracts, proof, freshness, or
  project truth.

Size policy:

- file sizes are deferred for the first manifest implementation;
- optional future size fields may exist only as deferred/null structural slots.

## Source class and exclusion policies

Allowed B1 source classes:

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

Default excluded paths:

```text
.git
.punk/runtime
.punk/cache
.punk/indexes
node_modules
target
```

Default caution categories:

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

Generated and vendored material must stay candidate-marked and must not be
treated as project-authored truth.

## Authority and privacy policies

Future manifest output must stay advisory observed structure only.

It must not claim canonical, accepted, contract, decision, proof, or project
truth authority.

Privacy rules:

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

## Claim boundary

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

Candidate claims belong to a later reviewed reconstruction layer.

## Eval spec created

Created:

- `evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md`

The spec defines future eval expectations for repo-relative paths, no absolute
path leakage, no file contents, no claim fields, advisory observed-structure
authority, default runtime/cache exclusions, generated/vendored candidate
marking, no network/no AI, no contract/proof claims, deferred hashes, deferred
sizes, and bounded source classes.

## Drift observed

No implementation drift was observed.

This task changed docs/eval/work artifacts only. It did not change `crates/**`,
`.punk/**`, `schemas/**`, CLI behavior, runtime storage, repo traversal,
manifest writing, file content reading, hash computation, or Writer behavior.

## Checks run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md docs/product/BROWNFIELD-INVENTORY.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md work/STATUS.md work/goals/goal_prepare_brownfield_source_corpus_manifest_design_v0_1.md work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-design-v0-1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-design-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next selected goal

`work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md`

Reason: this is still a trust boundary. It should be verified before any
manifest model or implementation is selected.

## Boundaries preserved

No source inventory implementation, repo scan, file walker, language detector,
manifest writer, hash computation, file content reading, source corpus
manifest generation, claim extraction, claim ledger population, AI summaries,
LLM calls, module map, architecture recovery, intent recovery, contract
generation, Contract Context Pack generation, Conformance Pack runtime,
Migration Contract runtime, Regenerative Spec behavior, gate/proof runtime,
Writer behavior, runtime `.punk` storage, grayfield reconciliation, or
spec-as-source behavior was activated.

## Doc impact
```yaml
  classification: docs-only
  canonical_docs:
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
  eval_specs:
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
  work_artifacts:
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_design_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_design_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-design-v0-1.md
    - work/STATUS.md
  reason: "Adds B2 source corpus manifest schema and policy design before any inventory implementation, scanner, writer, content read, or runtime behavior."
```
