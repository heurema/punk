---
id: report_2026_04_29_truth_baseline_docs_v0_1
goal_id: goal_truth_baseline_docs_v0_1
actor: vitaly
created_at: 2026-04-29
kind: docs-truth-alignment
---

## Summary

Aligned the repo entry, changelog, lineage note, documentation map, and work ledger around the current implemented surface and the v0.1-prep Current Truth Baseline stage.

This pass makes the current executable CLI surface explicit, keeps broader architecture docs framed as target/phase-gated/parked/future unless marked active, and records the lineage boundary for adjacent repositories without treating them as current `punk` truth.

No product feature, runtime behavior, CLI command, `.punk/` storage, proofpack writer, gate writer, adapter, provider/model runner, autonomous agent execution, or publishing automation was added.

## Research Gate

Classification: R1
Required: yes
Rationale: bounded docs/work-ledger alignment using repo-tracked canonical docs and maintainer/user-approved audit synthesis; no new architecture/runtime behavior.

Decision: Proceed.

Research refs used:

- `README.md`
- `AGENTS.md`
- `work/STATUS.md`
- `docs/product/START-HERE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/DOGFOODING.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/PUBLIC-NARRATIVE.md`
- `CONTRIBUTING.md`

## Scope Override

The previous `selected_next` was `work/goals/goal_define_proofpack_writer_host_path_resolution_boundary_v0_1.md`.

This PR intentionally overrides that `selected_next` because the maintainer/user requested a truth-baseline documentation alignment pass before continuing feature/boundary progression.

After this docs pass, `work/STATUS.md` returns `selected_next` to `work/goals/goal_define_proofpack_writer_host_path_resolution_boundary_v0_1.md` so the paused proofpack-writer boundary track remains visible and not deleted.

## Changed Files

- `README.md`
- `CHANGELOG.md`
- `docs/product/LINEAGE.md`
- `docs/product/DOCUMENTATION-MAP.md`
- `work/STATUS.md`
- `work/goals/goal_truth_baseline_docs_v0_1.md`
- `work/reports/2026-04-29-truth-baseline-docs-v0-1.md`

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Aligned the repo entry, lineage, changelog, and live work ledger with the current implemented CLI surface and v0.1-prep truth-baseline stage."
  touched_surfaces:
    - README.md
    - CHANGELOG.md
    - docs/product/LINEAGE.md
    - docs/product/DOCUMENTATION-MAP.md
    - work/STATUS.md
    - work/goals/goal_truth_baseline_docs_v0_1.md
    - work/reports/2026-04-29-truth-baseline-docs-v0-1.md
  required_updates:
    - README.md
    - CHANGELOG.md
    - docs/product/LINEAGE.md
    - docs/product/DOCUMENTATION-MAP.md
    - work/STATUS.md
    - work/goals/goal_truth_baseline_docs_v0_1.md
    - work/reports/2026-04-29-truth-baseline-docs-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - git diff --check
    - python3 scripts/check_research_gate.py
    - python3 scripts/check_work_ledger.py
    - scripts/check.sh docs-governance --files README.md CHANGELOG.md docs/product/LINEAGE.md docs/product/DOCUMENTATION-MAP.md work/STATUS.md work/goals/goal_truth_baseline_docs_v0_1.md work/reports/2026-04-29-truth-baseline-docs-v0-1.md --report work/reports/2026-04-29-truth-baseline-docs-v0-1.md
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files README.md CHANGELOG.md docs/product/LINEAGE.md docs/product/DOCUMENTATION-MAP.md work/STATUS.md work/goals/goal_truth_baseline_docs_v0_1.md work/reports/2026-04-29-truth-baseline-docs-v0-1.md --report work/reports/2026-04-29-truth-baseline-docs-v0-1.md` - PASS with 0 failures and 1 warning: `DOC_GLOSSARY_TERM_UNDECLARED` for existing `docs/product/DOCUMENTATION-MAP.md` heading `Research notes`.
- `cargo check --workspace` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repository path leaks reported.

`last_validated_commit` remains `null` because this validation was run against the working tree before a commit was created.

## Acceptance Notes

- README states the current implemented CLI surface exactly as `punk flow inspect`, `punk eval run smoke`, and `punk eval run smoke --format json`.
- README states that `punk init`, `.punk/` runtime storage, active proofpack writing, provider/model/agent execution, and PubPunk publishing automation are not active.
- `docs/product/LINEAGE.md` records adjacent-repo lineage as advisory input, not current implementation truth.
- `CHANGELOG.md` records the pre-1.0 truth-baseline documentation stage.
- `work/STATUS.md` records the maintainer/user-approved selected-next override and returns the paused proofpack-writer host path resolution goal to `selected_next` after completion.
