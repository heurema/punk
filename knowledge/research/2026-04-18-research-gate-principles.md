---
id: research_2026_04_18_research_gate_principles
title: "Research gate principles for punk"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-18
updated_at: 2026-04-18
review_after: 2026-07-18
components: [governance, research, roadmap]
related_goals: []
related_files:
  - docs/product/RESEARCH-GATE.md
source_refs:
  - docs/product/ROADMAP.md
confidence: medium
research_level: R1
---

# Research gate principles for punk

## Question

How should `punk` require research before important architecture decisions without slowing every small implementation step?

## Decision context

The project has many good ideas and high risk of feature expansion before core stability.

## Sources reviewed

This initial note is a placeholder. Future research notes must cite concrete external sources.

## Recommendation

Use research depth levels:

- R0 for no research required
- R1 for quick scans
- R2 for design research
- R3 for deep research

Require research refs in important architecture contracts.

## What stays out of scope

- research for trivial edits
- collecting unsorted links
- accepting weak commentary as decision evidence

## Required evals

- eval_research_required_for_major_decision
- eval_contract_has_research_refs_when_required
