---
id: channel_x
name: "X / Twitter"
status: draft
owner: vitaly
created_at: 2026-04-18
updated_at: 2026-04-20
channel_type: social
external_url: null
---

# X / Twitter

## Purpose

Short public updates and threads.

## Audience

- builders who discover ideas through fast-moving public threads
- people following the build in real time
- adjacent researchers and toolmakers who respond to compact takes

## Shared style source

Use:

- `public/channels/_shared/style.toml`
- `public/channels/_shared/voice.md`
- `public/channels/_shared/normalize.toml`
- `public/channels/_shared/lexicon.toml`
- `public/channels/_shared/review-checklist.md`

## Format rules

- Prefer one idea per post.
- Use either one short standalone post or a 4-10 post thread.
- Make the first line carry the tension, claim, or lesson.
- Keep standalone posts compact; avoid padding just to sound dramatic.
- Use at most one link, usually in the final post of a thread.
- Do not use hashtag spray.
- If the post mentions the product state, make the experimental/early-stage status clear when omission could mislead.
- Avoid threadbait, hype language, and vague founder-posturing.

## Pre-publish checks

Before manual publication, run the shared review checklist and normalization rules.

## Manual publication steps

1. Finalize the channel variant in `public/posts/`.
2. Apply shared normalization and review checks.
3. Publish manually.
4. Record publication receipt in `public/publications/`.
5. Add metrics snapshot manually when useful.

## Metrics to collect

- impressions
- likes
- replies
- reposts
- follows
