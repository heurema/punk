---
id: docs_modules_pubpunk_workspace_instructions
kind: module-doc
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-20
review_after: 2026-06-19
related_docs:
  - docs/modules/pubpunk.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/INSTRUCTION-SOURCES.md
  - publishing/README.md
related_evals:
  - evals/specs/pubpunk-workspace-instruction-packet.v0.1.md
  - evals/specs/pubpunk-inventory-reader.v0.1.md
  - evals/specs/pubpunk-inventory-input-packet.v0.1.md
  - evals/specs/pubpunk-host-handoff.v0.1.md
  - evals/specs/pubpunk-channel-connector-profile-resolution.v0.1.md
  - evals/specs/pubpunk-publish-request-packet.v0.1.md
  - evals/specs/pubpunk-publish-receipt-preflight.v0.1.md
  - evals/specs/pubpunk-publish-receipt-write-handoff.v0.1.md
  - evals/specs/pubpunk-publish-operation-evidence-handoff.v0.1.md
  - evals/specs/pubpunk-publish-receipt-evidence-event-handoff.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Workspace and Instructions

## Purpose

Define the smallest workspace and instruction packet for future PubPunk work.

This resolves the first PubPunk conformance blockers without activating
PubPunk runtime, Module Host runtime, real filesystem reads, workspace creation,
adapters, publishing, metrics collection, PubPunk-owned receipt writing, or
credential use.

## Selected workspace policy

PubPunk uses `split_explicit_refs`.

| Surface | Policy | Initial refs | Boundary |
|---|---|---|---|
| Punk public narrative artifacts | `repo_native` | `publishing/`, `publishing/posts/`, `publishing/publications/`, `publishing/metrics/`, `publishing/channels/` | Repo-tracked public-memory artifacts for this repo. |
| Project-specific operational publishing workspaces | `external_workspace` | `punk-publishing://project/<project-id>` or a caller-provided external workspace ref | Explicit pointer only; not hidden truth and not auto-discovered. |
| User-level shared publishing state | `global_punk` | none selected | Future-only; not selected for the first PubPunk work. |
| Project `.punk` publishing state | `project_punk` | none selected | Future-only; no `.punk` writes in this packet. |

The next PubPunk implementation slice must not rely on untracked local host
paths in repo-tracked artifacts. If an external workspace is needed, the work
order or future host input bundle must pass an explicit logical workspace ref.

## Initial instruction refs

Future PubPunk work should receive these thin source refs:

- `docs/modules/pubpunk.md`;
- `docs/modules/pubpunk-workspace-instructions.md`;
- `docs/product/MODULE-AUTHORING.md`;
- `docs/product/MODULE-CONFORMANCE.md`;
- `docs/product/MODULE-HOST-CONTRACT.md`;
- `docs/product/INSTRUCTION-SOURCES.md`;
- `publishing/README.md`.

These refs orient the executor. They do not replace contracts, evals, receipts,
reports, gate decisions, or proofpacks.

## Executor guidance

When asked to run PubPunk work, an executor should:

- identify the requested project and publishing workspace ref;
- keep repo-native `publishing/` refs separate from external workspace refs;
- use caller-provided metadata by default;
- list candidate refs, receipt refs, metrics refs, and gaps;
- list target channels and any needed connector profile refs;
- prefer official or documented channel APIs before browser automation when a
  later work order grants publishing or metrics automation scope;
- avoid reading draft bodies unless a later work order grants that scope;
- avoid initializing a workspace unless a later work order grants that scope;
- represent publishing, metrics, adapter, browser, network, and credential work
  as side-effect request proposals only;
- return advisory assessment evidence, not final decisions.

## First PubPunk work boundary

The existing `punk-mod-pubpunk` crate contains a side-effect-free inventory
reader model, inventory input packet, and inventory assessment model over
caller-provided publishing metadata.

The reader model takes explicit observed metadata refs, checks them against
explicit allowed source refs, and may build the input packet only when unblocked.
It allows an empty observed item set for new projects. It does not scan the
filesystem, read draft bodies, initialize a workspace, collect metrics, write
receipts, publish, or invoke adapters.

The input packet must carry explicit workspace, instruction, source,
capability, receipt-field, and optional token-cost refs before the assessment
model runs. It should not restart PubPunk from zero, add a public CLI, scan the
filesystem, generate drafts, publish externally, collect metrics, write
receipts, or activate Module Host runtime.

The current smoke evidence can hand the ready packet and advisory assessment to
existing Module Host preflight, envelope, and receipt proposal models. This is a
model-chain check only. It does not invoke a module, load plugins, initialize a
workspace, write receipts, publish, collect metrics, invoke adapters, or activate
Module Host runtime.

