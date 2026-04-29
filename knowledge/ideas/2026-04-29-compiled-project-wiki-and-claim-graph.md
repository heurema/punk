---
id: idea_2026_04_29_compiled_project_wiki_and_claim_graph
kind: idea
status: parked
authority: speculative
owner: vitaly
created_at: 2026-04-29
updated_at: 2026-04-29
review_after: 2026-07-29
components: [project-memory, knowledge-vault, compiled-wiki, claim-graph, context-compiler]
related_research:
  - knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md
related_goals:
  - goal_record_project_knowledge_vault_direction_v0_1
supersedes: []
superseded_by: null
---

# Compiled Project Wiki and Claim Graph

## Status

This is not product truth.
This is not active implementation.
This is parked/future unless promoted through ADR/roadmap/goal/contract/eval/proof.

## Idea

Punk may later maintain a compiled project wiki as a derived view over a typed claim graph, project memory graph, code graph, receipts, and canonical docs.

The wiki should answer practical questions such as:

- what is true now;
- which artifact made it true;
- what changed recently;
- what is stale or superseded;
- what contradicts what;
- which sources should support the current contract.

It should not answer those questions by treating a vector index or LLM summary as truth.

## Why not RAG-first

RAG primarily asks which chunks may help answer a query. Punk needs a stricter project-memory layer that asks what is authoritative, current, stale, contradicted, accepted, proven, or merely advisory.

RAG/vector search can become a candidate-discovery helper later. It should not become the vault itself.

## Future pieces

### Typed claims

Derived claim records could capture statements, source refs, authority, status, stale conditions, supersession links, contradiction sets, and proof/report refs.

### Claim graph

A claim graph could connect claims to source artifacts, decisions, evals, proofpacks, code refs, and public narrative.

### Code graph

A code graph could connect files, modules, symbols, tests, evals, and helpers. It should be a future primary code-context layer for code-aware contract packs and impact analysis.

### Project graph

The project graph should preserve the existing memory chain:

```text
goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative
```

### Compiled wiki

A compiled wiki could be generated from canonical artifacts plus derived claims/graphs. Every page should cite source refs and show stale/supersession/conflict signals.

The compiled wiki is derived view only. It must never become canonical because it is easier to read than the source artifacts.

### Context compiler

A future Context Compiler could accept a request, goal, or contract draft and produce a bounded context pack with refs, source authority, code graph neighbors, decisions, evals, stale/conflict/unknown flags, budget metadata, and a receipt.

It would be advisory and side-effect-bound. It would not approve work, execute work, write decisions, or own truth.

### Future Knowledge Impact Report

A later convention could require meaningful code/doc changes to say which knowledge claims, docs, derived views, evals, or public narratives are affected.

This idea does not add that convention now.

## Minimal MVP idea

A minimal future MVP could be:

1. a small manually curated claim file or report section;
2. a deterministic linter that checks source refs and authority/status fields;
3. a derived Markdown view that lists active/stale/conflicting claims;
4. a context-pack check that rejects orphan speculative claims;
5. no graph DB, embeddings, daemon, MCP runtime, or autonomous rewrite behavior.

## Non-goals

Do not implement now:

- compiled wiki generator;
- claim schema;
- graph DB;
- embeddings or vector DB;
- retrieval adapter;
- context compiler;
- daemon;
- MCP runtime;
- automatic canonical doc rewriting;
- autonomous self-healing docs;
- Knowledge Impact Report convention.

## Promotion conditions

Before promotion, this idea needs:

- ADR or roadmap placement;
- bounded goal/contract;
- eval/spec cases for authority, stale/conflict handling, and rebuildability;
- proof that derived views remain non-authoritative;
- explicit scope that avoids RAG-as-primary-memory architecture.
