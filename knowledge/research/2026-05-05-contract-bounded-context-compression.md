---
id: research_contract_bounded_context_compression_2026_05_05
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
review_after: 2026-06-05
classification: r2-design-research
canonical_for: []
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/RESEARCH-GATE.md
related_adrs:
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
related_evals:
  - evals/specs/context-pack-boundary.v0.1.md
  - evals/specs/context-pack-compiler-boundary.v0.1.md
supersedes: []
superseded_by: null
---

# Contract-Bounded Deterministic Context Compression

## Verdict

Adopt as design/eval direction only.

Punk should treat a Contract Context Pack as deterministic derived evidence
selected during `plot`, frozen before `cut`, and verified later through raw refs,
receipts, evals, and gate/proof surfaces.

This note does not promote runtime context-pack storage, MCP integration, shell
hooks, embeddings, vector databases, hidden session memory, cross-chat memory,
LLM summarization, proofpack writing, or gate writing into active behavior.

## Question

How should Punk construct a Contract Context Pack that gives an executor the
minimum sufficient context needed for a bounded task while preserving proof,
auditability, raw-source traceability, reproducibility, and resistance to hidden
memory or stale-index errors?

## Decision context

The active lifecycle remains:

```text
plot -> cut -> gate
```

Context selection belongs to `plot`. `cut` consumes the frozen pack and records
new context requests, deviations, validator output, and receipts. `gate` verifies
raw refs, receipts, evals, and proof requirements before writing final decisions.

A Contract Context Pack is not a fourth lifecycle phase, prompt manager,
retrieval engine, project truth, proofpack, or executor-local memory store.

## Source quality table

| Source | URL | Tier | Use |
|---|---|---:|---|
| Existing Punk Context Pack research | knowledge/research/2026-04-28-context-pack-boundary.md | A | Current repo-local boundary and terminology. |
| ADR-0016 Contract Context Pack boundary | docs/adr/ADR-0016-contract-context-pack-boundary.md | A | Existing decision surface; do not duplicate with ADR-0017 unless a distinct decision appears. |
| Context Pack boundary eval spec | evals/specs/context-pack-boundary.v0.1.md | A | Current eval owner for pack invariants. |
| Context-Pack Compiler boundary eval spec | evals/specs/context-pack-compiler-boundary.v0.1.md | A | Future side-effect-free compiler boundary. |
| lean-ctx repository | https://github.com/yvgude/lean-ctx | A/B | Comparator for local context runtime, cache stubs, signatures, MCP, and shell/log compaction. Inspect code before adopting claims. |
| Aider repo map docs | https://aider.chat/docs/repomap.html | A/B | Prior art for compact repo-wide structural context. |
| Language Server Protocol 3.17 | https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/ | A | Deterministic definitions/references/symbol navigation direction. |
| SCIP repository | https://github.com/sourcegraph/scip | A | Precise code intelligence index model for future derived views. |
| Kythe docs | https://kythe.io/docs/ | A | Future code graph fact model inspiration. |
| Glean docs | https://glean.software/docs/ | A | Future fact database comparator, not MVP infra. |
| tree-sitter docs | https://tree-sitter.github.io/tree-sitter/ | A | Deterministic AST/signature extraction direction. |
| Universal Ctags | https://github.com/universal-ctags/ctags | A | Low-complexity symbol extraction fallback. |
| ripgrep | https://github.com/BurntSushi/ripgrep | A | Deterministic lexical retrieval baseline. |
| Zoekt | https://github.com/sourcegraph/zoekt | A | Future trigram code search comparator. |
| CodeQL docs | https://codeql.github.com/docs/ | A | Future static analysis/context source. |
| Semgrep docs | https://semgrep.dev/docs/ | A | Future pattern analysis/context source. |
| RepoBench paper | https://huggingface.co/papers/2306.03091 | A | Repository-level retrieval/completion benchmark shape. |
| SWE-bench | https://www.swebench.com/SWE-bench/ | A | End-task benchmark comparator, not context-quality authority. |
| Lost in the Middle | https://direct.mit.edu/tacl/article/doi/10.1162/tacl_a_00638/119630/Lost-in-the-Middle-How-Language-Models-Use-Long | A | Supports minimal, placement-aware packs over giant prompts. |
| LongLLMLingua | https://arxiv.org/abs/2310.06839 | A | Learned compression comparator; advisory only for Punk. |
| LLMLingua | https://arxiv.org/abs/2310.05736 | A | Learned compression comparator; advisory only for Punk. |
| Continue docs | https://docs.continue.dev/ | B | Comparator for agent context provider patterns. |
| Cursor docs | https://docs.cursor.com/ | B | Comparator for index/search/agent context behavior. |
| Sourcegraph Cody docs | https://sourcegraph.com/docs/cody | B | Comparator for code intelligence plus assistant context. |
| Claude Code memory docs | https://docs.anthropic.com/en/docs/claude-code/memory | B | Comparator for explicit memory risks. |

Tier C sources were not used as decision-supporting evidence.

## Main finding

The right direction is not generic RAG and not LLM prompt compression first.

