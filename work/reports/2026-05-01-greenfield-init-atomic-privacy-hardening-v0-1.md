---
id: report_2026_05_01_greenfield_init_atomic_privacy_hardening_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_harden_greenfield_init_atomic_and_privacy_v0_1.md
---

# Greenfield Init Atomic Privacy Hardening v0.1

## Summary

- Hardened active greenfield `punk init <project-id>` with full-scaffold preflight before writes.
- Preserved existing modified scaffold files and prevented partial missing scaffold creation on blocked init.
- Changed normal human init output to render `target_root: .` instead of an absolute host path.

## Files Changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_harden_greenfield_init_atomic_and_privacy_v0_1.md`
- `work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md`
- `work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md`

## Atomicity Fix

- `punk-project` now computes a preflight report for every planned scaffold entry before applying any writes.
- Missing entries are reported as `planned` during blocked preflight rather than `created`.
- If any `conflict` or `failed` artifact exists in preflight, init returns `result: blocked` and applies no missing scaffold writes.
- Clean preflight still uses create-new/no-overwrite application, preserving the existing idempotent second init behavior.

## Privacy Output Fix

- `ProjectInitReport::render_human()` now prints `target_root: .`.
- Normal human output no longer includes absolute local project roots such as host usernames or temporary directory paths.

## Tests Added

- `init_conflict_is_noop_for_all_other_artifacts`
- `init_failed_preflight_does_not_leave_partial_scaffold`
- `render_human_redacts_absolute_target_root`
- CLI conflict coverage now checks no partial scaffold files are created.
- Smoke eval coverage now includes `eval_project_init_conflict_is_atomic_noop`.

## Manual Verification

Manual temp conflict verification was run after rebuilding `target/debug/punk`.

Result: blocked output preserved `.punk/memory/STATUS.md`, created no other scaffold files, and printed `target_root: .` without an absolute local path.

## Boundaries Preserved

- Greenfield-only compact `.punk/memory/` init remains the only active setup behavior.
- No root-level `work/`, `knowledge/`, `docs/adr/`, or `publishing/` user-project layout was added.
- No brownfield or grayfield behavior was added.
- No repo scan, AI summary, generated docs, contracts, gate/proof runtime, Writer, agents/adapters, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added.
- `.punk/runtime` and derived `.punk` runtime stores remain inactive.

## Checks Run

- `cargo test -p punk-project init_ -- --nocapture` - PASS.
- `cargo test -p punk-cli init_command_ -- --nocapture` - PASS.
- `cargo test -p punk-eval smoke_report -- --nocapture` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo build -p punk-cli` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- Manual temp-root conflict `~/.local/bin/punk-dev init demo-project` - PASS; output returned `result: blocked`, preserved custom `.punk/memory/STATUS.md`, created no other scaffold files, and rendered `target_root: .`.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-project/src/lib.rs crates/punk-cli/src/main.rs crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_harden_greenfield_init_atomic_and_privacy_v0_1.md work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md --report work/reports/2026-05-01-greenfield-init-atomic-privacy-hardening-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next Recommended Goal

`work/goals/goal_polish_greenfield_init_docs_and_flow_inspect_preview_v0_1.md`

Reason: current-dir init docs/help wording, `flow inspect` preview goal-ref leakage, and any real `.punk/project/` docs/ignore mismatch are smaller UX cleanup items that should stay separate from the atomicity/privacy code slice.

## Doc impact
```yaml
  classification: code-doc
  reason: "Changes active greenfield init operational safety and privacy-safe human output, and records the current crate status."
```
