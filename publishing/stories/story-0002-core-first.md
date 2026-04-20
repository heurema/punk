---
id: story_0002_core_first
title: "Why Punk starts core-first"
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
story_type: principle
related_goals:
  - goal_bootstrap_punk_core
related_knowledge:
  - docs/product/START-HERE.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
related_posts: []
---

# Why Punk starts core-first

## Core idea

The first job is not to turn on every promising capability.

The first job is to define the trust surfaces and keep the active path smaller than the workspace.

## Why it matters

A system that grows faster than its boundaries becomes hard to trust.

That is why `punk` starts with flow, events, evals, contracts, gate, proof, and project memory before modules, adapters, and automation.

## Narrative beats

1. Earlier lines accumulated many useful ideas.
2. Convergence created pressure to build one runtime.
3. The reset cut scope back to the stable core.
4. Core-first became an explicit product rule.

## Current status

The active target is the stable core.

Modules such as PubPunk are parked for now.

## What stays out of scope

- provider zoo
- publishing automation
- plugin marketplace
- UI-first workflow

## Possible posts

- Why boundaries come before behavior
- Why modules are later
- Why the active surface must stay smaller than the repo
