---
id: report_2026_04_20_duplicate_definition_warning_check
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add warning-level duplicate definition candidate checks for touched canonical docs.

## What changed

- Extended `scripts/check_docs_governance.py` with a low-noise duplicate-definition candidate warning.
- Scoped the v0 rule to definition-like H2 sections outside `docs/product/GLOSSARY.md`.
- Kept shared term definitions glossary-only for v0.
- Added fixture coverage for:
  - absent duplicate-definition warning;
  - duplicate-definition candidate warning;
  - glossary definitions remaining allowed.
- Kept the new behavior advisory-only: duplicate definition candidates warn, but do not fail the check.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds warning-level duplicate definition candidate checks for touched canonical docs."
  touched_surfaces:
    - eval
    - glossary
    - documentation-map
  required_updates:
    - scripts/check_docs_governance.py
    - evals/cases/docs/duplicate-definition/README.md
    - evals/cases/docs/duplicate-definition/duplicate-definition-absent.pass.yaml
    - evals/cases/docs/duplicate-definition/duplicate-definition-candidate.warn.yaml
    - evals/cases/docs/duplicate-definition/glossary-definition-allowed.pass.yaml
    - work/reports/2026-04-20-duplicate-definition-warning-check.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `python3 - <<'PY' ...` fixture sanity check for file presence, case ids, and expected pass/warning shapes.
- `python3 - <<'PY' ...` temp repo check that validates absent-warning, duplicate-definition warning, and glossary-allowed behavior.
- `python3 scripts/check_docs_governance.py --files scripts/check_docs_governance.py evals/cases/docs/duplicate-definition/README.md evals/cases/docs/duplicate-definition/duplicate-definition-absent.pass.yaml evals/cases/docs/duplicate-definition/duplicate-definition-candidate.warn.yaml evals/cases/docs/duplicate-definition/glossary-definition-allowed.pass.yaml work/reports/2026-04-20-duplicate-definition-warning-check.md --report work/reports/2026-04-20-duplicate-definition-warning-check.md`
- `scripts/check.sh`

## What remains

- Write a `docs-governance-v0-status` consolidation report instead of adding more heuristics immediately.
- Defer any broader prose-level duplicate-definition detection until there is evidence this narrow H2 rule is insufficient.

## Risks

- The H2-only duplicate-definition candidate rule is intentionally narrow; some duplicate definitions may remain unflagged until a later iteration.
- If real exceptions emerge for shared term definitions outside the glossary, they should be handled with a narrow allowlist rather than relaxing the rule broadly.

## Knowledge updates needed

- None beyond this report, the checker change, and the new fixtures.
