---
id: docs_modules_pubpunk_control_manifest
kind: module-doc
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
related_docs:
  - docs/modules/pubpunk.md
  - docs/modules/pubpunk-article-hook-tuning-fixture.md
  - docs/modules/pubpunk-workspace-instructions.md
  - docs/product/MODULE-CONTROL-PLANE.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/product/MODULE-HOST-CONTRACT.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/module-control-plane.v0.1.md
  - evals/specs/pubpunk-control-manifest.v0.1.md
  - evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Control Manifest

## Purpose

Define the first PubPunk-specific module control manifest fixture.

This fixture applies the Module Control Plane contract to PubPunk without
activating PubPunk module execution, reflection, publishing, metrics, adapters,
credential access, receipt writing, event writing, gate writing, proofpack
writing, or acceptance authority.

## Boundary

This is an advisory fixture.

It is not an active manifest, parser input, registry entry, Module Host runtime
input, behavior-artifact writer input, reflection schedule, side-effect grant,
gate decision, proofpack, or acceptance claim.

## Manifest fixture

```yaml
module_control_manifest:
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
      promoted_by_ref: null
      source_hash: null
      review_after: 2026-06-21
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
    allowed_triggers:
      - none
      - manual
      - scheduled_proposal_only
    evidence_window_ref: null
    min_evidence_refs: null
    output_kind: tuning_proposal
    auto_apply: false
  provenance:
    artifact_set_hash: null
    latest_gate_ref: null
    latest_eval_ref: null
    latest_tuning_proposal_ref: null
  drift_budget:
    status: not_started
    since_ref: null
    cumulative_change: none
    re_audit_required: false
  non_authority: true
```

## Behavior artifact

The only current behavior artifact in this fixture is:

```text
docs/modules/pubpunk-workspace-instructions.md
```

That artifact orients PubPunk work around split explicit refs. It does not
replace contracts, evals, receipts, reports, gate decisions, proofpacks, or
Module Host validation.

Future PubPunk behavior artifacts, such as style profiles, channel templates,
publishing playbooks, or routing rules, must be added through a later bounded
goal and must cite this control manifest or its successor.

## Overlay posture

No current PubPunk user-local overlay is selected by this fixture.

Future PubPunk can use an explicit overlay stack:

```text
packaged defaults
  -> project/workspace behavior artifacts
  -> user-local behavior artifacts
  -> run-local override
```

The current project/workspace layer contains only
`docs/modules/pubpunk-workspace-instructions.md`.

User-local PubPunk overlays, such as a personal hook profile or channel style
preference, must stay `project_truth: false` unless exported and promoted
through a selected path. They must not grant publishing, metrics, adapter,
browser, credential, event, gate, proofpack, or acceptance authority.

## User-local customization posture

No current PubPunk user-local behavior artifact is selected by this fixture.

Future PubPunk user-local artifacts may include private hook profiles, style
profiles, prompt addenda, template overrides, routing preferences, or workflow
preferences for one operator or installation.

They must be local/private by default, must stay outside repo truth, must not
contain secrets or sensitive personal data, and must not include executable
code, scripts, adapters, browser automation, credential access, publishing,
metrics collection, event writing, gate writing, proofpack writing, or
acceptance behavior.

Exporting a PubPunk user-local artifact into project/workspace behavior requires
a behavior artifact promotion packet.

## Article hook tuning fixture posture

`docs/modules/pubpunk-article-hook-tuning-fixture.md` shows a concrete
non-applying example of a PubPunk article hook tuning chain.

That fixture demonstrates request, evidence, tuning proposal, user-local hook
profile, promotion packet, and resolved behavior set shapes. It is not selected
as current PubPunk behavior, does not contain real article metrics or external
research, and does not activate user-local config writing, publishing,
adapters, browser automation, gates, proofpacks, or acceptance behavior.

## Capability posture

The fixture grants no capabilities.

It explicitly denies:

- external publishing;
- credential reads;
- adapter invocation;
- browser automation;
- metrics collection;
- direct event-log writes;
- final decision writes;
- proofpack writes.

If a later PubPunk slice needs `read_workspace_metadata`,
`resolve_connector_profile`, receipt writing, operation-evidence writing, or
receipt/evidence event handoff behavior, that slice must continue to use its
own scoped packet, host validation, and eval evidence. This control manifest
does not grant those capabilities.

## Reflection posture

Reflection is disabled:

```yaml
reflection:
  trigger: none
  auto_apply: false
```

Future reflection may only produce a tuning proposal. It must not apply
changes, mutate behavior artifacts, grant capabilities, publish, collect
metrics, invoke adapters, write receipts, write events, write gate decisions,
write proofpacks, or claim acceptance.

## Manual tuning posture

The fixture lists `manual` as an allowed future trigger, but its current trigger
remains `none`.

A live operator request such as "review recent articles and tune the next hook"
would be a manual tuning request, not PubPunk self-modification. The request
would need explicit local evidence refs, optional metrics refs, optional
external research refs only when granted, a tuning proposal, and an approval or
gate promotion ref before any behavior artifact can change.

This fixture does not let PubPunk rewrite
`docs/modules/pubpunk-workspace-instructions.md`, external style profiles,
prompts, skills, templates, channel playbooks, or configs by itself.

## Promotion posture

No current PubPunk behavior artifact promotion is selected by this fixture.

If a future PubPunk tuning proposal is approved, promotion still needs a
separate behavior artifact promotion packet. That packet must record the target
artifact, artifact location kind, before/after refs or hashes, applied change,
evidence, eval/check status, provenance, rollback, and capability/side-effect
delta.

For external publishing workspace artifacts, the promotion packet must cite an
explicit workspace, receipt, or local config ref. It must not treat untracked
external prompts, styles, templates, or channel playbooks as hidden project
truth.

## Tuning proposal readiness

No current PubPunk tuning proposal is selected by this fixture.

The first tuning proposal should wait for concrete evidence refs, such as
receipts, reports, eval outputs, or repeated operator observations. Without
evidence, a proposal remains an example or finding, not a candidate for
promotion.

## Non-goals

This fixture does not add:

- Rust code;
- manifest parsing;
- deterministic checker implementation;
- PubPunk module execution through the Punk-owned Module Host;
- Module Host runtime;
- background reflection;
- behavior-artifact writing;
- `.punk` runtime state;
- workspace initialization;
- filesystem scanning;
- draft body reads;
- API calls;
- browser automation;
- credential access;
- adapter invocation;
- external publishing;
- metrics collection;
- PubPunk-owned receipt writing;
- PubPunk-owned operation-evidence writing;
- PubPunk-owned event-log writing;
- gate writing;
- proofpack writing;
- acceptance claims.

## Validation

Run the read-only fixture marker check with:

```sh
scripts/check.sh pubpunk-control-manifest
```

The check verifies that this fixture and
`evals/specs/pubpunk-control-manifest.v0.1.md` keep the advisory PubPunk
control manifest, empty grants, denied side effects, disabled reflection,
non-current tuning, user-local, hook fixture, and promotion boundaries visible
without activating PubPunk module execution behavior.
