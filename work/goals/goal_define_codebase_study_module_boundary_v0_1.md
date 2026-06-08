---
id: goal_define_codebase_study_module_boundary_v0_1
title: "Define codebase study module boundary v0.1"
status: done
owner: "vitaly"
module: "product"
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
    - "work/goals/goal_define_codebase_study_module_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/modules/**"
    - "evals/specs/**"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Defines the future codebase-study Punk module as a Unix-style module with explicit input packet and explicit advisory output packet."
  - "Selects or defers the module id/name without creating naming drift."
  - "Defines the module's relationship to Brownfield observation packets, Source Corpus Manifest handoff, Module Host, Module Authoring Baseline, and future conformance packet requirements."
  - "Keeps scanner implementation, file walking, source content reading, source filesystem hashing, size collection, claims, manifest generation from repository state, runtime storage, CLI behavior, module execution, lab code import, benchmark execution, gate/proof authority, and acceptance inactive."
knowledge_refs:
  - "docs/product/MODULES.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
  - "work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This defines a future module boundary and its relationship to Brownfield inventory, Module Host, and Module Authoring Baseline. Existing repo-tracked docs and the local-lab advisory note are enough unless scope expands toward implementation, active scanning, traversal, content reads, hashing, claims, runtime storage, module execution, lab execution, or external benchmark claims."
  research_refs:
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md"
    - "work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "docs/modules/**"
    - "evals/specs/**"
  rationale: "The next step should define the module boundary before any implementation or active source observation behavior."
---

## Context

The Brownfield source inventory observation packet boundary is verified as
docs/eval guidance only. The maintainer clarified that studying a codebase
should be owned by a separate Punk module, with each module following a
Unix-style contract: explicit input packet, one narrow responsibility, explicit
output packet, and no final authority.

## Intent

Define the future codebase-study module boundary before any implementation.

## Non-scope

Do not implement source inventory generation, repo scanning, file walking,
source content reading, source filesystem hashing, size collection, manifest
generation from repository state, claim extraction, AI summaries, module maps,
architecture recovery, intent recovery, contract generation, gate/proof
runtime, Punk `Writer` behavior, runtime `.punk` storage, CLI behavior, module
execution, lab code import, benchmark suite execution, or benchmark-result
authority.

## Outcome

Done in `work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md`.

The future codebase-study Punk module is defined as a parked docs/eval boundary
with canonical module id `codebase-study`, prose name `Codebase Study`, and docs
path `docs/modules/codebase-study.md`.

The boundary defines explicit source observation request input, advisory source
inventory observation packet output, failure modes, denied capabilities, and
negative eval cases for auto-discovery, content reads, manifest assembly,
runtime `.punk` writes, and authority claims.

No source inventory implementation, repo scan, file walker, content read,
source filesystem hash computation, size collection, manifest generation from
repository state, claim extraction, AI summary, runtime storage, CLI behavior,
module execution, benchmark execution, lab code import, or authority promotion
was activated.
