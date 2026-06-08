---
id: report_2026_06_08_brownfield_source_inventory_observation_packet_boundary_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md
selected_next: work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
related_docs:
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
  - docs/product/CRATE-STATUS.md
related_evals:
  - evals/specs/brownfield-inventory-boundary.v0.1.md
  - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
related_research:
  - knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md
doc_impact:
  classification: docs-only
  refs:
    - work/STATUS.md
    - work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/reports/2026-06-08-brownfield-source-inventory-observation-packet-boundary-v0-1.md
    - knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
research_gate:
  classification: R2
  required: true
---

# Brownfield source inventory observation packet boundary v0.1

## Summary

Defined the docs/eval boundary for a future Brownfield source inventory
observation packet before any active observer, scanner, file walker, content
reader, hash collector, size collector, or manifest generator from repository
state is selected.

The packet remains:

```text
packet_status = advisory
authority = observed_structure
```

## What changed

- Added the source inventory observation packet boundary to
  `docs/product/BROWNFIELD-INVENTORY.md`.
- Added the packet handoff boundary to the Source Corpus Manifest model track
  in `docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md`.
- Extended Brownfield inventory and manifest eval specs with packet, handoff,
  lab-advisory, and evaluation-route cases.
- Updated documentation map, glossary, and crate-status current truth.
- Recorded the read-only advisory lab pass in
  `knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md`.
- Closed the selected goal and selected
  `work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md`
  as the next checkpoint.

## Lab pass

`code-intel-kernel` contributed advisory design ideas only:

- explicit selectors or observed refs;
- evidence ids;
- warnings, limitations, and missing-evidence fields;
- deterministic but non-semantic ordering;
- no edit localization, root-cause, intent, or patch recommendation authority.

`agent-bench-lab` contributed advisory evaluation requirements only:

- frozen task/config inputs;
- repeated runs for reliability;
- run-validity checks before score use;
- separate success, score, cost, latency, tool-call, failed-tool-call, policy,
  hidden-check, and mutation-check reporting;
- invalid runs are not averaged, ranked, or promoted.

Observed local lab checkout state was not clean during this pass, so future
reuse of lab results, fixtures, code, or benchmark claims must re-verify the
current checkout state.

## Boundary preserved

This slice does not activate:

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
- grayfield reconciliation;
- Conformance Pack runtime;
- Migration Contract runtime;
- Regenerative Spec behavior;
- spec-as-source behavior;
- lab code import;
- benchmark suite execution;
- benchmark-result authority.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Defines a docs/eval observation packet boundary and work-ledger checkpoint before any Brownfield source inventory implementation."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/reports/2026-06-08-brownfield-source-inventory-observation-packet-boundary-v0-1.md
    - knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
  required_updates:
    - work/STATUS.md
    - work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md
    - work/reports/2026-06-08-brownfield-source-inventory-observation-packet-boundary-v0-1.md
    - knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md
    - docs/product/BROWNFIELD-INVENTORY.md
    - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/brownfield-inventory-boundary.v0.1.md
    - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
```

## Verification

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files work/STATUS.md work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md work/goals/goal_verify_brownfield_source_inventory_observation_packet_boundary_v0_1.md work/reports/2026-06-08-brownfield-source-inventory-observation-packet-boundary-v0-1.md knowledge/research/2026-06-08-brownfield-observation-local-lab-refs.md docs/product/BROWNFIELD-INVENTORY.md docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/CRATE-STATUS.md evals/specs/brownfield-inventory-boundary.v0.1.md evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md` passed with zero warnings.
- `git diff --check` passed.

## Not tested

No Rust tests are required for this docs/eval boundary slice. No Rust code,
runtime behavior, CLI behavior, scanner, file walker, source content reader,
hash collector, manifest generator from repository state, `.punk` state, lab
runner, or benchmark suite was changed.
