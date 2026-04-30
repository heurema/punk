---
id: goal_remove_legacy_pr_intake_gate_local_engine_v0_1
title: "Remove legacy local PR Intake Gate engine v0.1"
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
    - "scripts/check.sh"
    - "scripts/pr_intake_gate.py"
    - "scripts/test_pr_intake_gate.py"
    - "CONTRIBUTING.md"
    - "CHANGELOG.md"
    - "work/STATUS.md"
    - "work/goals/goal_remove_legacy_pr_intake_gate_local_engine_v0_1.md"
    - "work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
    - ".github/pr-intake-gate.yml"
    - ".github/workflows/pr-intake-gate.yml"
acceptance:
  - "Legacy local PR Intake Gate script and fixture test are removed from `scripts/`."
  - "`scripts/check.sh` no longer exposes a broken `pr-intake-gate` target that would call deleted local files."
  - "Contributor docs point to the shared `heurema/repo-governance` action, Punk-local policy, and PR check verification path."
  - "Shared workflow and Punk-local policy remain unchanged."
  - "No Rust code, `.punk` runtime state, product runtime behavior, gate decision kernel, proofpack behavior, provider, model, agent, marketplace, SaaS, or active CLI behavior changes are made."
knowledge_refs:
  - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
  - "CONTRIBUTING.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/RESEARCH-GATE.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "This removes stale repository-governance automation code after moving PR Intake Gate execution to a shared action, but does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior."
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
    - "CONTRIBUTING.md"
    - "CHANGELOG.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md"
  rationale: "The diff removes obsolete repo-governance automation and updates work-ledger/docs pointers without changing canonical product docs or runtime behavior."
---

## Context

The maintainer approved immediate cleanup of the local PR Intake Gate engine after the shared `heurema/repo-governance` action rollout.

This is a user-directed scope override outside the current `selected_next` proofpack writer advisory review.

## Notes

The cleanup is intentionally operational and bounded:

- Punk keeps `.github/pr-intake-gate.yml` as the local source of policy truth;
- the workflow still calls `heurema/repo-governance/actions/pr-intake-gate@v0.1.0`;
- required status context remains `pr-intake-gate`;
- local `scripts/check.sh pr-intake-gate` is removed because the reusable engine tests now live in `heurema/repo-governance`;
- no Rust code, `.punk`, product docs, runtime gate/proof behavior, provider, model, agent, marketplace, SaaS, or active CLI changes are made.

After this scope override, `work/STATUS.md:selected_next` remains `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
