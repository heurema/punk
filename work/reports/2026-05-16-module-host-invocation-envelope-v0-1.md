---
id: report_2026_05_16_module_host_invocation_envelope_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-16
updated_at: 2026-05-16
goal_ref: work/goals/goal_add_module_host_invocation_envelope_v0_1.md
---

# Module Host Invocation Envelope v0.1

## Summary

Added the first incubating Module Host code slice as a pure invocation envelope
and advisory assessment wrapper in `punk-module-host`.

The model validates explicit module invocation metadata and wraps already
produced advisory module output. It does not load plugins, invoke modules, read
or write files, create receipts, mutate event logs, call APIs, read
credentials, invoke adapters, publish externally, write gate decisions, write
proofpacks, or claim acceptance.

## Files changed

- `Cargo.toml`
- `Cargo.lock`
- `crates/punk-module-host/Cargo.toml`
- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/Cargo.toml`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_module_host_invocation_envelope_v0_1.md`
- `work/reports/2026-05-16-module-host-invocation-envelope-v0-1.md`

## Runtime slice selected

Selected slice: pure module-host invocation envelope and advisory assessment
wrapper.

This is not Module Host runtime and not plugin execution. It is the generic
local boundary needed before module-owned assessment models can be safely wired
into future runtime, receipt, policy, or gate/proof paths.

## Validation run

Validation was run after the implementation and docs/status update. Results:

- `cargo fmt --check`: PASS
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo run -q -p punk-cli -- eval run smoke`: PASS
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files Cargo.lock Cargo.toml README.md crates/punk-eval/Cargo.toml crates/punk-eval/src/lib.rs crates/punk-module-host/Cargo.toml crates/punk-module-host/src/lib.rs docs/product/MODULE-HOST.md docs/product/MODULES.md docs/product/CRATE-STATUS.md work/STATUS.md work/goals/goal_add_module_host_invocation_envelope_v0_1.md work/reports/2026-05-16-module-host-invocation-envelope-v0-1.md --report work/reports/2026-05-16-module-host-invocation-envelope-v0-1.md`: PASS with definition-shape warnings in `docs/product/MODULE-HOST.md` and unchanged definition-shape warnings in `docs/product/MODULES.md`.
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds an incubating module-host crate and smoke coverage for pure module assessment envelopes."
  touched_surfaces:
    - README.md
    - docs/product/MODULE-HOST.md
    - docs/product/MODULES.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_add_module_host_invocation_envelope_v0_1.md
    - work/reports/2026-05-16-module-host-invocation-envelope-v0-1.md
  required_updates:
    - README.md
    - docs/product/MODULE-HOST.md
    - docs/product/MODULES.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: `README.md`,
  `docs/product/MODULE-HOST.md`, `docs/product/MODULES.md`,
  `docs/product/CRATE-STATUS.md`.
- Module Host moves from docs-only parked boundary to incubating pure envelope
  model only.
- Module Host runtime remains parked.
- Plugin runtime remains parked.
- PubPunk runtime remains parked.
- Active CLI surface unchanged.
- Active runtime side-effect scope unchanged.
- No module was invoked by the host.
- No publication receipt writer was created.
- No publication happened.
- No external side effects, GitHub/API/browser behavior, credential reads,
  bots, adapters, automatic issue/PR creation, event-log mutation, gate
  decisions, proofpacks, DAO, token, or funding behavior were added.

## Drift observed

The PubPunk boundary held: the new generic host envelope did not add publishing
inventory behavior to `punk-project`, `punk-cli`, or active-core publishing
commands.

## Out of scope

- Module Host runtime
- Plugin loading
- Dynamic module execution
- Module manifests
- Wasm or Extism runtime
- Public CLI expansion
- Filesystem reads or writes
- Receipt writing
- Event-log mutation
- External publishing
- Browser/API calls
- Credential access
- Bots/adapters
- Metrics collection
- Runtime receipts
- Gate/proof authority
- Automatic issue or PR creation

## Next code slice

Recommended next slice: local-only module assessment receipt proposal model.
It should still be pure/no-IO and should describe what a future host-approved
module receipt would need to contain before any receipt writer or side-effect
request path is introduced.
