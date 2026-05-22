# PubPunk Article Hook Live Tuning Handoff Example v0.1

Date: 2026-05-21
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic review cases for the filled, example-only PubPunk article
hook live tuning handoff.

This spec checks that the example uses the live tuning handoff template without
claiming real article reads, metrics collection, external research execution,
user-local config writing, runtime activation, or current PubPunk behavior.

## Required deterministic eval cases

### PUBPUNK-HOOK-LIVE-HANDOFF-001: example is non-authoritative

The example must mark session, evidence, proposal, decision, and handoff
records as non-authoritative and example-only.

### PUBPUNK-HOOK-LIVE-HANDOFF-002: session cites template and runbook

The session must cite the PubPunk control manifest, live tuning runbook, handoff
template, request ref, active work ref, target behavior, and `auto_apply:
false`.

### PUBPUNK-HOOK-LIVE-HANDOFF-003: evidence limitations are explicit

The evidence packet must contain example-only evidence refs plus explicit
limitations for no real article reads, no metrics collection, and no external
research execution.

### PUBPUNK-HOOK-LIVE-HANDOFF-004: proposal remains proposal-only

The proposal must cite the evidence packet, eval refs, check refs, rollback,
approval ref, capability status, side-effect status, immutable-core status, and
`auto_apply: false`.

### PUBPUNK-HOOK-LIVE-HANDOFF-005: approval is not auto-apply

The decision record must include `approval_is_auto_apply: false` and must not
select real current behavior.

### PUBPUNK-HOOK-LIVE-HANDOFF-006: handoff remains user-local

The handoff must use `proposed_artifact_location_kind: user_local_config`,
mark no capability delta, no side-effect delta, immutable core untouched, no
secrets, no private data, and no executable code.

### PUBPUNK-HOOK-LIVE-HANDOFF-007: hook profile is example content

The hook profile text must be labeled example content and must not be treated
as a current PubPunk behavior artifact.

### PUBPUNK-HOOK-LIVE-HANDOFF-008: completion checklist records limitations

The completion checklist must record example-only refs and state that project
or workspace promotion is not applicable for this user-local example.

### PUBPUNK-HOOK-LIVE-HANDOFF-009: validation remains read-only

The example may require `scripts/check.sh module-control-suite`, but that
command remains read-only and does not parse the example, write config, promote
artifacts, or activate runtime behavior.

## Required markers

```yaml
required_markers:
  - pubpunk_live_tuning_session
  - pubpunk_live_tuning_evidence_packet
  - pubpunk_live_tuning_proposal
  - pubpunk_live_tuning_decision
  - pubpunk_live_tuning_handoff
  - status: example_only
  - approval_is_auto_apply: false
  - proposed_artifact_location_kind: user_local_config
  - capability_delta: none
  - side_effect_delta: none
  - immutable_core_touched: false
  - contains_secrets: false
  - contains_private_data: false
  - executable_code_allowed: false
  - auto_apply: false
```

## Validator command

```sh
scripts/check.sh pubpunk-live-tuning-pack
scripts/check.sh module-control-suite
```

The commands validate the current live tuning and control-plane marker suites.
They do not read articles, collect metrics, run research, write config, promote
artifacts, activate PubPunk runtime, publish, invoke adapters, write gates,
write proofpacks, or claim acceptance.

## Non-goals

This spec does not define module code, host code, manifest parsing, behavior
resolving, runtime behavior, user-local config writing, behavior-artifact
writing, article reading, metrics collection, external research execution,
adapter behavior, browser automation, credential access, publishing behavior,
event writing, gate writing, proofpack writing, or acceptance claims.
