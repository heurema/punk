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
- active-core eval definitions and deterministic reports

The core must not own:

- provider-specific orchestration
- module-specific business logic
- marketplace behavior
- hidden memory
- unbounded autonomy
- hosted eval or tracing truth

## Universal lifecycle

All modules use the same grammar:

```text
Goal -> Contract -> Run -> Receipt -> ModuleAssessment -> DecisionObject -> Proofpack
```

`ModuleAssessment` is advisory evidence.

`DecisionObject` is final and is written only by `gate`.

## Workspace activation model

A crate or folder can exist before it is active.

Statuses:

- `active-core`
- `incubating`
- `parked`
- `retired`

The public CLI must only expose active capabilities.

## Project memory plane

Project memory has four repo-tracked truth surfaces:

- `work/` — goals, reports, generated views
- `knowledge/` — product truth, architecture truth, decisions, research, ideas
- `docs/adr/` — tracked architecture decisions
- `public/` — public-build narrative and receipts

Runtime and derived data live under `.punk/`.

### Knowledge Vault architecture boundary

Knowledge Vault owns the repo-tracked knowledge artifact contract under `knowledge/`.

Retrieval is an inspectable advisory view over project memory. It can provide evidence to `plot`, `cut`, and `gate`, but it cannot approve work, execute work, or write final decisions.

Future storage and retrieval must preserve these boundaries:

- repo artifacts remain source inputs,
- `.punk/` indexes and views are derived and rebuildable,
- modules may assess memory relevance,
- modules cannot decide authority,
- adapters may invoke external systems,
- adapters cannot own project truth.

This does not change the lifecycle:

```text
plot -> cut -> gate
```

## Module host boundary

The module host is an architectural boundary, not an active plugin runtime.

A module is a Punk lifecycle participant. A plugin is only a possible future packaging or execution mechanism for a module.

The module host must remain Punk-owned: it invokes modules, validates module receipts and assessments, records allowed events, and leaves final decision writing to `gate`.

A future plugin runtime must not write final decisions, mutate the event log directly, create a hidden project memory store, bypass contract scope, bypass `gate`, or gain filesystem, network, environment, secret, process, shell, or publishing authority by default.

See `docs/product/MODULE-HOST.md` and `docs/adr/ADR-0010-defer-wasm-plugin-host.md`.

## Eval plane

Eval is not a later feature. Minimal evals are part of the core.

The initial eval plane is:

- local-first
- repo-stored
- deterministic-first
- fixture-based
- hard-gate plus scorecard
- Pydantic Evals-compatible later, but not dependent on Python/web/SaaS in the core path

The eval plane assesses evidence. It does not accept work. Only `gate` writes final decisions.

Phase 2 active-core evals must check:

- state transition validity
- append-only event writing
- `cut` scope and approval guards
- run receipt existence
- gate decision exclusivity
- proofpack artifact links and hashes
- report schema shape
- baseline regressions
- waiver ledgering
- parked capability isolation

See `docs/product/EVAL-PLANE.md` and `docs/adr/ADR-0008-eval-plane.md`.

## Flow controller

Commands are transitions over persisted state.

The LLM can suggest a next step, but runtime flow state decides whether the step is allowed.

## Research gate

Research is a governance layer before important architecture changes. It is not an execution mode and does not add a fourth runtime phase.

The lifecycle stays:

```text
plot -> cut -> gate
```

Research informs `plot` by providing prior-art evidence, failure modes, and recommended constraints.

Important contracts should cite research refs when the work changes core behavior, storage, module interfaces, flow, eval, adapters, or project memory.
