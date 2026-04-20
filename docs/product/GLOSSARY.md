---
id: docs_product_glossary
kind: glossary
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - shared-terms
related_docs:
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/DOC-GOVERNANCE.md
supersedes: []
superseded_by: null
---

# Glossary

## Purpose

This glossary defines shared Punk terms so product docs, ADRs, research notes, evals, and work reports do not invent overlapping definitions.

If another document needs one of these definitions, it should link here or quote only a short summary.

## Status terms

### active-core

Part of the current stability target. It must remain green under required evals and must not be bypassed by modules, adapters, or future product surfaces.

### incubating

A capability or boundary that may exist and be tested, but is not default or user-facing.

### parked

A capability or boundary that may have minimal docs/stubs, but is out of current scope and must not appear as the current operator path.

### retired

Removed, superseded, or legacy-only.

### raw

Captured but not yet shaped into stable knowledge.

### draft

Proposed but not yet accepted as current truth.

### accepted

Explicitly adopted artifact or decision.

## Lifecycle terms

### goal

A durable statement of intended work. Goals usually live under `work/goals/` and can be linked to contracts, reports, evals, decisions, and proofs.

### flow

A validated lifecycle instance using Punk's shared grammar:

```text
plot -> cut -> gate
```

### plot

The phase that shapes work and creates or prepares a contract. Research can inform `plot`, but research is not a fourth runtime phase.

### cut

The phase that executes bounded, approved work under a contract. `cut` cannot run unapproved work.

### gate

The phase that verifies evidence, writes the final decision, and links proof. Only `gate` writes final decisions.

### contract

A scoped executable agreement for work. Every executable goal resolves to a contract before work runs.

A meaningful contract will eventually carry `DocImpact` when docs truth may change.

### scope

The explicit boundary of what a contract allows. Scope limits what `cut` may do and what `gate` may evaluate.

### transition

A state change in a flow. Every valid transition is checked by runtime state and writes an event.

## Evidence terms

### event

An append-only local audit record for lifecycle, guard, eval, decision, proof, or inspect activity. Events may reference artifacts by id, relative path, and hash.

### run

A bounded execution of work under an approved contract.

### receipt

A local evidence artifact describing what happened in a run, eval, guard, module, adapter, or other bounded operation. A receipt is evidence, not final acceptance.

### eval

A local assessment that checks behavior, fixtures, reports, schemas, regressions, or other conditions. Eval results can inform `gate`, but do not accept work.

### assessment

Advisory evidence produced by a rule, module, policy, adapter, guard, or eval. Assessments may be referenced by `gate`, but cannot write final decisions.

### decision object

The final decision artifact written by `gate`. A decision can accept, reject, request rework, defer, or otherwise resolve the contract according to allowed gate semantics.

### proofpack

A local provenance manifest that links contract, run receipt, eval report, decision, output artifacts, hashes, and event range/root where applicable.

### artifact ref

A reference to a repo-tracked or runtime artifact, usually using a relative path, id, and hash.

### hash

A digest used to make artifacts verifiable. Hash stability and path normalization must be defined by the relevant schema or ADR.

## Documentation and memory terms

### project memory

Repo-tracked continuity across goals, contracts, reports, evals, decisions, proofs, knowledge, ADRs, and public narrative. Project memory is explicit and authority-tagged.

### knowledge vault

The repo-tracked knowledge surface under `knowledge/`. It is not hidden runtime memory and not a vector database source of truth.

### retrieval memory

Runtime or derived retrieval state such as indexes, frecency databases, caches, query history, symbol indexes, or vector indexes. Retrieval memory can help find evidence but is not project truth.

### memory link graph

Explicit links across artifacts, typically:

```text
goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative
```

### authority

A tag describing how a knowledge artifact should be treated. Recommended authorities include `canonical`, `advisory`, and `speculative`.

### canonical

The current owner of truth for a surface. Canonical docs may be summarized elsewhere, but they should not be contradicted.

### advisory

Useful evidence or guidance that can support decisions but is not current implementation truth by itself.

### speculative

Idea-level content that must stay out of default implementation truth until promoted.

### status

A tag describing the lifecycle of a knowledge or work artifact, such as draft, accepted, superseded, parked, or retired.

### DocImpact

The explicit declaration of whether and how a change affects docs truth, ownership, supersession, archives, or public claims.

### superseded

Still inspectable history, but no longer the current truth because another artifact replaced it.

### archived

Preserved historical artifact that should not be treated as current truth.

## Research terms

### research note

A curated repo-tracked analysis under `knowledge/research/`. Research is advisory until promoted.

### research gate

The governance process required before important product or architecture changes. It defines when research is required, what depth is needed, and how research is promoted.

### research intake

The process that classifies external ideas as `adopt`, `defer`, `park`, or `avoid` before they affect architecture, roadmap, evals, or implementation.

### idea

A speculative or deferred item stored under `knowledge/ideas/`. Ideas are not implementation truth by default.

### promotion

The path by which research or ideas become roadmap, ADR, contract, docs, or implementation truth. Promotion does not erase the source artifact.

### anti-pattern

A mechanism or pattern Punk should avoid because it conflicts with laws, trust posture, local-first behavior, or active-core scope.

## Boundary terms

### module

A lifecycle participant that can assess and emit receipts or recommendations. A module cannot write final decisions.

### plugin

A possible future packaging or execution mechanism for modules. A plugin runtime is not active while the module host is parked.

### adapter

A wrapper around provider/tool capabilities. An adapter can invoke external or local tools, but cannot own project truth or write final decisions.

### repo-search adapter

A parked adapter boundary for local or remote code/repo retrieval. It remains advisory and read-only unless explicitly promoted through Research Gate, ADR, receipt schema, and conformance evals.

### side effect

An operation that changes files, processes, network state, environment, external services, publishing channels, secrets, or other non-read-only surfaces.

### no-network default

The rule that active-core Punk behavior should not perform hidden network export or remote telemetry by default.

### derived view

A convenience view generated from canonical artifacts. A derived view must be regenerable and must not become a second source of truth.

## Public narrative terms

### public narrative

Repo-tracked public-build memory under `public/`: stories, drafts, channel notes, publication receipts, and metrics snapshots.

### publication receipt

A record that a public artifact was published manually or automatically. Manual publication is allowed before PubPunk automation, but should still leave a receipt.

### metrics snapshot

A manual or future automated record of public content performance. Metrics must identify source, channel, and collection time.
