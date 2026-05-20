# PubPunk Publish Request Packet v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk publish request packet.

The packet is the boundary between PubPunk's advisory publishing inventory work
and the existing Module Host side-effect request and policy-gate preflight
models. It carries explicit refs for a future external publish request without
publishing, invoking adapters, reading draft bodies, writing receipts, or
activating runtime behavior.

This spec does not activate PubPunk runtime, Module Host runtime, module
loading, filesystem reads, workspace initialization, external publishing,
metrics collection, adapter invocation, receipt writing, gate writing,
proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-PUBLISH-REQUEST-001: candidate and channel refs are explicit

The packet must include safe explicit refs for the candidate content and target
channel. Both refs must also be present in the packet's allowed source refs.

### PUBPUNK-PUBLISH-REQUEST-002: host side-effect refs are explicit

The packet must include safe explicit refs for the future side-effect request,
intent, policy, adapter, payload, and receipt proposal. These refs are model
inputs only; they do not create receipts, invoke adapters, or publish.

### PUBPUNK-PUBLISH-REQUEST-003: publish grant is narrow

The packet must grant only `request_external_publish` for the publish-request
step. Unsupported grants, including adapter invocation, metrics collection,
credential reads, gate/proof writing, direct event-log writes, and acceptance
claims, must block readiness.

### PUBPUNK-PUBLISH-REQUEST-004: receipt expectations include host validation

The packet must require expected receipt fields and must include `side_effects`
and `host_validation`. This describes future evidence requirements only and
does not write a receipt.

### PUBPUNK-PUBLISH-REQUEST-005: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, private customer data,
sensitive code, and raw prompts must be blocked.

### PUBPUNK-PUBLISH-REQUEST-006: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-PUBLISH-REQUEST-007: side-effect request refs are produced only when ready

The packet may project refs into `PubPunkPublishSideEffectRequestRefs` only when
packet readiness has no blockers.

### PUBPUNK-PUBLISH-REQUEST-008: Module Host chain stays preflight-only

The smoke case must chain the ready PubPunk packet into existing Module Host
side-effect request proposal and policy-gate preflight models without invoking
adapters, policy engines, gate, module runtime, filesystem IO, browser/API
behavior, or external publishing.

### PUBPUNK-PUBLISH-REQUEST-009: no runtime promotion is implied

Passing this smoke case means only that the local model chain is coherent. It
does not promote PubPunk to active runtime, public CLI, adapter behavior,
external publishing, metrics collection, receipt writing, gate writing,
proofpack writing, or acceptance authority.

## Expected advisory result

```yaml
pubpunk_publish_request_packet_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  candidate_ref_explicit: true
  channel_ref_explicit: true
  allowed_source_refs_cover_candidate_channel_payload: true
  request_external_publish_grant_required: true
  side_effect_request_refs_ready: true
  module_host_side_effect_request_proposal_ready: true
  module_host_policy_gate_preflight_ready: true
  receipt_writer_active: false
  adapter_invocation_active: false
  external_publish_active: false
  publishing_runtime: false
  side_effects: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication execution, adapter behavior, external
publishing, metrics collection, token collection, receipt writing, gate writing,
proofpack writing, or acceptance claims.
