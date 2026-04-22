---
id: report_2026_04_22_connect_flow_transitions_to_event_log
goal_id: goal_connect_flow_transitions_to_event_log
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Goal

Connect pure flow transitions to the append-only event log without activating `.punk/` runtime paths, CLI inspect behavior, or smoke eval.

## What changed

- Extended `punk-flow` with transition-attempt facts that describe:
  - the current state
  - the attempted command
  - whether the transition was applied or denied
  - the next state or denial evidence needed for audit
- Kept the existing `transition()` API intact by making it reuse the new transition-attempt facts.
- Kept `punk-events` producer-neutral and added `punk-flow::transition_attempt_event_draft(...)` so flow transition facts can become event drafts without making the event log an authority surface.
- Added tests proving:
  - allowed transitions yield applied event evidence
  - denied transitions yield guard-denial event evidence
  - denied transitions do not mutate flow state
  - no decision or gate naming appears in the denied event kind
- Updated the Work Ledger to:
  - mark `goal_connect_flow_transitions_to_event_log` done
  - promote `goal_add_flow_inspect_command` as the next ready goal
  - keep smoke eval blocked until the first inspect surface exists

## Research notes considered

## Research Gate

Classification: R1
Required: yes
Rationale:
This diff touches flow semantics and event evidence boundaries.
Research considered:
- user-provided deep research summary about audit and event logs, manual approval workflows, OPA-style decision logs, and safe agent outputs.
Adopted now:
- event logs are evidence, not authority;
- allowed transitions produce event evidence;
- denied transitions produce guard-denial evidence;
- denied transitions do not mutate state;
- derived views must not become source of truth.
Deferred:
- manual approval pilot;
- workflow contract schema;
- decision record schema;
- OPA-style policy decision logs;
- dashboard projection;
- Structured Outputs or LLM extraction;
- Research Gate workflow automation.
Decision:
Proceed with this bounded glue diff. Add Research Gate workflow automation as a separate follow-up diff.

This diff intentionally adopts only the event evidence boundary from audit/manual-approval research.

Adopted now:
- audit and event records are evidence, not authority;
- denied transitions leave inspectable guard-denial evidence;
- derived views must not become source of truth.

Deferred:
- manual approval workflow pilot;
- workflow contract schema;
- decision record schema;
- OPA-style policy decision metadata;
- dashboard projection;
- Structured Outputs or LLM extraction.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Connects pure flow transition facts to append-only event drafts and updates the Work Ledger to move from event kernel work to the first inspect surface."
  touched_surfaces:
    - project-memory
  required_updates:
    - Cargo.lock
    - crates/punk-flow/src/lib.rs
    - crates/punk-flow/Cargo.toml
    - work/STATUS.md
    - work/goals/goal_connect_flow_transitions_to_event_log.md
    - work/goals/goal_add_flow_inspect_command.md
    - work/goals/goal_add_smoke_eval_harness.md
    - work/reports/2026-04-22-connect-flow-transitions-to-event-log.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo-test
    - cargo-check
    - work-ledger-checks
    - docs-governance
```

## Checks run

- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo check --workspace`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-connect-flow-transitions-to-event-log.md --report work/reports/2026-04-22-connect-flow-transitions-to-event-log.md`

## Result

- Allowed and denied transitions can be expressed as event drafts.
- Denied transitions leave guard-denial evidence without mutating flow state.
- Event drafts remain evidence only and do not claim decision authority.
- `cargo check --workspace` passes.
- No `.punk/` writes occur.
- No CLI inspect, smoke eval, gate, proof, or contract-loop behavior is introduced.

## Out of scope confirmation

This diff does not implement:

- `.punk/` event persistence activation
- `punk flow inspect` itself
- smoke eval harness behavior
- gate or proof runtime
- manual approval workflow pilot
- OPA, dashboards, or provider integration

## Next recommended goal

`work/goals/goal_add_flow_inspect_command.md`
