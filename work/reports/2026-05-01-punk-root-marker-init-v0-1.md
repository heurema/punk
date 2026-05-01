---
id: report_2026_05_01_punk_root_marker_init_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_add_punk_root_marker_init_v0_1.md
---

# Punk Root Marker Init v0.1

## Summary

- Extended `punk init` to create `.punk/README.md` and `.punk/project.toml`.
- Kept `.punk` as marker/setup metadata only.
- Runtime stores such as `.punk/events`, `.punk/contracts`, `.punk/runs`, `.punk/evals`, `.punk/decisions`, `.punk/proofs`, `.punk/indexes`, and `.punk/views` remain inactive and are not created.

## Boundary

- `.punk/project.toml` records schema, TODO project identity fields, Dogfooding Level 0, inactive runtime persistence, and `work/STATUS.md` as live state.
- Existing no-overwrite/conflict behavior applies to the `.punk` marker files.
- No flow/event persistence, contracts, receipts, gate artifacts, proofpacks, or acceptance claims are added.

## Checks Run

- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo run -q -p punk-cli -- eval run smoke` - PASS.
- `cargo run -q --manifest-path /Users/vi/personal/heurema/punk/Cargo.toml -p punk-cli -- init` from a temporary empty project root - PASS; created `.punk/README.md` and `.punk/project.toml` and 0 `.punk` runtime dirs.
- `cargo build -p punk-cli` - PASS.
- `~/.local/bin/punk-dev flow inspect` - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files ... --report work/reports/2026-05-01-punk-root-marker-init-v0-1.md` - PASS with 0 failures and 3 existing docs-governance warnings.

## Doc impact

```yaml
  classification: code-doc
  reason: "Adds `.punk` marker/setup files to the Level 0 init scaffold and updates canonical docs to distinguish marker files from inactive runtime stores."
```

- `docs/product/START-HERE.md`, `docs/product/CRATE-STATUS.md`, `docs/product/DOGFOODING.md`, `docs/product/FLOW.md`, `docs/product/ROADMAP.md`, and `docs/product/PROJECT-MEMORY.md` now describe the `.punk` marker boundary.

## Knowledge Impact

- `.punk` is no longer absent after init; it is present only as project marker/setup metadata.
- Future runtime storage work must still be selected separately before creating `.punk/events`, `.punk/contracts`, `.punk/runs`, `.punk/evals`, `.punk/decisions`, `.punk/proofs`, indexes, or views.
