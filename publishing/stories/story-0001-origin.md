---
id: story_0001_origin
title: "Why Punk exists"
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-18
updated_at: 2026-04-20
story_type: origin
related_goals:
  - goal_bootstrap_punk_core
related_knowledge:
  - docs/product/START-HERE.md
  - docs/product/ROADMAP.md
  - docs/product/DOGFOODING.md
  - docs/product/JOURNAL.md
related_posts:
  - publishing/posts/0001-first-public-build-note.md
---

# Why Punk exists

## Core idea

`punk` starts from one question:

Can you stop reading code as the main trust mechanism and still trust the result?

That question turned a loose line of AI coding experiments into a search for bounded work, explicit scope, proof, and project memory.

## Why it matters

If the code read is no longer the only review surface, trust has to move somewhere else.

That pushed the work toward:

- planning before execution
- bounded contracts against drift
- gates instead of taste
- proof that can travel with the change
- project memory that stays inspectable

## Narrative beats

1. The trust problem became explicit.
2. Plans hardened into specs, then contracts.
3. Reliability moved into gates, proof, and bounded flow.
4. Separate ideas converged into one runtime surface: `punk`.

## Current status

This is an early-stage, core-first build.

The current active surface is the stable core, not publishing automation, provider adapters, or product modules.

## What stays out of scope

- no claim that `punk` is production-ready
- no claim that PubPunk automation already exists inside `punk`
- no claim that every old experiment survived unchanged

## Possible posts

- First public build note
- The question that started Punk
- Why trust moved outside the code read

## Claims to verify

- [ ] The story does not overclaim implementation readiness.
- [ ] The story distinguishes current manual structure from future PubPunk automation.
- [ ] The story does not leak private/internal-only material.
