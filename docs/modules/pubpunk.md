# PubPunk

Status: incubating model / runtime parked

## Purpose

PubPunk is the future content-engineering module for `punk`.

It handles public narrative, content production, distribution, and metrics.

This document defines the module boundary before any PubPunk runtime, draft
planner, receipt writer, adapter, or external publish behavior is implemented.
The current code slice is limited to a side-effect-free inventory reader model,
inventory input packet, inventory assessment model, channel connector profile
resolution packet, publish request packet, publish receipt preflight packet,
publish receipt write handoff packet, publish operation evidence handoff
packet, and publish receipt/evidence event handoff packet, plus smoke evidence
for wrapping advisory PubPunk outputs through existing
Module Host preflight, envelope, receipt proposal, side-effect request
proposal, policy-gate preflight, side-effect receipt writer preflight, and
first active local receipt writer and operation-evidence writer models, and
for appending a bounded receipt/evidence handoff event through `punk-events`.

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
- channel connector plans and readiness

## Current state

PubPunk is not active as runtime, CLI, adapter, or publisher.

There is no automation in the new `punk` core yet.

The first incubating PubPunk crate, `punk-mod-pubpunk`, can validate an
explicit inventory reader input, build an explicit inventory input packet when
that reader input is ready, assess caller-provided publishing inventory
metadata and receipt gaps as advisory module evidence only, prepare an explicit
channel connector profile resolution packet for selecting an API, browser, or
manual connector path from explicit metadata, prepare an explicit publish
request packet for a future host side-effect request, and prepare an explicit
publish receipt preflight packet for the existing Module Host side-effect
receipt writer preflight model. The reader model accepts explicit observed
metadata refs and keeps them inside explicit allowed source refs. The input
packet requires explicit workspace, instruction, source, capability,
receipt-field, and optional token-cost refs before assessment. The connector
profile resolution packet requires explicit inventory assessment, candidate,
channel, connector profile, API availability, browser policy, manual handoff,
credential signal, payload, allowed-source, instruction, expected-receipt,
privacy, and optional token-cost refs before it can project selected connector
refs. The publish request packet requires explicit candidate, channel, policy,
connector profile resolution, connector profile, selected connector strategy,
adapter, payload, receipt, host side-effect request, and expected-receipt refs
before it can project side-effect request refs. The publish receipt preflight
packet requires explicit receipt target, storage, operation-evidence,
idempotency, rollback, error, adapter invocation receipt, channel, connector
profile resolution, connector profile, selected connector strategy, payload,
and expected-receipt refs before it can project host receipt-writer preflight
refs.
The publish receipt write handoff packet requires explicit preflight, receipt
writer model, target path, receipt bytes, operation-evidence, adapter
invocation receipt, connector profile, channel, payload, and expected-receipt
refs before it can project handoff refs for the existing Module Host first
active local receipt writer. The publish operation evidence handoff packet
requires explicit receipt write handoff, receipt writer result, receipt storage,
receipt target, receipt path, receipt bytes, operation-evidence target path,
operation-evidence bytes, operation-evidence write result, idempotency,
rollback, error, adapter invocation receipt, connector profile, channel,
payload, and expected-receipt refs before it can project handoff refs for the
existing Module Host operation-evidence writer. These PubPunk models do not
read files, write receipts, write operation evidence, publish, call external
APIs, open browsers, read credentials, invoke adapters, write gate decisions,
write proofpacks, or claim acceptance.

The current `punk-eval` smoke evidence also proves the first host handoff chain:
`PubPunkInventoryInputPacket` readiness, PubPunk inventory assessment, Module
Host invocation preflight, advisory envelope, and Module Host receipt proposal.
This evidence does not invoke modules, load plugins, activate Module Host
runtime, write receipts, publish, collect metrics, invoke adapters, or promote
authority.

The current smoke evidence also proves the first publish request packet chain:
a ready PubPunk packet consumes resolved connector refs before it projects
explicit candidate, channel, policy, adapter, payload, and receipt refs into
the existing Module Host side-effect request proposal and policy-gate preflight
models. This evidence does not publish, invoke adapters, run policy engines,
invoke gate, write receipts, read draft bodies, or activate runtime behavior.

