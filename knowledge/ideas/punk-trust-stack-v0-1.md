---
id: idea_punk_trust_stack_v0_1
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-04
updated_at: 2026-05-04
review_after: 2026-07-20
components:
  - project-memory
  - context-pack
  - contract
  - eval-plane
  - gate-policy
  - proofpack
  - neuro-symbolic
related_research:
  - knowledge/research/2026-05-04-adjacent-paradigms-for-punk.md
related_ideas:
  - knowledge/ideas/neuro-symbolic-ai-for-punk.md
related_evals:
  - evals/specs/contract-clause-coverage.v0.1.md
  - evals/specs/context-pack-compiler-boundary.v0.1.md
  - evals/specs/flow-counterexample-state-model.v0.1.md
  - evals/specs/gate-policy-input.v0.1.md
  - evals/specs/proofpack-provenance-projection.v0.1.md
supersedes: []
superseded_by: null
---

# Punk Trust Stack v0.1

## Summary

Punk Trust Stack v0.1 is an advisory backlog for connecting bounded context, clause coverage, counterexample evals, gate policy inputs, provenance-shaped proofpacks, and project-memory links.

This backlog is not product truth by itself.

## Why this fits Punk

It reinforces existing Punk laws:

- one CLI: `punk`;
- one lifecycle grammar: `plot / cut / gate`;
- `contract -> evidence -> gate decision -> proof`;
- only `gate` writes final decisions;
- proof comes before acceptance;
- modules may assess, not decide;
- adapters may invoke, not own truth;
- project memory is explicit, repo-tracked where possible, authority-tagged, and not a giant prompt.

## Why this is not active runtime behavior

This note does not add runtime behavior, CLI commands, Rust dependencies, provider adapters, MCP, plugin runtime, graph DB, vector DB, sync, cloud behavior, gate writer, proof writer, context-pack writer, contract storage, or autonomous agent execution.

Ideas here are advisory until promoted through ADR, roadmap, goal/contract, implementation, eval, and proof.

## Adopt-now candidates

Adopt now only as docs/eval/spec direction:

- Clause Coverage Matrix.
- Minimal Context-Pack Compiler boundary.
- Flow Counterexample State Model.
- Gate Policy Input fixtures.
- Proofpack Provenance Projection.
- NeSy advisory pattern: neural proposals, symbolic validation.
- Synthetic negative eval generation as an eval-authoring idea, not runtime generation.

## Deferred candidates

- Neuro-symbolic gate assessor.
- Contract DSL / natural-language-to-contract translator.
- Typed Claim Graph / Knowledge Vault implementation.
- Drift / Coherence Checker.
- Public Narrative Claim Checker.
- Capability Manifest.
- Reproducible Experiment Bundle.
- Attested Proofpack.

## Parked candidates

- Local-first sync substrate.
- CRDT/local-sync substrate.
- Patch-theoretic VCS replacement.
- Proof-carrying plugins.
- Full theorem-proving layer.
- Constraint-solver operator UI.
- Universal collaborative data layer.

## Avoid list

- Giant repo prompt as truth.
- Vector DB as truth.
- LLM-as-gate.
- LLM self-approval at gate.
- Second lifecycle grammar or generic workflow engine.
- Cloud control plane / hosted hidden memory.
- Provider ecosystem now.
- Plugin marketplace now.
- Automatic public publishing.

## Core backlog table

