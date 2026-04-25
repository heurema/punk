# Proof-before-acceptance semantics v0.1

Date: 2026-04-25
Status: proposed boundary
Authority: advisory/design

## Purpose

Define how Punk should interpret `proof before acceptance` before any runtime `gate`, proofpack writer, `.punk/decisions`, `.punk/proofs`, schema, or CLI implementation exists.

This is a design/spec artifact only.

It protects the closure path:

```text
bounded evidence -> gate decision -> proofpack -> acceptance claim
```

from turning proofpack into a second decision surface or turning a gate decision into accepted work before proof exists.

## Status and authority

This is a proposed v0.1 semantics boundary.

It is not implementation.

It does not activate `gate`.

It does not activate proofpack generation.

It does not define runtime schemas, CLI commands, validators, storage layouts, or `.punk` writes.

Future implementation requires a separate bounded goal.

## Core reconciliation

Punk keeps both core laws:

```text
Only gate writes the final decision.
Proof comes before acceptance.
```

These are not in conflict if the artifacts stay separate:

- gate decision = the only final authority over the evidence;
- proofpack = post-decision provenance that makes the decision inspectable;
- acceptance claim = a project claim that requires both an accepting gate decision and matching proof.

A gate decision may be final authority.

It is not enough, by itself, to claim accepted work if required proof has not been produced.

A proofpack may make an accepting decision inspectable.

It cannot change the decision, accept work, or reject work by itself.

## Artifact sequence

The conservative first sequence is:

```text
goal
  -> contract
  -> approved bounded run
  -> receipt evidence
  -> eval / validator / semantic assessment evidence
  -> gate decision
  -> proofpack over decision and evidence
  -> acceptance claim
```

The sequence means:

1. **Contract scopes work.**
   The contract is authorization, not acceptance.
2. **Receipt records a bounded run.**
   A receipt is evidence, not proof or closure.
3. **Eval, validator, and semantic assessment outputs assess evidence.**
   They may support or block closure, but they do not decide.
4. **Gate writes the decision.**
   The decision is the only final authority over the evidence.
5. **Proofpack makes the decision inspectable.**
   The proofpack references the decision and evidence with stable refs and hashes.
6. **Acceptance may be claimed only after proof.**
   An acceptance claim requires an accepting gate decision and a matching proofpack.

## Decision versus acceptance

A future gate decision may record a closing outcome over bounded scope.

Acceptance is the project-level claim that the work can be treated as accepted.

For positive acceptance claims, Punk should require:

- an accepting gate decision ref;
- a proofpack ref for that decision;
- matching contract, receipt, eval/assessment, event, and output refs or hashes required by policy;
- no unresolved required evidence gaps that policy says block proof.

If an accepting decision exists but proof is missing, the safe project state is:

```text
decision written, acceptance not yet claimable
```

Reports and docs must not shorten that state to `accepted`.

## Negative or non-accepting outcomes

`proof before acceptance` applies to positive acceptance claims.

A rejecting, needs-work, blocked, or deferred gate decision does not become acceptance.

Such a decision may still need evidence refs and may later be included in a proofpack for auditability, but it does not satisfy acceptance.

A later implementation goal may decide whether every gate decision, including rejection, must produce a proofpack.

This spec only requires proof before positive acceptance claims.

## Same command, separate artifacts

A future `gate` command may eventually orchestrate both decision writing and proofpack generation.

That does not collapse the artifacts.

Even if one command produces both outputs, the logical order remains:

```text
gate writes decision -> proofpack references decision -> acceptance may be claimed
```

The proofpack must not be embedded as hidden decision state.

The decision must not silently absorb proofpack contents.

## Relationship to receipt evidence

Receipts feed gate/proof.

A receipt is valid if it records a bounded run with required refs and boundary notes.

A receipt is not proof of success.

A receipt is not acceptance.

A receipt must not carry fields that present receipt-local observations as final closure.

## Relationship to eval and validator outcomes

Eval reports and validator outcomes feed gate/proof.

`passed` validator evidence can support an accepting decision.

