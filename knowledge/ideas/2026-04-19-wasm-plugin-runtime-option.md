---
id: idea_2026_04_19_wasm_plugin_runtime_option
title: "Wasm plugins as a future Module Host runtime option"
kind: idea
status: raw
authority: speculative
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
review_after: 2026-07-19
components: [module-host, plugins, wasm, security]
related_goals: []
related_files:
  - docs/product/MODULE-HOST.md
  - docs/adr/ADR-0010-defer-wasm-plugin-host.md
  - knowledge/research/2026-04-19-wasm-plugin-module-host-r1.md
source_refs:
  - knowledge/research/2026-04-19-wasm-plugin-module-host-r1.md
confidence: medium
---

# Wasm plugins as a future Module Host runtime option

## Summary

Punk may eventually support modules packaged as Wasm plugins. This is a parked idea, not active architecture and not an operator promise.

## Idea

A future Module Host could invoke sandboxed Wasm components or plugins so module authors can write extensions outside the core Rust workspace.

Potential forms:

- direct Wasmtime Component Model + WIT
- Extism-style plugin manifests and PDKs
- native Rust modules first, with Wasm kept for third-party modules later

## Why preserve it

Wasm could help with:

- cross-language module authoring
- sandboxed execution
- explicit capability grants
- local-first plugin distribution by file or hash
- reproducible receipt and proof metadata for module runs

## Non-negotiable constraints

Any future plugin runtime must preserve Punk laws:

- one CLI: `punk`
- one lifecycle: `plot / cut / gate`
- modules assess, but do not decide
- adapters invoke, but do not own truth
- only `gate` writes final decisions
- proof comes before acceptance
- project memory remains explicit and authority-tagged

## Anti-promises

Do not claim today:

- Punk has plugins
- Punk has a plugin marketplace
- `punk plugin` is a command
- third-party Wasm extensions are supported
- Wasm has been selected as the Module Host runtime

## Promotion trigger

Revisit during Phase 6 Module Host work, after core flow, eval, contract, gate, and proof foundations are stable.

Promotion requires R2 research and a new ADR.
