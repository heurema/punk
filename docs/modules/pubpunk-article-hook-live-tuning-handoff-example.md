---
id: docs_modules_pubpunk_article_hook_live_tuning_handoff_example
kind: module-example
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
related_docs:
  - docs/modules/pubpunk.md
  - docs/modules/pubpunk-live-tuning-runbook.md
  - docs/modules/pubpunk-live-tuning-handoff-template.md
  - docs/modules/pubpunk-article-hook-tuning-fixture.md
  - docs/modules/pubpunk-control-manifest.md
  - docs/product/MODULE-CONTROL-PLANE.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md
  - evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md
  - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Article Hook Live Tuning Handoff Example

## Purpose

Show one filled, example-only handoff for a manual PubPunk article hook tuning
request.

This example uses the shape from
`docs/modules/pubpunk-live-tuning-handoff-template.md` and the scenario from
`docs/modules/pubpunk-article-hook-tuning-fixture.md`.

## Boundary

This is an advisory example only.

It is not an active handoff, runtime input, user-local config, behavior
artifact writer input, gate input, proofpack, or acceptance claim.

No real articles were read. No metrics were collected. No external research
was run. No user-local artifact was written. No behavior was selected as
current PubPunk behavior.

## Session header

```yaml
pubpunk_live_tuning_session:
  id: example-session-pubpunk-article-hooks-v0-1
  status: example_only
  authority: non_authoritative
  module_id: pubpunk
  control_manifest_ref: docs/modules/pubpunk-control-manifest.md
  runbook_ref: docs/modules/pubpunk-live-tuning-runbook.md
  template_ref: docs/modules/pubpunk-live-tuning-handoff-template.md
  requested_at: 2026-05-21
  operator_ref: example-operator
  request_ref: example-operator-request-pubpunk-article-hooks-v0-1
  active_work_ref: example-active-work-article-drafting-session-v0-1
  target_behavior:
    artifact_kind: hook_profile
    target_artifact_refs:
      - user-local:pubpunk/article-hook-profile
  auto_apply: false
  non_authority: true
```

## Evidence packet

```yaml
pubpunk_live_tuning_evidence_packet:
  id: example-evidence-pubpunk-article-hooks-v0-1
  status: example_only
  authority: non_authoritative
  session_ref: example-session-pubpunk-article-hooks-v0-1
  local_evidence_refs:
    - example-local-evidence:last-14-days-posts-summary
    - example-local-evidence:hook-retention-comparison
  metrics_refs:
    - example-metrics:article-engagement-snapshot
  external_research_allowed: true
  external_research_refs:
    - example-research:writing-hooks-patterns
  evidence_limitations:
    - example_refs_only
    - no_real_articles_read_by_this_example
    - no_metrics_collection_performed_by_this_example
    - no_external_research_performed_by_this_example
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
  id: example-proposal-pubpunk-article-hooks-v0-1
  status: example_only
  authority: non_authoritative
  session_ref: example-session-pubpunk-article-hooks-v0-1
  module_id: pubpunk
  trigger: manual
  request_ref: example-operator-request-pubpunk-article-hooks-v0-1
  active_work_ref: example-active-work-article-drafting-session-v0-1
  target_artifact_refs:
    - user-local:pubpunk/article-hook-profile
  proposed_change_ref: example-proposed-change:pubpunk-user-local-hook-profile
  proposed_change_kind: new_artifact
  summary: "Add a user-local hook profile that starts posts from a concrete operator pain before naming the method."
  evidence_packet_ref: example-evidence-pubpunk-article-hooks-v0-1
  eval_refs:
    - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
    - evals/specs/pubpunk-live-tuning-handoff-template.v0.1.md
    - evals/specs/pubpunk-article-hook-live-tuning-handoff-example.v0.1.md
  check_refs:
    - scripts/check.sh module-control-suite
  evidence_limitations:
    - example_refs_only
    - no_real_articles_read_by_this_example
    - no_metrics_collection_performed_by_this_example
    - no_external_research_performed_by_this_example
  capability_change_requested: false
  immutable_core_touched: false
  side_effect_policy_changed: false
  drift_budget_impact: narrow
  rollback_ref: example-rollback:remove-user-local-hook-profile
  approval_ref: example-operator-approval:article-hook-profile
  requested_gate_ref: null
  auto_apply: false
  non_authority: true
```