`failed`, `unavailable`, `skipped`, `unsupported`, and `deferred` outcomes may block proof or require explicit gate policy treatment.

Missing required evidence must stay visible.

Missing evidence must not be hidden by a proofpack.

## Relationship to semantic assessor evidence

Semantic assessors may provide clause-scoped advisory evidence.

They do not decide.

They do not produce proof by themselves.

A proofpack may later reference semantic assessment output only as evidence that the gate considered or policy allowed.

## Relationship to proofpack boundary

A proofpack is post-gate provenance.

It may reference:

- gate decision id and hash;
- contract id and hash;
- run receipt ids and hashes;
- eval/assessment refs and hashes;
- event range or root;
- output artifact refs and hashes;
- rule or guard version refs.

The proofpack makes a decision inspectable.

It does not decide.

## Relationship to Project Memory

Project Memory may link accepted work only when the required acceptance chain is present.

The chain should stay inspectable:

```text
goal -> contract -> receipt -> eval/assessment -> gate decision -> proofpack -> project memory/public narrative
```

A derived `latest`, index, or summary view must not become the acceptance source of truth.

## Required deterministic eval cases

### PBA-001: acceptance requires decision and proof

A positive acceptance claim must reference an accepting gate decision and a matching proofpack.

### PBA-002: decision without proof is not accepted

A gate decision that lacks required proofpack linkage must not be represented as accepted work.

### PBA-003: proofpack is not a decision

A proofpack must not contain authority fields that make it the final decision writer.

### PBA-004: proofpack is post-gate

A canonical proofpack must reference an existing gate decision.

Pre-gate draft proofpack semantics remain out of scope.

### PBA-005: evidence surfaces remain evidence

Receipts, eval reports, validator outcomes, semantic assessments, event evidence, and executor claims must not become acceptance by themselves.

### PBA-006: missing required evidence blocks proof claims

Required missing-validator outcomes that policy says block proof must not be hidden by proofpack or acceptance language.

### PBA-007: accepting decision and proofpack stay separate

A future implementation may produce decision and proofpack in one command, but the artifacts must remain separately referenceable.

### PBA-008: non-accepting decisions are not acceptance

Rejected, blocked, needs-work, or deferred decisions must not be represented as acceptance.

### PBA-009: manual Level 0 done is not future gate acceptance

Manual Work Ledger `done` remains manual closure with evidence.

It is not future `gate` acceptance.

### PBA-010: no runtime activation implied

Docs and reports that reference this semantics spec must not describe `gate`, proofpack writer, `.punk/decisions`, `.punk/proofs`, schemas, CLI commands, or runtime storage as active implementation.

## Non-goals

This v0.1 semantics spec does not define:

- `gate` implementation;
- gate outcome vocabulary;
- proofpack writer behavior;
- proofpack schema file;
- decision schema file;
- receipt schema implementation;
- validator implementation;
- semantic assessor implementation;
- `.punk/decisions` storage;
- `.punk/proofs` storage;
- CLI commands;
- `punk init`;
- provider/model/agent adapters;
- automation.

## Future implementation constraints

Any later implementation step should be explicit about:

- gate decision outcome vocabulary;
- proof obligation status;
- decision-proof cross-link rules;
- stable ref/hash normalization;
- how proofpack generation failure affects acceptance claims;
- whether non-accepting decisions require proofpacks;
- how missing-validator outcomes affect proof eligibility;
- how manual Level 0 `done` maps, or does not map, to future gate acceptance.

## Deferred for later specs or goals

Still deferred after this semantics spec:

- runtime gate implementation;
- proofpack writer implementation;
- decision/proof schema files;
- real `.punk` runtime storage;
- receipt schema/runtime implementation;
- semantic assessor implementation;
- possible future `punk init`.

## Next bounded step

After this proof-before-acceptance semantics spec, the conservative next move is a seventh advisory Work Ledger Review before selecting runtime storage, receipt schema/runtime, gate/proof implementation, docs/CLI mismatch, or another docs/spec branch.
