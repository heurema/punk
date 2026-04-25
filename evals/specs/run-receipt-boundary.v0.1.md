# Run receipt boundary v0.1

Date: 2026-04-22
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that future Punk run receipts must satisfy before any `.punk/runs` implementation or receipt schema work begins.

This is a design/spec artifact only.

It clarifies how future run receipts may later exist as inspectable evidence without becoming storage truth, gate output, or proofpack replacement.

## Status and authority

This is a proposed v0.1 boundary document.

It is not implementation.

It does not activate `.punk/runs`.

It does not define receipt schema files or validators.

It does not become product truth by itself.

Future implementation requires a separate bounded goal and may require additional evals and an ADR if receipt storage semantics become canonical architecture.

## Relationship to adjacent artifacts

### Relationship to contract

A run receipt belongs to an actual bounded run that is authorized by contract scope and lifecycle state.

A receipt should reference the contract that authorized the run.

A receipt is not the contract itself.

### Relationship to flow transition

Flow transitions describe lifecycle movement and guard outcomes.

A run receipt may later reference relevant flow states or events around a run, but it does not replace transition validation.

Denied transitions remain guard evidence and do not become receipts.

### Relationship to event log

The append-only event log records lifecycle events, including denials and later committed transitions.

A run receipt is a distinct run evidence artifact.

It may summarize or reference event ids or event ranges, but it must not duplicate the entire event log or replace it as canonical lifecycle history.

### Relationship to eval report

An eval report assesses behavior.

A run receipt records what a run actually did.

Receipts and eval reports may later cross-reference each other, but they must remain separate evidence surfaces.

### Relationship to future gate decision

A run receipt may exist before `gate`.

It does not accept work.

It does not write a final decision.

Future `gate` may later reference a receipt by id/hash as evidence only.

### Relationship to future proofpack

A future proofpack may later include receipt refs or hashes.

A receipt is only one evidence artifact inside a later proof story.

It is not itself a proofpack.

## Receipt principles

Future run receipt behavior must satisfy all of the following:

1. **Receipt is future, not active now.**
   No `.punk/runs` runtime behavior is implied by this spec.

2. **Receipt is an evidence artifact, not a decision.**
   A receipt records what a run did. It must not accept work or replace `gate`.

3. **Receipt can exist before gate.**
   It belongs to the execution path before final decision writing.

4. **Receipt does not imply final acceptance.**
   Receipt existence or green-looking output does not close the loop.

5. **Denied transitions produce event evidence only, not receipts.**
   Guard denials stay in the append-only event layer until a real bounded run exists.

6. **Receipt should correspond to an actual bounded run.**
   It should not be written for draft-only planning or denied transition previews.

7. **Future receipts should be append-only and run-scoped.**
   Each run should produce its own receipt artifact rather than mutating prior history.

8. **Mutable `latest` must not be canonical truth.**
   A convenience pointer or derived summary may exist later, but canonical receipt truth must remain the append-only run-scoped artifact set.

## Minimum future receipt metadata

The first future receipt boundary should assume at least:

- `receipt_id`;
- `contract_ref`;
- `contract_hash` or artifact-hash concept;
- `run_id`;
- `run_scope`;
- `flow_state_before`;
- `flow_state_after`, if applicable;
- `event_refs`;
- `eval_report_refs`, if available;
- `produced_artifact_refs`;
- `produced_at`;
- command or runner ref, if applicable;
- `boundary_notes`.

If no stable actor or runner identity model exists yet, actor identity may be deferred in the first implementation step, but the deferral must be explicit.

Field boundary v0.1 is defined in `evals/specs/minimal-receipt-fields.v0.1.md`. That spec is advisory/design only and does not activate receipt schema or `.punk/runs` storage.

## Distinction from adjacent artifacts

### Event log

- event log = append-only lifecycle history;
- receipt = one run's execution evidence artifact.

The event log records denials, transitions, and lifecycle facts.

The receipt records what a specific run actually did and produced.

### Eval report

- eval report = assessment artifact;
- receipt = execution artifact.

Eval may later assess a run or reference its outputs, but receipt and eval must not collapse into one artifact.

### Proofpack

- receipt = one evidence artifact;
- proofpack = later provenance bundle over multiple artifacts.

Proofpack may later reference receipt hashes or refs.

Receipt must not pretend to be a proof manifest.

### Gate decision

- receipt = pre-gate evidence;
- gate decision = final decision artifact.

Only `gate` writes closure.

## Non-goals

This v0.1 boundary does not define:

- implementation;
- `.punk/runs` creation;
- file writes;
- receipt storage layout implementation;
- receipt schema implementation;
- validators;
- gate/proof implementation;
- CLI receipt commands;
- remote transparency logs or export formats.

## Future implementation constraints

Any later implementation step should be explicit about:

- append-only semantics;
- run-scoped layout semantics;
- receipt id and run id rules;
- contract ref/hash expectations;
- event ref expectations;
- produced artifact ref expectations;
- how derived `latest` or inspect views remain rebuildable rather than canonical.

## Why denied transitions do not write receipts

Denied transitions are important evidence, but they are not runs.

They belong in the append-only event log because:

- the run never actually starts;
- no run artifact set is produced;
- treating denials as receipts would blur guard evidence and execution evidence;
- later gate/proof references need a stable distinction between attempted transition evidence and actual run evidence.

## Why receipt is not a decision surface

A receipt must stay below `gate` because:

- execution evidence does not answer whether work is finally accepted;
- a run may complete with warnings, failed assessments, or missing proof;
- only `gate` can weigh receipt, eval, proof, and other evidence into a final decision.

## Deferred for later specs or goals

Still deferred after this boundary spec:

- `.punk/runs` layout implementation;
- receipt schema implementation;
- receipt validator rules;
- gate/proof reference mechanics beyond evidence-only linking;
- actor identity hardening;
- artifact hashing requirements, if not yet stable.

## Next bounded step

Given the recent contract and receipt-boundary sequence, the next conservative step is a third advisory Work Ledger Review before choosing the next implementation branch.
