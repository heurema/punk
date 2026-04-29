---
id: research_2026_04_29_project_knowledge_vault_for_agents
title: "Project Knowledge Vault for Agents"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-29
updated_at: 2026-04-29
review_after: 2026-07-29
components: [project-memory, knowledge-vault, context-compiler, code-graph]
related_goals:
  - goal_record_project_knowledge_vault_direction_v0_1
related_files:
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ARCHITECTURE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
  - evals/specs/context-pack-boundary.v0.1.md
source_refs:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/CONTRACT-TRACKER.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/DOGFOODING.md
  - docs/product/LINEAGE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
  - evals/specs/context-pack-boundary.v0.1.md
  - https://www.sciencestack.ai/paper/2512.13564v1
  - https://gist.github.com/karpathy
  - https://proceedings.iclr.cc/paper_files/paper/2025/hash/4a4a3c197deac042461c677219efd36c-Abstract-Conference.html
  - https://www.researchgate.net/publication/403308415_Codebase-Memory_Tree-Sitter-Based_Knowledge_Graphs_for_LLM_Code_Exploration_via_MCP
  - https://huggingface.co/papers/2507.07957
  - https://github.com/mem0ai/mem0
  - https://huggingface.co/papers/2502.12110
  - https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents
  - https://www.langchain.com/blog/context-engineering-for-agents
  - https://direct.mit.edu/tacl/article/doi/10.1162/tacl_a_00638/119630/Lost-in-the-Middle-How-Language-Models-Use-Long
  - https://www.trychroma.com/research/context-rot
confidence: medium
research_level: R2
---

# Project Knowledge Vault for Agents

## Question

How should Punk think about project knowledge and memory for AI-assisted development without collapsing into RAG, hidden agent memory, or self-updating docs as truth?

## Decision context

Punk is a local-first bounded work kernel. Its lifecycle remains `plot -> cut -> gate`, and only `gate` writes final decisions. Project memory must remain explicit, repo-tracked where possible, and authority-tagged.

Retrieval can provide evidence. It cannot own truth, approve work, write decisions, or silently promote remembered content into implementation truth.

Contract Context Pack is now the first practical brick: a side-effect-free, contract-linked evidence-selection boundary that keeps task context bounded, cited, and advisory.

This note records architecture/product direction only. It does not implement runtime behavior, storage, graph DB, embeddings, vector DB, retrieval adapters, compiled wiki generation, context compiler, daemon, MCP runtime, CLI behavior, or Knowledge Vault.

## Sources reviewed

This note uses current Punk canonical docs as the strongest inputs. It also uses the source list below as external/user-provided research context. External sources were not independently revalidated during this patch unless explicitly marked otherwise, so their claims are used conservatively as directional inputs.

- Memory in the Age of AI Agents, external/user-provided research input: `https://www.sciencestack.ai/paper/2512.13564v1`
- Karpathy LLM Wiki gist, external/user-provided research input: `https://gist.github.com/karpathy`
- RepoGraph, external/user-provided research input: `https://proceedings.iclr.cc/paper_files/paper/2025/hash/4a4a3c197deac042461c677219efd36c-Abstract-Conference.html`
- Codebase-Memory, external/user-provided research input: `https://www.researchgate.net/publication/403308415_Codebase-Memory_Tree-Sitter-Based_Knowledge_Graphs_for_LLM_Code_Exploration_via_MCP`
- MIRIX, external/user-provided research input: `https://huggingface.co/papers/2507.07957`
- Mem0, external/user-provided research input: `https://github.com/mem0ai/mem0`
- A-MEM, external/user-provided research input: `https://huggingface.co/papers/2502.12110`
- Anthropic context engineering, external/user-provided research input: `https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents`
- LangChain context engineering, external/user-provided research input: `https://www.langchain.com/blog/context-engineering-for-agents`
- Lost in the Middle, external/user-provided research input: `https://direct.mit.edu/tacl/article/doi/10.1162/tacl_a_00638/119630/Lost-in-the-Middle-How-Language-Models-Use-Long`
- Chroma Context Rot, external/user-provided research input: `https://www.trychroma.com/research/context-rot`

