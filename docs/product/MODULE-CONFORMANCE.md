---
id: docs_product_module_conformance
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-21
review_after: 2026-06-19
canonical_for:
  - module-conformance-packet
  - module-conformance-review-boundary
  - module-readiness-finding-boundary
related_docs:
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONTROL-PLANE.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/MODULES.md
  - docs/product/MODULE-HOST.md
  - docs/product/DELIBERATION-BUDGET.md
  - docs/product/INSTRUCTION-SOURCES.md
related_evals:
  - evals/specs/module-authoring-baseline.v0.1.md
  - evals/specs/module-control-plane.v0.1.md
  - evals/specs/module-conformance-packet.v0.1.md
  - evals/specs/module-host-contract-stub.v0.1.md
supersedes: []
superseded_by: null
---

# Module Conformance Packet

## Purpose

Define the reviewable packet that checks whether a proposed module follows the
Module Authoring Baseline before implementation work starts.

This exists so future module work does not rely on an executor remembering
rules from context. A module that lacks this packet is not ready for
implementation unless the work report records an explicit no-packet rationale
and its limitations.

## Boundary

Use this packet as advisory readiness evidence only.

It is not a runtime validator, manifest parser, module registry, Module Host
feature, gate decision, proofpack, receipt writer, workspace initializer, or
public CLI command.

## Packet shape

Use this shape for each module candidate:

```yaml
module_conformance_packet:
  id: module_conformance_pubpunk_v0_1
  status: advisory
  authority: non_authoritative
  module_id: pubpunk
  module_ref: docs/modules/pubpunk.md
  baseline_version: module-authoring-baseline.v0.1
  baseline_ref: docs/product/MODULE-AUTHORING.md
  control_plane_ref: docs/product/MODULE-CONTROL-PLANE.md
  host_contract_ref: docs/product/MODULE-HOST-CONTRACT.md
  eval_refs:
    - evals/specs/module-authoring-baseline.v0.1.md
    - evals/specs/module-control-plane.v0.1.md
    - evals/specs/module-conformance-packet.v0.1.md
    - evals/specs/module-host-contract-stub.v0.1.md
  lifecycle_target: parked
  purpose_summary: "Publishing and editorial workflows."
  non_goals:
    - "No external publishing."
    - "No adapter invocation."
    - "No final decision writing."
  workspace_policy:
    selected: external_workspace
    source_of_truth_refs: []
    initializer_status: not_selected
  instruction_policy:
    source_refs: []
    generated_views: deferred
  control_plane:
    manifest_ref: null
    behavior_artifact_refs: []
    tuning_proposal_refs: []
    reflection_trigger: none
    auto_apply: false
  deliberation:
    budget_class: standard
    report_ref: null
    limitation: "Deliberation packet not attached yet."
  capabilities:
    granted: []
    denied:
      - direct_event_log_write
      - final_decision_write
      - proofpack_write
      - external_publish
      - credential_read
      - adapter_invoke
  expected_outputs:
    - module_assessment
    - module_receipt
    - runner_aid
    - side_effect_request_proposal
  findings:
    - id: missing_workspace_source_refs
      severity: medium
      status: open
      question: "Where is the operational workspace source of truth?"
  readiness:
    ready_for_implementation: false
    ready_for_runtime_activation: false
    next_smallest_step: "Resolve open findings."
  non_authority: true
```

This is a documentation shape, not an active schema.

## Review use

Use the packet to answer one question:

```text
Can this module move into the next bounded implementation slice without
inventing new module rules?
```

The answer can be:

| Result | Meaning |
|---|---|
| `ready_for_docs_only_next` | More boundary docs or eval/spec work can proceed. |
| `ready_for_skeleton` | A minimal parked/incubating skeleton may be proposed. |
| `blocked_by_findings` | Open findings must be resolved first. |
| `not_selected` | The module is not the next selected work item. |

These results are advisory. Only later selected gate/proof paths can make final
acceptance claims.

Before a packet returns `ready_for_skeleton`, it should cite
`docs/product/MODULE-HOST-CONTRACT.md` or record an explicit no-contract
rationale.

## Findings

Use findings to keep uncertainty explicit:

| Severity | Use when |
|---|---|
| `low` | Missing detail does not block the next docs/eval step. |
| `medium` | Missing detail blocks implementation readiness. |
| `high` | The module would violate a Punk law, module boundary, or authority boundary. |

Common findings:

- missing baseline version;
- missing host contract ref;
- missing workspace policy;
- missing empty-workspace initializer decision;
- missing instruction refs;
- missing Deliberation Budget record;
- missing side-effect policy;
- undeclared credential or network need;
- implied direct event-log mutation;
- implied final decision or proof authority;
- unclear receipt/evidence requirement;
- no conformance eval refs.
- missing module control manifest when behavior artifacts are used;
- behavior artifact tries to grant capabilities;
- reflection trigger is not explicit;
- tuning proposal lacks evidence or eval refs.

## DevPunk use

Treat a future DevPunk packet like any other module packet.

DevPunk gets no shortcut because it is about development. It must declare its
scope, non-goals, workspace policy, instruction refs, denied capabilities,
expected outputs, evidence needs, and conformance refs before implementation.

## PubPunk use

Treat PubPunk as urgent but not exempt.

Before PubPunk implementation starts, its packet should identify the publishing
workspace policy, instruction refs, metrics/publishing side-effect boundary,
receipt/evidence expectations, and the smallest safe next step.

## Current documentation boundary

Documented now:

- packet shape;
- advisory readiness results;
- finding severity;
- DevPunk and PubPunk application notes;
- eval/spec target.

Not active now:

- packet parser;
- deterministic checker;
- runtime enforcement;
- module registry;
- Module Host activation;
- DevPunk implementation;
- PubPunk implementation;
- workspace initialization;
- external publishing;
- metrics collection;
- credential reads;
- gate/proof behavior.
