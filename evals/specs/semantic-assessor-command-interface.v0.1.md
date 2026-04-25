# Semantic assessor command interface v0.1

Date: 2026-04-25
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary for a future semantic assessor command interface before any semantic assessor implementation, model/provider adapter, CLI command, runtime schema, `.punk` storage, gate behavior, or proofpack behavior exists.

This is a design/spec artifact only.

It protects the user flow:

```text
contract clause -> scoped evidence refs -> semantic assessment -> advisory evidence -> later gate input
```

from turning semantic review into hidden acceptance, executor self-review, or model-owned truth.

## Status and authority

This is a proposed v0.1 command-interface boundary.

It is not implementation.

It does not activate a `punk` CLI command.

It does not define a model runner, provider adapter, prompt format, schema file, semantic judge service, gate decision, proofpack writer, or `.punk` storage.

Future implementation requires a separate bounded goal.

## Interface model

A future semantic assessor is a validator-like evidence producer.

It should assess one bounded question against scoped evidence.

It must not decide acceptance.

It must not replace deterministic validators.

It must not use hidden chat or executor memory as authority.

Shorthand:

```text
assess a clause
cite evidence
stay advisory
```

## Future input boundary

A future semantic assessment request should carry at least:

- `request_id`;
- `schema_version`;
- `assessor_id` or requested assessor profile;
- `assessment_kind`;
- `contract_ref`;
- `contract_clause_ref`;
- `contract_clause_text` or stable clause hash/ref;
- `scope_ref`;
- `evidence_refs`;
- `receipt_refs`, if available;
- `artifact_refs`, if available;
- `policy_refs`;
- `question`;
- `allowed_context_refs`;
- `redaction_notes` or sensitivity flags;
- `requested_at`.

The input should be reference-first.

Raw secrets, credentials, private env dumps, provider payloads, and unconstrained chat history must not be passed by default.

## Future output boundary

A future semantic assessment result should carry at least:

- `assessment_id`;
- `request_id`;
- `schema_version`;
- `assessor_id`;
- `assessor_kind`;
- `validator_outcome` using missing-validator policy vocabulary;
- `assessment_result`, if the assessor ran;
- `contract_clause_ref`;
- `scope_ref`;
- `evidence_refs_used`;
- `summary`;
- `rationale_refs` or short rationale;
- `confidence` or uncertainty marker, if produced;
- `limitations`;
- `assessor_relation` or independence note, when known;
- `observed_at`;
- `boundary_notes`.

`validator_outcome` uses:

- `passed`;
- `failed`;
- `unavailable`;
- `skipped`;
- `unsupported`;
- `deferred`.

`assessment_result` should stay clause-local and advisory, for example:

- `supports_clause`;
- `contradicts_clause`;
- `inconclusive`;
- `not_applicable`.

The exact result vocabulary may be narrowed by a later implementation goal.

## Missing assessor behavior

If the assessor cannot run, the result must use the missing-validator policy vocabulary.

Examples:

- command/model/provider unavailable -> `unavailable`;
- assessor intentionally not run -> `skipped`;
- clause not suitable for semantic assessment -> `unsupported`;
- assessment intentionally postponed -> `deferred`;
- assessor runs and reports a violation or unusable output -> `failed`.

An unavailable semantic assessor must not be replaced by executor self-review.

A skipped semantic assessor must not be hidden from reports.

## Advisory-only boundary

Semantic assessor output is evidence.

It is not:

- gate decision;
- proof;
- proofpack;
- acceptance;
- merge approval;
- executor self-review;
- project truth by itself.

Future `gate` may consume semantic assessment refs as evidence.

Future proofpack may reference semantic assessment refs after gate.

Neither future surface is activated by this spec.

## Same-executor and same-model risk

If the assessor is the same executor, same session, same model, or same provider path that produced the work, the result must be marked with an `assessor_relation` or limitation.

Same-source assessment may still be useful as advisory evidence.

It must not be treated as independent review unless a later accepted policy explicitly allows that classification.

## Required deterministic eval cases

### SEM-IFACE-001: clause ref required

A semantic assessment request must identify the contract clause or bounded question being assessed.

Global `looks good` reviews are not sufficient interface evidence.

### SEM-IFACE-002: evidence refs required

A semantic assessment request must use explicit evidence refs or allowed context refs.

Hidden chat memory or executor-local memory must not be the authority source.

### SEM-IFACE-003: output is advisory

A semantic assessment result must not contain final decision fields such as `accepted`, `approved`, `gate_outcome`, or `final_decision` as assessor authority.

### SEM-IFACE-004: missing assessor follows missing-validator policy

Unavailable, skipped, unsupported, failed, and deferred semantic assessor outcomes must follow `evals/specs/missing-validator-policy.v0.1.md`.

### SEM-IFACE-005: unavailable assessor is not self-review

If the semantic assessor is unavailable, executor self-review cannot be promoted to equivalent semantic assessment evidence.

### SEM-IFACE-006: same-source assessor is marked

If assessor and executor share the same source, model, provider path, or session, the output must mark that relationship or limitation.

### SEM-IFACE-007: no provider lock-in

The interface must not require a specific model, provider, IDE, skill, or agent runtime as the authority path.

### SEM-IFACE-008: reference-first context

The interface should pass artifact refs, evidence refs, and allowed context refs rather than raw unbounded project state by default.

### SEM-IFACE-009: safe output capture

Semantic assessment output must not require raw secrets, credentials, private env dumps, or provider payloads.

### SEM-IFACE-010: receipt linkage is evidence-only

A future receipt may reference semantic assessment results, but that linkage must not make the receipt a gate decision or proofpack.

### SEM-IFACE-011: gate remains final closure authority

Future gate may weigh semantic assessment evidence, but semantic assessment output itself must not close work.

### SEM-IFACE-012: no runtime activation implied

Docs and reports that reference this interface must not describe semantic assessor commands, model/provider adapters, CLI behavior, gate, proofpack, or `.punk` storage as active implementation.

## Non-goals

This v0.1 interface does not define:

- command implementation;
- CLI syntax;
- JSON schema file;
- model runner;
- provider adapter;
- prompt template;
- semantic judge service;
- deterministic validator replacement;
- receipt schema implementation;
- gate behavior;
- proofpack writing;
- `.punk` storage;
- `punk init`;
- automation.

## Future implementation constraints

Any later implementation step should be explicit about:

- input and output schema versioning;
- clause-ref rules;
- evidence-ref rules;
- missing-assessor outcomes;
- same-source assessor disclosure;
- safe context limits;
- redaction behavior;
- how receipts reference semantic assessment output;
- how gate consumes semantic evidence without delegating final decision.

## Deferred for later specs or goals

Still deferred after this interface spec:

- semantic assessor implementation;
- model/provider adapters;
- receipt schema/runtime implementation;
- real gate runtime;
- real proofpack runtime;
- real `.punk` runtime storage;
- possible future `punk init`.

## Next bounded step

After this semantic assessor interface spec, the conservative next move is a sixth advisory Work Ledger Review before selecting runtime storage, receipt schema, gate/proof, or docs/CLI mismatch work.
