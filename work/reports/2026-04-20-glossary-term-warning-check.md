---
id: report_2026_04_20_glossary_term_warning_check
goal_id: goal_bootstrap_punk_core
actor: vitaly
created_at: 2026-04-20
kind: handoff
---

## Goal

Add warning-level glossary term consistency checks for touched canonical docs.

## What changed

- Extended `scripts/check_docs_governance.py` with heading-only glossary term warnings for touched/new canonical docs.
- Used `docs/product/GLOSSARY.md` as the sole term authority.
- Added warning coverage for undeclared term headings and redefined known glossary terms.
- Added a pass case for a known glossary term heading that only links back to the glossary.
- Kept the new behavior advisory-only: glossary drift warns, but does not fail the check.

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds warning-level glossary term consistency checks for touched canonical docs."
  touched_surfaces:
    - eval
    - documentation-map
    - glossary
  required_updates:
    - scripts/check_docs_governance.py
    - evals/cases/docs/glossary/README.md
    - evals/cases/docs/glossary/glossary-known-term.pass.yaml
    - evals/cases/docs/glossary/glossary-undeclared-term.warn.yaml
    - evals/cases/docs/glossary/glossary-term-redefined.warn.yaml
    - work/reports/2026-04-20-glossary-term-warning-check.md
  supersedes: []
  archive_plan: []
  evals_required:
    - docs-governance-checks
```

## Checks run

- `python3 - <<'PY' ...` fixture sanity check for file presence, case ids, and expected pass/warning shapes.
- `python3 - <<'PY' ...` temp repo check that validates pass, undeclared-term warning, and redefined-term warning behavior.
- `python3 scripts/check_docs_governance.py --files scripts/check_docs_governance.py evals/cases/docs/glossary/README.md evals/cases/docs/glossary/glossary-known-term.pass.yaml evals/cases/docs/glossary/glossary-undeclared-term.warn.yaml evals/cases/docs/glossary/glossary-term-redefined.warn.yaml work/reports/2026-04-20-glossary-term-warning-check.md --report work/reports/2026-04-20-glossary-term-warning-check.md`
- `scripts/check.sh`

## What remains

- Add duplicate definition candidate warnings as the next heuristic layer.
- Revisit broader free-text or backtick-term detection only if heading-only coverage proves too weak.

## Risks

- Heading-only glossary detection is intentionally narrow; it catches obvious term drift but not all term misuse in prose.
- The definition-like heuristic is conservative; some subtle redefinitions may stay unflagged until later if they avoid explicit definition wording.

## Knowledge updates needed

- None beyond this report, the checker change, and the new fixtures.
