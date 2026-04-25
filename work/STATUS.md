---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: vitaly
ledger_version: work-ledger.v0.1
dogfooding_level: 0
updated_at: 2026-04-25
current_phase: "Dogfooding Level 0 / Phase 3 contract-loop bootstrap"
current_focus: "Define Project Memory storage boundary v0.1"
selected_next: "work/goals/goal_define_project_memory_storage_boundary_v0_1.md"
last_validated_commit: null
---

# Work Status

## Now

- Current focus: define Project Memory storage boundary v0.1.
- Selected next: `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`
- Why this is next: the R2 storage research chose a layered direction; before implementation, Punk needs a docs/spec-only boundary and eval cases for Markdown authority, JSONL events, SQLite derived indexes, and service-backed non-authority.
- Acceptance:
  - `work/STATUS.md` remains the only live work-state source of truth.
  - `selected_next` points to one `ready` goal.
  - Project Memory storage boundary v0.1 is defined before runtime storage implementation.
  - repo Markdown authority, JSONL runtime-event boundary, SQLite derived-index boundary, and service-backed non-authority are covered by spec/eval cases.
  - contract, flow, event, eval, receipt, decision, and proof stay bounded to their documented surfaces until later goals explicitly activate implementation.
  - process-shell reuse stays setup-neutral: no required IDE, CLI ritual, model, provider, prompt, skill, or local runtime setup.
  - `.punk/contracts`, `.punk/evals`, `.punk/runs`, `.punk/decisions`, `.punk/proofs`, gate, proof, Event Ledger runtime work, GoalRail runtime work, and `punk init` remain deferred until later bounded goals explicitly activate them.

## Next Candidates

| Goal | Status | Why candidate | Blocked by |
|---|---|---|---|
| `work/goals/goal_define_project_memory_storage_boundary_v0_1.md` | `ready` | The R2 storage research is complete; next step is a docs/spec-only boundary before runtime storage. | — |

## Blocked

| Item | Blocked by | Needed to unblock |
|---|---|---|
| Gate or proof implementation | future bounded gate/proof goals | Reconcile proofpack minimum metadata and `proof before acceptance` semantics before runtime closure work. |
| `.punk/contracts`, `.punk/evals`, `.punk/runs`, `.punk/decisions`, or `.punk/proofs` storage | `work/goals/goal_define_project_memory_storage_boundary_v0_1.md` | Define Project Memory storage boundary v0.1 and eval cases before activating runtime storage. |
| Process capture inbox or Event Ledger research | repeated evidence of capture or inspectability failure | Revisit only if the process shell or a later review shows a repeated gap. |
| GoalRail runtime pilot | future gate/proof/storage closure and GoalRail-specific selected goal | Keep GoalRail limited to process-shell reuse until runtime authority surfaces exist. |
| `punk init` docs/CLI mismatch | separate docs/CLI reconciliation goal | `docs/product/START-HERE.md` lists `punk init`, but current CLI exposes only flow inspect and smoke eval surfaces. |

## Done Recently

