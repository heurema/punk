---
id: report_2026_05_01_contract_core_commit_plan_v0_1
kind: work-report
status: complete
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal: work/goals/goal_prepare_contract_core_commit_plan_v0_1.md
---

# Contract-core commit plan v0.1

## Summary

Commit-plan verdict:

```text
manual_split_decision_required
```

The tree is coherent and checks are green, but it should not be committed as one blind unit. The plan is clear enough to split, but the final staging/commit decision should be made explicitly by the maintainer because two high-impact files are shared across tracks:

- `crates/punk-eval/src/lib.rs`
- `docs/product/CRATE-STATUS.md`

No files were staged and no commits were created.

Recommended next selected goal:

```text
work/goals/goal_manual_commit_decision_contract_core_v0_1.md
```

## Files inspected

Commands/surfaces inspected:

```bash
git status --short
git diff --stat
git diff --name-only
git ls-files --others --exclude-standard
```

Inspected by group:

- contract-core model/eval chain: `crates/punk-contract/src/lib.rs`, `crates/punk-eval/src/lib.rs`, `docs/product/CONTRACT-SCHEMA.md`, `docs/product/CRATE-STATUS.md`, related eval specs, goals, and reports;
- proofpack-writer track: `crates/punk-proof/src/lib.rs`, proofpack writer eval specs, goals, reports, and shared `crates/punk-eval/src/lib.rs` / `docs/product/CRATE-STATUS.md` hunks;
- replayability research side-track: `knowledge/research/2026-04-30-replayable-project-memory.md`, `knowledge/ideas/**`, `docs/product/NORTH-STAR.md`, `docs/product/PROJECT-MEMORY.md`, `docs/product/DOCUMENTATION-MAP.md`, `docs/product/GLOSSARY.md`, related goals/reports;
- work ledger/status/goals/reports: `work/STATUS.md`, `work/goals/**`, `work/reports/**`.

## Current working tree inventory

Inventory at planning time:

```text
tracked modified files: 11
untracked files before this planning report: 69
total pre-plan files: 80
planning artifacts added by this task: 2
total classified files in this report: 82
```

Tracked diff stat at planning time:

```text
11 files changed, 18263 insertions(+), 4153 deletions(-)
```

No `.punk/` runtime state, build artifacts, temporary directories, `node_modules`, or `.DS_Store` appeared in `git status --short`.

## Group counts

- Group A: 26
- Group B: 37
- Group C: 11
- Group D: 6
- Group E: 0
- Group F: 0
- Split A/B: 2

Notes:

- `Split A/B` files are not suspicious; they need patch-level staging if contract-core and proofpack-writer commits are split.
- Group E has no current file because no surgical docs-governance cleanup was performed in this task.
- Group F has no current file; no unrelated/suspicious file was found.

## Group classification table

