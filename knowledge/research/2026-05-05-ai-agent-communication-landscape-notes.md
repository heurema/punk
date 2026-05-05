---
id: research_2026_05_05_ai_agent_communication_landscape_notes
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
review_after: 2026-06-05
research_level: R2
components:
  - project-memory
  - community
  - adapters
  - module-host
  - public-narrative
  - contribution-governance
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
related_docs:
  - docs/product/RESEARCH-GATE.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/MODULE-HOST.md
  - docs/product/MODULES.md
  - docs/product/PUBLIC-NARRATIVE.md
source_refs:
  - https://docs.github.com/discussions
  - https://docs.github.com/en/issues
  - https://core.telegram.org/bots/api
  - https://core.telegram.org/bots/features
  - https://discord.com/developers/docs
  - https://api.slack.com/
  - https://modelcontextprotocol.io/
  - https://github.com/google-a2a/A2A
  - https://langchain-ai.github.io/langgraph/
  - https://microsoft.github.io/autogen/
  - https://docs.crewai.com/
  - https://github.com/openai/swarm
  - https://github.com/heurema/mycel
confidence: medium
supersedes: []
superseded_by: null
---

# AI-agent communication landscape notes for Punk Community Lab

## Research status and authority

This is advisory R2 research.

It does not activate runtime behavior, CLI behavior, Writer behavior, gate/proof
writers, adapters, bots, live sends, module-host behavior, Topic Graph,
CommunityPunk, raw chat storage, or source-of-truth changes.

Research findings do not become product truth until promoted through the normal
Punk path:

```text
research note
  -> ADR / roadmap decision
  -> goal / contract
  -> implementation
  -> eval
  -> gate / proof
  -> project-memory update
```

This note was created from maintainer-provided task context plus an advisory
AI-agent/community communication landscape scan. Source refs are sufficient for
idea capture, but should be revalidated before any ADR, adapter implementation,
or active product behavior depends on them.

## Question

How should Punk use community and AI-agent communication without turning chat,
agents, votes, or external platforms into project truth?

## Decision context

Punk is an early-stage local-first bounded work kernel.

The current repository posture is core-first and trust-surface-first.

Module Host, adapters, PubPunk automation, active CommunityPunk behavior, and
external live side effects are not active.

The relevant Punk laws and boundaries are:

- chat/community surfaces are raw signal, not source of truth;
- repo artifacts remain project truth;
- modules may assess, not decide;
- adapters may invoke, not own truth;
- only `gate` writes final decisions;
- project memory is explicit and authority-tagged;
- raw ideas are not implementation truth by default.

## Sources reviewed

This scan considered:

- GitHub Discussions and Issues;
- Telegram Bot API and bot feature model;
- Discord and Slack app/bot permission models;
- MCP and A2A-style task/tool communication;
- LangGraph, AutoGen, CrewAI, and Swarm-style multi-agent handoff patterns;
- mycel-style async inbox/agent coordination;
- agent-first public community/network patterns such as Moltbook-like systems.

## Source quality table

| Source class | Examples | Tier | Use in this note | Limitation |
|---|---|---|---|---|
| Official platform docs | GitHub, Telegram, Discord, Slack | A | Transport and permission constraints | Does not define Punk authority model |
| Official protocol/tool docs | MCP, A2A, LangGraph, AutoGen, CrewAI, Swarm | A/B | Agent handoff and approval patterns | Not community governance by itself |
| Project-local prior art | mycel | B | Private inbox and async coordination direction | Not selected as dependency now |
| Exploratory agent networks | Moltbook-like public agent communities | C | Failure-mode scouting | Weak trust and noisy authority signals |

## Existing systems / prior art

### Clarification and approval channels

Examples:

- LangGraph interrupts;
- AutoGen user-proxy / human input patterns;
- CrewAI human input;
- A2A task states such as input-required;
- task-runner approvals.

Fit for Punk: high.

Clarification should be a bounded object linked to work/contract context.

Risks:

- interruption fatigue;
- asking the wrong audience;
- leaking private context into a public surface.

### Private mailbox / inbox systems

Examples:

- mycel-like encrypted async mailbox;
- agent inboxes;
- email-like approval queues.

Fit for Punk: high.

