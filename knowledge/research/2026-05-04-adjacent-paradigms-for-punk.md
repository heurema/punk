---
id: research_2026_05_04_adjacent_paradigms_for_punk
title: "Adjacent Paradigms for Punk"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-04
updated_at: 2026-05-04
review_after: 2026-07-20
components:
  - project-memory
  - knowledge-vault
  - context-pack
  - eval-plane
  - gate-policy
  - proofpack
  - neuro-symbolic
related_goals:
  - work/goals/goal_add_punk_trust_stack_research_and_eval_specs_v0_1.md
related_files:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/DOGFOODING.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
  - evals/specs/context-pack-boundary.v0.1.md
  - knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md
related_evals:
  - evals/specs/contract-clause-coverage.v0.1.md
  - evals/specs/context-pack-compiler-boundary.v0.1.md
  - evals/specs/flow-counterexample-state-model.v0.1.md
  - evals/specs/gate-policy-input.v0.1.md
  - evals/specs/proofpack-provenance-projection.v0.1.md
source_refs:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/DOGFOODING.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/GLOSSARY.md
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
  - evals/specs/context-pack-boundary.v0.1.md
  - knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md
confidence: medium
research_level: R2
---

# Adjacent Paradigms for Punk

## Question

Which adjacent paradigms should inform Punk's next docs/eval/spec direction without promoting broad AI-platform, runtime, CLI, adapter, database, or cloud scope into the active product?

## Decision context

Punk's current posture is narrow: one CLI, one lifecycle grammar, local trust evidence, explicit project memory, and `gate` as the only final decision writer.

The active executable surface remains intentionally small. Broader docs are target architecture, phase-gated design, parked scope, or future scope unless explicitly promoted. Research and ideas are advisory until promoted through ADR, roadmap, goal/contract, implementation, eval, and proof.

This note captures an adjacent-paradigm and neuro-symbolic scan as repo-tracked research. It does not change product truth by itself.

## Sources reviewed

The strongest sources for this patch are the current Punk canonical docs and existing repo-tracked research. The adjacent-paradigm clusters below come from a user-provided research scan and design prompt dated 2026-05-04. Live web/source verification was not performed in this patch, so external source families are treated as "user-provided research scan / not independently revalidated in this patch."

- Punk canonical docs: `README.md`, `docs/product/START-HERE.md`, `docs/product/PUNK-LAWS.md`, `docs/product/ARCHITECTURE.md`, `docs/product/ROADMAP.md`, `docs/product/CRATE-STATUS.md`, `docs/product/DOGFOODING.md`, `docs/product/RESEARCH-GATE.md`, `docs/product/PROJECT-MEMORY.md`, `docs/product/PUBLIC-NARRATIVE.md`, `docs/product/DOCUMENTATION-MAP.md`, `docs/product/GLOSSARY.md`.
- Existing Punk context-pack boundary: `docs/adr/ADR-0016-contract-context-pack-boundary.md` and `evals/specs/context-pack-boundary.v0.1.md`.
- Existing Knowledge Vault direction: `knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md`.
- User-provided adjacent-paradigm scan: context engineering, clause verification, state-machine lifecycle validation, property/model/counterexample evals, provenance/attestations, policy-as-code, event sourcing, capability boundaries, repo-first knowledge systems, human-AI checkpoints, reproducible research, and neuro-symbolic verification hybrids.

## Source quality table

| Source set | Tier | Why included | Key limitation |
|---|---:|---|---|
| Punk canonical docs | A | Current source of truth for active/not-active boundaries, laws, lifecycle, gate/proof authority, project memory, and roadmap phase gates | Internal product truth, not external validation |
| ADR-0016 and `context-pack-boundary.v0.1` | A | Existing contract-context-pack boundary; directly relevant to bounded context and clause support | Proposed/advisory boundary; not final gate acceptance |
| Project Knowledge Vault research note | A/B | Existing repo-tracked direction for repo-first knowledge, derived views, typed claims, context packs, and RAG avoidance | Advisory note; no runtime promotion by itself |
| User-provided adjacent-paradigm scan | C until revalidated | Useful source list and design lens for R2/R3 synthesis | Not independently revalidated in this patch |
| External paradigm families named in the scan | C until source-specific review | Useful as prompts for failure modes, eval patterns, and boundary vocabulary | No source-specific evidence claim should be treated as verified here |

Repo canonical docs remain the strongest source of truth for this patch.

## Existing systems / prior art

### 1. Context engineering / bounded context packs

Useful direction: select bounded, source-anchored context for a contract instead of relying on a giant prompt or executor-local memory.

Punk fit: Contract Context Pack already names this boundary. Future work should strengthen clause/support mapping, stale source surfacing, and explicit exclusions before any retrieval or compiler behavior is promoted.

### 2. Clause coverage and claim verification

Useful direction: every hard requirement should have an explicit coverage path through validators, receipt fields, proof requirements, source refs, assumptions, unknowns, or human review.

