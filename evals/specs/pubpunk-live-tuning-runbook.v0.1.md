# PubPunk Live Tuning Runbook v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the PubPunk live tuning runbook.

This spec covers the manual operator path from live customization request to
evidence packet, tuning proposal, approval decision, and promotion handoff.

It does not activate PubPunk module execution, manifest parsing, behavior resolving,
user-local config writing, behavior-artifact writing, article reading, metrics
collection, external research execution, adapter invocation, publishing, event
writing, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-LIVE-TUNING-001: request is explicit

A live tuning session must start with an explicit request ref, active work ref,
target behavior kind, and target artifact refs or limitations.

### PUBPUNK-LIVE-TUNING-002: request is not mutation authority

A natural-language, voice, or text request must not be interpreted as
permission for PubPunk to rewrite prompts, skills, configs, instruction
sources, or user-local artifacts.

### PUBPUNK-LIVE-TUNING-003: evidence plan is explicit

The runbook must require local evidence refs, metrics refs when metrics are
used, external research refs when research is allowed and used, or explicit
limitations.

### PUBPUNK-LIVE-TUNING-004: evidence packet separates refs and limitations

The evidence packet must distinguish actual retained evidence refs from
limitations such as no article reads, no metrics, or no external research.

### PUBPUNK-LIVE-TUNING-005: proposal is complete before approval

A tuning proposal must cite target artifact refs, evidence, eval/check status,
capability change status, side-effect status, immutable-core status, drift
budget impact, rollback, and requested promotion path or explicit limitations.

### PUBPUNK-LIVE-TUNING-006: approval is not auto-apply

Operator approval may be recorded as promotion input only. It must not apply
patches, write configs, select current behavior, or mutate behavior artifacts.

### PUBPUNK-LIVE-TUNING-007: decision routes are explicit

The decision must be one of proposed, rejected, no-change,
approved-for-user-local, or approved-for-project-promotion.

### PUBPUNK-LIVE-TUNING-008: user-local path stays private

User-local artifacts must remain local/private, outside repo truth, and marked
not project truth unless exported and promoted through a selected path.

### PUBPUNK-LIVE-TUNING-009: project promotion requires promotion packet

Project/workspace promotion must require a promotion packet with target
artifact, location kind, before/after refs or hashes, evidence, eval/check
status, provenance, rollback, and capability/side-effect delta.

### PUBPUNK-LIVE-TUNING-010: capability and side-effect changes stop the flow

Any proposed capability grant, side-effect expansion, credential behavior,
adapter invocation, publishing behavior, browser automation, metrics
collection, event writing, gate writing, proofpack writing, or acceptance
behavior must stop the runbook and route to a separate bounded goal.

### PUBPUNK-LIVE-TUNING-011: sensitive data is blocked

Artifacts must not contain secrets, credentials, private tokens, or sensitive
personal data.

### PUBPUNK-LIVE-TUNING-012: executable behavior is blocked

Artifacts must not contain executable code, provider hooks, adapters, scripts,
browser automation, or side-effectful actions at this boundary.

### PUBPUNK-LIVE-TUNING-013: provenance is required

Future outputs shaped by tuned behavior must cite the resolved behavior
artifact set or record an explicit limitation.

### PUBPUNK-LIVE-TUNING-014: suite validation remains read-only

The runbook may require `scripts/check.sh module-control-suite`, but that
command remains read-only and does not activate runtime, writing, promotion, or
acceptance behavior.

### PUBPUNK-LIVE-TUNING-015: template remains non-authoritative

The handoff template may provide a reusable fill-in shape, but it must remain
advisory and must not become runtime input, config writer input, behavior
artifact writer input, gate input, proofpack, or acceptance claim by itself.

## Minimal handoff shape

```yaml
pubpunk_live_tuning_handoff:
  id: null
  status: proposed
  authority: non_authoritative
  module_id: pubpunk
  control_manifest_ref: docs/modules/pubpunk-control-manifest.md
  request_ref: null
  active_work_ref: null
  evidence_packet_ref: null
  tuning_proposal_ref: null
  approval_ref: null
  target_artifact_refs: []
  proposed_artifact_location_kind: user_local_config
  proposed_artifact_ref: null
  promotion_packet_ref: null
  resolved_behavior_set_ref: null
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  contains_secrets: false
  contains_private_data: false
  executable_code_allowed: false
  rollback_ref: null
  auto_apply: false
  non_authority: true
```

## Validator command

```sh
scripts/check.sh pubpunk-live-tuning-pack
scripts/check.sh module-control-suite
```

The commands validate the current live tuning and control-plane marker suites.
They do not read articles, collect metrics, run research, write config, promote
artifacts, activate PubPunk module execution, publish, invoke adapters, write gates,
write proofpacks, or claim acceptance.

## Non-goals

This spec does not define module code, host code, manifest parsing, behavior
resolving, runtime behavior, user-local config writing, behavior-artifact
writing, article reading, metrics collection, external research execution,
adapter behavior, browser automation, credential access, publishing behavior,
event writing, gate writing, proofpack writing, or acceptance claims.
