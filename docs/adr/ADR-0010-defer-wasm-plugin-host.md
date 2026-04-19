# ADR-0010: Defer Wasm plugin host until Module Host phase

Status: Proposed
Date: 2026-04-19

Research refs:

- `knowledge/research/2026-04-19-wasm-plugin-module-host-r1.md`

## Context

Punk may eventually need extension points for domain modules and third-party capabilities. WebAssembly is a plausible runtime for sandboxed, cross-language plugins.

However, the current product direction is core-first. The active target is the stable kernel: flow, event log, evals, contracts, gate decisions, proofpacks, and inspectable state. The roadmap parks Module Host work until Phase 6, after core foundations are stable.

A Wasm plugin host would affect module interfaces, module permissions, adapter boundaries, external side-effect policy, eval policy, and the public CLI contract. Those are Research Gate surfaces.

## Decision

Do not implement a Wasm plugin system now.

Treat Wasm plugins as a parked future implementation option for the Module Host.

Adopt only these architecture constraints now:

- a module is a Punk lifecycle participant
- a plugin is only a possible packaging or execution mechanism for a module
- the Module Host remains Punk-owned and runtime-agnostic while parked
- future module/plugin capabilities are deny-by-default
- plugins/modules may emit host-validated receipts and assessments, not final decisions
- `gate` remains the only final decision writer

Before any plugin-runtime implementation, Phase 6 Module Host work must run R2 design research comparing at least:

- direct Wasmtime Component Model + WIT
- Extism plugin framework
- native Rust modules with Wasm deferred again

## Consequences

### Positive

- Preserves core-first discipline.
- Avoids adding a new runtime dependency before trust surfaces are stable.
- Prevents `plugin` from becoming a second lifecycle or truth model.
- Keeps future extension architecture open without committing too early.
- Gives Phase 6 concrete research and eval requirements.

### Negative

- No near-term third-party extension story.
- Future module-host design still needs R2 research.
- Some interface choices may need later refactoring once the host API is real.

## Guardrails

Until promoted by a later ADR:

- no `punk plugin` public CLI
- no plugin marketplace
- no Wasm runtime dependency in active core
- no plugin install/update mechanism
- no stable plugin SDK or WIT contract
- no filesystem, network, environment, secret, or publishing capability for plugins
- no direct event-log writes from modules/plugins
- no final decisions from modules/plugins

## Required future evals

When Module Host implementation begins, add conformance evals proving that modules/plugins:

- cannot write final decisions
- cannot mutate the event log directly
- cannot bypass scope
- emit only host-validated receipts and assessments
- fail closed on invalid schemas
- respect configured resource and capability limits
- do not create hidden project-memory truth stores

## References

- `docs/product/MODULE-HOST.md`
- `docs/product/ROADMAP.md`
- `docs/product/MODULES.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PUNK-LAWS.md`
- `knowledge/ideas/2026-04-19-wasm-plugin-runtime-option.md`
