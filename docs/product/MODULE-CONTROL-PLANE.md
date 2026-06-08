---
id: docs_product_module_control_plane
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-06-08
review_after: 2026-06-21
canonical_for:
  - module-control-manifest-boundary
  - module-behavior-artifact-boundary
  - module-behavior-overlay-boundary
  - module-user-local-customization-boundary
  - module-tuning-proposal-boundary
  - module-behavior-artifact-promotion-boundary
  - propose-only-reflection-boundary
  - behavior-provenance-boundary
related_docs:
  - docs/product/MODULE-CONTROL-VALIDATION.md
  - docs/product/MODULE-TUNING-HANDOFF-TEMPLATE.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/product/INSTRUCTION-SOURCES.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/DELIBERATION-BUDGET.md
related_research:
  - knowledge/research/2026-05-21-adaptive-module-control-plane.md
related_ideas:
  - knowledge/ideas/2026-05-21-adaptive-module-control-plane.md
related_evals:
  - evals/specs/module-control-validation.v0.1.md
  - evals/specs/module-tuning-handoff-template.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
supersedes: []
superseded_by: null
---

# Module Control Plane

## Purpose

Define the contract for adjustable modules without allowing modules to
self-modify.

The control plane exists for modules such as PubPunk and future operator
modules where behavior may need frequent local tuning while the implementation
stays stable.

## Boundary

This document defines a documentation and eval contract only.

It is not a runtime schema, manifest parser, module registry, Module Host
runtime, reflection scheduler, behavior-artifact writer, adapter invoker,
gate writer, proofpack writer, or acceptance authority.

## Core rule

Editable behavior artifacts are behavior-changing artifacts.

They are not trusted merely because they are text, config, skills, prompts,
playbooks, profiles, routing rules, templates, or workflow preferences.

Behavior artifact changes should move through:

```text
evidence refs
  -> tuning proposal
  -> eval/check refs
  -> gate rationale
  -> provenance refs
  -> promoted behavior artifact
```

Reflection may propose improvements. It must not apply them.

## Definitions

| Term | Meaning |
|---|---|
| Module control manifest | Advisory declaration of the behavior artifacts, immutable core boundaries, capability requests, reflection status, and provenance refs for one module. |
| Behavior artifact | A source artifact that can influence module behavior without changing implementation code. |
| Behavior overlay stack | Ordered set of behavior artifact layers used to resolve packaged defaults, project/workspace guidance, user-local customization, and run-local overrides. |
| User-local behavior artifact | Private behavior artifact scoped to one operator or installation, not project truth unless explicitly exported and promoted. |
| Immutable core | Boundaries that must not be moved into editable artifacts, such as capability envelopes, host validation, gate interface, receipt emission, and proof authority. |
| Tuning proposal | Advisory proposal to change behavior artifacts, backed by evidence refs and eval/check refs. |
| Behavior artifact promotion packet | Advisory record for a deliberate behavior artifact promotion, including proposal, approval, before/after refs, checks, provenance, and rollback refs. |
| Reflection run | Bounded advisory analysis over explicit evidence that may emit tuning proposals only. |
| Behavior provenance | Refs and optional hashes that identify which behavior artifact set shaped a module output or proposal. |
| Drift budget | A review threshold for cumulative behavior artifact change since the last full review or gate promotion. |

## Contract flow

```text
module control manifest
  -> module output cites behavior artifact set
  -> evidence accumulates through receipts/reports/events/evals
  -> reflection or operator drafts tuning proposal
  -> proposal cites target artifacts, evidence, eval/check refs, and drift impact
  -> gate later may promote the behavior artifact change
```

The flow is advisory until a later selected gate/proof path promotes any
specific artifact.

## Operator-triggered tuning flow

Use this flow when a module needs tuning during active work, not only during a
scheduled reflection window.

Manual in-session tuning starts when the operator asks the module to adjust its
behavior in natural language, voice, or text while a workflow is already in
progress. That request is a manual tuning request. It is not permission for the
module to self-modify.

Example flow:

```text
operator notices weak article hooks
  -> operator requests a hook-tuning pass
  -> module gathers explicit local evidence refs
  -> module may gather external research refs only when granted
  -> module drafts a tuning proposal for a behavior artifact
  -> operator or future gate path approves or rejects promotion
  -> promoted artifact records provenance for later module outputs
```

