# PubPunk Channel Connector Profile Resolution v0.1

Date: 2026-05-20
Status: implemented boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for the first PubPunk channel connector
profile resolution packet.

The packet is the boundary between PubPunk's advisory inventory work and later
publish request/preflight work. It selects a connector strategy from explicit
API, browser, and manual fallback metadata without calling APIs, opening
browsers, reading credentials, invoking adapters, publishing, collecting
metrics, writing receipts, or activating runtime behavior.

This spec does not activate PubPunk runtime, Module Host runtime, module
loading, filesystem reads, workspace initialization, external publishing,
metrics collection, adapter invocation, browser automation, credential reads,
receipt writing, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-CONNECTOR-PROFILE-001: connector refs are explicit

The packet must include safe explicit refs for the inventory assessment,
candidate, channel, connector profile, API availability, browser automation
policy, manual handoff, credential signal, and payload. These refs must also be
present in the packet's allowed source refs.

### PUBPUNK-CONNECTOR-PROFILE-002: API is selected first

When an explicit API availability signal says the channel API is available, the
packet must select `api` before browser automation or manual handoff. The
first slice must reject custom strategy orders that put browser or manual ahead
of API.

### PUBPUNK-CONNECTOR-PROFILE-003: browser fallback requires policy

When API is unavailable, the packet may select `browser` only when the explicit
browser automation policy allows it. The selection itself must not open a
browser or read credentials.

### PUBPUNK-CONNECTOR-PROFILE-004: manual fallback remains valid

When automated paths are unavailable and manual fallback is explicitly allowed,
the packet must select `manual`.

### PUBPUNK-CONNECTOR-PROFILE-005: no allowed path blocks readiness

When API, browser, and manual paths are all unavailable or disallowed, the
packet must block readiness instead of inventing a connector route.

### PUBPUNK-CONNECTOR-PROFILE-006: grant is narrow

The packet must grant only `resolve_connector_profile`. Unsupported grants,
including external publishing, adapter invocation, metrics collection,
credential reads, gate/proof writing, direct event-log writes, and acceptance
claims, must block readiness.

### PUBPUNK-CONNECTOR-PROFILE-007: receipt expectations name the boundary

The packet must require expected receipt fields including
`connector_profile_resolution`, `selected_connector_strategy`,
`credential_signal_ref`, `manual_fallback`, `side_effects`, and
`host_validation`. This describes future evidence requirements only and does
not write a receipt.

### PUBPUNK-CONNECTOR-PROFILE-008: privacy remains metadata-only

Raw post bodies, raw external payloads, credentials, private customer data,
sensitive code, and raw prompts must be blocked.

### PUBPUNK-CONNECTOR-PROFILE-009: token cost ref is metadata only

An optional token cost ref may be carried as a safe repo-relative metadata ref.
The packet must not collect token usage automatically.

### PUBPUNK-CONNECTOR-PROFILE-010: no runtime promotion is implied

Passing this smoke case means only that the local model can select a connector
strategy from explicit metadata. It does not promote PubPunk to active runtime,
public CLI, adapter behavior, browser automation, external publishing, metrics
collection, receipt writing, gate writing, proofpack writing, or acceptance
authority.

## Expected advisory result

```yaml
pubpunk_channel_connector_profile_resolution_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  connector_profile_ref_explicit: true
  allowed_source_refs_cover_connector_inputs: true
  resolve_connector_profile_grant_required: true
  api_selected_when_available: true
  non_api_first_order_blocks: true
  browser_selected_only_when_allowed: true
  manual_selected_when_automation_unavailable: true
  no_allowed_strategy_blocks: true
  browser_automation_active: false
  adapter_invocation_active: false
  external_publish_active: false
  metrics_collection_active: false
  publishing_runtime: false
  side_effects: false
  non_authority: true
```

## Non-goals

This spec does not define runtime invocation, public CLI behavior, module
manifests, plugin loading, dynamic dispatch, workspace creation, filesystem
scanning, draft body reads, publication execution, adapter behavior, browser
automation, credential handling, external publishing, metrics collection, token
collection, receipt writing, gate writing, proofpack writing, or acceptance
claims.
