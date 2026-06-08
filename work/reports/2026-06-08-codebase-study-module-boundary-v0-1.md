---
id: report_2026_06_08_codebase_study_module_boundary_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_define_codebase_study_module_boundary_v0_1.md
selected_next: work/goals/goal_verify_codebase_study_module_boundary_v0_1.md
related_docs:
  - docs/modules/codebase-study.md
  - docs/product/MODULES.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
related_evals:
  - evals/specs/codebase-study-module-boundary.v0.1.md
  - evals/specs/brownfield-inventory-boundary.v0.1.md
  - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
related_research:
  - knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md
doc_impact:
  classification: docs-only
  refs:
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
    - work/goals/goal_verify_codebase_study_module_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md
    - docs/modules/codebase-study.md
    - docs/product/MODULES.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/codebase-study-module-boundary.v0.1.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
research_gate:
  classification: R2
  required: true
---

# Codebase Study module boundary v0.1

## Summary

Defined Codebase Study as a parked future Punk module boundary before any
source observation implementation.

Canonical identity:

```text
module_id = codebase-study
prose_name = Codebase Study
docs_path = docs/modules/codebase-study.md
```

The selected Unix-style shape is:

```text
explicit source observation request
  -> codebase-study module
  -> advisory source inventory observation packet
  -> Source Corpus Manifest model handoff
```

## What changed

- Added `docs/modules/codebase-study.md` as the parked module boundary.
- Added `evals/specs/codebase-study-module-boundary.v0.1.md`.
- Updated `docs/product/MODULES.md` and `docs/product/DOCUMENTATION-MAP.md`
  with the new module boundary.
- Added short Brownfield Inventory and Source Corpus Manifest references to
  point future agents to the module boundary instead of scanner behavior.
- Added Brownfield eval guards that name `codebase-study` as the canonical
  module id for this boundary.
- Closed the selected goal and selected
  `work/goals/goal_verify_codebase_study_module_boundary_v0_1.md`.

## Boundary preserved

This slice does not activate:

- source inventory generation;
- repo scanning;
- file walking;
- source content reading;
- source filesystem hashing;
- size collection;
- manifest generation from repository state;
- source indexing;
- claim extraction;
- AI summaries;
- module maps;
- architecture recovery;
- intent recovery;
- contract generation;
- gate/proof runtime;
- Punk `Writer` behavior;
- runtime `.punk` storage;
- CLI behavior;
- module execution;
- lab code import;
- benchmark suite execution;
- benchmark-result authority.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Defines the parked Codebase Study module boundary and eval spec before any source observation implementation."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
    - work/goals/goal_verify_codebase_study_module_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md
    - docs/modules/codebase-study.md
    - docs/product/MODULES.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/codebase-study-module-boundary.v0.1.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
  required_updates:
    - work/STATUS.md
    - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
    - work/goals/goal_verify_codebase_study_module_boundary_v0_1.md
    - work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md
    - docs/modules/codebase-study.md
    - docs/product/MODULES.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/codebase-study-module-boundary.v0.1.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
```

## Verification

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files work/STATUS.md work/goals/goal_define_codebase_study_module_boundary_v0_1.md work/goals/goal_verify_codebase_study_module_boundary_v0_1.md work/reports/2026-06-08-codebase-study-module-boundary-v0-1.md docs/modules/codebase-study.md docs/product/MODULES.md docs/product/DOCUMENTATION-MAP.md docs/product/BROWNFIELD-INVENTORY.md docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md evals/specs/codebase-study-module-boundary.v0.1.md evals/specs/brownfield-inventory-boundary.v0.1.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md` passed with zero warnings.
- `git diff --check` passed.

## Not tested

No Rust tests are required for this docs/eval boundary slice. No Rust code,
runtime behavior, CLI behavior, scanner, file walker, source content reader,
hash collector, manifest generator from repository state, `.punk` state,
module execution, lab runner, or benchmark suite was changed.
