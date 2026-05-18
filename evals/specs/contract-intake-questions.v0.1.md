# Contract Intake Questions v0.1

Date: 2026-05-18
Status: proposed boundary
Authority: advisory/design

## Purpose

Define eval cases for turning a raw user request into a safer contract draft
through forcing questions, assumptions, unknowns, scope narrowing, and readiness
classification.

This spec is inspired by gstack-style product interrogation, but maps it to
Punk's `plot` phase. It does not activate a new lifecycle phase, product coach,
LLM drafting command, executor brief generator, or public CLI behavior.

## Status and authority

Contract-intake questions are `plot` aids.

They are not contracts until a contract artifact exists and is approved for run
through the appropriate boundary.

## Required deterministic eval cases

### CONTRACT-INTAKE-001: raw request keeps source text

A contract-intake draft must preserve the raw user request ref or text summary
without treating it as accepted scope.

### CONTRACT-INTAKE-002: assumptions are explicit

Every inferred assumption must be listed separately from user-confirmed facts.

### CONTRACT-INTAKE-003: unknowns are explicit

Material unknowns must be listed with blocking/non-blocking status and next
action: clarify, research, defer, reject, or human review.

### CONTRACT-INTAKE-004: narrowest bounded slice is proposed

For broad or vague requests, the intake should propose the smallest bounded
slice that can produce evidence without activating out-of-scope behavior.

### CONTRACT-INTAKE-005: alternatives are advisory

Implementation alternatives may be listed, but they must not become accepted
scope until the selected option appears in the contract.

### CONTRACT-INTAKE-006: non-goals are first-class

Non-goals and denied behaviors must be captured before readiness for `cut`.

### CONTRACT-INTAKE-007: executor choice is not authority

A draft may mention possible executors or runner aids, but executor choice must
not become acceptance authority.

### CONTRACT-INTAKE-008: research requirement is surfaced

If the request touches core laws, flow semantics, eval policy, storage, module
interfaces, adapter boundaries, project memory, knowledge/retrieval, side-effect
policy, roadmap phase boundaries, or public CLI contract, the intake must mark
Research Gate requirement or explicit no-research rationale.

### CONTRACT-INTAKE-009: readiness blocks unresolved hard clauses

A draft cannot be ready for user confirmation or approved-for-run model state
while hard clauses lack validator refs, required receipt fields, proof
requirement refs, or explicit human-gate-review reason.

### CONTRACT-INTAKE-010: parked/future behavior is not silently promoted

If the user asks for parked or future behavior, the intake must mark it as
park/defer/avoid or require roadmap promotion rather than treating it as active.

## Minimal fixture shape

```yaml
contract_intake:
  intake_id: contract_intake_fixture_001
  raw_request_ref: user_request_fixture_001
  status: draft
  assumptions:
    - id: assumption_001
      text: The user wants docs/eval artifacts only.
      confirmed: false
  unknowns:
    - id: unknown_001
      text: Whether code implementation is required.
      blocking: true
      next_action: clarify
  proposed_slice:
    title: Capture runner-aid boundary docs and eval specs.
    active_scope: docs_eval_spec_only
  alternatives: []
  non_goals:
    - public CLI additions
  research_gate:
    required: true
    rationale: Persistent model-control artifact boundary.
  readiness: needs_confirmation
```

## Non-goals

This spec does not define a contract writer, `.punk/contracts` storage, LLM
contract drafting, CLI behavior, executor brief generation, gate writing,
proofpack writing, or acceptance claims.
