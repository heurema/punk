---
id: research_2026_04_19_wasm_plugin_module_host_r1
title: "Wasm plugin runtime option for the Punk module host"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
review_after: 2026-07-19
components: [module-host, modules, architecture, eval, security]
related_goals: []
related_files:
  - docs/product/ARCHITECTURE.md
  - docs/product/MODULES.md
  - docs/product/MODULE-HOST.md
  - docs/product/EVAL.md
  - docs/product/RED-TEAM-REVIEW.md
  - docs/adr/ADR-0010-defer-wasm-plugin-host.md
  - knowledge/ideas/2026-04-19-wasm-plugin-runtime-option.md
source_refs:
  - https://component-model.bytecodealliance.org/
  - https://component-model.bytecodealliance.org/running-components/wasmtime.html
  - https://github.com/WebAssembly/WASI/blob/main/docs/DesignPrinciples.md
  - https://docs.wasmtime.dev/examples-interrupting-wasm.html
  - https://extism.org/docs/concepts/manifest/
confidence: medium
research_level: R1
---

# Wasm plugin runtime option for the Punk module host

## Question

Should Punk introduce a Wasm plugin system now, reserve Wasm as a future module-host runtime option, or avoid it entirely?

## Decision context

Punk is currently core-first. The active stability target is the kernel: project identity, flow state, event log, local evals, contract lifecycle, gate decisions, proofpacks, and inspectable state.

A Wasm plugin host would change module interfaces, module permissions, adapter boundaries, external side-effect policy, eval policy, and the public CLI contract. Those are Research Gate surfaces.

This R1 note records the idea and promotes only the smallest architectural invariant needed now: keep the future Module Host runtime-agnostic and deny-by-default. It does not select Wasm, Extism, Wasmtime, WIT, or any plugin SDK.

## Sources reviewed

| Source | Tier | Why included | Key limitation |
|---|---|---|---|
| Punk product docs | A | Repo source of truth for core-first, laws, roadmap, module status, eval policy, and project memory | Current docs intentionally park module-host work |
| WebAssembly Component Model docs | A | Defines components, WIT interfaces, worlds, and WASI 0.2 status | Standards and tooling continue to evolve |
| WASI design principles | A | Capability-based security and no ambient authority align with Punk laws | Capability model does not design Punk governance by itself |
| Wasmtime Component Model docs | A | Shows direct Rust-compatible runtime candidate and default-deny resource stance | Runtime selection would still require host design |
| Wasmtime interruption docs | A | Fuel and epoch interruption inform resource-limit evals | Does not cover every host-call blocking or resource problem |
| Extism manifest docs | A/B | Mature plugin-oriented Wasm framework with manifests, hashes, memory, hosts, and paths | Framework ABI may not match a WIT-first Punk module contract |

## Existing systems / prior art

- The WebAssembly Component Model targets interoperable WebAssembly components with typed interfaces.
- WASI 0.2 provides stable WIT definitions that components can target.
- WASI's design is capability-based and avoids ambient authority.
- Wasmtime is a plausible Rust host candidate and denies filesystem and environment access by default unless granted.
- Extism is a plausible higher-level plugin framework with manifests, optional code hashes, memory constraints, allowed hosts, and allowed paths.

## Failure modes found

1. Roadmap bypass by a good idea. Wasm plugins are attractive, but implementing them before flow, eval, contract, gate, and proof are stable would violate core-first sequencing.
2. Plugin becomes a second kernel. A plugin runtime could accidentally gain its own lifecycle, state store, event log, or acceptance rule.
3. Gate bypass. Plugins must never write final decisions or mark acceptance.
4. Ambient capability leak. Filesystem, network, environment, secrets, shell/process, and publishing must be denied unless explicitly granted.
5. Hidden project memory. Plugin-local storage must not become canonical memory.
6. Resource exhaustion. Wasm sandboxing does not remove the need for host-enforced CPU, memory, output-size, and timeout limits.
7. Blocking host calls. Fuel or epoch interruption helps guest execution, but the host must still handle blocking host calls, async timeouts, and cancellation.
8. Supply-chain trust. Third-party plugins introduce provenance, hash pinning, signing, source-build, and local trust UX decisions.
9. ABI and API churn. Early commitment to WIT or a framework ABI can create compatibility debt.

