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
