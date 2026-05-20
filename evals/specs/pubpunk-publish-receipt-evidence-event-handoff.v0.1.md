# PubPunk Publish Receipt/Evidence Event Handoff v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk publish
receipt/evidence event handoff packet.

The packet is the boundary between PubPunk's publish operation evidence
handoff model and the existing `punk-events` local receipt/evidence event
writer. It carries explicit refs for a host-owned `receipt_evidence_handoff`
event without making PubPunk an event-log writer, evidence writer, receipt
writer, publisher, adapter, gate/proof authority, or runtime surface.

This spec does not activate PubPunk runtime, Module Host runtime, module
loading, filesystem reads by PubPunk, workspace initialization, external
publishing, metrics collection, adapter invocation, PubPunk-owned event-log
mutation, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-001: handoff refs are explicit

The packet must include safe explicit refs for publish operation evidence
handoff, receipt writer result, operation evidence write result, receipt,
operation evidence, event log, event source, event correlation, adapter
invocation receipt, payload, channel, and connector profile.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-002: receipt and evidence refs stay under `.punk/runs`

The receipt and operation evidence refs must be safe and must stay under
`.punk/runs`. Other artifact refs must block readiness before the event
handoff can project refs.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-003: receipt and evidence refs stay separated

The receipt ref and operation evidence ref must not collapse to the same ref.
This mirrors the existing `punk-events` helper boundary and prevents a single
artifact from pretending to be both receipt and evidence.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-004: event log ref is fixed

The event log ref must be `.punk/events/flow.jsonl`. Other event log refs must
block readiness before the handoff can project refs.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-005: source refs stay allowed

The publish operation evidence handoff, receipt writer result, operation
evidence write result, receipt, operation evidence, event source, event
correlation, payload, channel, connector profile, and adapter invocation
receipt refs must be present in the packet's allowed source refs. These are
metadata refs only; PubPunk does not read the referenced files in this slice.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-006: event handoff grant is narrow

The packet must grant only `request_receipt_evidence_event_handoff` for this
handoff step. Unsupported grants, including external publishing, adapter
invocation, metrics collection, credential reads, receipt writing,
operation-evidence writing, direct event-log writes, gate/proof writing, and
acceptance claims, must block readiness.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-007: receipt expectations include event handoff coverage

The packet must require expected receipt fields and must include `side_effects`,
`host_validation`, `adapter_invocation_receipt`, `operation_evidence`,
`publication_receipt`, `receipt_write_result`,
`operation_evidence_write_result`, and `receipt_evidence_event_handoff`.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-008: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, private customer data,
sensitive code, and raw prompts must be blocked.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-009: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-010: event handoff refs are produced only when ready

The packet may project refs into
`PubPunkPublishReceiptEvidenceEventHandoffRefs` only when packet readiness has
no blockers.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-011: local event append stays bounded

The smoke case must chain the ready PubPunk packet into the existing
`punk-events` receipt/evidence handoff helper.

The append may write exactly one `receipt_evidence_handoff` event only to an
explicit temporary `.punk/events/flow.jsonl` log under an initialized temporary
project root. It must not create `.punk/runs` artifacts, invoke adapters, policy
engines, gate, module runtime, browser/API behavior, external publishing,
receipt writing, operation-evidence writing, proofpack writing, or acceptance
claims.

### PUBPUNK-PUBLISH-RECEIPT-EVIDENCE-EVENT-HANDOFF-012: no runtime promotion is implied

Passing this smoke case means only that the local model chain is coherent. It
does not promote PubPunk to active runtime, public CLI, adapter behavior,
external publishing, metrics collection, persisted publication receipt writer,
persisted operation evidence writer, PubPunk-owned event writer, gate writing,
proofpack writing, or acceptance authority.

## Expected advisory result

```yaml
pubpunk_publish_receipt_evidence_event_handoff_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  publish_operation_evidence_handoff_ref_explicit: true
  receipt_writer_result_ref_explicit: true
  operation_evidence_write_result_ref_explicit: true
  receipt_ref_under_punk_runs: true
  operation_evidence_ref_under_punk_runs: true
  receipt_and_evidence_refs_separated: true
  event_log_ref_fixed_to_punk_flow_log: true
  request_receipt_evidence_event_handoff_grant_required: true
  event_handoff_refs_ready: true
  local_receipt_evidence_event_writer_exercised_in_temp_log: true
  pubpunk_writes_event_log: false
  pubpunk_writes_operation_evidence: false
  pubpunk_writes_receipt: false
  adapter_invocation_active: false
  external_publish_active: false
  publishing_runtime: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication execution, adapter behavior, external
publishing, metrics collection, token collection, PubPunk-owned receipt
writing, PubPunk-owned operation-evidence writing, PubPunk-owned event-log
writing, gate writing, proofpack writing, or acceptance claims.
