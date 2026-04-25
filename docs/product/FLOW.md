---
id: docs_product_flow
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-25
review_after: 2026-07-20
canonical_for:
  - flow-semantics
  - lifecycle-state-machine
  - command-transition-rules
  - expert-override-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
related_adrs:
  - docs/adr/ADR-0002-flow-and-eval-before-features.md
supersedes: []
superseded_by: null
---

# Flow

## Purpose

Flow prevents commands from becoming free actions.

A command is valid only if it is an allowed transition from the current persisted state.

## Core states

```text
project_initialized
goal_created
contract_drafted
awaiting_approval
approved
running
receipt_written
decision_written
proof_written
reported
closed
blocked
escalated
cancelled
```

## Core rule

```text
current_state + command -> allowed transition?
```

If not allowed, the runtime returns:

- current state
- required state
- next allowed commands
- optional expert override path

## Expert override

Overrides are not default behavior.

If supported, an override must:

- require a reason
- write an event
- appear in inspect output
- appear in proof context when relevant
- never bypass hard safety invariants

## Target commands

Current implemented CLI subset:

- `punk flow inspect`

Future target commands include:

- `punk init`
- `punk start --from work/goals/<id>.md`
- `punk plot approve <contract-id>`
- `punk cut run <contract-id>`
- `punk gate run <run-id>`
- `punk gate proof <decision-id>`

Future target commands are not current behavior until implemented and exposed through `punk-cli`.
