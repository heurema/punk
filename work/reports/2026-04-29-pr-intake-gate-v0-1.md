---
id: report_2026_04_29_pr_intake_gate_v0_1
goal_id: goal_add_pr_intake_gate_v0_1
actor: vitaly
created_at: 2026-04-29
kind: handoff
---

## Summary

Added a deterministic PR Intake Gate v0.1 for Punk, adapted from Signum but tightened for Punk's Work Ledger, Research Gate, and high-risk source-of-truth surfaces.

## Research Gate

Classification: R1
Required: yes
Rationale:
This adopts a bounded PR intake mechanism from Signum and touches GitHub workflow/repo-governance trust surfaces. It does not change Punk runtime, crate, storage, CLI, gate, or proofpack behavior.
Research refs:
- `knowledge/research/2026-04-29-pr-intake-gate-review.md`
- `docs/product/START-HERE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/RESEARCH-GATE.md`
- `CONTRIBUTING.md`
Decision:
Adopt a Punk-specific deterministic gate, not a generic copy of Signum.

## Files Changed

- `.github/pr-intake-gate.yml`
- `.github/workflows/pr-intake-gate.yml`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `scripts/pr_intake_gate.py`
- `scripts/test_pr_intake_gate.py`
- `scripts/check.sh`
- `CONTRIBUTING.md`
- `CHANGELOG.md`
- `.gitignore`
- `knowledge/research/2026-04-29-pr-intake-gate-review.md`
- `work/goals/goal_add_pr_intake_gate_v0_1.md`
- `work/reports/2026-04-29-pr-intake-gate-v0-1.md`
- `work/STATUS.md`

## Implementation Summary

- Added `.github/workflows/pr-intake-gate.yml` using `pull_request_target` with trusted base checkout, pinned `actions/checkout` SHA, and read-only PR permission plus issue-write permission for labels/comments.
- Added `.github/pr-intake-gate.yml` for trivial limits, high-risk path globs, linked-intent patterns, label definitions, and bot-comment marker.
- Added `scripts/pr_intake_gate.py`, a stdlib-only gate that reads PR event metadata and changed-file metadata through GitHub API or dry-run fixtures.
- Added labels/verdicts: `intake/pass`, `intake/needs-linked-intent`, `intake/high-risk`, with trusted auto-creation of configured labels and `maintainer/override-intake` bypass.
- Added Punk-native linked-intent matching for `work/goals/...`, `work/reports/...`, `docs/adr/...`, `knowledge/research/...`, and `evals/specs/...` in addition to GitHub Issues/Discussions.
- Added `scripts/test_pr_intake_gate.py` and `scripts/check.sh pr-intake-gate` for fixture-backed local validation.
- Updated contributor docs, PR template, changelog, research note, goal, report, and work ledger.
- Added `.gitignore` Python bytecode ignores so local `py_compile` validation does not leave temp artifacts to stage.

## Improvements over Signum baseline

- Punk-native linked intent refs are accepted, not only GitHub Issue/Discussion refs.
- High-risk paths map to Punk's source-of-truth and trust surfaces: `.github/**`, `scripts/**`, `crates/**`, `docs/product/**`, `work/**`, `knowledge/**`, `evals/**`, public narrative, site, and brand.
- The local check is wired into `scripts/check.sh pr-intake-gate` for a single repo-native validation command.
- Configured intake labels can be created by the trusted workflow instead of requiring a separate manual label bootstrap.
- Research note records adoption/defer/park/avoid mapping before promotion.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "The diff adds repo-governance automation and contributor-facing process docs without changing canonical product docs or runtime behavior."
  touched_surfaces:
    - "CONTRIBUTING.md"
    - ".github/PULL_REQUEST_TEMPLATE.md"
    - "CHANGELOG.md"
    - ".gitignore"
    - "knowledge/research/2026-04-29-pr-intake-gate-review.md"
    - "work/STATUS.md"
  required_updates:
    - "CONTRIBUTING.md"
    - ".github/PULL_REQUEST_TEMPLATE.md"
    - "CHANGELOG.md"
    - ".gitignore"
    - "work/STATUS.md"
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks Run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `python3 -m py_compile scripts/pr_intake_gate.py scripts/test_pr_intake_gate.py` - PASS
- `scripts/check.sh pr-intake-gate` - PASS
- `scripts/check.sh docs-governance --files .github/PULL_REQUEST_TEMPLATE.md .github/pr-intake-gate.yml .github/workflows/pr-intake-gate.yml .gitignore CHANGELOG.md CONTRIBUTING.md knowledge/research/2026-04-29-pr-intake-gate-review.md scripts/check.sh scripts/pr_intake_gate.py scripts/test_pr_intake_gate.py work/STATUS.md work/goals/goal_add_pr_intake_gate_v0_1.md work/reports/2026-04-29-pr-intake-gate-v0-1.md --report work/reports/2026-04-29-pr-intake-gate-v0-1.md` - PASS, 0 failures, 0 warnings
- `grep -R "$PWD" -n work docs scripts .github AGENTS.md knowledge evals README.md CONTRIBUTING.md CHANGELOG.md || true` - PASS, no absolute local path findings

## Scope Boundaries Preserved

- no Rust code changes;
- no `.punk/` runtime state;
- no product-doc changes under `docs/product/**`;
- no active CLI behavior changes;
- no runtime gate decision or proofpack behavior changes;
- no provider/model/agent execution requirement;
- no branch-protection API mutation.

## Deferred

- Configure GitHub branch protection to require `pr-intake-gate` before merge.
- Add a separate DCO sign-off check if maintainers want automated DCO enforcement.
- Revisit whether small `README.md` changes should become high-risk once public positioning stabilizes.

## Next Recommended Action

Return to `work/goals/goal_run_fifty_fifth_work_ledger_review.md`.
