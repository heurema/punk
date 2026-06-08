---
id: report_2026_06_08_codebase_study_source_observation_request_packet_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_define_codebase_study_source_observation_request_packet_v0_1.md
selected_next: work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
related_docs:
  - docs/modules/codebase-study.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
related_evals:
  - evals/specs/codebase-study-module-boundary.v0.1.md
  - evals/specs/codebase-study-conformance-packet.v0.1.md
  - evals/specs/codebase-study-source-observation-request-packet.v0.1.md
doc_impact:
  classification: docs-only
  refs:
    - docs/modules/codebase-study.md
    - evals/specs/codebase-study-source-observation-request-packet.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_source_observation_request_packet_v0_1.md
    - work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md
research_gate:
  classification: R1
  required: true
---

# Codebase Study source observation request packet v0.1

## Summary

Defined the future Codebase Study source observation request packet boundary.

The packet is request shape only:

```yaml
request_packet_boundary:
  packet_status: requested
  authority: request_only
  capability_grants: []
  non_authority: true
```

It records what a caller must supply before future Codebase Study source
observation can be requested. It does not grant execution, scanning, traversal,
content reads, hashing, size collection, storage, lab execution, benchmark
authority, gate/proof authority, or acceptance.

## What changed

- Added the source observation request packet section to
  `docs/modules/codebase-study.md`.
- Added `evals/specs/codebase-study-source-observation-request-packet.v0.1.md`.
- Closed the selected request packet goal.
- Selected
  `work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md`
  as the next bounded checkpoint.

## Packet boundary

Required request-packet surfaces:

- packet id and schema version;
- `module_id = codebase-study`;
- requested operation kind;
- requester or goal ref;
- explicit source scope refs and exclude refs;
- requested observation categories;
- capability requests, kept separate from grants;
- privacy policy ref, redaction policy ref, or explicit blockers;
- expected output authority;
- downstream handoff intent;
- evaluation requirement refs or blockers;
- warnings, limitations, blockers, or refusal state;
- `non_authority = true`.

Illustrative shape:

```yaml
codebase_study_source_observation_request_packet:
  schema_version: codebase-study-source-observation-request-packet.v0.1
  packet_status: requested
  authority: request_only
  module_id: codebase-study
  requested_by_ref: work/goals/example.md
  operation:
    kind: source_observation_request
  source_scope:
    mode: explicit_refs_only
    include_refs:
      - docs/modules/codebase-study.md
    exclude_refs:
      - .punk/**
  requested_observation_categories:
    - path_exists
    - path_kind_candidate
    - source_class_candidate
    - warning
    - blocker
    - limitation
  capability_requests:
    - scoped_path_observation_over_explicit_refs
  capability_grants: []
  privacy_policy_ref: null
  redaction_policy_ref: null
  expected_output:
    kind: advisory_source_inventory_observation_packet
    packet_status: advisory
    authority: observed_structure
  downstream_handoff_intent:
    target: brownfield_observation_packet_boundary
    source_corpus_manifest_writer: not_requested
  blockers:
    - capability_grants_not_selected
    - privacy_policy_not_selected
    - redaction_policy_not_selected
  non_authority: true
```

## Rejected requests

The boundary rejects or blocks requests for:

- ambient repository discovery;
- implicit current-working-directory traversal;
- auto-discovery of all files;
- file content reads, code summaries, snippets, document excerpts, raw
  environment values, or secret-like values;
- source filesystem hashes or file sizes;
- Source Corpus Manifest generation or writer behavior;
- runtime `.punk` writes or event-log mutation;
- `code-intel-kernel` execution, `agent-bench-lab` execution, lab code import,
  or benchmark-result authority;
- gate decisions, proof writing, acceptance claims, project truth, or final
  decisions.

## Boundary confirmation

- No Rust code was changed.
- No request parser was implemented.
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
  reason: "Defines the Codebase Study source observation request packet boundary before any implementation or active source observation behavior."
  touched_surfaces:
    - docs/modules/codebase-study.md
    - evals/specs/codebase-study-source-observation-request-packet.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_source_observation_request_packet_v0_1.md
    - work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md
  required_updates:
    - docs/modules/codebase-study.md
    - evals/specs/codebase-study-source-observation-request-packet.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_source_observation_request_packet_v0_1.md
    - work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md
```

## Validation

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files docs/modules/codebase-study.md evals/specs/codebase-study-source-observation-request-packet.v0.1.md work/STATUS.md work/goals/goal_define_codebase_study_source_observation_request_packet_v0_1.md work/goals/goal_define_codebase_study_capability_privacy_boundary_v0_1.md work/reports/2026-06-08-codebase-study-source-observation-request-packet-v0-1.md` passed with zero warnings.
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

Define Codebase Study capability and privacy boundary v0.1:

- keep capability requests separate from grants;
- define denied capabilities and future grant prerequisites;
- define privacy/redaction blockers;
- keep content reads, source snippets, raw environment values, secret-like
  values, source hashing, size collection, runtime storage, lab execution,
  benchmark authority, gate/proof authority, and acceptance denied.
