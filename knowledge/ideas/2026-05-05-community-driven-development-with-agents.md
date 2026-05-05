---
id: idea_2026_05_05_community_driven_development_with_agents
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
review_after: 2026-06-05
components:
  - project-memory
  - community
  - contribution-governance
  - agents
  - research-intake
related_research:
  - knowledge/research/2026-05-05-ai-agent-communication-landscape-notes.md
related_ideas:
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/MODULE-HOST.md
  - docs/product/MODULES.md
supersedes: []
superseded_by: null
---

# Community-Signaled, Evidence-Gated Development with AI Agents

## Status

This is an advisory idea artifact.

It is not active-core behavior, not a roadmap decision, not an active module,
and not implementation truth.

## One-liner

Punk should accept community and user-agent input as advisory signal and
evidence, then route it through explicit topic matching, maintainer review,
linked intent, evals, and future gate/proof before it becomes project truth or
work.

## Principle

Community-Signaled, Evidence-Gated Development.

Community members and their AI agents may provide:

- ideas;
- failures;
- reproducible cases;
- source links;
- research leads;
- duplicate or topic suggestions;
- clarification answers;
- bounded PR intents;
- implementation attempts against maintainer-approved linked intent.

They do not own:

- roadmap priority;
- project truth;
- final decisions;
- acceptance;
- canonical topic status;
- merge authority.

## Strategic framing

The concept is not:

```text
community voting controls the roadmap
```

The concept is:

```text
community and user agents provide advisory signals
  -> Punk preserves evidence and boundaries
  -> maintainers review linked intent
  -> evals and future gate/proof decide acceptance
```

Roadmap, truth, final decisions, and acceptance remain governed by Punk laws,
repo artifacts, maintainers, evals, and future gate/proof.

## Core model

```text
Community Surface
  -> Community Signal
  -> Community Inbox Item
  -> Topic Match Assessment
  -> Topic / Evidence / Law / Phase Assessment
  -> Maintainer Review
  -> Linked Intent Artifact
  -> Research / Goal / Issue / PR / Publishing / Discard
```

Community surfaces are raw signal surfaces only.

They do not own project truth.

## Why this matters

If Punk becomes useful, many humans and many AI agents may bring overlapping
ideas, duplicate requests, proposed fixes, evidence links, and PR attempts.

A raw community feed would create noise and false authority.

The goal is to convert noisy community and agent activity into bounded,
inspectable project-memory artifacts without letting chat, popularity, or agent
volume become authority.

## Current boundary

The current boundary is manual and repo-safe:

```text
Community Signal
  -> Community Inbox Item
  -> Topic / Duplicate / Evidence Assessment
  -> Maintainer Review
  -> Linked Intent Artifact
  -> Research / Goal / Issue / PR / Discard
```

No live bot, adapter, Topic Graph runtime, CommunityPunk active module, or raw
chat storage is introduced by this idea.

## Signal classes

Initial signal taxonomy:

```text
idea
research/source
bug
failure
question
PR-intent
agent-contribution
noise
```

Do not over-model this taxonomy until manual digest cycles show which
distinctions are useful.

## Topic relation vocabulary

Initial advisory vocabulary:

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

This is design vocabulary only.

It does not create a Topic Graph database, schema system, automatic merge
policy, or canonical topic authority.

## Support levels

Votes and reactions are weak support signals, not decisions.

Support score counts evidence diversity, not crowd size.

```text
L0 reaction / like
L1 comment with reason
L2 concrete use case
L3 real failure report
L4 reproducible artifact / test / source / benchmark
L5 contributor commitment with bounded intent
L6 maintainer strategic need
```

Example:

```text
240 reactions + no reproducible case + no contributor commitment
  -> weak or medium support

4 reactions + 2 concrete failures + 1 reproducible case + 1 bounded contributor intent
  -> strong support
```

## Priority assessment dimensions

A topic assessment should consider:

- Punk fit;
- law compatibility;
- phase fit;
- evidence strength;
- signal diversity;
- contributor commitment;
- cost / complexity;
- reversibility;
- strategic need;
- risk.

## Routing buckets

Suggested output buckets:

```text
needs clarification
duplicate or refinement
research candidate
contributor-ready
backlog topic
parked
avoid
adopt now
```

`adopt now` should be rare.

Most community input should become one of:

- duplicate or refinement;
- research candidate;
- backlog topic;
- parked;
- avoid;
- contributor-ready after maintainer review.

## Anti-sybil and anti-noise posture

Votes, reactions, and agent endorsements are advisory only.

Rules:

- raw vote count never decides roadmap;
- repeated low-evidence comments collapse into one signal cluster;
- identical or near-identical agent-generated messages are deduplicated
  aggressively;
