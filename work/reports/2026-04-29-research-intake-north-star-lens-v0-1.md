---
id: report_2026_04_29_research_intake_north_star_lens_v0_1
goal_id: manual_research_intake_north_star_lens_v0_1
actor: vitaly
created_at: 2026-04-29
kind: docs-governance
---

## Summary

This patch adds the North Star as an explicit Research Intake lens for classifying external ideas.

- no runtime behavior was added;
- no code was changed;
- no CLI surface was changed;
- no roadmap phase order was changed;
- no crate status was changed;
- no `.punk/` runtime storage was activated;
- no provider/model/agent/module/adapter behavior was activated;
- `work/STATUS.md` selected_next was not changed.

## Scope

Allowed:

- add a short North Star lens to `docs/product/RESEARCH-INTAKE.md`;
- add this report with DocImpact.

Not allowed:

- edit README.md;
- edit ROADMAP.md;
- edit CRATE-STATUS.md;
- edit PUNK-LAWS.md;
- edit ARCHITECTURE.md;
- edit PUBLIC-NARRATIVE.md;
- edit work/STATUS.md;
- edit crates;
- edit evals;
- edit `.punk/`;
- create modules, adapters, CLI commands, runtime schemas, or implementation code.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Added the North Star as an explicit Research Intake lens for external idea classification without changing active scope."
  touched_surfaces:
    - docs/product/RESEARCH-INTAKE.md
    - docs/product/NORTH-STAR.md
    - work/reports/2026-04-29-research-intake-north-star-lens-v0-1.md
  required_updates:
    - docs/product/RESEARCH-INTAKE.md
    - work/reports/2026-04-29-research-intake-north-star-lens-v0-1.md
  supersedes: []
  archive_plan: []
  evals_required:
    - git diff --check
    - python3 scripts/check_research_gate.py
    - python3 scripts/check_work_ledger.py
    - scripts/check.sh docs-governance --files docs/product/RESEARCH-INTAKE.md docs/product/NORTH-STAR.md work/reports/2026-04-29-research-intake-north-star-lens-v0-1.md --report work/reports/2026-04-29-research-intake-north-star-lens-v0-1.md
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files docs/product/RESEARCH-INTAKE.md docs/product/NORTH-STAR.md work/reports/2026-04-29-research-intake-north-star-lens-v0-1.md --report work/reports/2026-04-29-research-intake-north-star-lens-v0-1.md` - PASS with 0 failures and 2 warnings.

## Drift observed

- Existing/unrelated `DOC_DUPLICATE_DEFINITION_CANDIDATE` warning for `Adopt criteria` in `docs/product/RESEARCH-INTAKE.md`.
- Existing/unrelated `DOC_DUPLICATE_DEFINITION_CANDIDATE` warning for `Current parked mechanisms` in `docs/product/RESEARCH-INTAKE.md`.
