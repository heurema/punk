---
id: docs_product_start_here
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-05-16
review_after: 2026-07-20
canonical_for:
  - product-entry-path
  - internal-orientation
  - active-operator-surface
  - current-build-posture
  - not-active-now-boundary
related_docs:
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/NORTH-STAR.md
  - docs/product/GLOSSARY.md
  - docs/product/PUNK-LAWS.md
related_adrs:
  - docs/adr/ADR-0004-dogfooding-from-day-zero.md
supersedes: []
superseded_by: null
---

# Start Here

## Purpose

This page is the internal entry point for working on `punk`.

It tells you what is active now, what is not active yet, and which documents own each kind of project truth.

## What Punk is

`punk` is a local-first bounded work kernel.

It gives every module the same lifecycle grammar:

```text
plot -> cut -> gate
```

- `plot` shapes work and creates a contract.
- `cut` executes bounded approved work.
- `gate` verifies evidence and writes the final decision.
- proof/proofpack makes the decision inspectable before acceptance.

Only `gate` writes final decisions.

## North star

Punk's high-level direction is defined in `docs/product/NORTH-STAR.md`.

Punk starts from today's code-first and spec-first practices because they are real and useful, but its long-term direction is to research, test, and gradually promote better primary artifacts for bounded software work: intent, contracts, specs, examples, invariants, evidence, decisions, and proof.

This direction does not activate future capabilities. Current active behavior remains governed by the build posture, active surface, roadmap, crate status, evals, and gate/proof boundaries below.

## Current build posture

Core first. Modules later.

Do not promote a capability into the active surface until:

1. it has a flow transition
2. it has at least one eval case
3. it writes inspectable artifacts
4. it does not bypass `gate`
5. it does not create a second source of truth

## Status vocabulary

Use these terms exactly:

- `active-core` — part of the stability target; must stay green
- `incubating` — exists and is tested, but not default/user-facing
- `parked` — boundary exists; minimal stub/docs only
- `retired` — removed or legacy-only

See `docs/product/DOCUMENTATION-MAP.md` and `docs/product/GLOSSARY.md`.

## Active now

The implemented CLI surface today is intentionally small:

- `punk init <project-id>`
- `punk init <project-id> --mode brownfield`
- `punk flow inspect`
- `punk publishing locate [--project-root <path>] [--json]`
- `punk eval run smoke`
- `punk eval run smoke --format json`

`punk init <project-id>` is active as the default greenfield Dogfooding Level 0 compact manual project-memory scaffold.
It records `project_id` and `entry_mode = greenfield`, then writes repo-tracked `.punk/memory/` durable memory, `.punk` marker/setup files, and thin source instruction entrypoints under `.punk/instructions/` with create-new/no-overwrite behavior.
Run it from the target project root; it initializes the current directory in place and does not create a new subdirectory named `<project-id>`.
For user projects, the default layout is compact `.punk/memory/`; root-level `work/`, `knowledge/`, `docs/adr/`, and `publishing/` are Punk repository dogfooding layout, not the default init layout.
`punk init <project-id> --mode brownfield` is active only as a brownfield entry scaffold.
It creates empty advisory reconstruction placeholders under `.punk/memory/reconstruction/` plus the same thin `.punk/instructions/` source entrypoints, records `reconstruction_status = not_started`, and does not scan, reconstruct, summarize, generate contracts/specs, accept claims, or write runtime state.
Instruction page-index behavior is active only as a deterministic advisory model and source-page scaffold. `.punk/views/instructions/page-index.json` is named as a future rebuildable view path, but `punk init` does not create `.punk/views/` or generated instruction views.
The Runtime Automation Spine is active only as narrow `punk-events` library slices: appending flow event drafts to `.punk/events/flow.jsonl` under an explicit initialized project root, plus appending a local receipt/evidence handoff event that links safe receipt and operation-evidence refs. It is local-only event evidence, not decision authority and not external automation; the handoff helper does not write receipt or operation-evidence artifacts.
The current Module Host active write slices are library-only: one writes exact caller-provided side-effect receipt bytes under an explicit `.punk/runs` target, and one writes exact caller-provided operation-evidence bytes for a successful receipt-writer result under an explicit `.punk/runs` target. They do not expose CLI behavior, invoke modules/adapters/policy engines/gate, mutate event logs, publish, comment, create pull requests, write proofpacks, or claim acceptance.
The first publishing-related CLI slice is active only as `punk publishing locate`. It reads `.punk/publishing.toml` and local-only `.punk/publishing.local.toml`, validates the logical workspace binding, and reports the external local publishing workspace. It does not write files, publish, open browsers, call APIs, read credentials, activate adapters, or run PubPunk automation.
Do not add inventory, drafting, planning, receipt, or publish behavior to active core or `punk-project` as a follow-up to this locator. That work belongs behind a future PubPunk/module-host boundary after the module host and side-effect policy are deliberately selected.
The `.punk/runtime/` tree and runtime evidence directories beyond that narrow event-log slice remain inactive; init does not activate flow persistence, contracts, run receipts, gate artifacts, proofpacks, or acceptance claims.
Brownfield reconstruction and grayfield reconciliation remain future modes.