This may be more important than a live community bot.

Risks:

- stale backlog;
- hidden queue becoming a second source of truth;
- unpromoted answers lost outside repo artifacts.

### Team chat bots

Examples:

- Telegram bots;
- Slack apps;
- Discord bots.

Fit for Punk: medium as future transport.

Risks:

- chat becomes de facto truth;
- raw transcript storage;
- privacy mode or permission drift;
- live-send reputation damage;
- moderation burden.

### Structured issues and discussions

Examples:

- GitHub Discussions for open community input;
- GitHub Issues for scoped work;
- Discourse-style topic moves/merges/moderation;
- Linear/Jira duplicate and triage flows.

Fit for Punk: high for public/community work routing.

Risks:

- backlog inflation;
- over-formalization too early;
- duplicate chaos if no triage discipline.

### Protocol messaging and task handoffs

Examples:

- MCP;
- A2A;
- OpenAI Swarm-style handoffs;
- webhooks;
- task lifecycle protocols.

Fit for Punk: useful as future adapter/module plumbing, not as community truth.

Risks:

- protocol surface outruns trust boundaries;
- tool/context provider becomes hidden authority.

### Multi-agent councils and public agent social feeds

Examples:

- role-based multi-agent collaboration systems;
- Moltbook-like agent social networks.

Fit for Punk: low as core.

Potential value:

- scouting;
- agent-readable onboarding;
- portable agent identity ideas;
- experimentation.

Risks:

- noise;
- self-reinforcing hallucinations;
- sybil behavior;
- weak identity assurance;
- popularity mistaken for correctness;
- public posting as attack surface.

## Failure modes found

- community chat becomes de facto truth;
- popularity or upvotes become roadmap authority;
- agents reinforce each other's hallucinations;
- prompt injection through community messages;
- leaked secrets in chat;
- duplicate feature chaos;
- moderation burden becomes the real product;
- agents spam repeated proposals;
- non-trivial PRs arrive without linked intent;
- adapters become hidden memory owners;
- Topic Graph becomes a second source of truth.

## Options considered

| Option | Description | Recommendation | Reason |
|---|---|---|---|
| Manual Community Lab | Human-run intake, pinned policy, manual digest | adopt | Low risk, immediate evidence |
| Read-only intake adapter | Forward-only bot/webhook intake with receipts | defer | Needs policy, redaction, receipts |
| Draft-only replies | Agent drafts responses, human approves | defer | Valuable after manual workflow is proven |
| Gated live sends | Bot sends after explicit approval | park | High moderation, reputation, and privacy risk |
| Agent-to-agent public network | Moltbook-like integration | avoid as core / park as research | Weak trust, high noise |
| Private project/team inbox | Clarification and approval queue | adopt as direction / defer implementation | Strong fit for bounded work |
| First-party structured surface | mycel-like "chat that is not chat" | defer | Strong strategic fit, high build cost |

## Recommendation

Adopt the framing and manual experiment:

```text
Community-Signaled, Evidence-Gated Development
```

Start with:

- manual Punk Community Lab;
- pinned participation policy;
- no bot at launch;
- no raw transcript archive;
- manual community digest cycles;
- linked-intent requirement for non-trivial external or agent PRs;
- evidence-weighted support model instead of raw voting.

Do not implement CommunityPunk, Topic Graph, or transport adapters yet.

## Adoption map

### Adopt now

- Manual Punk Community Lab.
- Community-Signaled, Evidence-Gated Development framing.
- Agent advisory policy.
- Support levels that value evidence over vote count.
- Manual digest cycles.
- Linked-intent requirement for non-trivial agent PRs.

### Defer

- Topic Graph schema/model.
- Private clarification inbox implementation.
- Read-only adapters.
- Draft-only replies.
- GitHub Discussions integration.
- mycel integration.

### Park

- Live bot sends.
- CommunityPunk runtime module.
- Telegram, Discord, Slack, GitHub, mycel, or email adapters.
- Agent-to-agent public network integration.
- Automated Topic Graph.

### Avoid

- vote-driven roadmap;
- chat as source of truth;
- raw transcript archive;
- auto-promotion of social signals;
- public agent social feed as truth;
- automatic topic merge by embeddings;
- unbounded external agent PRs.