The safer Punk direction is:

```text
contract clauses / risks / non-goals / validators / proof requirements
  -> deterministic candidate source universe
  -> minimal context selection
  -> compressed or derived views with raw fallbacks
  -> clause coverage map
  -> retrieval/context receipts
  -> frozen pack hash
```

Learned retrieval, embeddings, rerankers, LLM summaries, and prompt compression
may later propose candidate sources. They remain advisory until accepted into a
pack through deterministic, inspectable rules and raw artifact refs.

## Adoption map

### Adopt as design/eval direction

- Contract Context Pack is derived evidence, not truth.
- Context selection is produced and frozen during `plot`.
- Material clause coverage is mandatory.
- Non-goals and exclusions are first-class coverage surfaces.
- Compressed views require raw fallback refs.
- Summary, signature, cache stub, or compressed view cannot satisfy proof alone.
- Cache/index provenance must fail closed when stale, mismatched, or missing.
- Cut-time discoveries are deviations, amendments, re-plot inputs, or gate
  handling inputs, not silent pack mutation.
- Gate verifies raw artifacts and receipts, not summaries.

### Defer

- runtime context-pack writer;
- context scoring beyond deterministic checks;
- graph scoring, PageRank, or heat diffusion;
- embeddings and semantic retrieval;
- learned reranking;
- LLM summarization;
- context compression beyond deterministic derived views;
- executor brief generation.

### Park

- MCP context runtime;
- shell hook default behavior;
- session memory or CCP;
- hidden cross-chat memory;
- cloud/shared context sync;
- vector DB as project memory;
- persistent context runtime.

### Avoid

- summary-only proof;
- hidden memory as truth;
- token savings as the primary success metric;
- derived cache/index as project truth;
- context mutation during `cut` without deviation or re-plot handling;
- fixed top-k retrieval as the sole policy.

## Mechanism classification

| Mechanism | Classification | Notes |
|---|---|---|
| Lexical retrieval with `ripgrep` or equivalent | adopt as design direction | Deterministic, inspectable, strong for identifiers/errors/tests/logs. |
| Diff-based context | adopt as design direction | Naturally bounded but must include non-diff invariant files when clauses require them. |
| Static-symbol retrieval | defer implementation, adopt spec/eval direction | LSP, ctags, SCIP, tree-sitter signatures can provide traceable refs if provenance is recorded. |
| AST signatures | adopt as compressed view direction | Useful orientation context; never proof alone. |
| CLI/log compression | adopt eval/spec direction | Must preserve command, exit status, failures, warnings, omissions, and raw log refs. |
| Cache/delta stubs | defer runtime, adopt receipt concept | Must be content-addressed, repo-namespaced, hash-based, and fail closed when stale. |
| Code graph or dependency graph context | defer broad systems | Useful for nonlocal tasks, but heavy and stale-risky. |
| Learned/LLM compression | advisory only | Lossy and hard to reproduce; cannot own proof. |
| Embedding/RAG context | defer as candidate generator only | Useful for vague tasks; opaque and stale-risky. |
| Agent memory/session continuity | park | Only explicit, authority-tagged, reviewed memory can enter durable Punk artifacts. |

## lean-ctx comparator

Useful ideas to borrow later:

- mode-aware reads such as full, diff, signatures, map, and snippet;
- deterministic shell/log compaction;
- content-addressed cache stubs;
- tree-sitter signatures;
- graph or index freshness checks;
- raw fallback and tee-style log preservation;
- token-savings benchmarks paired with evidence-preservation checks.

Weak fit for active Punk:

- MCP runtime as active-core surface;
- shell hook as default operator path;
- CCP/session memory as truth;
- cloud sync or remote context state;
- compressed output satisfying proof by itself;
- benchmark savings as correctness evidence.

## Required eval intake

`evals/specs/context-pack-boundary.v0.1.md` should explicitly cover:

- raw fallback for compressed views;
- no summary-only proof;
- stale cache/index fail-closed behavior;
- memory/advisory source boundary;
- deterministic rebuild for fixed inputs;
- frozen-after-plot behavior;
- cut-time discovery as deviation/amendment/re-plot/gate handling;
- CLI/log compaction preserving failures, warnings, commands, and raw log refs;
- material clause coverage or explicit unknown;
- non-goal and exclusion coverage.

## Open questions

1. Should compressed CLI output belong to context pack v0.1 or a separate
   shell/log receipt spec?
2. Should new material context discovered during `cut` require re-plot by
   default, or can a narrow gate-handled deviation be sufficient?
3. Which deterministic source universe is the first practical target: diff +
   grep + explicit refs only, or diff + grep + symbols?
4. Should `lean-ctx` be used as a benchmark comparator or remain research-only?

## Recommendation

Do not create a duplicate ADR for the existing Contract Context Pack boundary.
`docs/adr/ADR-0016-contract-context-pack-boundary.md` is the current decision
surface. Add eval/research detail to the existing boundary line unless a future
separate decision emerges.

Implement nothing runtime-first. The next safe step is eval/spec and
side-effect-free model refinement only.