Punk fit: this strengthens `contract -> evidence -> gate decision -> proof` and prevents semantic summaries from replacing verifiable coverage.

### 3. Formal/state-machine lifecycle validation

Useful direction: lifecycle state and transition guards should be modelled as legal/illegal transitions with denial evidence.

Punk fit: maps directly to `plot / cut / gate`, event expectations, and no-forbidden-artifact checks.

### 4. Property-based, model-based, and counterexample-first evals

Useful direction: start with ways the lifecycle can be violated, then ensure the model denies those paths.

Punk fit: flow, gate, proof, context-pack, and parked-scope boundaries are easier to protect through counterexamples than through happy-path docs alone.

### 5. Provenance / attestations / SLSA / in-toto / W3C PROV-style thinking

Useful direction: proof artifacts should expose subject, materials, activity, actor/agent, hashes, and event ranges.

Punk fit: useful as local provenance projection only. Compliance claims, key management, remote transparency logs, and release signing stay deferred or parked.

### 6. Policy-as-code / auditable gate policies

Useful direction: gate inputs should be explicit and auditable before any decision is made.

Punk fit: adopt the shape of gate-policy inputs and denial reasons without taking on OPA/Rego/CUE or any external policy-engine dependency now.

### 7. Append-only event logs / event sourcing / rebuildable projections

Useful direction: facts that matter should be replayable from append-only events and canonical artifacts; convenience views are derived.

Punk fit: aligns with local trust evidence and project-memory link graph. It should not become a hidden database or service-backed authority surface.

### 8. Capability manifests / least-authority module boundaries

Useful direction: future modules/adapters should declare what they may read, write, invoke, assess, and emit.

Punk fit: defer until module host work is selected. Useful now only as a boundary idea and future eval target.

### 9. Repo-first knowledge systems / typed claims / stale and contradiction surfacing

Useful direction: knowledge should preserve source refs, authority, status, stale conditions, contradictions, and unknowns.

Punk fit: matches Knowledge Vault direction. Typed claim graph implementation remains deferred.

### 10. Human-AI collaboration checkpoints / checklist discipline

Useful direction: explicit checkpoints prevent executor self-certification, overbroad changes, and hidden assumptions.

Punk fit: maps to contract clauses, validators, run receipts, evals, proof requirements, and report discipline rather than global prompts as product truth.

### 11. Reproducible research / experiment bundles / research objects

Useful direction: research and eval work should be reproducible from source refs, parameters, artifacts, and results.

Punk fit: useful later for eval and research bundles. Not needed for current runtime.

### 12. Neuro-symbolic / NeSy verification hybrids

Useful direction: neural systems can propose candidates and semantic mappings; symbolic/deterministic Punk surfaces validate, block, and record evidence.

Punk fit: adopt the advisory pattern, not an AI authority layer.

## Main findings

1. The safest near-term move is not "add AI platform features"; it is "add boundary specs that make future trust surfaces testable."
2. Punk's core advantage is the authority split: neural or semantic systems may assess, but deterministic artifacts, gate policy inputs, events, receipts, and proof links decide what can be accepted.
3. Clause coverage is the connecting tissue between bounded context, evals, gate policy, proofpacks, and project memory.
4. Counterexample-first specs are the right shape for preventing drift: they capture what must not happen before any runtime or CLI surface exists.
5. Provenance thinking is valuable as a local projection over Punk artifacts, not as a compliance or signing claim in the current phase.
6. Repo-first knowledge and context compilation should stay advisory and source-linked; vector DB or hidden memory must not become project truth.

## Punk Trust Stack v0.1

Punk Trust Stack v0.1 is a docs/eval/spec direction:

```text
bounded context
  -> covered contract clauses
  -> counterexample-first evals
  -> explicit gate policy inputs
  -> provenance-shaped proofpacks
  -> project-memory links
```

The stack is not active runtime behavior. It is a way to organize boundary artifacts so later implementation slices have clear promotion gates.

Trust surface mapping:

| Layer | Near-term artifact | Authority |
|---|---|---|
| bounded context | Context-Pack Compiler boundary extension | advisory/spec |
| covered contract clauses | Clause Coverage Matrix spec | advisory/spec |
| counterexample-first evals | Flow Counterexample State Model spec | advisory/spec |
| explicit gate policy inputs | Gate Policy Input spec | advisory/spec |
| provenance-shaped proofpacks | Proofpack Provenance Projection spec | advisory/spec |
| project-memory links | research, idea, goal, report refs | advisory until promoted |

## Neuro-symbolic / NeSy ideas for Punk

Core pattern:

```text
LLM proposes, Punk disposes
```

Neural systems may draft, classify, map, retrieve candidates, and assess.

Symbolic and deterministic Punk surfaces validate, block, record evidence, and preserve authority.

`gate` remains the only final decision writer.

NeSy ideas to capture:

- Clause Coverage Graph / Matrix: adopt now as docs/eval/spec direction.
- Synthetic negative eval generation: adopt now as counterexample design direction.
- Neuro-symbolic gate assessor: defer as advisory evidence only.
- Contract DSL + natural-language translator: defer.
- Knowledge Vault typed claim graph: defer.
- Drift theorem prover / coherence checker: defer.
- Public narrative claim checker: defer.
- Full neuro-symbolic agent runtime: park/avoid.
- LLM-as-gate: avoid.

Boundary rules:

- Semantic assessors can map clauses to likely evidence gaps, but cannot satisfy coverage without concrete refs.
- Synthetic negative cases can propose eval fixtures, but deterministic eval definitions must be reviewed before they become gate-relevant.
- Any future NeSy assessor output is an assessment, not a decision object.
- Neural retrieval can suggest candidate context refs, but speculative or stale sources remain excluded or flagged by deterministic context-pack policy.

## Adoption map: ADOPT NOW / DEFER / PARK / AVOID

### ADOPT NOW as docs/eval/spec direction

- Clause Coverage Matrix.
- Minimal Context-Pack Compiler boundary.
- Counterexample-first flow evals.
- Gate Policy Input fixtures.
- Proofpack Provenance Projection.
- NeSy advisory pattern: neural proposals, symbolic validation.

### DEFER

- Neuro-symbolic gate assessor.
- Contract DSL / NL-to-contract translator.
- Typed claim graph / Knowledge Vault implementation.
- Capability manifests before module host.
- Reproducible experiment bundles.
- Dynamic planning documents.
- Public narrative claim checker.

### PARK

- CRDT/local-sync substrate.
- Patch-theoretic VCS replacement.
- Proof-carrying plugins.
- Full theorem-proving layer.
- Constraint-solver operator UI.
- Universal collaborative data layer.

### AVOID

- Giant repo prompt / AGENTS.md as truth.
- Vector DB / RAG as project truth.
- LLM self-approval at gate.
- Second lifecycle grammar / generic workflow engine.
- Cloud control plane / hosted hidden memory.
- Provider-adapter ecosystem now.
- Plugin marketplace now.
- Automatic public publishing.

## What stays out of scope

- New runtime behavior.
- New CLI commands.
- New Rust dependencies.
- Provider adapters, MCP, plugin runtime, graph DB, vector DB, CRDT sync, cloud/control-plane behavior, or autonomous agent runner.
- OPA/Rego/CUE dependency or any external policy-engine dependency.
- LLM gate, LLM self-approval, or any assessor writing final decisions.
- Proofpack writer, gate writer, context-pack writer, contract storage, or runtime project-memory storage.
- Public claim that Punk is production-ready or works end-to-end.

## Impact on roadmap

- No phase promotion.
- No change to current CLI surface.
- Strengthens Phase 2 and Phase 3 eval/spec preparation.
- Supports Phase 4 Project Memory and future Knowledge Vault boundaries.
- Adds future input to Phase 5 project coherence and Phase 6 module-host capability thinking, without promoting them now.

## Required evals

This patch creates the initial spec artifacts:

- `evals/specs/contract-clause-coverage.v0.1.md`
- `evals/specs/context-pack-compiler-boundary.v0.1.md`
- `evals/specs/flow-counterexample-state-model.v0.1.md`
- `evals/specs/gate-policy-input.v0.1.md`
- `evals/specs/proofpack-provenance-projection.v0.1.md`

Future implementation work should add deterministic fixture cases only when a bounded goal promotes each spec into model/helper behavior.

## Required docs / ADRs / contracts

- Idea backlog: `knowledge/ideas/punk-trust-stack-v0-1.md`.
- NeSy idea note: `knowledge/ideas/neuro-symbolic-ai-for-punk.md`.
- Dogfooding goal/report for this patch.
- Future ADR only if Punk chooses to promote Trust Stack v0.1 from advisory research/spec direction into accepted architecture.
- Future contracts must cite this research only when selecting a bounded implementation slice.

Canonical product docs do not need to change in this patch because no product truth or active behavior is promoted.

## Decision Log candidate

**Date:** 2026-05-04
**Project:** Punk
**Decision:** Ask Codex to add Punk Trust Stack v0.1 as research, idea backlog, and eval/spec artifacts only.
**Reason:** Adjacent-paradigm and NeSy ideas are useful, but unsafe to promote directly into runtime or CLI.
**What this prevents:** agent-platform drift, LLM-as-gate, RAG/vector truth, hidden memory, dependency creep, premature module/adapters/sync implementation.
**Review date:** 2026-07-20

## Open questions

- Should `context-pack-compiler-boundary.v0.1.md` remain a narrow extension, or should a later patch merge it into `context-pack-boundary.v0.1.md` after review?
- What is the smallest deterministic fixture shape for a Clause Coverage Matrix?
- Should synthetic negative eval generation remain a research-only practice, or become an explicit eval authoring aid later?
- When Phase 3 gate inputs become concrete, should policy input fixtures live under `evals/specs/`, `evals/cases/`, or a future contract schema fixture directory?
- What evidence threshold is needed before any NeSy assessor command interface is selected?
