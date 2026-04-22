---
id: report_2026_04_22_flow_inspect_command
goal_id: goal_add_flow_inspect_command
actor: vitaly
created_at: 2026-04-22
kind: handoff
---

## Goal

Add the smallest honest `punk flow inspect` surface on top of the current flow state and event evidence kernels.

## What changed

- Replaced the Phase 0 CLI placeholder with a minimal command router in `crates/punk-cli/src/main.rs`.
- Added a bounded `punk flow inspect` command that renders a limited kernel preview instead of pretending that persisted runtime state already exists.
- Wired `punk-cli` to `punk-flow` and `punk-events` so the inspect surface can render real flow/event primitives.
- Kept the inspect output explicit about its boundary:
  - runtime persistence is inactive
  - current persisted runtime state is unavailable
  - the command previews allowed and denied transition evidence from the current kernels only
- Added CLI tests proving:
  - the root help points to `punk flow inspect`
  - inspect output stays honest about limited runtime state
  - inspect output includes allowed and denied preview evidence
  - unknown commands fail with narrow usage guidance
- Updated the Work Ledger to:
  - close `goal_add_flow_inspect_command`
  - promote `goal_add_smoke_eval_harness` to the next ready goal

## Research Gate

Classification: R1
Required: yes
Rationale:
This diff touches a public CLI/operator surface and inspectable flow/event views, but stays a narrow implementation over already accepted flow and event kernels.
Research considered:
- `docs/product/START-HERE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `work/reports/2026-04-21-flow-state-kernel.md`
- `work/reports/2026-04-21-append-only-event-log.md`
- `work/reports/2026-04-22-connect-flow-transitions-to-event-log.md`
- `work/reports/2026-04-22-research-gate-preflight.md`
Adopted now:
- expose a first inspect surface only after real flow/event evidence exists;
- keep inspect honest about limited runtime state;
- derived inspect output stays convenience, not truth;
- event evidence remains evidence, not decision authority.
Deferred:
- `.punk/` runtime persistence activation;
- inspect over persisted event files;
- smoke eval harness;
- gate/proof or contract-loop behavior;
- dashboard or product UI.
Decision:
Proceed with a limited kernel-preview `punk flow inspect` and keep `.punk/` runtime activation out of scope.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds the first bounded flow inspect command and updates the Work Ledger to move from inspect to smoke eval work."
  touched_surfaces:
    - project-memory
  required_updates:
    - crates/punk-cli/Cargo.toml
    - crates/punk-cli/src/main.rs
    - Cargo.lock
    - work/STATUS.md
    - work/goals/goal_add_flow_inspect_command.md
    - work/goals/goal_add_smoke_eval_harness.md
    - work/reports/2026-04-22-flow-inspect-command.md
  supersedes: []
  archive_plan: []
  evals_required:
    - research-gate-check
    - work-ledger-checks
    - cargo-test
    - cargo-check
    - docs-governance
```

## Checks run

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo test -p punk-cli`
- `cargo test -p punk-flow`
- `cargo test -p punk-events`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- flow inspect`
- `scripts/check.sh docs-governance --files work/reports/2026-04-22-flow-inspect-command.md --report work/reports/2026-04-22-flow-inspect-command.md`

## Result

- `punk flow inspect` now exists as the first bounded CLI inspect surface.
- The command explicitly reports that runtime persistence is inactive and that it is rendering a limited kernel preview.
- The command shows allowed and denied flow/event evidence without writing `.punk/` state.
- `goal_add_smoke_eval_harness` is now the next ready goal.

## Out of scope confirmation

This diff does not implement:

- `.punk/` runtime activation
- inspect over persisted runtime files
- smoke eval harness behavior
- gate or proof runtime
- module, plugin, adapter, or Knowledge Vault work
- product UI or dashboard projection

## Next recommended goal

`work/goals/goal_add_smoke_eval_harness.md`
