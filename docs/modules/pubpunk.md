# PubPunk

Status: incubating model / runtime parked

## Purpose

PubPunk is the future content-engineering module for `punk`.

It handles public narrative, content production, distribution, and metrics.

This document defines the module boundary before any PubPunk runtime, draft
planner, receipt writer, adapter, or external publish behavior is implemented.
The current code slice is limited to a side-effect-free inventory assessment
model over caller-provided metadata.

## Scope

Future PubPunk may own:

- publishing workspace inventory
- content strategy
- editorial calendar
- post drafting
- publication planning
- channel-specific variants
- voice/style profiles
- publication receipts
- manual or automated metrics snapshots
- content retrospectives
- growth loops

## Current state

PubPunk is not active as runtime, CLI, adapter, or publisher.

There is no automation in the new `punk` core yet.

The first incubating PubPunk crate, `punk-mod-pubpunk`, can assess explicit
caller-provided publishing inventory metadata and receipt gaps as advisory
module evidence only. It does not read files, write receipts, publish, call
external APIs, read credentials, invoke adapters, write gate decisions, write
proofpacks, or claim acceptance.

The existing `punk publishing locate` resolver is a transitional core locator
only. It must not be used as precedent for adding publishing inventory,
drafting, planning, receipt creation, or publish behavior to `punk-project`,
`punk-cli`, or any active-core crate. Those behaviors are PubPunk/module work.

However, the public artifact structure exists from day zero under:

```text
publishing/
```

PubPunk must later connect to this structure instead of inventing a second content store.

## Boundary model

PubPunk is a domain module. It is not core.

Core may eventually invoke PubPunk through the Module Host. Core must remain
responsible for contracts, scope, event policy, receipts, gate decisions,
proof refs, and source-of-truth boundaries.

PubPunk may eventually inspect publishing workspace metadata and produce
module receipts and assessments. Those outputs are advisory evidence until the
host validates them and `gate` writes any final decision.

## Workspace and instruction packet

The first workspace/instruction packet is
`docs/modules/pubpunk-workspace-instructions.md`.

It selects split explicit refs: repo-native `publishing/` artifacts for Punk's
own public narrative, plus explicit external workspace refs for
project-specific publishing operations.

This packet does not grant filesystem reads, create workspaces, publish,
collect metrics, write receipts, invoke adapters, or activate runtime behavior.

## Future module inputs

A future PubPunk invocation should receive an explicit bounded input bundle:

- module id and version;
- contract ref;
- run ref;
- project ref;
- publishing workspace ref;
- requested operation;
- allowed source refs;
- granted capabilities;
- privacy and redaction policy;
- expected receipt fields.

The module must not discover hidden project truth, read credentials, infer
scope from external platforms, or treat a publishing workspace as a second
project memory store.

## Future module outputs

A future PubPunk invocation may return:

- module receipt;
- module assessment;
- candidate content refs;
- draft or plan refs;
- publication receipt proposals;
- style/fact-check findings;
- side-effect request proposals.

These outputs must not be final decisions, proofpacks, accepted claims,
roadmap decisions, or project truth by themselves.

## Capability boundary

Capabilities are denied by default.

Candidate future capabilities must be granted separately:

- read publishing workspace metadata;
- read specific draft files;
- write local draft artifacts;
- write local receipt proposals;
- request external publishing through an adapter;
- request metrics collection through an adapter.

External publishing, credential access, browser/API calls, adapter invocation,
and metrics collection require explicit policy, side-effect receipts, and
conformance evals before activation.

## Required conformance

PubPunk must follow Punk Laws.

It may:

- inventory publishing workspace metadata after module-host promotion;
- draft content
- prepare channel variants
- prepare publication plans
- collect metrics
- emit content receipts and assessments

It may not:

- live inside `punk-project` as publishing business logic
- extend active core with more publishing subcommands before module promotion
- publish externally without declared side-effect policy
- bypass gate/proof where gate applies
- write final decisions
- mutate the event log directly
- keep a separate hidden truth store
- overwrite public history
- treat metrics as truth without source/channel metadata

## Initial activation boundary

First activation remains simple:

- module-owned local inventory assessment model over explicit metadata;
- module-owned local draft/plan model;
- module receipt and assessment shape;
- no public CLI until Module Host promotion path is selected;
- no external side effects.

External posting APIs and automated metrics collection come later.

## Non-goals

This boundary does not activate:

- PubPunk runtime;
- Module Host runtime;
- a new `punk publishing` command;
- publishing inventory implementation;
- draft generation;
- receipt writing;
- external publishing;
- browser/API calls;
- credential access;
- adapters or bots;
- automatic issue or PR creation;
- publication as project truth.
