# Eval storage boundary v0.1

Date: 2026-04-22
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that future Punk eval storage must satisfy before any `.punk/evals` implementation begins.

This is a design/spec artifact only.

It clarifies when eval storage may later be introduced and what constraints must hold so that stored eval reports remain evidence rather than hidden mutable truth.

## Status and authority

This is a proposed v0.1 boundary document.

It is not implementation.

It does not activate `.punk/evals`.

It does not become product truth by itself.

Future implementation requires a separate bounded goal and may require additional evals and an ADR if storage semantics become canonical architecture.

## Relationship to adjacent artifacts

### Relationship to `smoke-eval-report.v0.1`

`evals/specs/smoke-eval-report.v0.1.md` defines the proposed machine-readable smoke report shape.

This boundary spec does not replace that schema.

Instead, it defines the conditions under which artifacts shaped like that report may later be stored.

### Relationship to current `SmokeEvalReport`

The current Rust `SmokeEvalReport` remains the canonical in-code shape for the active smoke harness.

This boundary spec does not change the in-code representation.

### Relationship to future `.punk/evals`

`.punk/evals` is future runtime/derived storage.

It is not active now.

If activated later, it must store local evidence artifacts without becoming a second decision surface.

### Relationship to future gate/proof

Future gate/proof may later reference stored eval artifacts by id and hash.

Eval storage must not itself write acceptance, decision, or proof state.

## Storage boundary principles

Future eval storage must satisfy all of the following:

1. **Storage is future, not active now.**
   No `.punk/evals` runtime behavior is implied by this spec.

2. **Stored eval reports are evidence, not decisions.**
   A stored report may describe assessment results, but it must not accept work, approve work, or replace `gate`.

3. **Storage must be append-only.**
   New eval runs should add new artifacts rather than mutate prior runs.

4. **Storage must be run-scoped.**
   Each stored report should belong to a specific eval run identity rather than a global mutable slot.

5. **Mutable `latest` must not be canonical truth.**
   A convenience pointer or derived view may exist later, but canonical truth must remain the append-only run-scoped artifact set.

6. **Every stored report must identify schema and version.**
   Stored artifacts must make their schema/version explicit.

7. **Every stored report should have stable run identity.**
   Future storage should use a stable `run_id` or equivalent identifier.

8. **Stored reports should later be hash-referenceable.**
   Future gate/proof should be able to reference stored reports by id/hash without copying their full contents into decision state.

## Minimum future stored report metadata

The first future stored report artifact should carry at least:

- `schema_version`;
- `suite_id`;
- `run_id`;
- `produced_at`.

If `produced_at` is deferred in a future step, that step must explicitly justify the deferral.

A future stored report should also carry:

- command or run source identity, such as the invoking CLI surface or runner identity;
- report-hash or artifact-hash concept;
- `case_results`;
- `boundary_notes`.

These fields exist to support later correlation and inspectability, not to turn the report into a decision object.

## Baseline boundary

Baseline comparison is future work.

When it is introduced later, it must obey these constraints:

- a baseline must reference an explicit selected report or report artifact;
- a baseline must not mean mutable latest;
- a baseline pass must not mean gate acceptance;
- baseline comparison should stay on deterministic fields only.

This boundary spec does not define the baseline artifact or baseline comparison algorithm.

## Waiver boundary

Waiver handling is future work.

When it is introduced later, it must obey these constraints:

- a waiver requires explicit scope;
- a waiver requires a reason;
- a waiver requires an owner or approver;
- a waiver requires expiry or a revisit trigger;
- a waiver is not a silent ignore;
- a waiver does not weaken Punk laws or change assessment-vs-decision boundaries.

This boundary spec does not define the waiver artifact format.

## Non-goals

This v0.1 boundary does not define:

- `.punk/evals` implementation;
- file writes;
- report persistence behavior;
- baseline comparison implementation;
- waiver ledger implementation;
- validators;
- gate/proof implementation;
- remote storage;
- dashboarding or coherence scoring.

## Future implementation constraints

Any later implementation step should be explicit about:

- append-only storage semantics;
- run-scoped layout semantics;
- schema/version requirements;
- run identity requirements;
- hash/reference requirements for later gate/proof integration;
- canonical artifact vs derived convenience view separation.

## Why mutable `latest` is not allowed as truth

A mutable `latest` report is convenient but unsafe as canonical truth because it:

- overwrites history;
- hides comparison context;
- makes baseline selection implicit;
- makes future proof references unstable.

If a `latest` view exists later, it should be derived and rebuildable from append-only stored artifacts.

## Deferred for later specs

Still deferred after this boundary spec:

- baseline boundary design details;
- waiver boundary design details;
- `.punk/evals` layout implementation;
- stored report retention policy;
- gate/proof reference mechanics beyond the requirement that they be hash/id based.

## Next bounded step

If design work proceeds, the next honest step is a separate baseline/waiver boundary spec.
