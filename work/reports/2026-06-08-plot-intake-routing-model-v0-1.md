---
id: report_2026_06_08_plot_intake_routing_model_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_add_plot_intake_routing_model_v0_1.md
related_docs:
  - docs/product/PLOT-INTAKE.md
  - docs/product/CRATE-STATUS.md
related_evals:
  - evals/specs/plot-intake-routing-recommendation.v0.1.md
doc_impact:
  classification: code-doc
  refs:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/PLOT-INTAKE.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/plot-intake-routing-recommendation.v0.1.md
    - work/STATUS.md
research_gate:
  classification: R1
  required: true
---

# Plot Intake routing model v0.1

## Summary

Added the first side-effect-free Plot Intake routing model for supported
slash-command-shaped request text.

The model lets future harness bridges package `/punk ...`, `/pub ...`, and
`/dev ...` request text into a Punk-owned routing contract before any CLI,
adapter, runtime, or module execution work.

## What changed

- Added `PlotIntakeRequest`, route enums, boundary flags, and
  `route_plot_intake_request` in `punk-contract`.
- Routed `/punk ...` content requests to advisory `phase_route=plot`,
  `module_route=pubpunk`, and `next_handoff=pubpunk.contract_intake`.
- Treated `/pub ...` and `/dev ...` as non-authoritative route hints.
- Kept plain text without a supported prefix uncaptured.
- Kept empty or unclear supported requests clarification-required.
- Added `punk-contract` unit tests and one `punk-eval` smoke case.
- Updated current-status docs and work-ledger evidence.

## Boundary preserved

This slice does not activate:

- public CLI routing;
- harness slash-command files;
- Codex, Claude Code, Gemini, `agy`, or other harness adapters;
- provider/model calls;
- `.punk` writes;
- Module Host runtime;
- module loading;
- PubPunk module execution;
- draft writing;
- publishing;
- browser/API calls;
- credential access;
- receipt writing;
- event writing;
- gate decisions;
- proofpacks;
- acceptance claims.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds a side-effect-free Plot Intake routing model and smoke coverage without activating CLI/runtime/module behavior."
  touched_surfaces:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/PLOT-INTAKE.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/plot-intake-routing-recommendation.v0.1.md
    - work/STATUS.md
    - work/goals/goal_add_plot_intake_routing_model_v0_1.md
    - work/reports/2026-06-08-plot-intake-routing-model-v0-1.md
  required_updates:
    - crates/punk-contract/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/product/PLOT-INTAKE.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/plot-intake-routing-recommendation.v0.1.md
    - work/STATUS.md
    - work/goals/goal_add_plot_intake_routing_model_v0_1.md
    - work/reports/2026-06-08-plot-intake-routing-model-v0-1.md
```

## Verification

- `python3 scripts/check_research_gate.py` passed.
- `python3 scripts/check_work_ledger.py` passed.
- `python3 scripts/check_docs_governance.py --repo . --files docs/product/CRATE-STATUS.md docs/product/PLOT-INTAKE.md evals/specs/plot-intake-routing-recommendation.v0.1.md work/STATUS.md work/goals/goal_add_plot_intake_routing_model_v0_1.md work/reports/2026-06-08-plot-intake-routing-model-v0-1.md` passed.
- `cargo fmt --check` passed.
- `cargo test -p punk-contract` passed.
- `cargo test -p punk-eval` passed.
- `cargo check --workspace` passed.
- `cargo run -q -p punk-cli -- eval run smoke` passed.

## Not tested

No public CLI routing, harness adapter, provider/model call, `.punk` write,
Module Host runtime, PubPunk module execution, publishing, receipt writer, gate
writer, proofpack writer, or acceptance behavior was tested because none was
implemented.
