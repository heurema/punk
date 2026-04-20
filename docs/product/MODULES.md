---
id: docs_product_modules
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
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

A module can specialize work. It cannot own truth.

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

## First module strategy

Do not activate DevPunk or PubPunk until the core flow, eval, contract, gate, and proof loop are stable.

## PubPunk

PubPunk is the future content-engineering module.

It is parked for now.

Current public narrative data lives under `publishing/`.

When activated, PubPunk must connect to:

- `publishing/stories/`
- `publishing/posts/`
- `publishing/channels/`
- `publishing/publications/`
- `publishing/metrics/`

It should not create a second content truth store.

## Plugin packaging rule

A plugin is not a Punk lifecycle primitive.

If plugins are introduced later, a plugin is only a packaging or execution mechanism for a module. The module still uses the universal lifecycle:

```text
Goal -> Contract -> Run -> Receipt -> ModuleAssessment -> DecisionObject -> Proofpack
```

The host may accept a plugin-produced receipt or assessment only after host validation. The plugin cannot become a second kernel, second event log, second memory system, or second decision authority.

Future plugin capabilities must be deny-by-default: no filesystem, network, environment, secrets, external publishing, direct event-log mutation, or final decision writing unless a later promoted architecture grants a scoped and receipted capability.
