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

- researchers following the origin and architecture of Punk
- developers and experimenters evaluating the design tradeoffs
- technical founders interested in bounded AI work and local-first tooling

## Shared style source

Inherit the shared voice and policy from:

- `public/channels/_shared/style.toml`
- `public/channels/_shared/voice.md`

## Format rules

- use explicit titles, not clickbait
- prefer 600-1400 words for standard notes
- keep sections visible: hook, context, what changed, tradeoffs, current status, CTA
- use exact dates, phases, or source refs when describing history
- distinguish active-core from parked or future-only surfaces
- prefer 1-3 links, usually to repo, docs, Journal, or public artifacts
- end with one clear next action, question, or pointer

## Manual publication steps

1. Finalize the channel variant in `public/posts/`.
2. Publish manually.
3. Record publication receipt in `public/publications/`.
4. Add metrics snapshot manually when useful.

## Metrics to collect

- views
- comments
- subscriptions
