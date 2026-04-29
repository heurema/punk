---
id: docs_product_north_star
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-29
updated_at: 2026-04-29
review_after: 2026-06-15
canonical_for:
  - project-north-star
  - frontier-direction
  - spec-and-intent-native-direction
  - code-first-to-spec-as-source-trajectory
related_docs:
  - docs/product/START-HERE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
supersedes: []
superseded_by: null
---

# North Star

## Purpose

Define Punk's high-level direction across product, architecture, research, roadmap, public narrative, and review decisions.

This document is directional. It does not activate new runtime behavior.

## Core thesis

Punk does not exist only to make the old code-first software process faster.

Punk starts with today's working practices because they are real, useful, and necessary. But its long-term direction is to discover, test, and gradually promote better primary artifacts for software work.

The desired trajectory is:

```text
code-first discipline
  -> spec-anchored work
  -> spec-and-intent-native workflows
  -> future spec-as-source experiments
```

## Directional commitments

Punk should make intent, contracts, evidence, decisions, and proof more inspectable than executor claims.

Code remains essential, but it should increasingly be linked to explicit intent, bounded scope, validation evidence, gate decisions, and project memory.

Future spec-as-source work is experimental direction, not current behavior. It must be earned through research, contracts, evals, roadmap gates, and proof-bearing implementation.

## Current boundary

This north star does not change the active CLI surface, roadmap phase order, crate status, runtime storage, or active scope.

Current executable CLI remains limited to:

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

Active `.punk/` runtime storage for work, contracts, runs, evals, decisions, and proofs remains inactive.

Active proofpack writing, provider/model/agent execution, module/plugin runtime, and adapters remain inactive or parked according to the canonical docs.

## How to use this document

Use this document as a directional lens when choosing between otherwise valid product, architecture, research, roadmap, public narrative, or review options.

Do not use it to bypass `docs/product/PUNK-LAWS.md`, promote future scope, expose new CLI behavior, claim production readiness, or treat speculative capabilities as active.