The current smoke evidence also proves the first channel connector profile
resolution: a ready PubPunk packet selects API first when an explicit API
availability ref says it is available, falls back to browser automation only
when API is unavailable and explicit browser policy allows it, and falls back
to manual handoff when both automated paths are unavailable. This evidence does
not call APIs, open browsers, read credentials, invoke adapters, publish,
collect metrics, write receipts, or activate runtime behavior.

The current smoke evidence also proves the first publish receipt preflight
chain: a ready PubPunk packet projects explicit receipt target, storage,
operation-evidence, idempotency, rollback, error, adapter invocation receipt,
connector profile resolution, connector profile, selected connector strategy,
channel, and payload refs into the existing Module Host side-effect receipt
writer preflight model. This evidence does not write receipts, invoke adapters,
publish, run policy engines, invoke gate, read draft bodies, collect metrics,
or activate runtime behavior.

The current smoke evidence also proves the first publish receipt write handoff:
a ready PubPunk packet projects explicit preflight, receipt writer, target path,
receipt bytes, operation-evidence, adapter invocation receipt, connector,
channel, and payload refs into the existing Module Host first active local
receipt writer. The writer writes exact caller-provided bytes only to an
explicit temporary `.punk/runs` target. This evidence does not invoke adapters,
publish, run policy engines, invoke gate, mutate event logs, persist operation
evidence, write proofpacks, claim acceptance, or activate PubPunk or Module Host
runtime behavior.

The current smoke evidence also proves the first publish operation evidence
handoff: a ready PubPunk packet projects explicit receipt writer result,
operation-evidence target path, operation-evidence bytes, adapter invocation
receipt, connector, channel, and payload refs into the existing Module Host
operation-evidence writer. The writer writes exact caller-provided evidence
bytes only to an explicit temporary `.punk/runs` target after a successful
receipt write. This evidence does not invoke adapters, publish, run policy
engines, invoke gate, mutate event logs, write receipts, write proofpacks, claim
acceptance, or activate PubPunk or Module Host runtime behavior.

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

This packet does not grant broad filesystem reads, create workspaces, publish,
collect metrics, write receipts, invoke adapters, or activate runtime behavior.
The current reader model may require the narrow `read_workspace_metadata` grant
as declared input evidence, but it still does not perform IO.

## Current inventory reader model

The current side-effect-free inventory reader input is:

```text
PubPunkInventoryReaderInput
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- required instruction refs;
- allowed source refs;
- observed item metadata refs;
- `read_workspace_metadata` capability;
- metadata-only privacy and redaction policy;
- expected receipt fields.

The reader allows an empty observed item set so a new project can start without
existing drafts, receipts, or metrics snapshots. It blocks raw post bodies,
unsafe refs, observed refs outside the allowed source set, unsupported
capabilities, and unsafe optional token-cost refs.

When ready, the reader can build `PubPunkInventoryInputPacket` and switch the
next capability to `assess_provided_inventory`. This is still deterministic
model evidence only. It is not a filesystem reader, workspace initializer,
publication receipt writer, metrics collector, adapter, public CLI, Module Host
runtime invocation, gate writer, proofpack writer, or acceptance authority.

## Current module input packet

The current side-effect-free input packet is:

```text
PubPunkInventoryInputPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- allowed source refs;
- instruction refs;
- granted capabilities;
- privacy and redaction policy;
- expected receipt fields.

The packet may carry a `token_cost_ref` as a metadata ref only. PubPunk does not
collect token usage automatically in this slice.

`PubPunkInventoryInputPacket` can convert to the existing
`PubPunkInventoryInput` only when the packet assessment is ready. The module
must not discover hidden project truth, read credentials, infer scope from
external platforms, or treat a publishing workspace as a second project memory
store.

## Current host handoff evidence

The current host handoff evidence is:

```text
eval_pubpunk_host_handoff_chains_packet_to_receipt_proposal_without_side_effects
```

It starts from the input packet, runs the PubPunk inventory assessment, then
uses existing Module Host preflight, advisory envelope, and receipt proposal
models.

