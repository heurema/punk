---
id: goal_fix_pr12_codex_review_symlink_ancestor_write_guard_v0_1
title: "Sweep yesterday Codex Review threads and fix PR #12 symlink ancestor guard v0.1"
status: done
owner: "vitaly"
module: "proof"
priority: P1
authority: canonical
created_at: 2026-05-02
updated_at: 2026-05-02
selected_at: 2026-05-02
started_at: 2026-05-02
completed_at: 2026-05-02
blocked_by: []
scope:
  include:
    - "crates/punk-proof/src/lib.rs"
    - "work/goals/goal_fix_pr12_codex_review_symlink_ancestor_write_guard_v0_1.md"
    - "work/reports/2026-05-02-pr12-codex-review-symlink-ancestor-write-guard-v0-1.md"
    - "work/STATUS.md"
  exclude:
    - "crates/punk-project/**"
    - "crates/punk-cli/**"
    - "crates/punk-eval/**"
    - "docs/product/**"
    - ".punk/**"
acceptance:
  - "PRs created, closed, merged, or updated on 2026-05-01 are checked for Codex Review threads."
  - "The first active proofpack writer write slice rejects symlinks in every storage-root-relative parent ancestor before writing."
  - "A regression test covers a non-terminal parent symlink redirecting a nested target outside the storage root."
  - "Non-actionable stale threads are recorded with rationale instead of forcing unrelated repo changes."
  - "No greenfield init, CLI, runtime storage, gate writer, proof writer expansion, Writer orchestration, acceptance claim writer, brownfield, grayfield, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior is added."
knowledge_refs:
  - "https://github.com/heurema/punk/pull/3#discussion_r3169822877"
  - "https://github.com/heurema/punk/pull/12#discussion_r3173176711"
contract_refs: []
report_refs:
  - "work/reports/2026-05-02-pr12-codex-review-symlink-ancestor-write-guard-v0-1.md"
decision_refs: []
proof_refs: []
latest_proof_ref: null
supersedes: []
superseded_by: null
research_gate:
  classification: R1
  required: true
  rationale: "This is a repo-local fix for a concrete Codex Review finding on a merged PR; no external product research is needed."
  research_refs:
    - "https://github.com/heurema/punk/pull/12#discussion_r3173176711"
    - "work/STATUS.md"
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: code-doc
  required_updates:
    - "work/STATUS.md"
    - "work/reports/2026-05-02-pr12-codex-review-symlink-ancestor-write-guard-v0-1.md"
  rationale: "The code fix needs a work-ledger record and report, but it does not change product docs or active CLI behavior."
---

## Summary

Checked the PRs created, closed, merged, or updated on 2026-05-01 for Codex Review threads. PR #12 had the only actionable current-tree finding.

The first active proofpack writer write slice now walks each storage-root-relative parent prefix with `symlink_metadata` before writing. This blocks non-terminal parent symlinks instead of only checking the final parent path.

## Boundaries

No greenfield init, CLI, runtime storage, gate writer, proof writer expansion, Writer orchestration, acceptance claim writer, brownfield, grayfield, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added.
