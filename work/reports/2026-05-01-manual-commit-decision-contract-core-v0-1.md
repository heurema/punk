---
id: report_2026_05_01_manual_commit_decision_contract_core_v0_1
kind: work-report
status: complete
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal: work/goals/goal_manual_commit_decision_contract_core_v0_1.md
---

# Manual commit decision for contract-core tree v0.1

## 1. Summary

Verdict:

```text
ready for manual commit decision; recommend Strategy C unless maintainer explicitly accepts edited patch staging risk
```

Recommended strategy:

```text
Strategy C — One large checkpoint commit
```

Reason: the tree is coherent and checks are green, but `crates/punk-eval/src/lib.rs` and `docs/product/CRATE-STATUS.md` are not clean split files. They contain mixed contract-core and proofpack-writer changes in shared imports, shared smoke-suite registration/assessment text, and long status bullets. Manual `git add -p` is technically possible, but would require edited hunks and high attention to avoid a misleading or non-compiling intermediate commit.

No files were staged. No commits were created.

If the maintainer values atomic history more than staging safety, the fallback is Strategy A with edited `git add -p` hunks and full checks after every partial commit.

Current classified file count after this decision task:

```text
Group A — contract-core side-effect-free model/eval chain: 26
Group B — proofpack-writer track: 37
Group C — replayability research side-track: 11
Group D — shared ledger/status/planning: 8
Split A/B: 2
Group E/F: 0
Total: 84
```

## 2. Split-risk analysis

### `crates/punk-eval/src/lib.rs`

Contract-core changes include:

- `punk_contract` imports for Contract Schema Blueprint, hard-clause mapping, receipt requirements, gate input policy, proof requirements, user intent-to-contract draft, and confirmation boundary;
- `run_smoke_suite()` registration for contract-core cases:
  - Contract Schema Blueprint;
  - hard clause mapping;
  - receipt requirements;
  - gate input policy;
  - proof requirements;
  - user intent-to-contract draft and confirmation boundary;
  - `ContractStatus` exclusion of `accepted` / `rejected` / `partially_accepted`;
- smoke assessment text and boundary/deferred notes for contract-core side-effect-free boundaries;
- contract-core helper/eval functions in the current file, roughly around current line ranges:
  - `1173..1289` Contract Schema Blueprint and status checks;
  - `1291..1888` hard-clause mapping helpers/evals;
  - `1892..2319` receipt requirements evals;
  - `2333..2779` proof requirements evals;
  - `2786..3285` gate input policy evals;
  - `3286..3888` user intent / confirmation / approved-for-run evals.

Proofpack-writer changes include:

- `std::fs`, `std::process`, `AtomicU64`, and time/temp-path helpers used by the first active write slice smoke coverage;
- `punk_proof` imports for proofpack writer host path resolution, concrete path/storage policy, first active write slice, and hash/reference integration;
- `run_smoke_suite()` registration for proofpack-writer host path, concrete path/storage policy, first active write slice, and hash/reference integration cases;
- proofpack-writer assessment text, boundary notes, and deferred notes;
- proofpack-writer helper/eval functions, especially current line ranges:
  - `3996+` sample writer refs;
  - `4341+` artifact digest helpers;
  - `6012+` host path resolution model eval;
  - `6312+` concrete path/storage policy model eval;
  - `6723+` first active write slice helpers;
  - `6861+` first active write slice smoke eval;
  - `6990+` hash/reference integration model eval;
  - `7476+` referenced artifact verification helper eval.

Can the file be patch-staged safely?

```text
Technically yes, but not safely by default.
```

Risk if committed as one piece:

- It mixes contract-core and proofpack-writer history in one file.
- The resulting history is less atomic, but it is less likely to be misleading than an edited partial patch that accidentally drops imports, cases, or moved proofpack eval functions.

Risk if patch-staged:

- `git diff` represents some changes as large replacement/move-like hunks where contract-core functions were inserted before existing proof/gate/proofpack functions.
- Manual edited hunks must keep imports, function definitions, smoke-suite registration, assessment text, and notes consistent in each intermediate commit.
- A bad partial stage can compile-fail or create a commit that appears to remove proofpack evals and re-add them later.

Check after partial staging:

```bash
git diff --cached -- crates/punk-eval/src/lib.rs
cargo test -p punk-contract -p punk-eval
cargo check --workspace
```

### `docs/product/CRATE-STATUS.md`

Contract-core changes include:

- current implemented behavior for `punk-contract` line/paragraph describing:
  - user intent-to-contract draft model;
  - Contract Schema Blueprint;
  - confirmation boundary;
  - hard-clause mapping;
  - receipt requirements;
  - gate input policy;
  - proof requirements;
  - explicit no-runtime/no-Writer/no-gate-proof authority wording;
- `punk-eval` smoke coverage phrases for contract-core cases;
- active-core responsibility bullets for Contract Schema Blueprint, intent-to-contract, confirmation, hard clauses, receipt requirements, gate input policy, and proof requirements.

Proofpack-writer changes include:

- `punk-eval` smoke coverage phrases for proofpack writer host path resolution, concrete path/storage policy, first active write slice, and hash/reference integration;
- `punk-proof` current behavior paragraph covering host path resolution, concrete path/storage policy, first active write slice, and hash/reference integration;
- future broader writer/runtime boundary wording;
- active-core responsibility proofpack writer bullet expansion.

Can the file be patch-staged safely?

```text
Technically yes, but only with edited hunks because several changes live in the same long Markdown bullet/paragraph.
```

Risk if committed as one piece:

- It couples contract-core and proofpack-writer status narrative.
- It remains honest, but reviewers must read the commit message/report to avoid interpreting proofpack-writer status as contract-core activation.

Risk if patch-staged:

- Splitting long lines in `git add -p` can easily omit boundary qualifiers.
- If contract-core status is staged without matching eval/status phrases, docs-governance may pass but narrative coherence can drift.

Check after partial staging:

```bash
git diff --cached -- docs/product/CRATE-STATUS.md
scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md --report <relevant-report>
python3 scripts/check_work_ledger.py
```

## 3. Recommended manual commit strategy

Chosen strategy:

```text
Strategy C — One large commit
```

Recommended primary sequence:

```text
commit 1: one large checkpoint commit containing the coherent current tree
commit 2: optional docs-governance warning cleanup later, if desired
commit 3: post-commit verification goal/report after maintainer confirms commit exists
```

Why not Strategy A by default:

- `crates/punk-eval/src/lib.rs` has mixed imports, smoke-suite registration, assessment strings, boundary notes, and large replacement/move-like hunks.
- `docs/product/CRATE-STATUS.md` has long mixed bullets that need manual edited hunks to split.
- A bad split would be worse than a large but honest checkpoint commit.

Fallback Strategy A is still available if the maintainer wants atomic commits and is willing to manually edit patch hunks.

## 4. Exact file grouping

### Strategy C primary commit: one checkpoint commit