This is not runtime invocation. It is deterministic model-chain evidence only.
It does not add a module manifest, dynamic dispatch, public CLI, filesystem
reader, workspace initializer, external publisher, metrics collector, receipt
writer, gate writer, proofpack writer, or acceptance authority.

## Current channel connector profile resolution packet

The current side-effect-free channel connector profile resolution packet is:

```text
PubPunkChannelConnectorProfileResolutionPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- inventory assessment ref;
- candidate content ref;
- channel ref;
- connector profile ref;
- API availability ref;
- browser automation policy ref;
- manual handoff ref;
- credential signal ref;
- payload ref;
- strategy order fixed to API before browser before manual for this slice;
- allowed source refs covering inventory assessment, candidate, channel,
  connector profile, API availability, browser policy, manual handoff,
  credential signal, and payload refs;
- instruction refs;
- `resolve_connector_profile` capability;
- metadata-only privacy policy;
- expected receipt fields including `side_effects`, `host_validation`,
  `connector_profile_resolution`, `selected_connector_strategy`, `channel_ref`,
  `connector_profile_ref`, `credential_signal_ref`, and `manual_fallback`;
- optional token-cost ref.

When ready, the packet can project only
`PubPunkResolvedChannelConnectorProfileRefs`. It selects the connector strategy
from explicit caller-provided signals: API when available, browser automation
only when API is unavailable and browser policy allows it, and manual handoff
when the automated paths are unavailable but manual fallback is allowed. It
does not call an API, open a browser, read credentials, invoke an adapter,
publish externally, collect metrics, write receipts, read draft bodies, or
activate PubPunk or Module Host runtime behavior.

## Current publish request packet

The current side-effect-free publish request packet is:

```text
PubPunkPublishRequestPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- inventory assessment ref;
- candidate content ref;
- channel ref;
- connector profile resolution ref;
- connector profile ref;
- selected connector strategy ref;
- host side-effect request ref;
- intent ref;
- policy ref;
- adapter ref;
- payload ref;
- receipt proposal ref;
- allowed source refs covering candidate, channel, connector profile
  resolution, connector profile, selected connector strategy, and payload refs;
- instruction refs;
- `request_external_publish` capability;
- metadata-only privacy policy;
- expected receipt fields including `side_effects`, `host_validation`,
  `connector_profile_resolution`, `connector_profile_ref`, and
  `selected_connector_strategy`;
- optional token-cost ref.

When ready, the packet can project only
`PubPunkPublishSideEffectRequestRefs` for existing Module Host side-effect
request and policy-gate preflight models. Direct adapter or payload refs are
not enough to bypass the connector profile resolution boundary. The packet
does not write the side-effect request, invoke an adapter, publish externally,
call a policy engine, invoke gate, write receipts, read draft bodies, collect
metrics, or activate PubPunk or Module Host runtime behavior.

## Current publish receipt preflight packet

The current side-effect-free publish receipt preflight packet is:

```text
PubPunkPublishReceiptPreflightPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- publish request ref;
- receipt writer preflight ref;
- policy gate preflight ref;
- receipt target ref;
- receipt storage ref;
- operation evidence ref;
- idempotency ref;
- rollback ref;
- error ref;
- adapter invocation receipt ref;
- payload ref;
- channel ref;
- connector profile resolution ref;
- connector profile ref;
- selected connector strategy ref;
- allowed source refs covering payload, channel, connector profile resolution,
  connector profile, and selected connector strategy refs;
- instruction refs;
- `request_external_publish` capability;
- metadata-only privacy policy;
- expected receipt fields including `side_effects`, `host_validation`,
  `connector_profile_resolution`, `connector_profile_ref`,
  `selected_connector_strategy`, `adapter_invocation_receipt`,
  `operation_evidence`, and `publication_receipt`;
- optional token-cost ref.

When ready, the packet can project only
`PubPunkPublishReceiptWriterPreflightRefs` for the existing Module Host
side-effect receipt writer preflight model. Direct adapter, channel, payload,
or connector profile refs are not enough to bypass the connector profile
resolution boundary. The packet does not write a receipt, persist operation
evidence, invoke an adapter, publish externally, call a policy engine, invoke
gate, read draft bodies, collect metrics, or activate PubPunk or Module Host
runtime behavior.

## Current publish receipt write handoff packet

The current side-effect-free publish receipt write handoff packet is:

```text
PubPunkPublishReceiptWriteHandoffPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- publish receipt preflight ref;
- receipt writer preflight, active-behavior, file-IO plan, target/storage
  policy, host-path observation, concrete path/storage policy, and
  operation-evidence persistence refs;
