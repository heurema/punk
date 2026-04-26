---
id: goal_add_artifact_hash_policy_helpers_v0_1
title: "Add artifact hash policy helpers v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: 2026-04-25
completed_at: 2026-04-25
blocked_by: []
scope:
  include:
    - "crates/punk-core/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-proof/**"
    - ".punk/**"
acceptance:
  - "`punk-core` exposes side-effect-free helpers for artifact hash policy v0.1 digest format validation."
  - "Helpers recognize canonical `sha256:<64 lowercase hex chars>` digests and reject uppercase labels, uppercase hex, bare hex, unsupported algorithms, empty values, and placeholders."
  - "Helpers expose repo-relative artifact ref validation for no leading slash, no `.`/`..`/empty segments, no backslashes, no URL scheme, and no `~` prefix."
  - "Helpers are validation/parsing only and do not compute hashes, normalize artifact bytes, write schemas, write proofpacks, write gate decisions, or touch `.punk/` runtime state."
  - "No CLI command, provider/model/agent adapter, automation, runtime writer, schema file, or dependency is added."
knowledge_refs:
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "docs/product/CRATE-STATUS.md"
  - "work/reports/2026-04-25-artifact-hash-policy-v0-1.md"
  - "work/reports/2026-04-25-seventeenth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This is a narrow implementation of the already-defined artifact hash policy v0.1 as side-effect-free validation helpers without hashing, dependencies, schemas, runtime storage, or CLI behavior."
  research_refs:
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-25-artifact-hash-policy-v0-1.md"
    - "work/reports/2026-04-25-seventeenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
  rationale: "Execution will implement side-effect-free helper behavior in active-core and record a work report; product docs/spec updates are not expected unless behavior boundaries change."
---

## Context

Artifact hash policy v0.1 now defines digest identity and artifact ref boundaries as docs/spec.

`punk-core` is the active-core crate intended for deterministic helpers and future hashing helpers, but it is still a minimal skeleton.

The next smallest implementation step is validation/parsing helpers only:

- canonical digest format validation;
- repo-relative artifact ref validation;
- no hash computation;
- no proofpack integration;
- no CLI/runtime/schema activation.

## Notes

Keep this side-effect-free and dependency-free.

Do not compute hashes.
Do not normalize artifact bytes.
Do not change `punk-proof` in this slice.
Do not add schema files.
Do not write proofpacks.
Do not write gate decisions.
Do not add CLI behavior.
Do not write `.punk/` state.
Do not claim acceptance.


## Outcome

Completed artifact hash policy helpers v0.1 in `punk-core`.

Added side-effect-free helpers for:

- canonical digest validation: `sha256:<64 lowercase hex chars>`;
- repo-relative artifact ref validation;
- explicit capability flags showing no hash computation, no artifact-byte normalization, and no runtime writes.

The helpers reject uppercase labels, uppercase hex, bare hex, unsupported algorithms, empty values, placeholders, absolute refs, home refs, URL refs, backslash refs, empty path segments, `.` segments, and `..` segments.

No CLI behavior, `.punk/` state, schema file, dependency, `punk-proof` integration, proofpack writer, gate decision writer, provider/model/agent adapter, automation, active hash computation, hash normalization, or acceptance claim was added.
