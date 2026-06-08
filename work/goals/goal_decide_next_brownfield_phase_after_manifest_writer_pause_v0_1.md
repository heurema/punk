---
id: goal_decide_next_brownfield_phase_after_manifest_writer_pause_v0_1
title: "Decide next brownfield phase after manifest writer pause v0.1"
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
    - "work/goals/goal_decide_next_brownfield_phase_after_manifest_writer_pause_v0_1.md"
    - "work/reports/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/**"
    - "evals/specs/**"
acceptance:
  - "Chooses exactly one next bounded Brownfield phase or records an explicit deferral."
  - "Keeps source inventory generation, repo scanning, file walking, content reading, source filesystem hashing, claims, runtime storage, CLI behavior, and broader Writer behavior out of this decision-only step."
  - "Records the decision, rationale, non-scope, and next selected goal or deferral in the work ledger."
knowledge_refs:
  - "work/reports/2026-06-08-pause-after-brownfield-manifest-writer-first-slice-v0-1.md"
  - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-verification-v0-1.md"
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
  rationale: "This goal is a local work-ledger decision checkpoint. External research is not needed unless the selected next phase expands into repo traversal, content reads, source hashing, claims, runtime storage, or implementation."
  research_refs:
    - "work/reports/2026-06-08-pause-after-brownfield-manifest-writer-first-slice-v0-1.md"
    - "work/reports/2026-05-04-brownfield-source-corpus-manifest-writer-first-slice-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: work-ledger-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The next step should decide the next bounded Brownfield phase without activating source inventory or runtime behavior."
---

## Context

The first side-effectful Brownfield Source Corpus Manifest writer slice has a
closed pause checkpoint.

The writer slice remains limited to an already-constructed model, explicit safe
target, successful preflight, deterministic canonical bytes, one safe target
write, and in-memory non-authoritative operation evidence.

## Intent

Decide the next bounded Brownfield phase before any broader source inventory
work is selected.

## Decision boundary

The decision may select a future docs/spec boundary, verification step,
side-effect-free model, first active implementation slice, or explicit deferral.

The decision must name the next goal and preserve its non-scope before any
implementation, runtime storage, CLI behavior, or source inventory execution is
started.

## Non-scope

Do not implement source inventory generation, repo scanning, file walking,
source content reading, source filesystem hashing, manifest generation from
repository state, claim extraction, AI summaries, module maps, architecture
recovery, intent recovery, contract generation, gate/proof runtime, Punk
`Writer` behavior, runtime `.punk` storage, CLI behavior, grayfield
reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.
