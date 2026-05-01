# User intent to contract UX flow boundary v0.1

Date: 2026-04-30
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the UX boundary for how Punk should move from a user's raw request to an understood intent and then to a contract draft before any execution starts.

This is a design/spec artifact only. It does not activate runtime contract storage, CLI behavior, provider/model/agent execution, context compiler behavior, gate decision writing, proofpack writing, or proofpack Writer behavior.

## Boundary under test

```text
user request
  -> intent understanding
  -> clarification or refusal when needed
  -> contract draft
  -> user confirmation
  -> approved bounded work
  -> execution
  -> receipt/evidence
  -> eval/assessment
  -> gate decision
  -> proofpack
  -> acceptance claim
```

Writer is downstream. It must not define, infer, or repair the upstream intent/contract boundary.

## Status and authority

This boundary is advisory/design until promoted by later goals.

At Dogfooding Level 0, the durable artifacts are repo-tracked goals, reports, specs, and status updates. Level 0 `done` is manual closure with evidence, not future `gate` acceptance.

## Terms

| Term | Meaning | Not meaning |
|---|---|---|
| User request | The raw user-provided ask, including any desired outcome, constraints, examples, urgency, and context. | Not yet a contract, not yet approved work. |
| Intent | Punk's bounded interpretation of what the user wants and why. | Not implementation, not execution permission, not acceptance. |
| Clarification | A question or narrowing step needed before Punk can draft a safe contract. | Not a hidden assumption or silent scope expansion. |
| Refusal or stop | A safe halt when Punk cannot proceed within laws, scope, evidence, or authority boundaries. | Not a gate decision or proof outcome. |
| Contract draft | An inspectable proposal for bounded work, scope, non-goals, assumptions, validation, evidence, and risks. | Not accepted work and not proof. |
| User confirmation | The user's explicit approval, edit, or rejection of the contract draft. | Not gate acceptance and not proof. |
| Approved contract | The bounded authorization for execution. | Not proof that execution succeeded. |
| Writer | A downstream proofpack artifact writer. | Not intent parser, contract authorizer, gate decision writer, or acceptance claimant. |

## UX state machine

| State | Punk does | User-visible output | Exit condition |
|---|---|---|---|
| Raw request captured | Preserve the raw ask and identify obvious goal, constraints, and context. | Short restatement of the ask or direct next step when trivial. | Enough information exists to form an intent candidate, or clarification is required. |
| Intent candidate | Summarize the intended outcome, scope shape, and likely work type. | Plain-language intent summary. | Intent is clear enough to draft a contract, or ambiguity blocks drafting. |
| Clarification required | Ask the smallest necessary question(s). | Focused questions with why they matter. | User answers, narrows, cancels, or the work is refused/deferred. |
| Refused/deferred | Stop before contract/execution. | Reason and safe alternative when possible. | No execution; optional future goal/research route. |
| Contract draft | Propose bounded work and validation expectations. | Contract draft or Level 0 goal/report plan. | User confirms, edits, rejects, or asks to split. |
| Contract confirmed | Treat the contract as authorization to execute only the approved scope. | Clear acknowledgement of bounded execution scope. | Execution may start. |
| Execution complete | Produce receipts/evidence and report what was or was not verified. | Outcome report with checks and gaps. | Gate/proof path may later close work when active. |
| Gate/proof/acceptance | Keep decision, proofpack, and acceptance claims separate. | Decision/proof refs when future runtime exists. | Positive acceptance claim only after accepting gate decision plus matching proof. |

## Required pre-execution capture

Before Punk starts non-trivial execution, the contract draft or Level 0 work artifact should capture:

1. raw request summary;
2. understood intent;
3. included scope;
4. excluded scope and non-goals;
5. assumptions and explicit unknowns;
6. acceptance criteria;
7. evidence and validator/check plan;
8. Research Gate classification and refs when required;
9. expected changed artifact classes;
10. safety, rollback, or side-effect boundaries when relevant;
11. DocImpact or knowledge-impact expectation when docs/product/project-memory truth may change;
12. downstream closure expectation: receipt/evidence first, then assessment/eval, then gate decision, then proofpack, then acceptance claim.

If any required item is unknown and material, Punk should ask a clarification question or explicitly record the unknown before drafting or executing.

## Clarification triggers

Punk should ask clarifying questions before drafting a contract when any of these are unclear or conflicting:

- desired outcome;
- success criteria;
- included scope;
- forbidden scope;
- destructive or irreversible side effects;
- authority to touch external systems, credentials, PII, publishing, billing, security-sensitive behavior, or user data;
- runtime/code/schema/CLI/storage changes versus docs/spec-only work;
- Research Gate classification or missing required research;
- validation/evidence expectations;
- whether parked/future capabilities may be promoted;
- whether a broad request should be split into bounded contracts;
- conflict between user request and canonical docs/laws.

Ask the smallest useful question. Do not ask performative questions when a safe, bounded assumption is already explicit in the contract draft.

## Refusal or stop triggers

Punk should refuse, stop, or defer before execution when the request would require:

- bypassing `gate` or proof-before-acceptance;
- treating executor claims as proof;
- unbounded autonomy or hidden authority;
- destructive actions without explicit authorization;
- secrets, credentials, or PII exposure;
- runtime behavior, storage, schema, CLI, adapter, automation, or Writer activation outside the selected goal;
- adopting external research without required Research Gate artifacts;
- collapsing advisory assessments into final decisions;
- promoting parked capabilities as active behavior without roadmap/eval/work-ledger support.

When possible, Punk should propose the smallest safe alternative: clarify, split, create a research goal, create a boundary goal, or record a deferred candidate.

## Contract draft minimum shape

A contract draft should be inspectable without hidden chat context.

Minimum fields or sections:

```yaml
contract_draft:
  raw_request_summary: "..."
  intent: "..."
  scope:
    include: []
    exclude: []
  non_goals: []
  assumptions: []
  unknowns: []
  acceptance: []
  evidence_plan: []
  research_gate:
    classification: R0
    required: false
    refs: []
  side_effect_boundaries: []
  doc_impact: null
  downstream_closure:
    receipt_required: true
    gate_decision_required_for_acceptance: true
    proofpack_required_for_positive_acceptance: true
```

This shape is a boundary fixture, not an active schema or runtime storage format.

## Required deterministic eval cases

### INTENT-001: raw_request_is_not_contract

A raw user request must not be treated as an approved contract unless bounded scope, acceptance, and evidence expectations are explicitly captured.

### INTENT-002: unclear_material_scope_requires_clarification

If included scope, forbidden scope, success criteria, or side effects are materially unclear, Punk must ask a clarification question, split the work, or record an explicit unknown before execution.

### INTENT-003: no_execution_before_contract_confirmation

Non-trivial work must not start until the contract draft or Level 0 work artifact is confirmed by the user or by an explicitly selected Work Ledger goal.

### INTENT-004: user_confirmation_is_not_gate_acceptance

User confirmation authorizes bounded execution. It does not accept the result and does not replace gate/proof.

### INTENT-005: writer_is_downstream_only

Proofpack Writer behavior must remain downstream of intent, contract, execution, receipt/evidence, assessment/eval, and gate/proof boundaries. Writer must not infer intent, approve contracts, write gate decisions, or claim acceptance.

### INTENT-006: refusal_for_hidden_authority_or_unbounded_side_effects

Requests that depend on hidden authority, unbounded autonomy, destructive side effects, secrets/PII exposure, or parked capability activation must stop, clarify, or route to a separate bounded goal before execution.

### INTENT-007: evidence_plan_required

A contract draft must identify expected evidence, validators, checks, reports, or explicit no-eval rationale before non-trivial execution.

### INTENT-008: research_gate_before_important_architecture_or_runtime_change

If work changes core behavior, storage, module interfaces, flow, eval, adapters, project memory, or external side effects, the contract draft must carry Research Gate classification and required refs before execution.

### INTENT-009: executor_claims_are_not_proof

Executor statements such as “done”, “tests passed”, or “scope preserved” must be verified, captured as unverified, or rejected/deferred. They cannot become acceptance by themselves.

### INTENT-010: contract_context_pack_is_evidence_selection

A future Contract Context Pack may support the contract draft, but it cannot become the contract, define acceptance criteria, write decisions, or replace user confirmation.

## Non-goals

This v0.1 boundary does not define or implement:

- runtime `.punk/contracts` storage;
- CLI commands;
- schema files;
- contract writer implementation;
- context-pack writer implementation;
- executor brief generation;
- provider/model/agent adapters;
- automation;
- gate decision writer;
- proofpack writer behavior;
- active proofpack referenced artifact verification;
- acceptance claim writer;
- `punk init`.

## Future rollout

1. Use this boundary as manual review criteria for the next Work Ledger review.
2. If selected later, define a side-effect-free intent/contract draft model before runtime storage or CLI behavior.
3. Add deterministic eval coverage only when repeated drift appears or when the contract draft model is promoted.
4. Resume deeper proofpack Writer work only after the review confirms the upstream intent-to-contract boundary is sufficient for downstream proof artifact recording.
