---
id: docs_product_module_host_contract
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
review_after: 2026-06-19
canonical_for:
  - module-host-contract-stub
  - module-host-input-output-boundary
  - module-capability-envelope-boundary
  - module-side-effect-request-boundary
related_docs:
  - docs/product/MODULE-HOST.md
  - docs/product/MODULES.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/product/CONTRACT-SCHEMA.md
related_evals:
  - evals/specs/module-host-contract-stub.v0.1.md
supersedes: []
superseded_by: null
---

# Module Host Contract Stub

## Purpose

Define the smallest shared contract a future Module Host must preserve when it
invokes modules.

This stub exists before PubPunk or DevPunk skeleton work so the first module
does not invent its own host shape.

## Boundary

Use this contract as a design target for module conformance and skeleton work.

It is not Module Host runtime, a module loader, a manifest parser, a capability
grant engine, an event-log writer, a receipt writer, a provider orchestrator, a
CLI command, or gate/proof behavior.

## Contract flow

Use this flow for future module invocation:

```text
host prepares scoped input bundle
  -> host attaches denied-by-default capability envelope
  -> module returns advisory outputs
  -> host validates outputs before any run/event/proof path can use them
  -> gate later writes final decisions, when applicable
```

The module never owns project truth and never writes final decisions.

## Input bundle

Use this bundle shape for future host-scoped module inputs:

```yaml
module_input_bundle:
  module_id: pubpunk
  module_version_ref: null
  contract_ref: null
  run_ref: null
  project_ref: null
  module_goal_ref: null
  allowed_source_refs: []
  workspace_refs: []
  instruction_refs: []
  operation:
    kind: inventory_assessment
    requested_by_ref: work/goals/example.md
  privacy_policy_ref: null
  redaction_policy_ref: null
  capability_envelope_ref: module_capability_envelope_example
  expected_output_refs: []
  non_authority: true
```

This is a contract shape, not an active runtime schema.

## Capability envelope

Capabilities are denied by default.

```yaml
module_capability_envelope:
  id: module_capability_envelope_example
  status: advisory
  grants: []
  denies:
    - filesystem_read
    - filesystem_write
    - network
    - environment
    - secrets
    - process_shell
    - adapter_invoke
    - external_publish
    - metrics_collect
    - direct_event_log_write
    - final_decision_write
    - proofpack_write
  future_grant_requests: []
  non_authority: true
```

A module may ask for a future capability. It must not treat the request as a
grant.

## Advisory outputs

Use these output kinds for future module returns:

- `module_assessment`;
- `module_receipt`;
- `runner_aid`;
- `draft_ref`;
- `plan_ref`;
- `side_effect_request_proposal`;
- `conformance_finding`.

These outputs are advisory until the host validates them and later gate/proof
paths use them.

## Side-effect request proposal

Use side-effect request proposals for work that would need external or
authority-bearing behavior later:

```yaml
side_effect_request_proposal:
  id: side_effect_request_pubpunk_publish_example
  status: proposed
  requested_capability: external_publish
  intent_ref: null
  policy_ref: null
  payload_ref: null
  adapter_ref: null
  expected_receipt_fields:
    - actor
    - channel
    - target
    - timestamp
    - payload_digest
    - result_status
  blockers:
    - missing_policy
    - missing_adapter_receipt_shape
  non_authority: true
```

The proposal does not call APIs, publish, collect metrics, read credentials, or
write receipts.

## Event-log boundary

Modules must not mutate event logs directly.

A future host may record host-approved events only after validating module
outputs and capability boundaries. This stub does not define an event writer.

## PubPunk implication

PubPunk can be the first business module only when its first skeleton stays
inside this contract:

- explicit input bundle;
- empty capability grants unless separately selected;
- publishing and metrics denied;
- side effects represented as proposals only;
- outputs advisory;
- no runtime activation.

## Dry-run implication

Before PubPunk skeleton work, a trivial read-only module dry run may use this
contract to test the Module Conformance Packet without publishing urgency.

The dry run should validate the process, not create a new product module.

## Current documentation boundary

Documented now:

- input bundle shape;
- capability envelope shape;
- advisory output set;
- side-effect request proposal shape;
- event-log boundary;
- PubPunk skeleton implication;
- read-only dry-run implication.

Not active now:

- Module Host runtime;
- module loading;
- manifest parsing;
- capability enforcement;
- event writing;
- module invocation;
- adapter invocation;
- external publishing;
- metrics collection;
- credential access;
- receipt writing;
- gate/proof behavior.
