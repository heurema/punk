---
id: research_2026_05_09_community_driven_development_governance
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-05-09
updated_at: 2026-05-09
review_after: 2026-06-09
research_level: R3
components:
  - project-memory
  - community
  - contribution-governance
  - agents
  - research-intake
  - public-narrative
related_ideas:
  - knowledge/ideas/2026-05-05-community-driven-development-with-agents.md
  - knowledge/ideas/2026-05-05-channel-agnostic-community-lab.md
  - knowledge/ideas/2026-05-05-community-intake-flow.md
  - knowledge/ideas/2026-05-05-community-lab-automated-intake-responder.md
related_docs:
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/PUBLIC-NARRATIVE.md
  - CONTRIBUTING.md
source_basis:
  - "user-provided deep research synthesis, 2026-05-09"
source_refs:
  - "Debian"
  - "Apache"
  - "Python"
  - "Blender"
  - "OTW/AO3"
  - "SCP Wiki"
  - "Nouns"
  - "ENS"
  - "Optimism"
  - "Gitcoin"
  - "Open Collective"
  - "GitHub"
  - "Discourse"
  - "Matrix"
  - "Discord"
  - "Telegram"
  - "Loomio"
  - "Forgejo"
  - "commons-governance literature"
confidence: medium
supersedes: []
superseded_by: null
---

# Community-driven development governance for Punk

## Research status and authority

This is advisory R3 research and project memory.

It does not promote community governance, CommunityPunk, Topic Graph,
microgrants, DAO, token voting, public budget mechanics, runtime intake,
adapters, bots, live community sends, GitHub Discussions, Telegram, Discord, or
any other community surface into active Punk behavior.

Research remains advisory until promoted through the normal Punk path:

```text
research note
  -> ADR / roadmap decision
  -> goal / contract
  -> implementation
  -> eval
  -> gate / proof
  -> project-memory update
```

## Question

How should Punk start community-driven development safely, using community and
user-agent input without letting chat, votes, agents, funding mechanics, or
external platforms own project truth?

## Decision context

Punk is an early-stage local-first bounded work kernel.

The active posture is core-first, project-memory-first, and gate/proof-first.
The current executable CLI surface is intentionally small. Future modules,
adapters, PubPunk automation, CommunityPunk, Topic Graph, and external
community surfaces remain inactive unless separately promoted.

Relevant Punk boundaries:

- only `gate` writes final decisions;
- modules may assess, not decide;
- adapters may invoke, not own truth;
- project memory is explicit and authority-tagged;
- raw ideas are not implementation truth by default;
- research is advisory until promoted through ADR, roadmap, goal/contract,
  implementation, eval, and proof;
- public/community artifacts must not silently become project truth.

## Source basis and validation caveat

The source base is a user-provided deep research synthesis referencing Debian,
Apache, Python, Blender, OTW/AO3, SCP Wiki, Nouns, ENS, Optimism, Gitcoin,
Open Collective, GitHub, Discourse, Matrix, Discord, Telegram, Loomio, Forgejo,
and commons-governance literature.

Exact URLs are not recorded in this note.

Source URLs require revalidation before ADR, implementation, legal, funding, or
governance promotion.

This note extracts governance patterns and boundaries. It does not claim that
the referenced projects endorse Punk's model, and it does not import their
governance systems wholesale.

## Existing systems / prior art patterns

### Mature open-source projects

Debian, Apache, Python, and Blender point toward durable maintainer authority,
reviewable contribution paths, clear project norms, issue/PR workflows, release
discipline, and transparent but bounded community participation.

Pattern for Punk:

- community participation is useful signal;
- trusted maintainers preserve project direction;
- contribution access should be earned through evidence and bounded work;
- popularity does not automatically decide roadmap or acceptance.

### Volunteer and cultural commons communities

OTW/AO3 and SCP Wiki point toward explicit norms, moderation, role boundaries,
curation, and policies for content, attribution, safety, and conflict.

Pattern for Punk:

- a community surface needs rules before growth;
- raw contributions need review and curation;
- archival impulse must not become raw transcript storage;
- public participation needs moderation boundaries.

### DAO, token, and grants ecosystems

Nouns, ENS, Optimism, Gitcoin, Open Collective, and adjacent funding/governance
patterns show useful mechanics for proposals, public budgets, funding rounds,
grants, and accountability, but also expose risks around plutocracy, popularity
games, governance overhead, capture, and premature financialization.

Pattern for Punk:

- proposal transparency can be useful later;
- microgrants and budgets require real budget, accounting, fraud controls, and
  decision policy;
- token or DAO mechanics should not be the first community primitive.

### Platform and discussion tools

