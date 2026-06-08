# PubPunk Publish Operation Evidence Handoff v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk publish operation
evidence handoff packet after channel connector profile resolution.

The packet is the boundary between PubPunk's publish receipt write handoff model
and the existing Module Host operation-evidence writer for receipt-writer
results. It carries explicit resolved connector evidence plus refs for a
host-owned exact-byte operation evidence write without making PubPunk an
evidence writer, receipt writer, publisher, adapter, event-log mutator,
gate/proof authority, or runtime surface.

This spec does not activate PubPunk module execution, Module Host runtime, module
loading, filesystem reads by PubPunk, workspace initialization, external
publishing, metrics collection, adapter invocation, event-log mutation, gate
writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-001: handoff refs are explicit

The packet must include safe explicit refs for publish receipt write handoff,
receipt writer result, receipt storage, receipt target, receipt target path,
receipt bytes, operation evidence, operation evidence target path, operation
evidence bytes, operation evidence write result, idempotency, rollback, error,
adapter invocation receipt, payload, channel, connector profile resolution,
connector profile, and selected connector strategy.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-002: operation evidence target stays under `.punk/runs`

The operation evidence target path ref must be safe and must stay under
`.punk/runs` for the first active handoff. Other target paths must block
readiness before the handoff can project refs.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-003: operation evidence target matches writer result ref

The operation evidence target path ref must match the operation evidence ref
from the receipt writer result. This mirrors the existing Module Host
operation-evidence writer precondition and prevents PubPunk from inventing a
different evidence destination.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-004: source refs stay allowed

The publish receipt write handoff, receipt writer result, payload, channel,
connector profile resolution, connector profile, selected connector strategy,
adapter invocation receipt, receipt bytes, and operation evidence bytes refs
must be present in the packet's allowed source refs. Direct adapter, channel,
payload, or connector profile refs are not enough to bypass channel connector
profile resolution. These are evidence refs only; PubPunk does not read the
referenced files in this slice.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-005: write handoff grant is narrow

The packet must grant only `request_operation_evidence_write` for this handoff
step. Unsupported grants, including external publishing, adapter invocation,
metrics collection, credential reads, receipt writing, direct event-log writes,
gate/proof writing, and acceptance claims, must block readiness.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-006: receipt expectations include evidence write coverage

The packet must require expected receipt fields and must include `side_effects`,
`host_validation`, `connector_profile_resolution`, `connector_profile_ref`,
`selected_connector_strategy`, `adapter_invocation_receipt`,
`operation_evidence`, `publication_receipt`, `receipt_write_result`,
`operation_evidence_bytes`, `operation_evidence_target_path`, and
`operation_evidence_write_result`.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-007: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, private customer data,
sensitive code, and raw prompts must be blocked.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-008: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-009: host writer refs are produced only when ready

The packet may project refs into `PubPunkPublishOperationEvidenceHandoffRefs`
only when packet readiness has no blockers.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-010: Module Host evidence write stays bounded

The smoke case must chain the ready PubPunk packet into the existing Module Host
operation-evidence writer after a successful receipt write.

The write may write exact caller-provided evidence bytes only to an explicit
temporary `.punk/runs` target. It must not invoke adapters, policy engines,
gate, module runtime, browser/API behavior, external publishing, event-log
mutation, receipt writing, proofpack writing, or acceptance claims.

### PUBPUNK-PUBLISH-OPERATION-EVIDENCE-HANDOFF-011: no runtime promotion is implied

Passing this smoke case means only that the local model chain is coherent. It
does not promote PubPunk to active module execution, public CLI, adapter behavior,
external publishing, metrics collection, persisted publication receipt writer,
persisted operation evidence writer, gate writing, proofpack writing, or
acceptance authority.

## Expected advisory result

```yaml
pubpunk_publish_operation_evidence_handoff_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  publish_receipt_write_handoff_ref_explicit: true
  receipt_writer_result_ref_explicit: true
  connector_profile_resolution_ref_explicit: true
  connector_profile_ref_explicit: true
  selected_connector_strategy_ref_explicit: true
  allowed_source_refs_cover_payload_channel_resolved_connector: true
  operation_evidence_target_path_ref_under_punk_runs: true
  operation_evidence_target_matches_writer_result_ref: true
  operation_evidence_bytes_ref_explicit: true
  request_operation_evidence_write_grant_required: true
  operation_evidence_handoff_refs_ready: true
  module_host_operation_evidence_writer_ready: true
  module_host_operation_evidence_writer_exercised_in_temp_target: true
  pubpunk_reads_operation_evidence_bytes: false
  pubpunk_writes_operation_evidence: false
  pubpunk_writes_receipt: false
  adapter_invocation_active: false
  external_publish_active: false
  event_log_mutation_active: false
  publishing_runtime: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication execution, adapter behavior, external
publishing, metrics collection, token collection, PubPunk-owned receipt
writing, PubPunk-owned operation-evidence writing, event-log mutation, gate
writing, proofpack writing, or acceptance claims.
