# Review Assessment receipt v0.1

Date: 2026-05-18
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary for advisory review assessment artifacts before Punk
promotes code review, security review, QA review, docs-drift review,
design/DX review, second-opinion review, or release-readiness review behavior.

This is a design/spec artifact only. It does not activate a public review CLI,
DevPunk, Module Host runtime, provider adapters, browser QA, security scanners,
automatic docs rewrite, gate writing, proofpack writing, or acceptance claims.

## Status and authority

A review assessment is evidence.

It is not a gate decision.

## Required deterministic eval cases

### REVIEW-ASSESS-001: assessment has contract ref

A review assessment without a contract ref or explicit maintenance reason must
not be eligible as gate input.

### REVIEW-ASSESS-002: findings map to clause or scope refs

Every material finding must map to a contract clause, scope boundary, validator,
receipt field, proof requirement, or explicit human-gate-review reason.

### REVIEW-ASSESS-003: input artifacts are explicit

An assessment must list the artifacts it reviewed. A finding over an implicit or
hidden input surface is invalid for gate-input eligibility.

### REVIEW-ASSESS-004: reviewer claims are unverified by default

Claims such as `tests passed`, `bug fixed`, `safe`, `secure`, `ready`, or
`complete` must default to `unverified` unless linked to validator/eval/receipt
evidence or explicit human gate review.

### REVIEW-ASSESS-005: final decision wording is blocked

A review assessment must not claim final acceptance, final rejection, proof
completion, or gate outcome. It may recommend `fix`, `defer`, `waive`, `reject`,
or `needs_human_review` as advisory recommendation only.

### REVIEW-ASSESS-006: side effects require receipt visibility

If a review tool or reviewer changed files, opened a browser, called an API,
published, deployed, commented, created a PR, or touched credentials, the
assessment must link side-effect receipts or mark gate-input eligibility false.

### REVIEW-ASSESS-007: auto-fix output preserves scope

If a review assessment includes proposed or performed auto-fixes, changed
artifact refs must stay within contract scope or be represented as deviations.

### REVIEW-ASSESS-008: confidence and severity are explicit

Every finding must include severity and confidence. Missing confidence or
severity prevents hard-gate use.

### REVIEW-ASSESS-009: suppressed findings are visible

If a reviewer suppresses, ignores, waives, or deems a finding false-positive,
the assessment must record the reason. Silent suppression is invalid.

### REVIEW-ASSESS-010: second opinion cannot decide

A second-opinion or cross-model review can agree, disagree, or add findings. It
cannot pass/fail gate by itself.

## Minimal fixture shape

```yaml
review_assessment:
  assessment_id: review_assessment_fixture_001
  assessment_kind: code_review
  status: advisory
  authority: non_authoritative
  assessor_kind: model
  assessor_ref: gstack_like_review
  contract_ref: contract_fixture_001
  run_ref: run_fixture_001
  input_artifact_refs:
    - path: src/lib.rs
      hash: sha256:fixture
  findings:
    - finding_id: finding_001
      severity: medium
      confidence: 0.82
      clause_ref: contract.scope.allowed_paths
      evidence_refs: []
      claim_status: unverified
      recommendation: needs_human_review
      proposed_change_refs: []
  side_effects_claimed: false
  side_effect_receipt_refs: []
  gate_input_eligible: true
  non_authority: true
```

## Non-goals

This spec does not define review execution, assessor prompts, provider
selection, code modifications, browser testing, security scanner integration,
release automation, runtime storage, gate writing, proofpack writing, or
acceptance claims.