- receipt target, storage, target path, and receipt bytes refs;
- operation evidence, idempotency, rollback, and error refs;
- adapter invocation receipt ref;
- payload ref;
- channel ref;
- connector profile ref;
- allowed source refs covering payload, channel, connector profile, adapter
  invocation receipt, and receipt bytes refs;
- instruction refs;
- `request_publication_receipt_write` capability;
- metadata-only privacy policy;
- expected receipt fields including `side_effects`, `host_validation`,
  `adapter_invocation_receipt`, `operation_evidence`, `publication_receipt`,
  `receipt_bytes`, `receipt_target_path`, and `receipt_write_result`;
- optional token-cost ref.

When ready, the packet can project only
`PubPunkPublishReceiptWriteHandoffRefs` for the existing Module Host first
active local side-effect receipt writer. The PubPunk packet does not write a
receipt, read receipt bytes, persist operation evidence, invoke an adapter,
publish externally, call a policy engine, invoke gate, read draft bodies,
collect metrics, or activate PubPunk or Module Host runtime behavior. The
current smoke case uses the projected refs to call the already-existing Module
Host writer against an explicit temporary `.punk/runs` target only.

## Current publish operation evidence handoff packet

The current side-effect-free publish operation evidence handoff packet is:

```text
PubPunkPublishOperationEvidenceHandoffPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- publish receipt write handoff ref;
- receipt writer result ref;
- receipt storage, target, target path, and receipt bytes refs;
- operation evidence ref and matching operation evidence target path ref;
- operation evidence bytes ref;
- operation evidence write result ref;
- idempotency, rollback, and error refs;
- adapter invocation receipt ref;
- payload ref;
- channel ref;
- connector profile ref;
- allowed source refs covering publish receipt write handoff, receipt writer
  result, payload, channel, connector profile, adapter invocation receipt,
  receipt bytes, and operation evidence bytes refs;
- instruction refs;
- `request_operation_evidence_write` capability;
- metadata-only privacy policy;
- expected receipt fields including `side_effects`, `host_validation`,
  `adapter_invocation_receipt`, `operation_evidence`, `publication_receipt`,
  `receipt_write_result`, `operation_evidence_bytes`,
  `operation_evidence_target_path`, and `operation_evidence_write_result`;
- optional token-cost ref.

When ready, the packet can project only
`PubPunkPublishOperationEvidenceHandoffRefs` for the existing Module Host
operation-evidence writer. The PubPunk packet does not write operation
evidence, read operation evidence bytes, write receipts, invoke an adapter,
publish externally, call a policy engine, invoke gate, read draft bodies,
collect metrics, or activate PubPunk or Module Host runtime behavior. The
current smoke case uses the projected refs to call the already-existing Module
Host writer against an explicit temporary `.punk/runs` target only after a
successful receipt write.

## Current publish receipt/evidence event handoff packet

The current side-effect-free publish receipt/evidence event handoff packet is:

```text
PubPunkPublishReceiptEvidenceEventHandoffPacket
```

It must provide:

- module id and version;
- contract ref;
- run ref;
- project ref;
- `split_explicit_refs` workspace policy;
- publishing workspace ref;
- publish operation evidence handoff ref;
- receipt writer result ref;
- operation evidence write result ref;
- receipt ref and operation evidence ref;
- event log ref, fixed to `.punk/events/flow.jsonl`;
- event source and event correlation refs;
- adapter invocation receipt ref;
- payload ref;
- channel ref;
- connector profile ref;
- allowed source refs covering publish operation evidence handoff, receipt
  writer result, operation evidence write result, receipt, operation evidence,
  event source, event correlation, payload, channel, connector profile, and
  adapter invocation receipt refs;
- instruction refs;
- `request_receipt_evidence_event_handoff` capability;
- metadata-only privacy policy;
- expected receipt fields including `side_effects`, `host_validation`,
  `adapter_invocation_receipt`, `operation_evidence`, `publication_receipt`,
  `receipt_write_result`, `operation_evidence_write_result`, and
  `receipt_evidence_event_handoff`;
- optional token-cost ref.

When ready, the packet can project only
`PubPunkPublishReceiptEvidenceEventHandoffRefs` for the existing local
receipt/evidence event writer in `punk-events`. The PubPunk packet does not
write the event log, write receipts, write operation evidence, read files,
invoke an adapter, publish externally, call a policy engine, invoke gate, read
draft bodies, collect metrics, or activate PubPunk or Module Host runtime
behavior. The current smoke case uses the projected refs to call the
already-existing `punk-events` receipt/evidence handoff helper against an
explicit temporary `.punk/events/flow.jsonl` log only, without creating
`.punk/runs` artifacts.

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

## Channel connector strategy

Projects may publish into multiple channels. PubPunk should treat every
project/channel pair as an explicit channel surface with its own connector
profile.

The preferred connector order is:

1. Use an official or documented channel API when one exists and the work order
   grants that channel scope.
2. Use a public/read-side API for metrics when publishing APIs are unavailable
   but metrics can be collected safely.
3. Use browser automation only when an API path is unavailable or insufficient,
   and only after explicit policy, credential, and receipt requirements are in
   place.
4. Keep manual handoff as a valid fallback when credentials, platform policy,
   account safety, or implementation maturity blocks automation.

Connector profiles are PubPunk/module adapter surfaces, but current code only
resolves profile refs and strategy choice from explicit metadata. They must
stay per-channel and per-project, must not be inferred from hidden local state,
and must preserve source refs, target channel refs, credential boundaries,
metrics refs, receipt refs, and side-effect policy separately.

This strategy and the current resolution model do not activate connectors,
browser automation, credentials, network calls, publishing, metrics collection,
or adapter invocation in the current slice.

## Capability boundary

Capabilities are denied by default.

Candidate future capabilities must be granted separately:

- read publishing workspace metadata;
- resolve channel connector profile;
- read specific draft files;
- write local draft artifacts;
- write local receipt proposals;
- request local publication receipt write handoff;
- request local operation evidence write handoff;
- request local receipt/evidence event handoff;
- request external publishing through an adapter;
- request metrics collection through an adapter.

External publishing, credential access, browser/API calls, adapter invocation,
and metrics collection require explicit policy, side-effect receipts, and
conformance evals before activation.

In the current code, `read_workspace_metadata` is accepted only by
`PubPunkInventoryReaderInput`, and `assess_provided_inventory` is accepted only
by `PubPunkInventoryInputPacket`. `resolve_connector_profile` is accepted only
by `PubPunkChannelConnectorProfileResolutionPacket` as a side-effect-free
selection grant over explicit API/browser/manual metadata.
`request_external_publish` is accepted only by `PubPunkPublishRequestPacket` and
`PubPunkPublishReceiptPreflightPacket` as request/preflight-model grants for
future host side-effect policy. `request_publication_receipt_write` is accepted
only by `PubPunkPublishReceiptWriteHandoffPacket` as a handoff grant to the
existing Module Host first active local receipt writer.
`request_operation_evidence_write` is accepted only by
`PubPunkPublishOperationEvidenceHandoffPacket` as a handoff grant to the
existing Module Host operation-evidence writer after a successful receipt write.
`request_receipt_evidence_event_handoff` is accepted only by
`PubPunkPublishReceiptEvidenceEventHandoffPacket` as a handoff grant to the
existing local receipt/evidence event writer. None of these grants activates
PubPunk runtime, adapter behavior, publication, metrics collection,
PubPunk-owned receipt/evidence writing, direct event-log mutation, gate/proof
authority, or acceptance promotion.

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
- runtime publishing inventory implementation;
- filesystem-scanning inventory implementation;
- draft generation;
- receipt writing;
- operation-evidence writing;
- external publishing;
- browser/API calls;
- credential access;
- adapters or bots;
- automatic issue or PR creation;
- publication as project truth.
