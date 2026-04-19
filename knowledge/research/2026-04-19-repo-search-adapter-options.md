---
id: research_2026_04_19_repo_search_adapter_options
title: "Repo search adapter options for punk"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
review_after: 2026-07-19
components: [adapters, repo-search, retrieval, project-memory, eval]
related_goals: []
related_files:
  - docs/adapters/repo-search.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/PROJECT-MEMORY.md
  - knowledge/ideas/2026-04-19-code-retrieval-ideas.md
source_refs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/PROJECT-MEMORY.md
  - https://github.com/dmtrKovalenko/fff.nvim
  - https://github.com/dmtrKovalenko/fff.nvim/blob/main/crates/fff-mcp/src/main.rs
  - https://oraios.github.io/serena/01-about/035_tools.html
  - https://github.com/oraios/serena
confidence: medium
research_level: R2
---

# Repo Search Adapter Options

## Question

What repo-search and code-retrieval approaches should inform a future `punk`
adapter for low-latency, low-token, inspectable code evidence retrieval?

## Decision context

`punk` is core-first. Adapters are parked and must not become active before
the core flow, event, eval, contract, gate, proof, and project memory surfaces
are stable.

Any repo-search capability must preserve these boundaries:

- adapters can invoke tools, but cannot own truth
- modules can assess, but cannot decide
- only `gate` writes the final decision
- project memory is explicit and authority-tagged
- hidden indexes, histories, or frecency stores are not canonical truth

This note is an initial R2 seed. It preserves current conclusions while the
full Deep Research artifact is still pending.

## Sources reviewed

| Source | Tier | Why included | Key limitation |
|---|---|---|---|
| `docs/product/PUNK-LAWS.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `PROJECT-MEMORY.md` | A | Current project constraints and source of truth for active-core boundaries | Internal intent, not external validation |
| `dmtrKovalenko/fff.nvim` README and `fff-mcp` source entrypoint | A/B | Documents local file/content search, MCP stdio model, frecency/history, and query discipline | Project-specific behavior; not evaluated inside `punk` yet |
| Serena docs and repository | A | Documents LSP-backed symbol retrieval, references, and MCP tool surface | Also includes editing/refactoring capabilities that exceed first-scope retrieval |
| Classical `ripgrep` / `fd` workflow | B | Deterministic local baseline and easy eval fixture target | Lexical only; misses semantic relationships |
| Remote indexed code search as an architecture class | B | Useful comparison class for scale/cross-repo search | Side-effect, auth, and hidden-index risks remain unresolved |

## Existing systems / prior art

### FFF / `fff-mcp`

Useful ideas:

- narrow tool specialization
- local stdio MCP server
- file search, grep, and multi-grep style modes
- query discipline for agents
- frecency and git-aware ranking
- healthcheck/install operational pattern

Punk risk:

- frecency/history are useful retrieval memory but must not become project
  memory or implementation truth

### Serena

Useful ideas:

- symbol-level retrieval
- `find_symbol`
- `find_referencing_symbols`
- `get_symbols_overview`
- language-server-backed code structure
- token-efficient navigation in large codebases

Punk risk:

- Serena also exposes editing/refactoring/memory-like capabilities
- a first `punk` repo-search adapter should use read-only retrieval only
- symbolic editing should stay parked until a separate decision exists

### Classical local search

Useful ideas:

- deterministic baseline
- simple dependency surface
- good eval fixture target
- easy receipting and reproduction

Punk risk:

- lexical search misses semantic relationships and can encourage over-searching

### Remote / indexed search

Useful ideas:

- large-scale and cross-repo lookup

Punk risk:

- remote side effects
- authentication boundaries
- hidden/stale indexes
- weaker reproducibility

## Failure modes found

- over-searching instead of reading evidence
- treating ranked results as truth
- stale indexes or language-server cache drift
- hidden retrieval state influencing decisions
- remote search leaking private code or metadata
- token waste from broad result dumps
- missing relevant files because lexical search has low recall
- dangerous coupling of search and editing tools
- adapter results bypassing contract scope or `gate`

## Options considered

### Option A — Build a native search engine now

Decision: avoid.

Reason: too early for the current roadmap and distracts from active-core.

### Option B — Wrap one backend later

Decision: defer.

Reason: simplest first implementation, but risks backend lock-in.

### Option C — Define a backend-agnostic read-only adapter boundary now

Decision: adopt as parked documentation.

Reason: preserves useful design now without activating behavior.

### Option D — Treat retrieval memory as project memory

Decision: avoid.

Reason: violates the explicit authority-tagged memory model.

## Recommendation

Adopt now as parked documentation:

- a read-only `repo-search` adapter boundary
- retrieval receipts as inspectable evidence
- retrieval memory as non-canonical runtime state
- backend capability manifests
- bounded search budget expectations

Defer:

- concrete backend selection
- MCP integration
- remote search
- symbolic editing/refactoring

Avoid:

- active-core dependency on `fff` or Serena
- hidden retrieval truth
- retrieval tools writing decisions or project memory directly

## What stays out of scope

- public CLI behavior
- active MCP integration
- autonomous code editing
- provider-specific orchestration
- remote search without side-effect policy
- project memory replacement
- hidden vector memory as truth

## Impact on roadmap

- Phase 0-5: no implementation impact
- Phase 6: module host should allow advisory retrieval receipts later
- Phase 7: Dev module can cite this research but should not depend on active adapters
- Phase 9: repo-search adapter can be considered for promotion after completed
  R2 research, receipt schema, and eval coverage

## Required evals

Before promotion:

- known-file retrieval on fixture repo
- known-symbol retrieval on fixture repo
- query discipline / bounded-search budget
- repeated-run reproducibility
- stale index detection
- no-write/no-decision conformance
- output-size truncation behavior
- latency budget on small/medium repos

## Required docs / ADRs / contracts

- `docs/adapters/repo-search.md`
- update `docs/product/ARCHITECTURE.md` with parked adapter boundary
- update `docs/product/ROADMAP.md` Phase 9 notes
- update `docs/product/CRATE-STATUS.md` parked adapter docs
- update `docs/product/PROJECT-MEMORY.md` to distinguish retrieval memory from project memory
- later ADR only after the full research report is accepted

## Open questions

1. Should the first backend be `fff-mcp`, `ripgrep/fd`, Serena, or a small native wrapper?
2. Should `symbols` and `references` be in the first adapter contract or an optional extension?
3. Should remote code search be forbidden until side-effect policy exists?
4. Where exactly should retrieval receipts live?
5. How much ranking metadata is enough for proof/debuggability?
