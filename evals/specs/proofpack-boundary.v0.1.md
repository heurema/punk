# Proofpack boundary v0.1

Date: 2026-04-22
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that future Punk proofpacks must satisfy before any proofpack writer, `.punk/proofs`, validator, signing, transparency-log, or CLI implementation begins.

This is a design/spec artifact only.

It clarifies how a future proofpack may later act as a provenance and evidence bundle without becoming a second decision surface or hidden storage truth.

## Status and authority

This is a proposed v0.1 boundary document.

It is not implementation.

It does not activate proofpack generation.

It does not activate `.punk/proofs`.

It does not define validators, runtime storage, signing, transparency logs, or CLI behavior.

It does not become product truth by itself.

Future implementation requires a separate bounded goal and may require additional evals and an ADR if proof storage, signing, or hash normalization semantics become canonical architecture.

## Relationship to adjacent artifacts

### Relationship to contract

A proofpack may later reference the contract that scoped the work.

The contract remains the bounded authorization surface.

It is not the provenance bundle.

### Relationship to flow and event evidence

Flow evidence and the append-only event log record what transitions and lifecycle facts happened.

A proofpack may later reference event ids, event ranges, or an event-root concept.

Flow and event evidence remain primary history surfaces, not provenance bundles.

### Relationship to run receipt

A run receipt records bounded run evidence.

A proofpack may later reference one or more receipts.

Receipt stays pre-gate-capable evidence.

Proofpack does not replace receipt semantics.

### Relationship to eval report

An eval report records assessment evidence.

A proofpack may later reference eval reports and their hashes.

Eval remains assessment-only and does not become provenance authority by itself.

### Relationship to gate decision

A gate decision is the future closure authority.

A proofpack may later reference the gate decision that closed the work.

A proofpack does not decide.

A gate decision does not become the proofpack.

They remain separate artifacts and may later cross-link explicitly.

Positive acceptance claims require both an accepting gate decision and matching proof.

See `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

### Relationship to output artifacts

A proofpack may later reference the produced output artifacts and their hashes.

Those outputs stay separate artifacts.

Proofpack makes them inspectable in context.

It does not absorb or replace them.

## Proofpack principles

Future proofpack behavior must satisfy all of the following:

1. **Proofpack is future, not active now.**
   No runtime proof behavior is implied by this spec.

2. **Proofpack is a separate evidence and provenance bundle.**
   It organizes linked evidence for inspection.

3. **Proofpack is not the closure authority.**
   `gate` remains the only future closure authority.

4. **Proofpack references artifacts rather than absorbing them.**
   It should point to stable refs and hashes instead of embedding full artifact bodies when direct embedding is not strictly required.

5. **Canonical proofpack is post-gate.**
   The first safe canonical model is a proofpack that exists after the gate decision is available.

6. **Any pre-gate draft notion is out of scope for now.**
   This avoids early overlap with receipt, eval, and event semantics.

7. **Future proofpacks should be versioned and hash-referenceable.**
   Stable schema versioning and hash refs matter more than narrative summaries.

8. **Mutable `latest` must not be canonical truth.**
   Any later convenience pointer or summary must stay derived and rebuildable from append-only proof artifacts.

9. **Proofpack and gate decision remain separate artifacts.**
   Later cross-links are useful; artifact collapse is not.

10. **Proofpack must stay narrow about sensitive content.**
    Raw provider payloads, secrets, environment details, and hidden storage bodies must not be pulled in by default.

11. **Proofpack enables acceptance claims but does not decide.**
    A matching proofpack is required before positive acceptance can be claimed, but `gate` remains the decision authority.

## Why canonical proofpack is post-gate

The first canonical proofpack should exist only after gate because that ordering keeps the surfaces clear:

- receipt can exist before gate as bounded run evidence;
- eval can exist before gate as assessment evidence;
- event evidence can exist before gate as lifecycle history;
- gate decision later closes the work;
- proofpack then bundles the already-linked evidence around that closure.

If proofpack appears canonically before gate, it risks becoming either:

- a duplicate receipt surface;
- a hidden pre-decision checklist;
- or an implicit second decision artifact.

## Why proofpack references artifacts instead of absorbing them

A proofpack should stay small, inspectable, and replay-friendly.

Reference-first design avoids:

- duplicating full evidence bodies;
- blurring provenance and authority;
- creating hidden storage truth;
- expanding privacy and redaction risk;
- making narrative summaries look canonical.

Reference-oriented proof artifacts also fit later hash checking more naturally.

## Minimum future proofpack metadata

The first future proofpack boundary should assume at least:

- `proofpack_id`;
- `schema_version`;
- `gate_decision_ref` and decision-hash concept;
- `contract_ref` and contract-hash concept;
- `run_receipt_refs` and receipt-hash concept, if present;
- `eval_refs` and eval-hash concept, if present;
- `event_range` or event-root concept, if applicable;
- `output_artifact_refs` and output-hash concept;
- `created_at`;
- `boundary_notes`.

A later implementation may also require:

- project identity;
- rule or guard-set refs;
- privacy flags;
- verification outcome summary for ref/hash consistency;
- toolchain or generator metadata.

Those additions should remain explicit rather than implied.

## Distinction from adjacent artifacts

### Event log

- event log = append-only lifecycle history;
- proofpack = provenance bundle over linked evidence.

The event log records what happened.

Proofpack makes a selected closure inspectable.

### Run receipt

- run receipt = bounded run evidence;
- proofpack = post-gate provenance bundle over multiple evidence surfaces.

Receipt can exist before gate.

Proofpack does not replace receipt.

### Eval report

- eval report = assessment artifact;
- proofpack = evidence bundle that may reference assessment artifacts.

Eval says what was assessed.

Proofpack links that assessment into a larger closure context.

### Gate decision

- gate decision = closure authority;
- proofpack = provenance bundle.

Gate answers whether work was closed.

Proofpack answers how that closure can be inspected.

## Non-goals

This v0.1 boundary does not define:

- implementation;
- proofpack writer behavior;
- `.punk/proofs` storage layout;
- validators;
- signing;
- transparency logs;
- hash normalization algorithm details;
- gate or proof code;
- CLI commands;
- remote attestation exports.

## Future implementation constraints

Any later implementation step should be explicit about:

- append-only proofpack semantics;
- decision-proof cross-link rules;
- what counts as stable artifact refs and hashes;
- how output artifacts are identified;
- how privacy-sensitive content stays out of proof artifacts;
- how derived `latest` or inspect views stay rebuildable instead of canonical.

## Deferred for later specs or goals

Still deferred after this boundary spec:

- proofpack implementation;
- `.punk/proofs` activation;
- proofpack validators;
- signing or transparency-log behavior;
- draft-vs-final proofpack modeling;
- hash normalization details;
- full acceptance claim schema;
- gate implementation;
- output artifact export formats.

## Next bounded step

After this proofpack boundary spec, the next conservative move is another advisory Work Ledger Review before selecting a new implementation or process branch.
