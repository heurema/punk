---
id: report_2026_04_20_review_window_warning_check
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add warning-level `review_after` freshness checks for touched canonical documentation artifacts.

## What changed

- Extended `scripts/check_docs_governance.py` with a warning-level `review_after` check for touched/new canonical docs.
- Added a `--today YYYY-MM-DD` override so date-based checks remain deterministic in eval fixtures.
- Added fixture coverage for a current review window and an expired review window.
- Kept the new behavior advisory-only: expired review windows warn, but do not fail the check.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds warning-level review_after checks for touched canonical documentation artifacts."
  touched_surfaces:
    - eval
    - documentation-map
  required_updates:
    - scripts/check_docs_governance.py
    - evals/cases/docs/review-window/README.md
    - evals/cases/docs/review-window/review-window-current.pass.yaml
    - evals/cases/docs/review-window/review-window-expired.warn.yaml
    - work/reports/2026-04-20-review-window-warning-check.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `python3 - <<'PY' ...` fixture sanity check for file presence, case ids, and expected pass/warning shapes.
- `python3 - <<'PY' ...` temp repo check that validates both current and expired `review_after` behavior with `--today 2026-04-20`.
- `python3 scripts/check_docs_governance.py --files scripts/check_docs_governance.py evals/cases/docs/review-window/README.md evals/cases/docs/review-window/review-window-current.pass.yaml evals/cases/docs/review-window/review-window-expired.warn.yaml work/reports/2026-04-20-review-window-warning-check.md --report work/reports/2026-04-20-review-window-warning-check.md --today 2026-04-20`
- `scripts/check.sh`

## What remains

- Add glossary-term drift warnings as the next heuristic layer.
- Add duplicate definition candidate warnings after glossary coverage is in place.

## Risks

- `review_after` warnings are advisory only; they highlight stale docs but do not prove content is wrong.
- The new `--today` override should stay narrow and only support deterministic date-based checks, not evolve into a general config surface.

## Knowledge updates needed

- None beyond this report, the checker change, and the new fixtures.
