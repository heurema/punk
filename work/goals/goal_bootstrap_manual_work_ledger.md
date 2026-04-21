---
id: goal_bootstrap_manual_work_ledger
title: "Bootstrap manual Work Ledger discipline"
status: done
owner: "vitaly"
module: "core"
priority: P0
authority: canonical
created_at: 2026-04-21
updated_at: 2026-04-21
selected_at: 2026-04-21
started_at: 2026-04-21
completed_at: 2026-04-21
blocked_by: []
scope:
  include:
    - "work/STATUS.md"
    - "work/_templates/goal.md"
    - "work/goals/**"
    - "work/reports/**"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/CONTRACT-TRACKER.md"
    - "docs/product/DOCUMENTATION-MAP.md"
    - "scripts/check_work_ledger.py"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "site/**"
acceptance:
  - "work/STATUS.md exists as the only live work-state source of truth."
  - "Goals use normalized lifecycle metadata."
  - "selected_next is explicit and points to a ready goal."
  - "Docs describe Level 0 manual Work Ledger discipline."
  - "A Python-stdlib validator passes."
knowledge_refs:
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/DOGFOODING.md"
  - "docs/product/CONTRACT-TRACKER.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-21-manual-work-ledger-bootstrap.md"
decision_refs:
  - "docs/product/CONTRACT-TRACKER.md"
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
doc_impact:
  classification: docs-only
  required_updates:
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/DOGFOODING.md"
    - "docs/product/CONTRACT-TRACKER.md"
  rationale: "Introduces a Level 0 manual Work Ledger discipline before runtime contract tracking exists."
---

## Context

Create a bounded Dogfooding Level 0 live-state layer so the next project step is visible without relying on chat history.

## Notes

This goal is a manual compatibility layer for the future Contract Tracker.

It is not runtime state, not a task-tracker product, and not a `.punk/` implementation.
