# PubPunk Control Manifest v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the PubPunk control manifest fixture.

This spec applies `docs/product/MODULE-CONTROL-PLANE.md` to PubPunk without
activating PubPunk module execution, manifest parsing, reflection, behavior-artifact
writing, adapter invocation, publishing, metrics collection, event writing,
gate writing, proofpack writing, or acceptance claims.

## Status and authority

The PubPunk control manifest fixture is advisory until a later validator,
Module Host path, or gate policy promotes any part of it.

## Required deterministic eval cases

### PUBPUNK-CONTROL-001: fixture cites required refs

The fixture must cite `docs/modules/pubpunk.md`,
`docs/modules/pubpunk-workspace-instructions.md`,
`docs/modules/pubpunk-article-hook-tuning-fixture.md`,
`docs/product/MODULE-CONTROL-PLANE.md`,
`docs/product/MODULE-AUTHORING.md`,
`docs/product/MODULE-CONFORMANCE.md`, and
`docs/product/MODULE-HOST-CONTRACT.md`.

### PUBPUNK-CONTROL-002: behavior artifact is explicit

The fixture must list `docs/modules/pubpunk-workspace-instructions.md` as an
advisory `instruction_source` behavior artifact.

### PUBPUNK-CONTROL-003: grants are empty

The fixture must keep `capabilities.granted` empty.

### PUBPUNK-CONTROL-004: side effects are denied

The fixture must deny external publishing, credential reads, adapter
invocation, browser automation, metrics collection, direct event-log writes,
final decision writes, and proofpack writes.

### PUBPUNK-CONTROL-005: immutable core is explicit

The fixture must list immutable core boundaries for capability envelope, host
validation, gate interface, receipt emission, proof authority, side-effect
policy, credential policy, and direct event-log write policy.

### PUBPUNK-CONTROL-006: reflection is disabled

The fixture must set `reflection.trigger: none` and `auto_apply: false`.

### PUBPUNK-CONTROL-007: no tuning proposal is selected

The fixture must not select a current PubPunk tuning proposal without concrete
evidence refs and eval/check refs.

### PUBPUNK-CONTROL-008: scoped packet capabilities are not inherited

Existing PubPunk packet models may require scoped capabilities in their own
packet evidence. This fixture must not treat those scoped packet capabilities
as manifest-level grants.

### PUBPUNK-CONTROL-009: fixture is not runtime

The fixture must not imply active manifest parsing, Module Host runtime,
PubPunk module execution, background reflection, behavior-artifact writing, adapter
invocation, browser automation, credential access, publishing, metrics
collection, event writing, gate writing, proofpack writing, or acceptance
claims.

### PUBPUNK-CONTROL-010: manual tuning stays future and proposal-only

The fixture may mention `manual` as an allowed future trigger, but current
reflection must remain `trigger: none`. A live request to tune article hooks or
style guidance must be represented as a manual tuning proposal with evidence
refs, optional metrics refs, optional external research refs when granted, and
approval or gate promotion refs. The fixture must not allow PubPunk to rewrite
behavior artifacts by itself.

### PUBPUNK-CONTROL-011: no promotion is selected

The fixture must not select a current behavior artifact promotion. Any future
approved PubPunk tuning proposal must still require a promotion packet with
target artifact, artifact location kind, before/after refs or hashes, evidence,
eval/check refs, provenance, rollback, and capability/side-effect delta.

### PUBPUNK-CONTROL-012: user-local overlay is not selected

The fixture must not select a current PubPunk user-local overlay. Future
user-local style, hook, template, or routing preferences must be marked
`project_truth: false` unless explicitly exported and promoted through a
selected path, and must not grant capabilities or expand side effects.

### PUBPUNK-CONTROL-013: no user-local artifact is selected

The fixture must not select a current PubPunk user-local behavior artifact.
Future private hook, style, prompt, template, routing, or workflow artifacts
must stay local/private by default, contain no secrets or sensitive personal
data, allow no executable code, and require a promotion packet before export to
project/workspace behavior.

### PUBPUNK-CONTROL-014: hook tuning fixture is non-current

The article hook tuning fixture may show a full request/proposal/user-local
artifact/promotion/resolved-behavior chain, but it must remain example-only and
must not select real PubPunk behavior, real evidence, real metrics, external
research execution, user-local config writing, publishing, adapters, browser
automation, gates, proofpacks, or acceptance behavior.

## Minimal fixture shape

```yaml
pubpunk_control_manifest_fixture:
  id: module_control_pubpunk_v0_1
  status: advisory
  authority: non_authoritative
  module_id: pubpunk
  module_ref: docs/modules/pubpunk.md
  lifecycle_target: incubating
  module_version_ref: crates/punk-mod-pubpunk
  baseline_ref: docs/product/MODULE-AUTHORING.md
  host_contract_ref: docs/product/MODULE-HOST-CONTRACT.md
  control_plane_ref: docs/product/MODULE-CONTROL-PLANE.md
  conformance_ref: docs/product/MODULE-CONFORMANCE.md
  behavior_artifacts:
    - ref: docs/modules/pubpunk-workspace-instructions.md
      kind: instruction_source
      authority: advisory
  immutable_core:
    - capability_envelope
    - host_validation
    - gate_interface
    - receipt_emission
    - proof_authority
    - side_effect_policy
    - credential_policy
    - direct_event_log_write_policy
  capabilities:
    granted: []
    denied:
      - external_publish
      - credential_read
      - adapter_invoke
      - browser_automation
      - metrics_collect
      - direct_event_log_write
      - final_decision_write
      - proofpack_write
    requests: []
  reflection:
    trigger: none
    output_kind: tuning_proposal
    auto_apply: false
  provenance:
    artifact_set_hash: null
    latest_gate_ref: null
    latest_eval_ref: null
    latest_tuning_proposal_ref: null
  drift_budget:
    status: not_started
    cumulative_change: none
    re_audit_required: false
  non_authority: true
```

## Non-goals

This spec does not define module code, host code, manifest parsing,
deterministic checker implementation, PubPunk module execution, Module Host runtime,
workspace creation, filesystem scanning, draft body reads, instruction
generation, behavior-artifact writing, adapter behavior, browser automation,
credential access, publishing behavior, metrics collection, gate writing,
proofpack writing, or acceptance claims.

## Validator command

```sh
scripts/check.sh pubpunk-control-manifest
```

The command is a read-only marker check over this spec and
`docs/modules/pubpunk-control-manifest.md`. It does not parse manifests, resolve
behavior artifacts, run reflection, write config, promote artifacts, activate
PubPunk module execution, publish, collect metrics, invoke adapters, write gates, write
proofpacks, or claim acceptance.
