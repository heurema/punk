---
id: ideas_context_pack_future_work_2026_04_29
kind: idea
status: parked
authority: speculative
owner: vitaly
created_at: 2026-04-29
review_after: 2026-07-29
related_research:
  - knowledge/research/2026-04-28-context-pack-boundary.md
related_adrs:
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
supersedes: []
superseded_by: null
---

# Context Pack future work

## Status

Parked / future.

These ideas are not active-core and must not be described as current operator behavior.

## Future ideas

### Context selector strategies

- deterministic first selector based on contract clauses and canonical refs;
- optional model-assisted summarizer after selection;
- budget-aware compression after authority filtering;
- clause coverage scorecard;
- stale/superseded contradiction detector;
- context precision / recall scorecard adapted from RAG evaluation.

### Retrieval integration

- repo-search adapter returns candidate refs and retrieval receipts;
- result ranking remains advisory;
- vector indexes and embeddings remain non-canonical;
- retrieval receipts can be linked from context packs, run receipts, reports, or proofpacks.

### Executor handoff

- task-scoped executor brief generated from contract + context pack subset;
- brief has explicit “derived from” refs;
- brief cannot introduce acceptance criteria;
- brief divergence is an eval failure.

### Context pack storage

- optional `.punk/contracts/<contract-id>/context-pack.json` after runtime contract storage exists;
- derived views under `.punk/views/` after storage authority is settled;
- repo-tracked summaries only when needed for reports, ADRs, or knowledge updates.

## Explicitly not now

Do not implement now: embeddings, reranking, MCP retrieval, provider-specific context routers, autonomous coding agent execution, persistent prompt manager, global agent memory, automatic knowledge promotion, cloud/service-backed context memory.

## Review trigger

Revisit only after Phase 3 contract loop and Phase 4 project-memory authority checks have stronger runtime/eval coverage.
