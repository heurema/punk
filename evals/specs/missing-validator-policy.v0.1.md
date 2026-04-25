# Missing-validator policy v0.1

Date: 2026-04-25
Status: proposed boundary
Authority: advisory/design

## Purpose

Define how Punk should classify validators that cannot provide normal evidence before any runtime gate, proofpack, storage, receipt expansion, semantic assessor implementation, or CLI implementation depends on validator outcomes.

This is a design/spec artifact only.

It protects the user flow:

```text
expected check -> validator outcome -> evidence gap or evidence -> later gate input
```

from silently converting missing local tools, skipped checks, or executor claims into proof.

## Status and authority

This is a proposed v0.1 policy boundary.

It is not implementation.

It does not activate validators.

It does not define CLI behavior, schemas, runtime storage, gate decisions, proofpacks, provider/model/agent adapters, semantic assessor commands, or `.punk/` writes.

Future implementation requires a separate bounded goal.

## Policy model

Punk is setup-neutral, but not validation-neutral.

That means:

- Punk must not force users to install tools or change their repo setup just to satisfy the process.
- Punk must not silently treat unavailable checks as passed.
- Punk must report missing validation honestly as evidence state.
- Future gate/proof behavior must consume missing-validator outcomes as evidence inputs, not final decisions.

Shorthand:

```text
missing is visible
missing is not passing
missing is not proof
```

## Validator outcome vocabulary

### `passed`

The validator ran against the intended scope and produced evidence that its checked condition passed.

Requirements:

- validator identity is known;
- checked scope is known;
- result source is captured;
- output is linkable or summarizable as evidence.

`passed` is still evidence, not final acceptance.

### `failed`

The validator ran and found a violation, failed assertion, non-zero check result, or unacceptable output.

Requirements:

- failure source is captured;
- checked scope is known;
- failure must not be rewritten as unavailable or skipped.

`failed` is evidence against acceptance. It is not a final decision by itself.

### `unavailable`

The validator could not run because a required command, dependency, runtime, credential, service, file, environment, or capability was missing or inaccessible.

Examples:

- command not found;
- dependency not installed;
- local service not running;
- required env var missing;
- platform unsupported at runtime;
- network/service unavailable for a check that needs it.

`unavailable` is an evidence gap.

It must not count as `passed`.

It must not force setup changes.

### `skipped`

The validator was intentionally not run even though it may have been available.

Requirements:

- skip reason is explicit;
- actor or policy source is explicit where available;
- requiredness is explicit.

`skipped` is not proof.

A skipped required validator remains an evidence gap unless a future policy explicitly allows the skip.

### `unsupported`

The validator is not applicable to the project, platform, language, artifact, or current contract scope.

Requirements:

- applicability reason is explicit;
- this must not be used to hide missing setup;
- unsupported status should be stable under repeated evaluation for the same scope.

`unsupported` may be acceptable as a non-run outcome only when the contract or policy makes the validator conditional or out of scope.

### `deferred`

The validator is intentionally left unresolved for later work or later gate policy.

Requirements:

- deferral reason is explicit;
- revisit trigger or follow-up owner is recorded where possible;
- deferral is visible in reports and future evidence bundles.

`deferred` is an open evidence gap.

It is not proof.

## Requiredness vocabulary

Validator outcomes should be interpreted with a requiredness level.

### `required`

The validator is expected for the current contract or policy.

A `required` validator with `failed`, `unavailable`, `skipped`, or unresolved `deferred` outcome must not be treated as proof of success.

Future gate policy may reject, request rework, or explicitly defer. It must not silently accept.

### `advisory`

The validator can provide useful evidence, but is not required for closure.

Missing advisory evidence should be visible, but it need not block every later decision.

### `conditional`

The validator is required only when a condition applies.

The condition must be explicit enough to explain `unsupported` or `skipped` outcomes.

## Minimal future evidence shape

This policy does not implement a schema, but future receipts/eval reports should be able to carry at least:

- `validator_id`;
- `validator_kind`;
- `requiredness`;
- `outcome`;
- `scope_ref`;
- `reason_code` for non-`passed` outcomes;
- `summary`;
- `evidence_ref` or output excerpt when safe;
- `observed_at`;
- `follow_up_ref` for `deferred` outcomes, if available.

