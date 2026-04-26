---
id: goal_reconcile_crate_status_artifact_hash_helper_status
title: "Reconcile CRATE-STATUS artifact hash helper status"
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
acceptance:
  - "`docs/product/CRATE-STATUS.md` accurately describes `punk-core` as owning implemented side-effect-free artifact digest and repo-relative artifact ref validation helpers, without claiming active hash computation or byte normalization."
  - "`docs/product/CRATE-STATUS.md` records that `punk-eval` smoke coverage includes artifact hash policy helper behavior, without claiming `.punk/evals` storage, baselines, waivers, or runtime eval reports."
  - "`docs/product/CRATE-STATUS.md` records that `punk-proof` validates proof artifact hash string shape through `punk-core` helpers, while proofpack link/hash integrity remains structural only."
  - "The doc does not claim proofpack writer behavior, active artifact hash computation, byte normalization, `.punk/proofs` storage, gate acceptance, schema files, adapters, automation, provider/model runners, extra CLI commands, or `punk init` are active."
  - "The current implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
  - "No Rust code, CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "docs/product/CRATE-STATUS.md"
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
  - "work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md"
  - "work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md"
  - "work/reports/2026-04-26-twentieth-work-ledger-review.md"
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
  rationale: "This is a docs-currentness update against repo-tracked implementation state and canonical docs; no external research or architecture change is required."
  research_refs:
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
    - "work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md"
    - "work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md"
    - "work/reports/2026-04-26-twentieth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-26-crate-status-artifact-hash-helper-status.md"
  rationale: "Execution will update a canonical product doc for current helper implementation honesty and record a work report."
---

## Context

Artifact hash policy v0.1 is now defined, implemented as side-effect-free `punk-core` validation helpers, covered by the local smoke eval, and used by `punk-proof` proof artifact hash validation.

`docs/product/CRATE-STATUS.md` still says `punk-core` is a minimal helper crate skeleton. That wording no longer matches the implemented helper subset.

## Notes

Keep this as a docs-currentness update only.

Do not change Rust code.
Do not add CLI behavior.
Do not write `.punk/` state.
Do not implement gate or proofpack writers.
Do not compute hashes.
Do not normalize bytes or hashes.
Do not add schema files.
Do not add adapters, automation, provider/model runners, or `punk init`.
