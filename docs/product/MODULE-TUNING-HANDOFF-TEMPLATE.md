---
id: docs_product_module_tuning_handoff_template
kind: product-template
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
canonical_for:
  - module-tuning-handoff-template
  - module-manual-tuning-handoff-boundary
  - module-proposal-only-tuning-template
related_docs:
  - docs/product/MODULE-CONTROL-PLANE.md
  - docs/product/MODULE-CONTROL-VALIDATION.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/modules/pubpunk-live-tuning-handoff-template.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/module-tuning-handoff-template.v0.1.md
  - evals/specs/module-control-validation.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
supersedes: []
superseded_by: null
---

# Module Tuning Handoff Template

## Purpose

Provide a generic fill-in template for one module tuning session.

Use this when an operator asks any module to adjust behavior in text or voice
during active work, or when a scheduled proposal-only reflection pass produces
a candidate behavior change.

Module-specific templates, such as
`docs/modules/pubpunk-live-tuning-handoff-template.md`, may specialize this
shape but should preserve its no-auto-apply and non-authority boundaries.

## Boundary

This template is advisory only.

It is not an active handoff, runtime input, manifest parser input, config
writer input, overlay resolver input, behavior-artifact writer input, adapter
request, publishing request, event write request, gate decision, proofpack, or
acceptance claim.

Do not treat blank fields, natural-language requests, generated proposals, or
operator approval as permission to apply behavior changes.

## Session header

```yaml
module_tuning_session:
  id: null
  status: draft
  authority: non_authoritative
  module_id: null
  control_manifest_ref: null
  control_plane_ref: docs/product/MODULE-CONTROL-PLANE.md
  validation_runbook_ref: docs/product/MODULE-CONTROL-VALIDATION.md
  requested_at: null
  operator_ref: null
  request_ref: null
  request_channel: text | voice | scheduled_proposal_only
  trigger: manual | scheduled_proposal_only
  active_work_ref: null
  target_behavior:
    artifact_kind: null
    target_artifact_refs: []
  evidence_plan_ref: null
  auto_apply: false
  non_authority: true
```

## Evidence packet

```yaml
module_tuning_evidence_packet:
  id: null
  status: draft
  authority: non_authoritative
  session_ref: null
  local_evidence_refs: []
  metrics_refs: []
  external_research_allowed: false
  external_research_refs: []
  evidence_limitations: []
  privacy:
    contains_secrets: false
    contains_private_data: false
  no_local_evidence_collection_performed_by_template: true
  no_metrics_collection_performed_by_template: true
  no_external_research_performed_by_template: true
  non_authority: true
```

## Tuning proposal

```yaml
module_tuning_proposal:
  id: null
  status: draft
  authority: non_authoritative
  session_ref: null
  module_id: null
  trigger: manual | scheduled_proposal_only
  request_ref: null
  active_work_ref: null
  target_artifact_refs: []
  proposed_change_ref: null
  proposed_change_kind: patch | replacement | new_artifact | retirement | no_change
  summary: null
  evidence_packet_ref: null
  eval_refs:
    - evals/specs/module-tuning-handoff-template.v0.1.md
    - evals/specs/module-control-plane.v0.1.md
  check_refs:
    - scripts/check.sh module-tuning-handoff-template
    - scripts/check.sh module-control-suite
  evidence_limitations: []
  capability_change_requested: false
  immutable_core_touched: false
  side_effect_policy_changed: false
  drift_budget_impact: unknown
  rollback_ref: null
  approval_ref: null
  requested_gate_ref: null
  auto_apply: false
  non_authority: true
```

## Decision record

```yaml
module_tuning_decision:
  id: null
  status: proposed
  authority: non_authoritative
  session_ref: null
  tuning_proposal_ref: null
  decision: proposed | rejected | no_change | approved_for_user_local | approved_for_project_promotion
  approval_ref: null
  approval_is_auto_apply: false
  next_artifact_ref: null
  promotion_required: true
  non_authority: true
```

## Handoff packet

```yaml
module_tuning_handoff:
  id: null
  status: proposed
  authority: non_authoritative
  module_id: null
  session_ref: null
  control_manifest_ref: null
  request_ref: null
  active_work_ref: null
  evidence_packet_ref: null
  tuning_proposal_ref: null
  approval_ref: null
  target_artifact_refs: []
  proposed_artifact_location_kind: user_local_config | repo_tracked | external_workspace
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

## Completion checklist

Before a handoff can be treated as coherent:

- module id is present;
- request ref is present;
- request channel is recorded;
- active work ref is present or limitation is recorded;
- target artifact refs are present or limitation is recorded;
- evidence packet ref is present;
- evidence limitations are explicit;
- metrics refs are present when metrics shaped the proposal;
- external research refs are present when research shaped the proposal;
- tuning proposal ref is present;
- eval/check refs are present or limitations are explicit;
- approval ref is present only when approval happened;
- approval is not auto-apply authority;
- capability delta is `none`;
- side-effect delta is `none`;
- immutable core is not touched;
- no secrets, private data, or executable code are included;
- rollback ref is present or limitation is recorded;
- promotion packet is present before project/workspace promotion;
- resolved behavior set is cited before future outputs use tuned behavior.

## Validation

Run:

```sh
scripts/check.sh module-tuning-handoff-template
scripts/check.sh module-control-suite
```

The dedicated template check verifies this generic handoff template and spec.
The suite also checks the governing control-plane contract, validation runbook,
and current PubPunk fixtures.

## Non-goals

This template does not add:

- Rust code;
- CLI behavior;
- module runtime;
- Module Host runtime;
- manifest parsing;
- overlay resolving;
- user-local config writing;
- behavior-artifact writing;
- article reading;
- metrics collection;
- external research execution;
- browser automation;
- credential access;
- adapter invocation;
- external publishing;
- receipt writing;
- event writing;
- gate writing;
- proofpack writing;
- acceptance claims.
