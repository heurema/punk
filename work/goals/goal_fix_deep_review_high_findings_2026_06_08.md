---
id: goal_fix_deep_review_high_findings_2026_06_08
title: "Fix deep review high findings 2026-06-08"
status: done
owner: "vitaly"
module: "trust-kernel"
priority: P0
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
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-proof/Cargo.toml"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "crates/punk-module-host/src/lib.rs"
    - "scripts/check_docs_governance.py"
    - "scripts/check.sh"
    - ".github/workflows/docs-governance.yml"
    - "docs/_schema/doc-impact.v0.1.schema.yaml"
    - "Cargo.lock"
    - "work/goals/goal_fix_deep_review_high_findings_2026_06_08.md"
    - "work/reports/2026-06-08-deep-review-high-fixes.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "publishing/**"
    - "docs/modules/**"
    - "knowledge/**"
acceptance:
  - "Reject intermediate-directory symlink escapes in file artifact digest verification."
  - "Add a single proof acceptance authority report over a real gate decision, proofpack, declared digests, and required verification evidence."
  - "Reject duplicate or conflicting declared proof artifact digests in the acceptance authority path."
  - "Make referenced-artifact verification evidence compare expected and observed digests instead of self-asserting from one digest."
  - "Route eval acceptance smoke coverage through the new authority report."
  - "Make DocImpact parsing tolerate existing top-level and nested report blocks and sync the schema enum with established report practice."
  - "Keep DocImpact advisory at Dogfooding Level 0 while preserving warning signal."
  - "Expose research-gate, work-ledger, and governance-suite check targets and run required governance checks in CI."
  - "Add observable boundary probes to representative eval no-gate/no-proof/no-writer cases."
  - "Mark module side-effect gate approval coverage as self-asserted, not verified."
knowledge_refs: []
contract_refs:
  - "work/reviews/deep-review-2026-06-08.md"
report_refs:
  - "work/reports/2026-06-08-deep-review-high-fixes.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R2
  required: true
  rationale: "This slice implements the user-provided deep review's high-priority trust-kernel findings. The review artifact is repo-tracked input evidence for the bounded fixes."
  research_refs:
    - "work/reviews/deep-review-2026-06-08.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-proof/src/lib.rs"
    - "crates/punk-eval/src/lib.rs"
    - "crates/punk-module-host/src/lib.rs"
    - "scripts/check_docs_governance.py"
    - "scripts/check.sh"
    - ".github/workflows/docs-governance.yml"
    - "docs/_schema/doc-impact.v0.1.schema.yaml"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Fixes high-priority review findings in proof acceptance, artifact digest containment, governance enforcement, eval observability, and module approval-boundary labeling without activating gate writers, proofpack writers, external publishing, adapters, credentials, or acceptance claims."
---

## Context

The user supplied `work/reviews/deep-review-2026-06-08.md` and asked to fix
the findings in priority order starting from high.

This goal records the bounded HIGH slice only.

## Notes

- Review source: `work/reviews/deep-review-2026-06-08.md`.
- Medium and lower findings remain out of this goal unless selected later.
- No `.punk/` runtime state, publishing behavior, adapter invocation, gate
  writer, proofpack writer, or acceptance claim is activated.
