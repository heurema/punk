# ADR-0016: Contract Context Pack boundary

Status: Proposed
Date: 2026-04-29

Research refs:

- `knowledge/research/2026-04-28-context-pack-boundary.md`

## Context

Punk needs `plot` to prepare enough context for a bounded contract without letting model/executor context become hidden project memory. Too little context lets an executor infer hidden assumptions. Too much context introduces distractors, stale knowledge, flattened contradictions, and hidden prompt state.

The boundary must preserve current Punk laws: one lifecycle grammar, every executable goal resolves to a contract, only `gate` writes final decisions, proof comes before acceptance, modules assess but do not decide, adapters invoke but do not own truth, project memory is explicit and authority-tagged, and retrieval is advisory evidence only.

## Decision

Adopt **Contract Context Pack** as the name for a contract-linked, authority-aware evidence-selection artifact.

Adopt **Context Pack Boundary** as the policy that governs that artifact.

A Contract Context Pack is evidence for a contract. It is not a final decision, project truth by itself, prompt manager, retrieval engine, executor memory store, or fourth lifecycle phase.

The active-core implementation may add side-effect-free model and validation helpers to `punk-contract` so contracts can reason about context pack shape without activating storage, CLI behavior, retrieval adapters, or executor brief generation.

## Required invariants

1. No orphan context: every included context item supports a contract clause, scope boundary, risk, non-goal, validator, or proof requirement.
2. No unsupported material clause: every material contract clause is covered by a source ref, user clarification, assumption, or explicit unknown.
3. Bidirectional support/coverage mapping: every `covered_by` item ref resolves to an included context item that supports that clause, and every included item support claim appears in clause coverage.
4. Authority before similarity: canonical active sources outrank advisory sources; speculative sources are excluded by default; stale/superseded sources are flagged.
5. Contradictions are surfaced rather than flattened.
6. Retrieval remains advisory and receipt-bearing.
7. Executor briefs are derived and cannot override contract or acceptance criteria.

## Active, deferred, parked, avoided

Active now:

- boundary terminology;
- side-effect-free model/validator in `punk-contract`;
- docs and eval/spec coverage;
- explicit separation between contract, context pack, and executor brief.

Deferred:

- persisted context pack writer;
- `.punk/contracts` or `.punk/views` storage;
- context scoring heuristics;
- context compression;
- executor brief generation.

Parked:

- repo-search adapter integration;
- reranking;
- vector indexes;
- contextual retrieval preprocessing;
- multi-agent context isolation;
- persistent memory tooling.

Avoid:

- giant prompts;
- fixed top-k retrieval as the sole policy;
- embeddings or retrieval indexes as project truth;
- executor-local memory as hidden implementation truth;
- retrieval writing gate decisions.

## Consequences

Positive:

- gives `plot` a bounded context discipline without activating agents;
- prevents hidden prompt/memory accumulation;
- connects every context item to contract semantics;
- supports future executor handoffs without making briefs authoritative;
- creates deterministic eval hooks before retrieval infrastructure exists.

Costs / risks:

- adds curation overhead to contract shaping;
- can become bureaucratic if every tiny contract requires a heavy pack;
- false confidence is possible if context metrics are treated as authority;
- compression can hide important details if adopted too early.

Mitigations:

- keep v0.1 side-effect-free and optional at the schema/model level;
- start with deterministic boundary checks;
- require explicit unknowns instead of pretending coverage is complete;
- keep runtime storage and retrieval adapters parked.

## Required eval direction

Before runtime promotion, define eval/spec cases for orphan-free context inclusion, material clause coverage, speculative-source exclusion, stale/superseded flags, contradiction surfacing, executor brief non-authority, retrieval receipt non-authority, gate proof/evidence citation, and rebuildability from refs/receipts/metadata.

## Gate note

This ADR is proposed, not final acceptance. Only future `gate` writes final acceptance. The current implementation should remain side-effect-free and must not activate storage, CLI behavior, retrieval adapters, or executor execution.
