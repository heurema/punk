# PubPunk Publish Receipt Preflight v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk publish receipt
preflight packet after channel connector profile resolution.

The packet is the boundary between PubPunk's publish request model, resolved
connector evidence, and the existing Module Host side-effect receipt writer
preflight model. It carries explicit refs for future publication receipt
writing without writing receipts, persisting operation evidence, publishing,
invoking adapters, reading draft bodies, or activating runtime behavior.

This spec does not activate PubPunk runtime, Module Host runtime, module
loading, filesystem reads, workspace initialization, external publishing,
metrics collection, adapter invocation, receipt writing, operation-evidence
persistence, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-001: receipt refs are explicit

The packet must include safe explicit refs for the publish request, receipt
writer preflight, policy gate preflight, receipt target, receipt storage,
operation evidence, idempotency, rollback, error, adapter invocation receipt,
payload, channel, connector profile resolution, connector profile, and selected
connector strategy.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-002: resolved connector refs stay allowed

The payload, channel, connector profile resolution, connector profile, and
selected connector strategy refs must be present in the packet's allowed source
refs. Connector refs are evidence inputs only; they do not invoke adapters or
read credentials.

The packet must not treat direct adapter, channel, payload, or connector profile
refs as enough to bypass channel connector profile resolution.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-003: publish grant is narrow

The packet must grant only `request_external_publish` for this preflight step.
Unsupported grants, including adapter invocation, metrics collection,
credential reads, gate/proof writing, direct event-log writes, and acceptance
claims, must block readiness.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-004: receipt expectations include evidence coverage

The packet must require expected receipt fields and must include `side_effects`,
`host_validation`, `connector_profile_resolution`, `connector_profile_ref`,
`selected_connector_strategy`, `adapter_invocation_receipt`,
`operation_evidence`, and `publication_receipt`. This describes future evidence
requirements only and does not write a receipt.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-005: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, private customer data,
sensitive code, and raw prompts must be blocked.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-006: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-007: host writer refs are produced only when ready

The packet may project refs into
`PubPunkPublishReceiptWriterPreflightRefs` only when packet readiness has no
blockers.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-008: Module Host chain stays preflight-only

The smoke case must chain the ready PubPunk packet into the existing Module Host
side-effect receipt writer preflight model after the existing side-effect
request proposal and policy-gate preflight models are ready. The chain must not
invoke adapters, policy engines, gate, module runtime, filesystem IO,
browser/API behavior, external publishing, receipt writing, or
operation-evidence persistence.

### PUBPUNK-PUBLISH-RECEIPT-PREFLIGHT-009: no runtime promotion is implied

Passing this smoke case means only that the local model chain is coherent. It
does not promote PubPunk to active runtime, public CLI, adapter behavior,
external publishing, metrics collection, receipt writing, operation-evidence
persistence, gate writing, proofpack writing, or acceptance authority.

## Expected advisory result

```yaml
pubpunk_publish_receipt_preflight_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  receipt_target_ref_explicit: true
  receipt_storage_ref_explicit: true
  operation_evidence_ref_explicit: true
  connector_profile_resolution_ref_explicit: true
  connector_profile_ref_explicit: true
  selected_connector_strategy_ref_explicit: true
  allowed_source_refs_cover_payload_channel_resolved_connector: true
  request_external_publish_grant_required: true
  receipt_writer_preflight_refs_ready: true
  module_host_side_effect_request_proposal_ready: true
  module_host_policy_gate_preflight_ready: true
  module_host_receipt_writer_preflight_ready: true
  receipt_writer_active: false
  adapter_invocation_active: false
  external_publish_active: false
  operation_evidence_persistence_active: false
  publishing_runtime: false
  side_effects: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication execution, adapter behavior, external
publishing, metrics collection, token collection, receipt writing,
operation-evidence persistence, gate writing, proofpack writing, or acceptance
claims.