## Options considered

### Option A

Implement Wasm plugins now.

Pros:

- clear extensibility story
- cross-language plugin authoring
- stronger sandbox story than arbitrary subprocesses

Cons:

- activates a future capability before the core is stable
- creates a new trust surface before module conformance evals exist
- risks a premature public `punk plugin` contract

### Option B

Adopt a runtime-agnostic Module Host boundary now.

Pros:

- preserves the module and plugin distinction
- keeps host ownership of receipts, events, and decisions
- adds no runtime dependency or public CLI behavior

Cons:

- does not solve future runtime selection yet
- still needs R2 before implementation

### Option C

Choose direct Wasmtime Component Model + WIT later.

Pros:

- strong fit for typed host-controlled interfaces
- fewer framework-specific abstractions

Cons:

- still a significant host design surface
- WIT commitment may be premature

### Option D

Choose Extism later.

Pros:

- faster plugin developer experience
- plugin-oriented manifest model

Cons:

- framework ABI may diverge from a future Punk-native interface
- another dependency choice to defend long-term

### Option E

Native Rust modules first; defer Wasm again.

Pros:

- smallest extension-surface complexity
- best fit if first-party modules are enough

Cons:

- weaker third-party extensibility story
- may delay cross-language module authoring too far

## Recommendation

Adopt now:

- module and plugin vocabulary boundary
- runtime-agnostic Module Host doc
- deny-by-default future capability model
- host-validated receipt and assessment rule
- future module and plugin conformance eval candidates
- ADR that defers Wasm plugin implementation until Phase 6 R2

Park as idea:

- Wasm plugin runtime
- WIT as canonical module interface language
- Extism as plugin framework
- third-party plugin installation
- plugin marketplace
- signing and provenance UX
- `punk plugin` public CLI

Avoid now:

- Wasm runtime dependency in active core
- stable plugin SDK
- any external side-effect capability for plugins
- claiming Punk is extensible through plugins today

## What stays out of scope

- implementation of `punk-module-host`
- public plugin commands
- plugin registry or marketplace
- automatic plugin install or update
- choosing Wasmtime, Extism, or WIT
- network, filesystem, or secret access for plugins
- external publishing side effects

## Impact on roadmap

No change to Phases 0 through 5.

Phase 6 remains the Module Host phase. This note adds a future R2 topic for Phase 6: compare native modules, Wasmtime Component Model + WIT, Extism, and any other concrete host boundary before implementation.

## Required evals

If Module Host or plugin runtime work is promoted later, add conformance evals proving:

- module or plugin cannot write a final decision
- module or plugin cannot mutate the event log directly
- module or plugin cannot bypass contract scope
- invalid receipt or assessment schema fails closed
- undeclared filesystem, network, environment, or secret access is denied
- declared capabilities are recorded in receipts
- resource limits are enforced and receipted
- infinite-loop or long-running fixtures are interrupted
- oversized output is denied or bounded
- plugin-local state is not canonical project memory

## Required docs / ADRs / contracts

This note should be paired with:

- `docs/adr/ADR-0010-defer-wasm-plugin-host.md`
- `docs/product/MODULE-HOST.md`
- `knowledge/ideas/2026-04-19-wasm-plugin-runtime-option.md`

Future implementation requires:

- Phase 6 R2 research
- module-host ADR
- capability model doc
- conformance eval fixtures
- contract and proof schema updates if needed

## Open questions

- Should first-party modules ever run through the plugin sandbox, or only third-party modules?
- Should Punk prefer WIT-first interfaces or an Extism-style manifest and host-function ABI?
- Is in-process Wasm enough, or should higher-risk plugins run in worker processes?
- What local-first trust UX is acceptable for third-party code?
- What exact receipt fields are needed for capability grants and resource limits?