| Idea | Trust surface | Phase fit | Verdict | Smallest MVP | Required eval | Scope creep risk |
|---|---|---|---|---|---|---|
| Clause Coverage Matrix | contract clauses -> validators/receipts/proof/human review | Phase 2/3 prep | ADOPT NOW as docs/eval/spec | Markdown fixture shape plus blocked/covered examples | `contract-clause-coverage.v0.1` | Medium: can become heavyweight contract bureaucracy |
| Minimal Context-Pack Compiler boundary | bounded context | Phase 3/4 prep | ADOPT NOW as boundary extension | Narrow extension to existing context-pack spec | `context-pack-compiler-boundary.v0.1` | High: can drift into prompt manager or retrieval engine |
| Flow Counterexample State Model | lifecycle guard behavior | Phase 1/2 prep | ADOPT NOW as eval/spec | Counterexamples for illegal transitions and forbidden artifacts | `flow-counterexample-state-model.v0.1` | Low: stay docs/spec until model helper selected |
| Gate Policy Input fixtures | gate evidence inputs | Phase 3 prep | ADOPT NOW as eval/spec | Minimum input shape and negative fixtures | `gate-policy-input.v0.1` | Medium: can become policy engine dependency |
| Proofpack Provenance Projection | proof inspection | Phase 3 prep | ADOPT NOW as eval/spec | Local-only subject/material/activity mapping | `proofpack-provenance-projection.v0.1` | Medium: can overclaim SLSA/in-toto/signing |
| NeSy Advisory Assessor boundary | semantic assessment | Phase 5+ | DEFER | Assessor emits clause-specific evidence gaps only | Future semantic-assessor boundary extension | High: can become LLM-as-gate |
| Synthetic Negative Eval Generator | eval authoring | Phase 2+ | ADOPT NOW as idea, DEFER tooling | Human-reviewed generated counterexample list | Future eval-authoring checks | Medium: generated tests can encode false assumptions |
| Typed Claim Graph | Knowledge Vault | Phase 4/5 | DEFER | Repo-tracked claim fixture with source/status/stale fields | Future knowledge-authority eval | High: graph can become hidden truth |
| Drift / Coherence Checker | project coherence | Phase 5 | DEFER | Advisory report over stale/conflict refs | Future coherence eval | Medium: checker can imply final decision authority |
| Public Narrative Claim Checker | public memory | Phase 8 | DEFER | Advisory claim/support checklist for posts | Future public-claim eval | Medium: can block publishing with weak heuristics |
| Capability Manifest | module/adapters | Phase 6/9 | DEFER | Manifest schema sketch after module host selection | Future module conformance eval | Medium: can imply plugin runtime now |
| Reproducible Experiment Bundle | eval/research | Phase 2/4 | DEFER | Research bundle metadata for one eval report | Future experiment-bundle eval | Low: mostly artifact hygiene |
| Attested Proofpack | proof/supply chain | Later | DEFER/PARK | Local provenance fields before signatures | Future proofpack provenance eval | High: can imply signing/compliance claims |
| Local-first sync substrate | project memory | Later | PARK | None now | None now | High: can create cloud/control-plane drift |
| Giant repo prompt as avoid | project memory anti-pattern | All phases | AVOID | Explicit anti-pattern note | Context-pack orphan/speculative-source evals | High: collapses project truth into prompt text |
| Vector DB as truth as avoid | Knowledge Vault anti-pattern | All phases | AVOID | Explicit anti-pattern note | Future knowledge-vault authority eval | High: hides authority and stale source state |
| LLM-as-gate as avoid | gate anti-pattern | All phases | AVOID | Explicit anti-pattern note | Gate authority boundary evals | Critical: violates only-gate-writes-final-decisions |

## Smallest reversible MVPs

- Add a Clause Coverage Matrix fixture to a future contract spec.
- Add one counterexample eval case for each illegal `plot / cut / gate` transition.
- Add gate policy input fixtures that record missing evidence without writing decisions.
- Add a proofpack provenance projection fixture that is rebuildable and local-only.
- Add a context-pack compiler boundary fixture that references the existing context-pack spec and avoids duplicate truth.

## Required evals

- `evals/specs/contract-clause-coverage.v0.1.md`
- `evals/specs/context-pack-compiler-boundary.v0.1.md`
- `evals/specs/flow-counterexample-state-model.v0.1.md`
- `evals/specs/gate-policy-input.v0.1.md`
- `evals/specs/proofpack-provenance-projection.v0.1.md`

## Scope creep risks

- Turning context compilation into prompt management.
- Treating semantic assessment as evidence coverage without concrete refs.
- Treating proofpack as a decision surface.
- Treating vector/RAG retrieval as project truth.
- Adding provider adapters before active-core trust surfaces are stable.
- Importing policy engines, graph databases, sync layers, or signing stacks before a bounded goal selects them.

## Open questions

- Should Trust Stack v0.1 eventually become an ADR, or remain research/idea/eval guidance until a concrete implementation slice is selected?
- Should clause coverage be represented as a separate artifact or embedded in future contract schema fixtures?
- Which counterexamples should become smoke evals first after Phase 2 is selected?
- What minimum provenance fields are needed before proofpack writer expansion resumes?
