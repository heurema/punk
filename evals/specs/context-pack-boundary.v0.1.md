# Context Pack boundary v0.1

Date: 2026-04-29
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary that Contract Context Packs must satisfy before any runtime context-pack storage, retrieval adapter, executor brief generator, compression system, vector index, or model-assisted context selector becomes active.

This is a design/spec artifact only. It does not activate storage, CLI behavior, repo-search, embeddings, agents, or executor brief generation.

## Status and authority

A Contract Context Pack is a linked evidence artifact for one contract.

It is not a final decision, project truth by itself, prompt manager, retrieval engine, executor-local memory, or fourth lifecycle phase.

## Required deterministic eval cases

### CTX-PACK-001: no orphan context

Every included context item must support at least one contract clause, scope boundary, risk, non-goal, validator, or proof requirement. A context pack containing an included item with an empty `supports` list must be rejected.

### CTX-PACK-002: every material clause has support or unknown

Every material contract clause must have at least one of: source ref, user clarification, explicit assumption, or explicit unknown. A required clause with no coverage must block approval for bounded work or be recorded as an explicit unknown.

When coverage cites included context items, every `covered_by` ref must resolve
to an included item id, and the referenced item must claim support for that
clause. Likewise, an included item's `supports` claim must be represented in
`clause_coverage`. One-way support/coverage mappings are invalid.

### CTX-PACK-003: speculative sources excluded by default

Context items with authority `speculative` must not appear in `items` by default. They may appear in `exclusions` with an explanation, or be cited only as advisory input by an explicitly scoped contract.

### CTX-PACK-004: stale and superseded sources are flagged

A context item with status `stale`, `superseded`, `archived`, or `retired` must not be treated as current truth. The pack must either exclude it or mark the status explicitly.

### CTX-PACK-005: contradictions surfaced, not flattened

If two relevant sources conflict, the context pack must record a contradiction set. The selector must not silently summarize the conflict into one hidden prompt claim.

### CTX-PACK-006: budget overrun does not silently truncate

If the context pack exceeds its configured item/token budget, Punk must explicitly narrow the contract, compress after selection, mark the pack incomplete, or require clarification/research. Silent truncation is not allowed.

### CTX-PACK-007: retrieval receipt is advisory

A retrieval receipt may record backend, query, constraints, result summary, timing, and index/cache provenance. It must not own project truth, write final decisions, bypass `gate`, or promote retrieved content into canonical knowledge.

### CTX-PACK-008: executor brief cannot override contract

A future executor brief derived from a context pack must not add requirements, change acceptance criteria, override validators, or supersede the contract.

### CTX-PACK-009: gate cites proof/evidence, not executor claims

A gate decision may cite a context pack as evidence selection, but acceptance still requires verified evidence and proofpack links. Executor claims such as “done” or “tests passed” are not proof.

### CTX-PACK-010: rebuildability

A persisted context pack must be rebuildable or inspectable from refs, metadata, exclusions, unknowns, contradiction sets, and retrieval receipts. Hidden prompt text or executor-local memory must not be required to understand why context was selected.

## Minimal fixture shape

```yaml
context_pack:
  id: ctxpack_fixture_001
  contract_ref: contract_fixture_001
  status: draft
  items:
    - item_id: item_laws
      ref: docs/product/PUNK-LAWS.md
      authority: canonical
      doc_status: active
      supports:
        - rules.only_gate_decides
  clause_coverage:
    - clause_id: rules.only_gate_decides
      covered_by: [item_laws]
      coverage_type: source
    - clause_id: scope.output
      covered_by: []
      coverage_type: unknown
  exclusions:
    - ref: knowledge/ideas/example.md
      reason: speculative
      class: speculative
  contradictions: []
  unknowns:
    - statement: exact output format not yet selected
      blocking: true
      next_action: clarify
  retrieval_receipts: []
```

## Non-goals

This v0.1 boundary does not define runtime context-pack storage, `.punk/` context-pack writer, CLI commands, retrieval adapter implementation, vector index behavior, context compression, executor brief generation, model-assisted summarization, gate implementation, or proofpack writer behavior.

## Future implementation constraints

Any future implementation must keep context pack as evidence, contract as normative scope, gate as final decision writer, proof required before acceptance, retrieval advisory and receipt-bearing, and executor briefs derived and non-authoritative.
