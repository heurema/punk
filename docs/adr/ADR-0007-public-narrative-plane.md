# ADR-0007: Public narrative plane before PubPunk automation

Status: proposed

## Context

`punk` is intended to be a public build.

A future PubPunk module will handle content engineering: posts, channels, publication, metrics, and growth loops.

However, the project needs to preserve the first public story, channel artifacts, and manual publication receipts before PubPunk exists in the new architecture.

## Decision

Add a repo-tracked Public Narrative Plane under `publishing/`.

Use `publishing/` instead of `public/` so the repo-tracked publication plane is not confused with the conventional frontend static-assets directory name.

This plane stores:

- narrative arcs
- post drafts/finals
- channel descriptors
- publication receipts
- manual metrics snapshots
- public assets

PubPunk must later adopt this structure rather than create a separate hidden content store.

## Current scope

Allowed now:

- manual post drafting
- multiple target channels
- manual publication receipts
- manual metrics snapshots
- public narrative docs

Out of scope now:

- automated posting
- social API integrations
- metrics scraping
- scheduling
- growth automation

## Consequences

Positive:

- public build starts before code automation
- first public artifacts are not lost in chat
- PubPunk gets a stable data contract later

Negative:

- manual discipline required
- public artifacts need privacy/claim checks
