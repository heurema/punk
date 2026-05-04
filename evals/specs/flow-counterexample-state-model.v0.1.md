# Flow Counterexample State Model v0.1

Date: 2026-05-04
Status: proposed boundary
Authority: advisory/design

## Purpose

Specify counterexample-first evals for the `plot / cut / gate` lifecycle.

This spec does not add new CLI behavior or flow runtime behavior.

## Status and authority

This is an eval/spec artifact only.

It describes future fixture expectations for legal transitions, denied transitions, no-forbidden-artifact behavior, and event/receipt expectations.

## Legal states

The future model should distinguish at least:

- `goal_recorded`
- `contract_draft`
- `contract_approved_for_run`
- `run_started`
- `run_receipt_recorded`
- `gate_decision_written`
- `proofpack_available`
- `acceptance_claim_allowed`
- `denied`

These states are model vocabulary for evals. They do not imply runtime storage exists now.

## Legal transitions

Allowed transition shape:

| From | Transition | To | Expected evidence |
|---|---|---|---|
| `goal_recorded` | `plot_contract` | `contract_draft` | contract draft ref or explicit clarification/refusal |
| `contract_draft` | `approve_for_run` | `contract_approved_for_run` | scope and hard-clause coverage |
| `contract_approved_for_run` | `cut_run` | `run_started` | run start event |
| `run_started` | `record_receipt` | `run_receipt_recorded` | run receipt ref |
| `run_receipt_recorded` | `gate_review` | `gate_decision_written` | gate decision ref |
| `gate_decision_written` | `generate_proofpack` | `proofpack_available` | proofpack refs and hashes |
| `proofpack_available` | `claim_acceptance` | `acceptance_claim_allowed` | accepting decision plus matching proof |

## Illegal transition examples

Any transition that bypasses contract approval, run receipt, gate decision, or proof must be denied and must not create forbidden artifacts.

## Expected denial behavior

Denied transitions should leave an inspectable denial reason. Where runtime events exist, they should write denial events or receipts according to the future event policy.

Denial must not write the artifact that the denied transition would have produced.

## Expected no-forbidden-artifact behavior

Illegal transitions must not create:

- run receipt before an allowed run;
- gate decision before required evidence;
- proofpack before gate decision;
- acceptance claim before matching proof;
- final decision from a module, adapter, proofpack, eval report, or context pack;
- public CLI exposure for parked capabilities.

## Event expectation for allowed transitions

When event runtime exists, allowed transitions should write append-only events with transition name, source state, target state, artifact refs, and rule/guard refs where applicable.

This spec does not activate event storage.

## Denial receipt/event expectation

Where applicable, denied transitions should record:

- attempted transition;
- current state;
- missing required precondition;
- forbidden artifact prevention;
- rule/guard ref;
- no-final-decision marker when denial is not a gate decision.

## Positive cases

### FCSM-001: approved contract can start cut

Given an approved-for-run contract with scope and hard-clause coverage, `cut_run` may move to `run_started`.

Expected result: allowed transition evidence.

### FCSM-002: run receipt enables gate review

Given a run receipt satisfying required fields, `gate_review` may write a gate decision.

Expected result: gate decision is the only final decision artifact.

### FCSM-003: proof follows gate decision

Given a gate decision, proofpack generation may link contract, receipt, eval, decision, outputs, hashes, and event range/root where applicable.

Expected result: proofpack available after decision.

## Required counterexamples

### FCSM-101: cut before approved contract

Attempting `cut` before approved-for-run contract status must be denied.

Expected result: no run receipt, no gate decision, no proofpack.

### FCSM-102: run without scope

A run attempt without explicit contract scope must be denied.

Expected result: no run receipt and denial reason records missing scope.

### FCSM-103: gate before run receipt

Gate review before required run receipt evidence must be denied.

Expected result: no gate decision.

### FCSM-104: proof before gate decision

Proofpack generation before gate decision must be denied.

Expected result: no proofpack.

### FCSM-105: acceptance claim before proof

Acceptance claim before matching proof must be denied.

Expected result: no acceptance claim.

### FCSM-106: module writes final decision

Any module attempt to write final decision must be denied.

Expected result: module output remains advisory assessment only.

### FCSM-107: adapter promotes retrieved text as project truth

Any adapter or retrieval output that promotes retrieved text into project truth must be denied.

Expected result: retrieval remains advisory evidence with receipt, if used.

### FCSM-108: eval report treated as final acceptance

An eval report cannot be final acceptance.

Expected result: eval remains assessment and gate still required.

### FCSM-109: proofpack used as decision surface

A proofpack cannot write or replace the gate decision.

Expected result: proofpack remains proof/provenance only.

### FCSM-110: parked capability exposed in public CLI

Parked capability exposure through public CLI must be denied or fail docs/CLI governance checks.

Expected result: no public CLI command for parked scope.

## Non-goals

This spec does not add new CLI behavior, runtime flow persistence, event storage, denial writer, gate writer, proof writer, adapter behavior, module behavior, or acceptance claim writer.

## Future implementation constraints

Future flow evals should be counterexample-first: every new allowed transition needs at least one denial case showing which forbidden artifact must not be produced.
