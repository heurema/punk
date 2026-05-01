---
id: report_2026_04_30_replayable_project_memory_direction
kind: work-report
status: final
authority: evidence
created_at: 2026-04-30
updated_at: 2026-04-30
goal_ref: work/goals/goal_capture_replayable_project_memory_direction.md
research_gate: R0
external_research: not-used
---

# Replayable Project Memory Direction

## Summary

Captured Replayable Project Memory as an advisory idea and concise product-doc direction.

The direction: Punk should preserve project artifacts so future humans and agents can replay intent, transpose contracts, and verify migrations or reimplementations.

This does not activate deterministic replay, current spec-as-source behavior, automatic migration, code generation, runtime storage, CLI behavior, Writer, gate writers, proof writers, agents, or adapters.

## Files changed

- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `docs/product/NORTH-STAR.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `work/STATUS.md`
- `work/goals/goal_capture_replayable_project_memory_direction.md`
- `work/reports/2026-04-30-replayable-project-memory-direction.md`

## Why this is an idea/direction, not implementation

The artifact records a strategic thesis and vocabulary for future research and planning.

It does not add code, CLI behavior, runtime storage, `.punk/` writes, generation, migration automation, gate writers, proof writers, Writer activation, agents, adapters, or policy engines.

The idea remains advisory until promoted through research, roadmap, goals/contracts, implementation, eval, gate, and proof.

## North Star impact

`docs/product/NORTH-STAR.md` now includes a short `Replayability direction` section.

It states that Punk should preserve project memory so future humans and agents can replay intent, transpose contracts, and verify migrations/reimplementations.

It also explicitly says this does not activate deterministic automatic replay, current spec-as-source behavior, code generation, migration automation, CLI/runtime behavior, agents, adapters, Writer, gate writers, or proof writers.

## Project Memory impact

`docs/product/PROJECT-MEMORY.md` now includes a short `Replayable project memory direction` section.

It says project memory should preserve enough linked intent, contracts, evidence, decisions, examples, invariants, and proof refs to support future intent replay and migration/reimplementation contracts.

It also preserves the authority boundary: derived summaries, future conformance packs, migration packs, and regenerative spec packs must not become truth without review, gate decisions, and proof.

## What remains advisory

- Replayable Project Memory as a strategic direction.
- Replayability ladder levels.
- Contract transposition vocabulary.
- Future conformance/equivalence framing.
- Candidate future artifacts.

## What remains parked/future

- Conformance Pack.
- Migration Contract Pack.
- Regenerative Spec Pack.
- Spec-as-source implementation.
- Automatic code generation.
- Automatic migration.
- Project replay into another language.

## Greenfield/Brownfield/Grayfield relationship

The idea artifact preserves the distinction:

```text
Greenfield with Punk = recorded replay memory
Brownfield without Punk = reconstructed replay candidates
Grayfield = partial recorded memory + partial reconstruction
```

Greenfield projects can record replay memory from day zero. Brownfield projects can only reconstruct replay candidates unless reviewed and promoted. Grayfield projects combine partial recorded memory with partial reconstruction.

## Future research relationship

Future research topic:

```text
Replayable Project Memory / Regenerative Specs / Round-trip Project Reconstruction
```

Research directions recorded:

- characterization testing;
- golden master testing;
- approval testing;
- executable specs;
- property-based testing;
- specification mining;
- invariant inference;
- traceability recovery;
- architecture reconstruction;
- semantic equivalence testing;
- CodeSpeak takeover/mixed mode/intent recovery;
- GitHub Extract/Edit/Apply;
- SpecLang.

## Checks run

- `git status --short --branch` — PASS, with pre-existing dirty/untracked working tree entries still present.
- `python3 scripts/check_research_gate.py` — PASS.
- `python3 scripts/check_work_ledger.py` — PASS.
- `cargo fmt --check` — PASS.
- `cargo check --workspace` — PASS.
- `cargo test -p punk-contract -p punk-eval` — PASS.
- `git diff --check` — PASS.
- `scripts/check.sh docs-governance --files knowledge/ideas/2026-04-30-replayable-project-memory.md docs/product/NORTH-STAR.md docs/product/PROJECT-MEMORY.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md work/STATUS.md work/goals/goal_capture_replayable_project_memory_direction.md work/reports/2026-04-30-replayable-project-memory-direction.md --report work/reports/2026-04-30-replayable-project-memory-direction.md` — PASS with 0 failures and scoped warnings recorded below.

## Drift observed

No runtime, CLI, Writer, gate/proof writer, agent, adapter, migration automation, or spec-as-source behavior was activated.

The selected implementation path was preserved:

```text
work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
```

## Remaining warnings

Docs governance returned 0 failures and 2 warnings:

- `DOC_GLOSSARY_TERM_UNDECLARED` for the existing `Research notes` heading in `docs/product/DOCUMENTATION-MAP.md`.
- `DOC_DUPLICATE_DEFINITION_CANDIDATE` for the existing `Project coherence` heading in `docs/product/PROJECT-MEMORY.md`.

These are low-severity documentation-heading warnings and are tracked in `work/STATUS.md`.

## Next recommended action

Continue with the current selected implementation goal unless the maintainer explicitly pivots to init/project-state discovery:

```text
work/goals/goal_integrate_contract_receipt_requirements_v0_1.md
```

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Capture Replayable Project Memory as a strategic direction and advisory idea without runtime activation."
  touched_surfaces:
    - docs/product/NORTH-STAR.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - knowledge/ideas/2026-04-30-replayable-project-memory.md
    - work/STATUS.md
    - work/goals/goal_capture_replayable_project_memory_direction.md
    - work/reports/2026-04-30-replayable-project-memory-direction.md
  required_updates:
    - docs/product/NORTH-STAR.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