| File | Group | Notes |
|---|---|---|
| `crates/punk-contract/src/lib.rs` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `crates/punk-eval/src/lib.rs` | Split A/B | Patch-level split required: contract-core smoke evals and proofpack-writer smoke evals share this file. |
| `crates/punk-proof/src/lib.rs` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `docs/product/CRATE-STATUS.md` | Split A/B | Patch-level split required: status updates cover contract-core and proofpack-writer surfaces. |
| `docs/product/DOCUMENTATION-MAP.md` | Group C | Replayability advisory research/docs side-track. |
| `docs/product/GLOSSARY.md` | Group C | Replayability advisory research/docs side-track. |
| `docs/product/NORTH-STAR.md` | Group C | Replayability advisory research/docs side-track. |
| `docs/product/PROJECT-MEMORY.md` | Group C | Replayability advisory research/docs side-track. |
| `evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/STATUS.md` | Group D | Shared work ledger, stabilization, or commit-planning artifact. |
| `work/goals/goal_run_fifty_fifth_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `docs/product/CONTRACT-SCHEMA.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `knowledge/ideas/2026-04-30-replayability-backlog.md` | Group C | Replayability advisory research/docs side-track. |
| `knowledge/ideas/2026-04-30-replayable-project-memory.md` | Group C | Replayability advisory research/docs side-track. |
| `knowledge/research/2026-04-30-replayable-project-memory.md` | Group C | Replayability advisory research/docs side-track. |
| `work/goals/goal_add_proofpack_writer_concrete_path_storage_policy_model_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_add_proofpack_writer_first_active_write_slice_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_add_proofpack_writer_hash_reference_verification_integration_model_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_add_proofpack_writer_host_path_resolution_model_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_capture_contract_schema_blueprint_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_capture_replayable_project_memory_direction.md` | Group C | Replayability advisory research/docs side-track. |
| `work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_define_proofpack_writer_concrete_path_storage_policy_boundary_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_define_proofpack_writer_first_active_write_slice_boundary_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_integrate_contract_gate_input_policy_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_integrate_contract_proof_requirements_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_integrate_hard_clause_mapping_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_pause_proofpack_writer_track_for_user_intent_alignment_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_prepare_contract_core_commit_plan_v0_1.md` | Group D | Shared work ledger, stabilization, or commit-planning artifact. |
| `work/goals/goal_reconcile_proofpack_writer_storage_schema_with_host_path_model_v0_1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_run_fifty_eighth_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_fifty_ninth_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_fifty_seventh_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_fifty_sixth_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_sixtieth_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_sixty_first_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_sixty_fourth_work_ledger_review.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_run_sixty_second_work_ledger_review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/goals/goal_run_sixty_third_work_ledger_review.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/goals/goal_save_replayable_project_memory_research_note.md` | Group C | Replayability advisory research/docs side-track. |
| `work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md` | Group D | Shared work ledger, stabilization, or commit-planning artifact. |
| `work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-04-30-contract-schema-blueprint-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-04-30-fifty-eighth-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-fifty-fifth-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-fifty-ninth-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-fifty-seventh-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-fifty-sixth-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-hard-clause-mapping-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-boundary-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-model-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-first-active-write-slice-boundary-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-first-active-write-slice-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-hash-reference-verification-integration-model-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-host-path-resolution-model-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-storage-schema-host-path-reconciliation-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-proofpack-writer-track-pause-for-user-intent-alignment-v0-1.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-replayable-project-memory-direction.md` | Group C | Replayability advisory research/docs side-track. |
| `work/reports/2026-04-30-save-replayable-project-memory-research-note.md` | Group C | Replayability advisory research/docs side-track. |
| `work/reports/2026-04-30-sixtieth-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-sixty-first-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-sixty-fourth-work-ledger-review.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-04-30-sixty-second-work-ledger-review.md` | Group B | Proofpack-writer track; keep separate from contract-core commits. |
| `work/reports/2026-04-30-sixty-third-work-ledger-review.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-05-01-contract-gate-input-policy-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-05-01-contract-proof-requirements-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-05-01-contract-receipt-requirements-v0-1.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-05-01-hard-clause-mapping-before-receipts-ledger-check.md` | Group A | Contract-core side-effect-free chain / related eval, goal, or report. |
| `work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md` | Group D | Shared work ledger, stabilization, or commit-planning artifact. |
| `work/reports/2026-05-01-contract-core-commit-plan-v0-1.md` | Group D | Shared work ledger, stabilization, or commit-planning artifact. |
| `work/goals/goal_manual_commit_decision_contract_core_v0_1.md` | Group D | Shared work ledger, stabilization, or commit-planning artifact. |

## Dependency analysis

### Group A — Contract-core side-effect-free model/eval chain

Can commit independently: yes, with patch-level staging for shared files.

Depends on:

- no runtime dependency on Group B or C;
- historical/work-ledger context references the paused proofpack-writer track, so reports/status should either be included with relevant commits or finalized later in Group D.

Depended on by:

- future Writer/runtime readiness reviews;
- future gate/proof/storage decisions;
- commit-plan/status finalization.

Checks after commit:

```bash
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
cargo fmt --check
cargo check --workspace
cargo test -p punk-contract -p punk-eval
git diff --check
scripts/check.sh docs-governance --files <group-a-files> --report <group-a-report>
```

Splitting difficulty:

