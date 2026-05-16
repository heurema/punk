---
id: report_2026_05_16_pubpunk_module_boundary_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-16
updated_at: 2026-05-16
goal_ref: work/goals/goal_define_pubpunk_module_boundary_v0_1.md
---

# PubPunk Module Boundary v0.1

## Summary

Defined PubPunk as future module-owned publishing behavior, not active-core
behavior.

This is a docs/spec boundary only. No runtime, CLI, adapter, bot, publishing,
receipt writer, API call, credential access, or external side effect was added.

## Files changed

- `docs/product/MODULES.md`
- `docs/modules/pubpunk.md`
- `evals/specs/pubpunk-module-boundary.v0.1.md`
- `work/STATUS.md`
- `work/goals/goal_define_pubpunk_module_boundary_v0_1.md`
- `work/reports/2026-05-16-pubpunk-module-boundary-v0-1.md`

## Boundary defined

- PubPunk is a future domain module.
- Core remains responsible for contracts, scope, events, receipts, gate
  decisions, proof refs, and source-of-truth boundaries.
- PubPunk may later produce module receipts and assessments.
- Capabilities are denied by default.
- External publishing, browser/API calls, credential reads, adapters, and
  metrics collection require explicit policy, grants, and side-effect receipts.
- Publishing workspace artifacts are input/source surfaces, not project truth.
- `gate` remains the only final decision writer.

## Validation run

Validation was run after the boundary update. Results:

- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files docs/product/MODULES.md docs/modules/pubpunk.md evals/specs/pubpunk-module-boundary.v0.1.md work/STATUS.md work/goals/goal_define_pubpunk_module_boundary_v0_1.md work/reports/2026-05-16-pubpunk-module-boundary-v0-1.md --report work/reports/2026-05-16-pubpunk-module-boundary-v0-1.md`: PASS with unchanged definition-shape warnings in `docs/product/MODULES.md`.
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Defines PubPunk module boundary and eval/spec constraints before implementation."
  touched_surfaces:
    - docs/product/MODULES.md
    - docs/modules/pubpunk.md
    - evals/specs/pubpunk-module-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_pubpunk_module_boundary_v0_1.md
    - work/reports/2026-05-16-pubpunk-module-boundary-v0-1.md
  required_updates:
    - docs/product/MODULES.md
    - docs/modules/pubpunk.md
    - evals/specs/pubpunk-module-boundary.v0.1.md
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: `docs/product/MODULES.md`.
- Module docs changed: `docs/modules/pubpunk.md`.
- Eval/spec boundary added: `evals/specs/pubpunk-module-boundary.v0.1.md`.
- PubPunk remains parked.
- Module Host remains parked.
- Active CLI surface unchanged.
- Active runtime scope unchanged.
- Existing `punk publishing locate` remains a transitional locator only.
- No active-core publishing command was added.
- No Rust code changed.
- No publishing, browser/API, credential, bot, adapter, issue/PR, receipt,
  gate, proofpack, DAO, token, or funding behavior was added.

## Out of scope

- PubPunk runtime
- Module Host runtime
- Publishing inventory implementation
- Draft generation
- Receipt writing
- External publishing
- Browser/API calls
- Credential access
- Bots/adapters
- Metrics collection
- Runtime receipts
- CLI command expansion

## Next implementation slice

After this boundary, the smallest code slice is a module-owned, side-effect-free
PubPunk inventory model with no public CLI and no filesystem writes. It should
take explicit caller-provided refs/metadata, return an advisory module
assessment shape, and prove through eval/smoke coverage that it cannot publish,
read credentials, call APIs, create receipts, or write final decisions.
