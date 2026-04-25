---
id: report_2026_04_25_eighth_work_ledger_review
goal_id: goal_run_eighth_work_ledger_review
actor: vitaly
created_at: 2026-04-25
kind: handoff
---

## Summary

Run the eighth advisory Work Ledger Review over the docs/CLI mismatch repair and recent proof/validation boundary sequence.

This review is advisory.
It does not decide acceptance.
It does not create a second live source of truth.
Follow-up work still requires explicit goal selection through the normal Work Ledger flow.

Explicit review conclusions:

- `punk init` docs/CLI mismatch: **repaired as docs-only active-surface wording**;
- current implemented CLI truth: **`punk flow inspect`, `punk eval run smoke`, `punk eval run smoke --format json`**;
- validation/proof boundaries: **coherent enough to support future implementation planning**;
- next safest branch: **docs-governance regression check for active CLI surface truth**.

## Research Gate

Classification: R0
Required: no
Rationale:
This is an advisory review over existing repo-tracked evidence. It changes only work-ledger artifacts and selects the next bounded goal.
Decision:
Proceed.

## Evidence Reviewed

Review range:

- `94e9cc0` - Define proof-before-acceptance semantics
- `1e225cc` - Run seventh Work Ledger Review
- `84d838e` - Reconcile punk init docs CLI mismatch

Repo-tracked evidence:

- `work/STATUS.md`
- `work/goals/goal_reconcile_punk_init_docs_cli_mismatch.md`
- `work/goals/goal_run_eighth_work_ledger_review.md`
- `work/reports/2026-04-25-punk-init-docs-cli-mismatch.md`
- `README.md`
- `docs/product/START-HERE.md`
- `docs/product/DOGFOODING.md`
- `docs/product/FLOW.md`
- `docs/product/CRATE-STATUS.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`
- `evals/specs/minimal-receipt-fields.v0.1.md`

## Findings

### F-001

```yaml
id: F-001
domain: docs
finding: "The `punk init` mismatch is repaired in docs: current implemented CLI surface is explicitly listed, and `punk init` is described as a future setup target rather than current behavior."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/CRATE-STATUS.md
  - docs/product/DOGFOODING.md
  - docs/product/FLOW.md
  - work/reports/2026-04-25-punk-init-docs-cli-mismatch.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Explicit current-vs-future wording protects active-surface honesty."
revisit_trigger: "If docs again list an unimplemented command as active behavior."
```

### F-002

```yaml
id: F-002
domain: validation
finding: "The docs/CLI mismatch was caught by manual review, not by a deterministic docs-governance rule. A regression check should make this class of active-surface overclaim harder to reintroduce."
severity: high
recurrence: first-seen
evidence_strength: direct
evidence_refs:
  - work/reports/2026-04-25-punk-init-docs-cli-mismatch.md
  - scripts/check_docs_governance.py
disposition: bounded-goal
destination:
  type: work-goal
  ref: work/goals/goal_add_active_cli_surface_docs_governance_check.md
driver: vitaly
rationale_prevents_recurrence: "A docs-governance check turns the repair into a regression guard without widening runtime or CLI scope."
revisit_trigger: "If active docs mention unimplemented CLI commands as current behavior."
```

### F-003

```yaml
id: F-003
domain: sequencing
finding: "Runtime storage, receipt schema/runtime, gate/proof implementation, and proofpack writer work remain valuable next branches, but adding the docs-governance guard is smaller and directly follows the repaired failure mode."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - docs/product/ROADMAP.md
  - work/reports/2026-04-25-punk-init-docs-cli-mismatch.md
  - work/STATUS.md
disposition: no-action
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "The guardrail prevents active-surface drift before larger implementation branches."
revisit_trigger: "After the docs-governance guard lands or when a runtime implementation goal is explicitly selected."
```

### F-004

