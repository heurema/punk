---
id: eval_spec_plot_intake_evidence_plan_v0_1
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
  - docs/product/PROJECT-MEMORY.md
---

# Eval spec: Plot Intake evidence plan v0.1

## Purpose

Ensure Plot Intake prepares inspectable evidence and receipt expectations without writing receipts or decisions.

## Status

Design/advisory only. This spec does not implement an eval runner or runtime storage.

## Cases

### Case 1 — evidence plan covers readiness

If contract-draft readiness is marked ready, the assessment should include:

- required evidence artifacts or explicit no-evidence rationale;
- source policy;
- freshness policy when relevant;
- contradiction handling;
- required validator/eval refs or missing-validator note.

### Case 2 — receipt requirements are projected only

The assessment may declare what a future run receipt must prove, but it must not write a receipt.

Expected:

- receipt fields are declared as candidates;
- no `.punk/runs` writes;
- no receipt id claimed as persisted unless provided as an existing ref.

### Case 3 — project-memory refs are explicit

The assessment should include relevant goal, research, docs, ADR, report, eval, decision, proof, or public-narrative refs when available.

If none are available, it should explicitly state that no refs were found or required.

### Case 4 — assumptions stay assumptions

Assumptions must not be promoted into confirmed requirements unless backed by explicit source refs or user confirmation.
