# punk

<p align="center">
  <img src="brand/landing/punk-readme.svg" width="440" alt="punk mascot" />
</p>

`punk` is an experimental, early-stage, local-first bounded work kernel and sandbox for developers, researchers, and experimenters.

It explores bounded AI work, project memory, contracts, evals, gates, and proof-bearing workflows.

It is not a finished product, not production-ready, and not guaranteed to work end-to-end out of the box.

The current executable CLI surface is intentionally small. Treat broader architecture docs as target architecture, phase-gated design, parked scope, or future scope unless a document explicitly marks a capability active.

Live landing: [punks.run](https://punks.run)

## What punk is

`punk` exists to make bounded work inspectable:

```text
contract -> evidence -> gate decision -> proof
```

Only `gate` writes final decisions.

Proof makes decisions inspectable before acceptance is claimed.

Modules may assess, not decide. Adapters may invoke, not own truth. Project memory is explicit and authority-tagged.

## Current implemented surface

Today, the executable CLI surface is intentionally limited to:

- `punk init <project-id>`
- `punk init <project-id> --mode brownfield`
- `punk flow inspect`
- `punk publishing locate [--project-root <path>] [--json]`
- `punk eval run smoke`
- `punk eval run smoke --format json`

Everything else in product docs should be read as target architecture, phase-gated design, parked scope, or future scope unless explicitly marked active.

Runtime automation has one narrow active library slice: `punk-events` can append flow event drafts to `.punk/events/flow.jsonl` under an explicit initialized project root. This is local-only event evidence, not a CLI transition runner, not external automation, and not decision authority.

Module Host has incubating library slices: `punk-module-host` can preflight a pure module invocation envelope, wrap advisory module assessment output, and model future module receipt field coverage without writing receipts. It does not load plugins, invoke modules, expose CLI behavior, read or write files, create receipts, mutate event logs, call APIs, read credentials, invoke adapters, publish, or write gate/proof authority.

Publishing has one narrow active resolver surface: `punk publishing locate` reads `.punk/publishing.toml` plus local-only `.punk/publishing.local.toml`, validates the logical workspace binding, and reports the external local publishing workspace in human or JSON form. It creates no files, publishes nothing, opens no browser, calls no API, reads no credentials, and does not activate PubPunk automation. Do not extend this core CLI surface with inventory, drafting, planning, receipt, or publish behavior; that belongs behind a future PubPunk/module-host boundary.

PubPunk has one incubating library slice: `punk-mod-pub` can assess caller-provided publishing inventory metadata and receipt gaps as advisory module evidence. It has no public CLI, performs no filesystem IO, creates no publication receipts, publishes nothing, invokes no adapters, writes no gate/proof authority, and does not make publishing workspaces project truth.

`punk init <project-id>` is the default greenfield Dogfooding Level 0 compact project-memory scaffold.

Run it from the target project root. It initializes the current directory in place and does not create a new subdirectory named `<project-id>`.

It records `project_id` and `entry_mode = greenfield`, writes repo-tracked starter memory under `.punk/memory/`, writes `.punk` marker/setup files, creates thin local instruction entrypoints under `.punk/instructions/`, and does not activate brownfield reconstruction, grayfield reconciliation, runtime storage, contracts, gates, proofs, Writer behavior, or network analysis.

`punk init <project-id> --mode brownfield` is active only as a brownfield entry scaffold for an existing project.

It records `entry_mode = brownfield`, `reconstruction_status = not_started`, and `authority = advisory_candidates_only`, then creates empty advisory reconstruction placeholders under `.punk/memory/reconstruction/` plus the same thin local instruction entrypoints under `.punk/instructions/`.

It does not scan the repository, infer intent, summarize code, generate contracts/specs, accept claims, write runtime state, or prove anything.

`.punk/` runtime storage is not active for this purpose.

Active proofpack writing is not active.

Provider/model/agent execution is not active.

PubPunk publishing automation is not active.

## Current project stage

The current stage is v0.1-prep: Current Truth Baseline.

The goal of this stage is not to add product features.
The goal is to align README, product docs, work ledger, changelog, and lineage notes so the repository clearly separates:

- current implemented behavior;
- active-core target;
- incubating or side-effect-free models;
- parked/future surfaces.

## Current direction

Start core-first.

Create the workspace and documentation boundaries early. Activate behavior slowly.

A crate, folder, module, adapter, or public narrative surface may exist before it is part of the active operator path.

## First principle

One CLI. Many modules. Shared laws. Project memory. Proof-bearing work.

The lifecycle grammar is:

```text
plot -> cut -> gate
```

## Active target

The current active target is the stable core:

- project identity
- flow state machine
- append-only event log
- minimal local eval harness
- simple contract lifecycle
- gate decision
- proofpack
- inspectable state
- project memory links

This target is broader than the implemented CLI surface above. Do not treat target architecture as current behavior until it is explicitly promoted.

See:

- `docs/product/START-HERE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/LINEAGE.md`
- `work/STATUS.md`
- `CHANGELOG.md`

## Not implemented / not active yet

The following may be documented or parked, but they are not current operator surfaces:

- brownfield reconstruction or grayfield `punk init`
- LLM contract drafting
- coding agent execution
- active proofpack writing
- `.punk/` runtime storage beyond the narrow local event-log writer slice, including work, runs, evals, decisions, or proofs
- persisted flow state beyond local event evidence
- publishing execution beyond local workspace location
- generated instruction views beyond source `.punk/instructions/` pages
- gate decision writer
- acceptance claim writer
- PubPunk publishing automation
- provider adapters
- MCP integration
- knowledge embeddings as project truth
- plugin marketplace
- module/plugin runtime
- council
- skill auto-promotion
- cloud sync or SaaS control plane

## Documentation system

Punk docs follow a source-of-truth map to avoid duplicate or conflicting claims.

Meaningful changes should declare `DocImpact`, update the canonical owner, and preserve superseded truth instead of silently deleting it.

Start with:

- `docs/product/START-HERE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/DOC-GOVERNANCE.md`
- `docs/product/LINEAGE.md`

In this repository's dogfooding layout, research and ideas are stored separately:

- research notes: `knowledge/research/`
- idea backlog: `knowledge/ideas/`

Research and ideas do not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Dogfooding

`punk` is developed with `punk`, but only at the trust level it has earned.

At first this means self-tracking in Punk's own dogfooding layout:

- goals in `work/goals/`
- reports in `work/reports/`
- knowledge in `knowledge/`
- decisions in `docs/adr/`

Self-execution comes later, after flow, eval, contract, gate, and proof are stable enough.

See `docs/product/DOGFOODING.md`.

## Public build

`punk` is a public build from day zero.

Stories, post drafts, manual publication receipts, and metrics snapshots live under `publishing/`.

Future PubPunk automation must adopt that structure instead of creating a hidden content store.

See `docs/product/PUBLIC-NARRATIVE.md`.

## Open source and community

- [LICENSE](LICENSE)
- [NOTICE](NOTICE)
- [CONTRIBUTING](CONTRIBUTING.md)
- [CODE_OF_CONDUCT](CODE_OF_CONDUCT.md)
- [SECURITY](SECURITY.md)
- [SUPPORT](SUPPORT.md)
- [TRADEMARKS](TRADEMARKS.md)
- [DCO](DCO.md)

## License

[Apache License 2.0](LICENSE)
