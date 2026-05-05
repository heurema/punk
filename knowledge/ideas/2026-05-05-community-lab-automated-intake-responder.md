---
id: idea_2026_05_05_community_lab_automated_intake_responder
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
  - public-narrative
  - agents
related_research:
  - knowledge/research/2026-05-05-ai-agent-communication-landscape-notes.md
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
  - knowledge/ideas/2026-05-05-community-intake-flow.md
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/MODULE-HOST.md
  - docs/product/MODULES.md
  - CONTRIBUTING.md
  - .github/PULL_REQUEST_TEMPLATE.md
supersedes: []
superseded_by: null
---

# Community Lab Automated Intake Responder

## Status

This is an advisory idea and boundary artifact.

It modifies the operational expectation captured in
`knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md`:

```text
Manual answering is not the desired default UX.
Manual digest remains a review and aggregation path.
First-message handling should be automated by a bounded intake responder once
policy, receipts, and privacy disclosure are defined.
```

This artifact does not add code, CLI behavior, Telegram integration, adapter
runtime, Topic Graph implementation, CommunityPunk runtime, raw chat storage, or
project-truth promotion.

Architecture correction:

```text
The Automated Intake Responder is a Community Visor automation component.
It is not the center of the architecture.
The center is Community Intake Flow.
```

Related intake-flow boundary:

- `knowledge/ideas/2026-05-05-community-intake-flow.md`

## Formula

```text
Automatic first response,
human / future gate for promotion.
```

Automate:

- classification;
- redaction / safety precheck;
- repo artifact lookup;
- likely duplicate / related-topic assessment;
- policy check;
- bounded first reply;
- minimal private receipt.

Do not automate:

- promotion into project truth;
- issue creation;
- goal creation;
- roadmap priority;
- PR acceptance;
- final decisions;
- gate/proof outcomes.

## Responder flow

```text
Community Intake Item or allowed visor message
  -> responder receives allowed update inside a Community Visor
  -> redaction / safety precheck
  -> classify kind
  -> search repo artifacts
  -> detect likely existing topics / ideas / rules
  -> decide response type
  -> send immediate bounded reply
  -> write minimal private receipt / intake link
  -> optionally include redacted summary in future digest
```

Not:

```text
message
  -> bot creates issue
  -> bot edits roadmap
  -> bot says accepted
  -> bot asks agent to PR
```

## Boundary

The responder is an intake assistant.

It may assess, suggest, link, challenge, ask for clarification, and route to
existing artifacts.

It may not decide, accept, merge, promote truth, create roadmap priority,
create issues/goals automatically, approve PRs, claim gate/proof status, or
declare canonical truth.

## Relationship to PR #35

PR #35 safely captured the first Community Lab frame as advisory/manual/no-bot.

This artifact does not invalidate that boundary.

It refines the operational path:

- manual digest remains useful for quality control and periodic review;
- manual first response is not acceptable as the default user experience;
- a bounded automated responder should handle first-message routing once policy,
  receipts, and privacy disclosure exist.

## Responder modes

### Mode A: command / mention gated

The responder sees only messages explicitly addressed to it:

```text
/idea ...
/failure ...
/source ...
/question ...
/pr ...
/agent ...
@PunkLabBot ...
reply to bot message
```

Default recommendation for v0.1.

Why:

- privacy-safe;
- low noise;
- matches Telegram bot privacy defaults;
- easier to explain;
- faster to launch.

Tradeoff:

- users must learn commands, mentions, or reply behavior;
- ordinary unmarked messages may not get automatic response.

### Mode B: full listener

The responder receives all new group messages.

Allowed only after explicit pin/policy disclosure and storage policy review.

Why:

- best first-user UX;
- no command memory burden.

Tradeoff:

- higher privacy burden;
- stronger anti-loop and anti-spam requirements;
- clearer "bot may read new messages" disclosure is mandatory;
- greater risk that chat is mistaken for system-owned memory.

## Recommendation

Start with Mode A for 1-2 weeks.

Design the external bot prototype so Mode B can be enabled later by explicit
configuration after pin, privacy, rate-limit, and storage policy checks.

If seamless UX is selected over privacy-safe startup, Mode B may be selected
first, but only with explicit group disclosure:

```text
The bot may read all new messages in this group.
It replies with advisory routing only.
Raw chat is not project truth and is not stored as project truth.
```

## Signal taxonomy

Use the existing Community-Signaled taxonomy:

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

