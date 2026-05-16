# PubPunk

Status: parked

## Purpose

PubPunk is the future content-engineering module for `punk`.

It handles public narrative, content production, distribution, and metrics.

## Scope

Future PubPunk may own:

- content strategy
- editorial calendar
- post drafting
- channel-specific variants
- voice/style profiles
- publication receipts
- manual or automated metrics snapshots
- content retrospectives
- growth loops

## Current state

PubPunk is not active.

There is no automation in the new `punk` core yet.

The existing `punk publishing locate` resolver is a transitional core locator
only. It must not be used as precedent for adding publishing inventory,
drafting, planning, receipt creation, or publish behavior to `punk-project`,
`punk-cli`, or any active-core crate. Those behaviors are PubPunk/module work.

However, the public artifact structure exists from day zero under:

```text
publishing/
```

PubPunk must later connect to this structure instead of inventing a second content store.

## Required conformance

PubPunk must follow Punk Laws.

It may:

- draft content
- prepare channel variants
- collect metrics
- emit content receipts and assessments

It may not:

- publish externally without declared side-effect policy
- bypass gate/proof where gate applies
- keep a separate hidden truth store
- overwrite public history
- treat metrics as truth without source/channel metadata

## Initial activation boundary

First activation should remain simple:

- local drafts
- manual publication receipts
- manual metrics snapshots

External posting APIs and automated metrics collection come later.
