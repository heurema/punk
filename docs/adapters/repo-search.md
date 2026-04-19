# Repo Search Adapter

## Status

- Status: `parked`
- Authority: `advisory`
- Current surface: documentation only
- Future owner: `punk-adapters::repo_search`
- Roadmap phase: Phase 9 adapters
- Related research: `knowledge/research/2026-04-19-repo-search-adapter-options.md`

This document defines a future boundary for repository and code retrieval.
It does not activate an adapter and does not expose public CLI behavior.

## Purpose

A repo-search adapter should help a module or operator find code evidence
quickly without turning retrieval into a source of truth.

The adapter exists to answer bounded questions such as:

- which files are likely relevant to this contract?
- where is a symbol defined?
- where is a symbol referenced?
- which files contain a literal pattern?
- what should be read next?

The adapter must return advisory evidence, not decisions.

## Non-goals

The first repo-search adapter must not:

- write code
- refactor code
- edit files
- publish external side effects
- decide whether work is accepted
- replace `gate`
- replace project memory
- promote hidden retrieval memory into canonical truth

Symbolic editing and refactoring tools belong to a separate future decision.

## Candidate backends

Candidate backends are research inputs, not committed dependencies.

| Backend class | Examples | Strength | Risk |
|---|---|---|---|
| Text/fuzzy local search | `ripgrep`, `fd`, `fff-mcp` | fast, local, inspectable | lexical misses, ranking heuristics |
| Symbol/LSP retrieval | Serena-style symbol tools, LSP | structure-aware, token-efficient | language support gaps, cache/index state |
| Code graph search | LSIF/SCIP/graph systems | strong references and definitions | setup cost, stale graph risk |
| Remote search | Sourcegraph/Zoekt-like services | scale and cross-repo lookup | external side effects, auth, hidden index |
| Semantic/vector search | embeddings/vector DB | concept search | nondeterminism, hidden ranking, stale vectors |

## Initial capability model

The smallest future adapter surface should be read-only.

```text
mode:
  files       # find candidate files/modules
  grep        # literal or regex content search
  multi_grep  # bounded OR/multi-pattern search
  symbols     # optional: symbol definition/overview
  references  # optional: symbol references
```

A backend does not need to implement every mode. Missing modes should be
declared in its manifest and covered by conformance tests.

## Request shape

```yaml
query: string
mode: files | grep | multi_grep | symbols | references
constraints:
  paths: []
  globs: []
  exclude: []
  languages: []
budget:
  max_results: 20
  max_elapsed_ms: 1000
  max_answer_chars: 12000
trace:
  contract_ref: optional
  goal_ref: optional
```

## Response shape

```yaml
results:
  - path: string
    line: optional number
    column: optional number
    kind: file | match | symbol | reference
    score: optional number
    snippet: optional string
    metadata: map
ranking:
  backend: string
  ranking_factors: []
receipt:
  backend: string
  backend_version: string
  query: string
  mode: string
  constraints: map
  result_count: number
  elapsed_ms: number
  index_state: optional map
  warnings: []
```

## Guarantees

A conforming repo-search adapter must guarantee:

1. It does not write final decisions.
2. It does not write project memory directly.
3. It emits inspectable retrieval receipts when used inside a run.
4. It declares backend capabilities and limitations.
5. It separates canonical repo artifacts from runtime retrieval caches.
6. It exposes enough provenance to diagnose stale or surprising results.
7. It can be disabled without corrupting project state.

## Agent usage discipline

Future agent-facing instructions should prefer bounded retrieval:

- use file search to explore project structure
- use content search for known identifiers or literals
- use multi-pattern search instead of repeated near-identical searches
- use symbol search when a backend provides reliable language-server support
- read the top relevant artifact after a small number of searches
- stop searching when enough evidence exists for the current contract

The adapter should support a search budget so an agent cannot loop
indefinitely through retrieval tools.

## Receipts

A retrieval receipt is evidence, not truth.

A receipt should record:

- backend and version
- query/mode/constraints
- elapsed time
- result count
- top result paths
- ranking metadata when available
- index/cache provenance when available
- warnings such as stale index, truncated output, or unsupported mode

Receipts may later be referenced by run receipts, reports, eval outputs, or
proofpacks.

## Required evals before promotion

Before this boundary becomes active, add evals for:

- known-file retrieval on fixture repos
- known-symbol retrieval on fixture repos
- bounded search budget enforcement
- stale index detection
- reproducibility across repeated runs
- latency budget on small and medium repos
- output truncation behavior
- no-decision/no-write conformance

## Open questions

1. Should `punk` bless one default backend or support multiple backends?
2. Should symbol-level retrieval be part of the first read-only adapter?
3. Should remote code search be allowed only after an external side-effect policy?
4. Where should retrieval receipts live by default: `.punk/runs/`, `.punk/views/`, or both?
5. How much ranking metadata is required for a proofpack to be meaningful?
