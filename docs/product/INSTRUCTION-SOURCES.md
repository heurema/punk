---
id: docs_product_instruction_sources
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-18
updated_at: 2026-05-18
review_after: 2026-07-18
canonical_for:
  - instruction-source-boundary
  - generated-instruction-view-boundary
  - instruction-freshness-boundary
  - host-specific-runner-aid-transform-boundary
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/CRATE-STATUS.md
related_research:
  - knowledge/research/2026-05-15-page-index-instruction-navigation.md
  - knowledge/research/2026-05-18-gstack-workflow-prior-art.md
related_evals:
  - evals/specs/instruction-source-freshness.v0.1.md
supersedes: []
superseded_by: null
---

# Instruction Sources

## Purpose

Define how Punk treats local instruction source pages and generated instruction
views for humans, executors, modules, and future host-specific runner aids.

Instructions may orient execution.

Instructions do not own project truth.

## Current active boundary

Current project init behavior may create thin source instruction pages under:

```text
.punk/instructions/
```

Those pages are repo-tracked local orientation aids.

Generated instruction indexes or views under `.punk/views/` remain future,
rebuildable, advisory artifacts unless a later bounded goal activates a writer.

## Source vs view

| Surface | Role | Authority |
|---|---|---|
| `.punk/instructions/` source pages | Human/executor orientation | Advisory, repo-tracked source aid. |
| `.punk/views/instructions/page-index.json` | Future generated index | Rebuildable, non-authoritative view. |
| Host-specific generated skill docs | Future transformed runner aids | Non-authoritative, must cite source refs. |
| Product docs / ADRs / contracts | Canonical project truth where applicable | Authority-tagged according to docs/project memory rules. |

A generated view must never become the only place where a rule, invariant,
contract clause, or project decision exists.

## Allowed

Instruction source pages may:

- explain how to inspect local Punk memory;
- link canonical docs and project-memory artifacts;
- summarize current active/non-active boundaries;
- describe how to collect receipts;
- describe module-specific local orientation after module promotion;
- state that generated views are rebuildable and non-authoritative;
- warn about unsafe or parked behavior.

Generated instruction views may:

- list source pages;
- provide summaries;
- expose search or navigation metadata;
- render host-specific runner-aid variants;
- point back to source refs and hashes.

## Not allowed

Instruction source pages or generated views must not:

- write final decisions;
- claim proof;
- bypass `gate`;
- replace contracts, validators, receipts, evals, decisions, or proofpacks;
- silently activate module host behavior;
- silently activate provider adapters;
- silently activate browser, network, publishing, deploy, or credential behavior;
- store hidden executor memory as truth;
- make a generated summary canonical;
- duplicate large canonical docs without source refs and review.

## Freshness

Instruction freshness checks should flag:

- generated views missing source refs;
- generated views that include no source hash/version metadata when hashes are
  expected;
- generated output that references removed source pages;
- source pages that claim future capabilities are active;
- host-specific output that leaks another host's paths or tool names;
- instruction pages that define acceptance criteria not present in contracts;
- stale generated views after source changes.

Freshness findings are advisory drift signals. They are not gate decisions.

## Host-specific transforms

Future host-specific runner-aid transforms may render Punk instructions into
formats for Codex, Claude Code, Cursor, OpenCode, or other executors.

Such transforms must:

- be derived from repo-tracked source refs;
- declare host-specific rewrites;
- avoid provider-specific authority claims;
- preserve non-authority statements;
- keep generated output out of canonical truth;
- be covered by freshness/leakage evals;
- avoid hidden network behavior.

## Relationship to gstack

gstack's generated skill-doc and host-config pattern is useful prior art for
freshness and host-specific transforms.

Punk should adopt the freshness discipline, not the provider-specific authority
model.

This document does not add a PageIndex dependency, gstack dependency, Python
runtime, LLM summarization, OCR, MCP, vector DB, module host activation,
provider adapter, browser runtime, gate writer, proofpack writer, or acceptance
claim.

## Non-goals

This document does not define:

- generated instruction writer behavior;
- a public instruction CLI;
- module instruction runtime;
- skill installation;
- prompt compilation;
- browser/agent integration;
- external side effects;
- project-truth promotion.
