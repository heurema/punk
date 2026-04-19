# ADR-0013: Policy assessment vs gate decision boundary

Status: Accepted
Date: 2026-04-19

Research refs:

- `knowledge/research/2026-04-19-project-ideas-intake.md`

## Context

`punk` must preserve a strict boundary: modules and policies can assess; only `gate` can write the final decision.

External policy systems show useful primitives: allow/deny results, decision ids, query inputs, masking, and decision logs. `punk` should adopt these primitives without importing remote policy servers or making policy engines the decision owner.

## Decision

`punk` rules and modules may produce assessments.

A gate decision may reference assessments.

Assessments must not become final acceptance.

Every denied transition or failed guard must have:

- guard code
- human-readable explanation
- machine-readable reason
- relevant contract or flow id
- event id
- no raw sensitive payload by default

## Guard result schema sketch

```yaml
schema_version: punk.guard_result.v0.1
guard_id: guard.cut.requires_approved_contract
result: allow|deny|warn
decision_id: null
flow_id: flow_...
contract_id: contract_...
explanation: cut requires an approved contract
reason_code: CUT_REQUIRES_APPROVED_CONTRACT
evidence_refs:
  - .punk/contracts/contract_....json
privacy:
  redacted: true
```

## Consequences

Positive:

- guard denials become explainable
- `gate` remains the single final decision writer
- future policy engines can plug in as assessors
- event log can support offline debugging and replay

Negative:

- assessment and decision artifacts must be separate
- docs must avoid saying that a rule "accepted" work
- modules need explicit permission boundaries

## Out of scope

- adopting Rego as a mandatory language
- remote OPA server
- user-defined untrusted policies
- policy marketplace
