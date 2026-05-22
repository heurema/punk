---
id: docs_modules_pubpunk_article_hook_tuning_fixture
kind: module-fixture
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
related_docs:
  - docs/modules/pubpunk.md
  - docs/modules/pubpunk-control-manifest.md
  - docs/modules/pubpunk-workspace-instructions.md
  - docs/product/MODULE-CONTROL-PLANE.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
  - evals/specs/pubpunk-control-manifest.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Article Hook Tuning Fixture

## Purpose

Show one concrete, non-applying example of operator-triggered PubPunk
customization.

The fixture models this chain:

```text
operator request
  -> evidence refs
  -> tuning proposal
  -> user-local hook profile
  -> promotion packet
  -> resolved behavior set
```

## Boundary

This is an advisory fixture only.

It does not read real articles, collect metrics, run external research, write a
user-local config file, rewrite prompts, rewrite skills, rewrite
`docs/modules/pubpunk-workspace-instructions.md`, publish, invoke adapters,
open browsers, read credentials, write receipts, write events, write gate
decisions, write proofpacks, or claim acceptance.

All refs below are example-only unless they point to repo docs. They are not
evidence that the described analysis has happened.

## Scenario

The operator is writing articles with PubPunk-like guidance and notices that
recent posts lack a strong hook.

The operator asks:

```text
Review the last two weeks of posts, look at which hooks worked, include metrics
where available, check external writing research if allowed, and propose a hook
profile I can use for the next posts.
```

This request should produce a proposal first. It should not silently update
module behavior.

## Manual tuning request

```yaml
manual_in_session_tuning_request:
  id: manual_tuning_pubpunk_article_hooks_fixture_v0_1
  status: proposed
  authority: non_authoritative
  trigger: manual
  module_id: pubpunk
  operator_request_ref: example-operator-request-pubpunk-article-hooks-v0-1
  active_work_ref: example-active-work-article-drafting-session-v0-1
  target_artifact_refs:
    - user-local:pubpunk/article-hook-profile
  local_evidence_refs:
    - example-local-evidence:last-14-days-posts-summary
    - example-local-evidence:hook-retention-comparison
  metrics_refs:
    - example-metrics:article-engagement-snapshot
  external_research_allowed: true
  external_research_refs:
    - example-research:writing-hooks-patterns
  requested_output_kind: tuning_proposal
  auto_apply: false
  approval_ref: null
  promotion_ref: null
  non_authority: true
```

## Evidence packet

```yaml
hook_tuning_evidence_packet:
  id: evidence_pubpunk_article_hooks_fixture_v0_1
  status: example_only
  authority: non_authoritative
  module_id: pubpunk
  local_evidence_refs:
    - example-local-evidence:last-14-days-posts-summary
    - example-local-evidence:hook-retention-comparison
  metrics_refs:
    - example-metrics:article-engagement-snapshot
  external_research_refs:
    - example-research:writing-hooks-patterns
  evidence_limitations:
    - example_refs_not_actual_measurements
    - no_real_articles_read_by_this_fixture
    - no_external_research_run_by_this_fixture
  privacy:
    contains_secrets: false
    contains_private_data: false
  non_authority: true
```

## Tuning proposal

```yaml
tuning_proposal:
  id: tuning_proposal_pubpunk_article_hooks_fixture_v0_1
  status: proposed
  authority: non_authoritative
  module_id: pubpunk
  manifest_ref: docs/modules/pubpunk-control-manifest.md
  trigger: manual
  operator_request_ref: example-operator-request-pubpunk-article-hooks-v0-1
  active_work_ref: example-active-work-article-drafting-session-v0-1
  target_artifact_refs:
    - user-local:pubpunk/article-hook-profile
  proposed_change_ref: example-proposed-change:pubpunk-user-local-hook-profile
  proposed_change_kind: new_artifact
  summary: "Add a user-local hook profile that starts posts from a concrete operator pain before naming the method."
  evidence_refs:
    - example-local-evidence:last-14-days-posts-summary
    - example-local-evidence:hook-retention-comparison
    - example-metrics:article-engagement-snapshot
    - example-research:writing-hooks-patterns
  local_evidence_refs:
    - example-local-evidence:last-14-days-posts-summary
    - example-local-evidence:hook-retention-comparison
  metrics_refs:
    - example-metrics:article-engagement-snapshot
  external_research_refs:
    - example-research:writing-hooks-patterns
  evidence_limitations:
    - fixture_uses_example_refs_only
  eval_refs:
    - evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md
  check_refs: []
  capability_change_requested: false
  immutable_core_touched: false
  side_effect_policy_changed: false
  drift_budget_impact: narrow
  rollback_ref: example-rollback:remove-user-local-hook-profile
  expires_at: null
  approval_ref: example-operator-approval:article-hook-profile
  requested_gate_ref: null
  non_authority: true
```

