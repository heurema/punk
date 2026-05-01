---
id: report_2026_05_01_stabilize_uncommitted_contract_core_tree_v0_1
kind: work-report
status: complete
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal: work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md
---

# Stabilize uncommitted contract-core tree v0.1

## Summary

The current uncommitted tree is coherent enough to preserve, but too large and mixed for a single blind commit.

Stabilization verdict:

```text
split_commit_recommended
```

Recommended next selected goal:

```text
work/goals/goal_prepare_contract_core_commit_plan_v0_1.md
```

No new product behavior, model feature, runtime writer, CLI, storage, Writer, gate writer, proof writer, proofpack writer, artifact hash runtime, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added by this stabilization task.

## Files inspected

Inventory commands:

```bash
git status --short
git diff --stat
git diff --name-only
git ls-files --others --exclude-standard
```

Minimum inspected files/surfaces:

- `work/STATUS.md`
- `work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md`
- `work/goals/goal_prepare_contract_core_commit_plan_v0_1.md`
- `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md`
- `work/reports/2026-05-01-contract-proof-requirements-v0-1.md`
- `work/reports/2026-05-01-contract-gate-input-policy-v0-1.md`
- `work/reports/2026-05-01-contract-receipt-requirements-v0-1.md`
- `work/reports/2026-04-30-hard-clause-mapping-v0-1.md`
- `work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md`
- `work/reports/2026-04-30-contract-schema-blueprint-v0-1.md`
- `work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md`
- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `crates/punk-domain/src/lib.rs`
- `crates/punk-gate/src/lib.rs`
- `crates/punk-proof/src/lib.rs`
- `docs/product/CONTRACT-SCHEMA.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/NORTH-STAR.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`

## Working tree inventory

Tracked modified files:

- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `crates/punk-proof/src/lib.rs`
- `docs/product/CRATE-STATUS.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `docs/product/NORTH-STAR.md`
- `docs/product/PROJECT-MEMORY.md`
- `evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_run_fifty_fifth_work_ledger_review.md`

Untracked groups observed:

- Contract-core docs/model/eval artifacts:
  - `docs/product/CONTRACT-SCHEMA.md`
  - `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md`
  - `work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md`
  - `work/goals/goal_capture_contract_schema_blueprint_v0_1.md`
  - `work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md`
  - `work/goals/goal_integrate_hard_clause_mapping_v0_1.md`
  - `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md`
  - `work/goals/goal_integrate_contract_gate_input_policy_v0_1.md`
  - `work/goals/goal_integrate_contract_proof_requirements_v0_1.md`
  - corresponding `work/reports/2026-04-30-*` and `work/reports/2026-05-01-*` reports.
- Proofpack-writer track artifacts:
  - `evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md`
  - `evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md`
  - proofpack-writer goals/reports for host path, storage/schema reconciliation, concrete path/storage policy, first active write slice, and hash/reference integration.
- Replayability research side-track:
  - `knowledge/research/2026-04-30-replayable-project-memory.md`
  - `knowledge/ideas/2026-04-30-replayable-project-memory.md`
  - `knowledge/ideas/2026-04-30-replayability-backlog.md`
  - `work/goals/goal_capture_replayable_project_memory_direction.md`
  - `work/goals/goal_save_replayable_project_memory_research_note.md`
  - corresponding reports.
- Work-ledger/review artifacts:
  - advisory review goals/reports from the fifty-fifth through sixty-fourth reviews;
  - current review/stabilization goals and reports.

Tracked diff size at inventory time:

```text
11 tracked files changed, 18263 insertions(+), 4153 deletions(-)
```

No build artifacts, temporary directories, `node_modules`, `.DS_Store`, or `.punk/` runtime state appeared in `git status --short`.

## Contract-core chain status

The accumulated contract-core chain remains coherent as a side-effect-free model/eval path:

```text
raw request
-> intent
-> contract draft
-> user confirmation
-> approved_for_run
-> hard clause mapping
-> receipt requirements
-> gate input policy
-> proof requirements
```

Important boundaries remain explicit:

- `approved_for_run != ready_for_gate`
- `gate outcome -> proofpack -> acceptance claim`
- `hard clause mapping != runtime validator execution`
- `receipt requirements != receipt writer`
- `gate input policy != gate decision`
- `proof requirements != proofpack writer`
- `Writer != upstream planner`

## Replayability side-track status

Replayable Project Memory remains advisory research and idea memory.

It preserves the thesis:

```text
Punk should preserve replayable obligations, not replayable code.
```

It does not activate:

- Conformance Pack runtime;
- Migration Contract runtime;
- Regenerative Spec behavior;
- code generation;
- spec-as-source behavior;
- gate/proof/Writer authority.

The research wording uses `adopt-now discipline` rather than active runtime language, and source-reference hygiene remains a prerequisite before ADR/promotion use.

## Boundary audit

Stabilization found no new activation by the contract-core path for:

- CLI behavior;
- `.punk/contracts`, `.punk/runs`, `.punk/decisions`, or `.punk/proofs` runtime storage;
- runtime contract writer;
- runtime receipt writer;
- runtime validator execution;
- runtime gate writer;
- runtime proof writer;
- proofpack writer as part of contract-core;
- artifact hash runtime as part of contract-core;
- acceptance claim writer;
- Writer;
- Conformance Pack runtime;
- Migration Contract runtime;
- Regenerative Spec behavior;
- spec-as-source behavior.

Existing proofpack-writer work is a separate track in the uncommitted tree. It includes prior side-effect-free models plus a narrow first active exact-byte write slice for explicit caller-provided test targets. That track is not part of the contract-core activation path and should be commit-planned separately.

## Docs honesty audit

`docs/product/CRATE-STATUS.md` and `docs/product/CONTRACT-SCHEMA.md` currently preserve the important current-vs-target boundaries:

- contract-core additions are described as side-effect-free model/eval boundaries;
- `required_now` / `deferred` / `parked` / `future` are not framed as active runtime behavior;
- `approved_for_run` is not `ready_for_gate`;
- proofpack remains downstream of gate outcome;
- `accepted`, `rejected`, and `partially_accepted` remain out of `ContractStatus`;
- no `.punk/*` runtime storage, CLI, Writer, gate/proof writer, proofpack writer expansion, or active acceptance claim writer is claimed for contract-core.

The docs also preserve Replayable Project Memory as advisory research, with Conformance Pack deferred/future and Regenerative Spec Pack parked/future.

## Remaining warnings and classification

A broad docs-governance pass initially failed because `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md` lacked a `DocImpact` block.

Classification:

| Warning/finding | Classification | Decision |
|---|---|---|
| Missing `DocImpact` in `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md` | fix now | Fixed by adding a `DocImpact` block to the prior review report. |
| `Current implemented subset boundary` in `docs/product/CRATE-STATUS.md` looks like a duplicate definition candidate | accepted non-blocking / defer | It is an existing heading/governance warning; do not rename during stabilization because it could churn canonical status wording. |
| `Current CLI surface` in `docs/product/CRATE-STATUS.md` looks like a duplicate definition candidate | accepted non-blocking / defer | It is an existing heading/governance warning; route to a bounded docs-governance cleanup goal if needed. |
| `Research notes` in `docs/product/DOCUMENTATION-MAP.md` looks like an undeclared glossary term | accepted non-blocking / defer | It requires glossary/docs-map policy choice; not needed for commit-readiness. |
| `Project coherence` in `docs/product/PROJECT-MEMORY.md` looks like a duplicate definition candidate | accepted non-blocking / defer | It is an existing docs-governance warning; do not broaden this stabilization task. |
| Engram memory transport failure | accepted non-blocking | Memory calls failed with `Transport closed`; local repo-tracked report and ledger preserve the decision. |

## Checks run

Final check results:

```text
pass: python3 scripts/check_research_gate.py
pass: python3 scripts/check_work_ledger.py
pass: cargo fmt --check
pass: cargo check --workspace
pass: cargo test -p punk-contract -p punk-eval
pass: git diff --check
pass: scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md work/goals/goal_prepare_contract_core_commit_plan_v0_1.md work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md --report work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md
pass with 4 accepted/deferred warnings: scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md docs/product/CONTRACT-SCHEMA.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/NORTH-STAR.md docs/product/PROJECT-MEMORY.md knowledge/research/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayability-backlog.md work/STATUS.md --report work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md
```

Inventory/audit commands run:

```text
pass: git status --short
pass: git diff --stat
pass: git diff --name-only
pass: git ls-files --others --exclude-standard
pass: grep/rg boundary scans over contract/status/docs surfaces
```

A broader docs-governance audit over product docs/research surfaces was also used to classify warnings; the missing prior-report `DocImpact` finding was fixed and the broad audit now passes with the remaining warnings classified above.

## Commit-readiness recommendation

```text
split_commit_recommended
```

Reason:

- The tree is coherent and checks should be run/kept green before commit.
- The local diff is large and spans multiple logical tracks.
- `crates/punk-eval/src/lib.rs`, `docs/product/CRATE-STATUS.md`, `work/STATUS.md`, and work-ledger files contain changes from multiple tracks and likely need patch-level or at least carefully grouped staging.
- A concrete commit plan should be prepared before staging anything.

## Suggested commit grouping

Recommended logical groups:

1. Contract-core side-effect-free model/eval chain
   - `crates/punk-contract/src/lib.rs`
   - contract-core portions of `crates/punk-eval/src/lib.rs`
   - `docs/product/CONTRACT-SCHEMA.md`
   - contract-core portions of `docs/product/CRATE-STATUS.md`
   - user-intent, schema blueprint, confirmation, hard-clause, receipt, gate-input, proof-requirements goals/reports/eval specs.

2. Proofpack-writer track
   - `crates/punk-proof/src/lib.rs`
   - proofpack-writer portions of `crates/punk-eval/src/lib.rs`
   - proofpack-writer eval specs, goals, reports, and relevant `CRATE-STATUS.md` portions.

3. Replayability research side-track
   - `knowledge/research/2026-04-30-replayable-project-memory.md`
   - `knowledge/ideas/2026-04-30-replayable-project-memory.md`
   - `knowledge/ideas/2026-04-30-replayability-backlog.md`
   - relevant `NORTH-STAR.md`, `PROJECT-MEMORY.md`, `DOCUMENTATION-MAP.md`, `GLOSSARY.md`, goals, and reports.

4. Work-ledger / review evidence
   - `work/STATUS.md`
   - advisory review goals/reports and stabilization/review reports.
   - Prefer grouping ledger evidence with the semantic commit it supports when possible, rather than making one giant ledger-only commit.

5. Optional docs-governance cleanup
   - duplicate heading/glossary warning cleanup, only if chosen as a separate bounded cleanup goal.

## Drift observed

- The tree is large and mixed across contract-core, proofpack-writer, replayability research, docs, and work-ledger tracks.
- Proofpack-writer changes include a separate narrow active write slice; it must not be confused with contract-core proof requirements, which remain declarative only.
- Docs-governance warnings remain, but they are non-blocking for stabilization.
- Engram memory transport remains unavailable, so repo-tracked reports are the durable memory for this task.

## Next selected goal

Selected:

```text
work/goals/goal_prepare_contract_core_commit_plan_v0_1.md
```

## Scope/non-scope for the next goal

Scope:

- prepare a concrete split/staging/commit plan for the current uncommitted tree;
- identify logical commit groups;
- identify files requiring patch-level splitting;
- preserve existing boundaries and warning classifications.

Non-scope:

- no staging, commit, push, or PR unless explicitly requested;
- no new implementation;
- no Writer/runtime/CLI/storage/gate/proof/receipt/proofpack writer;
- no Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Stabilize and inventory the large uncommitted contract-core tree before staging, committing, or selecting further implementation."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md
    - work/goals/goal_prepare_contract_core_commit_plan_v0_1.md
    - work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md
    - work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md
  required_updates:
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