The active target remains the stable core:

- core flow state
- append-only events
- minimal proof-bearing contract loop after flow is stable

The active trust surfaces are:

- project identity
- flow state machine
- event log
- eval harness
- contract lifecycle
- gate decision
- proofpack
- inspectable state
- project-memory links

## Not active now

Do not build or describe these as current operator paths:

- autonomous coding agent execution
- PubPunk publishing automation
- publishing execution beyond local workspace location
- provider adapters
- MCP integration
- knowledge embeddings as project truth
- plugin marketplace
- SaaS workspace
- cloud sync
- UI-first workflow

Parked/future ideas may exist in docs, research, or idea backlog, but must stay clearly labelled.

## Documentation system of record

Read in this order for architecture/product work:

1. `docs/product/PUNK-LAWS.md` — hard laws
2. `docs/product/ARCHITECTURE.md` — current structural boundaries
3. `docs/product/ROADMAP.md` — phase gates and promotion criteria
4. `docs/product/CRATE-STATUS.md` — crate/folder status
5. `docs/product/RESEARCH-GATE.md` — when research is required
6. `docs/product/RESEARCH-INTAKE.md` — how external ideas are classified
7. `docs/product/TELEMETRY.md` — local trust telemetry
8. `docs/product/EVAL-PLANE.md` — eval semantics
9. `docs/product/PROJECT-MEMORY.md` — repo-tracked memory and authority
10. `docs/product/PUBLIC-NARRATIVE.md` — public-build artifacts
11. `docs/product/DOC-GOVERNANCE.md` — documentation lifecycle and `DocImpact`
12. `docs/product/DOCUMENTATION-MAP.md` — canonical owner registry
13. `docs/product/GLOSSARY.md` — shared term authority

Use `docs/product/DOCUMENTATION-MAP.md` when editing docs or resolving conflicts.

## Documentation integrity

Documentation is part of project memory.

Meaningful changes should declare `DocImpact`, update the canonical truth owner, and move replaced truth into `superseded`, `archived`, or `retired` state instead of silently deleting it.

See:

- `docs/product/DOC-GOVERNANCE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `evals/specs/docs-consistency.v0.1.md`

## Research intake rule

Before adopting an idea from another project, classify it as exactly one of:

- `adopt`
- `defer`
- `park`
- `avoid`

Adopt only if it strengthens active-core trust surfaces.

Research is advisory until promoted through ADR, roadmap, goal/contract, implementation, eval, and proof.

See:

- `docs/product/RESEARCH-GATE.md`
- `docs/product/RESEARCH-INTAKE.md`
- `knowledge/research/`
- `knowledge/ideas/`

## Dogfooding from day zero

Before `punk` can execute work, the Punk repository can still track its own work in the root dogfooding layout:

- create goals under `work/goals/`
- write reports under `work/reports/`
- keep knowledge under `knowledge/`
- record architectural decisions under `docs/adr/`

Do not claim self-execution until the required dogfooding level is reached.

See `docs/product/DOGFOODING.md`.

## Public build from day zero

The public narrative starts before code automation.

Use `publishing/` to preserve stories, posts, manual publication receipts, and metrics snapshots.

PubPunk automation may later adopt this structure.

See `docs/product/PUBLIC-NARRATIVE.md`.

## First working rule

If a change touches core laws, flow semantics, eval policy, storage, module interfaces, project memory, knowledge retrieval, adapter boundaries, or public CLI contract, check Research Gate before implementation.

If a change promotes a future or parked capability, update roadmap/status docs and add eval implications.
