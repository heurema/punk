---
id: goal_add_open_source_repository_baseline
title: "Add open source repository baseline"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-23
updated_at: 2026-04-23
selected_at: 2026-04-23
started_at: 2026-04-23
completed_at: 2026-04-23
blocked_by: []
scope:
  include:
    - "README.md"
    - "LICENSE"
    - "NOTICE"
    - "CONTRIBUTING.md"
    - "CODE_OF_CONDUCT.md"
    - "SECURITY.md"
    - "SUPPORT.md"
    - "TRADEMARKS.md"
    - "DCO.md"
    - ".github/**"
    - "work/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
acceptance:
  - "The repository declares Apache License 2.0 explicitly."
  - "Repository-level community health files and issue/PR intake exist."
  - "Brand and handoff asset rights are separated from the Apache code/documentation license."
  - "README points to the open-source/community files."
  - "The active `selected_next` work-ledger target remains unchanged after this user-directed scope override."
knowledge_refs:
  - "README.md"
  - "AGENTS.md"
  - "work/STATUS.md"
  - "docs/product/START-HERE.md"
  - "docs/product/PUNK-LAWS.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-23-open-source-repository-baseline.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "This is a bounded repo-governance/docs/legal/config change. It does not change active-core runtime behavior, crate semantics, or future parked capability boundaries."
  research_refs:
    - "AGENTS.md"
    - "work/STATUS.md"
    - "docs/product/START-HERE.md"
    - "docs/product/PUNK-LAWS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "This diff adds repo-front governance/legal/community docs and work-ledger tracking without changing canonical product docs."
---

## Context

The repository needed a first explicit open-source/community baseline: license, contribution rules, conduct/security/support guidance, and GitHub health files.

## Notes

This goal was completed as a user-directed scope override outside the current `selected_next`.

The diff intentionally:
- keeps active-core and parked capability boundaries unchanged;
- avoids Rust code, `.punk/`, and product-doc changes;
- leaves `work/goals/goal_run_fourth_work_ledger_review.md` as the live next bounded step after this repo-governance detour.
