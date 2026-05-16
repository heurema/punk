---
id: report_2026_05_16_publishing_module_boundary_guardrail_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-16
updated_at: 2026-05-16
goal_ref: work/goals/goal_correct_publishing_module_boundary_guardrail_v0_1.md
---

# Publishing Module Boundary Guardrail v0.1

## Summary

Corrected an architectural drift path where publishing inventory was being
routed into active core after the local publishing workspace locator.

No product/runtime/CLI behavior was added.

## Root cause

The concrete cause was a bad handoff in the locator artifacts:

- `punk publishing locate` was added as a transitional core CLI resolver.
- `work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md`
  ended with a next-slice suggestion for a local publishing plan/draft
  inventory reader.
- `work/STATUS.md` repeated that now-withdrawn next proposed publishing slice.
- The handoff did not say that inventory/drafting/planning must move behind
  PubPunk/module-host boundaries.

That made another active-core publishing subcommand look valid even
though `docs/modules/pubpunk.md` and `docs/product/MODULE-HOST.md` keep module
behavior parked.

## Correction made

- Withdrew the previous core publishing inventory follow-up suggestion.
- Clarified that `punk publishing locate` is a transitional resolver only.
- Clarified that publishing inventory, drafting, planning, receipt creation,
  and publish behavior belong to future PubPunk/module-host work.
- Added docs-governance coverage for unguarded non-locate publishing subcommand
  command claims and publishing next-slice handoffs that bypass the module
  boundary.

## Files changed

- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/PUBLIC-NARRATIVE.md`
- `docs/product/MODULE-HOST.md`
- `docs/modules/pubpunk.md`
- `scripts/check_docs_governance.py`
- `work/STATUS.md`
- `work/goals/goal_add_local_publishing_workspace_locator_v0_1.md`
- `work/goals/goal_correct_publishing_module_boundary_guardrail_v0_1.md`
- `work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md`
- `work/reports/2026-05-16-publishing-module-boundary-guardrail-v0-1.md`

## Validation run

Validation was run after the correction. Results:

- `python3 -m py_compile scripts/check_docs_governance.py`: PASS
- `python3 - <<'PY' ... check_publishing_module_boundary(...) ... PY`: PASS
  for a synthetic unguarded publishing inventory subcommand docs/work claim.
- `python3 scripts/check_research_gate.py`: PASS
- `python3 scripts/check_work_ledger.py`: PASS
- `scripts/check.sh docs-governance --files README.md docs/product/START-HERE.md docs/product/ROADMAP.md docs/product/CRATE-STATUS.md docs/product/PUBLIC-NARRATIVE.md docs/product/MODULE-HOST.md docs/modules/pubpunk.md scripts/check_docs_governance.py work/STATUS.md work/goals/goal_add_local_publishing_workspace_locator_v0_1.md work/goals/goal_correct_publishing_module_boundary_guardrail_v0_1.md work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md work/reports/2026-05-16-publishing-module-boundary-guardrail-v0-1.md --report work/reports/2026-05-16-publishing-module-boundary-guardrail-v0-1.md`: PASS with unchanged glossary/definition warnings in `docs/product/PUBLIC-NARRATIVE.md` and `docs/product/MODULE-HOST.md`.
- `git diff --check`: PASS

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Corrects the PubPunk/module-host boundary and adds a docs-governance guard against routing publishing module behavior into active core."
  touched_surfaces:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/MODULE-HOST.md
    - docs/modules/pubpunk.md
    - scripts/check_docs_governance.py
    - work/STATUS.md
    - work/goals/goal_add_local_publishing_workspace_locator_v0_1.md
    - work/goals/goal_correct_publishing_module_boundary_guardrail_v0_1.md
    - work/reports/2026-05-15-local-publishing-workspace-locator-v0-1.md
    - work/reports/2026-05-16-publishing-module-boundary-guardrail-v0-1.md
  required_updates:
    - README.md
    - docs/product/START-HERE.md
    - docs/product/ROADMAP.md
    - docs/product/CRATE-STATUS.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/MODULE-HOST.md
    - docs/modules/pubpunk.md
    - scripts/check_docs_governance.py
    - work/STATUS.md
```

## Knowledge impact

- Canonical product docs changed: `README.md`,
  `docs/product/START-HERE.md`, `docs/product/ROADMAP.md`,
  `docs/product/CRATE-STATUS.md`, `docs/product/PUBLIC-NARRATIVE.md`, and
  `docs/product/MODULE-HOST.md`.
- PubPunk remains parked.
- Module Host remains parked.
- Existing `punk publishing locate` remains the only active publishing-related
  CLI command.
- No additional active-core publishing command was added.
- No Rust code changed.
- No runtime behavior changed.
- No publishing, browser/API, credential, bot, adapter, issue/PR, receipt, gate,
  proofpack, DAO, token, or funding behavior was added.

## Out of scope

- Publishing inventory implementation
- PubPunk runtime
- Module Host activation
- Publishing execution
- External API calls
- Credentials
- Bots/adapters
- Runtime receipts
- CLI command expansion

## Next step

If publishing is selected again, start with a PubPunk/module-host boundary goal:
module manifest shape, capability policy, receipt/assessment shape, and
conformance evals. Do not add another active-core publishing subcommand
first.
