---
id: report_2026_05_01_level0_project_init_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_add_level0_project_init_v0_1.md
---

# Level 0 Project Init v0.1

## Summary

- Added a bounded `punk init` CLI command for Dogfooding Level 0 manual project-memory scaffolding.
- The command writes repo-tracked `work/`, `docs/adr/`, and `knowledge/` starter files only.
- Existing files are never overwritten; mismatched files are reported as blocking conflicts.

## Boundary

- No `.punk/` runtime state is created.
- No flow/event persistence is activated.
- No contract writer, run receipt writer, gate writer, proof writer, proofpack writer expansion, or acceptance claim writer is added.
- No provider/model runner, adapter, automation, storage index, or schema writer is added.

## Implementation Notes

- `punk-project` owns the Level 0 init scaffold writer and report model.
- `punk-cli` exposes `punk init` as a thin current-directory wrapper.
- `punk-eval` adds smoke coverage for scaffold creation and no-overwrite conflict behavior.
- Docs-governance active CLI fixtures now treat `punk init` as implemented and use `punk start --from ...` as the unimplemented-command example.

## Checks Run

- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo run -q -p punk-cli -- eval run smoke` - PASS.
- `cargo run -q --manifest-path /Users/vi/personal/heurema/punk/Cargo.toml -p punk-cli -- init` from a temporary empty project root - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files ... --report work/reports/2026-05-01-level0-project-init-v0-1.md` - PASS with 0 failures and 3 existing docs-governance warnings.

## Doc impact

```yaml
  classification: code-doc
  reason: "Adds a bounded active CLI setup surface and updates canonical docs to describe the Level 0 project-memory boundary."
```

- `docs/product/START-HERE.md` now lists `punk init` as active only for Level 0 manual scaffolding.
- `docs/product/CRATE-STATUS.md` now records `punk-project` init behavior and the current CLI surface.
- `docs/product/DOGFOODING.md`, `docs/product/PROJECT-MEMORY.md`, `docs/product/FLOW.md`, and `docs/product/ROADMAP.md` now distinguish Level 0 init from runtime persistence.
- `evals/cases/docs/active-cli-surface/README.md` and fixtures now align docs-governance with the implemented CLI surface.

## Knowledge Impact

- The active project-memory model now has a first executable setup affordance.
- Future runtime storage work must not reinterpret this as `.punk/` initialization; it is only manual scaffold initialization.
- The docs-governance cleanup goal remains ready after this maintainer-selected interruption.