The local evidence may include recent drafts, published receipts, comment
threads, metrics snapshots, review reports, or operator observations. External
research may be used only when the work order allows it and the resulting refs
are retained.

For an article-writing module, the proposal might say: "Recent posts with
concrete failure stories retained more attention than abstract process posts;
add a hook profile that starts from a painful operator moment before naming the
method." That is still a proposal. The module must not rewrite its own style
profile, prompt, skill, config, or instruction source merely because the
proposal is plausible.

User approval is promotion input, not auto-apply authority. At this boundary,
an accepted manual tuning request may only be written through an explicit
selected edit/promotion path that records evidence, review status, and
provenance.

Use this shape for future manual tuning requests:

```yaml
manual_in_session_tuning_request:
  id: manual_tuning_pubpunk_article_hooks_example_v0_1
  status: proposed
  authority: non_authoritative
  trigger: manual
  module_id: pubpunk
  operator_request_ref: null
  active_work_ref: null
  target_artifact_refs:
    - docs/modules/pubpunk-workspace-instructions.md
  local_evidence_refs: []
  metrics_refs: []
  external_research_allowed: false
  external_research_refs: []
  requested_output_kind: tuning_proposal
  auto_apply: false
  approval_ref: null
  promotion_ref: null
  non_authority: true
```

This is not an active request schema, background scheduler, evidence collector,
research runner, behavior-artifact writer, or approval mechanism.

## Module control manifest

Use this shape for future module control manifests:

```yaml
module_control_manifest:
  id: module_control_pubpunk_v0_1
  status: advisory
  authority: non_authoritative
  module_id: pubpunk
  module_ref: docs/modules/pubpunk.md
  lifecycle_target: incubating
  module_version_ref: null
  baseline_ref: docs/product/MODULE-AUTHORING.md
  host_contract_ref: docs/product/MODULE-HOST-CONTRACT.md
  control_plane_ref: docs/product/MODULE-CONTROL-PLANE.md
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
  capabilities:
    granted: []
    denied:
      - direct_event_log_write
      - final_decision_write
      - proofpack_write
      - external_publish
      - credential_read
      - adapter_invoke
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

This shape is not an active parser or runtime manifest.

## Behavior artifact refs

Behavior artifacts may include:

- instruction source pages;
- generated runner aids only when cited back to source refs;
- skills;
- playbooks;
- style or voice profiles;
- templates;
- routing rules;
- recurring workflow preferences;
- channel-specific operating notes.

Repo-tracked behavior artifacts must use repo-relative refs.

External workspace artifacts must be referenced through explicit workspace refs
or receipt refs. They must not become hidden project truth.

## Behavior overlay stack

Use an explicit overlay stack when a module needs user-local customization
without changing the stable implementation or upstream module defaults.

The stack separates where behavior comes from:

```text
immutable core
  -> packaged module defaults
  -> project/workspace behavior artifacts
  -> user-local behavior artifacts
  -> run-local override
```

The immutable core is not an overlay layer. It cannot be replaced by overlays.

Packaged module defaults are shipped with the module and should remain stable
across local tuning. Project/workspace artifacts are shared behavior sources for
one project or workspace. User-local artifacts are private customization for one
operator or installation. Run-local overrides are ephemeral and expire with the
current run unless promoted through the tuning/provenance path.

Use this shape for future overlay declarations:

```yaml
behavior_overlay_stack:
  id: behavior_overlay_pubpunk_example_v0_1
  status: advisory
  authority: non_authoritative
  module_id: pubpunk
  immutable_core_ref: docs/product/MODULE-CONTROL-PLANE.md
  layers:
    - name: packaged_defaults
      order: 10
      location_kind: module_package
      artifact_refs: []
      writable_by_module: false
      project_truth: false
    - name: project_workspace
      order: 20
      location_kind: repo_tracked | external_workspace
      artifact_refs:
        - docs/modules/pubpunk-workspace-instructions.md
      writable_by_module: false
      project_truth: true
    - name: user_local
      order: 30
      location_kind: user_local_config
      artifact_refs: []
      writable_by_module: false
      project_truth: false
    - name: run_local
      order: 40
      location_kind: ephemeral_run
      artifact_refs: []
      writable_by_module: false
      project_truth: false
      expires_at: null
  conflict_policy: later_layer_overrides_behavior_only
  capability_policy: overlays_cannot_grant_capabilities
  side_effect_policy: overlays_cannot_expand_side_effects
  provenance_required: true
  non_authority: true
