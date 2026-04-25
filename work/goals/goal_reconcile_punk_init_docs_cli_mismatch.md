---
id: goal_reconcile_punk_init_docs_cli_mismatch
title: "Reconcile punk init docs/CLI mismatch"
status: ready
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Current implemented CLI surface is documented honestly as `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
  - "`punk init` is not described as currently implemented, active, or required setup."
  - "Docs may keep `punk init` as a future/target/deferred capability only if the wording cannot be read as current behavior."
  - "No CLI command, runtime code, schema, `.punk/` state, gate/proof behavior, provider/model/agent adapter, or automation is implemented."
knowledge_refs:
  - "README.md"
  - "docs/product/START-HERE.md"
  - "docs/product/DOGFOODING.md"
  - "docs/product/ROADMAP.md"
  - "docs/product/CRATE-STATUS.md"
  - "work/reports/2026-04-25-seventh-work-ledger-review.md"
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a docs/product active-surface reconciliation grounded in current canonical docs and observed CLI behavior; no Deep Research is needed."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "README.md"
    - "docs/product/START-HERE.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/ROADMAP.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/2026-04-25-seventh-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; execution will update docs/report/status if selected."
---

## Context

`docs/product/START-HERE.md` currently lists `punk init` under active target, while the implemented CLI surface is only:

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

`punk init` may remain a future target, but docs must not imply it is currently implemented or required setup.

## Notes

Keep this docs-only. Do not add `punk init`, runtime setup, `.punk` state, schemas, gate/proof behavior, adapters, or automation.
