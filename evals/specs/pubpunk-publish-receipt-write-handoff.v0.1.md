# PubPunk Publish Receipt Write Handoff v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk publish receipt write
handoff packet.

The packet is the boundary between PubPunk's publish receipt preflight model
and the existing Module Host first active local side-effect receipt writer. It
carries explicit refs for a host-owned exact-byte receipt write without making
PubPunk a receipt writer, reading receipt bytes, persisting operation evidence,
publishing, invoking adapters, mutating event logs, or activating runtime
behavior.

This spec does not activate PubPunk runtime, Module Host runtime, module
loading, filesystem reads by PubPunk, workspace initialization, external
publishing, metrics collection, adapter invocation, operation-evidence
persistence, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-001: handoff refs are explicit

The packet must include safe explicit refs for publish receipt preflight,
receipt writer preflight, active behavior, file-IO plan, target/storage policy,
host-path observation, concrete path/storage policy, operation-evidence
persistence, receipt target, storage, target path, receipt bytes, operation
evidence, idempotency, rollback, error, adapter invocation receipt, payload,
channel, and connector profile.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-002: receipt target stays under `.punk/runs`

The receipt target path ref must be safe and must stay under `.punk/runs` for
the first active handoff. Other target paths must block readiness before the
handoff can project refs.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-003: source refs stay allowed

The payload, channel, connector profile, adapter invocation receipt, and receipt
bytes refs must be present in the packet's allowed source refs. These are
evidence refs only; PubPunk does not read the referenced files in this slice.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-004: write handoff grant is narrow

The packet must grant only `request_publication_receipt_write` for this handoff
step. Unsupported grants, including external publishing, adapter invocation,
metrics collection, credential reads, gate/proof writing, direct event-log
writes, and acceptance claims, must block readiness.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-005: receipt expectations include write coverage

The packet must require expected receipt fields and must include `side_effects`,
`host_validation`, `adapter_invocation_receipt`, `operation_evidence`,
`publication_receipt`, `receipt_bytes`, `receipt_target_path`, and
`receipt_write_result`.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-006: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, private customer data,
sensitive code, and raw prompts must be blocked.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-007: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-008: host writer refs are produced only when ready

The packet may project refs into `PubPunkPublishReceiptWriteHandoffRefs` only
when packet readiness has no blockers.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-009: Module Host write stays bounded

The smoke case must chain the ready PubPunk packet into the existing Module Host
first active local side-effect receipt writer after the existing side-effect
request, policy-gate, receipt-writer preflight, file-IO plan, target/storage,
host-path observation, concrete path/storage, and operation-evidence
persistence models are ready.

The write may write exact caller-provided bytes only to an explicit temporary
`.punk/runs` target. It must not invoke adapters, policy engines, gate, module
runtime, browser/API behavior, external publishing, event-log mutation,
operation-evidence persistence, proofpack writing, or acceptance claims.

### PUBPUNK-PUBLISH-RECEIPT-WRITE-HANDOFF-010: no runtime promotion is implied

Passing this smoke case means only that the local model chain is coherent. It
does not promote PubPunk to active runtime, public CLI, adapter behavior,
external publishing, metrics collection, persisted publication receipt writer,
operation-evidence persistence, gate writing, proofpack writing, or acceptance
authority.

## Expected advisory result

```yaml
pubpunk_publish_receipt_write_handoff_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  publish_receipt_preflight_ref_explicit: true
  receipt_writer_preflight_ref_explicit: true
  receipt_writer_active_behavior_ref_explicit: true
  receipt_writer_file_io_plan_ref_explicit: true
  receipt_writer_target_storage_policy_ref_explicit: true
  receipt_writer_host_path_observation_ref_explicit: true
  receipt_writer_concrete_path_storage_policy_ref_explicit: true
  operation_evidence_persistence_ref_explicit: true
  receipt_target_path_ref_under_punk_runs: true
  receipt_bytes_ref_explicit: true
  request_publication_receipt_write_grant_required: true
  receipt_write_handoff_refs_ready: true
  module_host_receipt_writer_write_ready: true
  module_host_receipt_writer_write_exercised_in_temp_target: true
  pubpunk_reads_receipt_bytes: false
  pubpunk_writes_receipt: false
  adapter_invocation_active: false
  external_publish_active: false
  operation_evidence_persistence_active: false
  event_log_mutation_active: false
  publishing_runtime: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication execution, adapter behavior, external
publishing, metrics collection, token collection, PubPunk-owned receipt
writing, operation-evidence persistence, gate writing, proofpack writing, or
acceptance claims.