Raw secrets, credentials, private env dumps, and provider payloads must not be captured by default.

## Future gate interpretation

Future `gate` remains the only final closure authority.

Validator outcomes are inputs to gate policy.

A future gate must not treat these as acceptance:

- executor claim that a validator passed;
- missing validator with no outcome;
- `unavailable` required validator;
- `skipped` required validator without allowed policy reason;
- `unsupported` validator without applicability reason;
- unresolved `deferred` required validator.

This spec does not define final gate outcome vocabulary.

## Future report and receipt interpretation

Reports and receipts may record validator outcomes as evidence.

They must keep these boundaries:

- report says what was observed;
- receipt says what a bounded run did;
- eval says what was assessed;
- future gate decides;
- future proofpack bundles evidence after gate.

A report line like `tests not run because command not found` is valid evidence.

It is not proof.

## Semantic assessor boundary

A semantic or LLM-based assessor can also be missing, unavailable, skipped, unsupported, failed, or deferred.

If the semantic assessor is unavailable, Punk must not substitute executor self-review as equivalent proof.

If an assessor runs, its output is advisory evidence unless a later accepted policy narrows that boundary.

Semantic assessor command interface v0.1 is defined in `evals/specs/semantic-assessor-command-interface.v0.1.md`.

## Required deterministic eval cases

### MV-001: unavailable does not pass

A required validator that cannot run because its command or dependency is missing must produce `unavailable`, not `passed`.

### MV-002: failed is distinct from unavailable

A validator that runs and returns a failing result must produce `failed`, not `unavailable`, `skipped`, or `deferred`.

### MV-003: skipped requires explicit reason

A skipped validator must include a skip reason and requiredness.

A skipped required validator remains an evidence gap unless an explicit future policy allows the skip.

### MV-004: unsupported requires applicability reason

An unsupported validator must explain why it does not apply to the current project, platform, artifact, or scope.

### MV-005: deferred requires follow-up context

A deferred validator must record why it is deferred and what should revisit it.

### MV-006: executor claim cannot replace validator evidence

An executor statement such as `tests passed` must not become a `passed` validator outcome unless the validator evidence is captured or verified.

### MV-007: setup-neutral means no forced installation

If a validator is unavailable, Punk may report the missing prerequisite and policy impact, but must not require setup changes as an implicit side effect of evaluation.

### MV-008: missing required evidence blocks proof claims

A required validator with `unavailable`, `skipped`, unresolved `deferred`, or unexplained `unsupported` outcome must not be represented as proof or final acceptance.

### MV-009: advisory missing evidence is visible

An advisory validator that is missing may be non-blocking later, but its absence must remain visible in evidence/report output.

### MV-010: semantic assessor unavailable is not self-review

If a semantic assessor is unavailable, executor self-review cannot be promoted to equivalent assessor evidence.

### MV-011: non-passed outcomes preserve scope refs

`failed`, `unavailable`, `skipped`, `unsupported`, and `deferred` outcomes must preserve enough scope information to know what was not validated.

### MV-012: no runtime activation implied

Docs and reports that reference this policy must not describe validators, gate, proofpack, `.punk/`, CLI, or semantic assessor runtime behavior as active unless a later implementation goal has activated it.

## Non-goals

This v0.1 policy does not define:

- validator implementation;
- validator registry;
- runtime schema;
- receipt schema;
- gate outcome vocabulary;
- proofpack writer behavior;
- CLI commands;
- `punk init`;
- semantic assessor command implementation;
- provider/model/agent adapters;
- `.punk/` storage;
- automation.

## Future implementation constraints

Any later implementation step should be explicit about:

- validator identity rules;
- requiredness source;
- scope refs;
- non-`passed` reason codes;
- safe output capture;
- redaction behavior;
- gate interpretation;
- report/receipt/proofpack links;
- setup-neutral user messaging.

## Deferred for later specs or goals

Still deferred after this policy spec:

- semantic assessor implementation;
- real gate runtime;
- real proofpack runtime;
- real `.punk/` runtime storage;
- possible future `punk init`.

## Next bounded step

Minimal receipt fields v0.1 are defined in `evals/specs/minimal-receipt-fields.v0.1.md` so future receipts can carry validator outcomes and evidence gaps without becoming gate decisions or proofpacks.
