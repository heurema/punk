---
id: goal_add_trusted_author_pr_intake_fast_path_v0_1
title: "Add trusted-author PR Intake Gate fast path v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-29
updated_at: 2026-04-29
selected_at: 2026-04-29
started_at: 2026-04-29
completed_at: 2026-04-29
blocked_by: []
scope:
  include:
    - ".github/pr-intake-gate.yml"
    - ".github/PULL_REQUEST_TEMPLATE.md"
    - "scripts/pr_intake_gate.py"
    - "scripts/test_pr_intake_gate.py"
    - "CONTRIBUTING.md"
    - "CHANGELOG.md"
    - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_trusted_author_pr_intake_fast_path_v0_1.md"
    - "work/reports/2026-04-29-trusted-author-pr-intake-fast-path-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "PR Intake Gate trusts authors with GitHub repository permission `admin`, `maintain`, or `write`."
  - "If permission lookup is unavailable or inconclusive, the gate falls back to `OWNER`, `MEMBER`, and `COLLABORATOR` author associations."
  - "Trusted authors pass intake without external contributor questionnaire or high-risk intake blocking."
  - "External non-trivial PRs require linked intent and required context sections unless `intake/accepted-for-pr` or `maintainer/override-intake` is present."
  - "External high-risk PRs remain blocked until `maintainer/override-intake`."
  - "First-time external contributors receive an advisory `intake/first-time-contributor` label."
  - "Local fixture tests cover permission trust, association fallback, external context failures, accepted-for-pr, first-time signal, and override behavior."
  - "No Rust code, `.punk` runtime state, active CLI behavior, product runtime behavior, gate decision, or proofpack behavior changes are made."
knowledge_refs:
  - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
  - "CONTRIBUTING.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/RESEARCH-GATE.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-29-trusted-author-pr-intake-fast-path-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "This refines the already adopted PR Intake Gate trust policy and touches GitHub repo-governance checks, but does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior."
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
    - ".github/PULL_REQUEST_TEMPLATE.md"
    - "CHANGELOG.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-29-trusted-author-pr-intake-fast-path-v0-1.md"
  rationale: "The diff refines repo-governance automation and contributor-facing process docs without changing canonical product docs or runtime behavior."
---

## Context

The maintainer approved splitting PR intake by trust level:

- trusted repository authors should not be slowed by external contributor intake;
- external contributors should still provide linked intent, context, no-code analysis, and maintainer review for high-risk changes.

This is a user-directed scope override outside the current `selected_next` proofpack writer advisory review.

## Notes

The implementation is intentionally repo-governance only:

- GitHub permission lookup first;
- `author_association` fallback;
- external-only context checks;
- first-time external advisory signal;
- maintainer `intake/accepted-for-pr` and `maintainer/override-intake` controls;
- no Rust code, `.punk`, product docs, runtime gate/proof behavior, provider, model, agent, marketplace, SaaS, or active CLI changes.

After this scope override, `work/STATUS.md:selected_next` remains `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
