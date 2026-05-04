# Contract Clause Coverage v0.1

Date: 2026-05-04
Status: proposed boundary
Authority: advisory/design

## Purpose

Specify that each hard contract clause must have explicit coverage before approved-for-run or gate-ready status.

This is an eval/spec boundary. It does not add runtime behavior, CLI behavior, contract storage, validator execution, gate writing, proofpack writing, or LLM assessment.

## Status and authority

This spec is advisory/design only.

It supports future contract and gate policy work, but it is not product truth by itself and does not approve any work.

## Definitions

### hard_clause

A contract clause that must be satisfied, covered, explicitly waived, or routed to human gate review before the contract can move toward approved-for-run or gate-ready status.

### coverage target types

Allowed coverage target types:

- `validator_ref`: a deterministic validator or check that can evaluate the clause.
- `required_receipt_field`: a run receipt field that must be present for gate review.
- `proof_requirement_ref`: a proof requirement that must link and hash the relevant evidence after gate decision.
- `source_ref`: a source artifact that supports the clause's meaning or boundary.
- `explicit_assumption`: a bounded assumption accepted for the contract.
- `explicit_unknown`: a known unknown that blocks or narrows the contract until resolved.
- `human_gate_review_reason`: a required manual gate review reason when deterministic coverage is not available.

### unsupported hard clause

A hard clause with no valid coverage target, no explicit unknown, and no human gate review reason.

Unsupported hard clauses block approved-for-run and gate-ready status.

### blocked approval

The expected state when one or more hard clauses are unsupported, covered only by invalid refs, or delegated to a module/assessor as final authority.

### advisory semantic assessment

A semantic or LLM-based assessment may propose mappings and identify gaps. It cannot satisfy a hard clause unless the assessment points to valid coverage targets that Punk can inspect.

### gate authority boundary

Clause coverage can inform gate policy. It does not write final decisions. Only `gate` writes final decisions.

## Positive cases

### CCC-001: hard clause covered by validator_ref

Given a hard clause `tests_must_pass`, when it maps to an existing `validator_ref`, approval may continue if all other required coverage is valid.

Expected result: coverage accepted for that clause.

### CCC-002: hard clause covered by required_receipt_field

Given a hard clause `run_must_record_changed_files`, when it maps to required receipt field `changed_files`, approval may continue if the receipt field is declared in the future receipt policy.

Expected result: coverage accepted for that clause.

### CCC-003: hard clause covered by proof_requirement_ref

Given a hard clause `output_artifacts_must_be_hash_linked`, when it maps to proof requirement `output_artifact_refs_with_digests`, approval may continue if the proof requirement is declared.

Expected result: coverage accepted for that clause.

### CCC-004: hard clause routed to explicit human gate review with reason

Given a hard clause that cannot yet be deterministically checked, when it is routed to human gate review with an explicit reason, approval may continue only if the contract records that reason and later gate policy treats it as review-required.

Expected result: coverage accepted as human-review coverage, not automated proof.

## Negative cases

### CCC-101: hard clause has no coverage

A hard clause with no coverage target must block approved-for-run and gate-ready status.

Expected result: blocked approval.

### CCC-102: hard clause covered only by speculative source

A hard clause covered only by a speculative idea note must not be treated as covered unless the contract explicitly marks the source as advisory and provides another valid coverage target, assumption, unknown, or human review reason.

Expected result: blocked approval.

### CCC-103: semantic assessor says "looks fine" but no coverage exists

A semantic assessor statement that a clause "looks fine" cannot satisfy coverage without a valid validator, receipt field, proof requirement, source ref, assumption, unknown, or human review reason.

Expected result: blocked approval.

### CCC-104: coverage points to missing validator/receipt/proof ref

Coverage that references a missing validator, unknown receipt field, or missing proof requirement must be treated as invalid.

Expected result: blocked approval.

### CCC-105: module assessment tries to satisfy final decision authority

A module assessment may map or assess coverage, but it cannot write final acceptance or satisfy the gate decision boundary.

Expected result: blocked approval for final-decision authority.

## Minimal fixture shape

```yaml
contract_clause_coverage:
  contract_ref: contracts/example-contract.md
  contract_hash: sha256:example
  clauses:
    - clause_id: scope.tests_must_pass
      kind: hard_clause
      covered_by:
        - type: validator_ref
          ref: validators/cargo-test
    - clause_id: evidence.changed_files_recorded
      kind: hard_clause
      covered_by:
        - type: required_receipt_field
          ref: changed_files
    - clause_id: review.security_risk
      kind: hard_clause
      covered_by:
        - type: human_gate_review_reason
          reason: "No deterministic validator exists for this local review slice."
  unsupported: []
```

## Non-goals

This spec does not implement a runtime writer, gate writer, proofpack writer, LLM assessor, or contract storage.

It also does not define a new CLI command, a policy-engine dependency, a graph database, a vector database, or an autonomous agent runner.

## Future implementation constraints

Any future model/helper must keep hard-clause coverage explicit, fail closed for unsupported hard clauses, keep semantic assessments advisory, and preserve `gate` as the only final decision writer.