## Source quality table

| Source set | Tier | Why included | Key limitation |
|---|---:|---|---|
| Punk canonical docs: README, START-HERE, PUNK-LAWS, ARCHITECTURE, PROJECT-MEMORY, CONTRACT-TRACKER, RESEARCH-GATE, DOGFOODING, LINEAGE, DOCUMENTATION-MAP | A | Current source of truth for lifecycle, authority, project memory, research gates, and active/not-active boundaries | Internal project truth, not external validation |
| ADR-0016 and `context-pack-boundary.v0.1` | A | Defines the first practical boundary for task-specific context selection | Proposed/advisory boundary, not final gate acceptance |
| Proceedings/papers such as RepoGraph, Lost in the Middle, and agent-memory papers | A/B | Support the need for code/project structure, context selectivity, and memory governance | Used from user-provided refs; not fully independently inspected in this patch |
| Official/vendor engineering docs such as Anthropic and LangChain context engineering | B | Useful operational vocabulary for context selection, compression, and agent context control | Advisory guidance; vendor framing may optimize for their systems |
| Repos such as Mem0 | B | Useful examples of agent memory system shape and failure modes | External implementation choices may not fit Punk laws |
| Gists/blogs/promotional or secondary pages | C | Useful as idea prompts and vocabulary | Not authority for Punk product truth |

## Main finding

Knowledge Vault is not RAG.

RAG answers:

```text
which chunks might help answer this query?
```

Knowledge Vault answers:

```text
what is true now?
who/what made it true?
what changed?
what became stale?
what contradicts what?
what is canonical vs advisory?
what context is enough for this contract?
```

Vector search or RAG may later help discover candidate evidence. It must not become the center of Project Memory architecture.

## Architecture direction

For Punk, Knowledge Vault should be understood as:

```text
canonical artifacts
  -> extracted claims / facts / relations
  -> receipts / provenance
  -> code graph + project graph
  -> compiled wiki / derived views
  -> context pack for current task
```

The center is a repo-first artifact ledger with authority, status, provenance, stale conditions, conflicts, and rebuildable derived views. Retrieval is a late auxiliary tactic for candidate discovery.

## Canonical source layer

Different artifacts are true about different things:

- code: implemented behavior;
- goals/roadmap: intended direction;
- contracts: approved bounded scope;
- ADRs: accepted architecture decisions;
- evals/tests: verified checks and regression expectations;
- reports/receipts: what happened;
- gate/proof: final accepted/rejected work state;
- public narrative: public claims after receipts.

Explicit boundary:

```text
code truth != product truth
product goals truth != implemented behavior
ADR truth != current code guarantee
eval truth != acceptance
gate/proof truth = final accepted state
```

This matters because a code-only memory, a wiki-only memory, or an LLM-summary memory will flatten authority. Punk needs artifact-specific truth and explicit promotion paths.

## Claim layer

Future Knowledge Vault work may extract typed claims, facts, and relations from canonical and advisory artifacts. Claims should carry source refs, authority, status, provenance, stale conditions, and supersession/conflict metadata.

Example direction only, not active schema:

```yaml
claim:
  id: claim_contract_context_pack_advisory
  kind: architecture_boundary
  statement: "Contract Context Pack is advisory evidence, not project truth."
  source_refs:
    - docs/adr/ADR-0016-contract-context-pack-boundary.md
    - docs/product/ARCHITECTURE.md
  authority: canonical
  status: active
  stale_if_changed:
    - docs/product/PUNK-LAWS.md
    - crates/punk-contract/src/lib.rs
```

This sketch is not an active schema, runtime contract, or storage format.

## Graph layer

Future Knowledge Vault direction includes two derived graphs.

Project memory graph:

```text
goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative
```

Code graph:

```text
file/module/symbol/test/eval/helper -> calls/owns/verifies/depends-on -> related artifacts
```

Both graphs are derived and rebuildable. They can guide context selection, drift detection, and impact analysis, but they do not own authority. If a graph conflicts with canonical artifacts, the canonical artifact plus gate/proof path wins.

