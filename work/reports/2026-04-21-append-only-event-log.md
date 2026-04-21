---
id: report_2026_04_21_append_only_event_log
goal_id: goal_add_append_only_event_log
actor: vitaly
created_at: 2026-04-21
kind: handoff
---

## Goal

Add the minimal append-only event log kernel after the pure flow state kernel.

## What changed

- Replaced the compile-only `punk-events` skeleton with:
  - a minimal `punk.event.v0.1` schema fixture
  - `EventDraft` and `EventRecord` types
  - a generic append-only event log writer over `std::io::Write`
  - a file-backed `create_new_file_event_log` helper that rejects existing paths instead of truncating them
  - deterministic JSONL serialization with stdlib only
- Added tests for:
  - stable sequence and event ids
  - append-only behavior preserving prior events
  - deterministic serialized evidence
  - deterministic escaping of quotes, backslashes, and newlines
  - file-backed evidence creation in temp paths only
  - rejecting existing file paths instead of overwriting them
- Updated the Work Ledger state to:
  - mark `goal_add_append_only_event_log` done
  - add a new glue goal for connecting flow transitions to the event log
  - keep inspect and smoke eval blocked until real flow-to-event evidence exists

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds the first append-only event log kernel and updates the Work Ledger to insert a flow-to-event integration step before inspect or eval work."
  touched_surfaces:
    - project-memory
  required_updates:
    - crates/punk-events/src/lib.rs
    - work/STATUS.md
    - work/goals/goal_add_append_only_event_log.md
    - work/goals/goal_connect_flow_transitions_to_event_log.md
    - work/goals/goal_add_flow_inspect_command.md
    - work/goals/goal_add_smoke_eval_harness.md
    - work/reports/2026-04-21-append-only-event-log.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo-test
    - cargo-check
    - work-ledger-checks
    - docs-governance
```

## Checks run

- `cargo test -p punk-events`
- `cargo check --workspace`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files work/reports/2026-04-21-append-only-event-log.md --report work/reports/2026-04-21-append-only-event-log.md`

## Result

- Event schema fixture exists.
- Append-only writer appends without mutating prior entries.
- Event writer leaves deterministic test evidence.
- `cargo check --workspace` passes.
- No `.punk/` writes occur.
- No CLI, eval, gate, proof, or contract-loop behavior is introduced.

## Out of scope confirmation

This diff does not implement:

- flow transitions emitting events yet
- `punk flow inspect`
- eval smoke harness behavior
- gate or proof runtime
- `.punk/` runtime activation beyond temp-path tests
- JSONL schema expansion beyond the minimal kernel fields

## Deferred

- reopening existing event logs for long-running runtime append
- canonical `.punk/events/*.jsonl` path activation
- broader schema hardening beyond the minimal kernel fields
- hash chain or event root support
- flow-to-event integration
- `punk flow inspect`
- smoke eval harness

## Next recommended goal

`work/goals/goal_connect_flow_transitions_to_event_log.md`
