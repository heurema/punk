---
id: goal_integrate_artifact_hash_policy_helpers_into_punk_proof_v0_1
title: "Integrate artifact hash policy helpers into punk-proof v0.1"
status: ready
owner: "vitaly"
module: "proof"
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
    - "Cargo.lock"
    - "crates/punk-proof/Cargo.toml"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/punk-cli/**"
    - "docs/_schema/**"
    - ".punk/**"
acceptance:
  - "`punk-proof` artifact hash values are validated through the existing side-effect-free artifact hash policy helpers from `punk-core`."
  - "Invalid proof artifact hashes such as empty, placeholder, bare, uppercase, short, unsupported algorithm, or otherwise non-canonical digest strings are rejected."
  - "Proof/eval sample digests use deterministic canonical-shaped static strings without computing hashes over artifact bytes."
  - "Proofpack link/hash integrity remains structural: it checks declared ref/kind/hash presence and does not compute hashes, normalize artifact bytes, write proofpacks, write gate decisions, write acceptance claims, or write runtime storage."
  - "No CLI commands, `.punk/` state, schema files, provider/model/agent adapters, automation, active hash computation, byte normalization, proofpack writer, or `punk init` are added."
knowledge_refs:
  - "evals/specs/artifact-hash-policy.v0.1.md"
  - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
  - "work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md"
  - "work/reports/2026-04-26-nineteenth-work-ledger-review.md"
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
  rationale: "This integrates already-defined and smoke-covered side-effect-free `punk-core` artifact hash policy helpers into the existing side-effect-free proof kernel without new architecture decisions or runtime activation."
  research_refs:
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "work/reports/2026-04-25-artifact-hash-policy-helpers-v0-1.md"
    - "work/reports/2026-04-26-artifact-hash-policy-smoke-eval-coverage.md"
    - "work/reports/2026-04-26-nineteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/reports/2026-04-26-punk-proof-artifact-hash-policy-helper-integration.md"
  rationale: "Execution will change proof kernel validation behavior and record a work report; no product docs/spec updates are expected unless behavior boundaries change."
---

## Context

Artifact hash policy v0.1 is now defined, implemented as side-effect-free `punk-core` helpers, and covered by local smoke eval cases.

`punk-proof` still treats proof artifact hashes as non-empty strings and test/eval fixtures still contain placeholder digest strings such as `sha256:decisionhash`.

The next smallest proof step is to reuse the shared helpers for proof artifact hash validation while keeping link/hash integrity structural only.

## Notes

Do not compute artifact hashes.
Do not normalize artifact bytes.
Do not write proofpacks.
Do not write gate decisions.
Do not write acceptance claims.
Do not write `.punk/` runtime state.
Do not add CLI behavior.
Do not add schema files.
Do not add adapters, automation, provider/model runners, or `punk init`.
