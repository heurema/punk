---
id: idea_2026_05_05_channel_agnostic_community_lab
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
review_after: 2026-06-05
components:
  - community
  - public-narrative
  - project-memory
  - research-intake
related_research:
  - knowledge/research/2026-05-05-ai-agent-communication-landscape-notes.md
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md
  - knowledge/ideas/2026-05-05-community-intake-flow.md
related_docs:
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
  - docs/product/MODULE-HOST.md
  - docs/product/MODULES.md
supersedes: []
superseded_by: null
---

# Channel-agnostic Punk Community Lab

## Status

This is an advisory idea artifact and community surface proposal.

It is not active runtime behavior, not a Telegram feature, and not product truth
by itself.

Operational update:

```text
Manual answering is not the desired default first-response UX.
Manual digest remains a review and aggregation path.
First-message response should be automated by a bounded intake responder once
policy, receipts, and privacy disclosure are defined.
Community Intake Flow is the higher-level flow connecting local Punk, visors,
intake items, discussion, and future promotion.
```

Related responder boundary:

- `knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md`

Related intake-flow boundary:

- `knowledge/ideas/2026-05-05-community-intake-flow.md`

## One-liner

Punk Community Lab is a channel-agnostic external signal surface for real
failures, cases, sources, questions, agent contributions, and ideas around
Punk.

Telegram may be the first Community Visor, but the product primitive is not
Telegram.

The broader architecture is Community Intake Flow.

## Purpose

Punk should not develop in isolation.

The Community Lab creates a controlled external evidence loop:

```text
real failures
  -> community signals
  -> manual triage
  -> topic / refinement / evidence links
  -> repo artifacts
  -> better research, contracts, evals, docs, and future work
```

## Core boundary

Community surfaces provide raw signal.

Repo artifacts remain project truth.

```text
Community surface
  -> raw signal
  -> community inbox item
  -> triage assessment
  -> maintainer review
  -> knowledge/research, knowledge/ideas, work/goals, issue, post, or discard
```

The chat, group, forum, inbox, or network itself is not source of truth.

## Community Visors

Telegram, Discord, GitHub Discussions, web dashboards, mycel/private inboxes,
email, and forums are Community Visors.

They display and collect advisory signals over Community Intake Flow.

They are not source of truth, decision surfaces, issues, or work acceptance.

All visor-originated messages converge into Community Intake Items before any
promotion path is considered.

## Initial surface mode

Manual surface setup only.

No bot is added by this artifact.

No live adapter is added by this artifact.

No history scraping.

No raw transcript archive.

No automatic Topic Graph.

No autonomous replies are added by this artifact.

The next selected direction for first-message handling is a separate bounded
automated intake responder boundary, not manual maintainer answering.

## First surface

A Telegram forum-style group can be used as the first manual surface.

This does not make Telegram canonical.

Recommended initial topics:

```text
Start Here
Failures & Cases
Ideas & Questions
Sources & Research
Agent Contributions
Lab Ops
```

Do not create a Telegram topic for every feature on day one.

Suggested maturity rule:

```text
Level 0: loose discussion in Telegram/forum
Level 1: repeated signal cluster in digest
Level 2: idea/research artifact in repo
Level 3: TopicCard-like metadata inside artifact
Level 4: future derived Topic Graph index
Level 5: future bot can route user to existing topic/thread
```

## Pinned policy draft

```text
Welcome to Punk Community Lab.

This is an open lab for real failures, cases, sources, questions, agent
contributions, and ideas around bounded AI work, contracts, evidence, gates,
proof, and project memory.

You can post:
- real agent/dev workflow failures;
- examples where context, specs, contracts, or evidence broke down;
- sources: papers, tools, repos, issues, benchmarks;
- ideas and questions for Punk;
- bounded agent contribution intents.

Important:
- this chat is not a source of truth;
- do not post secrets, credentials, private logs, customer data, sensitive
  code, or raw prompts;
- AI agents are welcome, but agent output is advisory;
- strong signals may be manually summarized and promoted into repo artifacts;
- raw chat logs are not stored as project truth;
- write "ok to quote" if a message may be quoted publicly;
- non-trivial PRs need linked intent before ordinary review.

Agents may:
- summarize;
- propose;
- find possible duplicates;
- attach evidence;
- draft bounded PR intents;
- implement maintainer-approved linked intent.

Agents may not:
- claim acceptance;
- claim roadmap priority;
- impersonate humans;
- spam repeated proposals;
- paste secrets/private logs/raw prompts;
- open non-trivial PRs without linked intent;
- treat chat as source of truth.

Current mode:
manual lab, no live Punk bot, no automated intake.
```

## Manual digest loop

Run a weekly or periodic manual digest.

This digest is a review and aggregation loop, not the desired default
first-response path.

The desired future first-response path is:

```text
automated responder replies immediately
  -> private receipt records bounded assessment
  -> digest later summarizes redacted patterns for maintainer review
```

Initial digest target:

```text
5-10 signals
1-3 duplicate/refinement links
1-2 candidate topics
0-1 contributor-ready item
parked/avoid examples
confusion notes
```

Suggested digest structure:

```md
# Community Intake Digest 0001

## Surface

Surface:
Period:
Maintainer:
Raw logs stored: no

## Signals

| Ref | Kind | Summary | Route |
|---|---|---|---|

## Topic clusters observed

| Cluster | Signals | Possible repo route |
|---|---|---|

## Agent contributions

| Signal | Agent role | Linked intent? | Route |
|---|---|---|---|

## Promoted

- ...

## Parked / discarded

- ...

## What confused us

- ...

## Next experiment

- ...
```

## Relationship to Topic Graph

Topic Graph is a future derived/rebuildable router over repo artifacts and
community receipts.

It is not a canonical memory store.

Do not implement Topic Graph before at least 2-4 manual digest cycles.

## Relationship to CommunityPunk

CommunityPunk is a future module candidate.

Current status:

```text
future / parked
```

CommunityPunk may eventually handle:

- triage;
- duplicate/topic assessment;
- reply drafts;
- clarification requests;
- promotion candidates;
- contribution routing.

It may not:

- write final decisions;
- own truth;
- bypass gate;
- treat chat as canonical;
- send live messages without explicit approval and receipts.

## Relationship to adapters

Telegram, Discord, GitHub, Slack, mycel, email, forums, and future agent
networks are transport surfaces/adapters.

Adapters may invoke or transport.

Adapters may not own project truth.

## Non-goals now

- no Telegram Bot API integration;
- no Discord, Slack, GitHub, or mycel webhook integration;
- no bot tokens;
- no environment variables;
- no live send;
- no topic auto-creation;
- no raw transcript archive;
- no `.punk/` runtime writes;
- no schema/template system;
- no code changes;
- no docs/product promotion.

## First manual launch checklist

- [ ] Create group/forum.
- [ ] Create initial topics.
- [ ] Pin policy.
- [ ] Do not add bot yet.
- [ ] Record group/channel metadata manually if published.
- [ ] Run first digest after first signal window.
- [ ] Promote only curated summaries into repo artifacts.
- [ ] Record what failed or felt confusing.

## Adoption map

| Mechanism | Recommendation | Reason |
|---|---|---|
| Manual Community Lab | adopt | Useful and low-risk |
| Telegram as first surface | adopt as surface only | Convenient, but not product primitive |
| GitHub Discussions | defer/adopt as future structured surface | Better public work routing |
| Agent Contributions topic | adopt | Makes agent-originated input explicit |
| Topic Graph | defer | Needs manual evidence first |
| Read-only bot | defer | Requires privacy and receipt policy |
| Live bot | park | High side-effect and moderation risk |
