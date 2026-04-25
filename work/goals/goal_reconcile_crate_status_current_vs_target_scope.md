---
id: goal_reconcile_crate_status_current_vs_target_scope
title: "Reconcile CRATE-STATUS current-vs-target scope"
status: ready
owner: "vitaly"
module: "docs"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
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
acceptance:
  - "`docs/product/CRATE-STATUS.md` distinguishes current implemented behavior from target crate ownership where gate/proof/hash wording could overclaim runtime behavior."
  - "The doc remains honest that current implemented CLI is only `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
  - "The update does not claim gate/proof writers, `.punk/` storage, hash computation, hash normalization, schemas, adapters, automation, or `punk init` are active."
  - "No Rust code, CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "docs/product/ROADMAP.md"
  - "work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md"
  - "work/reports/2026-04-25-fifteenth-work-ledger-review.md"
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
  rationale: "This is a docs honesty update against repo-tracked implementation state and canonical docs; no external research or architecture change is required."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/ROADMAP.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md"
    - "work/reports/2026-04-25-fifteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-25-crate-status-current-vs-target-scope.md"
  rationale: "Execution will update a canonical product doc for current-vs-target crate scope honesty and record a work report."
---

## Context

Punk now has side-effect-free kernels and smoke eval coverage for contract/flow/receipt/gate/proof boundaries.

`docs/product/CRATE-STATUS.md` still uses compact target-style wording such as `decision writing` and `proofpack writing/hashing` in the crate ownership table. That can be read as current runtime writer/hash behavior, while current implementation remains side-effect-free and runtime writer/storage/hash computation are deferred.

## Notes

Keep this as a docs honesty update only.

Do not change Rust code.
Do not add CLI behavior.
Do not write `.punk/` state.
Do not implement gate or proofpack writers.
Do not compute or normalize hashes.
Do not add schema files.
Do not add adapters, automation, provider/model runners, or `punk init`.
