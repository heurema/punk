---
id: docs_modules_pubpunk_workspace_instructions
kind: module-doc
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-20
review_after: 2026-06-19
related_docs:
  - docs/modules/pubpunk.md
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-CONFORMANCE.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/INSTRUCTION-SOURCES.md
  - publishing/README.md
related_evals:
  - evals/specs/pubpunk-workspace-instruction-packet.v0.1.md
  - evals/specs/pubpunk-inventory-input-packet.v0.1.md
  - evals/specs/pubpunk-host-handoff.v0.1.md
supersedes: []
superseded_by: null
---

# PubPunk Workspace and Instructions

## Purpose

Define the smallest workspace and instruction packet for future PubPunk work.

This resolves the first PubPunk conformance blockers without activating
PubPunk runtime, Module Host runtime, filesystem reads, workspace creation,
adapters, publishing, metrics collection, receipt writing, or credential use.

## Selected workspace policy

PubPunk uses `split_explicit_refs`.

| Surface | Policy | Initial refs | Boundary |
|---|---|---|---|
| Punk public narrative artifacts | `repo_native` | `publishing/`, `publishing/posts/`, `publishing/publications/`, `publishing/metrics/`, `publishing/channels/` | Repo-tracked public-memory artifacts for this repo. |
| Project-specific operational publishing workspaces | `external_workspace` | `punk-publishing://project/<project-id>` or a caller-provided external workspace ref | Explicit pointer only; not hidden truth and not auto-discovered. |
| User-level shared publishing state | `global_punk` | none selected | Future-only; not selected for the first PubPunk work. |
| Project `.punk` publishing state | `project_punk` | none selected | Future-only; no `.punk` writes in this packet. |

The next PubPunk implementation slice must not rely on untracked local host
paths in repo-tracked artifacts. If an external workspace is needed, the work
order or future host input bundle must pass an explicit logical workspace ref.

## Initial instruction refs

Future PubPunk work should receive these thin source refs:

- `docs/modules/pubpunk.md`;
- `docs/modules/pubpunk-workspace-instructions.md`;
- `docs/product/MODULE-AUTHORING.md`;
- `docs/product/MODULE-CONFORMANCE.md`;
- `docs/product/MODULE-HOST-CONTRACT.md`;
- `docs/product/INSTRUCTION-SOURCES.md`;
- `publishing/README.md`.

These refs orient the executor. They do not replace contracts, evals, receipts,
reports, gate decisions, or proofpacks.

## Executor guidance

When asked to run PubPunk work, an executor should:

- identify the requested project and publishing workspace ref;
- keep repo-native `publishing/` refs separate from external workspace refs;
- use caller-provided metadata by default;
- list candidate refs, receipt refs, metrics refs, and gaps;
- avoid reading draft bodies unless a later work order grants that scope;
- avoid initializing a workspace unless a later work order grants that scope;
- represent publishing, metrics, adapter, browser, network, and credential work
  as side-effect request proposals only;
- return advisory assessment evidence, not final decisions.

## First PubPunk work boundary

The existing `punk-mod-pubpunk` crate contains a side-effect-free inventory
input packet and inventory assessment model over caller-provided publishing
metadata.

The input packet must carry explicit workspace, instruction, source,
capability, receipt-field, and optional token-cost refs before the assessment
model runs. It should not restart PubPunk from zero, add a public CLI, scan the
filesystem, generate drafts, publish externally, collect metrics, write
receipts, or activate Module Host runtime.

The current smoke evidence can hand the ready packet and advisory assessment to
existing Module Host preflight, envelope, and receipt proposal models. This is a
model-chain check only. It does not invoke a module, load plugins, initialize a
workspace, write receipts, publish, collect metrics, invoke adapters, or activate
Module Host runtime.

## Capability envelope for the next slice

Default grants:

- `assess_provided_inventory`, when the work order passes explicit metadata.

Default denies:

- filesystem read;
- filesystem write;
- network;
- environment;
- secrets;
- process shell;
- adapter invocation;
- external publishing;
- metrics collection;
- direct event-log writes;
- final decision writes;
- proofpack writes;
- acceptance claims.

A later slice may request scoped filesystem read over explicit publishing refs.
That request is not a grant in this packet.

## Advisory assessment fields

Use these fields for the first PubPunk receipt or assessment shape:

```yaml
pubpunk_assessment:
  module_id: pubpunk
  operation: assess_provided_inventory
  workspace_policy: split_explicit_refs
  workspace_ref: null
  allowed_source_refs: []
  capability_grants:
    - assess_provided_inventory
  denied_capabilities:
    - filesystem_read
    - filesystem_write
    - network
    - secrets
    - adapter_invoke
    - external_publish
    - metrics_collect
    - direct_event_log_write
    - final_decision_write
    - proofpack_write
  inventory_counts:
    candidates: null
    receipts: null
    metrics_snapshots: null
  gap_findings: []
  side_effect_requests: []
  token_cost_ref: null
  non_authority: true
```

This is an advisory shape, not an active schema or runtime receipt writer.

## Input packet checks

For the current code slice, the input packet blocks:

- non-canonical module id;
- workspace policy other than `split_explicit_refs`;
- missing required instruction refs;
- unsafe instruction, source, workspace, or token-cost refs;
- item refs not present in the allowed source refs;
- missing `assess_provided_inventory` grant;
- unsupported grants such as publishing, metrics, adapter, credential, gate, or
  proof behavior;
- raw post bodies or privacy policy that allows raw/private payloads;
- missing expected receipt fields.

These checks are advisory readiness checks only. They do not read the
referenced files, create workspaces, collect token metrics, publish, or write
receipts.

## Non-goals

This packet does not activate:

- PubPunk runtime;
- Module Host runtime;
- generated instruction views;
- workspace initialization;
- filesystem scanning or draft body reads;
- public CLI behavior;
- provider orchestration;
- adapter invocation;
- external publishing;
- metrics collection;
- credential access;
- receipt writing;
- gate decisions;
- proofpack writing;
- acceptance claims.
