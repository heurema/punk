---
id: idea_lean_ctx_style_context_runtime
kind: idea
status: parked
authority: speculative
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
review_after: 2026-07-05
related_research:
  - knowledge/research/2026-05-05-contract-bounded-context-compression.md
related_adrs:
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
related_evals:
  - evals/specs/context-pack-boundary.v0.1.md
  - evals/specs/context-pack-compiler-boundary.v0.1.md
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/PROJECT-MEMORY.md
supersedes: []
superseded_by: null
---

# Parked Idea: lean-ctx-Style Context Runtime

## Status

Parked / future only.

This note does not describe active Punk behavior. It does not promote MCP,
shell hooks, hidden session memory, embeddings, vector DB, cloud sync, runtime
context storage, or compressed context as proof.

## Useful ideas

Punk may later borrow these ideas as design/eval inputs:

- mode-aware reads: `full`, `diff`, `signatures`, `map`, `snippet`;
- deterministic shell/log compaction;
- content-addressed cache stubs;
- tree-sitter or ctags signatures;
- project graph or index freshness checks;
- raw fallback refs for every compressed or derived view;
- token-savings benchmarks paired with evidence-preservation checks.

## Parked surfaces

These remain parked until separate Research Gate, ADR, receipt schema, eval, and
proof work promote them:

- MCP context server;
- shell hook default behavior;
- session memory or CCP;
- hidden cross-chat memory;
- cloud or shared context sync;
- embeddings or vector DB;
- runtime context store;
- active context-pack writer;
- executor brief generator.

## Hard boundaries

- Contract Context Pack remains derived evidence selected during `plot`.
- Raw and canonical artifacts outrank derived or compressed views.
- Compressed output cannot satisfy proof requirements by itself.
- Cache hits and signature views are orientation aids, not authority.
- Stale indexes or cache state must fail closed.
- `cut` must not silently mutate an approved context pack.

## Promotion requirements

Before this idea can move out of parked status:

1. Research Gate classifies and accepts a bounded runtime/storage problem.
2. A narrow ADR defines the new decision without duplicating ADR-0016.
3. A receipt schema exists for compressed/log output and cache/index provenance.
4. Eval specs cover raw fallback, stale state, deterministic rebuild, and
   proof-boundary behavior.
5. Proof exists that derived state remains non-authoritative.
6. Runtime storage authority and `.punk/` write boundaries are selected.

## Suggested future experiment

Run a lean-ctx-style comparator on a fixture repo and measure:

- token reduction;
- failure/warning preservation;
- raw fallback availability;
- stale cache/index behavior;
- deterministic reproducibility;
- clause coverage usefulness.

The experiment must remain research evidence until reviewed and promoted.