```

The overlay stack is not a resolver, loader, parser, config writer, secret
store, sync protocol, or runtime authority.

Rules:

- overlays may tune behavior, style, templates, routing, and workflow
  preferences;
- overlays must not grant capabilities, expand side effects, read credentials,
  invoke adapters, publish, write gates, write proofpacks, or claim acceptance;
- user-local overlays must not be committed or treated as project truth unless
  explicitly exported and promoted through a selected path;
- run-local overrides must be cited as ephemeral and must not silently become
  future defaults;
- module outputs must cite the resolved behavior artifact set, including layer
  refs or explicit null/limitation values.

## User-local customization artifacts

Use this shape when an installed module needs private per-user tuning without
changing packaged defaults or shared project/workspace behavior artifacts.

```yaml
user_local_behavior_artifact:
  id: user_local_pubpunk_article_hooks_example_v0_1
  status: proposed | active | retired
  authority: local_private
  module_id: pubpunk
  owner_scope: single_operator | single_installation
  layer: user_local
  artifact_kind: hook_profile | style_profile | prompt_addendum | template_override | routing_rule | workflow_preference
  artifact_ref: null
  artifact_hash: null
  storage_ref: null
  storage_policy: local_private_outside_repo
  source_request_ref: null
  tuning_proposal_ref: null
  promotion_ref: null
  exported_to_project: false
  project_truth: false
  contains_secrets: false
  contains_private_data: false
  executable_code_allowed: false
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  provenance_ref: null
  rollback_ref: null
  review_after: null
  non_authority: true
```

This artifact shape is not a local config writer, sync protocol, secret store,
plugin loader, executable extension point, or runtime resolver.

Rules:

- a natural-language, voice, or text customization request may create a
  proposed user-local artifact only through an explicit selected edit path;
- the artifact must stay local/private and outside repo truth by default;
- the artifact must not contain secrets, credentials, private tokens, or
  sensitive personal data;
- executable code, scripts, provider hooks, adapters, browser automation, or
  side-effectful actions are not valid user-local behavior artifacts at this
  boundary;
- exporting a user-local artifact into a project/workspace layer requires a
  behavior artifact promotion packet;
- future module outputs shaped by the artifact must cite it through the
  resolved behavior artifact set or record an explicit limitation.

## Immutable core

Behavior artifacts must not redefine:

- capability envelopes;
- host validation;
- gate interface;
- receipt emission;
- proof authority;
- final decision ownership;
- direct event-log mutation policy;
- side-effect policy;
- credential access policy.

If a tuning proposal touches immutable core, it is a finding, not a behavior
artifact update.

## Tuning proposal

Use this shape for future behavior-artifact change proposals:

```yaml
tuning_proposal:
  id: tuning_proposal_pubpunk_example_v0_1
  status: proposed
  authority: non_authoritative
  module_id: pubpunk
  manifest_ref: null
  trigger: manual | scheduled_proposal_only
  operator_request_ref: null
  active_work_ref: null
  target_artifact_refs:
    - docs/modules/pubpunk-workspace-instructions.md
  proposed_change_ref: null
  proposed_change_kind: patch | replacement | new_artifact | retirement
  summary: "Adjust PubPunk channel guidance after repeated evidence."
  evidence_refs: []
  local_evidence_refs: []
  metrics_refs: []
  external_research_refs: []
  evidence_limitations:
    - insufficient_independent_evidence
  eval_refs: []
  check_refs: []
  capability_change_requested: false
  immutable_core_touched: false
  side_effect_policy_changed: false
  drift_budget_impact: unknown
  rollback_ref: null
  expires_at: null
  approval_ref: null
  requested_gate_ref: null
  non_authority: true
