---
id: docs_product_module_host
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-05-16
review_after: 2026-07-20
related_docs:
  - docs/product/ARCHITECTURE.md
  - docs/product/MODULES.md
related_adrs:
  - docs/adr/ADR-0010-defer-wasm-plugin-host.md
supersedes: []
superseded_by: null
---

# Module Host

Status: incubating envelope / runtime parked

## Purpose

The Module Host is the future Punk-owned boundary for invoking domain modules while preserving the shared lifecycle, laws, receipts, assessments, gate decisions, and proofpacks.

The Module Host runtime is not active core yet. It remains parked until Phase 6.

The current code slices are narrower: `punk-module-host` defines a pure
invocation envelope preflight, advisory assessment wrapper, local-only module
receipt proposal model, and local-only side-effect request proposal model for
module models, plus local-only policy gate and side-effect receipt writer
preflight models, a side-effect receipt writer active behavior model, and a
side-effect receipt writer file IO plan model, a target/storage policy
readiness model, a host path observation model, and a concrete path/storage
policy readiness model, an operation-evidence persistence readiness model, and
a first active local receipt write slice.
It
can model future receipt field coverage, future external action preconditions,
future policy evidence readiness, future receipt writer readiness, and future
receipt writer outcomes, file IO plans, target/storage policy readiness, and
host path observations, concrete path/storage policy readiness, and
operation-evidence persistence readiness, and can write exact caller-provided
side-effect receipt bytes to an explicit caller-provided `.punk/runs` target.
It does not load plugins, invoke modules, expose CLI behavior, resolve or
canonicalize host paths, persist operation evidence, mutate event logs, invoke
policy engines, invoke gate, call APIs, read credentials, invoke adapters,
publish, comment, create pull requests, write gate decisions, write
proofpacks, or claim acceptance.

## Boundary rule

A module is a Punk lifecycle participant.

A plugin is only a possible future packaging or execution mechanism for a module.

Domain module behavior must not be smuggled into active core as a convenience
CLI command. If a future capability is module-owned, core may expose it only
after the Module Host boundary, capability policy, receipts, conformance evals,
and gate/proof handoff are deliberately promoted.

Commands, modules, plugins, and adapters are mechanisms over the deeper primitives:

```text
Goal -> Contract -> Run -> Receipt -> ModuleAssessment -> DecisionObject -> Proofpack
```

`gate` remains the only writer of final decisions.

## Runtime-agnostic rule

The Module Host must stay runtime-agnostic until Phase 6 research chooses otherwise.

Future implementation candidates include:

- first-party native Rust modules
- direct Wasmtime Component Model + WIT
- Extism-style Wasm plugins
- another host/runtime boundary discovered during R2 research

No plugin runtime candidate is selected by this document.

## Future host responsibilities

The future runtime host owns the boundary:

- load a module manifest
- validate declared hooks
- prepare scoped input bundles
- pass only granted capabilities
- invoke the module/plugin runtime
- validate returned receipts and assessments
- record host-approved events
- preserve inspectable state and proof refs

The module or plugin runtime may produce advisory evidence. It may not own truth.

## Deny-by-default capability model

Future module/plugin capabilities must default to denied:

- filesystem
- network
- environment
- secrets
- process/shell
- external publishing
- project memory writes
- direct event-log writes
- final decision writes

Granting a capability later must be explicit, scoped, receipted, and covered by conformance evals.

## Receipt and assessment rule

A module or plugin may emit:

- module receipt
- module assessment

The host validates both before they can affect the run record.

Neither artifact can close work, write a final decision, or become canonical project memory by itself.

The current incubating envelope can wrap an already-produced advisory module
assessment summary. It validates that the invocation is explicitly scoped, that
capabilities remain denied except pure assessment of provided input, and that
the wrapped module output does not report side effects or authority-bearing
behavior.

The current receipt proposal model is pure/no-IO. It checks that the
invocation and assessment envelope match, parses known expected receipt fields,
and reports advisory coverage for a future host-approved module receipt. It
does not write receipts, mutate event logs, invoke modules or adapters, call
APIs, write gate decisions, write proofpacks, or claim acceptance.

The current side-effect request proposal model is also pure/no-IO. It checks
that a ready receipt proposal covers side-effect and host-validation fields,
requires safe refs for intent, policy, payload, adapter, and receipt-proposal
preconditions, and reports advisory readiness for a future external action. It
does not invoke adapters, publish, comment, create pull requests, write
receipts, mutate event logs, call APIs, read credentials, write gate decisions,
write proofpacks, or claim acceptance.

The current policy gate preflight model is pure/no-IO advisory evidence. It
checks that a ready side-effect request proposal has explicit policy,
gate-input, side-effect receipt proposal, adapter invocation receipt, payload,
and proof-requirement refs before a future external action can be considered.
It does not invoke a policy engine, invoke gate, approve work, write gate
decisions, write proofpacks, invoke adapters, publish, comment, create pull
requests, write receipts, mutate event logs, call APIs, read credentials, or
claim acceptance.

The current side-effect receipt writer preflight model is pure/no-IO advisory
evidence. It consumes a ready policy gate preflight plus explicit receipt
target, storage, operation evidence, idempotency, rollback, error, adapter
invocation receipt, and payload refs before a future local receipt writer can
be considered. It does not create or write receipts, mutate event logs, read or
write files, invoke adapters, invoke policy engines, invoke gate, publish,
comment, create pull requests, call APIs, read credentials, write proofpacks,
write gate decisions, or claim acceptance.

