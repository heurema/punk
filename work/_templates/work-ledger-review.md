---
id: review_example
kind: work-ledger-review
status: draft
authority: advisory
owner: owner
created_at: 2026-04-22
updated_at: 2026-04-22
reviewed_range:
  from: "work/reports/..."
  to: "work/reports/..."
research_gate:
  classification: R0
  required: false
  rationale: ""
  research_refs: []
  external_research_refs: []
---

## Summary

State what period was reviewed and why the review is happening now.

This review is advisory.
It does not decide.
It does not create a second live source of truth.
Follow-up work requires explicit goal selection or user approval through the normal Work Ledger flow.

## Research Gate

Record the classification and decision.

## Evidence Reviewed

List the goals, reports, checks, and commit range reviewed.

## Findings

Use one block per finding.

```yaml
id: F-001
domain: process | architecture | validation | docs | research | product | code
finding: ""
severity: low | medium | high
recurrence: first-seen | repeated | systemic
evidence_strength: direct | mixed | inferred
evidence_refs: []
disposition: guardrail | bounded-goal | park-idea | research | adr | no-action
destination:
  type: work-goal | knowledge-idea | research-note | adr | skill-update | validator | no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: ""
revisit_trigger: ""
```

## Candidate Improvements

Only include candidates with direct evidence.
Use this lightweight schema:

```yaml
id: CI-001
based_on_findings: []
summary: ""
priority: low | medium | high
destination:
  type: work-goal | research-note | adr | no-action
  ref: ""
why_now: ""
why_not_now: ""
driver: vitaly
```

## ADR Candidates

Use a table of candidates only.
Do not create an ADR in the review itself.

| Finding | Why ADR-level | Suggested ADR | Status |
|---|---|---|---|

## Parked Ideas

List ideas that remain explicitly out of scope for now.

## No Action

List findings that do not require a follow-up yet.

## Next Recommended Action

Recommend exactly one conservative next goal.
Prefer an existing `ready` goal.
If no suitable `ready` goal exists, create at most one new narrow follow-up goal.

## Scope Boundaries Preserved

Confirm what stayed out of scope.
