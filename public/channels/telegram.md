---
id: channel_telegram
name: "Telegram"
status: draft
owner: vitaly
created_at: 2026-04-18
updated_at: 2026-04-20
channel_type: social
external_url: null
---

# Telegram

## Purpose

Short updates and community notes.

## Audience

- people following the build closely
- builders who want concise notes with direct value
- early readers who want context without a full essay

## Shared style source

Use:

- `public/channels/_shared/style.toml`
- `public/channels/_shared/voice.md`
- `public/channels/_shared/normalize.toml`
- `public/channels/_shared/lexicon.toml`
- `public/channels/_shared/review-checklist.md`

## Format rules

- Open with a concrete update, lesson, or value sentence.
- Prefer 400-1200 characters for ordinary posts.
- Be slightly more direct than the blog, but keep claims concrete.
- Avoid a separate title line by default.
- Use at most one link.
- Use short lists only when they improve scanning.
- Make current status and scope explicit when talking about the product.
- Avoid hype, vague teaser language, and automation claims that the repo does not support yet.

## Pre-publish checks

Before manual publication, run the shared review checklist and normalization rules.

## Manual publication steps

1. Finalize the channel variant in `public/posts/`.
2. Apply shared normalization and review checks.
3. Publish manually.
4. Record publication receipt in `public/publications/`.
5. Add metrics snapshot manually when useful.

## Metrics to collect

- views
- reactions
- comments
- shares
- subscribers
