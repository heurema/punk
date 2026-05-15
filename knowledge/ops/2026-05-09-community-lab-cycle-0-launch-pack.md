---
id: ops_2026_05_09_community_lab_cycle_0_launch_pack
kind: ops-note
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-09
updated_at: 2026-05-09
review_after: 2026-06-05
components:
  - community
  - project-memory
  - contribution-governance
  - agents
related_research:
  - knowledge/research/2026-05-09-community-driven-development-governance.md
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
  - knowledge/ideas/2026-05-05-community-intake-flow.md
  - knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
  - CONTRIBUTING.md
supersedes: []
superseded_by: null
---

# Community Lab cycle 0 launch pack

## Purpose

The first Community Lab cycle is a manual external signal experiment.

It collects real failures, questions, ideas, sources, and PR-intent from people
and user-controlled agents.

It does not create product truth, roadmap authority, acceptance, merge
authority, or canonical topic status.

Community surfaces are raw signal surfaces. Repo artifacts remain project
truth.

## Current mode

- Manual cycle.
- No bot.
- No live adapter.
- No automatic issue creation.
- No raw transcript archive.
- No roadmap voting.
- No DAO/token/funding.
- No CommunityPunk runtime.
- No Topic Graph runtime.

## Surface choice

Choose one first surface before launch.

Do not make the first surface canonical. It is only the first Community Visor
for manual cycle 0.

GitHub Discussions surface-specific prep is tracked separately in
`knowledge/ops/2026-05-09-community-lab-cycle-0-github-discussions-runbook.md`.

### Option A: GitHub Discussions

Best for repo proximity and public async discussion.

Tradeoff:

- lower social energy;
- better traceability;
- closer to issue and PR workflows;
- easier to link repo artifacts without treating chat as truth.

### Option B: Telegram forum group

Best for fast social energy and early community signal.

Tradeoff:

- higher moderation discipline needed;
- higher privacy discipline needed;
- stronger pinned-policy clarity needed;
- easier for chat to feel like authority unless boundaries are repeated.

### Option C: existing private/small invite group

Best for controlled first test.

Tradeoff:

- weaker public-build visibility;
- safer for testing the digest loop;
- less representative of open community behavior.

## Pinned welcome / policy text

Ready-to-copy pinned message:

```text
Welcome to Punk Community Lab.

This is a lab for real failures, cases, sources, questions, ideas, and bounded
contribution intent around Punk and bounded AI work.

You can post:
- real agent/dev workflow failures;
- concrete cases where context, specs, contracts, evidence, gates, proof, or
  project memory broke down;
- sources: papers, tools, repos, issues, benchmarks, docs;
- ideas and questions for Punk;
- bounded contribution or PR intent.

Important:
- this chat/surface is not a source of truth;
- repo artifacts remain truth;
- do not post secrets, credentials, private logs, customer data, sensitive
  code, or raw prompts;
- AI agents are welcome only as advisory contributors;
- agent output is advisory;
- non-trivial PRs require linked intent before ordinary review;
- write "ok to quote" if a message may be quoted publicly;
- strong signals may be manually summarized into repo artifacts;
- no bot is currently active.

Good signals include concrete failures, reproducible cases, source links with
why they matter, duplicate/refinement clues, and bounded contribution intent.

Current mode: manual cycle, no bot, no live adapter, no automatic issue
creation, no roadmap voting.
```

## Suggested initial topics / channels

For a forum-like surface:

- Start Here
- Failures & Cases
- Ideas & Questions
- Sources & Research
- Agent Contributions
- Lab Ops

For GitHub Discussions:

- pinned Start Here post;
- categories or labels if available:
  - failures/cases;
  - ideas/questions;
  - sources/research;
  - agent-contributions;
  - lab-ops.

Do not create a topic for every Punk feature during cycle 0.

## What counts as a useful signal

Use evidence-weighted support.

Weak:

- reaction only;
- vague idea;
- "+1";
- repeated agent-generated suggestion without new evidence.

Medium:

- concrete use case;
- reasoned comment;
- related source;
- duplicate/refinement clue.

Strong:

- real failure report;
- reproducible case;
- source with explanation;
- bounded PR-intent;
- contributor commitment;
- maintainer strategic need.

## First cycle target

Cycle 0 target:

- duration: 7 to 14 days or until 10 useful signals;
- target: 5-10 signals;
- 1-3 duplicate/refinement links;
- 1-2 candidate topics;
- 0-1 contributor-ready item;
- parked/avoid examples;
- confusion notes.

## Intake categories

Use these manual categories:

- idea;
- failure;
- bug;
- research/source;
- question;
- PR-intent;
- agent-contribution;
- duplicate/refinement;
- noise/spam.

## Manual digest template

Ready-to-copy digest template:

```md
# Community Intake Digest 0001

## Surface

Surface:
Period:
Maintainer:
Raw logs stored: no

## Signals

| Ref | Kind | Safe summary | Evidence level | Route |
|---|---|---|---|---|

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

## Promotion rules

Signal != issue.

Idea != work.

Chat != truth.

Issue creation is promotion.

PR requires linked intent.

A signal may become:

- research note;
- idea artifact;
- work goal;
- GitHub issue;
- public narrative candidate;
- discarded/parked/avoid.

Only maintainer review or future gate can promote.

Do not let a reaction count, chat thread, bot answer, agent claim, or repeated
suggestion become roadmap priority, project truth, acceptance, merge authority,
or canonical topic status.

## First-cycle success criteria

Success means:

- people understand what to post;
- no secrets/raw private logs are collected;
- at least a few useful signals appear;
- at least one duplicate/refinement is identified;
- at least one signal can be routed to research/idea/work or parked/avoid;
- project truth remains in repo artifacts;
- maintainer workload stays bounded.

Failure means:

- chat becomes roadmap voting;
- raw logs become memory;
- users expect instant implementation;
- agents spam proposals;
- non-trivial PRs appear without linked intent;
- surface requires bot/moderation before manual loop is understood.

## Next possible step after cycle

After one manual digest:

- review whether to run cycle 2;
- decide if GitHub Discussions or Telegram should remain the main visor;
- decide whether external automated responder is justified;
- do not implement responder before privacy, receipts, policy, and moderation
  are clear.

Possible follow-up artifacts:

- cycle 0 digest report;
- cycle 1 launch adjustment;
- external responder policy refinement;
- separate decision on first public surface if cycle 0 used a private/small
  group.