Include all current files, including the decision artifacts:

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
- `docs/product/CONTRACT-SCHEMA.md`
- `evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md`
- `evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md`
- `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`
- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `work/goals/goal_add_proofpack_writer_concrete_path_storage_policy_model_v0_1.md`
- `work/goals/goal_add_proofpack_writer_first_active_write_slice_v0_1.md`
- `work/goals/goal_add_proofpack_writer_hash_reference_verification_integration_model_v0_1.md`
- `work/goals/goal_add_proofpack_writer_host_path_resolution_model_v0_1.md`
- `work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md`
- `work/goals/goal_capture_contract_schema_blueprint_v0_1.md`
- `work/goals/goal_capture_replayable_project_memory_direction.md`
- `work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md`
- `work/goals/goal_define_proofpack_writer_concrete_path_storage_policy_boundary_v0_1.md`
- `work/goals/goal_define_proofpack_writer_first_active_write_slice_boundary_v0_1.md`
- `work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md`
- `work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md`
- `work/goals/goal_integrate_contract_gate_input_policy_v0_1.md`
- `work/goals/goal_integrate_contract_proof_requirements_v0_1.md`
- `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md`
- `work/goals/goal_integrate_hard_clause_mapping_v0_1.md`
- `work/goals/goal_manual_commit_decision_contract_core_v0_1.md`
- `work/goals/goal_pause_proofpack_writer_track_for_user_intent_alignment_v0_1.md`
- `work/goals/goal_prepare_contract_core_commit_plan_v0_1.md`
- `work/goals/goal_reconcile_proofpack_writer_storage_schema_with_host_path_model_v0_1.md`
- `work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md`
- `work/goals/goal_run_fifty_eighth_work_ledger_review.md`
- `work/goals/goal_run_fifty_ninth_work_ledger_review.md`
- `work/goals/goal_run_fifty_seventh_work_ledger_review.md`
- `work/goals/goal_run_fifty_sixth_work_ledger_review.md`
- `work/goals/goal_run_sixtieth_work_ledger_review.md`
- `work/goals/goal_run_sixty_first_work_ledger_review.md`
- `work/goals/goal_run_sixty_fourth_work_ledger_review.md`
- `work/goals/goal_run_sixty_second_work_ledger_review.md`
- `work/goals/goal_run_sixty_third_work_ledger_review.md`
- `work/goals/goal_save_replayable_project_memory_research_note.md`
- `work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md`
- `work/goals/goal_wait_for_manual_contract_core_commits_v0_1.md`
- `work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md`
- `work/reports/2026-04-30-contract-schema-blueprint-v0-1.md`
- `work/reports/2026-04-30-fifty-eighth-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-fifth-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-ninth-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-seventh-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-sixth-work-ledger-review.md`
- `work/reports/2026-04-30-hard-clause-mapping-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-boundary-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-model-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-first-active-write-slice-boundary-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-first-active-write-slice-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-hash-reference-verification-integration-model-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-host-path-resolution-model-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-storage-schema-host-path-reconciliation-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-track-pause-for-user-intent-alignment-v0-1.md`
- `work/reports/2026-04-30-replayable-project-memory-direction.md`
- `work/reports/2026-04-30-save-replayable-project-memory-research-note.md`
- `work/reports/2026-04-30-sixtieth-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-first-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-fourth-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-second-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-third-work-ledger-review.md`
- `work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md`
- `work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md`
- `work/reports/2026-05-01-contract-core-commit-plan-v0-1.md`
- `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md`
- `work/reports/2026-05-01-contract-gate-input-policy-v0-1.md`
- `work/reports/2026-05-01-contract-proof-requirements-v0-1.md`
- `work/reports/2026-05-01-contract-receipt-requirements-v0-1.md`
- `work/reports/2026-05-01-hard-clause-mapping-before-receipts-ledger-check.md`
- `work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md`
- `work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md`

Primary message draft:

```text
chore(work): checkpoint contract-core stabilization tree
```

Message constraints:

- mention contract-core side-effect-free chain;
- mention replayability research is advisory;
- mention proofpack-writer track includes a separate previously reviewed narrow exact-byte write slice;
- mention no `.punk` runtime storage, CLI, Writer, gate/proof writers, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior is activated by the checkpoint.

### Fallback Strategy A, commit 1: contract-core side-effect-free model/eval/docs

Safe whole-file list:

- `crates/punk-contract/src/lib.rs`
- `docs/product/CONTRACT-SCHEMA.md`
- `evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md`
- `work/goals/goal_add_user_intent_to_contract_draft_model_v0_1.md`
- `work/goals/goal_capture_contract_schema_blueprint_v0_1.md`
- `work/goals/goal_define_contract_draft_confirmation_boundary_v0_1.md`
- `work/goals/goal_define_user_intent_to_contract_ux_flow_boundary_v0_1.md`
- `work/goals/goal_integrate_contract_gate_input_policy_v0_1.md`
- `work/goals/goal_integrate_contract_proof_requirements_v0_1.md`
- `work/goals/goal_integrate_contract_receipt_requirements_v0_1.md`
- `work/goals/goal_integrate_hard_clause_mapping_v0_1.md`
- `work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md`
- `work/goals/goal_run_sixty_fourth_work_ledger_review.md`
- `work/goals/goal_run_sixty_third_work_ledger_review.md`
- `work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md`
- `work/reports/2026-04-30-contract-schema-blueprint-v0-1.md`
- `work/reports/2026-04-30-hard-clause-mapping-v0-1.md`
- `work/reports/2026-04-30-sixty-fourth-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-third-work-ledger-review.md`
- `work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md`
- `work/reports/2026-04-30-user-intent-to-contract-ux-flow-boundary-v0-1.md`
- `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md`
- `work/reports/2026-05-01-contract-gate-input-policy-v0-1.md`
- `work/reports/2026-05-01-contract-proof-requirements-v0-1.md`
- `work/reports/2026-05-01-contract-receipt-requirements-v0-1.md`
- `work/reports/2026-05-01-hard-clause-mapping-before-receipts-ledger-check.md`

Split-risk files:

- `crates/punk-eval/src/lib.rs` — include selected contract-core hunks only;
- `docs/product/CRATE-STATUS.md` — include selected contract-core status/eval/responsibility hunks only;
- `work/STATUS.md` — either skip until final ledger commit or include only contract-core completion rows if manually patch-staging ledger.

Draft message:

```text
feat(contract): add side-effect-free contract-core readiness chain
```

### Fallback Strategy A, commit 2: replayability advisory research

Safe whole-file list:

- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `docs/product/NORTH-STAR.md`
- `docs/product/PROJECT-MEMORY.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`
- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `work/goals/goal_capture_replayable_project_memory_direction.md`
- `work/goals/goal_save_replayable_project_memory_research_note.md`
- `work/reports/2026-04-30-replayable-project-memory-direction.md`
- `work/reports/2026-04-30-save-replayable-project-memory-research-note.md`

Draft message:

```text
docs(research): record replayable project memory direction
```

### Fallback Strategy A, commit 3: proofpack-writer track

Safe whole-file list:

- `crates/punk-proof/src/lib.rs`
- `evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md`
- `work/goals/goal_run_fifty_fifth_work_ledger_review.md`
- `evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md`
- `evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md`
- `work/goals/goal_add_proofpack_writer_concrete_path_storage_policy_model_v0_1.md`
- `work/goals/goal_add_proofpack_writer_first_active_write_slice_v0_1.md`
- `work/goals/goal_add_proofpack_writer_hash_reference_verification_integration_model_v0_1.md`
- `work/goals/goal_add_proofpack_writer_host_path_resolution_model_v0_1.md`
- `work/goals/goal_define_proofpack_writer_concrete_path_storage_policy_boundary_v0_1.md`
- `work/goals/goal_define_proofpack_writer_first_active_write_slice_boundary_v0_1.md`
- `work/goals/goal_define_proofpack_writer_referenced_artifact_verification_active_slice_boundary_v0_1.md`
- `work/goals/goal_pause_proofpack_writer_track_for_user_intent_alignment_v0_1.md`
- `work/goals/goal_reconcile_proofpack_writer_storage_schema_with_host_path_model_v0_1.md`
- `work/goals/goal_run_fifty_eighth_work_ledger_review.md`
- `work/goals/goal_run_fifty_ninth_work_ledger_review.md`
- `work/goals/goal_run_fifty_seventh_work_ledger_review.md`
- `work/goals/goal_run_fifty_sixth_work_ledger_review.md`
- `work/goals/goal_run_sixtieth_work_ledger_review.md`
- `work/goals/goal_run_sixty_first_work_ledger_review.md`
- `work/goals/goal_run_sixty_second_work_ledger_review.md`
- `work/reports/2026-04-30-fifty-eighth-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-fifth-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-ninth-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-seventh-work-ledger-review.md`
- `work/reports/2026-04-30-fifty-sixth-work-ledger-review.md`
- `work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-boundary-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-concrete-path-storage-policy-model-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-first-active-write-slice-boundary-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-first-active-write-slice-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-hash-reference-verification-integration-model-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-host-path-resolution-model-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-storage-schema-host-path-reconciliation-v0-1.md`
- `work/reports/2026-04-30-proofpack-writer-track-pause-for-user-intent-alignment-v0-1.md`
- `work/reports/2026-04-30-sixtieth-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-first-work-ledger-review.md`
- `work/reports/2026-04-30-sixty-second-work-ledger-review.md`

