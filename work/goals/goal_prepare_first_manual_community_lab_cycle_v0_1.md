---
id: goal_prepare_first_manual_community_lab_cycle_v0_1
title: "Prepare first manual Community Lab cycle v0.1"
status: done
owner: "vitaly"
authority: canonical
module: "project-memory"
priority: P2
created_at: 2026-05-09
updated_at: 2026-05-09
selected_at: 2026-05-09
started_at: 2026-05-09
completed_at: 2026-05-09
blocked_by: []
scope:
  include:
    - "knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md"
    - "work/goals/goal_prepare_first_manual_community_lab_cycle_v0_1.md"
    - "work/reports/2026-05-09-first-manual-community-lab-cycle-v0-1.md"
    - "work/STATUS.md"
    - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
  exclude:
    - "code"
    - "CLI"
    - "runtime .punk/**"
    - "docs/product/**"
    - "docs/adr/**"
    - "evals/**"
    - "bot/adapters"
    - "Telegram/Discord/GitHub integrations"
    - "live send"
    - "raw transcript storage"
    - "automatic issue/goal creation"
    - "DAO/token/funding"
    - "CommunityPunk runtime"
    - "Topic Graph runtime"
acceptance:
  - "Creates a surface-agnostic launch pack for first manual Community Lab cycle."
  - "Includes ready-to-copy pinned welcome/policy text."
  - "Includes suggested topics/categories."
  - "Includes evidence-weighted signal model."
  - "Includes manual digest template."
  - "Includes promotion rules."
  - "Keeps chat/surfaces advisory only."
  - "Adds no product/runtime/CLI/bot/adapter behavior."
  - "Leaves selected_next unchanged."
knowledge_refs:
  - "knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md"
  - "knowledge/research/2026-05-09-community-driven-development-governance.md"
  - "knowledge/ideas/2026-05-05-community-driven-development-with-agents.md"
  - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
  - "knowledge/ideas/2026-05-05-community-intake-flow.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-09-first-manual-community-lab-cycle-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This prepares an ops launch pack based on already captured R3 research and existing advisory community artifacts. It affects project-memory/community operating boundaries but does not implement runtime behavior, launch a surface, add a bot, add integrations, or promote Community Lab into active product behavior."
  research_refs:
    - "knowledge/research/2026-05-09-community-driven-development-governance.md"
    - "knowledge/ideas/2026-05-05-community-driven-development-with-agents.md"
    - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
    - "knowledge/ideas/2026-05-05-community-intake-flow.md"
    - "knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: advisory-ops-and-work-ledger
  required_updates:
    - "knowledge/ops/**"
    - "knowledge/ideas/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The change captures a manual Community Lab cycle launch pack without promoting product docs, active runtime, CLI behavior, bot/adapters, public narrative publication, CommunityPunk, Topic Graph, DAO, token, funding, or governance mechanics."
---

## Context

Step 1 captured the R3 community-driven development governance synthesis.

The next safe step is not a bot, adapter, DAO, token, funding program, or
active CommunityPunk runtime.

The next safe step is a surface-agnostic manual launch pack for Community Lab
cycle 0.

## Intent

Give the maintainer a practical, ready-to-run manual cycle pack that can work in
GitHub Discussions, Telegram, Discord, an existing private group, or another
Community Visor while preserving Punk's truth boundaries.

## In scope

- Advisory ops note for cycle 0 launch.
- Ready-to-copy pinned welcome/policy text.
- Suggested topics/categories.
- Evidence-weighted signal model.
- Manual digest template.
- Promotion rules and success/failure criteria.
- Work-ledger/report/status update for the explicit side-track.

## Out of scope

- Code.
- CLI behavior.
- Runtime `.punk/**`.
- `docs/product/**`.
- `docs/adr/**`.
- `evals/**`.
- Bot/adapters.
- Telegram, Discord, GitHub, Matrix, Discourse, Forgejo, Loomio, or other
  integrations.
- Live send.
- Raw chat or raw transcript storage.
- Automatic issue/goal creation.
- Product/runtime/community surface launch.
- Public narrative publication.
- CommunityPunk runtime.
- Topic Graph runtime.
- DAO, token, funding, bounty, treasury, or microgrant mechanics.

## Outcome

Done in `work/reports/2026-05-09-first-manual-community-lab-cycle-v0-1.md`.

The patch added advisory ops/work-ledger artifacts and a minimal Community Lab
idea pointer only.

It intentionally left selected_next unchanged:

```text
work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md
```

## Follow-up boundary

The next human action is to choose the first surface and run manual cycle 0.

Any automated responder, public surface expansion, CommunityPunk runtime,
Topic Graph, DAO/token, or funding work needs a separate bounded goal.
