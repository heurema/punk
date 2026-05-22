---
id: docs_modules_pubpunk_live_tuning_handoff_template
kind: module-template
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
related_docs:
  - docs/modules/pubpunk.md
  - docs/modules/pubpunk-live-tuning-runbook.md
  - docs/modules/pubpunk-control-manifest.md
  - docs/modules/pubpunk-article-hook-tuning-fixture.md
  - docs/product/MODULE-CONTROL-PLANE.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md
  - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Live Tuning Handoff Template

## Purpose

Provide a fill-in template for one manual PubPunk live tuning session.

Use this after an operator asks PubPunk to adjust behavior during active work,
for example when tuning article hooks from recent article evidence.

For a filled example, see
`docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md`.

## Boundary

This template is advisory only.

It is not an active handoff, runtime input, manifest parser input, config
writer input, behavior-artifact writer input, adapter request, publishing
request, gate decision, proofpack, or acceptance claim.

Do not treat blank fields, example refs, or operator approval as permission to
apply behavior changes.

## Session header

```yaml
pubpunk_live_tuning_session:
  id: null
  status: draft
  authority: non_authoritative
  module_id: pubpunk
  control_manifest_ref: docs/modules/pubpunk-control-manifest.md
  runbook_ref: docs/modules/pubpunk-live-tuning-runbook.md
  requested_at: null
  operator_ref: null
  request_ref: null
  active_work_ref: null
  target_behavior:
    artifact_kind: null
    target_artifact_refs: []
  auto_apply: false
  non_authority: true
```

## Evidence packet

```yaml
pubpunk_live_tuning_evidence_packet:
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
  no_article_reads_performed_by_template: true
  no_metrics_collection_performed_by_template: true
  no_external_research_performed_by_template: true
  non_authority: true
```

## Tuning proposal

```yaml
pubpunk_live_tuning_proposal:
  id: null
  status: draft
  authority: non_authoritative
  session_ref: null
  module_id: pubpunk
  trigger: manual
  request_ref: null
  active_work_ref: null
  target_artifact_refs: []
  proposed_change_ref: null
  proposed_change_kind: null
  summary: null
  evidence_packet_ref: null
  eval_refs:
    - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
    - evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md
  check_refs:
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
pubpunk_live_tuning_decision:
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
pubpunk_live_tuning_handoff:
  id: null
  status: proposed
  authority: non_authoritative
  module_id: pubpunk
  session_ref: null
  control_manifest_ref: docs/modules/pubpunk-control-manifest.md
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

- request ref is present;
- active work ref is present or limitation is recorded;
- target artifact refs are present or limitation is recorded;
- evidence packet ref is present;
- evidence limitations are explicit;
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
scripts/check.sh pubpunk-live-tuning-pack
scripts/check.sh module-control-suite
```

The dedicated live tuning pack check verifies this template, the live tuning
runbook, and the filled article-hook example. The suite also checks the
governing control-plane contract and PubPunk fixtures.

## Non-goals

This template does not add:

- Rust code;
- CLI behavior;
- PubPunk runtime;
- Module Host runtime;
- manifest parsing;
- behavior resolving;
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
