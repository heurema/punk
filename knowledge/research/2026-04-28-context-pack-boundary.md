---
id: research_context_pack_boundary_2026_04_28
kind: research-note
status: advisory
authority: advisory
owner: vitaly
created_at: 2026-04-28
review_after: 2026-07-28
research_depth: R2/R3
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-GATE.md
related_adrs:
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
supersedes: []
superseded_by: null
---

# Context Pack Boundary for Punk

## Question

How should Punk define a bounded context-selection mechanism so that `plot` can assemble exactly enough context for a goal or contract: not too little, not too much, authority-aware, scoped, inspectable, and compatible with `plot / cut / gate`?

## Decision context

Punk is an experimental, early-stage, local-first bounded work kernel. The active grammar remains `plot -> cut -> gate`. Only `gate` writes final decisions. Proof comes before acceptance. Project memory is explicit, repo-tracked, authority-tagged, and must not become one giant prompt. Retrieval is advisory and cannot become project truth or bypass `gate`.

This topic changes project-memory and knowledge/retrieval behavior, so it is research-gated. This note is advisory until promoted through ADR, roadmap, goal/contract, implementation, eval, and proof.

## Source quality table

| Source set | Tier | Use in this decision |
|---|---:|---|
| Punk canonical docs: README, START-HERE, PUNK-LAWS, ARCHITECTURE, ROADMAP, CRATE-STATUS, DOGFOODING, RESEARCH-GATE, PROJECT-MEMORY, PUBLIC-NARRATIVE | A | Source of truth for lifecycle, authority, active/parked scope, project memory, and advisory-only retrieval. |
| Anthropic context-engineering guidance and compaction/tool-result guidance | A/B | Supports treating context as a finite inference resource and selecting/compressing high-signal context deliberately. |
| LangChain context-engineering and handoff guidance | B | Provides operational vocabulary: write, select, compress, isolate; warns that handoffs need explicit context control. |
| Lost in the Middle | A | Shows long-context positional degradation and supports avoiding indiscriminate context stuffing. |
| LLMs Get Lost In Multi-Turn Conversation | A | Shows underspecified multi-turn tasks can degrade because models form premature assumptions and recover poorly. |
| Self-RAG and Adaptive-RAG | A | Argue against fixed, indiscriminate retrieval; retrieval should depend on task need and complexity. |
| RAGAs and related RAG metric docs | A/B | Provide useful metrics vocabulary: context precision, context recall, faithfulness, answer relevance. |
| LongLLMLingua and prompt-compression work | A | Useful for future compression after relevance and authority have already been established. |
| Promotional commentary, social posts, weak summaries | C | May inspire questions; not used as a core decision basis. |

## Failure modes found

| Failure mode | Description | Punk implication |
|---|---|---|
| Under-contextualized task | Missing goal, scope, output, validator, or authority context causes the model/executor to infer hidden assumptions. | `plot` must clarify, record assumptions, or mark unknowns before executable work. |
| Over-contextualized task | Large prompts dilute relevant information, hide contradictions, and introduce distractors. | Do not pass all project memory to an executor. |
| Orphan context | A context item is included because it seems relevant, but it supports no contract clause or validator. | Reject or drop orphan context. |
| Unsupported clause | A contract clause exists without source, clarification, assumption, or explicit unknown. | Block or mark the contract incomplete. |
| Flattened contradictions | Conflicting docs are merged into one hidden prompt narrative. | Record contradiction sets explicitly. |
| Stale/superseded truth | Old docs are treated as current because retrieval ranked them highly. | Authority/status must outrank semantic similarity. |
| Speculative leakage | Ideas enter implementation truth by default. | Exclude speculative artifacts unless explicitly cited as advisory. |
| Retrieval owns truth | Search/index state becomes the de facto memory layer. | Retrieval receipts are evidence only and never gate decisions. |
| Executor brief drift | A generated brief introduces scope or acceptance changes. | Briefs must be derived from the contract/context pack and remain advisory. |

## Options considered

| Option | Result | Verdict |
|---|---|---|
| Minimal context, rely on model interpolation | Cheap but amplifies underspecification and hidden assumptions. | avoid |
| Maximal context, pass everything known | Feels safe but creates context rot, distractors, and hidden conflict flattening. | avoid |
| Fixed top-k retrieval into prompt | Easy to implement but ignores authority, task complexity, and clause coverage. | defer |
| Contract-linked, authority-aware Context Pack | Preserves Punk laws while solving the immediate boundary problem. | adopt |
| Adaptive retrieval/reranking/vector memory | Useful future infrastructure, but out of active-core scope now. | park |