## Decision record

```yaml
pubpunk_live_tuning_decision:
  id: example-decision-pubpunk-article-hooks-v0-1
  status: example_only
  authority: non_authoritative
  session_ref: example-session-pubpunk-article-hooks-v0-1
  tuning_proposal_ref: example-proposal-pubpunk-article-hooks-v0-1
  decision: approved_for_user_local
  approval_ref: example-operator-approval:article-hook-profile
  approval_is_auto_apply: false
  next_artifact_ref: user-local:pubpunk/article-hook-profile
  promotion_required: true
  non_authority: true
```

The decision is example-only. It does not approve any real user-local artifact.

## Handoff packet

```yaml
pubpunk_live_tuning_handoff:
  id: example-handoff-pubpunk-article-hooks-v0-1
  status: example_only
  authority: non_authoritative
  module_id: pubpunk
  session_ref: example-session-pubpunk-article-hooks-v0-1
  control_manifest_ref: docs/modules/pubpunk-control-manifest.md
  request_ref: example-operator-request-pubpunk-article-hooks-v0-1
  active_work_ref: example-active-work-article-drafting-session-v0-1
  evidence_packet_ref: example-evidence-pubpunk-article-hooks-v0-1
  tuning_proposal_ref: example-proposal-pubpunk-article-hooks-v0-1
  approval_ref: example-operator-approval:article-hook-profile
  target_artifact_refs:
    - user-local:pubpunk/article-hook-profile
  proposed_artifact_location_kind: user_local_config
  proposed_artifact_ref: user-local:pubpunk/article-hook-profile
  promotion_packet_ref: example-promotion-pubpunk-article-hooks-v0-1
  resolved_behavior_set_ref: example-resolved-behavior-pubpunk-article-hooks-v0-1
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  contains_secrets: false
  contains_private_data: false
  executable_code_allowed: false
  rollback_ref: example-rollback:remove-user-local-hook-profile
  auto_apply: false
  non_authority: true
```

## Example hook profile

```text
Start from a concrete operator pain observed in recent work.
Name the broken expectation before naming the method.
Prefer a short failure story over an abstract claim.
Keep product claims out of the hook unless the post is explicitly product-led.
```

The text above is example content. It is not a current PubPunk behavior
artifact.

## Completion checklist

- Request ref present: yes, example-only.
- Active work ref present: yes, example-only.
- Target artifact ref present: yes, example-only user-local ref.
- Evidence packet ref present: yes, example-only.
- Evidence limitations explicit: yes.
- Tuning proposal ref present: yes, example-only.
- Eval/check refs present: yes.
- Approval ref present: yes, example-only.
- Approval is not auto-apply authority: yes.
- Capability delta is `none`: yes.
- Side-effect delta is `none`: yes.
- Immutable core is untouched: yes.
- No secrets, private data, or executable code: yes.
- Rollback ref present: yes, example-only.
- Promotion packet ref present before project/workspace promotion: not
  applicable; this example routes to user-local only.
- Resolved behavior set cited before future outputs use tuned behavior: yes,
  example-only.

## Validation

Run:

```sh
scripts/check.sh pubpunk-live-tuning-pack
scripts/check.sh module-control-suite
```

The dedicated live tuning pack check verifies this example, the live tuning
runbook, and the handoff template. The suite also checks the governing
control-plane contract and PubPunk fixtures.

## Non-goals

This example does not add:

- Rust code;
- CLI behavior;
- PubPunk module execution;
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
