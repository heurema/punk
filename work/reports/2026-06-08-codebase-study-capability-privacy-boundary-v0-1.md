---
id: report_2026_06_08_codebase_study_capability_privacy_boundary_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
selected_next: work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md
related_docs:
  - docs/modules/codebase-study.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
related_evals:
  - evals/specs/codebase-study-module-boundary.v0.1.md
  - evals/specs/codebase-study-conformance-packet.v0.1.md
  - evals/specs/codebase-study-source-observation-request-packet.v0.1.md
  - evals/specs/codebase-study-capability-privacy-boundary.v0.1.md
doc_impact:
  classification: docs-only
  refs:
    - docs/modules/codebase-study.md
    - evals/specs/codebase-study-capability-privacy-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
    - work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md
research_gate:
  classification: R1
  required: true
---

# Codebase Study capability and privacy boundary v0.1

## Summary

Defined the Codebase Study capability/privacy boundary for future source
observation.

The boundary keeps requests separate from grants:

```yaml
codebase_study_capability_privacy_boundary:
  boundary_status: proposed
  authority: non_authoritative
  capability_requests:
    - scoped_path_observation_over_explicit_refs
  capability_grants: []
  selected_capability_grants: []
  non_authority: true
```

No source-observation capability is selected in this slice.

## What changed

- Added the capability/privacy boundary section to
  `docs/modules/codebase-study.md`.
- Added `evals/specs/codebase-study-capability-privacy-boundary.v0.1.md`.
- Closed the selected capability/privacy goal.
- Selected
  `work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md`
  as the next bounded checkpoint.

## Denied by default

The boundary keeps these capabilities denied:

- ambient repository discovery;
- implicit current-working-directory traversal;
- repo scanning;
- file walking;
- source content reading;
- source snippets or document excerpts;
- raw environment values;
- secret-like values;
- source filesystem hashing;
- size collection;
- manifest generation from repository state;
- source indexing;
- claim extraction;
- architecture recovery;
- intent recovery;
- AI summaries;
- runtime `.punk` storage;
- event-log mutation;
- CLI behavior;
- module execution;
- lab code import;
- benchmark execution;
- network access;
- credential access;
- adapter invocation;
- gate decisions;
- proof writing;
- acceptance claims.

## Future grant prerequisites

Future capabilities may be requested, but not granted, for structural advisory
observation over caller-supplied refs:

- path existence candidate;
- path kind candidate;
- basename or extension marker;
- source-class candidate;
- generated or vendored candidate;
- warning;
- blocker;
- limitation.

Any future grant requires a later bounded goal with:

- explicit source refs;
- selected capability-grant refs;
- privacy and redaction refs or resolved blockers;
- expected receipt/evidence refs;
- evaluation-route refs;
- conformance review;
- deterministic eval coverage before implementation.

## Privacy and redaction blockers

Privacy and redaction blockers fail closed.

Missing privacy policy refs, missing redaction policy refs, source snippets,
document excerpts, raw environment values, secret-like values, credentials,
private transcripts, and sensitive content requests must produce blockers or
refusal. They must not be silently normalized into path observation or widened
into content reads.

## Boundary confirmation

- No Rust code was changed.
- No request parser was implemented.
- No capability resolver, privacy engine, or redaction engine was implemented.
- No Codebase Study skeleton was created.
- No Module Host runtime was activated.
- No module loading, module invocation, manifest parser, packet parser,
  capability enforcement, provider orchestration, workspace initializer,
  instruction generator, adapter, source observer, scanner, file walker,
  content reader, hash collector, size collector, source inventory generator,
  Source Corpus Manifest generator, runtime `.punk` writer, lab runner,
  benchmark runner, receipt writer, gate writer, proof writer, or acceptance
  claim was added.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Defines the Codebase Study capability/privacy boundary before any implementation or active source observation behavior."
  touched_surfaces:
    - docs/modules/codebase-study.md
    - evals/specs/codebase-study-capability-privacy-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
    - work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md
  required_updates:
    - docs/modules/codebase-study.md
    - evals/specs/codebase-study-capability-privacy-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
    - work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md
```

## Validation

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files docs/modules/codebase-study.md evals/specs/codebase-study-capability-privacy-boundary.v0.1.md work/STATUS.md work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md work/goals/goal_define_codebase_study_observation_receipt_evidence_boundary_v0_1.md work/reports/2026-06-08-codebase-study-capability-privacy-boundary-v0-1.md` passed with zero warnings.
- `python3 scripts/check_docs_governance.py --repo . --staged` passed with zero warnings.
- `git diff --check` passed.
- `git diff --cached --check` passed.

## Not tested

No Rust tests are required for this docs/eval-only slice. No Rust code,
runtime behavior, CLI behavior, request parser, scanner, file walker, source
content reader, hash collector, size collector, manifest generator from
repository state, `.punk` state, module execution, lab runner, or benchmark
suite was changed.

## Recommended next work

Define Codebase Study observation receipt/evidence boundary v0.1:

- define advisory receipt/evidence shape;
- keep receipt/evidence refs separate from source observation execution,
  runtime storage, gate/proof authority, benchmark authority, and acceptance;
- keep source observation implementation, repo scanning, file walking, content
  reading, source hashing, size collection, runtime storage, lab execution,
  benchmark execution, gate/proof behavior, and acceptance denied.
