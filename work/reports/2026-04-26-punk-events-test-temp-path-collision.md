---
id: report_2026_04_26_punk_events_test_temp_path_collision
goal_id: goal_fix_punk_events_test_temp_path_collision
actor: vitaly
created_at: 2026-04-26
kind: implementation
---

## Summary

Fixed a `punk-events` test-only temp path collision discovered while validating the eighteenth Work Ledger Review.

The failure mode:

- `cargo test --workspace` runs `punk-events` tests concurrently;
- two file-backed tests can call the shared temp path helper in the same process and same clock tick;
- the helper previously used only process id plus wall-clock nanoseconds;
- concurrent tests could therefore reuse the same `events.jsonl` path, producing `AlreadyExists` or two-line output failures.

## Research Gate

Classification: R0
Required: no
Rationale:
This is a local test helper reliability fix based on direct test failure evidence. It does not require external research and does not make an architecture decision.
Decision:
Proceed.

## Changed Files

- `crates/punk-events/src/lib.rs`
- `work/goals/goal_fix_punk_events_test_temp_path_collision.md`
- `work/reports/2026-04-26-punk-events-test-temp-path-collision.md`
- `work/STATUS.md`

## Implementation

- Added a test-module `AtomicU64` counter.
- Included the counter in `unique_temp_path()` alongside process id and wall-clock nanoseconds.
- Kept the change test-only; no event log runtime behavior, public API, CLI, schema, `.punk/` storage, gate/proof behavior, or proofpack writer behavior changed.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Test helper behavior changed to remove a validation flake; no canonical product docs or specs changed."
  touched_surfaces:
    - crates/punk-events/src/lib.rs
    - work/goals/goal_fix_punk_events_test_temp_path_collision.md
    - work/reports/2026-04-26-punk-events-test-temp-path-collision.md
    - work/STATUS.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `cargo fmt --all` - PASS
- `cargo check --workspace` - PASS
- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files crates/punk-events/src/lib.rs work/STATUS.md work/goals/goal_run_eighteenth_work_ledger_review.md work/goals/goal_add_artifact_hash_policy_smoke_eval_coverage.md work/goals/goal_fix_punk_events_test_temp_path_collision.md work/reports/2026-04-26-eighteenth-work-ledger-review.md work/reports/2026-04-26-punk-events-test-temp-path-collision.md --report work/reports/2026-04-26-eighteenth-work-ledger-review.md --report work/reports/2026-04-26-punk-events-test-temp-path-collision.md` - PASS, 0 failures, 0 warnings
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no repo-tracked absolute path leaks found
