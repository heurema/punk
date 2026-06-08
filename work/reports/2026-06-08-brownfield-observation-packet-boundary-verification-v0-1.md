---
id: report_2026_06_08_brownfield_observation_packet_boundary_verification_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
selected_next: work/goals/goal_define_codebase_study_module_boundary_v0_1.md
related_goals:
  - work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md
  - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
related_docs:
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
related_evals:
  - evals/specs/brownfield-inventory-boundary.v0.1.md
  - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
related_research:
  - knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md
doc_impact:
  classification: docs-only
  refs:
    - work/STATUS.md
    - work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
    - work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
research_gate:
  classification: R1
  required: true
---

# Brownfield observation packet boundary verification v0.1

## Summary

Verified the Brownfield source inventory observation packet boundary and
captured the maintainer clarification that codebase study should be a separate
Unix-style Punk module.

The verified flow is:

```text
explicit source observation request
  -> future codebase-study Punk module
  -> advisory source inventory observation packet
  -> Source Corpus Manifest model handoff
```

## Verification result

The packet boundary remains docs/eval guidance only.

It stays:

```text
packet_status = advisory
authority = observed_structure
```

The packet cannot be treated as project truth, a manifest, scanner result,
gate/proof authority, acceptance, benchmark authority, or Source Corpus
Manifest writer input.

## Boundary refinement

Updated the Brownfield inventory and Source Corpus Manifest docs/evals to
record that future codebase study belongs to a separate Punk module, not
Brownfield core, `punk-project`, Source Corpus Manifest writer behavior, or
generic Punk `Writer` behavior.

The future module may return advisory observation packets only. It must not
own final Brownfield decisions, contract approval, gate decisions, proof,
acceptance, runtime `.punk` storage, writer behavior, or broad Punk
orchestration.

The manifest model track may consume an accepted-for-handoff packet only. It
must not run the module, scan the repository, walk directories, read source
contents, compute hashes, collect sizes, or infer observations itself.

## Selected next

Selected:

```text
work/goals/goal_define_codebase_study_module_boundary_v0_1.md
```

This next step defines the module boundary first. It does not implement the
module.

## Boundary preserved

This verification slice does not activate:

- source inventory generation;
- repo scanning;
- file walking;
- source content reading;
- source filesystem hashing;
- size collection;
- manifest generation from repository state;
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
  reason: "Verification found a maintainer clarification that codebase study should be a separate Unix-style Punk module, so the Brownfield packet docs/evals were narrowed before selecting the next module-boundary goal."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
    - work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
  required_updates:
    - work/STATUS.md
    - work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/goals/goal_define_codebase_study_module_boundary_v0_1.md
    - work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
```

## Verification

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files work/STATUS.md work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md work/goals/goal_define_codebase_study_module_boundary_v0_1.md work/reports/2026-06-08-brownfield-observation-packet-boundary-verification-v0-1.md docs/product/BROWNFIELD-INVENTORY.md docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md evals/specs/brownfield-inventory-boundary.v0.1.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md` passed with zero warnings.
- `git diff --check` passed.

## Not tested

No Rust tests are required for this docs/eval/work-ledger verification slice.
No Rust code, runtime behavior, CLI behavior, scanner, file walker, source
content reader, hash collector, manifest generator from repository state,
`.punk` state, module execution, lab runner, or benchmark suite was changed.
