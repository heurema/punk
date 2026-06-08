---
id: goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1
title: "Verify brownfield source inventory observation packet boundary v0.1"
status: done
owner: "vitaly"
module: "project"
priority: P2
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: 2026-06-08
completed_at: 2026-06-08
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Verifies the Brownfield source inventory observation packet boundary remains docs/eval guidance only."
  - "Confirms packet authority stays advisory observed structure and cannot be confused with project truth, a manifest, a scanner result, gate/proof authority, acceptance, or benchmark authority."
  - "Confirms packet handoff into the Source Corpus Manifest model track does not activate writer behavior, repo scanning, file walking, content reads, hashing, size collection, claims, CLI behavior, runtime storage, or broader Writer behavior."
  - "Confirms `code-intel-kernel` and `agent-bench-lab` refs remain advisory and require current checkout verification before future reuse."
  - "Records verification evidence and selects the next bounded goal."
knowledge_refs:
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/brownfield-inventory-boundary.v0.1.md"
  - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
  - "work/reports/2026-06-08-brownfield-source-inventory-observation-packet-boundary-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is verification-only for a docs/eval observation-packet boundary derived from current repo-tracked docs, eval specs, and advisory local-lab research notes. External research is not needed unless scope changes toward active scanning, traversal, content reads, hashing, claims, runtime storage, lab execution, or implementation."
  research_refs:
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
    - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
    - "work/reports/2026-06-08-brownfield-source-inventory-observation-packet-boundary-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/brownfield-inventory-boundary.v0.1.md"
    - "evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md"
  rationale: "Verification found a maintainer clarification that codebase study should be a separate Unix-style Punk module, so the Brownfield packet docs/evals were narrowed before selecting the next module-boundary goal."
---

## Context

The Brownfield source inventory observation packet boundary is now captured in
docs and eval specs. It defines packet authority, explicit inputs, fail-closed
blockers, advisory lab use, `agent-bench-lab` evaluation requirements, and
handoff into the Source Corpus Manifest model track.

No observer, scanner, file walker, source content reader, filesystem hash
collector, manifest generator from repository state, runtime writer, CLI, lab
runner, or benchmark execution is active.

## Intent

Verify that the observation packet boundary is clear, non-authoritative, and
cannot be mistaken for active source inventory implementation or manifest
writer expansion.

## Verification focus

- Packet authority remains `advisory` and `observed_structure`.
- Packet inputs remain explicit, scoped, repo-relative, and evidence-linked.
- Missing evidence, sensitive paths, generated/vendored candidates, and
  unknown classifications remain visible rather than upgraded.
- Packet handoff maps only observed structure into manifest model fields.
- The active writer slice still accepts only an already-constructed manifest
  model, explicit target, and preflight result.
- Lab references remain advisory and require current checkout verification
  before future reuse.
- Future active observer/scanner results require an `agent-bench-lab`
  evaluation route before influencing Brownfield decisions.

## Non-scope

Do not implement source inventory generation, repo scanning, file walking,
source content reading, source filesystem hashing, size collection, manifest
generation from repository state, claim extraction, AI summaries, module maps,
architecture recovery, intent recovery, contract generation, gate/proof
runtime, Punk `Writer` behavior, runtime `.punk` storage, CLI behavior,
grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, spec-as-source behavior, lab code import, benchmark
suite execution, or benchmark-result authority.

## Outcome

Done in `work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md`.

Verification passed with a boundary refinement: future codebase study is a
separate Unix-style Punk module upstream of the observation packet, not
Brownfield core, `punk-project`, the Source Corpus Manifest writer, or generic
Punk `Writer` behavior.

The verified packet stays advisory observed structure only. The future module
may return an advisory source inventory observation packet from explicit input
and narrow capability grants, but it must not own final Brownfield decisions,
contract approval, gate decisions, proof, acceptance, runtime `.punk` storage,
writer behavior, or broad Punk orchestration.

Selected next:

```text
work/goals/goal_define_codebase_study_module_boundary_v0_1.md
```
