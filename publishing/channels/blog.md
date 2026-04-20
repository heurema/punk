---
id: channel_blog
name: "Blog"
status: draft
owner: vitaly
created_at: 2026-04-18
updated_at: 2026-04-20
channel_type: blog
external_url: null
---

# Blog

## Purpose

Long-form canonical public notes.

## Audience

- researchers and experimenters following the Journal
- technical builders evaluating the ideas behind `punk`
- founders/operators interested in bounded AI work and project memory

## Shared style source

Use:

- `publishing/channels/_shared/style.toml`
- `publishing/channels/_shared/voice.md`
- `publishing/channels/_shared/normalize.toml`
- `publishing/channels/_shared/lexicon.toml`
- `publishing/channels/_shared/review-checklist.md`

## Format rules

- Prefer 600-1400 words for ordinary notes; go longer only when extra detail is useful.
- Start with a concrete observation or tension, not a slogan.
- Use a clear title; no clickbait.
- Cover: what changed, why it matters, tradeoffs, current status, and what stays out of scope.
- Distinguish active-core from parked or future surfaces.
- Use 1-3 links max; prefer repo, Journal, ADR, or canonical docs.
- End with one explicit next action, question, or invitation to follow the build.

## Pre-publish checks

Before manual publication, run the shared review checklist and normalization rules.

## Manual publication steps

1. Finalize the channel variant in `publishing/posts/`.
2. Apply shared normalization and review checks.
3. Publish manually.
4. Record publication receipt in `publishing/publications/`.
5. Add metrics snapshot manually when useful.

## Metrics to collect

- views
- comments
- subscriptions
