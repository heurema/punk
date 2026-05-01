---
id: goal_prepare_contract_core_commit_plan_v0_1
title: "Prepare contract-core commit plan v0.1"
status: done
owner: "vitaly"
module: "process"
priority: P1
authority: canonical
created_at: 2026-05-01
updated_at: 2026-05-01
selected_at: 2026-05-01
started_at: 2026-05-01
completed_at: 2026-05-01
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/reports/**"
    - "work/goals/**"
  exclude:
    - ".punk/**"
    - "schemas/**"
acceptance:
  - "A concrete commit/split plan is prepared for the current uncommitted tree without staging or committing unless explicitly requested."
  - "The plan groups contract-core model/eval work, proofpack-writer track work, replayability research, docs/status updates, and work-ledger evidence into reviewable context units."
  - "The plan identifies any files requiring patch-level splitting because they contain changes from multiple tracks."
  - "No implementation, runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, or `punk init` behavior is added."
knowledge_refs:
  - "work/STATUS.md"
  - "work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-contract-core-commit-plan-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is repo-local commit planning over existing uncommitted artifacts; external research is not needed unless new product/runtime behavior is proposed."
  research_refs:
    - "work/STATUS.md"
    - "work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: process-ledger
  required_updates:
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The stabilization review recommends split commit planning before staging or committing the large mixed local tree."
---

## Context

The uncommitted tree is coherent but large and mixed across contract-core, proofpack-writer, replayability research, docs, goals, and reports.

Prepare a concrete commit/split plan before any staging, commit, PR, Writer, or runtime work.

## Notes

This is a commit-planning task only.

Do not stage, commit, push, or open PRs unless explicitly requested.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime writers, Writer activation, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.


## Completion

Completed on 2026-05-01.

Outcome:

- Classified the large uncommitted tree into contract-core, proofpack-writer, replayability research, shared ledger/status, and split-required shared files.
- Identified `crates/punk-eval/src/lib.rs`, `docs/product/CRATE-STATUS.md`, and `work/STATUS.md` as files that require careful patch-level staging or final ledger handling.
- Recommended split commits and provided draft commit messages without staging or committing.
- Selected a manual commit decision checkpoint next because the maintainer should choose commit execution strategy explicitly.

No implementation, runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, or `punk init` behavior was added.
