---
id: report_2026_05_15_punk_instruction_page_index_research_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-15
updated_at: 2026-05-15
goal_ref: work/goals/goal_research_punk_instruction_page_index_v0_1.md
---

# Punk Instruction Page Index Research v0.1

## Summary

Captured an advisory PageIndex-style instruction navigation direction for
Punk.

The selected pattern is a local hierarchical instruction page index under
`.punk`: thin entrypoints, focused source pages, module instruction subtrees,
and derived/rebuildable index views.

This is not an implementation patch.

## Files changed

- `knowledge/research/2026-05-15-page-index-instruction-navigation.md`
- `knowledge/ideas/2026-05-15-punk-instruction-page-index.md`
- `work/goals/goal_research_punk_instruction_page_index_v0_1.md`
- `work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md`
- `work/STATUS.md`

## Validation run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files knowledge/research/2026-05-15-page-index-instruction-navigation.md knowledge/ideas/2026-05-15-punk-instruction-page-index.md work/goals/goal_research_punk_instruction_page_index_v0_1.md work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md work/STATUS.md --report work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md` - PASS.
- `git diff --check` - PASS.
- `cargo check --workspace` - not run; no Rust code changed.

## Doc impact

```yaml
doc_impact:
  classification: research-promotion
  reason: "Adds advisory PageIndex-style instruction navigation research and idea artifacts without changing canonical product docs, runtime behavior, or active CLI behavior."
  touched_surfaces:
    - knowledge/research/2026-05-15-page-index-instruction-navigation.md
    - knowledge/ideas/2026-05-15-punk-instruction-page-index.md
    - work/goals/goal_research_punk_instruction_page_index_v0_1.md
    - work/reports/2026-05-15-punk-instruction-page-index-research-v0-1.md
    - work/STATUS.md
  required_updates:
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: None.
- Active runtime scope unchanged.
- Active CLI scope unchanged.
- `.punk` init behavior unchanged.
- Module host remains parked.
- PubPunk remains parked/future.
- PageIndex is recorded as an external reference and design pattern, not as a
  dependency.
- Generated instruction indexes and HTML views are advisory/rebuildable, not
  source of truth.
- Next proposed implementation slice: deterministic instruction page-index
  model plus thin `.punk/instructions/INDEX.md` scaffolding.

## Drift observed

- Existing `punk init` creates `.punk/README.md` and `.punk/project.toml`, but
  it does not yet create a full instruction hierarchy.
- Product docs already define module boundaries and derived `.punk/views/`, so
  a PageIndex-style instruction tree fits existing architecture if it remains
  derived and advisory.

## Out of scope

No Rust code, active CLI command, `punk init` behavior, `.punk` runtime write,
module host behavior, PubPunk activation, adapter behavior, PageIndex
dependency, Python runtime, LLM call, OCR, MCP, vector DB, cloud sync, hidden
truth store, external publication, bot, GitHub API call, PR automation, gate
decision writer, proofpack writer, or acceptance claim behavior was added.
