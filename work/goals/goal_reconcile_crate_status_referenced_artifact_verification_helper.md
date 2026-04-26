---
id: goal_reconcile_crate_status_referenced_artifact_verification_helper
title: "Reconcile CRATE-STATUS referenced artifact verification helper status"
status: ready
owner: "vitaly"
module: "docs"
priority: P1
authority: canonical
created_at: 2026-04-26
updated_at: 2026-04-26
selected_at: 2026-04-26
started_at: null
completed_at: null
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
    - "schemas/**"
acceptance:
  - "`docs/product/CRATE-STATUS.md` reflects that `punk-core` now includes a narrow evidence-only referenced artifact verification helper for one explicit repo-rooted regular file, typed repo-relative ref, and canonical expected digest."
  - "`docs/product/CRATE-STATUS.md` reflects that `punk-eval` smoke coverage includes referenced artifact verification helper behavior as local assessment only."
  - "CRATE-STATUS still clearly defers proofpack writer integration, proofpack referenced-ref verification integration, runtime storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init`."
  - "No Rust code, `.punk/` runtime state, schema files, CLI behavior, proofpack writer behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, or `punk init` are added."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
  - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "crates/punk-core/src/lib.rs"
  - "crates/punk-eval/src/lib.rs"
  - "work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md"
  - "work/reports/2026-04-26-thirty-third-work-ledger-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R0
  required: false
  rationale: "This is a docs-currentness reconciliation against repo-tracked implementation/report evidence only; no external research or architecture decision is required."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/referenced-artifact-verification-boundary.v0.1.md"
    - "evals/specs/file-io-artifact-hashing-boundary.v0.1.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/reports/2026-04-26-referenced-artifact-verification-helper-v0-1.md"
    - "work/reports/2026-04-26-thirty-third-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-crate-status-referenced-artifact-verification-helper.md"
  rationale: "The goal updates canonical crate-status wording and work artifacts only, without runtime/code/schema/CLI changes."
---

## Context

Referenced artifact verification helper v0.1 is now implemented in `punk-core` and covered by `punk-eval` smoke cases.

`docs/product/CRATE-STATUS.md` still contains pre-helper wording that says `punk-core` does not verify referenced artifact bytes and does not list referenced artifact verification smoke coverage.

Reconcile only canonical currentness wording. Do not add code, runtime, schema, CLI, writer, or gate behavior.

## Notes

Keep the wording narrow:

- the helper compares a canonical expected digest with the observed digest for one explicit regular file under one explicit repo root and validated repo-relative ref;
- the helper returns evidence-only outcomes;
- proofpack writer integration, proofpack referenced-ref verification integration, runtime storage, schemas, CLI behavior, gate decisions, acceptance claims, adapters, automation, provider/model runners, and `punk init` remain deferred.
