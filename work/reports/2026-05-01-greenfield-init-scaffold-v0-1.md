---
id: report_2026_05_01_greenfield_init_scaffold_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_add_greenfield_init_scaffold_v0_1.md
---

# Greenfield Init Scaffold v0.1

## Summary

- Changed active init usage to `punk init <project-id>`.
- Added lowercase slug validation for project ids.
- Recorded `project_id` and `entry_mode = greenfield` in the generated Level 0 scaffold.
- Kept `.punk` marker/setup files marker-only and kept all `.punk` runtime stores inactive.

## Boundary

- Active mode is greenfield only.
- Brownfield reconstruction and grayfield reconciliation remain deferred.
- No repo scanning, AI summaries, network behavior, contract generation, gate/proof runtime, Writer behavior, adapters, automation, or acceptance claims are added.
- Existing create-new/no-overwrite behavior still reports mismatched existing files as blocking conflicts.

## Checks Run

- `cargo fmt --all` - PASS.
- `cargo test -p punk-project` - PASS.
- `cargo test -p punk-cli` - PASS.
- `cargo test -p punk-eval` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo run -q -p punk-cli -- eval run smoke` - PASS.
- Manual temp-root init through the repo `punk-cli` binary with `weekend-project` - PASS; recorded `project_id`, `entry_mode = greenfield`, created `work/goals/goal_initial_project_setup.md`, and created 0 `.punk` runtime dirs.
- `cargo build -p punk-cli` - PASS.
- `~/.local/bin/punk-dev init weekend-project` from a temporary empty project root - PASS; used the updated symlinked dev build and created 0 `.punk` runtime dirs.
- `~/.local/bin/punk-dev flow inspect` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files ... --report work/reports/2026-05-01-greenfield-init-scaffold-v0-1.md` - PASS with 0 failures and 3 existing duplicate-definition warnings.

## Doc impact

```yaml
  classification: code-doc
  reason: "Promotes init from generic Level 0 scaffold to explicit greenfield `punk init <project-id>` with project identity and entry-mode metadata."
```

- `README.md`, `docs/product/START-HERE.md`, `docs/product/CRATE-STATUS.md`, `docs/product/DOGFOODING.md`, `docs/product/FLOW.md`, `docs/product/ROADMAP.md`, and `docs/product/PROJECT-MEMORY.md` now describe the greenfield-only init surface.
- `scripts/check_docs_governance.py` and `evals/cases/docs/active-cli-surface/README.md` now allow `punk init <project-id>` as an implemented CLI usage form.

## Knowledge Impact

- New greenfield projects can start with a concrete project id instead of TODO identity fields.
- Future brownfield and grayfield entry modes still need separate research-backed goals and should not reuse greenfield authority semantics.
