---
id: public_channel_shared_review_checklist
name: "Punk shared pre-publish review checklist"
status: draft
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
---

# Shared pre-publish review checklist

Use this before every manual publication.

## 1. Deterministic cleanup

- [ ] Text is normalized to NFC.
- [ ] No long em dash remains; use `-`.
- [ ] No ellipsis character remains; use `...`.
- [ ] No `ё` / `Ё` remains if the current project rule forbids it.
- [ ] No zero-width or NBSP junk remains.
- [ ] No trailing whitespace remains.

## 2. Artifact and formatting check

- [ ] No obvious copy artifacts from chats, notes, or prompts.
- [ ] No duplicated headings or broken bullets.
- [ ] No raw internal TODO markers remain.
- [ ] Links render correctly for the target channel.
- [ ] The final text fits the target format and length.

## 3. Claim and scope check

- [ ] The text does not imply `punk` is production-ready.
- [ ] The text does not imply PubPunk automation already exists inside `punk`.
- [ ] Parked or future surfaces are not described as current operator paths.
- [ ] Facts are backed by canonical docs, Journal artifacts, ADRs, or repo files.
- [ ] Opinions are framed as opinions.

## 4. Voice and quality check

- [ ] The tone stays clear, pragmatic, calm, and evidence-first.
- [ ] The post explains the tradeoff, not just the headline claim.
- [ ] The text says what changed, what matters, and what stays out of scope.
- [ ] No hype-first or founder-brand filler remains.

## 5. Channel fit check

- [ ] Channel-specific rules from `public/channels/*.md` were applied.
- [ ] Shared style source from `public/channels/_shared/` was applied.
- [ ] Blog: title and long-form structure are clear.
- [ ] X: one idea per post or thread; no threadbait.
- [ ] Telegram: direct value sentence first; no title line by default.
- [ ] LinkedIn: professional tone; no generic personal-brand copy.

## 6. Publication discipline

- [ ] Final variant is saved in `public/posts/`.
- [ ] After publishing, create a receipt in `public/publications/`.
- [ ] If metrics are collected later, save a manual snapshot in `public/metrics/`.
