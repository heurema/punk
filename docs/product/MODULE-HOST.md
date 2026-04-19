# Module Host

Status: parked

## Purpose

The Module Host is the future Punk-owned boundary for invoking domain modules while preserving the shared lifecycle, laws, receipts, assessments, gate decisions, and proofpacks.

The Module Host is not active core yet. It is parked until Phase 6.

## Boundary rule

A module is a Punk lifecycle participant.

A plugin is only a possible future packaging or execution mechanism for a module.

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

No candidate is selected by this parked document.

## Host responsibilities

The host owns the boundary:

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