Do not over-model the taxonomy in code until real usage shows which classes are
stable.

## Curated repo search paths

The responder should search only curated project-truth and advisory surfaces:

```text
README.md
docs/product/
docs/adr/
knowledge/ideas/
knowledge/research/
work/goals/
work/reports/
publishing/
.github/PULL_REQUEST_TEMPLATE.md
CONTRIBUTING.md
```

Do not search hidden runtime state or raw chat logs.

Initial search should be simple:

```text
frontmatter
  -> headings
  -> lexical search
  -> fuzzy title / alias search
```

Embeddings and Topic Graph indexes remain deferred.

## Assessment labels

Responder output is assessment, not decision.

Allowed assessment labels:

```text
likely_existing_topic
possible_duplicate
possible_refinement
related_artifact
no_close_match
violates_boundary
needs_clarification
PR_requires_linked_intent
```

The responder must not produce:

```text
accepted
roadmap_priority_set
issue_created
goal_created
PR_welcome
gate_passed
truth_updated
```

## Response modes

The responder should always choose one bounded response mode:

```text
existing_artifact_found
possible_duplicate_or_refinement
needs_clarification
new_signal_candidate
failure_use_case_follow_up
source_research_follow_up
PR_intent_linked_intent_required
boundary_violation_or_unsafe_content
noise_or_cannot_classify
```

## Reply rules

Replies should be short and immediate.

They should:

- name the classification;
- show at most a few related repo refs;
- state status such as advisory, parked, future, or active-core target only when
  a repo artifact explicitly says so;
- remind that chat is raw signal and repo artifacts remain truth when needed;
- ask for one useful next piece of evidence;
- avoid pretending the responder has accepted work.

Example existing-topic reply:

```text
Classified as: idea.
Likely related artifact:
- knowledge/ideas/2026-05-05-community-driven-development-with-agents.md

Status: advisory, not active runtime.
Route: possible refinement / future responder boundary.

Chat is raw signal; repo artifacts remain truth.
Can you add a concrete failure or use case?
```

Example PR-intent reply:

```text
Classified as: PR-intent.

For non-trivial PRs, Punk requires linked intent:
- Issue / Discussion
- work/goals/...
- work/reports/...
- docs/adr/...
- knowledge/research/...
- evals/specs/...

Direct PR without linked intent is likely to fail intake.
```

Example failure reply:

```text
Classified as: failure.

Useful failure signal. Please add:
1. tool/agent used;
2. expected behavior;
3. what happened;
4. safe repo/artifact/source link if available;
5. "ok to quote" if public quote is allowed.

Do not post secrets, private logs, raw prompts, or customer data.
```

Example unknown reply:

```text
I can route this if you mark it as one of:
/idea /failure /source /question /pr /agent

Current mode: advisory intake only.
Chat is not project truth.
```

## Forbidden claims

The responder must never say:

- accepted;
- approved;
- roadmap priority set;
- issue created;
- goal created;
- PR welcome;
- gate passed;
- proof accepted;
- truth updated;
- maintainer agreed;
- implementation will happen.

It may only say:

- classified as;
- possible match;
- possible duplicate;
- related artifact;
- needs clarification;
- linked intent required;
- candidate signal;
- advisory route.

## Receipt policy

Every automated reply should write a minimal private receipt.

Initial receipt shape:

```yaml
receipt_kind: community_intake_reply
surface: telegram
chat_id_hash: string
message_id: string
thread_id: string | null
observed_at: string
raw_stored: false
classification: idea | research/source | bug | failure | question | PR-intent | agent-contribution | noise
matched_refs:
  - repo_relative_path
reply_kind: existing_artifact_found | possible_duplicate_or_refinement | needs_clarification | new_signal_candidate | failure_use_case_follow_up | source_research_follow_up | PR_intent_linked_intent_required | boundary_violation_or_unsafe_content | noise_or_cannot_classify
reply_message_id: string | null
model_used: string | null
status: replied | skipped | blocked
```

Receipts may live in private local storage for the external bot prototype.

They are not project truth.

A later digest may manually promote redacted summaries into repo artifacts.

## Storage policy

Default:

- no raw transcript archive;
- no raw prompts;
- no secrets;
- no customer data;
- no private logs;
- no `.punk/` runtime writes from the external bot;
- no repo writes from the external bot;
- no automatic issue/goal creation.

Allowed:

