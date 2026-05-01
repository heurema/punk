---
id: report_2026_05_01_greenfield_init_compact_memory_layout_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_change_greenfield_init_to_compact_punk_memory_layout_v0_1.md
---

# Greenfield Init Compact Memory Layout v0.1

## Summary

- Changed default user-project init layout from root-level Punk memory dirs to compact `.punk/memory/`.
- Kept `.punk/README.md` and `.punk/project.toml` tracked marker/setup files.
- Added `[memory] layout = "compact"` and inactive `[runtime]` metadata to `.punk/project.toml`.
- Changed `.gitignore` away from blanket `.punk/` ignore toward future runtime/derived dirs only.

## Layout

Before this correction, `punk init <project-id>` created:

```text
work/
knowledge/
docs/adr/
.punk/
```

After this correction, `punk init <project-id>` creates:

```text
.punk/
  README.md
  project.toml
  memory/
    STATUS.md
    goals/
      goal_initial_project_setup.md
    reports/
    knowledge/
      ideas/
      research/
    adr/
```

## Boundary

- `.punk/memory/` is tracked durable project memory.
- `.punk/runtime/` is configured as inactive future runtime root and is not created.
- Root-level `work/`, `knowledge/`, `docs/adr/`, and `publishing/` remain valid for the Punk repository's dogfooding layout, but are no longer default user-project init output.
- Brownfield reconstruction, grayfield reconciliation, repo scanning, AI summaries, network behavior, contracts, gate/proof runtime, Writer behavior, agents, adapters, and replayability runtime remain inactive.

## Checks Run

- `cargo fmt --all` - PASS.
- `cargo fmt --check` - PASS.
- `cargo test -p punk-project` - PASS.
- `cargo test -p punk-cli` - PASS.
- `cargo test -p punk-eval` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo build -p punk-cli` - PASS.
- `~/.local/bin/punk-dev eval run smoke` after rebuild - PASS.
- Manual temp-root `~/.local/bin/punk-dev init demo-project` - PASS; created only `.punk/README.md`, `.punk/project.toml`, and `.punk/memory/**` files, created 0 runtime dirs, and `git status --short --untracked-files=all` showed `.punk/memory/**` files visible to git.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files ... --report work/reports/2026-05-01-greenfield-init-compact-memory-layout-v0-1.md` - PASS with 0 failures and 3 existing duplicate-definition warnings.
- `git diff --check` - PASS.

## Doc impact

```yaml
  classification: code-doc
  reason: "Changes default user-project init layout to compact tracked `.punk/memory/` and documents the split from Punk's root dogfooding layout."
```

- `README.md`, `docs/product/START-HERE.md`, `docs/product/CRATE-STATUS.md`, `docs/product/DOGFOODING.md`, `docs/product/FLOW.md`, `docs/product/ROADMAP.md`, and `docs/product/PROJECT-MEMORY.md` now distinguish the Punk repository dogfooding layout from the default user-project compact layout.

## Knowledge Impact

- Durable Punk memory remains repo-tracked.
- User projects avoid root clutter by default.
- Future runtime and derived state must stay separate from `.punk/memory/`.
