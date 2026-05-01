# Contract Schema Blueprint v0.1

Date: 2026-04-30
Goal: `work/goals/goal_capture_contract_schema_blueprint_v0_1.md`
Status: completed
Research Gate: R1, satisfied by repo-tracked product docs, Work Ledger state, and recent intent-to-contract evidence. No external research used.

## Summary

Captured the full high-level Punk Contract shape as `Contract Schema Blueprint v0.1`.

The change preserves the complete contract architecture before continuing smaller implementation slices.

It adds:

- side-effect-free blueprint model/types in `punk-contract`;
- side-effect-free smoke eval coverage in `punk-eval`;
- canonical product documentation in `docs/product/CONTRACT-SCHEMA.md`;
- registry/glossary/status updates;
- Work Ledger evidence for the scope override.

Selected next remains: `work/goals/goal_run_sixty_fourth_work_ledger_review.md`.

Reason for replacing the previously selected review temporarily: preserve the full contract schema blueprint before continuing incremental implementation, to avoid losing the high-level architecture.

## Research Gate

- Classification: R1.
- Reason: this is a product/architecture blueprint capture using repo-tracked product docs, current code, Work Ledger state, and the recent user intent-to-contract draft model.
- External research: none.
- Escalation preserved: runtime `.punk/contracts` storage, schema files, CLI behavior, contract writer, Writer activation, provider/model/agent adapters, automation, gate writer, proof writer, acceptance claim writer, policy engine, PKI/signatures, remote transparency logs, and `punk init` remain separate future goals.

## Blueprint sections captured

`Contract Schema Blueprint v0.1` now preserves these top-level sections:

- `identity`
- `authority`
- `lifecycle`
- `work`
- `boundaries`
- `clauses`
- `validation`
- `evidence`
- `receipt_requirements`
- `gate_policy`
- `proof_requirements`
- `change_control`
- `memory_links`

## Active-core vs deferred/parked/future

Active-core / `required_now`:

- identity and digest metadata;
- authority refs and upper-rule narrowing/weaken flags;
- lifecycle statuses without acceptance outcomes;
- goal refs, intent, rationale, assumptions, unknowns, research/decision refs, and acceptance criteria;
- scope include/exclude, non-scope, forbidden effects, and side-effect boundaries;
- typed clauses and hard-clause mapping requirements;
- validator plan, eval refs, unsupported-clause policy;
- evidence plan and source/freshness/contradiction flags;
- receipt required fields;
- gate policy inputs and hard-fail conditions;
- proof link/hash requirements;
- supersede-only change/deviation policy;
- memory links and DocImpact.

Deferred:

- context-clause coverage matrix;
- waiver ledger;
- partial acceptance workflow;
- richer compatibility/version checks;
- module/adapter boundary manifests;
- stale/superseded source automation.

Parked:

- PKI signatures;
- remote transparency logs;
- OPA/Rego or policy engine integration;
- full capability manifests;
- contract negotiation workflow.

Future-only:

- semantic intent recovery;
- contract-as-source generation;
- spec-native takeover;
- multi-agent coordinated contracts;
- semantic merge of contracts/specs/code.

## Implementation notes

- Added `CONTRACT_SCHEMA_BLUEPRINT_VERSION`.
- Added `ContractSchemaBlueprintV0_1`, `ContractSchemaSection`, `ContractSchemaField`, and `ContractSchemaFieldStatus`.
- Added `ContractSchemaAvoidPattern` for explicit avoid patterns.
- Extended `ContractStatus` to include `Superseded` and `Cancelled` while keeping acceptance outcomes out of Contract status.
- Added typed clause enums and `ContractClauseBlueprint` with hard-clause mapping validation.
- Added declarative `ContractGatePolicyBlueprint` that declares required inputs and does not write outcomes.
- Added declarative `ContractProofRequirementsBlueprint` that declares links/hashes and does not create proofpacks.
- Added `ContractWriterAuthorityBoundary` to preserve that Writer is not upstream contract authority.

## Evals added

Added `punk-eval` smoke coverage for:

- `eval_contract_schema_blueprint_has_required_top_level_sections`
- `eval_contract_schema_blueprint_marks_status_required_now_deferred_parked_future`
- `eval_contract_status_does_not_include_acceptance_decisions`
- `eval_hard_clause_requires_validator_receipt_proof_or_gate_review_mapping`
- `eval_proof_requirements_do_not_create_proofpack`
- `eval_gate_policy_does_not_write_decision`
- `eval_writer_is_not_upstream_contract_authority`

## Changed files

- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `docs/product/CONTRACT-SCHEMA.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_capture_contract_schema_blueprint_v0_1.md`
- `work/goals/goal_run_sixty_fourth_work_ledger_review.md`
- `work/reports/2026-04-30-contract-schema-blueprint-v0-1.md`

## Knowledge impact

- Punk now has a canonical Contract Schema Blueprint before runtime activation.
- Future incremental contract-flow work can implement one slice at a time without losing the full target shape.
- The next advisory review should consider whether to continue with `draft -> user confirmation -> approved_for_run`, hard-clause mapping integration, receipt requirements integration, gate input policy, proof requirements integration, or a later Writer step.

## Scope boundaries preserved

This step did not add:

- runtime `.punk/contracts` storage
- schema files
- CLI behavior
- contract writer
- Writer activation
- provider/model/agent adapters
- automation
- gate writer
- proof writer
- acceptance claim writer
- policy engine integration
- PKI/signatures
- remote transparency logs
- `punk init`

Level 0 `done` remains manual closure with evidence, not future `gate` acceptance.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Added side-effect-free Contract Schema Blueprint model, eval coverage, and canonical product documentation."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/CONTRACT-SCHEMA.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_capture_contract_schema_blueprint_v0_1.md
    - work/goals/goal_run_sixty_fourth_work_ledger_review.md
    - work/reports/2026-04-30-contract-schema-blueprint-v0-1.md
  required_updates:
    - docs/product/CONTRACT-SCHEMA.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/GLOSSARY.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - cargo test -p punk-contract -p punk-eval
    - cargo check --workspace
```

## Checks run

- `git status --short --branch`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `cargo fmt --all`
- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test -p punk-contract -p punk-eval`
- `git diff --check`
- `scripts/check.sh docs-governance --files crates/punk-contract/src/lib.rs crates/punk-eval/src/lib.rs docs/product/CONTRACT-SCHEMA.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_capture_contract_schema_blueprint_v0_1.md work/goals/goal_run_sixty_fourth_work_ledger_review.md work/reports/2026-04-30-contract-schema-blueprint-v0-1.md --report work/reports/2026-04-30-contract-schema-blueprint-v0-1.md`

Result: PASS.

Docs governance returned 0 failures and 3 existing warnings:

- `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary`
- `docs/product/CRATE-STATUS.md`: `Current CLI surface`
- `docs/product/DOCUMENTATION-MAP.md`: `Research notes`
