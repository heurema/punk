---
id: docs_product_review_assessment
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-18
updated_at: 2026-05-18
review_after: 2026-07-18
canonical_for:
  - review-assessment-boundary
  - advisory-assessment-receipt-shape
  - assessment-vs-decision-evidence-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/EVAL-PLANE.md
  - docs/product/PROJECT-MEMORY.md
related_research:
  - knowledge/research/2026-05-18-gstack-workflow-prior-art.md
related_evals:
  - evals/specs/review-assessment-receipt.v0.1.md
supersedes: []
superseded_by: null
---

# Review Assessment

## Purpose

Define a shared boundary for advisory review outputs: code review, security
review, QA review, design review, developer-experience review, docs-drift
review, release review, and second-opinion review.

A review assessment is evidence.

It is not a final decision.

## Principle

Reviewers, modules, tools, and models may assess.

Only `gate` decides.

```text
review assessment -> gate input candidate -> gate decision -> proofpack
```

## Assessment kinds

Initial assessment kinds:

- `code_review`
- `security_review`
- `qa_review`
- `design_review`
- `dx_review`
- `docs_drift_review`
- `release_readiness_review`
- `second_opinion_review`
- `contract_intake_review`

These kinds name lenses. They do not activate modules, tools, providers, browser
QA, security scanners, or release automation.

## Minimal shape

```yaml
review_assessment:
  assessment_id: assessment_example
  assessment_kind: code_review
  status: advisory
  authority: non_authoritative
  assessor_kind: human | model | tool | module | adapter
  assessor_ref: reviewer_or_tool_ref
  contract_ref: contract_example
  run_ref: run_example
  input_artifact_refs:
    - path: crates/example/src/lib.rs
      hash: sha256:example
  scope_refs:
    allowed: []
    denied: []
  findings:
    - finding_id: finding_001
      severity: low | medium | high | critical
      confidence: 0.8
      clause_ref: contract.clauses.scope
      evidence_refs: []
      claim_status: unverified | verified_by_validator | rejected | deferred
      recommendation: fix | defer | waive | reject | needs_human_review
      proposed_change_refs: []
  side_effects_claimed: false
  side_effect_receipt_refs: []
  gate_input_eligible: false
  non_authority: true
```

## Finding states

A finding can be:

- `unverified` — reviewer claim exists, but no validator or human gate review
  has verified it;
- `verified_by_validator` — deterministic or explicit validation supports the
  finding;
- `rejected` — evidence contradicted the finding;
- `deferred` — finding is relevant but outside current contract scope;
- `waived_candidate` — finding may be waived only by explicit gate policy later.

A finding state is not a gate decision.

## Gate input eligibility

A review assessment may be eligible as gate input only when it has:

- a contract ref;
- input artifact refs;
- clause or scope mapping for material findings;
- explicit confidence and severity;
- evidence refs or explicit unverified status;
- side-effect receipt refs when side effects are claimed;
- no hidden acceptance claim;
- no final-decision language.

Gate may cite the assessment, reject it, require more evidence, or defer it.

## Side effects

A review assessment must not hide side effects.

If the reviewer, tool, module, or adapter changed files, opened a browser,
called an API, deployed, published, commented, created a PR, or touched
credentials, the assessment must link side-effect receipts or mark the claim as
unverified/ineligible.

Assessment without receipt must not be treated as proof of performed work.

## Relationship to gstack-like reviews

A gstack-style `/review`, `/qa`, `/cso`, `/document-release`, `/plan-*`, or
`/codex` output maps to a Punk review assessment only after it is captured with
explicit refs, status, authority, scope mapping, evidence refs, and side-effect
visibility.

The slash command itself is not authority.

## Non-goals

This document does not activate:

- public review CLI commands;
- DevPunk;
- Module Host runtime;
- browser QA;
- provider-specific review tools;
- security scanner integration;
- deployment automation;
- automatic docs rewrite;
- gate writing;
- proofpack writing;
- acceptance claims.
