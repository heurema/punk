---
id: goal_add_active_cli_surface_docs_governance_check
title: "Add active CLI surface docs-governance check"
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
    - "scripts/check_docs_governance.py"
    - "docs/product/**"
    - "README.md"
    - "work/STATUS.md"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Docs-governance catches current/active CLI wording that lists unimplemented commands as implemented or active behavior."
  - "Implemented CLI truth remains limited to `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`."
  - "Future target commands such as `punk init` may still appear when clearly labeled future, target, deferred, or not current."
  - "Regression coverage is added without changing product CLI behavior."
  - "No runtime code, CLI command, schema, `.punk/` state, gate/proof behavior, provider/model/agent adapter, or automation is implemented."
knowledge_refs:
  - "work/reports/2026-04-25-punk-init-docs-cli-mismatch.md"
  - "work/reports/2026-04-25-eighth-work-ledger-review.md"
  - "docs/product/START-HERE.md"
  - "docs/product/CRATE-STATUS.md"
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
  rationale: "This is a bounded docs-governance regression check grounded in the just-fixed active CLI docs mismatch and current implemented CLI behavior."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/product/START-HERE.md"
    - "docs/product/CRATE-STATUS.md"
    - "work/reports/2026-04-25-punk-init-docs-cli-mismatch.md"
    - "work/reports/2026-04-25-eighth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Goal selection metadata only; execution will update governance script/report/status if selected."
---

## Context

The `punk init` docs/CLI mismatch was repaired manually.

Add a docs-governance regression check so current/active docs cannot reintroduce unimplemented CLI commands as active behavior.

## Notes

Keep this as a governance/eval guardrail. Do not implement `punk init`, widen the CLI, write `.punk` state, or add runtime setup behavior.
