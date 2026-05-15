---
id: report_2026_05_15_punk_instruction_page_index_scaffold_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-15
updated_at: 2026-05-15
goal_ref: work/goals/goal_add_punk_instruction_page_index_scaffold_v0_1.md
---

# Punk Instruction Page Index Scaffold v0.1

## Summary

Added the first active `.punk` instruction scaffold to `punk init`.

Both greenfield and brownfield init now create thin source instruction pages
under `.punk/instructions/` while preserving the compact `.punk/memory/`
project-memory layout and no-overwrite preflight behavior.

The implementation also adds a deterministic advisory instruction page-index
model for source refs. It does not write generated views.

## Runtime slice selected

- Selected: source instruction scaffold plus side-effect-free advisory
  instruction page-index model.
- Reason: module and publishing instructions need a navigable local source
  surface before generated views, module hosts, or publishing automation.
- Active scope: `.punk/instructions/` source pages created by `punk init`.
- Not selected: `.punk/views/` writer, generated HTML, PageIndex package,
  module host activation, PubPunk, adapters, LLM summarization, vector DB,
  external side effects, gate writer, proofpack writer, or acceptance claims.

## Files changed

- `crates/punk-project/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `crates/punk-cli/src/main.rs`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/STATUS.md`
- `work/goals/goal_add_punk_instruction_page_index_scaffold_v0_1.md`
- `work/reports/2026-05-15-punk-instruction-page-index-scaffold-v0-1.md`

Related advisory research artifacts already present in this diff:

- `knowledge/research/2026-05-15-page-index-instruction-navigation.md`
- `knowledge/ideas/2026-05-15-punk-instruction-page-index.md`
- `work/goals/goal_research_punk_instruction_page_index_v0_1.md`
- `work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md`

## Implementation notes

- Added `.punk/instructions/INDEX.md` and focused source page templates.
- Added `.punk/instructions/pages/` and `.punk/instructions/modules/`
  scaffolding.
- Added `[instructions]` metadata to `.punk/project.toml` with
  `views_active = false`.
- Added a deterministic advisory source-ref page-index model.
- Extended init conflict preflight so instruction path conflicts fail closed
  before partial writes.
- Extended smoke and CLI coverage for instruction files and absent
  `.punk/views/`.

## Validation run

Validation was run after implementation. Results:

- `cargo test -p punk-project`: PASS
- `cargo test -p punk-eval`: PASS
- `cargo test -p punk-cli`: PASS
- `cargo fmt --check`: PASS
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo run -q -p punk-cli -- eval run smoke`: PASS
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files crates/punk-project/src/lib.rs crates/punk-eval/src/lib.rs crates/punk-cli/src/main.rs README.md docs/product/START-HERE.md docs/product/ROADMAP.md docs/product/CRATE-STATUS.md docs/product/ARCHITECTURE.md docs/product/PROJECT-MEMORY.md knowledge/research/2026-05-15-page-index-instruction-navigation.md knowledge/ideas/2026-05-15-punk-instruction-page-index.md work/STATUS.md work/goals/goal_research_punk_instruction_page_index_v0_1.md work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md work/goals/goal_add_punk_instruction_page_index_scaffold_v0_1.md work/reports/2026-05-15-punk-instruction-page-index-scaffold-v0-1.md --report work/reports/2026-05-15-punk-instruction-page-index-scaffold-v0-1.md`: PASS with 4 pre-existing duplicate-definition warnings in `docs/product/ARCHITECTURE.md`.
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The patch changes `punk init` output, smoke coverage, and active-vs-derived instruction status."
  touched_surfaces:
    - crates/punk-project/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - crates/punk-cli/src/main.rs
    - README.md
    - docs/product/START-HERE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/PROJECT-MEMORY.md
    - work/STATUS.md
    - work/goals/goal_add_punk_instruction_page_index_scaffold_v0_1.md
    - work/reports/2026-05-15-punk-instruction-page-index-scaffold-v0-1.md
  required_updates:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/PROJECT-MEMORY.md
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: `README.md`,
  `docs/product/START-HERE.md`, `docs/product/ROADMAP.md`,
  `docs/product/CRATE-STATUS.md`, `docs/product/ARCHITECTURE.md`, and
  `docs/product/PROJECT-MEMORY.md`.
- Active init scope now includes thin `.punk/instructions/` source pages.
- Active runtime scope unchanged.
- Generated instruction views remain inactive and rebuildable.
- `.punk/views/instructions/page-index.json` is a future advisory view path,
  not source of truth.
- PageIndex remains an external reference and design pattern, not a dependency.
- Module host remains parked.
- PubPunk publishing automation remains parked/future.
- Provider adapters, bots, GitHub API behavior, and PR automation remain
  inactive.
- Only `gate` may write final decisions.

## Drift observed

- The repo had already recorded the advisory PageIndex-style instruction
  direction, but active init did not yet create source instruction pages.
- `.punk/views/` is documented as derived future state, so this patch keeps
  generated views absent until a separate bounded writer goal promotes them.

## Out of scope

- PageIndex dependency
- Python runtime
- LLM call or summarization
- OCR
- MCP
- Vector DB or embeddings as truth
- Generated HTML app
- `.punk/views/` writer
- Module host activation
- PubPunk runtime or publishing automation
- CommunityPunk runtime
- Provider adapters
- GitHub, Telegram, Discord, or bot behavior
- GitHub API calls
- Automatic issue or PR creation
- Raw transcript storage
- Gate decision writer
- Proofpack writer expansion
- Acceptance claim writer

## Next code slice

Add a local instruction-view generator only after a separate goal defines the
write policy for rebuildable `.punk/views/instructions/page-index.json`.

For zero-manual publishing work, the more important product slice remains a
future PubPunk/module path after local runtime state, receipts, policy, and
gate/proof integration are in place.