The current channel connector profile resolution packet can then select an API,
browser, or manual connector path from explicit inventory assessment,
candidate, channel, connector profile, API availability, browser policy,
manual handoff, credential signal, payload, instruction, and allowed-source
refs. It selects API first when available, browser automation only when API is
unavailable and browser policy allows it, and manual handoff when automated
paths are unavailable but manual fallback is allowed. It does not call APIs,
open browsers, read credentials, invoke adapters, publish, collect metrics,
write receipts, or activate PubPunk or Module Host runtime.

The current publish request packet can then carry explicit resolved connector,
candidate, channel, policy, adapter, payload, receipt, and host side-effect
request refs into the existing Module Host side-effect request proposal and
policy-gate preflight models. This remains a request/preflight chain only. It
does not publish, invoke adapters, run policy engines, invoke gate, write
receipts, read draft bodies, collect metrics, or activate PubPunk or Module
Host runtime.

The current publish receipt preflight packet can then carry explicit receipt
target, storage, operation-evidence, idempotency, rollback, error, adapter
invocation receipt, connector profile resolution, connector profile, selected
connector strategy, channel, and payload refs into the existing Module Host
side-effect receipt writer preflight model. This remains a preflight chain
only. It does not write receipts, persist operation evidence, publish, invoke
adapters, run policy engines, invoke gate, read draft bodies, collect metrics,
or activate PubPunk or Module Host runtime.

The current publish receipt write handoff packet can then carry explicit
preflight, receipt writer, target-path, receipt-bytes, operation-evidence,
adapter invocation receipt, connector profile resolution, connector profile,
selected connector strategy, channel, and payload refs into the existing Module
Host first active local receipt writer. PubPunk still does not read files or
write receipts itself. The smoke evidence writes exact caller-provided bytes
only to an explicit temporary `.punk/runs` target through the already-existing
host writer, and does not invoke adapters, publish, mutate event logs, persist
operation evidence, write gate/proof authority, or claim acceptance.

The current publish operation evidence handoff packet can then carry explicit
publish receipt write handoff, receipt writer result, operation-evidence target
path, operation-evidence bytes, adapter invocation receipt, connector profile
resolution, connector profile, selected connector strategy, channel, and payload
refs into the existing Module Host operation-evidence writer. Direct adapter,
channel, payload, or connector profile refs are not enough to bypass channel
connector profile resolution. PubPunk still does not read files, write receipts,
or write operation evidence itself. The smoke evidence writes exact
caller-provided evidence bytes only to an explicit temporary `.punk/runs`
target through the already-existing host writer after a successful receipt
write, and does not invoke adapters, publish, mutate event logs, write
gate/proof authority, or claim acceptance.

The current publish receipt/evidence event handoff packet can then carry
explicit publish operation evidence handoff, receipt writer result,
operation-evidence write result, receipt, operation-evidence, event source,
event correlation, adapter invocation receipt, connector profile resolution,
connector profile, selected connector strategy, channel, and payload refs into
the existing local receipt/evidence event writer. Direct adapter, channel,
payload, or connector profile refs are not enough to bypass channel connector
profile resolution. PubPunk still does not read files, write receipts, write
operation evidence, or mutate the event log itself. The smoke evidence appends
one bounded `receipt_evidence_handoff` event only under an explicit temporary
`.punk/events/flow.jsonl` through the already-existing `punk-events` helper,
and does not create `.punk/runs` artifacts, invoke adapters, publish, write
gate/proof authority, or claim acceptance.

## Capability envelope for the next slice

Default grants:

- `read_workspace_metadata`, when the work order passes explicit observed
  metadata refs to the reader model.
- `assess_provided_inventory`, when a ready reader or work order passes an
  explicit input packet.
- `resolve_connector_profile`, when a ready connector profile resolution packet
  is selecting API, browser, or manual strategy from explicit channel metadata.
- `request_external_publish`, only when a ready publish request packet is
  preparing explicit refs for future host side-effect policy or a ready publish
  receipt preflight packet is preparing explicit refs for future receipt-writer
  policy.
- `request_publication_receipt_write`, only when a ready publish receipt write
  handoff packet is preparing explicit refs for the existing Module Host first
  active local receipt writer.
- `request_operation_evidence_write`, only when a ready publish operation
  evidence handoff packet is preparing explicit refs for the existing Module
  Host operation-evidence writer after a successful receipt write.
- `request_receipt_evidence_event_handoff`, only when a ready publish
  receipt/evidence event handoff packet is preparing explicit refs for the
  existing local receipt/evidence event writer.

