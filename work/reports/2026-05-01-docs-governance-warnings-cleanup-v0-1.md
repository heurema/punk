---
id: report_2026_05_01_docs_governance_warnings_cleanup_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_cleanup_docs_governance_warnings_v0_1.md
---

# Docs Governance Warnings Cleanup v0.1

## Summary

- Reviewed the four known accepted/deferred docs-governance warnings.
- Fixed all four with minimal wording changes in existing docs sections.
- Preserved current product meaning and active/non-active boundaries.

## Warnings Inspected

- `docs/product/CRATE-STATUS.md`: `Current implemented subset boundary` duplicate-definition candidate.
- `docs/product/CRATE-STATUS.md`: `Current CLI surface` duplicate-definition candidate.
- `docs/product/DOCUMENTATION-MAP.md`: `Research notes` undeclared glossary-term candidate.
- `docs/product/PROJECT-MEMORY.md`: `Project coherence` duplicate-definition candidate.

## Warnings Fixed

- `docs/product/CRATE-STATUS.md`: changed definition-like intro wording in `Current implemented subset boundary`.
- `docs/product/CRATE-STATUS.md`: changed definition-like intro wording in `Current CLI surface`.
- `docs/product/DOCUMENTATION-MAP.md`: changed the `Research notes` update-rule intro from a term definition shape to an instruction shape.
- `docs/product/PROJECT-MEMORY.md`: changed the `Project coherence` intro from a definition shape to a review-time question shape.

## Warnings Left Accepted/Deferred

- None in the scoped docs-governance warning set.

## Files Changed

- `docs/product/CRATE-STATUS.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/PROJECT-MEMORY.md`
- `work/goals/goal_cleanup_docs_governance_warnings_v0_1.md`
- `work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md`
- `work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md`
- `work/STATUS.md`

## Product Behavior Confirmation

No new product behavior was added.

No init behavior, runtime storage, Writer, gate writer, proof writer, proofpack writer, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was activated.

Current truth remains:

- `punk init <project-id>` is active only as minimal greenfield compact `.punk/memory/` scaffold behavior.
- Brownfield and grayfield modes remain deferred.
- `.punk/runtime` remains inactive.
- Replayable Project Memory remains advisory direction.
- Contract-core runtime and Writer behavior remain inactive.

## Checks Run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md docs/product/DOCUMENTATION-MAP.md docs/product/PROJECT-MEMORY.md work/STATUS.md work/goals/goal_cleanup_docs_governance_warnings_v0_1.md work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md --report work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md` - PASS with 0 failures and 0 warnings.
- `scripts/check.sh docs-governance --files docs/product/CRATE-STATUS.md docs/product/DOCUMENTATION-MAP.md docs/product/PROJECT-MEMORY.md --report work/reports/2026-05-01-docs-governance-warnings-cleanup-v0-1.md` - PASS with 0 failures and 0 warnings.

## Drift Observed

- No product-behavior drift observed.
- The scoped docs-governance warnings were wording-shape warnings, not product truth conflicts.

## Next Selected Goal

`work/goals/goal_verify_greenfield_init_after_compact_layout_v0_1.md`

## Doc impact

```yaml
  classification: docs-only
  reason: "Surgically cleans docs-governance wording warnings without changing product behavior or active scope."
```
