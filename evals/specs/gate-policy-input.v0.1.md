# Gate Policy Input v0.1

Date: 2026-05-04
Status: proposed boundary
Authority: advisory/design

## Purpose

Specify the minimum input shape that a future gate policy should evaluate.

This is a policy input spec, not a gate writer, not an external policy-engine dependency, and not an acceptance claim writer.

## Status and authority

This spec is advisory/design only.

It defines future gate policy inputs and negative cases. It does not write gate outcomes.

## Minimum input shape

### contract_ref and contract_hash

Gate policy input must identify the contract being evaluated and its hash.

### approved_for_run status

Gate policy input must show whether the contract was approved for run before execution.

### run_receipt_refs

Gate policy input must include refs and hashes for required run receipts.

### receipt requirement coverage

Gate policy input must summarize whether required receipt fields are present, missing, unsupported, or waived with reason.

### eval/assessment refs

Gate policy input may include eval report refs, module assessments, semantic assessments, or guard results. These are evidence/assessment refs, not decisions.

### scope/deviation status

Gate policy input must represent whether the run stayed in scope, deviated, or has unresolved deviation.

### executor claim status

Gate policy input must distinguish verified evidence from executor self-claims such as "tests passed" or "done".

### clause coverage summary

Gate policy input must summarize hard-clause coverage, unsupported clauses, explicit assumptions, unknowns, and human gate review reasons.

### context pack summary, if applicable

If a context pack was used, gate policy input may include selected refs, exclusions, stale flags, contradictions, unknowns, and assumptions.

### proof requirements declaration

Gate policy input must declare what proof requirements will need to be linked after decision. Proofpack itself is not a required input before decision.

### decision writer boundary

Gate policy input must record that only `gate` may write the final decision.

## Positive cases

### GPI-001: complete deterministic evidence input

Contract ref/hash, approved-for-run status, run receipt refs, receipt coverage, eval refs, in-scope status, clause coverage, and proof requirements are present.

Expected result: gate policy may evaluate the input.

### GPI-002: explicit waiver with reason

A missing non-critical receipt field is waived with explicit reason and waiver authority.

Expected result: gate policy may evaluate with waiver visible.

### GPI-003: context pack summary is advisory

Context pack summary is included with selected refs and stale flags, but does not define acceptance criteria.

Expected result: gate policy may use it as evidence-selection context only.

## Required negative cases

### GPI-101: executor claim says tests passed but no verified eval/ref exists

An executor claim must not be treated as verified evidence.

Expected result: gate policy blocks or records unverified claim.

### GPI-102: missing required receipt field

A required receipt field is absent and no explicit waiver exists.

Expected result: gate policy blocks.

### GPI-103: unsupported hard clause

Hard-clause coverage summary contains unsupported hard clause.

Expected result: gate policy blocks.

### GPI-104: scope deviation unresolved

Run evidence shows unresolved scope deviation.

Expected result: gate policy blocks or requests rework.

### GPI-105: module assessment attempts final decision

Module assessment says "accepted" as final authority.

Expected result: gate policy rejects final-authority claim and treats module output as assessment only.

### GPI-106: proofpack presented before decision

A proofpack is presented as input before gate decision.

Expected result: gate policy rejects proofpack-as-precondition authority; proof requirements may be declared, but proof follows decision.

### GPI-107: stale/superseded source used without surfacing

Context or evidence uses stale source as current truth without flagging it.

Expected result: gate policy blocks or requires explicit stale-source handling.

### GPI-108: waiver missing explicit reason

A waiver without reason is invalid.

Expected result: gate policy blocks.

## Minimal fixture shape

```yaml
gate_policy_input:
  contract_ref: contracts/example-contract.md
  contract_hash: sha256:example-contract
  approved_for_run: true
  run_receipt_refs:
    - ref: runs/example-receipt.json
      hash: sha256:example-receipt
  receipt_requirement_coverage:
    missing: []
    waived: []
  eval_assessment_refs:
    - ref: evals/reports/example-smoke.json
      hash: sha256:example-eval
      authority: assessment
  scope_deviation_status: none
  executor_claim_status:
    unverified_claims: []
  clause_coverage_summary:
    unsupported_hard_clauses: []
  context_pack_summary:
    ref: context-packs/example.yaml
    stale_flags: []
    contradictions: []
  proof_requirements:
    - contract_hash
    - run_receipt_hash
    - gate_decision_hash
  decision_writer_boundary: gate_only
```

## Non-goals

This spec does not add a gate writer, external policy-engine dependency, Rego/CUE/OPA integration, acceptance claim writer, proofpack writer, runtime storage, validator runner, CLI behavior, or LLM gate.

## Future implementation constraints

Any future gate policy model must distinguish inputs from decisions, assessments from authority, proof requirements from proofpack artifacts, and executor claims from verified evidence.
