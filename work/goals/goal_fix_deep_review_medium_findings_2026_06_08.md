---
id: goal_fix_deep_review_medium_findings_2026_06_08
title: "Fix deep review medium findings 2026-06-08"
status: done
owner: "vitaly"
module: "trust-governance"
priority: P1
authority: canonical
created_at: 2026-06-08
updated_at: 2026-06-08
selected_at: 2026-06-08
started_at: 2026-06-08
completed_at: 2026-06-08
blocked_by: []
scope:
  include:
    - "work/reviews/deep-review-2026-06-08.md"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "crates/punk-flow/src/lib.rs"
    - "scripts/check_research_gate.py"
    - "scripts/check_work_ledger.py"
    - "scripts/check_module_control_markers.py"
    - "scripts/check_docs_governance.py"
    - "docs/product/CRATE-STATUS.md"
    - "work/goals/goal_fix_deep_review_medium_findings_2026_06_08.md"
    - "work/reports/2026-06-08-deep-review-medium-fixes.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "publishing/**"
    - "knowledge/**"
acceptance:
  - "Make proof hash/reference integration fail closed when required verification evidence is missing or incomplete."
  - "Reject stale canonical proofpack manifest bodies that no longer match the live proofpack under evaluation."
  - "Make structural link-hash integrity incomplete for duplicate or conflicting declared artifact digests."
  - "Document that structural proof readiness predicates are not content verification or acceptance authority."
  - "Enforce selected/current R1+ required research refs in the research-gate checker."
  - "Extend work-ledger existence checks to contract and proof refs."
  - "Make module-control marker checks robust to key/value spacing, case, and separator variants for forbidden active markers."
  - "Warn when DocImpact classification does not match code, Cargo, or architecture-significant changed files."
  - "Do not let active CLI future-context windows hide present-tense unimplemented command claims."
  - "Add back-half flow lifecycle and block/escalate/cancel matrix coverage while keeping flow states non-acceptance authority."
  - "Keep eval cleanup visible in diagnostics without making host cleanup success part of pass/fail."
  - "Improve eval case granularity and remove duplicated smoke assessment coverage text."
  - "Mark the empty punk-rules crate as parked in CRATE-STATUS."
knowledge_refs: []
contract_refs:
  - "work/reviews/deep-review-2026-06-08.md"
report_refs:
  - "work/reports/2026-06-08-deep-review-medium-fixes.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R2
  required: true
  rationale: "This slice implements the user-provided deep review's medium-priority trust, governance, flow, eval, and crate-status findings. The review artifact is repo-tracked input evidence for the bounded fixes."
  research_refs:
    - "work/reviews/deep-review-2026-06-08.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "crates/punk-flow/src/lib.rs"
    - "scripts/check_research_gate.py"
    - "scripts/check_work_ledger.py"
    - "scripts/check_module_control_markers.py"
    - "scripts/check_docs_governance.py"
    - "docs/product/CRATE-STATUS.md"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Fixes medium-priority review findings in proof hash/reference integration, governance evasion checks, flow lifecycle coverage, eval harness quality, and crate-status truth without activating runtime storage, gate writers, proofpack writers, external publishing, adapters, credentials, or acceptance claims."
---

## Context

The user supplied `work/reviews/deep-review-2026-06-08.md` and asked to fix
the findings in priority order starting from high.

This goal records the bounded MEDIUM slice after the HIGH slice was recorded
separately.

## Notes

- Review source: `work/reviews/deep-review-2026-06-08.md`.
- This slice keeps gate/proof runtime, proofpack writer expansion, acceptance
  claim writing, adapter invocation, browser automation, credentials, external
  publishing, and `.punk/` runtime state inactive.
