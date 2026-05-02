# Yesterday Codex Review Sweep and PR #12 Symlink Ancestor Write Guard v0.1

Date: 2026-05-02

## Summary

Checked PRs created, closed, merged, or updated on 2026-05-01 for Codex Review threads.

Yesterday PR set:

| PR | Result |
|---|---|
| #13 `feat(init): add compact greenfield init scaffold` | No review comments, review submissions, or review threads. |
| #12 `chore(work): close contract-core checkpoint series` | One actionable Codex Review P1; fixed in this slice. |
| #11 `test: smoke pr intake gate v0.3.0` | No Codex review threads. One maintainer smoke-close comment only. |
| #10 `security(github): guard pr intake instruction surfaces` | No review comments, review submissions, or review threads. |
| #9 `test: smoke pr intake gate v0.2.0` | No Codex review threads. One maintainer smoke-close comment says `unresolved_codex_threads=0`. |
| #8 `ci: pin pr intake gate v0.2.0` | No review comments, review submissions, or review threads. |
| #3 `chore(github): add deterministic PR intake gate` | Updated on 2026-05-01 and still has an unresolved Codex Review P1, but the finding targets deleted `scripts/pr_intake_gate.py`; the local engine was removed from Punk after migration to the shared `heurema/repo-governance` action. No current-tree Punk fix is available. |

Fixed the actionable P1 Codex Review finding from PR #12:

`https://github.com/heurema/punk/pull/12#discussion_r3173176711`

The first active proofpack writer write slice previously checked only the final parent path for symlink status. A symlink in a non-terminal parent prefix could redirect a nested target outside the intended storage root.

## Files Changed

- `crates/punk-proof/src/lib.rs`
- `work/goals/goal_fix_pr12_codex_review_symlink_ancestor_write_guard_v0_1.md`
- `work/reports/2026-05-02-pr12-codex-review-symlink-ancestor-write-guard-v0-1.md`
- `work/STATUS.md`

## Fix

- Added parent-prefix walking before the write attempt.
- Each storage-root-relative parent prefix is inspected with `symlink_metadata`.
- The write slice now fail-closes with `ParentDirectorySymlink`, `ParentDirectoryNotDirectory`, or `ParentDirectoryMissing` before `OpenOptions::create_new` can write through an unsafe parent chain.
- Kept the existing boundary: no host path canonicalization, no runtime storage, no CLI surface, no gate/proof authority expansion.

## Non-Actionable Current-Tree Findings

PR #3 has an unresolved Codex Review P1:

`https://github.com/heurema/punk/pull/3#discussion_r3169822877`

The finding targets `scripts/pr_intake_gate.py`, specifically rename metadata handling in the old local PR Intake Gate engine. That file no longer exists in Punk. Later work migrated PR Intake Gate execution to `heurema/repo-governance/actions/pr-intake-gate` and removed the local engine/test harness from this repository.

Current Punk evidence:

- `.github/workflows/pr-intake-gate.yml` uses the shared action pinned to commit SHA `7c7c203fbb170abc4c0dcd10201b67d495613aaf`.
- `scripts/pr_intake_gate.py` is absent from the current tree.
- `work/reports/2026-04-30-pr-intake-gate-local-engine-cleanup-v0-1.md` records the local-engine removal.

If the rename-metadata issue still exists, it belongs in `heurema/repo-governance`, not in the current Punk tree.

## Test Added

- `proofpack_writer_first_active_write_slice_blocks_intermediate_parent_symlink`

The test creates an intermediate parent symlink from the storage root to an outside directory and verifies that the write is blocked before any artifact is created outside the storage root.

## Checks

- `cargo test -p punk-proof proofpack_writer_first_active_write_slice_blocks_intermediate_parent_symlink -- --nocapture`
- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `~/.local/bin/punk-dev eval run smoke`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `git diff --check`
- `scripts/check.sh docs-governance --files crates/punk-proof/src/lib.rs work/STATUS.md work/goals/goal_fix_pr12_codex_review_symlink_ancestor_write_guard_v0_1.md work/reports/2026-05-02-pr12-codex-review-symlink-ancestor-write-guard-v0-1.md --report work/reports/2026-05-02-pr12-codex-review-symlink-ancestor-write-guard-v0-1.md`

## Boundaries Preserved

No greenfield init, CLI, runtime storage, gate writer, proof writer expansion, Writer orchestration, acceptance claim writer, brownfield, grayfield, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added.

## Next

Return to `work/goals/goal_pause_after_greenfield_init_checkpoint_v0_1.md` after checks pass.

## Doc impact
```yaml
  classification: code-doc
  reason: "Fixes a code-level PR review finding and updates work-ledger/report evidence without changing product docs or active CLI behavior."
```
