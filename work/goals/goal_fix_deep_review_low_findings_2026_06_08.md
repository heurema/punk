---
id: goal_fix_deep_review_low_findings_2026_06_08
title: "Fix deep review low findings 2026-06-08"
status: done
owner: "vitaly"
module: "trust-maintenance"
priority: P2
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
    - ".gitignore"
    - "AGENTS.md"
    - "README.md"
    - "crates/punk-cli/Cargo.toml"
    - "crates/punk-cli/src/main.rs"
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-domain/src/lib.rs"
    - "crates/punk-events/src/lib.rs"
    - "crates/punk-flow/src/lib.rs"
    - "crates/punk-gate/src/lib.rs"
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-module-host/src/lib.rs"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-proof/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/START-HERE.md"
    - "scripts/check_docs_governance.py"
    - "work/goals/goal_fix_deep_review_low_findings_2026_06_08.md"
    - "work/reports/2026-06-08-deep-review-low-fixes.md"
    - "work/STATUS.md"
  exclude:
    - ".punk/**"
    - "publishing/**"
    - "knowledge/**"
acceptance:
  - "Close the concrete low-priority correctness, security, CLI, event, governance, and documentation fidelity findings from the review."
  - "Make local flow-event append sequencing fail closed on torn/malformed tails."
  - "Derive flow event phase from the attempted command instead of hardcoding cut."
  - "Keep parked event replay variants visibly reserved and remove replay-support overclaiming from crate status."
  - "Remove dead CLI dependency coupling and keep CLI accepted grammar/help/tests aligned for init, help, flow usage, and eval format errors."
  - "Reject unsafe control characters in local publishing workspace roots and ignore local-only publishing pointer/residue paths."
  - "Tighten repo-relative/logical refs, timestamp shapes, and proof manifest canonicalization without activating runtime writers."
  - "Make governance parsing/checks robust for DocImpact heading variants, fenced-code refs, inline flow lists, parked negation, and documentation precedence drift."
  - "Record low-risk informational invariants in code comments/tests where they affect future safety."
knowledge_refs: []
contract_refs:
  - "work/reviews/deep-review-2026-06-08.md"
report_refs:
  - "work/reports/2026-06-08-deep-review-low-fixes.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: []
research_gate:
  classification: R2
  required: true
  rationale: "This slice implements the user-provided deep review's low-priority and low-risk informational hardening findings after the HIGH and MEDIUM slices. The review artifact is repo-tracked input evidence for the bounded fixes."
  research_refs:
    - "work/reviews/deep-review-2026-06-08.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - ".gitignore"
    - "AGENTS.md"
    - "README.md"
    - "crates/punk-cli/src/main.rs"
    - "crates/punk-contract/src/lib.rs"
    - "crates/punk-core/src/lib.rs"
    - "crates/punk-domain/src/lib.rs"
    - "crates/punk-events/src/lib.rs"
    - "crates/punk-flow/src/lib.rs"
    - "crates/punk-gate/src/lib.rs"
    - "crates/punk-mod-pubpunk/src/lib.rs"
    - "crates/punk-module-host/src/lib.rs"
    - "crates/punk-project/src/lib.rs"
    - "crates/punk-proof/src/lib.rs"
    - "docs/product/CRATE-STATUS.md"
    - "docs/product/START-HERE.md"
    - "scripts/check_docs_governance.py"
    - "work/STATUS.md"
    - "work/reports/**"
  rationale: "Fixes low-priority review findings in event sequencing, flow event phases, CLI grammar/help fidelity, local publishing pointer hardening, safe-ref policies, timestamp validation, proof manifest canonicalization, docs-governance parsing, and future-safety comments without activating runtime storage, gate writers, proofpack writers, external publishing, adapters, credentials, or acceptance claims."
---

## Context

The user supplied `work/reviews/deep-review-2026-06-08.md` and asked to fix
the findings in priority order starting from high.

This goal records the bounded LOW slice after the HIGH and MEDIUM slices were
recorded separately.

## Notes

- Review source: `work/reviews/deep-review-2026-06-08.md`.
- This slice also records small low-risk informational hardening where the
  review note had an immediate no-runtime fix.
- Large structural refactors remain separate future work when they would change
  architecture, ownership, or review scope.
- This slice keeps gate/proof runtime, proofpack writer expansion, acceptance
  claim writing, adapter invocation, browser automation, credentials, external
  publishing, and `.punk/` runtime state inactive.
