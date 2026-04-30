---
id: goal_migrate_pr_intake_gate_to_repo_governance_v0_1
title: "Migrate PR Intake Gate to repo-governance action v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-30
updated_at: 2026-04-30
selected_at: 2026-04-30
started_at: 2026-04-30
completed_at: 2026-04-30
blocked_by: []
scope:
  include:
    - ".github/workflows/pr-intake-gate.yml"
    - ".github/pr-intake-gate.yml"
    - "CHANGELOG.md"
    - "work/STATUS.md"
    - "work/goals/goal_migrate_pr_intake_gate_to_repo_governance_v0_1.md"
    - "work/reports/2026-04-30-pr-intake-gate-repo-governance-migration-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
    - "scripts/pr_intake_gate.py"
    - "scripts/test_pr_intake_gate.py"
acceptance:
  - "PR Intake Gate workflow uses `heurema/repo-governance/actions/pr-intake-gate@v0.1.0`."
  - "Punk-specific intake policy remains local in `.github/pr-intake-gate.yml`."
  - "Policy includes Punk project metadata used by the shared action output."
  - "Required check name remains `pr-intake-gate`."
  - "No Rust code, `.punk` runtime state, product runtime behavior, gate decision kernel, proofpack behavior, provider, model, agent, marketplace, SaaS, or active CLI behavior changes are made."
knowledge_refs:
  - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
  - "CONTRIBUTING.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/RESEARCH-GATE.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-pr-intake-gate-repo-governance-migration-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "This changes repository governance automation by moving PR Intake Gate execution to a shared action, but does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior."
  research_refs:
    - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
    - "docs/product/RESEARCH-GATE.md"
    - "CONTRIBUTING.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "CHANGELOG.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-pr-intake-gate-repo-governance-migration-v0-1.md"
  rationale: "The diff changes repo-governance automation and work-ledger state without changing canonical product docs or runtime behavior."
---

## Context

The maintainer approved moving the duplicated PR Intake Gate workflow engine into the shared `heurema/repo-governance` repository.

This is a user-directed scope override outside the current `selected_next` proofpack writer advisory review.

## Notes

The migration is intentionally operational and bounded:

- the workflow calls the shared action at `v0.1.0`;
- Punk keeps `.github/pr-intake-gate.yml` as the local source of policy truth;
- required status context remains `pr-intake-gate`;
- existing local scripts are not removed in this patch to avoid mixing shared-action rollout with historical cleanup;
- no Rust code, `.punk`, product docs, runtime gate/proof behavior, provider, model, agent, marketplace, SaaS, or active CLI changes are made.

After this scope override, `work/STATUS.md:selected_next` remains `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
