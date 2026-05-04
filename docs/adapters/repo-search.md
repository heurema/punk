# Repo Search Adapter

## Status

- Status: `parked`
- Authority: `advisory`
- Current surface: documentation only
- Future owner: `punk-adapters::repo_search`
- Roadmap phase: Phase 9 adapters
- Related research:
  - `knowledge/research/2026-04-19-repo-search-adapter-options.md`
  - `knowledge/research/2026-05-03-codebase-understanding-for-agents.md`
- Related eval specs:
  - `evals/specs/repo-search-adapter-boundary.v0.1.md`
  - `evals/specs/codebase-context-pack-boundary.v0.1.md`

This document defines a future boundary for repository and code retrieval.
It does not activate an adapter and does not expose public CLI behavior.

## Purpose

A repo-search adapter should help a module or operator find code evidence
quickly without turning retrieval into a source of truth.

The boundary term is **Codebase Evidence Boundary for Agents**:

```text
live workspace evidence first
derived indexes second
semantic memory never truth
```

The adapter exists to answer bounded questions such as:

- which files are likely relevant to this contract?
- where is a symbol defined?
- where is a symbol referenced?
- which files contain a literal pattern?
- what should be read next?

The adapter must return advisory evidence, not decisions.
It may locate bounded repository evidence. It must not claim to reconstruct
project truth or make agent-owned understanding authoritative.

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
- write contracts
- write proofpacks
- write gate decisions
- write acceptance claims
- expose editing or refactoring tools as part of the first read-only boundary
- treat MCP tools as authority owners
- use remote search before an external side-effect policy exists

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

Remote search remains parked until Punk has an external side-effect policy.

MCP tools are adapter candidates only. They cannot own authority, write project
memory, or decide acceptance.

## Initial capability model

The smallest future adapter surface should be read-only.

First mandatory future modes:

```text
mode:
  files       # find candidate files/modules
  grep        # literal or regex content search
  multi_grep  # bounded OR/multi-pattern search
```

Optional future modes:

```text
mode:
  symbols      # symbol overview or lookup
  definitions  # symbol definitions
  references   # symbol references
```

A backend does not need to implement every mode. Missing modes should be
declared in its manifest and covered by conformance tests.

Optional modes require backend capability declaration plus freshness, index, or
cache warnings when those affect results.

## Request shape

```yaml
query: string
mode: files | grep | multi_grep | symbols | definitions | references
constraints:
  include_paths: []
  exclude_paths: []
  globs: []
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
    snippet_hash: optional string
    metadata: map
ranking:
  backend: string
  deterministic: boolean
  ranking_factors: []
  personalized: boolean
  fallback_mode_used: boolean
receipt:
  id: string
  backend: string
  backend_version: string
  backend_kind: lexical | fuzzy | lsp | tree_sitter | code_graph | vector | remote
  capability_manifest_ref: string
  query: string
  mode: string
  constraints: map
  budget: map
  result_count: number
  elapsed_ms: number
  truncated: boolean
  index_state: none | live | stale | unknown
  local_only: boolean
  network_used: boolean
  side_effects: none | declared
  contract_ref: optional string
  goal_ref: optional string
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
8. It denies network use by default.
9. It reports unsupported modes instead of hiding fallback navigation.
10. It reports truncation, stale indexes, skipped files, and omissions.

## Relationship to Contract Context Packs

A future repo-search adapter may provide candidate refs for a Contract Context
Pack, but only through advisory retrieval receipts.

The adapter must not decide which context is authoritative, must not promote
retrieved content into project truth, and must not write gate decisions.

If retrieved content is included in a context pack, the pack must still record
clause support, source authority/status, exclusions, contradictions, unknowns,
and retrieval receipt refs.

## Agent usage discipline

Future agent-facing instructions should prefer bounded retrieval:

- use file search to explore project structure
- use content search for known identifiers or literals
- use multi-pattern search instead of repeated near-identical searches
- use symbol search when a backend provides reliable language-server support
- preserve retrieval receipt refs when results feed a Contract Context Pack
- read the top relevant artifact after a small number of searches
- stop searching when enough evidence exists for the current contract

The adapter should support a search budget so an agent cannot loop
indefinitely through retrieval tools.

## Receipts

A retrieval receipt is evidence, not truth.

A receipt should record:

- backend and version
- backend kind and capability manifest ref
- query/mode/constraints
- budget
- elapsed time
- result count
- truncation state
- top result paths
- ranking metadata when available
- index/cache provenance when available
- stale-index warnings
- skipped or omitted files
- unsupported modes
- privacy or secret warnings
- no-network assertion
- side-effect assertion
- `goal_ref` and `contract_ref` when used

Receipts may later be referenced by run receipts, reports, eval outputs, or
proofpacks.

## Required evals before promotion

Before this boundary becomes active, add evals for:

- known-file retrieval on fixture repos
- known-literal grep retrieval on fixture repos
- multi-grep bounded OR retrieval
- known-symbol retrieval on fixture repos, optional
- known-reference retrieval on fixture repos, optional
- bounded search budget enforcement
- stale index detection
- reproducibility across repeated runs
- latency budget on small and medium repos
- output truncation behavior
- skipped/omitted file reporting
- no-network default
- secret/env redaction in receipts
- receipt completeness
- no-decision/no-write/no-project-memory conformance
- Contract Context Pack receipt preservation

## Open questions

1. Should `punk` bless one default backend or support multiple backends?
2. Should symbol-level retrieval be part of the first read-only adapter?
3. Should remote code search be allowed only after an external side-effect policy?
4. Where should retrieval receipts live by default: `.punk/runs/`, `.punk/views/`, or both?
5. How much ranking metadata is required for a proofpack to be meaningful?