The current side-effect receipt writer active behavior model is also
pure/no-IO advisory evidence. It consumes a ready receipt writer preflight and
explicit observation data, then models planned-only, written, idempotent,
conflict, write-failed, partial-or-ambiguous, and preflight-failed outcomes
with selected, attempted, completed, failed, rollback-visible, and
error-visible steps. It does not create or write receipts, persist operation
evidence, mutate event logs, read or write files, invoke adapters, invoke
policy engines, invoke gate, publish, comment, create pull requests, call APIs,
read credentials, write proofpacks, write gate decisions, or claim acceptance.

The current side-effect receipt writer file IO plan model is pure/no-IO
advisory evidence. It consumes ready planned-only receipt writer active
behavior plus an explicit target path ref, write policy, idempotency basis,
temp/atomic policy, failure visibility, and boundary notes. It models storage
refs, receipt target refs, target path refs, create-new/idempotency policy,
rollback/error visibility, and operation-evidence persistence visibility before
any local write can be considered. It does not create or write receipts,
persist operation evidence, mutate event logs, read or write files, invoke
adapters, invoke policy engines, invoke gate, publish, comment, create pull
requests, call APIs, read credentials, write proofpacks, write gate decisions,
or claim acceptance.

The current side-effect receipt writer target/storage policy model is
pure/no-IO advisory evidence. It consumes a ready file IO plan plus explicit
policy refs for storage root selection, receipt target, target path derivation,
path encoding, parent directory, symlink handling, traversal, storage-root
escape, redaction, idempotency/conflict, temp/atomic behavior, and
operation-evidence persistence. It checks explicit storage, receipt target, and
target path refs plus required failure visibility before any local write can be
considered. It does not resolve host paths, read or write files, create or
write receipts, persist operation evidence, mutate event logs, invoke adapters,
invoke policy engines, invoke gate, publish, comment, create pull requests,
call APIs, read credentials, write proofpacks, write gate decisions, or claim
acceptance.

The current side-effect receipt writer host path observation model is
pure/no-IO advisory evidence. It consumes a ready target/storage policy model
plus an explicit host path kind, optional redacted host path ref, redaction
state, explicit observation blockers, and boundary notes. It models
storage-root-relative, runtime-relative, absolute, or unavailable host path
observations plus parent-directory, symlink, canonicalization, traversal,
storage-root escape, unsafe-ref, missing-ref, and redaction blockers before
any local write can be considered. It does not resolve or canonicalize host
paths, inspect the filesystem, read or write files, create or write receipts,
persist operation evidence, mutate event logs, invoke adapters, invoke policy
engines, invoke gate, publish, comment, create pull requests, call APIs, read
credentials, write proofpacks, write gate decisions, or claim acceptance.

The current side-effect receipt writer concrete path/storage policy model is
pure/no-IO advisory evidence. It consumes a ready target/storage policy model
plus a ready host path observation model, requires selected policy refs,
separated storage/receipt-target/target-path/host-path refs, redacted sensitive
host path observations, and boundary notes, then reports whether the future
local receipt writer has enough concrete path/storage policy evidence to be
considered. It does not resolve or canonicalize host paths, inspect the
filesystem, read or write files, create or write receipts, persist operation
evidence, mutate event logs, invoke adapters, invoke policy engines, invoke
gate, publish, comment, create pull requests, call APIs, read credentials,
write proofpacks, write gate decisions, or claim acceptance.

The current side-effect receipt writer operation-evidence persistence model is
pure/no-IO advisory evidence. It consumes a ready concrete path/storage policy
model, requires explicit operation evidence, idempotency, rollback, and error
refs plus a selected operation-evidence persistence policy, keeps those refs
separate and non-authoritative, and reports whether future local operation
evidence persistence is ready to be considered. It does not persist operation
evidence, create or write receipts, mutate event logs, resolve or canonicalize
host paths, inspect the filesystem, read or write files, invoke adapters,
invoke policy engines, invoke gate, publish, comment, create pull requests,
call APIs, read credentials, write proofpacks, write gate decisions, or claim
acceptance.

The current side-effect receipt writer first active write slice is local-only
file IO. It consumes a ready operation-evidence persistence readiness model,
requires exact caller-provided receipt bytes, an explicit absolute storage root,
and an explicit target-relative path under `.punk/runs/`, writes with
create-new/no-overwrite behavior, treats identical existing bytes as
idempotent, blocks different existing bytes as conflicts, and returns
in-memory non-authoritative operation evidence. It does not create parent
directories, persist operation evidence, mutate event logs, invoke adapters,
invoke policy engines, invoke gate, call APIs, read credentials, publish,
comment, create pull requests, write gate decisions, write proofpacks, or claim
acceptance. The smoke coverage writes only under an explicit temporary project
root.

## Wasm status

Wasm is a future implementation option, not a current product claim.

Today Punk does not provide:

- a Wasm runtime dependency in active core
- a public `punk plugin` command
- a plugin installer or marketplace
- a stable plugin SDK
- a stable WIT contract

## Promotion requirements

Before Phase 6 promotes Module Host implementation:

- run R2 design research on runtime options
- decide the host boundary through a new ADR
- define capability and receipt schemas
- add module/plugin conformance evals
- keep `gate` as the only final decision writer

## References

- `docs/product/ROADMAP.md`
- `docs/product/MODULES.md`
- `docs/product/EVAL.md`
- `docs/product/RED-TEAM-REVIEW.md`
- `docs/adr/ADR-0010-defer-wasm-plugin-host.md`
- `knowledge/research/2026-04-19-wasm-plugin-module-host-r1.md`
- `knowledge/ideas/2026-04-19-wasm-plugin-runtime-option.md`
