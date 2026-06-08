---
id: docs_product_modules
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-06-08
review_after: 2026-07-20
canonical_for:
  - module-boundaries
  - module-lifecycle
  - module-authority-boundary
  - parked-module-strategy
  - plugin-packaging-boundary
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/MODULE-HOST.md
  - docs/product/CRATE-STATUS.md
related_adrs:
  - docs/adr/ADR-0010-defer-wasm-plugin-host.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
supersedes: []
superseded_by: null
---

# Modules

## Purpose

Modules add domain-specific capability without changing the universal laws.

Examples:

- DevPunk — software development
- PubPunk — publishing and editorial workflows
- future modules — time, trading, research, operations

## Module rule

Use this section to keep the module rule narrow: a module can specialize work.
It cannot own truth.

New module work must follow the Module Authoring Baseline in
`docs/product/MODULE-AUTHORING.md` before implementation. The baseline defines
shared module anatomy, workspace policy, instruction policy, Deliberation Budget
usage, conformance packet expectations, and the DevPunk/PubPunk sequencing
boundary without activating Module Host runtime.

Modules may:

- draft module-specific contract payloads
- prepare bounded runs
- collect receipts
- provide assessments
- render inspect/status hints

Modules may not:

- write final decisions
- mutate the event log directly
- bypass scope
- publish side effects without declared permission
- weaken core laws

## Module lifecycle

```text
parked -> incubating -> active
```

A module becomes active only after passing module conformance evals.

The first module-host code slice is incubating only: `punk-module-host`
defines a pure invocation envelope, advisory assessment wrapper, and future
receipt field, side-effect request, policy gate preflight, and side-effect
receipt writer preflight, active behavior, file IO plan, target/storage policy
readiness, host path observation, and concrete path/storage policy readiness
and operation-evidence persistence readiness models, plus the first local-only
side-effect receipt writer write slice and operation-evidence write slice for
exact caller-provided bytes under explicit `.punk/runs` targets. It is not a
plugin runtime, module runner, CLI surface, broad operation evidence writer,
event writer, host path resolver, policy engine invoker, gate invoker, adapter,
publisher, PR creator, or gate/proof authority.

## First module strategy

Do not activate DevPunk or PubPunk until the core flow, eval, contract, gate, and proof loop are stable.

## PubPunk

Use this section as the current PubPunk status checkpoint.

PubPunk remains the future content-engineering module.

Its active module execution surface, CLI, adapters, and publishing behavior are
parked for now. PubPunk does not own a separate runtime; future PubPunk
execution must happen through the Punk-owned Module Host boundary.

The first code slice is incubating only: `punk-mod-pubpunk` provides a
side-effect-free inventory assessment model over caller-provided publishing
metadata. It is advisory evidence, not a publisher, not a receipt writer, not
Module Host activation, and not project truth.

Current public narrative data lives under `publishing/`.

The PubPunk boundary is defined in `docs/modules/pubpunk.md` and
`evals/specs/pubpunk-module-boundary.v0.1.md`. Publishing inventory, drafting,
planning, receipt creation, and publish behavior must stay behind that module
boundary and must not be added as active-core publishing subcommands before
Module Host promotion.

When activated, PubPunk must connect to:

- `publishing/stories/`
- `publishing/posts/`
- `publishing/channels/`
- `publishing/publications/`
- `publishing/metrics/`

It should not create a second content truth store.

## Plugin packaging rule

Use plugin packaging only as a future packaging or execution mechanism, not as
a Punk lifecycle primitive.

If plugins are introduced later, a plugin is only a packaging or execution mechanism for a module. The module still uses the universal lifecycle:

```text
Goal -> Contract -> Run -> Receipt -> ModuleAssessment -> DecisionObject -> Proofpack
```

The host may accept a plugin-produced receipt or assessment only after host validation. The plugin cannot become a second kernel, second event log, second memory system, or second decision authority.

Future plugin capabilities must be deny-by-default: no filesystem, network, environment, secrets, external publishing, direct event-log mutation, or final decision writing unless a later promoted architecture grants a scoped and receipted capability.
