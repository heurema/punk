# Minimal receipt fields v0.1

Date: 2026-04-25
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the minimal future receipt fields Punk needs before any receipt schema, `.punk/runs` storage, gate/proof runtime, semantic assessor implementation, or CLI implementation expands receipt behavior.

This is a design/spec artifact only.

It protects the user flow:

```text
bounded run -> receipt evidence -> later eval/gate/proof input
```

from turning receipts into final decisions, proofpacks, hidden storage truth, or executor self-reporting.

## Status and authority

This is a proposed v0.1 field boundary.

It is not implementation.

It does not change the current Rust receipt model.

It does not activate `.punk/runs`, receipt schema files, validators, gate decisions, proofpack writing, semantic assessors, or CLI behavior.

Future implementation requires a separate bounded goal.

## Relationship to current receipt boundary

`evals/specs/run-receipt-boundary.v0.1.md` already defines receipt as run evidence, not acceptance.

This spec narrows the first field set a future receipt schema should carry so receipts can reference:

- contract authorization;
- bounded run identity;
- run scope;
- event/eval evidence;
- produced artifacts;
- validator outcomes;
- missing-validator evidence gaps;
- boundary notes.

The current minimal Rust kernel already models a subset of these ideas. This spec does not change that code.

## Field groups

### Receipt identity

A future receipt should carry:

- `receipt_id`;
- `schema_version`;
- `produced_at`;
- `boundary_notes`.

Purpose:

- identify one receipt artifact;
- make schema evolution visible;
- record when the receipt was produced;
- preserve explicit deferrals or boundary caveats.

### Contract and run refs

A future receipt should carry:

- `contract_ref`;
- `contract_hash` or equivalent artifact-hash concept, when hashing is available;
- `run_id`;
- `run_scope_ref`;
- `work_goal_ref`, if available;
- `flow_state_before`;
- `flow_state_after`, if applicable.

Purpose:

- prove the receipt belongs to a bounded run;
- tie the run to authorized scope;
- keep receipt below contract and flow semantics.

### Event and eval refs

A future receipt should carry:

- `event_refs` or event-range/root concept, if available;
- `eval_report_refs`, if available.

Purpose:

- link lifecycle evidence;
- link assessment evidence;
- avoid embedding or replacing the event log or eval report.

### Validator outcome refs

A future receipt should carry zero or more validator outcome entries.

Each validator outcome should align with `evals/specs/missing-validator-policy.v0.1.md` and carry at least:

- `validator_id`;
- `validator_kind`;
- `requiredness`;
- `outcome`;
- `scope_ref`;
- `reason_code` for non-`passed` outcomes;
- `summary`;
- `evidence_ref` or safe output excerpt;
- `observed_at`;
- `follow_up_ref`, if outcome is `deferred`.

Allowed outcomes:

- `passed`;
- `failed`;
- `unavailable`;
- `skipped`;
- `unsupported`;
- `deferred`.

Purpose:

- keep missing validation visible;
- keep validator output scoped;
- avoid converting missing checks into proof.

### Produced artifact refs

A future receipt should carry:

- `produced_artifact_refs`;
- optional artifact hash concept, when hashing is available;
- optional redaction or sensitivity notes.

Purpose:

- show what the run produced;
- keep produced outputs separate from the receipt body;
- avoid pulling secrets or large raw payloads into receipts.

### Runner or command refs

A future receipt may carry:

- `runner_ref`;
- `command_ref`;
- `toolchain_ref`;
- `environment_summary_ref`.

These should be optional until actor/runner identity is stable.

They describe execution substrate.

They do not create authority.

## Explicit non-fields

A future receipt must not carry fields that make it look like final closure authority.

Avoid receipt fields such as:

- `accepted`;
- `approved`;
- `gate_outcome`;
- `final_decision`;
- `proof_status`;
- `proofpack_finalized`;
- `merged`;
- `done` as final truth.

