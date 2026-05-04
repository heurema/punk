# Context-Pack Compiler Boundary v0.1

Date: 2026-05-04
Status: proposed boundary extension
Authority: advisory/design

## Purpose

Specify a future side-effect-free boundary for compiling bounded, source-anchored context packs from contracts.

This is a narrow extension to `evals/specs/context-pack-boundary.v0.1.md`, which remains the current canonical context-pack boundary spec. This file adds compiler-shaped cases without duplicating or replacing that existing boundary.

## Status and authority

This is an eval/spec artifact only.

It is not a prompt manager, retrieval engine, vector DB, executor brief authority surface, decision object, proofpack, or project-truth owner.

## Relationship to existing context-pack boundary

`evals/specs/context-pack-boundary.v0.1.md` owns the current Contract Context Pack boundary.

This extension focuses only on a future compiler boundary:

```text
contract clauses + source refs + budget + exclusions
  -> side-effect-free context-pack candidate
```

The compiler candidate remains advisory and cannot define acceptance criteria, write decisions, write proof, execute retrieval, or persist runtime state.

## Definitions

### context budget

An explicit item, byte, token, or section budget used to keep selected context bounded. Budget overrun must be surfaced rather than silently truncating material clauses.

### selected source refs

Repo-relative or otherwise explicit source refs included in the context pack because they support a contract clause, scope boundary, risk, non-goal, validator, or proof requirement.

### excluded source refs

Refs considered but excluded, with explicit reason such as speculative authority, stale status, superseded status, contradiction, budget, privacy, or irrelevance.

### stale/superseded flags

Metadata that marks a source as stale, superseded, archived, retired, or not current truth.

### contradiction sets

Grouped refs or claims that disagree and must be surfaced instead of flattened into a single summary.

### unknowns

Material missing facts that affect contract scope, validation, risk, or gate review.

### assumptions

Explicit bounded assumptions that a contract may carry until reviewed, verified, or replaced.

### bidirectional clause/support mapping

Every included context item must map to at least one supported clause or boundary, and every material clause support entry must resolve to included context or an explicit assumption/unknown.

### no orphan context items

No context item may be included without supporting a contract clause, scope boundary, risk, non-goal, validator, or proof requirement.

### no unsupported material clauses

No material contract clause may pass compilation without a selected source ref, explicit assumption, explicit unknown, or human review route.

### speculative source exclusion by default

Speculative sources are excluded from implementation truth by default. They may appear in exclusions or explicitly advisory research context only.

### advisory-only authority

The context-pack compiler candidate is evidence selection, not acceptance, approval, decision, proof, or project truth.

## Positive cases

### CPC-001: selected source supports a hard clause

A canonical source ref supports a hard clause and appears in both `selected_source_refs` and the clause/support map.

Expected result: valid selected context.

### CPC-002: material clause has explicit unknown

A material clause has no source ref, but the pack records an explicit unknown and marks whether it blocks approval.

Expected result: valid incomplete context candidate, not silent coverage.

### CPC-003: stale source is surfaced

A relevant stale or superseded source is included only with a stale/superseded flag and current-source caveat, or is excluded with reason.

Expected result: stale truth is not flattened into current truth.

### CPC-004: speculative idea is excluded by default

A speculative idea is considered, then recorded in exclusions with reason `speculative`.

Expected result: excluded from implementation truth.

## Negative cases

### CPC-101: context item with no clause support

An included item with no clause, scope, risk, non-goal, validator, or proof support is invalid.

Expected result: reject or exclude the item.

### CPC-102: material clause with no source/assumption/unknown

A material clause that has no source ref, assumption, unknown, or human review route cannot be silently accepted.

Expected result: block context readiness.

### CPC-103: context pack defines acceptance criteria

A context pack that adds or changes acceptance criteria exceeds its authority.

Expected result: reject the pack.

### CPC-104: context pack writes decision

A context pack that writes or implies a final decision violates gate authority.

Expected result: reject the pack.

### CPC-105: stale source flattened into current truth

A stale, superseded, archived, or retired source summarized as current truth is invalid.

Expected result: reject or require explicit stale handling.

### CPC-106: speculative idea used as implementation truth

A speculative source used as if it were implementation truth is invalid.

Expected result: reject or reclassify as advisory input with separate coverage.

## Minimal fixture shape

```yaml
context_pack_compiler:
  contract_ref: contracts/example-contract.md
  context_budget:
    max_items: 10
  selected_source_refs:
    - ref: docs/product/PUNK-LAWS.md
      supports:
        - rules.only_gate_writes_decisions
  excluded_source_refs:
    - ref: knowledge/ideas/example.md
      reason: speculative
  stale_flags: []
  contradiction_sets: []
  unknowns:
    - id: unknown.validator_shape
      statement: "Exact validator fixture shape is not selected."
      blocks_approval: true
  assumptions: []
  clause_support:
    - clause_id: rules.only_gate_writes_decisions
      support_refs:
        - docs/product/PUNK-LAWS.md
```

## Non-goals

This is not a prompt manager, retrieval engine, vector DB, executor brief authority surface, decision object, proofpack, or project-truth owner.

This spec does not add runtime storage, `.punk/` writers, retrieval adapters, model-assisted summarization, context compression, CLI behavior, gate writing, proof writing, or contract storage.

## Future implementation constraints

Any future compiler must be side-effect-free until explicitly promoted, preserve repo/source refs, surface stale and contradiction state, exclude speculative sources by default, and keep acceptance criteria in the contract and final decisions in `gate`.