## Topic Graph and deduplication

Topic Graph should be a derived/rebuildable router, not a new knowledge store.

A safe future design uses a hybrid advisory cascade:

```text
exact alias/slug match
  -> fuzzy title matching
  -> full-text search
  -> semantic candidate retrieval over redacted summaries
  -> graph relation suggestion
  -> maintainer review
```

Never auto-merge by embedding score.

Advisory relations:

```text
duplicate_of
near_duplicate_of
refines
extends
related_to
evidence_for
evidence_against
failure_of
blocked_by
implemented_by
superseded_by
discussed_in
clarified_by
violates_law
```

Authority-sensitive relations such as `duplicate_of`, `implemented_by`,
`superseded_by`, and canonical topic changes require maintainer confirmation.

## Clarification request direction

Clarification requests should be bounded objects, not ambient chat.

Minimum future fields:

```yaml
request_id: string
work_ref: string
topic_ref: string | null
blocked_by: [string]
question: string
audience: owner_only | maintainer_only | team_private | public_community
allowed_surfaces: [string]
privacy_class: public_safe | team_only | owner_only
safe_context_summary: string
response_handling: record_as_signal | update_contract_candidate | discard
status: draft | awaiting_approval | sent | answered | expired | discarded
```

Default audience should be owner/maintainer, not public community.

Public community clarification should require explicit public-safe context.

## Privacy and platform posture

### Telegram

Prefer:

- Bot API only;
- no user-account sessions;
- no impersonation;
- forward-only intake;
- no history scraping;
- no raw transcript storage by default;
- clear pinned policy;
- explicit quote consent;
- receipts for reads/writes if automation appears later.

### Discord

Prefer:

- interaction-first;
- mention/slash-command/DM flows;
- no broad message firehose;
- least privilege.

### Slack

Prefer:

- private approvals and team inboxes;
- minimal scopes;
- event subscriptions only where needed.

### GitHub

Prefer:

- Discussions for open community input;
- Issues for scoped work;
- PRs only against linked intent.

### mycel

Treat as future structured private-surface transport / inbox inspiration.

Do not make it a dependency for the first Community Lab experiment.

## What stays out of scope

- code;
- CLI behavior;
- bot runtime;
- webhook server;
- tokens or environment variables;
- Telegram, Discord, Slack, GitHub, mycel, email, or forum adapters;
- embeddings;
- SQLite index;
- `.punk/views` or `.punk/indexes`;
- Topic Graph implementation;
- CommunityPunk crate/module/runtime;
- docs/product promotion;
- ADR as final architecture decision;
- GitHub issue creation for every idea;
- PR Intake Gate behavior changes;
- voting as roadmap mechanism.

## Impact on roadmap

No roadmap document changes are made by this note.

The idea maps to future Phase 6 module-host, Phase 8 PubPunk/public narrative,
and Phase 9 adapter work, but it remains advisory until explicitly promoted.

## Required evals

No eval spec is added in this patch.

Future work should add evals only when a bounded implementation surface is
selected, such as:

- community signal cannot become project truth without linked artifact;
- adapter receipts cannot own truth;
- agent PR intake requires linked intent;
- Topic Graph output remains derived/rebuildable;
- votes are weak support signals and cannot decide priority.

## Required docs, ADRs, or contracts

Current patch requires only:

- idea artifacts;
- this advisory research note;
- one work goal/report;
- minimal `work/STATUS.md` ledger update.

Future promotion would require a separate ADR or roadmap decision, a bounded
goal/contract, eval implications, and proof-bearing implementation evidence.

## First repo artifacts

This note supports:

- `knowledge/ideas/2026-05-05-community-driven-development-with-agents.md`
- `knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md`
- `work/goals/goal_capture_community_signaled_development_boundary_v0_1.md`

## Open questions

- First public surface: Telegram forum group or GitHub Discussions?
- Should `Agent Contributions` exist from day one?
- Are votes shown publicly or collapsed into weak support summaries?
- How many manual digest cycles before Topic Graph schema work: 2 or 4?
- Who performs the first digest?
- When can a topic become `Contributor-Ready`?
- Should private inbox be designed before any public bot?
