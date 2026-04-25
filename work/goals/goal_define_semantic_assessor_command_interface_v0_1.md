---
id: goal_define_semantic_assessor_command_interface_v0_1
title: "Define semantic assessor command interface v0.1"
status: done
owner: "vitaly"
module: "core"
priority: P1
authority: canonical
created_at: 2026-04-25
updated_at: 2026-04-25
selected_at: 2026-04-25
started_at: 2026-04-25
completed_at: 2026-04-25
blocked_by: []
scope:
  include:
    - "evals/specs/**"
    - "work/goals/**"
    - "work/reports/**"
  exclude:
    - "crates/**"
    - ".punk/**"
acceptance:
  - "Semantic assessor command interface v0.1 is defined as a docs/spec-only artifact before any semantic assessor implementation."
  - "Spec defines input/output boundary for clause-scoped advisory assessment evidence."
  - "Spec uses missing-validator outcomes for unavailable/skipped/unsupported/deferred assessors."
  - "Spec preserves semantic assessor output as evidence, not gate decision, proof, or executor self-review."
  - "No runtime code, CLI, schema, gate, proofpack, provider/model/agent adapter, or `.punk/` state is implemented."
knowledge_refs:
  - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
  - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
  - "evals/specs/missing-validator-policy.v0.1.md"
  - "evals/specs/minimal-receipt-fields.v0.1.md"
  - "evals/specs/semantic-assessor-command-interface.v0.1.md"
contract_refs: []
report_refs:
  - "work/reports/2026-04-25-semantic-assessor-command-interface-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "The semantic assessor interface is a docs/spec-only follow-up grounded in executor-agnostic validation, missing-validator policy, and minimal receipt fields."
  research_refs:
    - "docs/product/RESEARCH-GATE.md"
    - "docs/adr/ADR-0014-executor-agnostic-validation-boundary.md"
    - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
    - "evals/specs/missing-validator-policy.v0.1.md"
    - "evals/specs/minimal-receipt-fields.v0.1.md"
    - "evals/specs/semantic-assessor-command-interface.v0.1.md"
    - "work/reports/2026-04-25-minimal-receipt-fields-v0-1.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: architecture
  required_updates:
    - "evals/specs/semantic-assessor-command-interface.v0.1.md"
    - "evals/specs/executor-agnostic-validation-boundary.v0.1.md"
    - "evals/specs/missing-validator-policy.v0.1.md"
    - "evals/specs/minimal-receipt-fields.v0.1.md"
  rationale: "Semantic assessor command interface v0.1 defines advisory evidence boundaries before implementation."
---

## Context

Punk may later use semantic or LLM-based assessors as advisory evidence, but assessor output cannot become gate decision, proof, or executor self-review.

Before implementation, define the command interface boundary for clause-scoped semantic assessment evidence.

## Notes

Keep this docs/spec-only. Do not implement semantic assessor commands, model/provider adapters, runtime storage, CLI behavior, gate decisions, proofpacks, schemas, or `.punk` state.

## Outcome

Defined `evals/specs/semantic-assessor-command-interface.v0.1.md` as the docs/spec-only interface boundary for future clause-scoped semantic assessment evidence.

The boundary says semantic assessors:

- assess one clause or bounded question;
- cite explicit evidence refs;
- use missing-validator outcomes when unavailable, skipped, unsupported, failed, or deferred;
- remain advisory evidence only.

No runtime code, CLI, schema, gate, proofpack, provider/model/agent adapter, or `.punk/` state was implemented.
