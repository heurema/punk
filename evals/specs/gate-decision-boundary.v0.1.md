# Gate decision boundary v0.1

Date: 2026-04-22
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that future Punk gate decisions must satisfy before any `gate`, `.punk/decisions`, proof linkage, validator, or CLI implementation begins.

This is a design/spec artifact only.

It clarifies how a future `DecisionObject` may later act as final closure authority without collapsing contract, flow, event, receipt, eval, or proof surfaces into hidden decision behavior.

## Status and authority

This is a proposed v0.1 boundary document.

It is not implementation.

It does not activate `gate`.

It does not activate `.punk/decisions`.

It does not define validators, runtime storage, or CLI behavior.

It does not become product truth by itself.

Future implementation requires a separate bounded goal and may require additional evals and an ADR if decision storage or proof linkage semantics become canonical architecture.

## Relationship to adjacent artifacts

### Relationship to contract

A gate decision closes work over bounded contract scope.

A future decision should reference the contract it closes.

A contract authorizes bounded work, but it is not the final closure artifact.

### Relationship to flow evidence

Flow evidence records allowed or denied lifecycle transitions and guard outcomes.

A gate decision may later reference relevant transition evidence, but it does not replace runtime transition validation.

Flow evidence remains evidence-only.

### Relationship to event log

The append-only event log records lifecycle facts and guard evidence.

A gate decision may later reference event ids, event ranges, or an event-root concept.

The event log is canonical lifecycle evidence, not final closure authority.

### Relationship to run receipt

A run receipt records what a bounded run did.

A gate decision may later reference one or more receipts.

Receipt evidence may exist before `gate`, but it does not imply final closure.

### Relationship to eval report

An eval report records assessment results.

A gate decision may later reference eval reports as evidence.

Eval remains assessment-only and does not decide.

### Relationship to future proofpack

A future proofpack may later package or reference verifiable evidence, including the gate decision.

A gate decision does not replace proofpack.

A proofpack does not replace gate decision.

They remain separate artifacts and may later cross-link explicitly.

### Relationship to acceptance

An accepting gate decision is required for acceptance, but it is not sufficient to claim acceptance until required proof exists.

See `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Gate decision principles

Future gate decision behavior must satisfy all of the following:

1. **Gate decision is future, not active now.**
   No `gate` runtime behavior is implied by this spec.

2. **Only `gate` writes final closure.**
   No earlier surface may write the future closure artifact.

3. **Gate decision is closure authority, not just evidence.**
   It records the final outcome over already-produced evidence.

4. **Prior artifacts remain evidence-only.**
   Contract, flow evidence, event evidence, run receipts, and eval reports do not become closure authority.

5. **Gate decision references prior artifacts rather than absorbing them.**
   A decision should point to evidence refs or hashes instead of embedding or replacing full evidence bodies.

6. **Gate decision does not replace proofpack.**
   A decision is the closure artifact; proofpack is later provenance over linked evidence.

7. **Proofpack remains distinct but may later link to gate decision and vice versa.**
   Cross-links are useful; artifact collapse is not.

8. **Acceptance requires proof.**
   A positive acceptance claim requires an accepting gate decision plus matching proof.

9. **Mutable `latest` must not be canonical truth.**
   Any later convenience view must stay derived and rebuildable from append-only decision artifacts.

## Minimum future DecisionObject metadata

The first future gate decision boundary should assume at least:

- `decision_id`;
- `decision_status` or `outcome`;
- `contract_refs` and contract-hash concept;
- `run_receipt_refs` and receipt-hash concept, if present;
- `eval_refs` and eval-hash concept, if present;
- `event_refs`, event range, or event-root concept, if applicable;
- `proof_refs` and proof-hash concept later, when that surface exists;
- `created_at`;
- `boundary_notes`.

A later implementation may also require:

- reason code;
- human-readable rationale;
- rule or guard-set reference;
- explicit proof obligation status.

Those additions should remain explicit rather than silently implied.

## Distinction from adjacent artifacts

### Contract

- contract = bounded authorization surface;
- gate decision = final closure artifact.

A contract defines scope.

It does not close the loop.

### Flow evidence

- flow evidence = transition and guard evidence;
- gate decision = closure authority.

Flow evidence explains what transitions were allowed or denied.

It does not finalize work.

### Event log

- event log = append-only lifecycle history;
- gate decision = one final closure artifact.

Event evidence records what happened.

Gate decision records what the project concludes from that evidence.

### Run receipt

- run receipt = bounded run evidence;
- gate decision = closure authority over evidence.

Receipt may precede gate.

Receipt does not accept work.

### Eval report

- eval report = assessment artifact;
- gate decision = closure artifact.

A green eval may support closure.

It must not be closure by itself.

### Proofpack

- proofpack = later provenance bundle;
- gate decision = final closure artifact.

Proofpack makes closure inspectable.

It does not become closure authority.

## Why only gate writes final closure

Only one artifact should answer the question:

`what actually closed this work?`

If closure authority leaks into receipts, eval reports, event evidence, or proof artifacts, Punk loses:

- a stable closure source of truth;
- clear replay and inspection semantics;
- clean separation between evidence and authority;
- safe storage boundaries for later `.punk/decisions` or derived views.

Keeping final closure in `gate` prevents earlier surfaces from silently becoming decision owners.

## Why gate decision references evidence instead of absorbing it

A future gate decision should stay small and inspectable.

It should reference prior artifacts rather than absorb them because absorption would:

- duplicate raw evidence bodies;
- blur evidence and authority boundaries;
- increase privacy and redaction risk;
- make derived summaries more tempting as mutable truth.

Reference-first design preserves artifact separation and later hash-based inspection.

## Non-goals

This v0.1 boundary does not define:

- implementation;
- `gate` execution logic;
- `.punk/decisions` creation;
- storage layout implementation;
- `DecisionObject` Rust types;
- proofpack implementation;
- validators;
- CLI commands;
- remote transparency logs or exports.

## Future implementation constraints

Any later implementation step should be explicit about:

- append-only decision semantics;
- outcome vocabulary;
- contract/receipt/eval/event/proof ref rules;
- hash/reference normalization;
- proof-before-acceptance semantics;
- how proof links remain explicit without collapsing artifact boundaries;
- how derived `latest` or inspect views remain rebuildable rather than canonical.

## Deferred for later specs or goals

Still deferred after this boundary spec:

- gate execution implementation;
- proofpack boundary details;
- `.punk/decisions` layout implementation;
- decision validators;
- signing or transparency-log behavior;
- actor identity hardening.
- acceptance claim semantics beyond `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Next bounded step

If design work proceeds conservatively, the next honest step is research-first on the proofpack boundary before any gate or proof implementation branch.
