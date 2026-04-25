---
id: goal_public_site_executor_neutral_problem_copy
title: "Tighten public site executor-neutral problem copy"
status: done
owner: "vitaly"
module: "public-narrative"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: null
started_at: 2026-04-25
completed_at: 2026-04-25
blocked_by: []
scope:
  include:
    - "site/src/components/Problem.astro"
    - "work/goals/goal_public_site_executor_neutral_problem_copy.md"
    - "work/reports/2026-04-25-public-site-executor-neutral-problem-copy.md"
    - "work/STATUS.md"
  exclude:
    - "crates/**"
    - "Cargo.toml"
    - "Cargo.lock"
    - ".github/**"
    - ".punk/**"
    - "docs/product/**"
    - "evals/specs/**"
    - "knowledge/**"
acceptance:
  - "The active landing problem section no longer frames agents as the primary required worker."
  - "The problem section uses executor-neutral language aligned with the executor-agnostic validation boundary."
  - "The problem checklist avoids implying active automated gate behavior beyond public-copy scope."
  - "Historical journal entries remain unchanged."
  - "No runtime, code schema, CLI, provider adapter, prompt manager, skill manager, or `.punk/` runtime state is changed."
knowledge_refs:
  - "docs/product/PUBLIC-NARRATIVE.md"
  - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
  - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
  - "work/reports/2026-04-24-public-site-executor-neutral-copy.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-public-site-executor-neutral-problem-copy.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "Public copy affects trust claims, but this is a bounded follow-up using existing canonical public narrative and executor-boundary docs."
  research_refs:
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
    - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
    - "work/reports/2026-04-24-public-site-executor-neutral-copy.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: public-claim
  required_updates: []
  rationale: "The diff changes public landing copy to align with existing executor-neutral public narrative; it does not change canonical product docs."
---

## Context

The prior public-site correction made the hero, lifecycle, DevPunk copy, guide example, title, and receipt mock executor-neutral. It intentionally left one problem-section phrase, `AI agents are powerful`, because agents remain a valid use case.

After adopting advisory executor discipline artifacts, the active landing page should avoid agent-first wording where executor-neutral wording is equally clear.

## Notes

This goal is a small public-copy follow-up only:

- no runtime behavior;
- no schema changes;
- no CLI changes;
- no provider or model integration;
- no prompt or skill management;
- no historical journal rewrite.

`selected_next` remains `work/goals/goal_extract_goalrail_process_shell_pilot.md` after this user-requested public-copy correction.
