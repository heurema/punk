---
id: goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1
title: "Prepare brownfield source inventory observation packet boundary v0.1"
status: ready
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/product/GLOSSARY.md"
    - "docs/product/CRATE-STATUS.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Defines a docs/eval boundary for a future Brownfield source inventory observation packet before any scanner, file walker, or manifest generation from repository state is selected."
  - "Names allowed explicit observation inputs, include/exclude policy, source-class classification limits, sensitivity policy, and fail-closed blockers."
  - "Defines how an accepted observation packet may hand off to the existing Brownfield Source Corpus Manifest model/writer track without granting authority promotion."
  - "Reviews `code-intel-kernel` as an advisory local lab input for observation-packet ideas without treating lab state as Punk product truth."
  - "Defines an `agent-bench-lab` evaluation requirement for future active observation/scanner results before those results influence Brownfield decisions."
  - "Keeps source inventory generation, repo scanning, file walking, source content reading, source filesystem hashing, size collection, claim extraction, AI summaries, runtime storage, CLI behavior, and broader Writer behavior inactive."
knowledge_refs:
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "work/reports/2026-06-08-pause-after-brownfield-manifest-writer-first-slice-v0-1.md"
  - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-verification-v0-1.md"
  - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
contract_refs: []
report_refs: []
decision_refs:
  - "work/reports/2026-06-08-decide-next-brownfield-phase-after-manifest-writer-pause-v0-1.md"
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This remains a docs/eval boundary step, but the selected goal now intentionally includes advisory local-lab comparison and explicit evaluation implications. External deep research is not needed unless the scope expands toward active scanning, traversal, content reads, hashing, claims, runtime storage, or implementation."
  research_refs:
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "work/reports/2026-06-08-pause-after-brownfield-manifest-writer-first-slice-v0-1.md"
    - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-verification-v0-1.md"
    - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  rationale: "The next step should define the observation packet boundary, compare local lab ideas, and define evaluation requirements before any Brownfield source inventory implementation."
---

## Context

The first active Brownfield Source Corpus Manifest writer slice is implemented
and verified, but it accepts only an already-constructed manifest model.

The missing next boundary is the shape of future observed source inventory input
before any repo traversal or manifest generation from repository state.

The local `code-intel-kernel` lab may provide advisory ideas for code
observation boundaries. The local `agent-bench-lab` must be used to define how
future active observation/scanner results will be evaluated before those
results influence Brownfield decisions.

Local lab state must be verified at execution time and must not become Punk
product truth by reference.

## Intent

Define a docs/eval observation packet boundary for future Brownfield source
inventory work.

## Non-scope

Do not implement source inventory generation, repo scanning, file walking,
source content reading, source filesystem hashing, size collection, manifest
generation from repository state, claim extraction, AI summaries, module maps,
architecture recovery, intent recovery, contract generation, gate/proof
runtime, Punk `Writer` behavior, runtime `.punk` storage, CLI behavior,
grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, spec-as-source behavior, lab code import, benchmark
suite execution, or benchmark-result authority.