- `crates/punk-eval/src/lib.rs` contains both contract-core and proofpack-writer evals;
- `docs/product/CRATE-STATUS.md` contains both contract-core and proofpack-writer status updates;
- `work/STATUS.md` references all tracks and should be staged carefully or left for final Group D.

### Group B — Proofpack-writer track

Can commit independently: yes, but it should remain separate from Group A.

Depends on:

- earlier proofpack/product boundaries already represented by goals/reports;
- shared `crates/punk-eval/src/lib.rs` and `docs/product/CRATE-STATUS.md` hunks.

Depended on by:

- parked future proofpack Writer/referenced-ref work;
- work-ledger context that explains why the proofpack writer track was paused for contract-core alignment.

Checks after commit:

```bash
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
cargo fmt --check
cargo check --workspace
cargo test -p punk-eval
git diff --check
scripts/check.sh docs-governance --files <group-b-files> --report <group-b-report>
```

Splitting difficulty:

- high: `crates/punk-eval/src/lib.rs` is shared with Group A;
- medium: `docs/product/CRATE-STATUS.md` is shared with Group A;
- proofpack-writer track includes a prior narrow active exact-byte write slice for explicit caller-provided test targets, so commit message and review notes must not imply broader runtime storage or `.punk/proofs` activation.

### Group C — Replayability research side-track

Can commit independently: yes.

Depends on:

- no code dependency on Groups A or B;
- docs-governance warnings remain accepted/deferred unless the maintainer chooses cleanup first.

Depended on by:

- future ADR/research promotion only;
- no active contract-core runtime behavior.

Checks after commit:

```bash
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
git diff --check
scripts/check.sh docs-governance --files <group-c-files> --report work/reports/2026-04-30-save-replayable-project-memory-research-note.md
```

Splitting difficulty: low/medium. It has docs warnings that are currently non-blocking and classified as cleanup candidates.

### Group D — Work ledger / reports shared updates

Can commit independently: yes, but best as final ledger/status commit or patch-split into the semantic commits it supports.

Depends on:

- selected commit grouping;
- maintainer decision on whether to stage ledger evidence with each group or finalize after semantic commits.

Depended on by:

- future Codex sessions and work-ledger checks.

Checks after commit:

```bash
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
git diff --check
scripts/check.sh docs-governance --files <group-d-files> --report <group-d-report>
```

Splitting difficulty: medium/high. `work/STATUS.md` is a shared ledger file and should not be committed blindly with only one semantic group unless patch-staged.

### Group E — Docs warning cleanup

Can commit independently: yes, if selected later.

Depends on:

- maintainer decision to clean accepted warnings before or after semantic commits.

Depended on by: none.

Current recommendation: do not block commit planning on Group E.

### Group F — Unrelated / suspicious

No unrelated or suspicious files were found in the current inventory.

## Recommended commit order

Recommended sequence if the maintainer approves split commits:

1. Contract-core side-effect-free model/eval chain.
2. Replayability research side-track.
3. Proofpack-writer track, only if the maintainer wants to include the already-reviewed proofpack work in the same PR/series.
4. Work ledger/status finalization.
5. Optional docs-governance cleanup as a separate follow-up if desired.

Why this order:

- It preserves the active product direction: contract-core first, Writer/runtime later.
- Replayability remains advisory and can be reviewed separately as docs/research.
- Proofpack-writer work is isolated because it includes a prior narrow active exact-byte write slice and should not be read as part of contract-core proof requirements.
- Work ledger/status should be final because it cross-references all groups.

Alternative if minimizing patch-level complexity is more important:

1. Proofpack-writer track.
2. Contract-core side-effect-free model/eval chain.
3. Replayability research side-track.
4. Work ledger/status finalization.

That alternative may reduce shared-file staging churn in `crates/punk-eval/src/lib.rs`, but it makes proofpack-writer work appear before the contract-core stabilization decision in commit history.

## Draft commit messages

Recommended drafts:

```text
feat(contract): add side-effect-free contract-core readiness chain
```

Why:

