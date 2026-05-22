# Module Tuning Handoff Template v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the generic Module Tuning Handoff
Template.

This spec checks that a module can capture one manual or scheduled
proposal-only tuning session without turning operator approval, natural-language
requests, or blank fields into behavior mutation.

It does not activate module runtime, Module Host runtime, manifest parsing,
overlay resolving, user-local config writing, behavior-artifact writing,
article reading, metrics collection, external research execution, adapter
invocation, publishing, event writing, gate writing, proofpack writing, or
acceptance claims.

## Required deterministic eval cases

### MODULE-TUNING-HANDOFF-001: template is advisory

The template must mark itself non-authoritative and must not claim runtime,
gate, proofpack, or acceptance authority.

### MODULE-TUNING-HANDOFF-002: session header captures origin

The session header must cite module id, control manifest ref, control plane
ref, validation runbook ref, request ref, request channel, trigger, active work
ref, target behavior, evidence plan, and `auto_apply: false`.

### MODULE-TUNING-HANDOFF-003: text and voice requests are not mutation

The template must state that natural-language requests, generated proposals,
and operator approval are not permission to apply behavior changes.

### MODULE-TUNING-HANDOFF-004: evidence packet separates work from template

The evidence packet must record local evidence refs, metrics refs, external
research allowance, external research refs, evidence limitations, privacy, and
explicit flags showing the template itself performed no evidence collection,
metrics collection, or external research.

### MODULE-TUNING-HANDOFF-005: proposal cites evidence and checks

The proposal must cite request, active work, target artifacts, evidence packet,
eval refs, check refs, capability change status, immutable-core status,
side-effect status, drift impact, rollback, approval, and auto-apply status.

### MODULE-TUNING-HANDOFF-006: approval is not auto-apply

The decision record must include `approval_is_auto_apply: false`.

### MODULE-TUNING-HANDOFF-007: decision route is explicit

The decision must be one of proposed, rejected, no-change,
approved-for-user-local, or approved-for-project-promotion.

### MODULE-TUNING-HANDOFF-008: handoff records promotion boundary

The handoff must include proposed artifact location kind, promotion packet ref,
resolved behavior set ref, rollback ref, and non-authority status.

### MODULE-TUNING-HANDOFF-009: no capability or side-effect delta by default

The handoff must default to `capability_delta: none`,
`side_effect_delta: none`, and `immutable_core_touched: false`.

### MODULE-TUNING-HANDOFF-010: sensitive and executable content is blocked

The handoff must include `contains_secrets: false`,
`contains_private_data: false`, and `executable_code_allowed: false`.

### MODULE-TUNING-HANDOFF-011: completion checklist is explicit

The template must include a completion checklist that requires module id,
request refs, request channel, evidence refs or limitations, metrics/research
refs when used, no-auto-apply approval, no deltas, rollback, promotion packet
before project/workspace promotion, and resolved behavior provenance before
future outputs use tuned behavior.

### MODULE-TUNING-HANDOFF-012: suite validation remains read-only

The template may require `scripts/check.sh module-control-suite`, but that
command remains read-only and does not parse the template, write config,
promote artifacts, or activate runtime behavior.

## Minimal template markers

```yaml
required_markers:
  - module_tuning_session
  - module_tuning_evidence_packet
  - module_tuning_proposal
  - module_tuning_decision
  - module_tuning_handoff
  - request_channel: text | voice | scheduled_proposal_only
  - trigger: manual | scheduled_proposal_only
  - auto_apply: false
  - approval_is_auto_apply: false
  - capability_delta: none
  - side_effect_delta: none
  - immutable_core_touched: false
  - contains_secrets: false
  - contains_private_data: false
  - executable_code_allowed: false
```

## Validator command

```sh
scripts/check.sh module-tuning-handoff-template
scripts/check.sh module-control-suite
```

The commands validate the current generic tuning handoff template and
control-plane marker suite. They do not read articles, collect metrics, run
research, write config, promote artifacts, activate module runtime, publish,
invoke adapters, write gates, write proofpacks, or claim acceptance.

## Non-goals

This spec does not define module code, host code, manifest parsing, overlay
resolving, runtime behavior, user-local config writing, behavior-artifact
writing, article reading, metrics collection, external research execution,
adapter behavior, browser automation, credential access, publishing behavior,
event writing, gate writing, proofpack writing, or acceptance claims.
