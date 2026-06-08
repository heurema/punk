---
id: report_2026_06_08_harness_intake_routing_boundary_v0_1
kind: work-report
status: done
authority: evidence
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_capture_harness_intake_routing_boundary_v0_1.md
related_docs:
  - docs/product/PLOT-INTAKE.md
  - docs/product/DOCUMENTATION-MAP.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/MODULES.md
related_evals:
  - evals/specs/plot-intake-routing-recommendation.v0.1.md
doc_impact:
  classification: architecture
  refs:
    - docs/product/PLOT-INTAKE.md
    - docs/product/DOCUMENTATION-MAP.md
    - evals/specs/plot-intake-routing-recommendation.v0.1.md
research_gate:
  classification: R1
  required: false
---

# Harness intake routing boundary v0.1

## Summary

Captured the proposed harness slash-command bridge as an advisory Plot Intake
boundary candidate.

The captured shape is:

```text
executor harness command, such as /punk ...
  -> request envelope with harness metadata
  -> future Punk-owned intake route entrypoint
  -> advisory phase route plus module route
  -> clarification, contract intake, or blocked result before execution
```

## What changed

- `docs/product/PLOT-INTAKE.md` now records the future thin harness command
  bridge candidate, request envelope fields, route result shape, route-to-phase
  plus route-to-module separation, `/punk draft a blog post...` PubPunk-shaped
  example, and non-authority boundaries.
- `evals/specs/plot-intake-routing-recommendation.v0.1.md` now records
  advisory cases for thin harness commands, separate phase/module routing,
  Punk-owned route semantics, and non-authoritative route hints.
- `docs/product/DOCUMENTATION-MAP.md` now reflects that Plot Intake owns the
  harness slash-command bridge candidate and routing recommendation boundary.
- `work/STATUS.md` records this as a completed side-track while preserving the
  selected next pause checkpoint.

## Boundary preserved

This slice does not activate:

- CLI behavior;
- harness slash-command files;
- provider-specific routing;
- module routing runtime;
- Module Host runtime;
- PubPunk runtime;
- adapters;
- publishing;
- receipt writing;
- event writing;
- gate decisions;
- proofpacks;
- acceptance claims.

## Rationale

The design keeps user ergonomics and project authority separate:

- harness commands are runner aids;
- Punk owns route semantics;
- route results are advisory;
- phase route and module route are distinct;
- side effects remain blocked until a later contract, capability, receipt,
  gate, and proof path exists.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Captures an advisory Plot Intake routing boundary for future harness slash commands and phase/module route separation."
  touched_surfaces:
    - docs/product/PLOT-INTAKE.md
    - docs/product/DOCUMENTATION-MAP.md
    - evals/specs/plot-intake-routing-recommendation.v0.1.md
    - work/STATUS.md
    - work/goals/goal_capture_harness_intake_routing_boundary_v0_1.md
    - work/reports/2026-06-08-harness-intake-routing-boundary-v0-1.md
  required_updates:
    - docs/product/PLOT-INTAKE.md
    - docs/product/DOCUMENTATION-MAP.md
    - evals/specs/plot-intake-routing-recommendation.v0.1.md
    - work/STATUS.md
    - work/goals/goal_capture_harness_intake_routing_boundary_v0_1.md
    - work/reports/2026-06-08-harness-intake-routing-boundary-v0-1.md
```

## Verification

Checks run:

- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `python3 scripts/check_docs_governance.py --repo . --files docs/product/PLOT-INTAKE.md docs/product/DOCUMENTATION-MAP.md evals/specs/plot-intake-routing-recommendation.v0.1.md work/goals/goal_capture_harness_intake_routing_boundary_v0_1.md work/reports/2026-06-08-harness-intake-routing-boundary-v0-1.md work/STATUS.md`

All passed with zero warnings after adding the parseable `DocImpact` block.

## Not tested

No Rust tests are required for this docs-only capture. No CLI/runtime behavior
was changed.
