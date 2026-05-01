---
id: goal_manual_commit_decision_contract_core_v0_1
title: "Manual commit decision for contract-core tree v0.1"
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
  - "The maintainer chooses whether to execute split commits, hold the proofpack-writer track, clean docs warnings first, or keep the tree uncommitted."
  - "No staging, committing, pushing, or PR creation happens unless explicitly requested by the maintainer."
  - "The decision preserves the separation between contract-core, proofpack-writer, replayability research, and work-ledger/status tracks."
  - "No implementation, runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, or `punk init` behavior is added."
knowledge_refs:
  - "work/STATUS.md"
  - "work/reports/2026-05-01-contract-core-commit-plan-v0-1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local commit strategy decision over existing uncommitted artifacts; external research is not needed unless new product/runtime behavior is proposed."
  research_refs:
    - "work/STATUS.md"
    - "work/reports/2026-05-01-contract-core-commit-plan-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: process-ledger
  required_updates:
    - "work/STATUS.md"
  rationale: "The commit plan recommends an explicit human decision before any staging or commit execution."
---

## Context

The uncommitted tree is coherent and checks are green, but it mixes contract-core, proofpack-writer, replayability research, docs/status, eval specs, goals, and reports.

The commit plan recommends split commits but asks for an explicit maintainer decision before staging or committing because shared files require patch-level staging:

- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`

## Notes

This is a decision checkpoint only.

Do not stage, commit, push, or open PRs unless explicitly requested.

Do not add CLI behavior, `.punk/contracts` storage, `.punk/runs` storage, `.punk/decisions` storage, `.punk/proofs` storage, runtime writers, Writer activation, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, agent execution, provider adapters, policy engine integration, runtime side effects, or `punk init`.


## Completion

Completed on 2026-05-01.

Outcome:

- Reviewed the split-risk files `crates/punk-eval/src/lib.rs` and `docs/product/CRATE-STATUS.md`.
- Found patch staging technically possible but risky because the files mix contract-core and proofpack-writer imports, smoke-suite registration, assessment text, boundary notes, and long status bullets.
- Recommended Strategy C, one large checkpoint commit, unless the maintainer explicitly accepts edited patch-staging risk.
- Provided fallback Strategy A grouping and manual commands for maintainers who still want split commits.
- Selected `work/goals/goal_wait_for_manual_contract_core_commits_v0_1.md` next.

No implementation, runtime behavior, CLI, storage, Writer, gate writer, proofpack writer, artifact hash runtime, acceptance claim writer, adapter, policy engine, staging, commit, push, PR, or `punk init` behavior was added.
