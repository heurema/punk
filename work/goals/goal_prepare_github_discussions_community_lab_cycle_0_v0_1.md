---
id: goal_prepare_github_discussions_community_lab_cycle_0_v0_1
title: "Prepare GitHub Discussions Community Lab cycle 0 v0.1"
status: done
owner: "vitaly"
authority: canonical
module: "public-narrative"
priority: P2
created_at: 2026-05-09
updated_at: 2026-05-09
selected_at: 2026-05-09
started_at: 2026-05-09
completed_at: 2026-05-09
blocked_by: []
scope:
  include:
    - "knowledge/ops/2026-05-09-community-lab-cycle-0-github-discussions-runbook.md"
    - "publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md"
    - "publishing/channels/github-discussions-community-lab.md"
    - "work/goals/goal_prepare_github_discussions_community_lab_cycle_0_v0_1.md"
    - "work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md"
    - "work/STATUS.md"
    - "knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md"
  exclude:
    - "code"
    - "CLI"
    - "runtime .punk/**"
    - "docs/product/**"
    - "docs/adr/**"
    - "evals/**"
    - "GitHub API calls"
    - "actual publication"
    - "publication receipt"
    - "bot/adapters/integrations"
    - "raw transcript storage"
    - "automatic issue/goal creation"
    - "DAO/token/funding"
    - "CommunityPunk runtime"
    - "Topic Graph runtime"
acceptance:
  - "Creates a GitHub Discussions-specific manual launch runbook."
  - "Creates ready-to-copy discussion post draft."
  - "Creates channel descriptor."
  - "States GitHub Discussions is a Community Visor, not truth."
  - "States no publication has happened yet."
  - "Tells maintainer to create receipt only after manual posting."
  - "Adds no product/runtime/CLI/bot/adapter behavior."
  - "Leaves selected_next unchanged."
knowledge_refs:
  - "knowledge/research/2026-05-09-community-driven-development-governance.md"
  - "knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md"
  - "knowledge/ops/2026-05-09-community-lab-cycle-0-github-discussions-runbook.md"
  - "knowledge/ideas/2026-05-05-community-driven-development-with-agents.md"
  - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
  - "knowledge/ideas/2026-05-05-community-intake-flow.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This prepares a surface-specific public draft based on already captured R3 research and the Step 2 launch pack. It affects public-narrative/community operating boundaries but does not publish, call GitHub APIs, implement runtime behavior, add a bot, add integrations, or promote Community Lab into active product behavior."
  research_refs:
    - "knowledge/research/2026-05-09-community-driven-development-governance.md"
    - "knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md"
    - "knowledge/ideas/2026-05-05-community-driven-development-with-agents.md"
    - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
    - "knowledge/ideas/2026-05-05-community-intake-flow.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: advisory-ops-public-draft-and-work-ledger
  required_updates:
    - "knowledge/ops/**"
    - "publishing/posts/**"
    - "publishing/channels/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The change captures a manual GitHub Discussions Community Lab launch draft without product docs promotion, active runtime, CLI behavior, GitHub API calls, bot/adapters, CommunityPunk, Topic Graph, DAO, token, funding, publication receipt, or actual publication."
---

## Context

Step 1 captured the R3 Community-Signaled, Evidence-Gated Development
governance synthesis.

Step 2 captured a surface-agnostic Community Lab cycle 0 launch pack.

The maintainer selected GitHub Discussions as the default first Community Visor
because it is close to the repo, easier to connect to PR/Issue workflows, and
lower risk than Telegram for privacy/moderation in the first manual loop.

## Intent

Prepare ready-to-copy GitHub Discussions launch materials so the maintainer can
manually open the first Community Lab discussion.

## In scope

- GitHub Discussions-specific manual launch runbook.
- Ready-to-copy discussion title/body.
- GitHub Discussions channel descriptor.
- Work-ledger/report/status update for the explicit side-track.
- Minimal pointer from the surface-agnostic launch pack.

## Out of scope

- Code.
- CLI behavior.
- Runtime `.punk/**`.
- `docs/product/**`.
- `docs/adr/**`.
- `evals/**`.
- GitHub API calls.
- Actual publication.
- Publication receipt.
- GitHub Issues.
- Bot/adapters/integrations.
- Raw chat or raw transcript storage.
- Automatic issue/goal creation.
- CommunityPunk runtime.
- Topic Graph runtime.
- DAO, token, funding, bounty, treasury, or microgrant mechanics.

## Outcome

Done in
`work/reports/2026-05-09-github-discussions-community-lab-cycle-0-prep-v0-1.md`.

The patch added advisory ops/public-draft/channel/work-ledger artifacts only.

It intentionally left selected_next unchanged:

```text
work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md
```

## Follow-up boundary

The next human action is to manually enable/open GitHub Discussions, post the
draft, and then create a publication receipt.

No future agent should infer that publication happened from this draft alone.