```

This is not a patch writer and not an approval.

## Rules

A tuning proposal is incomplete when it lacks:

- module id;
- trigger and request origin;
- target artifact refs;
- proposed change ref or explicit no-change rationale;
- evidence refs or explicit evidence limitation;
- external research and metrics refs when those inputs shaped the proposal;
- eval/check refs or explicit not-yet-tested limitation;
- capability change status;
- immutable core status;
- drift budget impact;
- requested promotion path.

An incomplete proposal may still be useful as an advisory finding. It must not
be promoted as current module guidance.

## Promotion boundary

Behavior artifact promotion is future selected work.

At this boundary, a proposal may only request promotion. Operator approval can
be recorded, but it is not enough by itself to make the new behavior current.

A later selected edit, gate path, or user-local config path must record what
changed and why before future module outputs may cite the new behavior artifact
set.

Until that selected path exists, promotion packets are advisory records only.
They must not:

- edit files directly;
- auto-apply a patch;
- grant capabilities;
- invoke adapters;
- publish externally;
- read credentials;
- mutate event logs;
- write final decisions;
- write proofpacks;
- claim acceptance.

Operator approval does not weaken this boundary. It only identifies that a
human or future gate path may deliberately promote the behavior artifact change
through a separate selected edit/promotion path.

Use this shape for future behavior artifact promotion packets:

```yaml
behavior_artifact_promotion:
  id: behavior_promotion_pubpunk_article_hooks_example_v0_1
  status: proposed | promoted | rejected | reverted
  authority: non_authoritative
  module_id: pubpunk
  manifest_ref: null
  tuning_proposal_ref: null
  approval_ref: null
  promotion_path_ref: null
  target_artifact_ref: docs/modules/pubpunk-workspace-instructions.md
  artifact_location_kind: repo_tracked | external_workspace | user_local_config
  before_artifact_ref: null
  before_artifact_hash: null
  after_artifact_ref: null
  after_artifact_hash: null
  applied_change_ref: null
  evidence_refs: []
  eval_refs: []
  check_refs: []
  provenance_ref: null
  rollback_ref: null
  capability_delta: none
  side_effect_delta: none
  immutable_core_touched: false
  promoted_at: null
  non_authority: true
```

A promotion packet is incomplete when it lacks:

- tuning proposal ref or explicit rationale for direct manual promotion;
- approval ref or explicit no-approval limitation;
- target artifact ref;
- artifact location kind;
- before and after artifact refs or hashes;
- evidence and eval/check refs or limitations;
- provenance ref;
- rollback ref or explicit irreversible-change warning;
- capability, side-effect, and immutable-core status.

For repo-tracked behavior artifacts, refs must be repo-relative. For external
workspace or user-local config artifacts, refs must go through explicit
workspace refs, receipt refs, or local config refs. They must not be hidden
project truth.

The promoted artifact set, not the proposal text, is what future module outputs
cite as behavior provenance.

## PubPunk fixture

PubPunk can use the contract as a fixture without activating module execution
or runtime behavior:

```yaml
module_control_manifest:
  id: module_control_pubpunk_fixture_v0_1
  status: advisory
  module_id: pubpunk
  module_ref: docs/modules/pubpunk.md
  behavior_artifacts:
    - ref: docs/modules/pubpunk-workspace-instructions.md
      kind: instruction_source
      authority: advisory
  immutable_core:
    - capability_envelope
    - host_validation
    - gate_interface
    - receipt_emission
    - proof_authority
  capabilities:
    granted: []
    denied:
      - external_publish
      - credential_read
      - adapter_invoke
      - metrics_collect
      - direct_event_log_write
      - final_decision_write
      - proofpack_write
  reflection:
    trigger: none
    output_kind: tuning_proposal
    auto_apply: false
  non_authority: true
```

This fixture does not publish, collect metrics, read credentials, invoke
adapters, write receipts, write events, or activate PubPunk module execution
through the Punk-owned Module Host.

## Non-goals

This document does not define:

- Rust code;
- active manifest parsing;
- runtime reflection;
- background scheduling;
- behavior-artifact writer implementation;
- Module Host runtime;
- module registry;
- adapter invocation;
- browser automation;
- credential access;
- external publishing;
- metrics collection;
- gate writer;
- proofpack writer;
- acceptance claim.

## Validation

Run the read-only contract marker check with:

```sh
scripts/check.sh module-control-plane
```

Run the focused overlay, user-local artifact, tuning proposal, and promotion
boundary marker check with:

```sh
scripts/check.sh module-behavior-boundaries
```

Run the full current control-plane marker suite with:

```sh
scripts/check.sh module-control-suite
```

The check verifies that this contract and
`evals/specs/module-control-plane.v0.1.md` keep the advisory control-plane,
overlay, user-local artifact, tuning proposal, and promotion boundaries
visible without activating runtime behavior.
