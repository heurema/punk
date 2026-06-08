---
id: report_2026_06_08_deep_review_medium_fixes
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_fix_deep_review_medium_findings_2026_06_08.md
---

# Deep Review MEDIUM Fixes 2026-06-08

## Summary

Implemented the medium-priority fixes from
`work/reviews/deep-review-2026-06-08.md` after the HIGH slice.

## What changed

- Made proof hash/reference integration fail closed when required referenced
  artifact verification coverage is missing.
- Blocked stale proofpack canonical manifest bodies that no longer match the
  live proofpack under evaluation.
- Added duplicate/conflicting declared digest tracking to structural proofpack
  integrity.
- Added rustdoc clarifying that structural proof readiness is not content
  verification or acceptance authority.
- Hardened governance checks for selected/current R1+ research refs, dangling
  contract/proof refs, module-control marker variants, DocImpact classification
  mismatch warnings, and active CLI future-window suppression.
- Added explicit flow back-half lifecycle and block/escalate/cancel matrix
  tests while keeping `Reported`/`Closed` non-acceptance states.
- Improved eval harness diagnostics by naming failing cases, removing cleanup
  success from pass/fail conditions, and centralizing the long assessment body.
- Updated `docs/product/CRATE-STATUS.md` so the skeleton `punk-rules` crate is
  `parked`.

## Boundary confirmation

- No gate decision writer was added.
- No proofpack writer expansion was added.
- No acceptance claim writer was added.
- No adapter invocation, browser automation, credential access, external
  publishing, runtime storage, or `.punk/` runtime state was added.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  canonical_docs:
    - docs/product/CRATE-STATUS.md
  scripts:
    - scripts/check_research_gate.py
    - scripts/check_work_ledger.py
    - scripts/check_module_control_markers.py
    - scripts/check_docs_governance.py
  rust:
    - crates/punk-proof/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - crates/punk-flow/src/lib.rs
  work_artifacts:
    - work/reviews/deep-review-2026-06-08.md
    - work/goals/goal_fix_deep_review_medium_findings_2026_06_08.md
    - work/reports/2026-06-08-deep-review-medium-fixes.md
    - work/STATUS.md
  reason: "Fixes medium-priority review findings in proof hash/reference integration, governance evasion checks, flow lifecycle coverage, eval harness quality, and crate-status truth without activating runtime storage, gate writers, proofpack writers, external publishing, adapters, credentials, or acceptance claims."
```

## Validation

- `cargo test -p punk-proof -- --nocapture` - PASS.
- `cargo test -p punk-flow -- --nocapture` - PASS.
- `cargo test -p punk-eval -- --nocapture` - PASS.
- `scripts/check.sh governance-suite` - PASS.

## Remaining scope

This report covers the MEDIUM slice. Low and informational findings from the
review remain available for later priority-ordered work.
