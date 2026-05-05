---
id: idea_2026_05_05_community_intake_flow
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
review_after: 2026-06-05
components:
  - community
  - project-memory
  - contribution-governance
  - adapters
  - agents
  - public-narrative
related_research:
  - knowledge/research/2026-05-05-ai-agent-communication-landscape-notes.md
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
  - knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/MODULE-HOST.md
  - CONTRIBUTING.md
  - .github/PULL_REQUEST_TEMPLATE.md
supersedes: []
superseded_by: null
---

# Community Intake Flow

## Status

This is an advisory and exploratory idea artifact.

It is not active runtime behavior, not product truth, not a Telegram bot
architecture, not a GitHub Issue workflow implementation, and not a
CommunityPunk runtime.

It refines the Community Lab architecture after the Automated Intake Responder
boundary:

```text
Automatic first response,
human / future gate for promotion.
```

The responder is useful, but it is not the center of the architecture.

The center is Community Intake Flow.

## Why "proposal" is too narrow

"Proposal Flow" makes the system sound like every community input is an
improvement proposal.

That is too narrow.

Punk Community Lab must also handle:

- bugs;
- failures;
- reproducible cases;
- research/source links;
- questions;
- clarification replies;
- PR intents;
- agent contribution signals;
- duplicate/refinement suggestions;
- noise and spam.

A proposal is one kind of Community Intake Item.

It is not the umbrella primitive.

Optional secondary phrase:

```text
Community Work Signal Flow
```

Use `Community Intake Flow` as the central term.

## Definition

Community Intake Flow is the high-level flow that accepts community, user, and
agent input, normalizes it into an intake item, classifies it, checks repo
artifacts, routes it, displays safe summaries in visors, and may later promote
it to repo artifacts, GitHub Issues, PR paths, publishing, or discard.

Core architecture:

```text
User / User Agent / Community Agent
  -> Punk-mediated Intake
  -> Community Intake Item
  -> Classification + Policy Check + Repo Awareness
  -> Route:
       duplicate/refinement
       needs clarification
       bug/failure evidence
       research/source candidate
       issue candidate
       contributor-ready candidate
       parked
       avoid
       discard/noise
  -> Community Visors for visibility/discussion
  -> Maintainer / future gate promotion
  -> Repo artifact / GitHub Issue / PR path / publishing / discard
```

Community Visors display and collect signals.

They are not source of truth.

## Community Intake Item

A Community Intake Item is a normalized candidate item created from any incoming
signal.

Initial kinds:

```text
improvement
bug
failure
research/source
question
clarification
PR-intent
agent-contribution
duplicate/refinement suggestion
noise
```

An item is advisory until promoted through maintainer review or future gate.

It may link:

- original safe summary;
- classification;
- source surface;
- related repo artifacts;
- advisory relations;
- route;
- status;
- evidence requests;
- private receipt refs;
- later promotion refs.

It must not preserve raw secrets, private logs, customer data, raw prompts, or
unredacted chat transcripts as project truth.

## Community Visor

A Community Visor is a display and participation surface over Community Intake
Flow.

Examples:

```text
Telegram
Discord
GitHub Discussions
GitHub Issues only after promotion
web dashboard later
mycel/private inbox later
email/forum later
```

Important:

```text
Visor != truth
Visor != decision surface
Visor != issue
Visor != work acceptance
```

A visor may collect replies and show safe summaries or status updates.

The repo remains the project-truth surface.

## Automated Intake Responder

The Automated Intake Responder is a Community Visor automation component.

It is not the center of the architecture.

It may operate inside a visor such as Telegram, Discord, GitHub Discussions, a
future web dashboard, mycel/private inbox, email, or forum.

It may:

- classify;
- search repo artifacts;
- policy-check;
- reply;
- write private receipts.

It may not:

- create truth;
- create issues/goals;
- set roadmap priority;
- accept PRs;
- write final decisions;
- become CommunityPunk runtime.

Responder output is advisory assessment inside Community Intake Flow.

## Relationship to CommunityPunk

