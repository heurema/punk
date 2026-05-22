# Module Control Plane v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the Module Control Plane contract before
any module uses editable behavior artifacts, reflection, or tuning proposals as
implementation guidance.

This spec does not activate runtime manifest parsing, module loading,
background reflection, behavior-artifact writing, adapter invocation, event
writing, receipt writing, publishing, metrics collection, gate writing,
proofpack writing, or acceptance claims.

## Status and authority

Module Control Plane checks are advisory until a later validator, Module Host
path, or gate policy promotes them.

## Required deterministic eval cases

### MODULE-CONTROL-001: manifest cites contract refs

A module control manifest must cite module id, module ref, baseline ref, host
contract ref, and control plane ref.

### MODULE-CONTROL-002: behavior artifact refs are explicit

Behavior artifacts must be listed as explicit refs with kind and authority.
Repo-tracked refs must be repo-relative. External workspace artifacts must be
represented through explicit workspace or receipt refs.

### MODULE-CONTROL-003: behavior artifacts are non-authoritative

Behavior artifacts must not claim project truth, final-decision authority,
proof authority, gate authority, or direct event-log authority.

### MODULE-CONTROL-004: immutable core is protected

A manifest or tuning proposal must not move capability envelopes, host
validation, gate interface, receipt emission, proof authority, final decision
ownership, side-effect policy, or credential policy into editable behavior
artifacts.

### MODULE-CONTROL-005: capabilities stay deny-by-default

Capability requests in a manifest or proposal are not grants. Editable
behavior artifacts must not grant capabilities or override the capability
envelope.

### MODULE-CONTROL-006: reflection trigger is explicit

Reflection trigger must be one of `none`, `manual`, or
`scheduled_proposal_only`. Missing trigger state is incomplete.

### MODULE-CONTROL-007: reflection cannot auto-apply

Reflection output must be `tuning_proposal`, and `auto_apply` must be false.

### MODULE-CONTROL-008: provenance status is recorded

The manifest must record behavior-provenance fields such as artifact-set hash,
latest gate ref, latest eval ref, or explicit null/limitation values.

### MODULE-CONTROL-009: tuning proposal target is explicit

A tuning proposal must cite module id, manifest ref or limitation, target
artifact refs, and proposed change ref or explicit no-change rationale.

### MODULE-CONTROL-010: tuning proposal cites evidence

A tuning proposal must cite evidence refs or record explicit evidence
limitations. Self-produced evidence alone must be marked as a limitation.

### MODULE-CONTROL-011: tuning proposal cites eval or check status

A tuning proposal must cite eval/check refs or record an explicit
not-yet-tested limitation.

### MODULE-CONTROL-012: proposal cannot silently change side effects

A tuning proposal that changes side-effect policy, credential behavior,
external publishing behavior, adapter invocation, browser behavior, or metrics
collection must be treated as blocked or escalated to a separate bounded goal.

### MODULE-CONTROL-013: drift budget impact is recorded

A tuning proposal must record drift budget impact. Unknown impact is allowed
only as a limitation and cannot support automatic promotion.

### MODULE-CONTROL-014: PubPunk fixture has no active behavior

The PubPunk fixture must keep reflection trigger `none`, `auto_apply: false`,
empty grants, denied publishing/credential/adapter/metrics/event/decision/proof
capabilities, and advisory-only behavior artifact refs.

### MODULE-CONTROL-015: control plane is not runtime

The contract or spec must not imply active manifest parsing, module loading,
background reflection, behavior-artifact writing, adapter invocation, event
writing, publishing, metrics collection, gate writing, proofpack writing, or
acceptance claims.

### MODULE-CONTROL-016: manual in-session tuning is explicit

Manual in-session tuning must record `trigger: manual`, an operator request ref
or limitation, target artifact refs, and an active work ref or limitation. A
natural-language or voice request is not enough to mutate behavior artifacts.

### MODULE-CONTROL-017: manual tuning remains proposal-only

A manual tuning pass must emit a tuning proposal or no-change rationale. It
must not auto-apply patches, rewrite prompts, rewrite skills, rewrite configs,
grant capabilities, publish, collect metrics, invoke adapters, write receipts,
write events, write gate decisions, write proofpacks, or claim acceptance.

### MODULE-CONTROL-018: local evidence, metrics, and research are cited