## Compiled wiki

A compiled project wiki may become a useful derived view over claims, graphs, receipts, and canonical docs.

It must:

- cite source refs;
- carry authority/status metadata;
- show stale, superseded, conflict, and unknown signals;
- link back to receipts/provenance;
- be rebuildable from repo artifacts and derived indexes.

It must never become canonical truth just because it is convenient to read.

## Context compiler

Future direction:

```text
request / goal / contract draft
  -> classify task
  -> resolve scope
  -> select authoritative sources
  -> include code graph neighbors
  -> include decisions/contracts/evals
  -> flag stale/conflicts/unknowns
  -> enforce budget
  -> emit context pack + receipt
```

The Context Compiler is a future advisory boundary. It may compile task-specific context packs, but it cannot write decisions, approve work, execute work, or own truth.

Contract Context Pack is the first manual/side-effect-free step toward this direction.

## Automatic update rule

```text
Automatic update means:
  detect, invalidate, propose, rebuild derived views.

It does not mean:
  automatically rewrite canonical truth without review/gate path.
```

A future system may detect stale docs, invalidate derived views, propose a knowledge update, or rebuild a compiled wiki. It must not directly rewrite canonical laws, product docs, ADRs, contracts, or public narrative without a reviewed/promoted artifact path.

## Metrics / eval direction

Prefer Punk-specific metrics over generic RAG metrics:

- authority correctness;
- stale detection recall;
- claim-source coverage;
- code-doc drift detection;
- contract-clause coverage;
- contradiction surfacing;
- rebuildability;
- receipt completeness;
- context-pack orphan rate.

RAG metrics may be useful later for semantic candidate retrieval only. They should not measure whether the Knowledge Vault is true.

## Adoption map

### Adopt now as architectural direction

- Knowledge Vault as repo-first artifact ledger + rebuildable derived views.
- Canonical vs derived split.
- Authority/status/supersession metadata.
- Typed claims with source refs and stale conditions.
- Code graph as future primary code-context layer.
- Project memory graph: `goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative`.
- Compiled wiki as derived view, not truth.
- Context packs instead of giant prompts.
- Stale/conflict/unknown surfacing.
- Receipts/provenance for generated views.
- RAG/vector search only as late candidate discovery.

### Defer

- Automatic canonical doc rewriting.
- Semantic classifier as authority.
- Model-generated knowledge promotion.
- Automatic business-strategy rewriting.
- Heavy graph DB.
- Always-on daemon.
- MCP-heavy runtime.
- Compiled wiki generator implementation.
- Context compiler implementation.

### Park

- Vector DB as central vault.
- Multi-agent memory managers.
- Cloud memory service.
- Persistent shared agent memory.
- Autonomous self-healing docs that commit directly.
- Retrieval subagents.
- RAG-as-primary-memory architecture.

### Avoid

- “RAG = Knowledge Vault”.
- “LLM summary = truth”.
- “Code is the only truth”.
- “Wiki page is canonical because it is convenient”.
- “Agent remembered it, therefore it is project memory”.
- “Put everything into the context window”.

## Impact on roadmap

- No immediate runtime implementation.
- Adds direction for future Project Memory / Knowledge Vault phases.
- Strengthens Memory-Safe Core Development.
- Supports a future Knowledge Impact Report convention.
- Does not change current CLI surface.

## Required evals later

Future eval ideas only:

- stale doc detection;
- orphan context rejection;
- speculative source exclusion;
- code change requires knowledge impact;
- contradictory docs surfaced;
- derived wiki cites sources;
- context pack excludes speculative/parked claims.

## Required docs / ADRs / contracts

- This note may later support an ADR.
- Knowledge Impact Report convention should be a separate next patch.
- Full implementation requires later goals/contracts/evals.

## Open questions

- Should code-derived facts be canonical or evidence?
- Where should claims live?
- Manual vs semi-auto claim extraction?
- How automatic can compiled wiki updates be?
- What should block `plot`: stale docs, unresolved contradictions, missing evals, or only high-risk conflicts?
