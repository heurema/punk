# Public Narrative Plane

## Purpose

`punk` is a public build from day zero.

The project needs repo-tracked public narrative artifacts before PubPunk automation exists.

Stories, post drafts, channel notes, manual publication receipts, and manual metrics snapshots should be saved in a structure that PubPunk can later adopt without migration.

## Principle

Manual now. Structured now. Automated later.

Do not wait for PubPunk runtime to begin preserving public artifacts.

## Relationship to PubPunk

PubPunk is the future content-engineering module.

It will eventually:

- plan content
- draft content
- enforce voice/style rules
- prepare channel-specific variants
- record publication receipts
- collect metrics
- suggest follow-up content
- connect public narrative to project goals and knowledge

For now, the repo owns the content structure and humans update it manually.

## Storage

Repo-tracked public artifacts live under:

```text
public/
  narrative/
  stories/
  posts/
  channels/
  publications/
  metrics/
  assets/
  _templates/
  _schema/
```

Runtime/derived PubPunk indexes may later live under:

```text
.punk/indexes/pubpunk.*
.punk/views/public-narrative.*
```

## Canonical objects

### Story

A durable narrative arc.

Examples:

- why `punk` exists
- lessons from SpecPunk
- why core-first matters
- why public build matters

### Post

A concrete content draft or final post.

A post may target many channels.

### Channel

A public distribution surface.

Examples:

- blog
- X/Twitter
- Telegram
- LinkedIn
- newsletter
- community/forum

### Publication receipt

A record that a post was published manually or automatically.

It should include:

- channel
- URL or external ID
- published_at
- actor
- post_ref
- notes

### Metrics snapshot

A manual or automated record of content performance.

For now metrics are manual only.

It should include:

- channel
- publication_ref
- collected_at
- impressions/views
- likes/reactions
- comments/replies
- shares/reposts
- follows/subscriptions
- notes

## Rules

1. Public narrative is project memory.
2. Public posts should link to goals, knowledge, ADRs, or releases when possible.
3. Manual publication is allowed.
4. Manual publication must still leave a publication receipt.
5. Metrics are manual for now.
6. Metrics can be empty at first, but the place for metrics exists from day zero.
7. PubPunk must adopt this structure instead of creating a separate content truth.
8. Raw drafts do not become published truth until a receipt exists.
9. Public content must not leak private/internal-only material.
10. Public claims should cite canonical project knowledge or be marked as opinion.
11. Public narrative changes can be dogfooded before code exists.

## First artifacts

Before implementation starts, create placeholders:

```text
public/stories/story-0001-origin.md
public/posts/0001-first-public-build-note.md
```

These are templates/placeholders, not final copy.

If a post is published manually, add a receipt:

```text
public/publications/YYYY-MM-DD-0001-<channel>.md
```

## What stays out of scope now

- no automated publishing
- no social network API integration
- no metrics scraping
- no content scheduler
- no multi-channel optimizer
- no growth automation

Those belong to future PubPunk phases.
