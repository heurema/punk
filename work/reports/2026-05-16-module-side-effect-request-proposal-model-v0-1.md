---
id: report_2026_05_16_module_side_effect_request_proposal_model_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-16
updated_at: 2026-05-16
goal_ref: work/goals/goal_add_module_side_effect_request_proposal_model_v0_1.md
---

# Module Side-Effect Request Proposal Model v0.1

## Summary

Added a pure Module Host side-effect request proposal model in
`punk-module-host`.

The model consumes a module invocation envelope, ready receipt proposal, and
explicit side-effect request draft. It reports advisory precondition coverage
for a future external action. It does not invoke adapters, publish, comment,
create pull requests, write receipts, mutate event logs, invoke modules, read
or write files, call APIs, read credentials, write gate decisions, write
proofpacks, or claim acceptance.

## Files changed

- `crates/punk-module-host/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `README.md`
- `docs/product/MODULE-HOST.md`
- `docs/product/MODULES.md`
- `docs/product/CRATE-STATUS.md`
- `work/STATUS.md`
- `work/goals/goal_add_module_side_effect_request_proposal_model_v0_1.md`
- `work/reports/2026-05-16-module-side-effect-request-proposal-model-v0-1.md`

## Runtime slice selected

Selected slice: local-only module side-effect request proposal model.

This is not adapter invocation, publishing automation, comment automation, PR
automation, or a receipt writer. It is the next pure boundary model needed
before future policy gates, side-effect receipts, adapter invocation receipts,
or external action execution can be promoted.

## Validation run

Validation was run after the implementation and docs/status update. Results:

- `cargo fmt --check`: PASS
- `cargo check --workspace`: PASS
- `cargo test --workspace`: PASS
- `cargo run -q -p punk-cli -- eval run smoke`: PASS
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files README.md crates/punk-eval/src/lib.rs crates/punk-module-host/src/lib.rs docs/product/CRATE-STATUS.md docs/product/MODULE-HOST.md docs/product/MODULES.md work/STATUS.md work/goals/goal_add_module_side_effect_request_proposal_model_v0_1.md work/reports/2026-05-16-module-side-effect-request-proposal-model-v0-1.md --report work/reports/2026-05-16-module-side-effect-request-proposal-model-v0-1.md`: PASS with unchanged definition-shape warnings in `docs/product/MODULE-HOST.md` and `docs/product/MODULES.md`.
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds an incubating module-host side-effect request proposal model and smoke coverage."
  touched_surfaces:
    - README.md
    - docs/product/MODULE-HOST.md
    - docs/product/MODULES.md
    - docs/product/CRATE-STATUS.md
    - work/STATUS.md
    - work/goals/goal_add_module_side_effect_request_proposal_model_v0_1.md
    - work/reports/2026-05-16-module-side-effect-request-proposal-model-v0-1.md
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
- Module Host remains incubating library behavior only.
- Module Host runtime remains parked.
- Plugin runtime remains parked.
- PubPunk runtime remains parked.
- Active CLI surface unchanged.
- Active runtime side-effect scope unchanged.
- No module was invoked by the host.
- No adapter was invoked.
- No receipt writer was created.
- No event-log mutation was added.
- No publication receipt writer was created.
- No publication, comment, pull request, issue, or external action happened.
- No external side effects, GitHub/API/browser behavior, credential reads,
  bots, adapters, automatic issue/PR creation, gate decisions, proofpacks, DAO,
  token, or funding behavior were added.

## Drift observed

The module boundary held: the new side-effect request proposal model did not
add publishing, commenting, PR creation, adapter invocation, or receipt writing
to `punk-project`, `punk-cli`, `punk-events`, or active-core publishing
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
- External commenting
- Pull request creation
- Browser/API calls
- Credential access
- Bots/adapters
- Metrics collection
- Runtime receipts
- Gate/proof authority
- Automatic issue or PR creation

## Next code slice

Recommended next slice: local-only module policy gate preflight model. It
should still be pure/no-IO and should decide whether a side-effect request
proposal has enough policy, receipt, and gate/proof preconditions to become an
approved local plan before any adapter invocation or external side effect is
introduced.
