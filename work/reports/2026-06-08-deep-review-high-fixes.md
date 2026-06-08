---
id: report_2026_06_08_deep_review_high_fixes
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_fix_deep_review_high_findings_2026_06_08.md
---

# Deep Review HIGH Fixes 2026-06-08

## Summary

Implemented the high-priority fixes from
`work/reviews/deep-review-2026-06-08.md`.

## What changed

- Hardened `punk-core` file artifact hashing against intermediate-directory
  symlink escapes.
- Added `ProofAcceptanceAuthorityReport` in `punk-proof` to evaluate a real
  `GateDecision` plus `Proofpack` plus required verification evidence.
- Made acceptance authority reject wrong verified digest evidence,
  gate/proof ref mismatches, and duplicate/conflicting declared artifact
  digests.
- Changed referenced-artifact verification evidence so `verified(...)`
  compares expected and observed digests instead of self-asserting from one
  digest.
- Routed `punk-eval` acceptance smoke coverage through the new authority
  report while keeping structural proof readiness visible as structural only.
- Added observable boundary probes to representative eval cases that previously
  relied only on hardcoded false boundary flags.
- Added explicit `gate_approval_is_self_asserted_not_verified` markers to
  Module Host side-effect proposal/preflight surfaces.
- Fixed DocImpact parser tolerance for existing report shapes, synced allowed
  classifications with established reports, and kept Level 0 DocImpact as an
  advisory warning rather than a hard failure.
- Added `scripts/check.sh` targets for `research-gate`, `work-ledger`, and
  `governance-suite`, and wired required governance checks into CI.

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
    - docs/_schema/doc-impact.v0.1.schema.yaml
  scripts:
    - scripts/check_docs_governance.py
    - scripts/check.sh
  workflow:
    - .github/workflows/docs-governance.yml
  rust:
    - crates/punk-core/src/lib.rs
    - crates/punk-proof/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - crates/punk-module-host/src/lib.rs
  work_artifacts:
    - work/reviews/deep-review-2026-06-08.md
    - work/goals/goal_fix_deep_review_high_findings_2026_06_08.md
    - work/reports/2026-06-08-deep-review-high-fixes.md
    - work/STATUS.md
  reason: "Fixes high-priority review findings in proof acceptance, artifact digest containment, governance enforcement, eval observability, and module approval-boundary labeling without activating gate writers, proofpack writers, external publishing, adapters, credentials, or acceptance claims."
```

## Validation

- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `cargo test -p punk-core` - PASS.
- `cargo test -p punk-proof` - PASS.
- `cargo test -p punk-eval` - PASS.
- `cargo test -p punk-module-host` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-core/src/lib.rs --report work/reports/2026-05-05-community-intake-flow-v0-1.md` - PASS.
- `scripts/check.sh docs-governance --files crates/punk-core/src/lib.rs` - PASS with advisory DocImpact warning.
- `scripts/check.sh research-gate` - PASS.
- `scripts/check.sh work-ledger` - PASS.
- `scripts/check.sh module-control-suite` - PASS.
- `scripts/check.sh governance-suite` - PASS.
- `git diff --check` - PASS.

## Remaining scope

This report covers the HIGH slice. Medium and lower findings from the review
remain available for later priority-ordered work.
