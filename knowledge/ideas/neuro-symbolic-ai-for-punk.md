---
id: idea_neuro_symbolic_ai_for_punk
kind: idea
status: draft
authority: speculative
owner: vitaly
created_at: 2026-05-04
updated_at: 2026-05-04
review_after: 2026-07-20
components:
  - neuro-symbolic
  - gate-policy
  - contract
  - eval-plane
  - knowledge-vault
related_research:
  - knowledge/research/2026-05-04-adjacent-paradigms-for-punk.md
related_ideas:
  - knowledge/ideas/punk-trust-stack-v0-1.md
related_evals:
  - evals/specs/contract-clause-coverage.v0.1.md
  - evals/specs/flow-counterexample-state-model.v0.1.md
  - evals/specs/gate-policy-input.v0.1.md
supersedes: []
superseded_by: null
---

# Neuro-symbolic AI for Punk

## Summary

Neuro-symbolic ideas are useful for Punk only when they preserve the authority split:

```text
LLM proposes, Punk disposes
```

Neural systems may propose candidate mappings, summaries, counterexamples, and assessments. Punk's deterministic and symbolic surfaces validate, block, record evidence, and preserve final authority.

## Core pattern: neural proposals, symbolic validation, gate authority

- Neural systems may draft, classify, retrieve candidates, map clauses to evidence, and suggest negative evals.
- Symbolic/deterministic Punk surfaces must validate refs, coverage, state transitions, receipts, gate inputs, and proof requirements.
- `gate` remains the only final decision writer.
- Semantic assessor outputs are assessments, not decisions.
- Executor claims and LLM self-review are not proof.

## Adopt now

Adopt now only as docs/eval/spec direction:

- Clause Coverage Graph / Matrix.
- Synthetic negative eval generation as a human-reviewed eval-authoring pattern.
- Advisory pattern: neural proposals plus symbolic validation.
- Coverage gap reporting that blocks approval when hard clauses lack concrete coverage.

## Defer

- Neuro-symbolic gate assessor as future advisory evidence only.
- Contract DSL + natural-language translator.
- Knowledge Vault typed claim graph.
- Drift theorem prover / coherence checker.
- Public narrative claim checker.

## Park

- Full theorem-proving layer.
- Constraint-solver operator UI.
- Proof-carrying plugins.
- Broad symbolic planning system.
- Full neuro-symbolic agent runtime.

## Avoid

- LLM-as-gate.
- LLM self-approval at gate.
- Agent decides final acceptance.
- Semantic confidence score as hard-clause coverage.
- Vector DB/RAG as project truth.
- Hidden model memory as project memory.

## Punk laws touched

- One lifecycle grammar: `plot / cut / gate`.
- Only `gate` writes final decisions.
- Proof comes before acceptance.
- Modules may assess, not decide.
- Adapters may invoke, not own truth.
- Project memory is explicit and authority-tagged.
- Raw ideas are not implementation truth by default.
- Executor claims are not proof.

## Eval implications

- Add counterexamples where a semantic assessor says a clause "looks fine" but no validator, receipt field, proof requirement, source ref, assumption, unknown, or human review reason exists.
- Add counterexamples where module assessment tries to satisfy final decision authority.
- Add fixture shape for clause-to-evidence coverage gaps.
- Add future tests that generated negative evals require human review before becoming accepted eval fixtures.

## Out of scope

- Runtime assessor command.
- Provider adapter.
- Graph DB.
- Vector DB.
- Contract DSL runtime.
- Gate writer.
- Proof writer.
- Agent runner.
- Any model writing final decisions.