If later receipt summaries need status-like words, they must be clearly labeled as receipt-local evidence state, not gate closure.

## Missing-validator handling

Receipts may record missing validators.

They must preserve the difference between:

- missing because unavailable;
- missing because skipped;
- not applicable because unsupported;
- intentionally deferred;
- failed after running.

A receipt with missing required validator evidence may still be a valid receipt.

It is not proof of success.

It does not imply acceptance.

## Authority boundary

Receipt fields are evidence fields.

They do not decide.

```text
receipt = what a bounded run did and observed
gate = future closure authority
proofpack = future provenance bundle after gate
```

Receipt may be referenced by gate or proofpack later.

Receipt does not absorb or replace either one.

## Required deterministic eval cases

### RECEIPT-FIELDS-001: receipt has identity

A future receipt schema must include `receipt_id`, `schema_version`, and `produced_at` or equivalent explicit identity/version/time fields.

### RECEIPT-FIELDS-002: receipt links contract and run

A future receipt must include `contract_ref`, `run_id`, and `run_scope_ref` or equivalent fields.

### RECEIPT-FIELDS-003: receipt can link event and eval evidence

A future receipt must be able to reference event evidence and eval reports without embedding or replacing those artifacts.

### RECEIPT-FIELDS-004: receipt carries validator outcomes

A future receipt must be able to carry validator outcomes using the missing-validator policy vocabulary.

### RECEIPT-FIELDS-005: missing required validator remains evidence gap

A receipt with `unavailable`, `skipped`, unresolved `deferred`, or unexplained `unsupported` required validator outcome must not be represented as proof or acceptance.

### RECEIPT-FIELDS-006: failed validator remains distinct

A validator that ran and failed must remain `failed`, not be rewritten as missing or deferred.

### RECEIPT-FIELDS-007: produced artifacts are refs

Produced artifacts should be referenced through refs and optional hash concepts rather than absorbed as full raw payloads by default.

### RECEIPT-FIELDS-008: receipt does not include final decision fields

A future receipt schema must not include final closure fields such as `accepted`, `approved`, `gate_outcome`, or `final_decision` as receipt authority.

### RECEIPT-FIELDS-009: denied transitions do not create receipts

Denied transitions remain event/guard evidence and must not produce run receipts.

### RECEIPT-FIELDS-010: no runtime activation implied

Docs and reports that reference this field spec must not describe `.punk/runs`, receipt schemas, validators, gate, proofpack, or CLI receipt commands as active implementation.

### RECEIPT-FIELDS-011: safe output capture

Receipt fields must allow safe summaries or refs without requiring raw secrets, credentials, environment dumps, or provider payloads.

### RECEIPT-FIELDS-012: boundary notes preserve deferrals

A future receipt should preserve boundary notes for explicit deferrals, missing actor/runner identity, or unsupported validator contexts.

## Non-goals

This v0.1 spec does not define:

- receipt schema implementation;
- Rust type changes;
- `.punk/runs` storage;
- validator implementation;
- gate behavior;
- proofpack writing;
- semantic assessor command implementation;
- CLI commands;
- `punk init`;
- provider/model/agent adapters;
- automation.

## Future implementation constraints

Any later implementation step should be explicit about:

- append-only receipt semantics;
- field versioning;
- ref/hash normalization;
- validator outcome embedding vs referencing;
- safe output capture and redaction;
- runner identity deferral;
- boundary notes;
- how receipts remain evidence below gate/proof.

## Deferred for later specs or goals

Still deferred after this field spec:

- semantic assessor implementation;
- receipt schema/runtime implementation;
- real gate runtime;
- real proofpack runtime;
- real `.punk` runtime storage;
- possible future `punk init`.

## Next bounded step

Semantic assessor command interface v0.1 is defined in `evals/specs/semantic-assessor-command-interface.v0.1.md`.

The conservative next move is a sixth advisory Work Ledger Review before selecting runtime storage, receipt schema, gate/proof, or docs/CLI mismatch work.
