# Repo Search Adapter Boundary Eval v0.1

Date: 2026-05-03
Status: proposed boundary
Authority: advisory/design

## Purpose

Define advisory eval/spec boundaries for a future read-only repo-search adapter
before any repo-search runtime, CLI behavior, MCP integration, provider
adapter, remote search, vector index, code graph, Dev module, or agent
execution is active.

This is a design/spec artifact only. It does not require implementing these
evals now.

## Status

The repo-search adapter boundary is `parked`.

The current surface is documentation only.

Any future implementation must be promoted through Research Gate, receipt
schema, conformance evals, and a bounded goal/contract.

## Boundary protected

A repo-search adapter may locate repository evidence.
It must not write files, project memory, contracts, gate decisions, proofpacks, or acceptance claims.
It must emit retrieval receipts when used inside a contract/run context.
Retrieval output is advisory evidence, not truth.

## Non-goals

This spec does not define:

- CLI behavior;
- runtime storage;
- repo scanning implementation;
- file walking implementation;
- MCP integration;
- provider adapters;
- agent execution;
- Dev module runtime;
- vector DB;
- embedding index;
- code graph writer;
- remote search;
- editing tools;
- refactoring tools;
- project memory writer;
- gate decision writer;
- broader runtime proofpack writer orchestration, `.punk/proofs` writer
  behavior, or acceptance proof behavior.

## Fixture model

Future fixtures should use small local repositories with known files, known
literals, optional known symbols, optional references, large/skipped files, and
secret/env-like examples.

Fixture expectations should be repo-relative and deterministic.

The minimum future request shape should include:

```yaml
query:
  mode: files | grep | multi_grep | symbols | definitions | references
  raw_query: string
  constraints:
    include_paths: []
    exclude_paths: []
    globs: []
    languages: []
    max_results: integer
    max_elapsed_ms: integer
    max_answer_chars: integer
trace:
  contract_ref: optional string
  goal_ref: optional string
```

The minimum future response shape should include result refs, result count,
warnings, backend capability metadata, and a retrieval receipt.

## Hard gates

- `no_write_side_effects`
- `no_decision_write`
- `no_project_memory_write`
- `no_network_by_default`
- `bounded_budget_enforced`
- `receipt_required`
- `unsupported_mode_reported`
- `truncation_reported`
- `stale_index_reported`

## Eval cases

### RSA-001: known-file retrieval

Given a fixture repo with a known file path, `mode = files` should return the
expected repo-relative file ref within budget.

### RSA-002: known-literal grep retrieval

Given a fixture repo with a known literal, `mode = grep` should return the
expected repo-relative path and line ref within budget.

### RSA-003: multi-grep bounded OR retrieval

Given multiple literals, `mode = multi_grep` should return bounded matches for
any requested literal without repeated unbounded searches.

### RSA-004: bounded max_results

Given more matches than `max_results`, output must be bounded and must report
that results were truncated or limited.

### RSA-005: bounded max_elapsed_ms

Given a low elapsed-time budget, the adapter must stop, degrade, or report
budget exhaustion rather than silently exceed the declared budget.

### RSA-006: bounded max_answer_chars

Given a low answer-size budget, the adapter must truncate safely and emit a
truncation warning.

### RSA-007: unsupported symbol mode reports unsupported capability

If a backend lacks symbol support, a symbol request must report unsupported
capability. It must not fake symbol precision with broad lexical fallback.

### RSA-008: optional symbol lookup with declared backend

If a backend declares symbol support, a known-symbol fixture should return a
definition or symbol ref with backend and freshness metadata.

### RSA-009: optional reference lookup with declared backend

If a backend declares reference support, a known-reference fixture should return
reference refs with backend and freshness metadata.

### RSA-010: repeated query reproducibility

Repeated identical queries against the same fixture and same snapshot should
produce stable result refs or explicitly explain nondeterministic ranking.

### RSA-011: truncated output has warning

Any output truncation must be visible in receipt warnings.

### RSA-012: stale index has warning

If the backend uses an index and detects stale state, the receipt must include a
stale-index warning and index basis metadata when available.

### RSA-013: skipped file has warning

Skipped files, large-file omissions, ignored paths, or unsupported file classes
must be recorded in warnings when they may affect completeness.

### RSA-014: secret/env-like content is not emitted raw in receipts

Receipts must not include raw secret values, raw environment values,
credentials, or private host paths. They may record safe diagnostic categories
or hashes.

### RSA-015: remote/network use is denied unless explicit future policy exists

Network access must be denied by default. Remote search remains parked until a
separate external side-effect policy exists.

### RSA-016: adapter cannot write project memory

The adapter must not create or update `work/`, `knowledge/`, `docs/adr/`,
`publishing/`, `.punk/memory/`, or project-memory claims.

### RSA-017: adapter cannot write decision/proof artifacts

The adapter must not create or update `.punk/decisions`, `.punk/proofs`, gate
decision files, proofpacks, acceptance claims, or final decision fields.

### RSA-018: receipt includes backend, query, constraints, budget, result_count, warnings

Every contract/run-context use must emit a retrieval receipt with backend,
version, query, mode, constraints, budget, result count, truncation, stale-index
warnings, skipped/omitted files, network assertion, side-effect assertion, and
goal/contract refs when present.

## Required receipt fields

Design direction only. This is not an active runtime schema.

- receipt id;
- created timestamp;
- goal ref, when used;
- contract ref, when used;
- backend name;
- backend version;
- backend kind;
- backend capability manifest ref;
- local-only assertion;
- network-used flag;
- side-effect assertion;
- query mode;
- raw query or redacted query ref;
- include/exclude path constraints;
- glob/language constraints;
- `max_results`;
- `max_elapsed_ms`;
- `max_answer_chars`;
- repo root ref;
- VCS ref, when available;
- working tree state;
- index state;
- index basis, when available;
- result count;
- truncated flag;
- top result refs;
- deterministic ranking flag;
- ranking factors, when available;
- personalized ranking flag;
- fallback mode flag;
- stale-index warnings;
- skipped-file warnings;
- large-file omission warnings;
- unsupported-mode warnings;
- truncation warnings;
- privacy or secret warnings.

## Failure codes

- `repo_search_write_side_effect`
- `repo_search_decision_write`
- `repo_search_project_memory_write`
- `repo_search_network_used_without_policy`
- `repo_search_budget_not_enforced`
- `repo_search_receipt_missing`
- `repo_search_unsupported_mode_hidden`
- `repo_search_truncation_not_reported`
- `repo_search_stale_index_not_reported`
- `repo_search_secret_raw_in_receipt`
- `repo_search_fallback_overclaim`
- `repo_search_non_reproducible_without_notice`

## Promotion requirement

Before promotion from parked docs to any runtime adapter behavior, Punk must
have:

- accepted or superseding R2 research;
- explicit retrieval receipt schema;
- fixture-backed evals for required modes;
- no-write/no-decision/no-project-memory checks;
- no-network default checks;
- capability manifest checks;
- budget and truncation checks;
- stale-index and skipped-file warning checks;
- evidence that retrieval entering a Contract Context Pack remains advisory and
  clause-indexed.

## Out of scope

This spec does not activate repo-search, MCP, remote search, vector retrieval,
code graph runtime, Dev module runtime, agent execution, file editing,
refactoring, runtime `.punk` storage, gate decisions, proofpacks, or acceptance
claims.
