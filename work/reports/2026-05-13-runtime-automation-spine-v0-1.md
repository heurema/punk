# Runtime Automation Spine v0.1

Date: 2026-05-13
Goal: `work/goals/goal_start_runtime_automation_spine_v0_1.md`
Status: completed
Research Gate: R2, satisfied by repo-tracked product laws, architecture, roadmap, crate-status, project-memory, and work-ledger constraints. No external research used.

## Summary

Started the Runtime Automation Spine with the smallest local-only active-core slice: a bounded `punk-events` writer for flow event drafts.

The writer appends caller-provided event drafts to `.punk/events/flow.jsonl` under an explicit absolute project root with a `.punk/project.toml` marker.

This replaces no manual Community Lab artifacts and does not add external automation.

## Runtime slice selected

- Selected: append-only local event writer activation for flow event evidence.
- Reason: roadmap Phase 1 requires every future transition to write events before external automation can be trusted.
- Active scope: `.punk/events/flow.jsonl` only, created under an explicit initialized project root.
- Not selected: persisted flow state, CLI transition runner, replay, receipts, gate decision writer, proofpack writer runtime storage, adapters, publishing, or PR automation.

## Files changed

- `crates/punk-events/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/STATUS.md`
- `work/goals/goal_start_runtime_automation_spine_v0_1.md`
- `work/reports/2026-05-13-runtime-automation-spine-v0-1.md`

## Implementation notes

- Added `append_local_flow_event`.
- Added `.punk/events/flow.jsonl` constants.
- Required an explicit absolute project root and `.punk/project.toml` marker.
- Created `.punk/events/` only when the project marker is present and artifact refs are safe.
- Rejected absolute, home, URL, backslash, parent-segment, empty, or non-normal artifact refs.
- Rejected event directory and event log path conflicts fail-closed.
- Continued local event sequence ids by counting existing non-empty log lines.
- Kept event records evidence-only with no decision, proof, receipt, or acceptance authority.

## Validation run

Validation was run after implementation. Results:

- `cargo test -p punk-events`: PASS
- `cargo test -p punk-eval`: PASS
- `cargo test -p punk-cli`: PASS
- `cargo fmt --check`: PASS
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo run -q -p punk-cli -- eval run smoke`: PASS
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files crates/punk-events/src/lib.rs crates/punk-eval/src/lib.rs crates/punk-cli/src/main.rs README.md docs/product/START-HERE.md docs/product/CRATE-STATUS.md docs/product/PROJECT-MEMORY.md work/STATUS.md work/goals/goal_start_runtime_automation_spine_v0_1.md work/reports/2026-05-13-runtime-automation-spine-v0-1.md --report work/reports/2026-05-13-runtime-automation-spine-v0-1.md`: PASS
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The first local runtime event writer slice changes current crate behavior, smoke output, and active-vs-parked runtime status."
  touched_surfaces:
    - crates/punk-events/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - crates/punk-cli/src/main.rs
    - README.md
    - docs/product/START-HERE.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PROJECT-MEMORY.md
    - work/STATUS.md
    - work/goals/goal_start_runtime_automation_spine_v0_1.md
    - work/reports/2026-05-13-runtime-automation-spine-v0-1.md
  required_updates:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PROJECT-MEMORY.md
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: `README.md`, `docs/product/START-HERE.md`, `docs/product/CRATE-STATUS.md`, and `docs/product/PROJECT-MEMORY.md`.
- Active runtime scope changed only by adding the narrow local event writer slice.
- Runtime `.punk/events/flow.jsonl` is event evidence, not project truth.
- Gate/proof/module/adapter boundaries are unchanged.
- Only `gate` may write final decisions.
- Executor claims remain non-proof.
- Public narrative and Community Lab artifacts remain advisory/manual and are no longer the product automation path.
- Automated responder remains deferred/external.
- CommunityPunk runtime remains parked/future.
- PubPunk publishing automation remains parked/future.
- DAO/token/funding remain parked/avoided.

## Drift observed

- The working tree already contained unrelated Community Lab/publication draft and receipt artifacts before this patch. They were not reverted or broadened.
- The strategic product path is now runtime-first automation, not manual publication/digest work as the product path.

## Out of scope

- External side effects
- PubPunk runtime
- CommunityPunk runtime
- GitHub, Telegram, Discord, or provider adapters
- Bot runtime
- GitHub API calls
- Automatic issue or PR creation
- Publishing automation
- Raw transcript storage
- Roadmap voting
- DAO, token, or funding behavior
- Cloud sync or SaaS control plane
- Persisted flow state
- Runtime run receipts
- Gate decision writer
- Proofpack runtime storage
- Acceptance claim writer

## Next code slice

Add persisted local flow state with replay from `.punk/events/flow.jsonl`, then make `punk flow inspect` read that state without writing decisions or invoking external side effects.