If a proposal uses recent outputs, comments, metrics, or external research, it
must cite local evidence refs, metrics refs, and external research refs or
record explicit limitations. External research is incomplete unless the work
order allowed it and retained refs.

### MODULE-CONTROL-019: approval is not auto-apply authority

Operator approval may be recorded as `approval_ref`, but promotion still
requires a separate selected edit or gate path with provenance. Approval must
not be interpreted as permission for the module to self-modify.

### MODULE-CONTROL-020: promotion packet records before and after artifacts

A behavior artifact promotion packet must cite target artifact ref, artifact
location kind, before artifact ref or hash, after artifact ref or hash, applied
change ref or limitation, and provenance ref.

### MODULE-CONTROL-021: promotion cites proposal, approval, and checks

A promotion packet must cite tuning proposal ref, approval ref or limitation,
evidence refs, eval refs, and check refs or explicit limitations before it can
mark a behavior artifact set current.

### MODULE-CONTROL-022: promotion cannot smuggle capability changes

A promotion packet must record capability delta, side-effect delta, and
immutable-core status. Any non-empty capability or side-effect delta must be
blocked or escalated to a separate bounded goal.

### MODULE-CONTROL-023: external and user-local artifact refs are explicit

External workspace and user-local config behavior artifacts must be represented
through explicit workspace refs, receipt refs, or local config refs. They must
not be hidden project truth.

### MODULE-CONTROL-024: rollback is recorded

A promotion packet must cite a rollback ref or record an explicit
irreversible-change warning. Missing rollback state makes the packet incomplete
for promotion.

### MODULE-CONTROL-025: overlay stack order is explicit

A behavior overlay stack must record layer names, order, location kind,
artifact refs or null/limitations, and whether each layer is project truth.

### MODULE-CONTROL-026: user-local overlays are not project truth

User-local behavior artifacts must be marked `project_truth: false` unless they
are explicitly exported and promoted through a selected path. They must not be
committed or treated as shared defaults by implication.

### MODULE-CONTROL-027: overlays cannot grant capabilities

Behavior overlays must not grant capabilities, expand side effects, read
credentials, invoke adapters, publish, write gates, write proofpacks, or claim
acceptance. Capability or side-effect changes require a separate bounded goal.

### MODULE-CONTROL-028: run-local overrides are ephemeral

Run-local overrides must record expiration or explicit limitation. They must
not silently become future module defaults or user-local config.

### MODULE-CONTROL-029: resolved behavior set is cited

Module outputs shaped by overlays must cite the resolved behavior artifact set,
including layer refs or explicit null/limitation values.

### MODULE-CONTROL-030: user-local artifacts are scoped

A user-local behavior artifact must record module id, owner scope, layer,
artifact kind, artifact ref or limitation, storage ref or limitation, storage
policy, and `project_truth: false`.

### MODULE-CONTROL-031: user-local artifacts are private by default

User-local behavior artifacts must stay local/private and outside repo truth by
default. Exporting them to a project/workspace layer requires a behavior
artifact promotion packet.

### MODULE-CONTROL-032: user-local artifacts contain no secrets

User-local behavior artifacts must record `contains_secrets: false` and must
not include credentials, private tokens, or sensitive personal data.

### MODULE-CONTROL-033: user-local artifacts are not executable extensions

User-local behavior artifacts must record `executable_code_allowed: false` at
this boundary. Scripts, provider hooks, adapters, browser automation, and
side-effectful actions require separate bounded implementation work.

### MODULE-CONTROL-034: user-local provenance is visible

Module outputs shaped by a user-local behavior artifact must cite the artifact
through the resolved behavior artifact set or record an explicit limitation.

## Minimal fixture shape