- agent identities should be disclosed when possible;
- trusted contributors earn trust from evidence/history, not volume;
- unique workflows matter more than unique accounts;
- concrete failures matter more than popularity;
- suspected brigading becomes moderation signal, not priority signal.

## Agent policy

AI agents are welcome as advisory contributors.

Agents may:

- summarize;
- propose;
- find possible duplicates;
- attach evidence;
- challenge assumptions;
- draft bounded PR intents;
- implement maintainer-approved linked intent.

Agents may not:

- claim acceptance;
- claim roadmap priority;
- impersonate humans;
- spam repeated proposals;
- paste secrets, private logs, credentials, raw prompts, or customer data;
- open non-trivial PRs without linked intent;
- treat chat as source of truth;
- merge topics;
- approve PRs;
- write final decisions;
- declare canonical truth.

Inter-agent message posture:

```text
Agents may assess, suggest, link, challenge, and implement bounded linked intent.
Agents may not decide, accept, merge, promote truth, or change roadmap.
```

## Advisory message intents

Future agent-to-agent communication should be typed and advisory, not free-form
decision-making.

Allowed advisory intents may include:

```text
propose_signal
suggest_duplicate
attach_evidence
challenge_assumption
request_clarification
propose_route
claim_contribution_intent
submit_pr_link
report_validation_result
```

Forbidden authority intents:

```text
accept_topic
merge_topic
approve_pr
write_decision
declare_canonical
change_roadmap
```

## Lanes instead of one backlog

A single backlog will not scale.

Community input should flow through lanes:

```text
Inbox
Needs Clarification
Duplicate / Refinement
Evidence Candidates
Research Candidates
Contributor-Ready
Active-Core Candidates
Community Backlog
Parked
Avoid / Anti-patterns
Implemented / Done
```

The most important lane for AI-agent contributions is `Contributor-Ready`.

A topic should become `Contributor-Ready` only when:

- the problem is clear;
- existing options were checked;
- a no-code alternative was considered;
- scope is bounded;
- target artifact or surface is known;
- Research Gate classification is known;
- validation path is known;
- maintainer marks `PR welcome`, `research welcome`, `eval welcome`, or
  `docs welcome`.

## Issue and PR boundary

```text
Signal != Topic
Topic != Issue
Idea != Issue
Issue = scoped work candidate
PR = implementation attempt against linked intent
```

A non-trivial agent or external PR requires linked intent.

Suggested path:

```text
1. User/agent finds existing topic or proposes intake candidate.
2. Triage links it to topic / idea / research / goal.
3. Maintainer marks one of:
   - PR welcome
   - research welcome
   - eval welcome
   - docs welcome
   - not ready
4. Agent opens bounded PR.
5. PR links intent and follows the PR template.
6. PR Intake Gate evaluates metadata.
7. Human review / future gate decides.
```

This does not weaken existing PR Intake Gate rules.

## First experiment

Run 2-4 manual Punk Community Lab digest cycles before implementing
CommunityPunk, Topic Graph, or live adapters.

Each digest should test:

- can people and agents bring useful signals?
- can maintainers classify without chaos?
- can duplicates/refinements be linked?
- can signals become repo artifacts?
- can the project avoid chat becoming truth?
- which agent behaviors need stricter rules?

## Non-goals

- vote-driven roadmap;
- agent democracy;
- autonomous acceptance;
- public agent social feed as project truth;
- chat as source of truth;
- raw transcript memory;
- unbounded agent PRs;
- automatic issue creation for every idea;
- automatic topic merge by embedding score;
- live community bot as the starting point;
- CommunityPunk active module now;
- Telegram, Discord, GitHub, Slack, mycel, or email adapter now.

## Adoption map

| Mechanism | Recommendation | Reason |
|---|---|---|
| Manual Punk Community Lab | adopt | Low risk; gives external evidence without runtime drift |
| Agent policy for community input | adopt | Prevents agent spam and false authority |
| Support levels over raw votes | adopt | Reduces sybil and popularity bias |
| Contributor-ready lane | adopt | Lets agents help without unbounded PR chaos |
| Private clarification inbox | defer | Strong fit, but needs its own bounded design |
| Topic Graph implementation | defer | Needs manual digest evidence first |
| Read-only adapter | defer | Needs receipts and redaction policy first |
| Draft-only replies | defer | Valuable after policies and receipts exist |
| Live bot sends | park | High moderation and reputation risk |
| Public agent social feed as truth | avoid | High noise, sybil, and hallucination risk |

## Review trigger

Review after the first 2-4 manual digest cycles or by 2026-06-05, whichever
comes first.
