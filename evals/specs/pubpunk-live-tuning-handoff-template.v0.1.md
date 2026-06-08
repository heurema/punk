# PubPunk Live Tuning Handoff Template v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the PubPunk live tuning handoff template.

This spec checks that the template can capture one manual tuning session
without turning operator approval or blank fields into behavior mutation.

It does not activate PubPunk module execution, manifest parsing, behavior resolving,
user-local config writing, behavior-artifact writing, article reading, metrics
collection, external research execution, adapter invocation, publishing, event
writing, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### PUBPUNK-LIVE-HANDOFF-001: template is advisory

The template must mark itself non-authoritative and must not claim runtime,
gate, proofpack, or acceptance authority.

### PUBPUNK-LIVE-HANDOFF-002: session header cites control refs

The session header must cite PubPunk, the control manifest, the live tuning
runbook, request ref, active work ref, target behavior, and `auto_apply:
false`.

### PUBPUNK-LIVE-HANDOFF-003: evidence packet separates work from template

The evidence packet must record local evidence refs, metrics refs, external
research allowance, external research refs, evidence limitations, privacy, and
explicit flags showing the template itself performed no article reads, metrics
collection, or external research.

### PUBPUNK-LIVE-HANDOFF-004: proposal cites evidence and checks

The proposal must cite request, active work, target artifacts, evidence packet,
eval refs, check refs, capability change status, immutable-core status,
side-effect status, drift impact, rollback, approval, and auto-apply status.

### PUBPUNK-LIVE-HANDOFF-005: approval is not auto-apply

The decision record must include `approval_is_auto_apply: false`.

### PUBPUNK-LIVE-HANDOFF-006: decision route is explicit

The decision must be one of proposed, rejected, no-change,
approved-for-user-local, or approved-for-project-promotion.

### PUBPUNK-LIVE-HANDOFF-007: handoff records promotion boundary

The handoff must include proposed artifact location kind, promotion packet ref,
resolved behavior set ref, rollback ref, and non-authority status.

### PUBPUNK-LIVE-HANDOFF-008: no capability or side-effect delta by default

The handoff must default to `capability_delta: none`,
`side_effect_delta: none`, and `immutable_core_touched: false`.

### PUBPUNK-LIVE-HANDOFF-009: sensitive and executable content is blocked

The handoff must include `contains_secrets: false`,
`contains_private_data: false`, and `executable_code_allowed: false`.

### PUBPUNK-LIVE-HANDOFF-010: completion checklist is explicit

The template must include a completion checklist that requires refs,
limitations, no-auto-apply approval, no deltas, rollback, promotion packet
before project/workspace promotion, and resolved behavior provenance before
future outputs use tuned behavior.

### PUBPUNK-LIVE-HANDOFF-011: suite validation remains read-only

The template may require `scripts/check.sh module-control-suite`, but that
command remains read-only and does not parse the template, write config,
promote artifacts, or activate runtime behavior.

## Minimal template markers

```yaml
required_markers:
  - pubpunk_live_tuning_session
  - pubpunk_live_tuning_evidence_packet
  - pubpunk_live_tuning_proposal
  - pubpunk_live_tuning_decision
  - pubpunk_live_tuning_handoff
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