Default denies:

- broad filesystem read;
- draft body read;
- directory walks and workspace scans;
- filesystem write;
- network;
- environment;
- secrets;
- process shell;
- adapter invocation;
- external publishing;
- metrics collection;
- direct event-log writes;
- final decision writes;
- proofpack writes;
- acceptance claims.

A later slice may request scoped filesystem read over explicit publishing refs.
That request is not a grant in this packet.

## Channel connector guidance

Projects may post to several channels. PubPunk should keep each target channel
separate and require an explicit connector profile before any automated
publishing or metrics behavior is considered.

Connector selection order:

1. Prefer an official or documented channel API.
2. Use a public/read-side API for metrics when that is the safe available path.
3. Use browser automation only when API paths are unavailable or insufficient
   and the work order grants browser, credential, and receipt scope.
4. Keep manual handoff available when automation is unsafe, blocked by platform
   policy, or not mature enough.

The current connector profile resolution packet turns this guidance into
side-effect-free readiness evidence only. It does not grant network, browser,
credentials, adapter invocation, external publishing, metrics collection, or
receipt writing.

## Inventory reader checks

For the current code slice, the reader model blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing safe publishing workspace ref;
- missing required instruction refs;
- unsafe instruction, allowed-source, observed-item, workspace, or token-cost
  refs;
- observed item refs not present in the allowed source refs;
- missing `read_workspace_metadata` grant;
- unsupported grants such as draft reads, publishing, metrics, adapters,
  credentials, gate, proof, or acceptance behavior;
- raw post bodies or privacy policy that allows raw/private payloads;
- missing expected receipt fields.

These checks are advisory readiness checks only. The reader name does not mean
the model reads the filesystem; it classifies caller-provided metadata refs.

## Advisory assessment fields

Use these fields for the first PubPunk receipt or assessment shape:

```yaml
pubpunk_assessment:
  module_id: pubpunk
  operation: assess_provided_inventory
  workspace_policy: split_explicit_refs
  workspace_ref: null
  allowed_source_refs: []
  capability_grants:
    - assess_provided_inventory
  denied_capabilities:
    - filesystem_read
    - filesystem_write
    - network
    - secrets
    - adapter_invoke
    - external_publish
    - metrics_collect
    - direct_event_log_write
    - final_decision_write
    - proofpack_write
  inventory_counts:
    candidates: null
    receipts: null
    metrics_snapshots: null
  gap_findings: []
  side_effect_requests: []
  token_cost_ref: null
  non_authority: true
```

This is an advisory shape, not an active schema or runtime receipt writer.

## Input packet checks

For the current code slice, the input packet blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing required instruction refs;
- unsafe instruction, source, workspace, or token-cost refs;
- item refs not present in the allowed source refs;
- missing `assess_provided_inventory` grant;
- unsupported grants such as publishing, metrics, adapter, credential, gate, or
  proof behavior;
- raw post bodies or privacy policy that allows raw/private payloads;
- missing expected receipt fields.

These checks are advisory readiness checks only. They do not read the
referenced files, create workspaces, collect token metrics, publish, or write
receipts.

## Connector profile resolution checks

For the current code slice, the connector profile resolution packet blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing safe publishing workspace ref;
- missing inventory assessment, candidate, channel, connector profile, API
  availability, browser policy, manual handoff, credential signal, or payload
  refs;
- those refs not present in the allowed source refs;
- missing or non-API-first strategy order;
- missing required instruction refs;
- unsafe instruction, allowed-source, workspace, packet, or token-cost refs;
- missing `resolve_connector_profile` grant;
- unsupported grants such as external publishing, adapter invocation, metrics
  collection, credential reads, gate/proof behavior, direct event-log writes,
  or acceptance claims;
- raw post bodies, raw external payloads, secrets, or privacy policy that
  allows raw/private payloads;
- missing expected receipt fields, especially
  `connector_profile_resolution`, `selected_connector_strategy`,
  `credential_signal_ref`, and `manual_fallback`;
- no allowed API, browser, or manual strategy from explicit signals.

These checks are advisory readiness checks only. They select connector refs and
strategy metadata for future handoffs. They do not call APIs, open browsers,
read credentials, invoke adapters, publish, collect metrics, or write receipts.

## Publish request packet checks

For the current code slice, the publish request packet blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing safe publishing workspace ref;
- missing inventory assessment, candidate, channel, connector profile
  resolution, connector profile, selected connector strategy, side-effect
  request, intent, policy, adapter, payload, or receipt proposal refs;
