# Publishing

Repo-tracked public narrative and content artifacts.

This is the publication plane, not a frontend static-assets `public/` directory.

This folder exists before PubPunk automation.

Humans update it manually. Later PubPunk must adopt it as its source of truth.

Important:

These are `punk` public-memory artifacts and `punk` manual publication rules.

They are not a one-to-one import of the standalone `pubpunk` project contract.

If ideas from `pubpunk` are reused, they must be adapted under the `punk` rule hierarchy and current project boundaries.

See:

- `publishing/channels/_shared/RULE-HIERARCHY.md`

## Layout

```text
publishing/
  narrative/      public-facing positioning and recurring storylines
  stories/        durable narrative arcs
  posts/          concrete post drafts/finals and channel variants
  channels/       channel descriptors and manual account notes
  publications/   publication receipts
  metrics/        manual metrics snapshots
  assets/         public-facing assets
  _templates/     templates
  _schema/        schemas
```

## Rules

- If something is published manually, create a publication receipt.
- Metrics are manual for now.
- Prefer one receipt per channel publication.
- Do not automate publishing before core stability.
- Shared publication rules under `publishing/channels/_shared/` are subordinate to `punk` laws and project docs.
- Channel-specific rules may narrow shared rules, but must not weaken higher-level `punk` rules.