GitHub, Discourse, Matrix, Discord, Telegram, Loomio, and Forgejo are surfaces
for discussion, moderation, collaboration, or code contribution.

Pattern for Punk:

- tools are visors or transport surfaces;
- none of them should own project truth;
- platform-native votes/reactions are weak signals;
- issue creation is promotion, not raw intake;
- PRs should link accepted intent for non-trivial work.

### Commons-governance literature

Commons-governance patterns emphasize boundary clarity, local fit, visible
rules, monitoring, conflict handling, graduated participation, and mechanisms
for changing rules without collapsing authority.

Pattern for Punk:

- start with explicit boundaries;
- keep authority and evidence visible;
- avoid importing heavy governance before community evidence exists;
- adapt rules after observed failures.

## Extracted principles for Punk

- Community input is signal, not authority.
- User-controlled AI agents may participate as advisory contributors.
- Evidence quality outranks vote count, reaction count, or agent volume.
- Maintainers retain roadmap priority, project truth, final acceptance, and
  merge authority.
- Non-trivial PRs require linked intent.
- Issue creation is promotion, not the first response to a message.
- Community surfaces are raw signal surfaces or Community Visors, not truth.
- Automated responders may classify, suggest, link, and ask, but may not create
  project truth.
- Raw transcripts, private logs, secrets, raw prompts, customer data, and
  unredacted community logs must not become project memory.
- DAO, token, treasury, and grant mechanics are governance programs, not
  onboarding shortcuts.
- Manual digest cycles should precede automation.

## Failure modes found

- Vote-driven roadmap creates popularity bias and weakens maintainer strategy.
- Raw chat becomes de facto project truth.
- Raw transcript archive creates privacy and false-memory risk.
- Bot-created issues or goals inflate backlog and bypass promotion.
- Bot-owned priority creates hidden governance.
- Agents reinforce each other's low-evidence claims.
- Non-trivial agent PRs arrive without linked intent.
- Topic Graph or embedding matches become canonical topic authority.
- Microgrants or bounties create incentives before scope and review are ready.
- DAO/token governance shifts attention from evidence to financial or voting
  mechanics.
- Community surfaces create moderation and reputation burden before the core is
  stable.
- Funding or budget artifacts create legal/accounting obligations before real
  budget exists.

## Options considered

| Option | Description | Recommendation | Reason |
|---|---|---|---|
| Community-Signaled, Evidence-Gated Development | Community provides signal; maintainer review, linked intent, evals, and future gate/proof preserve truth | adopt | Matches Punk laws and current project-memory posture |
| Manual Community Lab digest cycle | Curated periodic digest from first chosen surface | adopt as next safe step | Produces evidence without runtime or governance drift |
| Evidence-weighted support levels | Support increases with failures, sources, reproducible artifacts, and bounded contribution intent | adopt | Reduces vote, sybil, and popularity bias |
| Contributor-ready lane | Maintainer-reviewed lane for scoped external or agent work | adopt | Lets agents help without unbounded PR chaos |
| Automated Intake Responder | Bot-like first-response classifier and router | defer | Useful after policy, receipts, privacy, and manual digest evidence |
| Topic Graph | Derived/rebuildable relation router over repo artifacts and intake summaries | defer | Needs real signals and eval boundaries first |
| GitHub Discussions / Telegram / Discord expansion | Additional visors/surfaces | defer | Start with one chosen visor and manual digest |
| Microgrants / bounties | Funding support for contributor work | defer | Requires budget, accounting, anti-abuse, and review policy |
| Formal maintainer council | Multi-maintainer governance body | defer | Useful only when contributor scale justifies it |
| CommunityPunk runtime module | Punk module for community intake/triage | park | Module runtime is not active and community evidence is not ready |
| Live adapters / bot sends | Runtime integration with external platforms | park | Side-effect, privacy, moderation, and reputation risk |
| DAO/token governance | Tokenized or DAO-first decision model | avoid / park as non-core research | Conflicts with evidence-gated maintainer authority |
| Vote-driven roadmap | Community voting controls priority | avoid | Conflicts with Punk truth and roadmap boundaries |

## Recommendation

Punk should implement community development as:

```text
Community-Signaled, Evidence-Gated Development
```

Meaning:

Community members and user-controlled AI agents may provide signals, ideas,
failures, sources, duplicate/refinement suggestions, questions, PR-intent, and
bounded contribution attempts.

They do not own roadmap priority, project truth, final decisions, acceptance,
merge authority, or canonical topic status.

The model is:

