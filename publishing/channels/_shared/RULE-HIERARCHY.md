---
id: public_channel_shared_rule_hierarchy
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-20
updated_at: 2026-04-20
---

# Rule hierarchy for manual public publishing

## Purpose

This document clarifies that the current public-writing rules in `punk` are `punk` rules.

They are not a direct import of `pubpunk` as-is.

`pubpunk` may inform future design and provide useful ideas, but it is not the active source of truth for `punk` public narrative rules.

## Current principle

Use the `punk` rule hierarchy first.

External systems or adjacent projects may inform lower-level design, but they do not override `punk` laws, project boundaries, or status vocabulary.

## Current manual precedence

For the current manual public narrative workflow, apply rules in this order:

```text
Punk Core Laws
  -> Punk project docs and public-narrative rules
    -> shared manual publication rules
      -> channel-specific rules
        -> post-specific variant instructions
```

## Mapping

### 1. Punk Core Laws

Canonical examples:

- `docs/product/PUNK-LAWS.md`
- `docs/product/START-HERE.md`
- `docs/product/ARCHITECTURE.md`

These define the top-level boundaries:

- no hidden second source of truth
- public artifacts are project memory
- manual publication is allowed
- parked/future surfaces must not be described as active
- public claims must be honest about status and scope

### 2. Punk project docs and public-narrative rules

Canonical examples:

- `docs/product/PUBLIC-NARRATIVE.md`
- `docs/product/JOURNAL.md`
- `docs/product/PROJECT-MEMORY.md`
- `publishing/README.md`

These define what the public surface is for `punk`.

### 3. Shared manual publication rules

Current shared files:

- `publishing/channels/_shared/style.toml`
- `publishing/channels/_shared/voice.md`
- `publishing/channels/_shared/normalize.toml`
- `publishing/channels/_shared/lexicon.toml`
- `publishing/channels/_shared/review-checklist.md`

These are project-local manual rules for the current phase.

They are not a separate product contract.

### 4. Channel-specific rules

Current files:

- `publishing/channels/blog.md`
- `publishing/channels/x.md`
- `publishing/channels/telegram.md`
- `publishing/channels/linkedin.md`

These narrow the shared manual rules for each channel.

### 5. Post-specific variant instructions

Current surface:

- `publishing/posts/*.md`

These are the most specific instructions for an individual draft or channel variant.

## Important clarification

`pubpunk` is not being adopted one-to-one here.

What may be reused later:

- useful ideas
- implementation patterns
- validation/check/review concepts
- structure that fits `punk` laws

What is not reused automatically:

- its exact project contract
- its exact directory semantics
- its exact target model
- its exact command surface
- any rule that conflicts with `punk` rule hierarchy

## Future module note

If a future pub module is promoted inside `punk`, it should adopt the existing repo-tracked public artifacts and fit them into the `punk` rule hierarchy.

It should not replace the hierarchy with a copied `pubpunk` contract.
