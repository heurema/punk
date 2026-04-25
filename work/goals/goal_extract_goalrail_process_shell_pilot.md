---
id: goal_extract_goalrail_process_shell_pilot
title: "Extract GoalRail process shell pilot"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-23
updated_at: 2026-04-25
selected_at: null
started_at: 2026-04-25
completed_at: 2026-04-25
blocked_by: []
scope:
  include:
    - "work/**"
    - "AGENTS.md"
    - ".agents/skills/punk-workflow/**"
  exclude:
    - "crates/**"
    - ".punk/**"
    - "docs/product/**"
    - "README.md"
acceptance:
  - "A bounded process-only pilot extracts the reusable Punk process shell for GoalRail without porting runtime surfaces."
  - "The pilot identifies the minimum reusable set: `AGENTS.md`, workflow discipline, `work/STATUS.md`, goals/reports, Research Gate, Work Ledger Review loop, and evidence-vs-authority rules."
  - "The pilot defines explicit non-goals for `.punk/`, gate/proof runtime, storage, validators, and automation."
  - "No Rust code, `.punk/`, runtime implementation, second live tracker, or product-doc promotion is introduced."
knowledge_refs:
  - "README.md"
  - "docs/product/PROJECT-MEMORY.md"
  - "docs/product/RESEARCH-GATE.md"
  - "work/reports/2026-04-22-fourth-work-ledger-review.md"
  - "work/pilots/goalrail-process-shell.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-goalrail-process-shell-pilot.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R1
  required: true
  rationale: "The pilot should use existing repo-tracked process evidence, but it still needs a bounded scan of canonical process docs and work-ledger artifacts before extracting anything for reuse."
  research_refs:
    - "README.md"
    - "docs/product/PROJECT-MEMORY.md"
    - "docs/product/RESEARCH-GATE.md"
    - "work/reports/2026-04-22-fourth-work-ledger-review.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: none
  required_updates: []
  rationale: "Process-only work artifact and work-ledger update; no canonical product docs are changed."
---

## Context

The fourth Work Ledger Review concluded that Punk's process shell is stable enough for a narrow GoalRail process-only pilot, while runtime closure, storage, and Event Ledger branches should stay deferred. ADR-0012 reconciliation and `proof before acceptance` semantics remain explicit prerequisites for later gate/proof runtime work, not for this pilot.

## Notes

Keep this goal narrow:
- process-only pilot;
- no `.punk/`;
- no runtime gate/proof work;
- no second tracker;
- no product-doc promotion;
- no new source of truth;
- no automation.

## Outcome

Completed as a process-only extraction.

The reusable GoalRail shell is recorded in `work/pilots/goalrail-process-shell.md`.

The handoff report is `work/reports/2026-04-25-goalrail-process-shell-pilot.md`.

No Rust code, CLI behavior, `.punk/` runtime state, schema, gate implementation, proofpack writer, Event Ledger runtime, adapters, modules, automation, second tracker, or product-doc promotion was introduced.
