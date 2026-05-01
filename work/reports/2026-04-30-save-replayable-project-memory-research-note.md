---
id: report_2026_04_30_save_replayable_project_memory_research_note
kind: work-report
status: final
authority: evidence
created_at: 2026-04-30
updated_at: 2026-05-01
goal_ref: work/goals/goal_save_replayable_project_memory_research_note.md
research_gate: R3
external_research: user-provided-deep-research-report
---

# Save Replayable Project Memory Research Note

## Summary

Saved the user-provided Replayable Project Memory Deep Research report as an advisory repo-tracked research note.

The research note is framed as advisory project memory only. It preserves the core thesis:

```text
Punk should preserve replayable obligations, not replayable code.
```

## Files changed

- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `work/STATUS.md`
- `work/goals/goal_save_replayable_project_memory_research_note.md`
- `work/reports/2026-04-30-save-replayable-project-memory-research-note.md`

## Research status and authority

The saved research note is advisory research, not product truth.

It does not activate runtime behavior, CLI behavior, Writer behavior, gate/proof writers, migration automation, code generation, agent execution, adapters, Conformance Pack runtime, Migration Contract Pack runtime, Regenerative Spec Pack behavior, or spec-as-source behavior.

Promotion still requires:

```text
research note
  -> ADR / roadmap decision
  -> goal / contract
  -> implementation
  -> eval
  -> gate / proof
  -> project-memory update
```

## Source reference hygiene

The source report contains Deep Research-local citation markers such as `turn18view0`, `turn35view1`, and `turn27search4`.

The research note now warns that those markers are not durable repo source refs by themselves.

Before ADR, roadmap promotion, contract, implementation, or eval-policy use, cited sources should be materialized into durable source refs: source URLs, official repository URLs, DOI/arXiv URLs, archived source paths, access dates, and short source ids.

## Changes made to the research note

- Added required research-note frontmatter.
- Added advisory authority and promotion-path notice near the top.
- Added source-reference hygiene after the source quality table.
- Replaced risky `active-core now` wording with `adopt-now discipline` in the preservation table.
- Added a status wording note clarifying that `adopt-now discipline` is design discipline, not runtime/CLI/storage behavior.
- Added Punk integration guardrails.
- Added Conformance Pack boundary.
- Added Contract Transposition boundary.
- Added bug-vs-requirement boundary.
- Added Decision Log candidate.
- Added recommended next artifacts.

## Changes made to the idea artifact

- Added link to `knowledge/research/2026-04-30-replayable-project-memory.md`.
- Added the refined thesis: `Punk should preserve replayable obligations, not replayable code.`
- Added refined adoption map.
- Replaced `active-core now` wording in replayability status rows with `adopt-now discipline` where it could be misread as active runtime behavior.

## Backlog artifact

Created `knowledge/ideas/2026-04-30-replayability-backlog.md` as an advisory backlog with defer, park, and avoid-now sections.

## Docs impact

Updated `docs/product/DOCUMENTATION-MAP.md` to register the new research note/backlog as supporting advisory knowledge refs.

Updated `docs/product/GLOSSARY.md` to link the advisory research note, sharpen boundary wording for Contract Transposition and Conformance Pack, and add Migration Contract Pack.

`docs/product/NORTH-STAR.md` and `docs/product/PROJECT-MEMORY.md` were inspected and left unchanged because their current wording already states that replayability direction does not activate runtime, CLI, Writer, gate/proof writers, agents, adapters, migration automation, code generation, or spec-as-source behavior.

## What remains advisory

- Replayable Project Memory as a strategic concept.
- Contract Transposition vocabulary.
- Conformance Pack concept.
- Migration Contract Pack concept.
- Regenerative Spec Pack concept.
- Recommended future ADR/eval/goal artifacts.

## What remains deferred, parked, or avoided

Deferred:

- Conformance Pack boundary;
- Migration Contract Pack;
- accepted-difference model;
- bug-vs-requirement classification;
- conformance/equivalence evals.

Parked:

- Regenerative Spec Pack;
- full-project replay;
- spec-as-source authority;
- automatic code generation;
- automatic cross-stack regeneration.

Avoided now:

- deterministic replay promises;
- generated specs as canonical truth;
- Conformance Pack as gate decision;
- preserving bugs as requirements;
- remote analysis by default.

## Active implementation path

The active implementation path was not changed.

Selected next remains:

```text
work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
```

## Checks run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS; selected next remained `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md`.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test -p punk-contract -p punk-eval` - PASS; 38 `punk-contract` tests and 6 `punk-eval` tests passed.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files knowledge/research/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayability-backlog.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md work/STATUS.md work/goals/goal_save_replayable_project_memory_research_note.md work/reports/2026-04-30-save-replayable-project-memory-research-note.md --report work/reports/2026-04-30-save-replayable-project-memory-research-note.md` - PASS with 0 failures and 1 warning.

## Drift observed

No runtime, CLI, Writer, gate/proof writer, agent execution, adapter, migration automation, code generation, Conformance Pack runtime, Migration Contract Pack runtime, Regenerative Spec behavior, or spec-as-source behavior was activated.

Pre-existing uncommitted working tree changes were present before this task and were not reverted.

## Remaining warnings

Docs governance returned one warning for the existing `Research notes` heading in `docs/product/DOCUMENTATION-MAP.md`:

```text
DOC_GLOSSARY_TERM_UNDECLARED
```

This warning existed before this task and remains a bounded docs-governance cleanup candidate. It did not block the check.

## Next recommended action

Continue with the current selected implementation goal:

```text
work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
```

## Doc impact

```yaml
doc_impact:
  classification: research-promotion
  reason: "Save Replayable Project Memory deep research as advisory project memory and add Punk integration guardrails without runtime activation."
  touched_surfaces:
    - knowledge/research/2026-04-30-replayable-project-memory.md
    - knowledge/ideas/2026-04-30-replayable-project-memory.md
    - knowledge/ideas/2026-04-30-replayability-backlog.md
  required_updates:
    - docs/product/DOCUMENTATION-MAP.md if new advisory knowledge owners or idea refs must be registered
    - docs/product/GLOSSARY.md only if term definitions need refinement
    - docs/product/NORTH-STAR.md only if current replayability wording overclaims
    - docs/product/PROJECT-MEMORY.md only if current replayability wording overclaims
  supersedes: []
  archive_plan: []
  evals_required:
    - "Future: evals/specs/replayability-conformance-pack.v0.1.md"
```
