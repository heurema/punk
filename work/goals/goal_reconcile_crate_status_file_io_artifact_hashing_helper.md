---
id: goal_reconcile_crate_status_file_io_artifact_hashing_helper
title: "Reconcile CRATE-STATUS file IO artifact hashing helper status"
status: done
owner: "vitaly"
module: "docs"
priority: P1
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: 2026-04-26
completed_at: 2026-04-26
blocked_by: []
scope:
  include:
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "`docs/product/CRATE-STATUS.md` accurately describes `punk-core` as owning exact-byte artifact hash computation plus a narrow file IO artifact hashing helper over explicit repo roots and validated repo-relative refs."
  - "`docs/product/CRATE-STATUS.md` records that file IO artifact hashing helper reads one regular file and remains evidence only."
  - "`docs/product/CRATE-STATUS.md` records that `punk-eval` smoke coverage includes file IO artifact hashing helper behavior."
  - "The wording keeps referenced artifact byte verification, proofpack writer behavior, proofpack writer hash-policy integration, `.punk/` runtime storage, schema files, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` deferred."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
  - "No Rust code, CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/artifact-hash-computation-helper.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "crates/punk-core/src/lib.rs"
  - "crates/punk-eval/src/lib.rs"
  - "work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md"
  - "work/reports/2026-04-26-thirtieth-work-ledger-review.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a docs-currentness reconciliation against repo-tracked implementation evidence and canonical crate status; no external research is required unless a product-doc conflict is found."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/artifact-hash-computation-helper.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-file-io-artifact-hashing-helper-v0-1.md"
    - "work/reports/2026-04-26-thirtieth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-crate-status-file-io-artifact-hashing-helper.md"
  rationale: "The selected goal should update canonical crate status wording and record the docs-currentness reconciliation without code/runtime changes."
---

## Context

`punk-core` now exposes a narrow file IO artifact hashing helper that computes canonical digest metadata for one explicit regular file under one explicit absolute repo root and validated repo-relative artifact ref.

`punk-eval` smoke coverage now includes this helper behavior as local assessment only.

`docs/product/CRATE-STATUS.md` still says `punk-core` does not read files and that file IO hashing is not active. That wording no longer matches the implemented helper subset.

## Notes

Do not change Rust code.
Do not add schema files.
Do not add CLI commands.
Do not write `.punk` state.
Do not implement proofpack writer behavior.
Do not add referenced artifact byte verification.
Do not imply acceptance authority.
Do not add adapters, automation, provider/model runners, or `punk init`.


## Outcome

Completed the CRATE-STATUS file IO artifact hashing helper status reconciliation.

`docs/product/CRATE-STATUS.md` now records that `punk-core` owns artifact digest/ref validation, exact-byte artifact hash computation, and a narrow evidence-only file IO artifact hashing helper for one explicit regular file under an explicit repo root and validated repo-relative artifact ref.

It also records that `punk-eval` smoke coverage includes file IO artifact hashing helper behavior.

The reconciliation did not add Rust code, CLI behavior, schema files, `.punk/` runtime state, proofpack writer behavior, referenced artifact byte verification, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init`.
