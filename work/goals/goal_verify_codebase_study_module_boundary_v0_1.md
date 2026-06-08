---
id: goal_verify_codebase_study_module_boundary_v0_1
title: "Verify codebase study module boundary v0.1"
status: ready
owner: "vitaly"
module: "product"
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
    - "work/goals/goal_verify_codebase_study_module_boundary_v0_1.md"
    - "work/reports/**"
    - "docs/modules/codebase-study.md"
    - "docs/product/MODULES.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "evals/specs/codebase-study-module-boundary.v0.1.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "schemas/**"
acceptance:
  - "Verifies Codebase Study remains a parked docs/eval module boundary only."
  - "Confirms canonical module id is `codebase-study` and aliases such as `CodePunk`, `SourcePunk`, and `code` remain non-canonical."
  - "Confirms the Unix-style input/output boundary rejects implicit repo discovery, auto traversal, content reads, manifest assembly, `.punk` writes, gate/proof authority, and acceptance."
  - "Records verification evidence and selects the next bounded goal."
knowledge_refs:
  - "docs/modules/codebase-study.md"
  - "docs/product/MODULES.md"
  - "docs/product/MODULE-AUTHORING.md"
  - "docs/product/MODULE-HOST-CONTRACT.md"
  - "docs/product/BROWNFIELD-INVENTORY.md"
  - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
  - "evals/specs/codebase-study-module-boundary.v0.1.md"
  - "work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md"
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
  rationale: "This is verification-only for a docs/eval module boundary derived from current repo-tracked docs and advisory provider passes. External research is not needed unless scope changes toward implementation, active scanning, traversal, content reads, hashing, claims, runtime storage, module execution, lab execution, or external benchmark claims."
  research_refs:
    - "docs/modules/codebase-study.md"
    - "docs/product/MODULES.md"
    - "docs/product/MODULE-AUTHORING.md"
    - "docs/product/MODULE-HOST-CONTRACT.md"
    - "docs/product/BROWNFIELD-INVENTORY.md"
    - "docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md"
    - "evals/specs/codebase-study-module-boundary.v0.1.md"
    - "work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: work-ledger
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Verification should update only work-ledger artifacts unless it finds drift that requires a narrow docs or eval-spec fix."
---

## Context

Codebase Study is now defined as a parked future Punk module boundary with the
canonical module id `codebase-study`.

It has docs/eval coverage for explicit source observation request input,
advisory observation packet output, failure modes, denied capabilities, lab
advisory boundaries, and negative cases for auto-discovery, content reads,
manifest assembly, `.punk` writes, and authority claims.

## Intent

Verify that the Codebase Study module boundary is clear, non-authoritative, and
cannot be mistaken for active scanner or source inventory implementation.

## Non-scope

Do not implement source inventory generation, repo scanning, file walking,
source content reading, source filesystem hashing, size collection, manifest
generation from repository state, claim extraction, AI summaries, module maps,
architecture recovery, intent recovery, contract generation, gate/proof
runtime, Punk `Writer` behavior, runtime `.punk` storage, CLI behavior, module
execution, lab code import, benchmark suite execution, or benchmark-result
authority.
