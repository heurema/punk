---
id: docs_modules_pubpunk_live_tuning_runbook
kind: module-runbook
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
related_docs:
  - docs/modules/pubpunk.md
  - docs/modules/pubpunk-control-manifest.md
  - docs/modules/pubpunk-article-hook-tuning-fixture.md
  - docs/modules/pubpunk-workspace-instructions.md
  - docs/product/MODULE-CONTROL-PLANE.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/pubpunk-live-tuning-runbook.v0.1.md
  - evals/specs/pubpunk-article-hook-tuning-fixture.v0.1.md
  - evals/specs/pubpunk-control-manifest.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Live Tuning Runbook

## Purpose

Define the manual operator path for live PubPunk behavior tuning.

Use this when an operator says something like:

```text
Review recent articles, identify which hooks worked, use metrics and external
research if allowed, and propose a better hook profile for the next posts.
```

The runbook keeps live tuning useful without letting PubPunk self-modify.

## Boundary

This is an advisory runbook only.

It is not runtime behavior, a manifest parser, behavior resolver, config
writer, evidence collector, metrics collector, research runner, adapter
invoker, publisher, gate writer, proofpack writer, or acceptance authority.

It does not read articles, collect metrics, run external research, write
user-local config, rewrite prompts, rewrite skills, publish, invoke adapters,
open browsers, read credentials, write receipts, write events, write gate
decisions, write proofpacks, or claim acceptance.

## Inputs

A live tuning session starts with an explicit operator request.

Minimum input shape:

```yaml
pubpunk_live_tuning_request:
  id: null
  module_id: pubpunk
  request_ref: null
  active_work_ref: null
  target_behavior:
    artifact_kind: hook_profile | style_profile | prompt_addendum | template_override | routing_rule | workflow_preference
    target_artifact_refs: []
  local_evidence_allowed: true
  metrics_allowed: false
  external_research_allowed: false
  expected_output: tuning_proposal
  auto_apply: false
  non_authority: true
```

If the operator asks for direct mutation, reinterpret the request as a proposal
request unless the current selected work explicitly authorizes a separate edit
or promotion path.

## Process

1. Capture the request.

Record the request ref, active work ref, target behavior kind, and target
artifact refs or limitations.

2. Build the evidence plan.

List local evidence refs first. Add metrics refs only when metrics are allowed
and retained. Add external research refs only when external research is allowed
by the work order and retained.

3. Produce an evidence packet.

The packet must distinguish actual evidence refs from limitations. If recent
articles, comments, metrics, or research were not inspected, say so directly.

4. Draft a tuning proposal.

The proposal must cite target artifact refs, evidence refs or limitations,
eval/check refs or limitations, capability change status, side-effect status,
immutable-core status, drift budget impact, rollback, and requested promotion
path.

5. Ask for operator approval.

Approval is promotion input only. It is not auto-apply authority.

6. Route the decision.

Use one of these outcomes:

```yaml
pubpunk_live_tuning_decision:
  status: proposed | rejected | no_change | approved_for_user_local | approved_for_project_promotion
  approval_ref: null
  tuning_proposal_ref: null
  next_artifact_ref: null
  promotion_required: true
  auto_apply: false
  non_authority: true
```

7. Record provenance.

Any future output shaped by the tuned behavior must cite the resolved behavior
artifact set or record an explicit limitation.

## Stop conditions

Stop before proposing a behavior change when:

- there is no request ref or active work ref;
- the target artifact is unknown;
- evidence refs are missing and no limitation is recorded;
- external research influenced the proposal but no retained research ref exists;
- metrics influenced the proposal but no retained metrics ref exists;
- the requested change grants capabilities or expands side effects;
- the request touches immutable core;
- the artifact would contain secrets, credentials, private tokens, or sensitive
  personal data;
- the artifact requires executable code, adapters, browser automation,
  publishing, metrics collection, event writing, gate writing, proofpack
  writing, or acceptance behavior;
- the operator asks PubPunk to silently rewrite its own behavior.

## Output packet

The minimum handoff packet for a live tuning session is:

```yaml
pubpunk_live_tuning_handoff:
  id: null
  status: proposed
  authority: non_authoritative
  module_id: pubpunk
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

For a reusable fill-in shape, use
`docs/modules/pubpunk-live-tuning-handoff-template.md`.
For a filled article-hook example, see
`docs/modules/pubpunk-article-hook-live-tuning-handoff-example.md`.

## Validation

Before treating a live tuning artifact as coherent, run:

```sh
scripts/check.sh pubpunk-live-tuning-pack
scripts/check.sh module-control-suite
```

This confirms that the current live tuning runbook, handoff template, filled
example, control-plane docs, and PubPunk fixtures still preserve the
proposal-only, no-runtime, no-auto-apply boundaries.

## Non-goals

This runbook does not add:

- Rust code;
- PubPunk module execution;
- Module Host runtime;
- manifest parsing;
- behavior resolver;
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
