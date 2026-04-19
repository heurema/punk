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

## First commands

```bash
punk init
punk flow inspect
punk start --from work/goals/<id>.md
punk plot approve <contract-id>
punk cut run <contract-id>
punk gate run <run-id>
punk gate proof <decision-id>
```
