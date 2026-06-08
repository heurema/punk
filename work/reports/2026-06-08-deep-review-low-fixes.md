---
id: report_2026_06_08_deep_review_low_fixes
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
goal_ref: work/goals/goal_fix_deep_review_low_findings_2026_06_08.md
---

# Deep Review LOW Fixes 2026-06-08

## Summary

Implemented the low-priority fixes from
`work/reviews/deep-review-2026-06-08.md` after the HIGH and MEDIUM slices.
Small low-risk informational notes were addressed where the fix was bounded,
local, and did not activate deferred runtime behavior.

## What changed

- Made `punk-events` derive the next append sequence from the last stored event
  record and reject torn or malformed JSONL tails.
- Derived flow event phase from the attempted command instead of hardcoding
  every transition event as `cut`.
- Removed the unreachable synthetic flow/event mapping error path.
- Marked parked event replay variants as reserved/not emitted and removed the
  `punk-events` replay-support overclaim from `CRATE-STATUS`.
- Removed the unused `punk-events` dependency from `punk-cli`.
- Aligned CLI usage/help/docs/tests for `--mode <greenfield|brownfield>`,
  explicit `--mode greenfield`, malformed `flow`, bare help, `--help`/`-h`,
  and unsupported eval formats.
- Replaced stringly local-pointer error classification with typed config value
  errors and rejected control characters in local publishing workspace roots.
- Ignored local-only publishing pointer/residue paths with
  `.punk/publishing.local.toml` and `.punk/publishing/`.
- Tightened repo-relative/path-like refs in contract/domain surfaces and
  Windows-drive-like logical refs in Module Host/PubPunk without rejecting
  existing logical `proofpack:` or `punk-publishing://` refs.
- Added UTC-second timestamp validation for gate decisions and proofpacks.
- Made proof manifest rendering stable across ref/digest insertion order.
- Made backslash target-path rejection also surface storage-root escape.
- Hardened docs-governance parsing for DocImpact heading variants, fenced/inline
  code references, inline-flow frontmatter lists, parked-capability negation,
  active CLI command wording, and documentation precedence drift.
- Recorded no-clobber/no-final-symlink invariants, contract guard relevance,
  artifact-path boundary intent, and the implicit `artifact_hashes` receipt
  fallback in code comments/tests.

## Deferred structural work

- No crate split, large safe-ref abstraction, or module-host/pubpunk finding
  model consolidation was attempted in this slice.
- No formal shared CLI parser was introduced beyond the bounded eval/help/usage
  diagnostics.
- No top-level integration-test crate was added; coverage was kept to focused
  unit/smoke tests around the reviewed gaps.
- `punk-rules` remains parked until real rule logic exists; the earlier MEDIUM
  slice fixed the current-status overclaim.

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
    - docs/product/START-HERE.md
  scripts:
    - scripts/check_docs_governance.py
  rust:
    - crates/punk-cli/src/main.rs
    - crates/punk-contract/src/lib.rs
    - crates/punk-core/src/lib.rs
    - crates/punk-domain/src/lib.rs
    - crates/punk-events/src/lib.rs
    - crates/punk-flow/src/lib.rs
    - crates/punk-gate/src/lib.rs
    - crates/punk-mod-pubpunk/src/lib.rs
    - crates/punk-module-host/src/lib.rs
    - crates/punk-project/src/lib.rs
    - crates/punk-proof/src/lib.rs
  work_artifacts:
    - work/reviews/deep-review-2026-06-08.md
    - work/goals/goal_fix_deep_review_low_findings_2026_06_08.md
    - work/reports/2026-06-08-deep-review-low-fixes.md
    - work/STATUS.md
  reason: "Fixes low-priority review findings in event sequencing, flow event phases, CLI grammar/help fidelity, local publishing pointer hardening, safe-ref policies, timestamp validation, proof manifest canonicalization, docs-governance parsing, and future-safety comments without activating runtime storage, gate writers, proofpack writers, external publishing, adapters, credentials, or acceptance claims."
```

## Validation

- `cargo fmt --all` - PASS.
- `cargo test -p punk-events -p punk-flow -p punk-cli -p punk-project -p punk-contract -p punk-domain -p punk-gate -p punk-proof -p punk-module-host -p punk-mod-pubpunk -- --nocapture` - PASS.
- `cargo test -p punk-cli -p punk-project -p punk-contract -p punk-core -p punk-flow -p punk-module-host -- --nocapture` - PASS.
- `cargo test --workspace -- --nocapture` - PASS.
- `scripts/check.sh governance-suite` - PASS.
- `scripts/check.sh research-gate` - PASS.
- `scripts/check.sh work-ledger` - PASS.
- `scripts/check.sh docs-governance --files .github/workflows/docs-governance.yml .gitignore AGENTS.md Cargo.lock README.md crates/punk-cli/Cargo.toml crates/punk-cli/src/main.rs crates/punk-contract/src/lib.rs crates/punk-core/src/lib.rs crates/punk-domain/src/lib.rs crates/punk-eval/src/lib.rs crates/punk-events/src/lib.rs crates/punk-flow/src/lib.rs crates/punk-gate/src/lib.rs crates/punk-mod-pubpunk/src/lib.rs crates/punk-module-host/src/lib.rs crates/punk-project/src/lib.rs crates/punk-proof/Cargo.toml crates/punk-proof/src/lib.rs docs/_schema/doc-impact.v0.1.schema.yaml docs/product/CRATE-STATUS.md docs/product/START-HERE.md scripts/check.sh scripts/check_docs_governance.py scripts/check_module_control_markers.py scripts/check_research_gate.py scripts/check_work_ledger.py work/STATUS.md work/reviews/deep-review-2026-06-08.md work/goals/goal_fix_deep_review_high_findings_2026_06_08.md work/goals/goal_fix_deep_review_medium_findings_2026_06_08.md work/goals/goal_fix_deep_review_low_findings_2026_06_08.md work/reports/2026-06-08-deep-review-high-fixes.md work/reports/2026-06-08-deep-review-medium-fixes.md work/reports/2026-06-08-deep-review-low-fixes.md --report work/reports/2026-06-08-deep-review-low-fixes.md` - PASS.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `git diff --check` - PASS.

## Remaining scope

This report covers the LOW slice plus bounded low-risk informational fixes.
Large structural refactors from the review remain intentionally separate
future work because they would broaden the architecture and review scope.
