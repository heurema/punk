---
id: report_2026_05_03_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md
---

# Brownfield Source Corpus Manifest Writer Implementation Boundary v0.1

## Summary

Defined the Brownfield Source Corpus Manifest writer implementation boundary as
docs/eval/spec only.

Verdict: boundary defined, no implementation selected.

The future minimal writer slice may only render canonical bytes from an
already-constructed `SourceCorpusManifest` model, require successful preflight,
write one safe reconstruction target, and emit non-authoritative operation
evidence.

## Why implementation boundary is needed before implementation

The preflight model can already decide whether a future write is safe from
explicit caller-provided evidence. Before selecting real write code, the repo
needs one more guardrail: what counts as the smallest permitted writer
implementation.

Without this boundary, a writer slice could drift into building the manifest
itself, scanning the repository, reading contents, hashing files, extracting
claims, or treating operation evidence as truth.

## Future writer input boundary

Future writer input must be:

- an already-constructed `SourceCorpusManifest` model;
- an explicit target path or the default safe target;
- an explicit preflight result for the same model and target.

Future writer input must not be:

- repository root to scan;
- directory list to walk;
- raw source files to inspect;
- AI prompt;
- claim ledger input.

## Future target path boundary

Allowed default target:

```text
.punk/memory/reconstruction/source-corpus-manifest.md
```

Allowed configured target:

```text
.punk/memory/reconstruction/<repo-relative-target>
```

Forbidden targets include absolute targets, path escape, symlink escape,
hidden inferred targets, and runtime or derived state surfaces:

```text
.punk/runtime
.punk/events
.punk/runs
.punk/decisions
.punk/proofs
.punk/cache
.punk/indexes
```

## Future preflight boundary

Writer requires successful preflight before writing bytes.

Blocking findings include:

- absolute target;
- path escape;
- symlink escape;
- different existing target;
- unknown target state;
- missing or unknown parent;
- non-advisory manifest;
- non-`observed_structure` authority;
- claim-like fields;
- absolute paths in manifest;
- content snippets;
- summaries;
- runtime storage target.

No write is allowed while any blocking finding is present.

## Future render boundary

Future rendering must produce deterministic canonical bytes from the supplied
model.

Rendering constraints:

- stable field order;
- no hidden timestamps;
- no runtime clock reads unless a later bounded goal allows clock injection;
- no host paths;
- no environment values;
- no local usernames;
- no raw file contents.

If timestamps are needed, they must come from the manifest model input unless a
later goal explicitly changes the clock boundary.

## Future atomic write boundary

Future write rules:

- write a temporary file in the same target directory;
- keep flush/fsync deferred unless an explicit durability policy is selected;
- use atomic rename where platform-compatible;
- leave no partial target on failure;
- block different existing target content unless a later overwrite boundary is
  selected;
- allow identical existing content as idempotent.

## Future operation evidence boundary

Future operation evidence may record only:

```text
attempted
blocked
written
idempotent
conflict
error
```

It is not proof, gate decision, acceptance, project truth, claim ledger, or
contract readiness.

## Future authority/privacy/runtime boundaries

A written manifest remains:

```text
manifest_status = advisory
authority = observed_structure
```

Writing it does not promote it, create claims, decide contract readiness, or
create contract inputs.

Future writer output must not include absolute paths, home/user paths, raw
environment values, secrets, file contents, private agent transcripts, or AI
summaries.

Future writer behavior must not activate `.punk/runtime`, `.punk/events`,
`.punk/runs`, `.punk/decisions`, `.punk/proofs`, gate/proof runtime, Punk
`Writer` behavior, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.

## Eval spec updates

Added future implementation-boundary eval cases:

- `writer_accepts_only_preflight_pass`
- `writer_rejects_blocking_preflight`
- `writer_input_is_manifest_model_not_repo_scan`
- `writer_render_is_deterministic`
- `writer_render_has_no_host_paths`
- `writer_render_has_no_hidden_runtime_clock`
- `writer_writes_only_safe_target`
- `writer_blocks_different_existing_target`
- `writer_idempotent_on_identical_target`
- `writer_no_partial_target_on_failure`
- `writer_operation_evidence_is_not_proof`
- `writer_operation_evidence_is_not_acceptance`
- `writer_does_not_promote_manifest_authority`
- `writer_does_not_create_claims`
- `writer_does_not_activate_runtime_storage`

These are eval spec requirements only. No writer tests were implemented.

## Drift observed

No drift requiring correction was observed.

The existing docs/eval/code state already kept the writer boundary and
preflight model inert. The missing guardrail was an explicit implementation
boundary for the future write slice.

## Checks run

- `gh pr view 28 --json number,title,state,mergeable,reviewDecision,statusCheckRollup,comments,reviews,headRefName,baseRefName,url`
- `gh api graphql -F owner='heurema' -F name='punk' -F number=28 -f query='query($owner:String!, $name:String!, $number:Int!) { repository(owner:$owner, name:$name) { pullRequest(number:$number) { reviewThreads(first:100) { nodes { isResolved isOutdated path line comments(first:10) { nodes { author { login } body url } } } } } } }'`
- `gh pr diff 28 --name-only`
- `gh pr merge 28 --rebase --delete-branch`
- `git checkout main`
- `git pull --ff-only origin main`
- `git status --short`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo check --workspace`
- `cargo test --workspace`
- `~/.local/bin/punk-dev eval run smoke`
- `git diff --check`
- `scripts/check.sh docs-governance --files docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md work/STATUS.md work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md work/goals/goal_verify_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md --report work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md`

## Next selected goal

`work/goals/goal_verify_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md`

The next goal is verification-only. It must not select writer implementation.

## Boundaries preserved

No manifest writer implementation, source inventory implementation, repo scan,
file walker, content reading, hash computation from filesystem, AI summaries,
claim extraction, contract generation, gate/proof runtime, Writer, Conformance
Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or
spec-as-source behavior was activated.

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
    - work/goals/goal_prepare_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md
    - work/goals/goal_verify_brownfield_source_corpus_manifest_writer_implementation_boundary_v0_1.md
    - work/reports/2026-05-03-brownfield-source-corpus-manifest-writer-implementation-boundary-v0-1.md
    - work/STATUS.md
  reason: "Defines the future Source Corpus Manifest writer implementation boundary without activating manifest writing, repo scanning, file walking, content reads, filesystem hash computation, runtime storage, claims, gate/proof runtime, Writer behavior, or spec-as-source behavior."
```