Split-risk files:

- `crates/punk-eval/src/lib.rs` — include selected proofpack-writer hunks only;
- `docs/product/CRATE-STATUS.md` — include selected proofpack-writer status/eval/responsibility hunks only.

Draft message:

```text
feat(proof): add bounded proofpack writer policy and write slices
```

Message must say the first active write slice is limited to explicit caller-provided test targets and does not activate `.punk/proofs`, runtime storage, CLI, gate decisions, or acceptance claims.

### Fallback Strategy A, commit 4: work ledger/status finalization

Safe whole-file list:

- `work/STATUS.md`
- `work/goals/goal_manual_commit_decision_contract_core_v0_1.md`
- `work/goals/goal_prepare_contract_core_commit_plan_v0_1.md`
- `work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md`
- `work/goals/goal_wait_for_manual_contract_core_commits_v0_1.md`
- `work/reports/2026-05-01-contract-core-commit-plan-v0-1.md`
- `work/reports/2026-05-01-stabilize-uncommitted-contract-core-tree-v0-1.md`
- `work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md`

Draft message:

```text
chore(work): finalize contract-core stabilization ledger
```

## 5. Suggested manual commands

These commands are for the maintainer to run manually. They were not run by Codex.

### Strategy C commands

```bash
git add -- \
  crates/punk-contract/src/lib.rs \
  crates/punk-eval/src/lib.rs \
  crates/punk-proof/src/lib.rs \
  docs/product/CRATE-STATUS.md \
  docs/product/CONTRACT-SCHEMA.md \
  docs/product/DOCUMENTATION-MAP.md \
  docs/product/GLOSSARY.md \
  docs/product/NORTH-STAR.md \
  docs/product/PROJECT-MEMORY.md \
  evals/specs/proofpack-writer-storage-schema-boundary.v0.1.md \
  evals/specs/proofpack-writer-concrete-path-storage-policy-boundary.v0.1.md \
  evals/specs/proofpack-writer-first-active-write-slice-boundary.v0.1.md \
  evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md \
  knowledge/ideas/2026-04-30-replayability-backlog.md \
  knowledge/ideas/2026-04-30-replayable-project-memory.md \
  knowledge/research/2026-04-30-replayable-project-memory.md \
  work/STATUS.md \
  work/goals \
  work/reports

git diff --cached --stat
git diff --cached --name-only
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
cargo fmt --check
cargo check --workspace
cargo test -p punk-contract -p punk-eval
git diff --check --cached
scripts/check.sh docs-governance --files $(git diff --cached --name-only) --report work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md
git commit -m "chore(work): checkpoint contract-core stabilization tree"
```

### Fallback Strategy A commands for split-risk files

For commit 1 and commit 3, use interactive patch staging:

```bash
git add -- <safe whole-file list for the group>
git add -p crates/punk-eval/src/lib.rs
git add -p docs/product/CRATE-STATUS.md
git diff --cached --stat
git diff --cached --name-only
git diff --cached -- crates/punk-eval/src/lib.rs docs/product/CRATE-STATUS.md
cargo test -p punk-contract -p punk-eval
cargo check --workspace
python3 scripts/check_work_ledger.py
scripts/check.sh docs-governance --files $(git diff --cached --name-only) --report <relevant-report>
git commit -m "<message>"
```