```yaml
module_control_plane_fixture:
  manifest:
    id: module_control_pubpunk_fixture_v0_1
    status: advisory
    authority: non_authoritative
    module_id: pubpunk
    module_ref: docs/modules/pubpunk.md
    baseline_ref: docs/product/MODULE-AUTHORING.md
    host_contract_ref: docs/product/MODULE-HOST-CONTRACT.md
    control_plane_ref: docs/product/MODULE-CONTROL-PLANE.md
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
    capabilities:
      granted: []
      denied:
        - external_publish
        - credential_read
        - adapter_invoke
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
  behavior_overlay_stack:
    id: behavior_overlay_pubpunk_example_v0_1
    status: advisory
    authority: non_authoritative
    module_id: pubpunk
    layers:
      - name: packaged_defaults
        order: 10
        location_kind: module_package
        artifact_refs: []
        writable_by_module: false
        project_truth: false
      - name: project_workspace
        order: 20
        location_kind: repo_tracked
        artifact_refs:
          - docs/modules/pubpunk-workspace-instructions.md
        writable_by_module: false
        project_truth: true
      - name: user_local
        order: 30
        location_kind: user_local_config
        artifact_refs: []
        writable_by_module: false
        project_truth: false
      - name: run_local
        order: 40
        location_kind: ephemeral_run
        artifact_refs: []
        writable_by_module: false
        project_truth: false
    conflict_policy: later_layer_overrides_behavior_only
    capability_policy: overlays_cannot_grant_capabilities
    side_effect_policy: overlays_cannot_expand_side_effects
    provenance_required: true
    non_authority: true
  user_local_behavior_artifact:
    id: user_local_pubpunk_example_v0_1
    status: proposed
    authority: local_private
    module_id: pubpunk
    owner_scope: single_operator
    layer: user_local
    artifact_kind: hook_profile
    artifact_ref: null
    artifact_hash: null
    storage_ref: null
    storage_policy: local_private_outside_repo
    source_request_ref: null
    tuning_proposal_ref: null
    promotion_ref: null
    exported_to_project: false
    project_truth: false
    contains_secrets: false
    contains_private_data: false
    executable_code_allowed: false
    capability_delta: none
    side_effect_delta: none
    immutable_core_touched: false
    provenance_ref: null
    rollback_ref: null
    review_after: null
    non_authority: true
  tuning_proposal:
    id: tuning_proposal_pubpunk_example_v0_1
    status: proposed
    authority: non_authoritative
    module_id: pubpunk
    manifest_ref: null
    trigger: manual
    operator_request_ref: null
    active_work_ref: null
    target_artifact_refs:
      - docs/modules/pubpunk-workspace-instructions.md
    proposed_change_ref: null
    proposed_change_kind: patch
    evidence_refs: []
    local_evidence_refs: []
    metrics_refs: []
    external_research_refs: []
    evidence_limitations:
      - insufficient_independent_evidence
    eval_refs: []
    check_refs: []
    capability_change_requested: false
    immutable_core_touched: false
    side_effect_policy_changed: false
    drift_budget_impact: unknown
    approval_ref: null
    requested_gate_ref: null
    non_authority: true
  behavior_artifact_promotion:
    id: behavior_promotion_pubpunk_example_v0_1
    status: proposed
    authority: non_authoritative
    module_id: pubpunk
    manifest_ref: null
    tuning_proposal_ref: null
    approval_ref: null
    promotion_path_ref: null
    target_artifact_ref: docs/modules/pubpunk-workspace-instructions.md
    artifact_location_kind: repo_tracked
    before_artifact_ref: null
    before_artifact_hash: null
    after_artifact_ref: null
    after_artifact_hash: null
    applied_change_ref: null
    evidence_refs: []
    eval_refs: []
    check_refs: []
    provenance_ref: null
    rollback_ref: null
    capability_delta: none
    side_effect_delta: none
    immutable_core_touched: false
    promoted_at: null
    non_authority: true
```

## Non-goals

This spec does not define module code, host code, manifest parsing, a
deterministic checker implementation, module registry behavior, Module Host
runtime, PubPunk runtime, workspace creation, instruction generation,
behavior-artifact writing, adapter behavior, publishing behavior, metrics
collection, credential access, gate writing, proofpack writing, or acceptance
claims.

## Validator command

```sh
scripts/check.sh module-control-plane
```

For the focused overlay, user-local artifact, tuning proposal, and promotion
boundary marker check:

```sh
scripts/check.sh module-behavior-boundaries
```

For the current aggregate control-plane marker suite:

```sh
scripts/check.sh module-control-suite
```

The command is a read-only marker check over this spec and
`docs/product/MODULE-CONTROL-PLANE.md`. It does not parse manifests, resolve
behavior artifacts, run reflection, write config, promote artifacts, activate
PubPunk runtime, or claim acceptance.
