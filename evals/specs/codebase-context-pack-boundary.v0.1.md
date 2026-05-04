# Codebase Context Pack Boundary Eval v0.1

Date: 2026-05-03
Status: proposed boundary
Authority: advisory/design

## Purpose

Define advisory eval/spec boundaries for selecting codebase evidence into a
future Contract Context Pack.

This spec protects the clause-indexed evidence-selection boundary before any
runtime context-pack writer, repo-search integration, Context Compiler runtime,
MCP integration, Dev module runtime, vector index, code graph runtime, or agent
execution becomes active.

## Status

This is a design/spec artifact only.

It does not activate storage, CLI behavior, retrieval, model-assisted
selection, executor brief generation, gate behavior, broader runtime proofpack
writer orchestration, `.punk/proofs` writer behavior, acceptance proof
behavior, or project truth promotion.

## Boundary protected

A Contract Context Pack may select codebase evidence for a contract.
It is clause-indexed advisory evidence.
It is not a prompt manager, retrieval engine, decision object, proofpack, project-truth owner, or executor memory store.

## Non-goals

This spec does not define:

- Contract Context Pack runtime storage;
- context-pack writer behavior;
- repository scanning;
- file walking;
- repo-search adapter implementation;
- MCP integration;
- Context Compiler runtime;
- Knowledge Vault runtime;
- vector DB;
- code graph writer;
- executor brief generator;
- agent execution;
- Dev module runtime;
- gate decision writer;
- broader runtime proofpack writer orchestration, `.punk/proofs` writer
  behavior, or acceptance proof behavior.

## Fixture model

Future fixtures should model one contract with material clauses, validators,
proof requirements, non-goals, risks, source refs, optional retrieval receipt
refs, stale/superseded/advisory/speculative source metadata, contradiction
sets, unknowns, and selected codebase evidence items.

The minimum future pack shape should preserve:

```yaml
context_pack:
  id: string
  contract_ref: string
  status: draft
  authority: advisory
  items:
    - item_id: string
      ref: string
      source_kind: code | test | docs | config | schema | migration | script | receipt
      authority: canonical | advisory | speculative
      source_status: active | draft | stale | superseded | archived | retired
      retrieval_receipt_refs: []
      supports: []
  clause_coverage:
    - clause_id: string
      covered_by: []
      coverage_type: source | validator | proof_requirement | non_goal | risk | unknown
  exclusions: []
  contradictions: []
  unknowns: []
```

This is fixture direction only, not an active runtime schema.

## Hard gates

- `no_orphan_context`
- `material_clause_coverage_required`
- `bidirectional_clause_support_mapping`
- `speculative_source_excluded_by_default`
- `stale_source_flagged`
- `superseded_source_flagged`
- `contradiction_surfaced`
- `unknowns_recorded`
- `retrieval_receipt_refs_preserved`
- `no_decision_authority`

## Eval cases

### CBCP-001: context item supports a contract clause

An included codebase evidence item must support at least one contract clause
when cited as clause evidence.

### CBCP-002: context item supports a validator

An included item may support a validator only when the validator ref or
validator rationale maps back to the item.

### CBCP-003: context item supports a proof requirement

An included item may support a proof requirement only when the proof
requirement ref maps back to the item.

### CBCP-004: context item supports a scope boundary or non-goal

An included item may support a scope boundary or non-goal when it explains what
must stay out of the contract or later work.

### CBCP-005: orphan context is rejected

An included item with no support mapping to a clause, validator, proof
requirement, risk, non-goal, scope boundary, or explicit unknown must be
rejected or moved to exclusions.

### CBCP-006: material clause with no source is flagged

Every material clause must have a source ref, validator ref, proof requirement,
user clarification, assumption, or explicit unknown. Missing support must be
flagged.

### CBCP-007: stale source is surfaced, not flattened

If a selected source is stale, the pack must preserve the stale status and must
not rewrite it into current context.

### CBCP-008: superseded source is surfaced, not flattened

If a selected source is superseded, the pack must preserve the superseded status
and link the replacement when known.

### CBCP-009: contradictory sources are grouped

Conflicting sources must be grouped in a contradiction set. The pack must not
merge them into a single unqualified summary.

### CBCP-010: speculative source excluded by default

Speculative sources must be excluded by default. If a later explicitly scoped
contract includes one as advisory input, the authority must remain visible.

### CBCP-011: advisory source can be included only with advisory authority preserved

Advisory sources may support research, risks, unknowns, or non-goals, but the
pack must not promote them to canonical authority.

### CBCP-012: retrieval receipt ref is preserved when retrieval was used

If repo-search or any retrieval backend helped locate an included item, the
pack must preserve retrieval receipt refs.

### CBCP-013: pack cannot define acceptance criteria

The pack must not add, remove, or change acceptance criteria. The contract owns
criteria.

### CBCP-014: pack cannot override validators

The pack must not override validators, waive validators, or convert advisory
context into validator success.

### CBCP-015: pack cannot write gate decision or proofpack

The pack must not create or update gate decisions, proofpacks, proof refs,
acceptance claims, or final decision fields.

## Failure codes

- `codebase_context_orphan_item`
- `codebase_context_material_clause_uncovered`
- `codebase_context_one_way_mapping`
- `codebase_context_speculative_source_included`
- `codebase_context_stale_source_flattened`
- `codebase_context_superseded_source_flattened`
- `codebase_context_contradiction_hidden`
- `codebase_context_unknown_missing`
- `codebase_context_receipt_ref_dropped`
- `codebase_context_advisory_promoted`
- `codebase_context_acceptance_criteria_defined`
- `codebase_context_validator_overridden`
- `codebase_context_decision_authority_claimed`
- `codebase_context_proofpack_written`

## Promotion requirement

Before any runtime codebase context-pack selection is promoted, Punk must have:

- accepted or superseding R2 research;
- fixture-backed no-orphan-context checks;
- material clause coverage checks;
- bidirectional support mapping checks;
- stale/superseded source checks;
- contradiction and unknown surfacing checks;
- speculative/advisory source authority checks;
- retrieval receipt preservation checks;
- no-decision/no-proofpack/no-acceptance checks.

## Out of scope

This spec does not activate Contract Context Pack runtime storage, repo-search
runtime, MCP, Context Compiler runtime, Knowledge Vault runtime, vector or graph
indexes, Dev module runtime, agent execution, gate decisions, proofpacks, or
acceptance claims.
