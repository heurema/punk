---
id: goal_define_artifact_hash_policy_v0_1
title: "Define artifact hash policy v0.1"
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
    - "evals/specs/**"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Artifact hash policy v0.1 is defined as a docs/spec-only artifact before hash computation, hash normalization, proofpack writer, or runtime storage work."
  - "Policy defines digest algorithm label expectations, digest string shape, artifact ref/path normalization boundary, and invalid/missing digest visibility."
  - "Policy states how future proofpack, gate decision, receipt, eval, and output artifact refs may cite hashes without absorbing raw artifact bodies."
  - "Policy keeps executor claims, smoke eval output, and proofpack metadata as evidence inputs, not final acceptance authority."
  - "No runtime code, CLI behavior, schema file, dependency, hash implementation, proofpack writer, gate decision writer, provider/model/agent adapter, automation, or `.punk/` state is added."
knowledge_refs:
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/ARCHITECTURE.md"
  - "docs/product/CRATE-STATUS.md"
  - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
  - "evals/specs/proofpack-boundary.v0.1.md"
  - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
  - "work/reports/2026-04-25-sixteenth-work-ledger-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This defines a narrow docs/spec policy for stable artifact hashes using existing canonical docs, ADRs, and proofpack/proof-before-acceptance specs before any implementation, dependency, schema, writer, or runtime storage work."
  research_refs:
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/ARCHITECTURE.md"
    - "docs/product/CRATE-STATUS.md"
    - "docs/adr/ADR-0012-minimal-proofpack-provenance.md"
    - "evals/specs/proofpack-boundary.v0.1.md"
    - "evals/specs/proof-before-acceptance-semantics.v0.1.md"
    - "work/reports/2026-04-25-sixteenth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "evals/specs/artifact-hash-policy.v0.1.md"
    - "work/reports/2026-04-25-artifact-hash-policy-v0-1.md"
  rationale: "Execution will add a docs/spec artifact that defines stable artifact hash policy before active hash implementation or writer/storage work."
---

## Context

Punk now has side-effect-free proofpack metadata, structural link/hash integrity checks, and smoke eval coverage for missing/complete proofpack digest links.

The next runtime-looking branches are still too large without a stable artifact hash policy:

- proofpack writer;
- hash computation;
- hash normalization;
- gate/eval/proof orchestration;
- `.punk/` proof/eval/run/decision storage.

Define the policy first so later code can be bounded and honest.

## Notes

Keep this docs/spec-only.

Do not implement hash computation.
Do not add a hashing dependency.
Do not normalize hashes in code.
Do not add schema files.
Do not write proofpacks.
Do not write gate decisions.
Do not add CLI behavior.
Do not write `.punk/` state.
Do not claim acceptance.
