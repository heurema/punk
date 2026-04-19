# punk

<p align="center">
  <img src="brand/landing/punk-readme.svg" width="440" alt="punk mascot" />
</p>

`punk` is a local-first bounded work kernel with universal laws, installable domain modules, project memory, and proof-bearing contracts.

## Current decision

Start core-first.

Create the workspace and documentation boundaries early, but activate behavior slowly.

A crate or folder may exist without being part of the active operator surface.

## First principle

One CLI. Many modules. Shared laws. Project memory. Proof-bearing work.

## Active now

The first active target is not DevPunk, PubPunk, provider adapters, or a marketplace.

The first active target is the stable core:

- project identity
- flow state machine
- event log
- minimal local eval harness
- simple contract lifecycle
- gate decision
- proofpack
- inspectable state

## Not active yet

- LLM contract drafting
- coding agent execution
- PubPunk publishing
- provider adapters
- MCP integration
- knowledge embeddings
- plugin marketplace
- council
- skill auto-promotion

See `docs/product/START-HERE.md`.


## Dogfooding

`punk` is developed with `punk`.

At first this means self-tracking:

- goals in `work/goals/`
- reports in `work/reports/`
- knowledge in `knowledge/`
- decisions in `docs/adr/`

Self-execution comes later, after flow, eval, contract, gate, and proof are stable enough.

See `docs/product/DOGFOODING.md`.


## Public narrative

`punk` is a public build from day zero.

Stories, post drafts, manual publication receipts, and metrics snapshots live under `public/`.

Future PubPunk automation must adopt that structure instead of creating a separate content store.
