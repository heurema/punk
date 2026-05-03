---
id: report_2026_05_03_brownfield_init_scaffold_verification_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_verify_brownfield_init_scaffold_v0_1.md
---

# Brownfield Init Scaffold Verification v0.1

## Summary

- Verified landed PR #16 for B0 brownfield init scaffold.
- Confirmed `punk init <project-id> --mode brownfield` creates only an advisory reconstruction workspace.
- Confirmed greenfield default behavior remains unchanged.
- Confirmed no repo scan, AI summary, claims, contracts/specs, runtime stores, gate/proof artifacts, Writer behavior, grayfield behavior, or source inventory behavior was activated.

## PR And CI

- PR: `https://github.com/heurema/punk/pull/16`
- Merge commit: `b09f41d2f0f5e2d5c8228fb5b8e77a699c173e06`
- Merge method: GitHub PR rebase merge.
- CI checks before merge: `Analyze (python)`, `pr-intake-gate`, and `changed-files-check` passed.
- PR comments, reviews, and review threads were empty before merge.

## Scaffold Shape

Fresh brownfield init in an empty temp directory passed.

Verified files:

```text
.punk/README.md
.punk/project.toml
.punk/memory/STATUS.md
.punk/memory/goals/goal_brownfield_reconstruction_baseline.md
.punk/memory/reports/
.punk/memory/reconstruction/README.md
.punk/memory/reconstruction/source-corpus-manifest.md
.punk/memory/reconstruction/claim-ledger.md
.punk/memory/reconstruction/unknowns.md
.punk/memory/reconstruction/contradictions.md
.punk/memory/reconstruction/contract-readiness.md
```

Verified `.punk/project.toml` records:

```text
entry_mode = "brownfield"
reconstruction_status = "not_started"
authority = "advisory_candidates_only"
```

## Greenfield Unchanged

Fresh greenfield init passed.

- `punk init demo-project` still records `entry_mode = "greenfield"`.
- Greenfield init does not create `.punk/memory/reconstruction/` placeholders.

## No Scan, No AI, No Claims

Brownfield init in a non-empty temp directory with dummy `src/` and `docs/` content passed.

Search under `.punk/` found no matches for:

```text
secret-ish
old docs
src/app.rs
docs/README.md
```

This verifies B0 brownfield scaffold did not inventory, scan, summarize, or import repo content.

## Conflict Atomicity

Brownfield conflict test passed.

- Pre-existing modified `.punk/memory/STATUS.md` caused `result: blocked`.
- The existing file content stayed unchanged.
- No other scaffold files were created.

## Git Visibility

After `git init` in a temp directory, `.punk/memory/reconstruction/README.md` and other `.punk/memory/**` files appeared in `git status --short --untracked-files=all`.

The scaffold remains trackable project memory, not ignored runtime state.

## Runtime Absence

Verified absent after brownfield init:

```text
.punk/events/
.punk/runs/
.punk/decisions/
.punk/proofs/
.punk/runtime/
.punk/cache/
.punk/contracts/
.punk/evals/
.punk/views/
.punk/indexes/
```

## Docs Wording

Docs grep confirmed current docs use scaffold, `not_started`, and advisory-only language for brownfield.

A targeted overclaim grep found no active docs wording that says Punk understands existing projects, reconstructs existing projects, creates project memory from existing code, generates contracts/specs, or creates brownfield claims. The only matching text is the prior implementation report explicitly saying that those phrases were not found.

## Checks Run

- `gh pr checks 16 --watch --interval 10` - PASS before merge.
- `gh pr view 16 --json number,url,state,isDraft,mergeStateStatus,reviewDecision,reviews,comments,statusCheckRollup` - PASS; no comments or reviews before merge.
- `gh api graphql ... reviewThreads` - PASS; no review threads before merge.
- Manual fresh brownfield init in empty temp dir - PASS.
- Manual brownfield init in non-empty temp dir with no-scan grep - PASS.
- Manual fresh greenfield init - PASS.
- Manual brownfield conflict atomic noop - PASS.
- Manual `.punk/memory/**` git visibility check - PASS.
- Manual runtime absence check - PASS.
- Docs scaffold/advisory wording grep - PASS.
- Docs overclaim grep - PASS.
- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo fmt --check` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_verify_brownfield_init_scaffold_v0_1.md work/goals/goal_prepare_brownfield_inventory_boundary_v0_1.md work/reports/2026-05-03-brownfield-init-scaffold-verification-v0-1.md --report work/reports/2026-05-03-brownfield-init-scaffold-verification-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next Selected Goal

`work/goals/goal_prepare_brownfield_inventory_boundary_v0_1.md`

This next goal must define the inventory boundary before any source corpus inventory implementation.

## Boundaries Preserved

No brownfield reconstruction, repo scan, AI summaries, source inventory generation, claim extraction, contract generation, spec generation, gate/proof runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was activated.

## Doc impact

```yaml
classification: work-ledger-only
reason: "Records verification evidence for the landed B0 brownfield init scaffold and selects the next boundary-only goal."
```