- candidate, channel, connector profile resolution, connector profile,
  selected connector strategy, or payload refs not present in the allowed
  source refs;
- missing required instruction refs;
- unsafe instruction, allowed-source, workspace, packet, or token-cost refs;
- missing `request_external_publish` grant;
- unsupported grants such as adapter invocation, metrics collection,
  credential reads, gate/proof behavior, direct event-log writes, or acceptance
  claims;
- raw post bodies or privacy policy that allows raw/private payloads;
- missing expected receipt fields, especially `side_effects`,
  `host_validation`, `connector_profile_resolution`, `connector_profile_ref`,
  and `selected_connector_strategy`.

These checks are advisory readiness checks only. They prepare refs for existing
Module Host side-effect request and policy-gate preflight models. They do not
write a side-effect request, invoke an adapter, run policy, invoke gate, publish,
read draft bodies, collect metrics, or write receipts.

## Publish receipt preflight packet checks

For the current code slice, the publish receipt preflight packet blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing safe publishing workspace ref;
- missing publish request, receipt writer preflight, policy gate preflight,
  receipt target, storage, operation-evidence, idempotency, rollback, error,
  adapter invocation receipt, payload, channel, connector profile resolution,
  connector profile, or selected connector strategy refs;
- payload, channel, connector profile resolution, connector profile, or
  selected connector strategy refs not present in the allowed source refs;
- missing required instruction refs;
- unsafe instruction, allowed-source, workspace, packet, or token-cost refs;
- missing `request_external_publish` grant;
- unsupported grants such as adapter invocation, metrics collection,
  credential reads, gate/proof behavior, direct event-log writes, or acceptance
  claims;
- raw post bodies or privacy policy that allows raw/private payloads;
- missing expected receipt fields, especially `side_effects`,
  `host_validation`, `connector_profile_resolution`, `connector_profile_ref`,
  `selected_connector_strategy`, `adapter_invocation_receipt`,
  `operation_evidence`, and `publication_receipt`.

These checks are advisory readiness checks only. They prepare refs for the
existing Module Host side-effect receipt writer preflight model. They do not
write receipts, persist operation evidence, invoke adapters, publish, run
policy, invoke gate, read draft bodies, collect metrics, or activate runtime.
Direct adapter, channel, payload, or connector profile refs are not enough to
bypass connector profile resolution.

## Publish receipt write handoff packet checks

For the current code slice, the publish receipt write handoff packet blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing safe publishing workspace ref;
- missing publish receipt preflight, receipt writer preflight, active behavior,
  file-IO plan, target/storage policy, host-path observation, concrete
  path/storage policy, or operation-evidence persistence refs;
- missing receipt target, storage, target path, receipt bytes,
  operation-evidence, idempotency, rollback, error, adapter invocation receipt,
  payload, channel, connector profile resolution, connector profile, or
  selected connector strategy refs;
- receipt target path refs outside `.punk/runs`;
- payload, channel, connector profile resolution, connector profile, selected
  connector strategy, adapter invocation receipt, or receipt bytes refs not
  present in the allowed source refs;
- missing required instruction refs;
- unsafe instruction, allowed-source, workspace, packet, path, or token-cost
  refs;
- missing `request_publication_receipt_write` grant;
- unsupported grants such as external publishing, adapter invocation, metrics
  collection, credential reads, gate/proof behavior, direct event-log writes,
  or acceptance claims;
- raw post bodies or privacy policy that allows raw/private payloads;
- missing expected receipt fields, especially `side_effects`,
  `host_validation`, `connector_profile_resolution`,
  `connector_profile_ref`, `selected_connector_strategy`,
  `adapter_invocation_receipt`, `operation_evidence`, `publication_receipt`,
  `receipt_bytes`, `receipt_target_path`, and `receipt_write_result`.

These checks are advisory readiness checks only. They prepare refs for the
existing Module Host first active local receipt writer. PubPunk does not read
receipt bytes, write receipts, persist operation evidence, invoke adapters,
publish, run policy, invoke gate, read draft bodies, collect metrics, or
activate runtime. The current smoke case exercises the already-existing host
writer only against an explicit temporary `.punk/runs` target.

## Non-goals

This packet does not activate:

- PubPunk runtime;
- Module Host runtime;
- generated instruction views;
- workspace initialization;
- filesystem scanning or draft body reads;
- public CLI behavior;
- provider orchestration;
- adapter invocation;
- external publishing;
- metrics collection;
- credential access;
- receipt writing;
- gate decisions;
- proofpack writing;
- acceptance claims.
