# Public

Repo-tracked public narrative and content artifacts.

This folder exists before PubPunk automation.

Humans update it manually. Later PubPunk must adopt it as its source of truth.

## Layout

```text
public/
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