## User-local hook profile

```yaml
user_local_behavior_artifact:
  id: user_local_pubpunk_article_hook_profile_fixture_v0_1
  status: active
  authority: local_private
  module_id: pubpunk
  owner_scope: single_operator
  layer: user_local
  artifact_kind: hook_profile
  artifact_ref: user-local:pubpunk/article-hook-profile
  artifact_hash: example-hash-not-computed
  storage_ref: example-local-config:pubpunk/user/article-hook-profile
  storage_policy: local_private_outside_repo
  source_request_ref: manual_tuning_pubpunk_article_hooks_fixture_v0_1
  tuning_proposal_ref: tuning_proposal_pubpunk_article_hooks_fixture_v0_1
  promotion_ref: behavior_promotion_pubpunk_article_hooks_fixture_v0_1
  exported_to_project: false
  project_truth: false
  contains_secrets: false
  contains_private_data: false
  executable_code_allowed: false
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  provenance_ref: resolved_behavior_pubpunk_article_hooks_fixture_v0_1
  rollback_ref: example-rollback:remove-user-local-hook-profile
  review_after: 2026-06-21
  non_authority: true
```

Example hook profile content:

```text
Start from a concrete operator pain observed in recent work.
Name the broken expectation before naming the method.
Prefer a short failure story over an abstract claim.
Keep product claims out of the hook unless the post is explicitly product-led.
```

## Promotion packet

```yaml
behavior_artifact_promotion:
  id: behavior_promotion_pubpunk_article_hooks_fixture_v0_1
  status: promoted
  authority: non_authoritative
  module_id: pubpunk
  manifest_ref: docs/modules/pubpunk-control-manifest.md
  tuning_proposal_ref: tuning_proposal_pubpunk_article_hooks_fixture_v0_1
  approval_ref: example-operator-approval:article-hook-profile
  promotion_path_ref: example-selected-edit-path:user-local-config
  target_artifact_ref: user-local:pubpunk/article-hook-profile
  artifact_location_kind: user_local_config
  before_artifact_ref: null
  before_artifact_hash: null
  after_artifact_ref: user-local:pubpunk/article-hook-profile
  after_artifact_hash: example-hash-not-computed
  applied_change_ref: example-applied-change:create-user-local-hook-profile
  evidence_refs:
    - example-local-evidence:last-14-days-posts-summary
    - example-metrics:article-engagement-snapshot
    - example-research:writing-hooks-patterns
  eval_refs:
    - evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md
  check_refs: []
  provenance_ref: resolved_behavior_pubpunk_article_hooks_fixture_v0_1
  rollback_ref: example-rollback:remove-user-local-hook-profile
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  promoted_at: 2026-05-21
  non_authority: true
```

`status: promoted` here is example-only. It does not select a real PubPunk
user-local artifact.

## Resolved behavior set

```yaml
resolved_behavior_set:
  id: resolved_behavior_pubpunk_article_hooks_fixture_v0_1
  status: example_only
  authority: non_authoritative
  module_id: pubpunk
  overlay_stack_ref: behavior_overlay_pubpunk_example_v0_1
  layers:
    - name: packaged_defaults
      artifact_refs: []
      limitations:
        - no_packaged_defaults_fixture
    - name: project_workspace
      artifact_refs:
        - docs/modules/pubpunk-workspace-instructions.md
    - name: user_local
      artifact_refs:
        - user-local:pubpunk/article-hook-profile
    - name: run_local
      artifact_refs: []
      limitations:
        - no_run_local_override
  artifact_set_hash: example-hash-not-computed
  provenance_refs:
    - behavior_promotion_pubpunk_article_hooks_fixture_v0_1
    - user_local_pubpunk_article_hook_profile_fixture_v0_1
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  non_authority: true
```

Future PubPunk outputs shaped by this example would cite
`resolved_behavior_pubpunk_article_hooks_fixture_v0_1` or record why the
resolved behavior set was unavailable.

## Validation

Run the read-only fixture check with:

```bash
scripts/check.sh pubpunk-hook-tuning-fixture
```

The check validates that this fixture and
`evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md` contain the required
request, evidence, tuning proposal, user-local artifact, promotion packet, and
resolved behavior set markers, and that they do not activate runtime, writers,
publishing, metrics, external research, secrets, or executable code.

## Non-goals

This fixture does not add:

- Rust code;
- manifest parsing;
- config resolving;
- user-local config writing;
- deterministic checker implementation;
- PubPunk runtime;
- Module Host runtime;
- background reflection;
- behavior-artifact writing;
- `.punk` runtime state;
- workspace initialization;
- filesystem scanning;
- article reads;
- real metrics collection;
- external research execution;
- API calls;
- browser automation;
- credential access;
- adapter invocation;
- external publishing;
- receipt writing;
- event writing;
- gate writing;
- proofpack writing;
- acceptance claims.
