---
id: goal_add_artifact_hash_policy_smoke_eval_coverage
title: "Add artifact hash policy smoke eval coverage"
status: ready
owner: "vitaly"
module: "core"
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
    - "crates/punk-eval/Cargo.toml"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - "crates/punk-proof/**"
    - ".punk/**"
acceptance:
  - "Smoke eval covers artifact hash policy helper behavior using the existing side-effect-free `punk-core` helpers."
  - "Smoke cases cover at least canonical digest acceptance, invalid digest rejection, valid repo-relative ref acceptance, invalid ref rejection, and helper boundary flags."
  - "Smoke output remains local assessment only and does not write `.punk/evals`, gate decisions, proofpacks, acceptance claims, runtime storage, or CLI state."
  - "No proofpack writer, active hash computation, byte normalization, schema file, provider/model/agent adapter, automation, or `punk init` is added."
knowledge_refs:
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
  - "work/reports/2026-04-26-eighteenth-work-ledger-review.md"
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
  rationale: "This adds deterministic smoke eval coverage over already-defined and implemented side-effect-free artifact hash policy helpers without new architecture decisions or runtime activation."
  research_refs:
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
    - "work/reports/2026-04-26-eighteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md"
  rationale: "Execution will change active-core eval behavior and record a work report; no product docs/spec updates are expected unless behavior boundaries change."
---

## Context

Artifact hash policy helpers v0.1 are implemented and unit-tested in `punk-core`.

The smoke eval still focuses on contract, flow, receipt, event, gate, and proof kernels. It does not yet expose artifact hash policy helper regressions through `punk eval run smoke`.

The next smallest guardrail is deterministic smoke coverage for the helper boundary before `punk-proof` integration, proofpack writer, runtime storage, schema work, or active hash computation.

## Notes

Keep this as local deterministic eval coverage only.

Do not write `.punk/evals`.
Do not add CLI behavior.
Do not change `punk-proof` in this slice.
Do not implement gate runtime.
Do not implement proofpack writer.
Do not compute hashes.
Do not normalize artifact bytes.
Do not add schema files.
Do not add adapters, automation, provider/model runners, or `punk init`.
