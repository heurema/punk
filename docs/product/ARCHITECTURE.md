# Architecture

## Target shape

```text
punk
├─ core laws
├─ flow controller
├─ eval plane
├─ project memory plane
├─ module host
├─ gate/proof
└─ adapters
```

## Core ownership

The core owns:

- project identity
- flow state
- rule snapshots
- contracts
- scope
- runs
- receipts
- decisions
- proofpacks
- event log
- inspectable views

The core must not own:

- provider-specific orchestration
- module-specific business logic
- marketplace behavior
- hidden memory
- unbounded autonomy

## Universal lifecycle

All modules use the same grammar:

```text
Goal
  -> Contract
    -> Run
      -> Receipt
      -> ModuleAssessment
      -> DecisionObject
      -> Proofpack
```

`ModuleAssessment` is advisory evidence. `DecisionObject` is final and is written only by `gate`.

## Workspace activation model

A crate or folder can exist before it is active.

Statuses:

- `active-core`
- `incubating`
- `parked`
- `retired`

The public CLI must only expose active capabilities.

## Project memory plane

Project memory has two repo-tracked sources:

- `work/` — goals, reports, generated views
- `knowledge/` — product truth, architecture truth, decisions, research, ideas

Runtime and derived data live under `.punk/`.

## Eval plane

Eval is not a later feature. Minimal evals are part of the core.

The initial eval plane is local, repo-stored, deterministic-first, and fixture-based.

## Flow controller

Commands are transitions over persisted state.

The LLM can suggest a next step, but runtime flow state decides whether the step is allowed.


## Research gate

Research is a governance layer before important architecture changes.

It is not an execution mode and does not add a fourth runtime phase.

The lifecycle stays:

```text
plot -> cut -> gate
```

Research informs `plot` by providing prior-art evidence, failure modes, and recommended constraints.

Important contracts should cite research refs when the work changes core behavior, storage, module interfaces, flow, eval, adapters, or project memory.