```text
community signal
  -> community intake item
  -> classification
  -> evidence / duplicate / law / phase assessment
  -> maintainer review
  -> linked intent
  -> research / idea / goal / issue / PR / publishing / discard
  -> eval / future gate / proof when applicable
```

The next safe operational step is the first manual Community Lab digest cycle.

That step should be manual, bounded, and inspectable. It should summarize a
small signal window, classify signals, identify duplicates/refinements, record
evidence quality, and route only maintainer-approved outputs into repo
artifacts.

## Adoption map: adopt / defer / park / avoid

### Adopt

- Community-Signaled, Evidence-Gated Development as framing.
- Manual Community Lab / Community Visor as raw signal surface.
- Evidence-weighted support levels instead of raw voting.
- Contributor-ready lane after maintainer review.
- Manual digest cycles before automation.
- Linked intent requirement for non-trivial PRs.
- Agent policy: agents may assess/suggest/link/implement bounded linked
  intent, but may not decide/accept/merge/own truth.

### Defer

- Automated Intake Responder.
- Topic Graph.
- GitHub Discussions / Telegram / Discord expansion beyond first chosen visor.
- Microgrants / bounties.
- Formal maintainer council.
- Public budget / treasury mechanics.
- Structured role ladder.

### Park

- CommunityPunk runtime module.
- Live adapters.
- Bot live sends.
- DAO/token governance.
- Open Collective / treasury governance until real budget exists.
- Local Punk/user-agent intake runtime.

### Avoid

- Vote-driven roadmap.
- Raw chat as source of truth.
- Raw transcript archive.
- Bot-created issues/goals.
- Bot-owned priority.
- Automatic PR acceptance.
- Agent democracy.
- Token-first community.
- DAO-first governance.
- Public agent feed as project truth.

## Impact on roadmap

No roadmap document changes are made by this note.

The recommendation aligns with existing future-phase boundaries:

- Project Memory: community signals may become advisory research, ideas, goals,
  reports, or later issues only through explicit promotion.
- Module Host / CommunityPunk: remains future/parked.
- PubPunk / public narrative: no publication surface changes.
- Adapters: external community transports remain parked until side-effect,
  privacy, receipt, and eval policy are promoted.
- Gate/proof: final acceptance still requires future gate/proof where
  applicable.

This note does not change active-core scope.

## Required evals

No eval spec is added in this patch.

Future implementation or promotion should add evals for:

- community signal cannot become project truth without linked artifact;
- raw chat and raw transcripts are excluded from project memory;
- automated responder cannot create issues, goals, priority, acceptance, or
  canonical truth;
- PR-intent for non-trivial work requires linked intent;
- evidence-weighted support cannot be reduced to raw vote count;
- Topic Graph output remains derived, rebuildable, advisory, and
  maintainer-reviewed before authority-sensitive relations;
- agents may assess/suggest/link/implement bounded linked intent but cannot
  decide, accept, merge, or own truth.

## Required docs / ADRs / contracts

Current patch requires only:

- this advisory research note;
- a work goal;
- a work report;
- a minimal `work/STATUS.md` side-track note;
- optional related-research links from existing advisory idea artifacts.

Future promotion would require:

- an ADR or roadmap decision if governance, roles, budgets, or runtime
  authority changes;
- a bounded goal/contract for first manual digest if selected;
- receipt/privacy policy before automation;
- side-effect policy before live adapters or bot sends;
- evals before runtime/module/adapter promotion;
- legal/accounting review before funding, treasury, grants, or DAO mechanics.

## What stays out of scope

- Product behavior.
- Runtime behavior.
- CLI behavior.
- Rust code.
- `.punk/` runtime or derived writes.
- CommunityPunk runtime.
- Topic Graph implementation.
- Bot/adapters/integrations.
- Telegram, Discord, GitHub Discussions, Matrix, Discourse, Forgejo, Loomio, or
  other public surface launch.
- Raw chat storage.
- Raw transcript archive.
- Bot-created issues or goals.
- Issue/goal auto-creation.
- Automatic PR acceptance.
- Public narrative publication.
- Microgrants, bounties, budget, treasury, DAO, or token mechanics.
- Product-doc promotion.
- ADR as final architecture decision.

## Open questions

- Which first Community Visor should be used for the first manual digest cycle?
- What is the smallest safe digest template for 5-10 signals?
- Who owns the first digest and review cadence?
- What support levels survive real use after the first signal window?
- What makes a signal contributor-ready rather than only research-ready?
- When should GitHub Discussions become a structured public surface?
- What privacy and quote-consent rules are required before public summaries?
- What evidence threshold justifies a future Automated Intake Responder?
- What budget threshold would justify any microgrant or Open Collective work?
- What governance scale would justify a formal maintainer council?