If `git add -p` asks to edit a hunk that mixes tracks, prefer aborting split staging and using Strategy C rather than producing a misleading partial commit.

## 6. Checks after each commit

After every manual commit:

```bash
cargo check --workspace
cargo test -p punk-contract -p punk-eval
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
git diff --check HEAD~1..HEAD
```

For docs/research commits:

```bash
scripts/check.sh docs-governance --files <committed-files> --report <relevant-report>
```

After all commits:

```bash
git status --short
python3 scripts/check_research_gate.py
python3 scripts/check_work_ledger.py
cargo fmt --check
cargo check --workspace
cargo test -p punk-contract -p punk-eval
```

## 7. Commit message drafts

Primary Strategy C draft:

```text
chore(work): checkpoint contract-core stabilization tree
```

Fallback split drafts:

```text
feat(contract): add side-effect-free contract-core readiness chain
docs(research): record replayable project memory direction
feat(proof): add bounded proofpack writer policy and write slices
chore(work): finalize contract-core stabilization ledger
```

Do not use messages that imply runtime activation, such as:

```text
implement contract runtime
activate proofpack writer runtime
add replayable project memory runtime
```

## 8. Boundary audit

| Proposed commit | Runtime behavior | CLI | Writer | Gate/proof writer | Spec-as-source | Notes |
|---|---|---|---|---|---|---|
| Strategy C checkpoint | Does not newly activate by this decision task; includes already-reviewed current tree | No new CLI | No | No gate/proof decision writer | No | Contains contract-core side-effect-free chain, replayability advisory research, and the separate proofpack-writer track with prior narrow exact-byte write slice. |
| Contract-core split commit | No | No | No | No | No | Side-effect-free model/eval/docs only. |
| Replayability split commit | No | No | No | No | No | Advisory research; Conformance Pack deferred and Regenerative Spec parked. |
| Proofpack-writer split commit | No `.punk` runtime storage; includes separate prior first active exact-byte write slice for explicit caller-provided test targets | No | No broader Writer | No gate/proof decision writer | No | Must not be described as runtime proofpack storage or acceptance authority. |
| Work ledger split commit | No | No | No | No | No | Process/status/report evidence only. |

## 9. Recommended next selected goal

Selected:

```text
work/goals/goal_wait_for_manual_contract_core_commits_v0_1.md
```

Reason: Codex should stop after producing manual plan and wait for the maintainer to either execute commits manually or choose a different strategy.

## Checks run

Final check results:

```text
pass: python3 scripts/check_research_gate.py
pass: python3 scripts/check_work_ledger.py
pass: cargo fmt --check
pass: cargo check --workspace
pass: cargo test -p punk-contract -p punk-eval
pass: git diff --check
pass: scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_manual_commit_decision_contract_core_v0_1.md work/goals/goal_wait_for_manual_contract_core_commits_v0_1.md work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md --report work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md
pass with 4 accepted/deferred warnings: scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md docs/product/CONTRACT-SCHEMA.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/NORTH-STAR.md docs/product/PROJECT-MEMORY.md knowledge/research/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayable-project-memory.md knowledge/ideas/2026-04-30-replayability-backlog.md work/STATUS.md --report work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md
pass: git status --short
pass: git diff --stat
pass: git diff --cached --name-only returned empty output
```

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Record manual commit decision guidance for the large mixed contract-core tree without staging, committing, or changing product behavior."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_manual_commit_decision_contract_core_v0_1.md
    - work/goals/goal_wait_for_manual_contract_core_commits_v0_1.md
    - work/reports/2026-05-01-manual-commit-decision-contract-core-v0-1.md
  required_updates:
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
