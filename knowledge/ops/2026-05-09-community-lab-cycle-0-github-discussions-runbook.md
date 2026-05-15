---
id: ops_2026_05_09_community_lab_cycle_0_github_discussions_runbook
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
  - public-narrative
  - agents
related_research:
  - knowledge/research/2026-05-09-community-driven-development-governance.md
related_ops:
  - knowledge/ops/2026-05-09-community-lab-cycle-0-launch-pack.md
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
  - knowledge/ideas/2026-05-05-community-intake-flow.md
related_publication_drafts:
  - publishing/posts/2026-05-09-community-lab-cycle-0-github-discussions-start-here.md
related_channels:
  - publishing/channels/github-discussions-community-lab.md
supersedes: []
superseded_by: null
---

# Community Lab cycle 0 GitHub Discussions runbook

## Purpose

This runbook prepares the first manual GitHub Discussions surface for Punk
Community Lab cycle 0.

It is a manual public signal experiment, not product behavior.

The maintainer can use this file to manually open one Start Here discussion,
pin it, and collect advisory replies as raw community signals.

## Current mode

- GitHub Discussions is a Community Visor, not source of truth.
- Repo artifacts remain truth.
- Discussion replies are raw/advisory signals.
- No bot.
- No auto issue creation.
- No roadmap voting.
- No automatic PR readiness.
- No raw transcript archive.
- No CommunityPunk runtime.
- No Topic Graph runtime.

## Recommended GitHub Discussions setup

- Create one pinned Start Here discussion manually.
- Use existing categories if they are already available.
- If categories can be chosen manually, prefer:
  - Announcements / Start Here
  - Ideas
  - Q&A
  - Show and tell / Cases
  - Research / Sources
- Do not require repository settings changes in this patch.
- Do not create labels or categories through code.

## Ready-to-copy discussion title

```text
Punk Community Lab Cycle 0: failures, cases, sources, ideas, and bounded contribution intent
```

## Ready-to-copy discussion body

```md
# Punk Community Lab Cycle 0

Punk is a contract-first project memory and proof/gate system for AI-assisted
software work. Today it is being built in small bounded slices, and repo
artifacts remain the durable record of product truth.

This Discussion is the first manual Punk Community Lab cycle. It is a place to
collect real failures, cases, sources, questions, ideas, and bounded
contribution intent from people and user-controlled AI agents.

Important boundary: this Discussion is not source of truth. Repo artifacts
remain truth. Replies here are raw/advisory signals until a maintainer manually
summarizes or routes them into a repo artifact.

## What to post

Useful replies can include:

- real AI/dev workflow failures;
- examples where context, specs, contracts, evidence, gates, proof, or project
  memory broke down;
- sources: papers, tools, repos, issues, benchmarks, docs;
- ideas and questions for Punk;
- bounded PR or contribution intent;
- agent-originated suggestions, clearly marked as agent output.

## Boundaries

- Reactions and votes do not decide the roadmap.
- No implementation is promised.
- Non-trivial PRs require linked intent before ordinary review.
- AI agent output is advisory.
- Do not post secrets, credentials, private logs, customer data, sensitive code,
  or raw prompts.
- Write "ok to quote" if a message may be quoted publicly.
- Strong signals may be manually summarized into repo artifacts.
- Summaries and digests should use safe summaries, not raw transcript archives.
- No bot is currently active.

## What a useful signal looks like

When possible, include:

- expected behavior;
- what happened;
- tool, agent, or workflow involved;
- safe link/source if available;
- why it matters;
- whether you are willing to help with docs, research, eval, or a bounded PR.

## How signals may be used

A signal may be manually:

- summarized into a digest;
- routed into research, ideas, goals, or issues;
- parked, avoided, or discarded;
- included only as a safe summary, not as a raw log.

Signal != issue. Idea != work. Chat != truth. Issue creation is promotion.

Only maintainer review or a future gate can promote a signal.

## Cycle 0 target

Cycle 0 runs for 7-14 days or until 10 useful signals appear.

After that, the maintainer may publish a manual digest and decide whether to
run another cycle.

## Reply with one of these

- Failure
- Source
- Idea
- Question
- PR-intent
- Agent contribution
```

## Maintainer checklist before posting

- Confirm GitHub Discussions is enabled.
- Choose/pin category manually.
- Review the copy.
- Confirm no private/internal info is in the post.
- Post manually.
- Save publication receipt after posting.
- Do not create issues immediately.
- Do not promise roadmap changes.

## Publication receipt placeholder guidance

After manual posting, create a publication receipt under:

```text
publishing/publications/YYYY-MM-DD-community-lab-cycle-0-github-discussions.md
```

Do not create that receipt before the post exists publicly.

This patch intentionally does not create a receipt because nothing has been
published.

## First response examples

For a failure report:

```text
Thanks. This is a useful failure signal. I will only use it as a safe summary
unless you write "ok to quote". If you can add expected behavior, what happened,
and the tool/workflow involved, it will be easier to route.
```

For a source:

```text
Thanks. Please add one sentence on why this source matters for Punk. A link
alone is a weak signal; a link plus relevance is easier to route.
```

For PR-intent:

```text
Thanks. Before a non-trivial PR, please link the intended change to an existing
repo artifact or ask for maintainer intent to be created. PR-intent is useful,
but it is not automatic acceptance.
```

For a vague idea:

```text
Can you make this concrete with a failure, workflow, source, or expected
outcome? Right now this is an idea signal, not work.
```

For unsafe/private data:

```text
Please remove secrets, private logs, customer data, sensitive code, and raw
prompts. A safe summary is enough for this lab.
```
