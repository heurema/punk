---
id: goal_public_site_executor_neutral_copy
title: "Make public site copy executor-neutral"
status: done
owner: "vitaly"
module: "public-narrative"
priority: P1
authority: canonical
created_at: 2026-04-24
updated_at: 2026-04-24
selected_at: 2026-04-24
started_at: 2026-04-24
completed_at: 2026-04-24
blocked_by: []
scope:
  include:
    - "site/src/components/Hero.astro"
    - "site/src/components/HowSection.astro"
    - "site/src/components/ModulesSection.astro"
    - "site/src/data/content.ts"
    - "site/src/layouts/Layout.astro"
    - "work/goals/**"
    - "work/reports/**"
    - "work/STATUS.md"
  exclude:
    - "crates/**"
    - "Cargo.toml"
    - "Cargo.lock"
    - ".punk/**"
    - "docs/product/**"
    - "evals/specs/**"
    - "knowledge/**"
acceptance:
  - "Hero no longer says agents are the only worker."
  - "Lifecycle run label is executor-neutral."
  - "DevPunk copy says bring-your-own executor or equivalent."
  - "Guide example says chosen executor, not agent-only execution."
  - "Public copy still allows agents as a use case without making agents the required architecture."
  - "No runtime code, schema, receipt fields, missing-validator policy, semantic assessor interface, or `.punk/` runtime surface is changed."
knowledge_refs:
  - "docs/product/PUBLIC-NARRATIVE.md"
  - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
  - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-24-public-site-executor-neutral-copy.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "Public copy affects public claims and should be checked against the executor-agnostic validation boundary, but the relevant canonical docs and user-provided review are already available."
  research_refs:
    - "docs/product/PUBLIC-NARRATIVE.md"
    - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
    - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: docs-only
  required_updates: []
  rationale: "The diff changes public site copy to align with existing canonical public narrative and executor-boundary docs; it does not change canonical product docs."
---

## Context

The executor-agnostic validation boundary is now canonical as a proposed ADR/eval-policy boundary. The public site still had a few agent-heavy phrases that could imply agents are the required architectural executor.

## Notes

This goal is complete:

- hero copy now says work happens in the user's runtime;
- lifecycle `run` subtitle now says executor under contract;
- DevPunk copy now says bring-your-own executor;
- guide example now says chosen executor;
- one problem-section phrase, `AI agents are powerful`, remains intentionally because agents remain an important use case, not a required authority surface.

The interrupted `selected_next` remains `work/goals/goal_extract_goalrail_process_shell_pilot.md` after this public-copy correction.
