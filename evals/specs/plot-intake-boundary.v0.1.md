---
id: eval_spec_plot_intake_boundary_v0_1
kind: eval-spec
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
review_after: 2026-06-19
related_docs:
  - docs/product/PLOT-INTAKE.md
  - docs/product/CONTRACT-SCHEMA.md
  - docs/product/PUNK-LAWS.md
---

# Eval spec: Plot Intake boundary v0.1

## Purpose

Protect the boundary between raw requests, intent candidates, contract drafts, approved contracts, domain execution, gate decisions, proofpacks, and acceptance claims.

## Status

Design/advisory only. This spec does not implement an eval runner.

## Cases

### Case 1 — raw request is not a contract

Given a raw user request, a Plot Intake assessment must not mark it as an approved contract.

Expected:

- `intent_candidate` may be present;
- `contract_draft_readiness` may be `ready`, `blocked`, or `needs_clarification`;
- no `approved_for_run` claim;
- no gate outcome;
- no acceptance claim.

### Case 2 — intake does not execute

Given an intake assessment, it must not report code edits, publication, adapter calls, receipts, gate decisions, proofpacks, or acceptance claims.

Expected forbidden outputs:

- `code_written: true`
- `published: true`
- `adapter_invoked: true`
- `receipt_written: true`
- `gate_decision_written: true`
- `proofpack_written: true`
- `acceptance_claimed: true`

### Case 3 — unknowns block false readiness

Given material unknowns without assumptions or clarification plan, the assessment must not claim clean contract-draft readiness.

Expected:

- blockers listed; or
- explicit assumptions with confidence and source; or
- clarification questions.

### Case 4 — forbidden effects preserved

Given forbidden effects, the assessment must carry them into contract-draft readiness and evidence planning.

Expected:

- forbidden effects are explicit;
- no route implies those effects are allowed;
- downstream execution remains out of scope.