```yaml
id: F-004
domain: process
finding: "The Work Ledger continues to preserve one selected next goal and one live status source after many small docs/spec/review steps."
severity: medium
recurrence: repeated
evidence_strength: direct
evidence_refs:
  - work/STATUS.md
  - docs/product/DOGFOODING.md
  - docs/product/PROJECT-MEMORY.md
disposition: keep-practice
destination:
  type: no-action
  ref: ""
driver: vitaly
rationale_prevents_recurrence: "Keeping the next step explicit avoids hidden planning state while runtime project memory is deferred."
revisit_trigger: "If multiple ready next goals are selected or a second tracker appears."
```

## Candidate Improvements

### CI-001

```yaml
id: CI-001
based_on_findings:
  - F-002
summary: "Create `work/goals/goal_add_active_cli_surface_docs_governance_check.md` as the next bounded governance/eval step."
priority: high
destination:
  type: work-goal
  ref: work/goals/goal_add_active_cli_surface_docs_governance_check.md
why_now: "The docs/CLI mismatch was just repaired; adding a regression guard is smaller and safer than starting runtime implementation immediately."
why_not_now: "Do not implement `punk init` or widen the CLI in this step."
driver: vitaly
```

### CI-002

```yaml
id: CI-002
based_on_findings:
  - F-003
summary: "Keep runtime storage, receipt schema/runtime, gate/proof implementation, proofpack writer, semantic assessor implementation, and GoalRail runtime work deferred until separately selected."
priority: medium
destination:
  type: no-action
  ref: ""
why_now: "Deferral keeps the next step focused on preventing recurrence of an observed docs/CLI truth bug."
why_not_now: "These branches remain valuable but are larger than the guardrail follow-up."
driver: vitaly
```

## Parked Ideas

- runtime Project Memory storage
- `.punk/contracts`, `.punk/evals`, `.punk/runs`, `.punk/decisions`, and `.punk/proofs` activation
- receipt schema/runtime implementation
- gate/proof implementation
- proofpack writer implementation
- semantic assessor implementation
- GoalRail runtime pilot
- possible future `punk init`
- service-backed project memory

## No Action

- Do not change runtime/product CLI in this review.
- Do not implement `punk init`.
- Do not write `.punk/` state.
- Do not select runtime storage yet.
- Do not implement gate/proof/proofpack behavior yet.
- Do not fold GoalRail runtime work into Punk's active-core scope.

## Next Recommended Action

Recommend: `work/goals/goal_add_active_cli_surface_docs_governance_check.md`
Defer: runtime storage, receipt schema/runtime, gate/proof implementation, proofpack writer, semantic assessor implementation, GoalRail runtime work, `punk init`
Park: service-backed storage as authority

Rationale:

- the active CLI mismatch was an observed docs-governance failure mode;
- current CLI truth is now explicit but should be guarded;
- a docs-governance regression check is narrower than runtime storage or gate/proof implementation.

## Doc impact

```yaml
doc_impact:
  classification: none
  reason: "This diff adds a bounded advisory review and work-ledger artifacts without changing canonical product docs, specs, research notes, runtime code, schemas, or CLI behavior."
  touched_surfaces:
    - work/reports/2026-04-25-eighth-work-ledger-review.md
    - work/goals/goal_run_eighth_work_ledger_review.md
    - work/goals/goal_add_active_cli_surface_docs_governance_check.md
    - work/STATUS.md
  required_updates: []
  supersedes: []
  archive_plan: []
  evals_required: []
```

## Checks run

- `git diff --check` - PASS
- `python3 scripts/check_research_gate.py` - PASS
- `python3 scripts/check_work_ledger.py` - PASS
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_run_eighth_work_ledger_review.md work/goals/goal_add_active_cli_surface_docs_governance_check.md work/reports/2026-04-25-eighth-work-ledger-review.md --report work/reports/2026-04-25-eighth-work-ledger-review.md` - PASS
- `cargo test --workspace` - PASS
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true` - PASS, no absolute repo paths found

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no schema changed;
- no `.punk` runtime state written;
- no runtime storage implemented;
- no receipt schema/runtime implemented;
- no gate/proof/proofpack implementation added;
- no semantic assessor implementation added;
- no CLI command added;
- no `punk init` command added;
- no provider/model/agent adapter added;
- no automation added.
