---
kind: idea-bank
status: active
authority: speculative
created: 2026-04-19
related_research: knowledge/research/2026-04-19-repo-search-adapter-options.md
related_decision: docs/adapters/repo-search.md
---

# Code Retrieval Ideas to Preserve

These ideas are useful but should not enter active-core yet.

They remain speculative until promoted through ADR, roadmap, goal/contract,
implementation, and eval results.

## Defer / preserve

### Bounded retrieval budget

Idea: give each contract/run a retrieval budget:

- max search calls
- max result count
- max elapsed time
- max answer characters

Why parked now:

- useful to prevent agent over-search loops
- needs adapter contract and eval coverage first

### Read-after-search rule

Idea: after a small number of search calls, require the operator/agent to read
one or more top artifacts before searching again.

Why parked now:

- may reduce token waste and improve evidence grounding
- needs concrete agent/runtime rules later

### Retrieval receipt as first-class evidence

Idea: every adapter call inside a run emits a receipt with:

- backend
- version
- query
- mode
- constraints
- result count
- top paths
- elapsed time
- ranking metadata
- index/cache state
- warnings

Why parked now:

- the shape is useful now
- storage location and schema still need research

### Retrieval memory is not project memory

Idea: frecency DBs, query histories, symbol indexes, and vector indexes may
improve ranking, but they remain runtime/derived state.

Why parked now:

- important architectural guardrail
- implementation details should wait for real adapter promotion work

### Symbol-first optional path

Idea: for languages with reliable LSP support, symbol search may be more
token-efficient than line-based grep.

Potential modes:

- `symbols`
- `references`
- `overview`

Why parked now:

- useful only if backend capability manifests and eval fixtures exist

### Read-only Serena-style backend

Idea: Serena-like symbolic retrieval is worth exploring, but symbolic editing
and refactoring should be disabled in the first `punk` adapter boundary.

Why parked now:

- retrieval is promising
- editing/refactoring is a larger trust boundary that needs separate research

### FFF-style query discipline

Idea: agent-facing retrieval tools should teach simple, bounded query behavior:

- search bare identifiers
- avoid complex regex by default
- use multi-pattern search for variants
- read evidence instead of endless search variations

Why parked now:

- good operational guidance
- still backend- and eval-dependent

### Stale-index warning

Idea: a backend should warn when index metadata is older than repository state,
or when it cannot prove freshness.

Why parked now:

- valuable for proof/debuggability
- depends on adapter receipts and index metadata shape

### Adapter conformance suite

Idea: each backend should pass a small suite:

- no writes
- no decisions
- capability manifest is accurate
- receipts are emitted
- budget is enforced
- repeated query is stable enough for debugging

Why parked now:

- absolutely needed before promotion
- not needed while the boundary stays docs-only

### Backend capability manifest

Idea: do not assume all backends are equal.

Manifest example:

```yaml
backend: fff-mcp
capabilities:
  files: true
  grep: true
  multi_grep: true
  symbols: false
  references: false
writes: false
external_side_effects: false
state:
  frecency: non_canonical
  index: derived
```

Why parked now:

- the contract is valuable
- real manifest fields should come from research plus evals

## Do not promote without a new ADR

- MCP integration in active-core
- autonomous code modification
- symbolic editing/refactoring
- hidden retrieval state as project truth
- remote code search before external side-effect policy