| Date | Item | Evidence |
|---|---|---|
| 2026-04-25 | Researched task/work storage before Project Memory implementation | `work/goals/goal_research_task_storage_before_project_memory.md`, `knowledge/research/2026-04-25-task-work-storage-before-project-memory.md`, `docs/adr/ADR-0015-project-memory-storage-direction.md`, `work/reports/2026-04-25-task-work-storage-research.md` |
| 2026-04-25 | Extracted the GoalRail process-only shell from Punk's work-ledger discipline | `work/goals/goal_extract_goalrail_process_shell_pilot.md`, `work/pilots/goalrail-process-shell.md`, `work/reports/2026-04-25-goalrail-process-shell-pilot.md` |
| 2026-04-25 | Tightened public site problem copy to executor-neutral wording | `work/goals/goal_public_site_executor_neutral_problem_copy.md`, `work/reports/2026-04-25-public-site-executor-neutral-problem-copy.md`, `site/src/components/Problem.astro` |
| 2026-04-24 | Made public site copy executor-neutral | `work/goals/goal_public_site_executor_neutral_copy.md`, `work/reports/2026-04-24-public-site-executor-neutral-copy.md`, `site/src/data/content.ts`, `site/src/components/Hero.astro`, `site/src/components/HowSection.astro`, `site/src/components/ModulesSection.astro`, `site/src/layouts/Layout.astro` |
| 2026-04-24 | Refined executor-agnostic validation boundary wording and evidence model | `work/goals/goal_refine_executor_agnostic_validation_boundary.md`, `work/reports/2026-04-24-executor-agnostic-validation-boundary-refinement.md`, `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`, `evals/specs/executor-agnostic-validation-boundary.v0.1.md` |
| 2026-04-24 | Adopted initial Contract over Prompt boundary as docs/ADR/eval-policy only | `work/goals/goal_execution_agnostic_contract_boundary.md`, `work/reports/2026-04-24-execution-agnostic-contract-boundary.md`, `knowledge/research/2026-04-24-contract-over-prompt.md`, `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`, `evals/specs/executor-agnostic-validation-boundary.v0.1.md` |
| 2026-04-23 | Ran the fourth advisory Work Ledger Review | `work/goals/goal_run_fourth_work_ledger_review.md`, `work/reports/2026-04-22-fourth-work-ledger-review.md` |
| 2026-04-23 | Added the repository open-source baseline | `work/goals/goal_add_open_source_repository_baseline.md`, `work/reports/2026-04-23-open-source-repository-baseline.md`, `README.md` |
| 2026-04-23 | Defined the proofpack boundary v0.1 spec | `work/goals/goal_define_proofpack_boundary_v0_1.md`, `work/reports/2026-04-22-proofpack-boundary-v0-1.md`, `evals/specs/proofpack-boundary.v0.1.md` |
| 2026-04-23 | Researched proofpack boundary | `work/goals/goal_research_proofpack_boundary.md`, `work/reports/2026-04-22-proofpack-boundary-research.md`, `knowledge/research/2026-04-22-proofpack-boundary.md` |
| 2026-04-23 | Defined the gate decision boundary v0.1 spec | `work/goals/goal_define_gate_decision_boundary_v0_1.md`, `work/reports/2026-04-22-gate-decision-boundary-v0-1.md`, `evals/specs/gate-decision-boundary.v0.1.md` |
| 2026-04-23 | Researched gate decision boundary | `work/goals/goal_research_gate_decision_boundary.md`, `work/reports/2026-04-22-gate-decision-boundary-research.md`, `knowledge/research/2026-04-22-gate-decision-boundary.md` |
| 2026-04-23 | Added run receipt smoke eval coverage | `work/goals/goal_add_run_receipt_smoke_eval.md`, `work/reports/2026-04-22-run-receipt-smoke-eval.md`, `crates/punk-eval/src/lib.rs` |
| 2026-04-23 | Connected run receipt to contract and flow | `work/goals/goal_connect_run_receipt_to_contract_flow.md`, `work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md`, `crates/punk-flow/src/lib.rs` |
| 2026-04-22 | Added the minimal run receipt kernel | `work/goals/goal_add_run_receipt_minimal.md`, `work/reports/2026-04-22-run-receipt-minimal.md`, `crates/punk-domain/src/lib.rs` |
| 2026-04-22 | Ran the third advisory Work Ledger Review | `work/goals/goal_run_third_work_ledger_review.md`, `work/reports/2026-04-22-third-work-ledger-review.md` |
| 2026-04-22 | Defined the run receipt boundary v0.1 spec | `work/goals/goal_define_run_receipt_boundary_v0_1.md`, `work/reports/2026-04-22-run-receipt-boundary-v0-1.md`, `evals/specs/run-receipt-boundary.v0.1.md` |
| 2026-04-22 | Researched run receipt boundary | `work/goals/goal_research_run_receipt_boundary.md`, `work/reports/2026-04-22-run-receipt-boundary-research.md`, `knowledge/research/2026-04-22-run-receipt-boundary.md` |
| 2026-04-22 | Added contract-flow smoke eval coverage | `work/goals/goal_add_contract_flow_smoke_eval.md`, `work/reports/2026-04-22-contract-flow-smoke-eval.md`, `crates/punk-eval/src/lib.rs` |
| 2026-04-22 | Connected contract status to flow guard state | `work/goals/goal_connect_contract_to_flow_state.md`, `work/reports/2026-04-22-connect-contract-to-flow-state.md`, `crates/punk-flow/src/lib.rs` |
| 2026-04-22 | Added the minimal contract lifecycle kernel | `work/goals/goal_add_contract_lifecycle_minimal.md`, `work/reports/2026-04-22-contract-lifecycle-minimal.md`, `crates/punk-contract/src/lib.rs` |
| 2026-04-22 | Ran the second advisory Work Ledger Review | `work/goals/goal_run_second_work_ledger_review.md`, `work/reports/2026-04-22-second-work-ledger-review.md` |

## Validation

- Last checked: 2026-04-25
- Command: `git diff --check && python3 scripts/check_research_gate.py && python3 scripts/check_work_ledger.py && scripts/check.sh docs-governance --files knowledge/research/2026-04-25-task-work-storage-before-project-memory.md docs/adr/ADR-0015-project-memory-storage-direction.md docs/product/PROJECT-MEMORY.md work/goals/goal_research_task_storage_before_project_memory.md work/goals/goal_define_project_memory_storage_boundary_v0_1.md work/reports/2026-04-25-task-work-storage-research.md work/STATUS.md --report work/reports/2026-04-25-task-work-storage-research.md && cargo test --workspace && grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals site/src || true`
- Result: `PASS`
- Notes:
  - `selected_next` is now `work/goals/goal_define_project_memory_storage_boundary_v0_1.md`
  - docs governance passed with one warning for pre-existing `Project coherence` duplicate-definition candidate in `docs/product/PROJECT-MEMORY.md`
  - ADR-0015 is proposed only; future `gate` still owns final acceptance
  - `.punk/`, runtime gate/proof, Event Ledger runtime, schema, CLI, adapter, automation, service-backed storage, and `punk init` work remain deferred