- hashed chat id;
- Telegram message id;
- thread id;
- timestamp;
- classification;
- matched repo refs;
- reply kind;
- reply message id;
- model/provider metadata if safe;
- redacted summary only when digesting.

## Policy pack inputs

The external bot should load a Responder Policy Pack from repo-tracked
artifacts.

Minimum policy pack content:

- allowed inputs;
- signal taxonomy;
- curated search paths;
- assessment labels;
- response modes and templates;
- forbidden claims;
- confidence thresholds;
- privacy rules;
- receipt fields;
- rate limits;
- escalation rules;
- when to stay silent;
- when to ask clarification;
- when to warn that PR needs linked intent.

This idea artifact is the initial policy pack candidate.

Do not create `knowledge/_templates/*` until a later bounded goal selects a
template system.

## Rate limits and loop prevention

Required before any live bot:

- deduplicate repeated message ids;
- never answer its own messages;
- cap replies per chat/thread/time window;
- cap bot-to-bot depth;
- suppress repeated low-confidence replies;
- require explicit command/mention/reply for Mode A;
- require stricter spam controls for Mode B.

## Pin update requirement

If a bot is added, the group pin must change before launch.

Mode A disclosure:

```text
Current mode:
automated advisory intake responder.
The bot responds to commands, mentions, and replies.
It classifies signals, checks repo artifacts, and replies with advisory routing.
It does not create issues, goals, PRs, decisions, or project truth.
Raw chat logs are not stored as project truth.
```

Mode B disclosure:

```text
Current mode:
automated advisory intake responder.
The bot may read all new messages in this group.
It classifies signals, checks repo artifacts, and replies with advisory routing.
It does not create issues, goals, PRs, decisions, or project truth.
Raw chat logs are not stored as project truth.
```

## External implementation boundary

Do not implement the responder in the Punk core repository now.

Preferred placement:

```text
heurema/punk-community-lab-bot
```

or a private deployment/ops repository:

```text
punk-lab-responder
```

Ownership split:

```text
punk repo:
  owns policy, research, ideas, and work goals

bot repo:
  owns Telegram integration, deployment, tokens, and runtime logs

Telegram group:
  owns raw conversation surface
```

The bot should use `heurema/punk` as a read-only source of repo artifacts.

## Minimal external architecture

```text
Telegram Bot API
  -> update handler
  -> redactor
  -> classifier
  -> repo index search
  -> policy checker
  -> reply composer
  -> sendMessage
  -> receipt writer
```

The bot may use `message_thread_id` for Telegram forum topics when sending
replies.

## Suggested commands

Even in Mode B, commands are useful:

```text
/idea
/failure
/source
/question
/pr
/agent
/status
/help
```

## Phased plan

### Phase 0: policy PR

- Automated Intake Responder boundary.
- Responder Policy Pack.
- Mode A / Mode B decision.
- Pin update draft.
- Receipt fields.
- Allowed replies.
- Forbidden claims.

No code.

### Phase 1: external bot prototype

- Telegram Bot API.
- Command/mention-gated mode first.
- Local clone/index of `heurema/punk`.
- Search curated artifact paths.
- Classifier plus deterministic guardrails.
- Reply templates.
- Private receipt log.

Not in Punk core.

### Phase 2: full-listener decision

- Only after pin update and storage policy are explicit.
- Privacy disabled / admin mode only if selected.
- Rate limits.
- Anti-loop.
- Anti-spam.
- No raw archive.

### Phase 3: auto digest

- Generate weekly digest draft.
- Maintainer reviews before repo PR.
- Digest is audit/review, not first-response queue.

## Adoption map

| Mechanism | Recommendation | Reason |
|---|---|---|
| Automated first response | adopt as direction | Manual answering does not meet first-user response expectations |
| Mode A command/mention gated | adopt for v0.1 | Lower privacy burden and easier launch |
| Mode B full listener | defer / explicit opt-in | Better UX, higher privacy and moderation burden |
| Private reply receipts | adopt as boundary | Needed for audit without raw transcript archive |
| Curated repo search | adopt | Keeps answers grounded in repo artifacts |
| External bot repo | adopt | Avoids Punk core adapter/runtime drift |
| Bot-created issues/goals | avoid | Violates promotion and source-of-truth boundaries |
| Bot-owned roadmap priority | avoid | Violates Punk decision boundaries |
| Raw transcript archive | avoid | High privacy and false-memory risk |

## Review trigger

Review by 2026-06-05 or after the first deployed responder prototype receives
real usage, whichever comes first.