- Captures user intent-to-contract draft, Contract Schema Blueprint, confirmation, hard-clause mapping, receipt requirements, gate input policy, and proof requirements.
- Must state this is model/eval only and does not activate runtime storage, CLI, Writer, gate/proof writers, or acceptance.

```text
docs(research): record replayable project memory direction
```

Why:

- Saves Replayable Project Memory as advisory research and idea backlog.
- Must preserve `preserve replayable obligations, not replayable code` and avoid Conformance Pack/runtime overclaim.

```text
feat(proof): add bounded proofpack writer policy and write slices
```

Why:

- Captures the proofpack-writer side-effect-free policy/model track plus the prior first active exact-byte write slice.
- Must specify the write slice is limited to explicit caller-provided test targets and does not activate `.punk/proofs`, runtime storage, CLI, gate decisions, or acceptance claims.

```text
chore(work): finalize contract-core stabilization ledger
```

Why:

- Records work-ledger/status/review/stabilization/commit-plan evidence after semantic groups are staged.
- Should be final or patch-split with each semantic commit.

Optional later:

```text
docs(product): clean docs governance term warnings
```

Why:

- Addresses accepted non-blocking docs-governance warnings without mixing cleanup into semantic commits.

## Boundary audit by group

| Group | CLI | runtime storage | Writer | gate/proof writer | proofpack writer | artifact hash runtime | acceptance claim writer | Conformance/Migration/Regenerative/spec-as-source |
|---|---|---|---|---|---|---|---|---|
| Group A | No | No | No | No | No | No | No | No |
| Group B | No CLI | No `.punk/proofs` or runtime storage | No | No gate/proof decision writer | Contains prior narrow exact-byte proofpack writer slice only; no broader runtime writer | Existing bounded hash/file helpers are separate proofpack evidence helpers; no broad artifact hash runtime activation by this planning task | No | No |
| Group C | No | No | No | No | No | No | No | No |
| Group D | No | No | No | No | No | No | No | No |
| Group E | No | No | No | No | No | No | No | No |

## Remaining warnings

Accepted/deferred docs-governance warnings:

- `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary` duplicate definition candidate.
- `docs/product/CRATE-STATUS.md`: `Current CLI surface` duplicate definition candidate.
- `docs/product/DOCUMENTATION-MAP.md`: `Research notes` undeclared glossary term.
- `docs/product/PROJECT-MEMORY.md`: `Project coherence` duplicate definition candidate.

Other warning:

- Engram memory transport is failing with `Transport closed`; repo-tracked reports and work ledger are the durable memory.

## Checks run

Final check results:

```text
pass: python3 scripts/check_research_gate.py
pass: python3 scripts/check_work_ledger.py
pass: cargo fmt --check
pass: cargo check --workspace
pass: cargo test -p punk-contract -p punk-eval
pass: git diff --check
pass: scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_prepare_contract_core_commit_plan_v0_1.md work/goals/goal_manual_commit_decision_contract_core_v0_1.md work/reports/2026-05-01-contract-core-commit-plan-v0-1.md --report work/reports/2026-05-01-contract-core-commit-plan-v0-1.md
pass with 4 accepted/deferred warnings: scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md docs/product/CONTRACT-SCHEMA.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/NORTH-STAR.md docs/product/PROJECT-MEMORY.md knowledge/research/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayability-backlog.md work/STATUS.md --report work/reports/2026-05-01-contract-core-commit-plan-v0-1.md
pass: git status --short
pass: git diff --stat
```

## Recommendation

```text
manual human commit decision before execution
```

The split plan is clear, but the tree contains shared files and a proofpack-writer track with a prior active exact-byte write slice. The next step should ask the maintainer which commit strategy to execute, rather than staging/committing automatically.

## Next selected goal

```text
work/goals/goal_manual_commit_decision_contract_core_v0_1.md
```

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Prepare a split commit plan for the large mixed contract-core, proofpack-writer, replayability, and work-ledger tree without staging or committing."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_prepare_contract_core_commit_plan_v0_1.md
    - work/goals/goal_manual_commit_decision_contract_core_v0_1.md
    - work/reports/2026-05-01-contract-core-commit-plan-v0-1.md
  required_updates:
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
