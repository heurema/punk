---
id: docs_product_crate_status
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-24
review_after: 2026-07-20
canonical_for:
  - crate-status
  - workspace-status-vocabulary
  - active-core-crate-boundaries
  - parked-crate-boundaries
  - executor-agnostic-validation-crate-boundary
related_docs:
  - docs/product/START-HERE.md
  - docs/product/ROADMAP.md
  - docs/product/ARCHITECTURE.md
related_adrs:
  - docs/adr/ADR-0014-executor-agnostic-validation-boundary.md
supersedes: []
superseded_by: null
---
# Crate Status

## Status vocabulary

- `active-core` — required for the current stability target
- `incubating` — tested but not default/user-facing
- `parked` — boundary exists; minimal stub/docs only
- `retired` — removed or legacy-only

## Initial target status

| Crate | Status | Owns |
|---|---|---|
| `punk-cli` | active-core | CLI routing |
| `punk-domain` | active-core | canonical types |
| `punk-core` | active-core | deterministic helpers, validation, hashing |
| `punk-events` | active-core | append-only event log |
| `punk-flow` | active-core | state machine and transition guards |
| `punk-rules` | active-core | Punk Laws and rule snapshots |
| `punk-eval` | active-core | smoke/local eval harness |
| `punk-contract` | active-core | contract schema and validation |
| `punk-gate` | active-core | decision writing |
| `punk-proof` | active-core | proofpack writing/hashing |
| `punk-project` | active-core | project identity and memory roots |
| `punk-module-host` | parked | module manifest and hook routing |
| `punk-adapters` | parked | provider/tool wrappers |
| `punk-mod-dev` | parked | software-development module |
| `punk-mod-pub` | parked | content/publishing module |

## Research intake crate stance

Do not add a standalone `punk-telemetry` product crate yet.

Telemetry ownership is distributed across active-core crates:

- `punk-events`: event schema, append-only writer, replay support
- `punk-flow`: transition and guard events
- `punk-eval`: eval run events and reports
- `punk-gate`: final decision events
- `punk-proof`: proofpack manifest and artifact hash refs
- `punk-project`: project identity and memory-link boundaries

Future export adapters, dashboards, or remote sinks are parked and must not be listed as active-core.

## Executor-agnostic validation stance

Do not add an active-core executor, prompt manager, skill manager, provider runner, or agent runtime crate in the current phase.

The current active-core responsibility is:

- contract shape
- scope validation
- validator plan and deterministic helpers
- receipt/evidence shape
- eval report shape
- gate decision exclusivity
- proofpack links and hashes

Punk may run validators in the user's environment, but the user's executor, provider, prompt, skill, or local model setup must not become active-core authority.

Executor-specific behavior and semantic assessor command interfaces remain parked until the roadmap explicitly promotes DevPunk, adapters, or an assessor boundary.

## Parked module docs

Parked modules may exist as documentation folders before they become workspace crates.

Initial parked module docs:

- `docs/modules/pubpunk.md`

Future crate name when promoted:

- `punk-mod-pub`

## Parked adapter docs

Parked adapters may exist as documentation folders before they become active
workspace behavior.

Initial parked adapter docs:

- `docs/adapters/repo-search.md`

Future implementation location when promoted:

- `punk-adapters::repo_search`

Promotion requires a completed research note, receipt schema, conformance
evals, and proof that the adapter cannot own project truth or write final
decisions.

## Parked module host note

`punk-module-host` is not a plugin runtime while parked.

The parked boundary may document module manifests, hook types, permissions, and conformance expectations. It must not add a Wasm runtime dependency, plugin installer, plugin marketplace, or public `punk plugin` command until a later ADR promotes that scope.
