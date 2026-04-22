---
id: goal_add_smoke_eval_report_artifact_shape
title: "Add smoke eval report artifact shape"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-22
updated_at: 2026-04-22
selected_at: null
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "crates/punk-cli/**"
    - "crates/punk-eval/**"
    - "evals/**"
    - "work/**"
  exclude:
    - ".punk/**"
    - "crates/punk-gate/**"
    - "crates/punk-proof/**"
acceptance:
  - "A bounded smoke-eval report artifact shape exists for the local smoke CLI run."
  - "The artifact shape stays local-first and does not require `.punk/evals` runtime storage."
  - "No baseline, waiver, or remote eval platform behavior is introduced."
  - "`cargo check --workspace` passes."
knowledge_refs:
  - "docs/product/EVAL.md"
  - "docs/product/EVAL-PLANE.md"
  - "docs/product/ROADMAP.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "Adds a bounded eval report surface after the local smoke CLI wrapper without activating runtime persistence or richer eval platform behavior."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/EVAL-PLANE.md"
    - "docs/product/EVAL.md"
  external_research_refs:
    - "work/reports/2026-04-22-smoke-eval-harness.md"
    - "work/reports/2026-04-22-work-ledger-review.md"
    - "work/reports/2026-04-22-work-ledger-review-loop.md"
    - "work/reports/2026-04-22-smoke-eval-cli-command.md"
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: ""
---

## Context

After the first honest `punk eval run smoke` command exists, define one bounded report artifact shape before any runtime storage, baseline, or waiver work.

## Notes

Keep this goal narrow:
- no `.punk/evals` persistence
- no baseline or waiver discipline
- no remote eval platform
- no gate/proof integration
