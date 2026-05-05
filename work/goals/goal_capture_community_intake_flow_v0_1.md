---
id: goal_capture_community_intake_flow_v0_1
title: "Capture community intake flow boundary v0.1"
status: done
owner: "vitaly"
module: "project-memory"
priority: P2
authority: canonical
created_at: 2026-05-05
updated_at: 2026-05-05
selected_at: 2026-05-05
started_at: 2026-05-05
completed_at: 2026-05-05
blocked_by: []
scope:
  include:
    - "knowledge/ideas/2026-05-05-community-intake-flow.md"
    - "knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md"
    - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
    - "work/goals/goal_capture_community_intake_flow_v0_1.md"
    - "work/reports/2026-05-05-community-intake-flow-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "crates/**"
    - "docs/product/**"
    - "docs/adr/**"
    - "evals/specs/**"
    - ".github/**"
    - "scripts/**"
    - "knowledge/_templates/**"
acceptance:
  - "Records Community Intake Flow as the broader advisory architecture above proposal-only and responder-only framing."
  - "Defines Community Intake Item as a normalized candidate item for improvements, bugs, failures, sources, questions, PR-intents, agent contributions, duplicate/refinement suggestions, and noise."
  - "Defines Community Visors as display/participation surfaces that do not own truth, decisions, issues, or work acceptance."
  - "Clarifies that Automated Intake Responder is a visor automation component, not the center of the architecture."
  - "Clarifies issue creation and PR-readiness as promotion paths requiring maintainer/future gate review and linked intent."
  - "Preserves no-code, no-runtime, no-adapter, no-bot, no-Topic-Graph, no-CommunityPunk-runtime, no-auto-issue, no-source-of-truth-change boundaries."
knowledge_refs:
  - "knowledge/ideas/2026-05-05-community-intake-flow.md"
  - "knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md"
  - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
contract_refs: []
report_refs:
  - "work/reports/2026-05-05-community-intake-flow-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R2
  required: true
  rationale: "This side-track refines Community Lab architecture and touches project memory, community architecture, future adapter/module boundaries, issue/PR promotion policy, external/community visors, and contribution governance. It records advisory docs/work-ledger artifacts only; no implementation is added."
  research_refs:
    - "knowledge/ideas/2026-05-05-community-driven-development-with-agents.md"
    - "knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md"
    - "knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md"
    - "work/reports/2026-05-05-automated-community-intake-responder-boundary-v0-1.md"
  external_research_refs:
    - "CONTRIBUTING.md"
    - ".github/PULL_REQUEST_TEMPLATE.md"
  blocked_reason: null
doc_impact:
  classification: advisory-research-and-work-ledger
  required_updates:
    - "knowledge/ideas/**"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "The change is an advisory terminology and architecture refinement. It does not promote Community Intake Flow, visors, responder, CommunityPunk, Topic Graph, adapter, or issue automation into active product truth."
---

## Context

After PR #36, the maintainer corrected the architecture framing.

The Automated Intake Responder is useful, but it should not be the center of
the architecture.

The broader primitive is Community Intake Flow.

## Intent

Capture the Community Intake Flow and Community Visor terminology before any
external bot, visor, intake ID, issue-promotion, or CommunityPunk work.

## In scope

- Advisory idea artifact for Community Intake Flow.
- Minimal updates to the responder and channel-agnostic Community Lab idea
  artifacts.
- Work-ledger/report/status update for the explicit side-track.

## Out of scope

- Code.
- CLI behavior.
- `.punk/` runtime or derived storage.
- Telegram, Discord, GitHub, Slack, mycel, email, or forum adapter.
- Bot.
- Live send.
- Topic Graph implementation.
- Embeddings or duplicate scoring implementation.
- CommunityPunk runtime.
- Module Host behavior.
- Raw chat storage.
- Issue/goal auto-creation.
- Roadmap automation.
- PR Intake Gate behavior changes.
- Product docs promotion claiming this is active.
- ADR.
- `knowledge/_templates/*`.

## Outcome

Done in `work/reports/2026-05-05-community-intake-flow-v0-1.md`.

The patch added advisory idea/work-ledger artifacts only.

It intentionally left runtime, CLI, dependencies, canonical product docs, ADRs,
eval specs, crate code, adapters, bots, Topic Graph, CommunityPunk, raw chat
storage, auto issue creation, and external side effects unchanged.
