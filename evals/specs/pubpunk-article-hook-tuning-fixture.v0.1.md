# PubPunk Article Hook Tuning Fixture v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the non-applying PubPunk article hook
tuning fixture.

This spec checks the full example chain from operator request to resolved
behavior set without activating PubPunk runtime, user-local config writing,
metrics collection, external research, publishing, adapters, browser
automation, receipts, events, gates, proofpacks, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-HOOK-TUNING-001: fixture is non-applying

The fixture must state that refs are example-only and that it does not select a
real PubPunk user-local artifact, hook profile, promotion, runtime behavior, or
configuration change.

### PUBPUNK-HOOK-TUNING-002: operator request is explicit

The manual tuning request must record `trigger: manual`, module id, operator
request ref, active work ref, target artifact refs, requested output kind, and
`auto_apply: false`.

### PUBPUNK-HOOK-TUNING-003: evidence limitations are explicit

The evidence packet and tuning proposal must cite local evidence refs, metrics
refs, and external research refs or record limitations. Example refs must be
marked as not actual measurements or research.

### PUBPUNK-HOOK-TUNING-004: tuning proposal is complete

The tuning proposal must cite module id, manifest ref, target artifact refs,
proposed change kind, evidence refs, eval refs, capability status,
immutable-core status, side-effect status, drift budget impact, rollback ref,
and approval ref.

### PUBPUNK-HOOK-TUNING-005: user-local hook profile is private

The user-local behavior artifact must use `authority: local_private`,
`owner_scope: single_operator`, `layer: user_local`,
`storage_policy: local_private_outside_repo`, `exported_to_project: false`,
and `project_truth: false`.

### PUBPUNK-HOOK-TUNING-006: user-local hook profile is safe

The user-local behavior artifact must record `contains_secrets: false`,
`contains_private_data: false`, `executable_code_allowed: false`,
`capability_delta: none`, `side_effect_delta: none`, and
`immutable_core_touched: false`.

### PUBPUNK-HOOK-TUNING-007: promotion packet records provenance and rollback

The promotion packet must cite the tuning proposal, approval, promotion path,
target artifact, artifact location kind, before/after refs or hashes,
evidence refs, eval refs, provenance ref, rollback ref, capability delta,
side-effect delta, and immutable-core status.

### PUBPUNK-HOOK-TUNING-008: resolved behavior set cites layers

The resolved behavior set must cite packaged defaults, project/workspace,
user-local, and run-local layers with artifact refs or explicit limitations.
It must cite provenance refs and record no capability or side-effect delta.

### PUBPUNK-HOOK-TUNING-009: no capability escalation

The fixture must not grant external publishing, metrics collection, credential
access, adapter invocation, browser automation, event writing, gate writing,
proofpack writing, or acceptance authority.

### PUBPUNK-HOOK-TUNING-010: no hidden project truth

The fixture must not treat user-local hooks, prompts, style profiles, templates,
or routing preferences as project truth unless exported through a behavior
artifact promotion packet. The provided example must remain advisory and
non-authoritative.

## Minimal fixture shape

```yaml
pubpunk_article_hook_tuning_fixture:
  request_ref: manual_tuning_pubpunk_article_hooks_fixture_v0_1
  evidence_ref: evidence_pubpunk_article_hooks_fixture_v0_1
  tuning_proposal_ref: tuning_proposal_pubpunk_article_hooks_fixture_v0_1
  user_local_artifact_ref: user_local_pubpunk_article_hook_profile_fixture_v0_1
  promotion_ref: behavior_promotion_pubpunk_article_hooks_fixture_v0_1
  resolved_behavior_ref: resolved_behavior_pubpunk_article_hooks_fixture_v0_1
  selected_as_current_behavior: false
  runtime_activated: false
  writer_activated: false
  publishing_activated: false
  metrics_collection_activated: false
  external_research_activated: false
  non_authority: true
```

## Validator command

Run the read-only fixture validator with:

```bash
scripts/check.sh pubpunk-hook-tuning-fixture
```

The validator checks for required fixture markers and rejects obvious activation
markers such as enabled auto-apply, selected current behavior, runtime
activation, writer activation, publishing activation, metrics activation,
external research activation, secrets, private data, or executable code.

## Non-goals

This spec does not define module code, host code, manifest parsing, config
resolving, user-local config writing, deterministic checker implementation,
PubPunk runtime, Module Host runtime, workspace creation, filesystem scanning,
article reads, real metrics collection, external research execution, adapter
behavior, browser automation, credential access, publishing behavior, receipt
writing, event writing, gate writing, proofpack writing, or acceptance claims.
