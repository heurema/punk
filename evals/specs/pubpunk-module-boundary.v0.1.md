# PubPunk module boundary v0.1

Date: 2026-05-16
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the boundary PubPunk must satisfy before any publishing inventory,
drafting, planning, receipt creation, adapter invocation, or external publish
behavior becomes active.

This is a design/spec artifact only. It does not activate PubPunk module execution,
Module Host runtime, active-core publishing subcommands, adapters, browser/API
calls, credential access, publication receipts, or external side effects.

## Status and authority

PubPunk is a future domain module.

It may produce module receipts and assessments after Module Host promotion.
Those outputs are advisory evidence until host validation and gate/proof
handoff. PubPunk cannot own project truth, write final decisions, bypass scope,
or make external publishing surfaces authoritative.

## Required deterministic eval cases

### PUBPUNK-001: module-owned, not core-owned

Publishing inventory, drafting, planning, receipt creation, and publish
behavior must be modeled as PubPunk/module work. A design or implementation
that adds this business logic to `punk-project`, `punk-cli`, or active core
before Module Host promotion must fail.

### PUBPUNK-002: locate remains transitional resolver only

The existing `punk publishing locate` surface may resolve a local publishing
workspace binding. It must not imply permission to add more active-core
publishing subcommands or to treat the resolver as active PubPunk module execution.

### PUBPUNK-003: denied capabilities by default

A PubPunk invocation with no explicit grants must have no filesystem write,
network, browser/API, environment, secrets, process, adapter, external
publishing, direct event-log write, or final decision capability.

### PUBPUNK-004: bounded input bundle required

Any future PubPunk invocation must receive an explicit input bundle containing
module id/version, contract ref, run ref, project ref, publishing workspace ref,
requested operation, allowed source refs, granted capabilities, privacy policy,
and expected receipt fields. Hidden discovery of project truth or external
platform state is invalid.

### PUBPUNK-005: workspace is not truth

The publishing workspace may be an input surface and artifact store. It must
not become project memory, final decision authority, roadmap authority, or a
second source of truth.

### PUBPUNK-006: module output is advisory

Candidate content refs, draft refs, publication plans, style/fact-check
findings, receipt proposals, and module assessments must be advisory until
host validation and gate/proof handoff. They cannot close work or claim
acceptance.

### PUBPUNK-007: external side effects require policy and receipts

External publishing, comments, metrics collection, browser/API calls,
credential reads, and adapter invocations must require explicit side-effect
policy, capability grant, and side-effect receipt before activation.

### PUBPUNK-008: no raw secrets or private payloads

PubPunk inputs, receipts, assessments, plans, and logs must not store secrets,
credentials, private tokens, customer data, sensitive code, raw prompts, raw
external payloads, or raw transcripts unless a later privacy policy explicitly
allows a redacted form.

### PUBPUNK-009: no automatic publication receipt from planning

Inventory, drafting, planning, or receipt proposal behavior must not create a
publication receipt. A publication receipt requires an actual publication
event or an explicitly recorded manual publication URL/source.

### PUBPUNK-010: adapters invoke but do not own truth

Future GitHub, Telegram, LinkedIn, Discord, browser, or other platform
adapters may invoke external actions only through granted capability and must
return side-effect receipts. Adapter output is not project truth by itself.

### PUBPUNK-011: deterministic local inventory is non-authoritative

A future local inventory model may list candidate post/draft refs and receipt
gaps from explicit local source refs. It must not print full post bodies by
default, read credentials, call APIs, create receipts, publish, or mutate
project truth.

### PUBPUNK-012: gate remains final decision writer

PubPunk may assess readiness, risks, claims, and receipt completeness. Only
`gate` may write a final decision.

## Minimal fixture shape

```yaml
pubpunk_module_invocation:
  module_id: pubpunk
  module_version: v0.1
  status: planned
  contract_ref: contracts/example
  run_ref: runs/example
  project_ref: project/example
  publishing_workspace_ref: punk-publishing://project/example
  requested_operation: inventory
  allowed_source_refs:
    - publishing/posts/example.md
  granted_capabilities: []
  privacy_policy:
    raw_post_bodies: false
    raw_external_payloads: false
    secrets: false
  expected_receipt_fields:
    - module_id
    - operation
    - source_refs
    - capability_grants
    - side_effects
```

## Non-goals

This v0.1 boundary does not define or activate PubPunk module execution, Module Host
runtime, active-core publishing subcommands, module manifest schema, module SDK,
plugin packaging, local inventory implementation, draft generation, receipt
writer, adapter implementation, external publishing, browser/API calls,
credential access, metrics collection, automatic issue/PR creation, gate
writer, proofpack writer, or acceptance claim writer.

## Future implementation constraints

Any future implementation must keep PubPunk as module-owned behavior, keep
capabilities denied by default, preserve receipts and assessments as evidence,
keep gate as final decision writer, and require explicit side-effect receipts
for any external action.
