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

- `punk flow inspect`
- `punk eval run smoke`
- `punk eval run smoke --format json`

Everything else in product docs should be read as target architecture, phase-gated design, parked scope, or future scope unless explicitly marked active.

`punk init` is a future setup target, not current behavior.

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

- `punk init`
- LLM contract drafting
- coding agent execution
- active proofpack writing
- `.punk/` runtime storage for work, runs, evals, decisions, or proofs
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

Research and ideas are stored separately:

- research notes: `knowledge/research/`
- idea backlog: `knowledge/ideas/`

Research and ideas do not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Dogfooding

`punk` is developed with `punk`, but only at the trust level it has earned.

At first this means self-tracking:

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