## Recommended Punk boundary

Use **Contract Context Pack** for the artifact and **Context Pack Boundary** for the policy. Do not use “Context Contract” as the main term because Punk already uses `contract` as the normative work artifact.

A Contract Context Pack is a bounded, inspectable, authority-aware evidence selection linked to one contract. It contains the smallest high-signal set of refs, summaries, exclusions, contradictions, unknowns, and retrieval receipts needed to support the contract’s material clauses, scope boundaries, risks, non-goals, validators, and proof requirements.

The pack is evidence. It is not final decision authority, not project truth by itself, not a retrieval engine, not a prompt manager, and not a fourth lifecycle phase.

## Required invariants

1. No orphan context: every included item supports a contract clause, scope boundary, risk, non-goal, validator, or proof requirement.
2. No unsupported material clause: every material contract clause has a source ref, user clarification, explicit assumption, or explicit unknown.
3. Bidirectional support/coverage mapping: every `covered_by` item ref resolves to an included context item that supports that clause, and every included item support claim appears in clause coverage.
4. Authority before similarity: canonical active sources outrank advisory sources; speculative sources are excluded by default; stale/superseded sources are marked; contradictions are surfaced.
5. Budget is a guardrail, not success: token or item budgets matter only after authority and clause coverage.
6. Retrieval remains advisory: search, ranking, indexes, and retrieval receipts cannot own truth or write decisions.
7. Executor briefs are derived: a brief may summarize the contract/context pack for a human, script, model, or agent, but cannot add requirements or override acceptance criteria.

## What belongs where

| Surface | Belongs there | Must stay out |
|---|---|---|
| Contract | Scope, deliverable, acceptance criteria, validators, decision refs, research refs, blocking unknowns, optional context pack ref/hash. | Large excerpts, retrieval chatter, executor prompt scaffolding. |
| Contract Context Pack | Selected refs, rationales, clause mappings, authority/status metadata, exclusions, contradictions, unknowns, retrieval receipt refs, budget metadata. | Final decisions, canonical truth promotion, executor-local memory. |
| Executor brief | Derived task-scoped handoff from contract and selected context pack subset. | New requirements, authority claims, acceptance changes. |

## Adoption map

Adopt now: term, boundary policy, no-orphan-context rule, no-unsupported-clause rule, authority/status filtering, explicit exclusions/contradictions/unknowns, side-effect-free contract-context-pack model/validator in `punk-contract`, and eval/spec coverage.

Defer: runtime storage, persisted writer, compression heuristics, context scoring beyond deterministic checks, executor brief generator.

Park: repo-search adapter integration, reranking, vector indexes, contextual retrieval preprocessing, multi-agent context isolation, persistent memory tooling.

Avoid: giant prompts, fixed top-k as sole policy, embeddings/indexes as project truth, executor-local memory as hidden truth, retrieval writing gate decisions.

## Required evals

- underspecified goal requires clarification before executable contract;
- every included context item maps to a contract clause;
- every material contract clause has source, clarification, assumption, or unknown;
- overstuffed context is rejected, compressed, or narrowed;
- stale/superseded docs are flagged;
- speculative knowledge is excluded by default;
- contradictions are surfaced, not flattened;
- executor brief cannot override contract or acceptance criteria;
- retrieval receipt is advisory and cannot write a decision;
- gate decision cites evidence/proof, not executor claims;
- context pack remains rebuildable from refs/receipts/metadata.

## Required docs / ADRs / contracts

- `docs/adr/ADR-0016-contract-context-pack-boundary.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/product/GLOSSARY.md`
- `evals/specs/context-pack-boundary.v0.1.md`
- future bounded implementation goal/contract before runtime storage or CLI.

## Open questions

1. Should the first version live as an optional field in the contract schema or as a separate linked artifact?
2. Should early selection be fully deterministic before model-assisted summarization?
3. What default item/token budget should each contract class receive?
4. What is the smallest receipt schema sufficient for later repo-search promotion?
5. When canonical docs conflict, should `plot` block automatically or allow a contract with explicit unresolved contradiction?

## Recommendation

Adopt Contract Context Pack as an active-core boundary principle and a side-effect-free `punk-contract` model/validator. Defer runtime storage, retrieval, compression, executor-brief generation, and CLI behavior until later phase gates.