CommunityPunk is a future/parked module candidate.

It is not activated by this idea.

CommunityPunk may eventually assess intake items, suggest duplicate/refinement
links, draft clarification questions, or prepare promotion candidates.

It may not decide, accept, merge, promote truth, own roadmap priority, or bypass
`gate`.

## Relationship to GitHub Issues and PRs

GitHub Issues are promotion artifacts, not raw intake.

PRs are implementation attempts against linked intent.

```text
Signal != Intake Item
Intake Item != Issue
Idea != Issue
Issue = scoped work candidate after promotion
PR = implementation attempt against linked intent
```

Non-trivial PRs still require linked intent and must follow `CONTRIBUTING.md`
and `.github/PULL_REQUEST_TEMPLATE.md`.

Community Intake Flow does not weaken the PR Intake Gate.

## Desired user flow

```text
1. User or user agent wants to suggest something to Punk.
2. They submit it through local Punk, a bot command, a visor message,
   GitHub Discussion, or future inbox.
3. Punk normalizes it into a Community Intake Item.
4. Punk classifies the item.
5. Punk checks existing repo artifacts.
6. Punk produces an advisory route for whether it is likely duplicate,
   refinement, new candidate, bug/failure evidence, research/source, PR-intent,
   parked, avoid, or noise.
7. Punk displays a safe advisory summary in one or more Community Visors if
   allowed.
8. People and agents may discuss or attach more signals in the visors.
9. Those replies become additional advisory signals linked to the intake item.
10. Only maintainer / future gate promotion can create repo truth, GitHub
    issue, work goal, roadmap effect, or PR-readiness.
11. PRs require linked intent and follow normal repository rules.
```

## Where local Punk fits

Preferred long-term entry point:

```text
local Punk / user agent -> Punk-mediated intake
```

Not:

```text
random chat message -> work item
```

Visor-originated messages are still allowed.

Both paths converge into Community Intake Item:

```text
local Punk proposal
Telegram message
GitHub Discussion comment
mycel inbox message
agent contribution
  -> Community Intake Item
```

## Example: improvement request

Input:

```text
What about a tester module for Punk?
```

Expected route:

```yaml
kind: improvement / module request
likely_related:
  - docs/product/MODULE-HOST.md
  - future Dev module / eval plane
route:
  - research_candidate
  - parked
status:
  - not implementation-ready
display:
  - safe summary in Community Visor
ask_for:
  - concrete failures
  - testing gaps
  - existing tools/sources
  - contributor research interest
```

Do not say accepted.

Do not create an issue immediately unless there is maintainer approval and
scoped criteria.

## Example: bug/failure

Input:

```text
My agent opened a PR without reading work/STATUS.md and changed unrelated files.
```

Expected route:

```yaml
kind: failure
relation:
  evidence_for:
    - agent contribution policy
    - PR intake boundaries
route:
  - evidence_candidate
  - possible docs/eval/policy improvement
ask_for:
  - agent/tool used
  - expected behavior
  - what changed
  - what rule/check would have prevented it
  - quote consent
```

This may be stronger than an idea with many votes.

## Example: source

Input:

```text
This paper/tool/repo seems related to proof-bearing agent work: <safe link>
```

Expected route:

```yaml
kind: research/source
route:
  - research_candidate
  - related_artifact
ask_for:
  - primary source link
  - relevant mechanism
  - why it fits Punk laws
  - risks or limits
```

The source does not become product truth until it is captured in repo-tracked
research and promoted through the normal process.

## Example: question

Input:

```text
Does Punk already support live agent execution?
```

Expected route:

```yaml
kind: question
route:
  - existing_artifact_found
likely_related:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/ROADMAP.md
reply:
  - current CLI surface is intentionally small
  - autonomous agent execution is not active
```

Questions may reveal documentation gaps, but they are not work items by
default.

## Example: PR-intent

Input:

```text
My agent can implement this now.
```

Expected response:

```text
Non-trivial PRs require linked intent.
Valid intent refs:
- Issue / Discussion
- work/goals/...
- work/reports/...
- docs/adr/...
- knowledge/research/...
- evals/specs/...

This item is not PR-ready until maintainer marks PR welcome / docs welcome /
research welcome / eval welcome.
```

## Status vocabulary

Draft advisory intake statuses:

```text
new
classified
duplicate_or_refinement
needs_clarification
needs_evidence
research_candidate
parked
avoid
issue_candidate
issue_created
contributor_ready
declined
superseded
discarded_noise
```

These are advisory intake statuses.

They are not gate decisions.

## Relation vocabulary

Reuse the existing advisory vocabulary:

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

Duplicate handling:

- duplicates should link to an existing intake item, topic, or artifact;
- refinements should attach as additional context;
- duplicate detection is advisory;
- canonical merge or closure requires maintainer/future gate review.

## Visor behavior

Community Visors should show safe summaries and status updates.

Example visor card:

```text
New intake item: Tester module for Punk

Kind: improvement / module request
Route: research candidate
Status: not implementation-ready

Why:
- touches module boundary
- relates to eval/validation
- likely blocked by Module Host / Dev module phases

Looking for:
- concrete failures
- examples of agent testing gaps
- existing tools/sources
- contributor research interest

Intake ID: PUNK-INTAKE-0007
```

If promoted:

```text
Intake item promoted to scoped issue:
#123 Research tester module boundary

This is research, not implementation.
PRs welcome only for source collection / boundary analysis / eval ideas.
```

## Issue creation rule

Issue creation is promotion, not the first step.

Create or recommend creating an issue only when:

- problem is clear;
- duplicate/refinement check is done;
- phase fit or explicit defer/park reason is known;
- enough evidence or strategic need exists;
- scope can be bounded;
- acceptance criteria can be drafted;
- validation path exists;
- maintainer/future gate approves.

Automatic issue creation is out of scope.

## No-code / no-runtime boundary

This artifact adds no:

- code;
- CLI behavior;
- Telegram, Discord, GitHub, Slack, mycel, email, or forum adapter;
- bot;
- live send;
- Topic Graph implementation;
- CommunityPunk runtime;
- Module Host behavior;
- raw chat storage;
- issue/goal auto-creation;
- roadmap automation;
- PR Intake Gate behavior change;
- schema/template system;
- docs/product promotion;
- new source of truth.

## Adoption map

| Mechanism | Recommendation | Reason |
|---|---|---|
| Community Intake Flow | adopt as advisory architecture | Covers proposals, failures, sources, questions, PR intents, and noise |
| Community Intake Item | adopt as vocabulary | Normalizes many signal kinds without turning them into issues |
| Community Visor | adopt as vocabulary | Keeps Telegram/Discord/GitHub/web/mycel/email as surfaces, not truth |
| Automated Intake Responder | adopt as component boundary | Useful first-response automation inside visors |
| Proposal Flow as umbrella | avoid | Too narrow; hides failures, evidence, questions, and PR intent |
| GitHub Issue on first message | avoid | Issue creation is promotion and needs scoped criteria |
| Local Punk/user-agent intake | defer as preferred future entry | Strong fit, but no runtime implementation in this patch |
| Visor safe summaries | adopt as direction | Enables discussion without raw transcript truth |
| Topic Graph implementation | defer | Future derived/rebuildable router only |
| CommunityPunk runtime | park | Future module candidate, not active now |

## Open questions

- What generates stable intake IDs in an external prototype?
- Which visor should be first after Telegram: GitHub Discussions, web, mycel,
  or email?
- Should Mode A command/mention-gated intake be enough for v0.1, or does
  full-listener UX justify the privacy burden?
- Which statuses should survive after the first real signal window?
- What is the smallest safe promotion packet from intake item to repo artifact
  or GitHub Issue?
- How should local Punk/user-agent intake converge with visor-originated
  messages without creating hidden runtime memory?

## Review trigger

Review by 2026-06-05 or after the first deployed intake/responder prototype
receives real usage, whichever comes first.
