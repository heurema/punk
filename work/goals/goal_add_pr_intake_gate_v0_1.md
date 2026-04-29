---
id: goal_add_pr_intake_gate_v0_1
title: "Add deterministic PR Intake Gate v0.1"
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
    - ".github/workflows/pr-intake-gate.yml"
    - ".github/PULL_REQUEST_TEMPLATE.md"
    - "scripts/pr_intake_gate.py"
    - "scripts/test_pr_intake_gate.py"
    - "scripts/check.sh"
    - "CONTRIBUTING.md"
    - "CHANGELOG.md"
    - ".gitignore"
    - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
    - "work/STATUS.md"
    - "work/goals/goal_add_pr_intake_gate_v0_1.md"
    - "work/reports/2026-04-29-pr-intake-gate-v0-1.md"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "A deterministic PR Intake Gate workflow runs from trusted base-branch code and does not checkout or execute PR head code."
  - "The gate labels PRs as `intake/pass`, `intake/needs-linked-intent`, or `intake/high-risk`, with `maintainer/override-intake` as explicit maintainer bypass."
  - "Non-trivial PRs require linked Issue, Discussion, or Punk repo-tracked intent artifact."
  - "High-risk Punk surfaces require maintainer attention before ordinary code review."
  - "The gate has local dry-run fixture tests wired through `scripts/check.sh pr-intake-gate`."
  - "Contributor docs, PR template, changelog, research note, report, and work ledger are updated."
  - "No Rust code, `.punk` runtime state, active CLI behavior, product runtime behavior, gate decision, or proofpack behavior changes are made."
knowledge_refs:
  - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
  - "CONTRIBUTING.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
  - "docs/product/RESEARCH-GATE.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-29-pr-intake-gate-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "This adopts a bounded PR intake mechanism from Signum and touches GitHub workflow/repo-governance trust surfaces, but does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior."
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
    - ".gitignore"
    - "work/STATUS.md"
    - "work/reports/2026-04-29-pr-intake-gate-v0-1.md"
  rationale: "The diff adds repo-governance automation and contributor-facing process docs without changing canonical product docs or runtime behavior."
---

## Context

The maintainer asked to bring Signum-style PR intake into Punk, with improvements where useful.

This is a user-directed scope override outside the current `selected_next` proofpack writer advisory review.

## Notes

The implementation is intentionally repo-governance only:

- trusted-base `pull_request_target` workflow;
- config-driven path/label policy;
- stdlib Python gate script;
- fixture-backed dry-run tests;
- Punk-native linked intent refs;
- no Rust code, `.punk`, product docs, runtime gate/proof behavior, provider, model, agent, marketplace, SaaS, or active CLI changes.

After this scope override, `work/STATUS.md:selected_next` remains `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
