---
id: goal_add_proofpack_integrity_smoke_eval_coverage
title: "Add proofpack integrity smoke eval coverage"
status: ready
owner: "vitaly"
module: "core"
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
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - ".punk/**"
acceptance:
  - "Smoke eval covers proofpack link/hash integrity readiness using the existing side-effect-free proofpack kernel."
  - "Smoke cases verify that complete declared ref/digest links can satisfy matching proof readiness."
  - "Smoke cases verify that missing required proofpack digests remain visible and block matching proof readiness."
  - "Smoke output remains a local assessment and does not write `.punk/evals`, gate decisions, proofpacks, acceptance claims, runtime storage, or CLI state."
  - "No CLI command, `.punk/` state, runtime writer, provider/model/agent adapter, automation, or schema file is added."
knowledge_refs:
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md"
  - "work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md"
  - "work/reports/2026-04-25-fourteenth-work-ledger-review.md"
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
  rationale: "This will add deterministic smoke eval coverage over already-defined and implemented side-effect-free proofpack integrity helpers without new architecture decisions or runtime activation."
  research_refs:
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "work/reports/2026-04-25-gate-proof-acceptance-smoke-eval.md"
    - "work/reports/2026-04-25-proofpack-link-hash-integrity-kernel-v0-1.md"
    - "work/reports/2026-04-25-fourteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-25-proofpack-integrity-smoke-eval-coverage.md"
  rationale: "Execution will change active-core eval behavior and record a work report; no product docs are expected unless active CLI or runtime semantics change."
---

## Context

Punk now has:

- side-effect-free gate decision authority;
- side-effect-free proofpack provenance metadata;
- structural proofpack link/hash integrity helpers;
- deterministic smoke eval coverage for the broader gate/proof acceptance chain.

The current smoke eval still checks abstract matching proofpack preconditions. It does not yet exercise complete/missing proofpack digest-link readiness.

## Notes

Keep this as local deterministic eval coverage only.

Do not write `.punk/evals`.
Do not add CLI behavior.
Do not implement gate runtime.
Do not implement proofpack writer.
Do not compute or normalize hashes.
Do not claim acceptance.
Do not add adapters, automation, provider/model runners, or schema files.
