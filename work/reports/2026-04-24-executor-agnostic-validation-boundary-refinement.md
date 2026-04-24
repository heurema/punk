---
id: report_2026_04_24_executor_agnostic_validation_boundary_refinement
goal_id: goal_refine_executor_agnostic_validation_boundary
actor: vitaly
created_at: 2026-04-24
kind: handoff
---

## Goal

Refine the previous Contract over Prompt diff from "execution-agnostic" to "executor-agnostic validation" without adding runtime code or new active surfaces.

## Research Gate

Classification: R1
Required: yes
Rationale:
This is a bounded correction to a previous R2 architecture/policy diff. Existing repo-tracked research and the user's clarification are enough; no Deep Research escalation is needed.

Research refs:

- `knowledge/research/2026-04-24-contract-over-prompt.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/RESEARCH-GATE.md`

Decision:
Proceed with docs/ADR/eval-policy correction only.

## What was wrong in the previous wording

- "Punk does not own execution" was too broad because Punk must run validators and capture receipts inside the user's runtime.
- "Execution-agnostic" could imply Punk is detached from execution, when the intended boundary is only executor independence.
- Executor claims were not strong enough as a separate non-proof category.
- Semantic/LLM-based assessment needed an explicit assess-not-decide boundary.
- Research Gate fix order missed receipt/evidence fields between deterministic validators and eval/proof promotion.

## What changed

- Renamed ADR-0014 to `docs/adr/ADR-0014-executor-agnostic-validation-boundary.md`.
- Renamed eval spec to `evals/specs/executor-agnostic-validation-boundary.v0.1.md`.
- Rewrote the architecture boundary around:

```text
Punk is executor-agnostic, not validation-agnostic.
Punk runs in the user's environment, but does not trust the user's executor as truth.
Punk does not own the executor.
Punk owns contract, validation protocol, evidence, gate, proof, and memory.
```

- Updated Punk laws to cover user runtime as substrate, executor claims as non-proof, semantic assessors as advisory evidence, and runner aids as non-truth.
- Updated Research Gate preferred fix order to include receipt/evidence fields.
- Updated Project Memory so executor-local memory becomes project memory only through verified artifact links.
- Updated roadmap, public narrative, crate-status, prior goal/report, and research note references.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no `.punk/` runtime state written;
- no public CLI behavior changed;
- no model runner, provider adapter, MCP integration, prompt manager, skill manager, plugin runtime, or semantic assessor command interface added;
- no contract schema fields were added;
- ADR status remains `Proposed`, not accepted.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Refines the executor-agnostic validation boundary across canonical docs, ADR, eval spec, research note, and Work Ledger artifacts without changing runtime behavior."
  touched_surfaces:
    - docs/product/PUNK-LAWS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/CRATE-STATUS.md
    - docs/adr/ADR-0014-executor-agnostic-validation-boundary.md
    - evals/specs/executor-agnostic-validation-boundary.v0.1.md
    - knowledge/research/2026-04-24-contract-over-prompt.md
  required_updates:
    - docs/product/PUNK-LAWS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/CRATE-STATUS.md
  supersedes:
    - docs/adr/ADR-0014-execution-agnostic-contract-boundary.md
    - evals/specs/execution-agnostic-boundary.v0.1.md
  archive_plan: []
  evals_required:
    - evals/specs/executor-agnostic-validation-boundary.v0.1.md
```

## Deferred

- whether semantic assessors become a generic active-core command interface or stay incubating;
- whether executor self-review is forbidden or allowed only with an advisory risk flag;
- whether `executor_policy`, `validation_runtime`, `semantic_assessment`, and `gate_policy` belong in contracts or later receipt/gate artifacts;
- reject vs defer behavior when a required validator is unavailable in the user runtime;
- where future semantic assessor specs should live.

## Checks run

- `git diff --check`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files docs/product/PUNK-LAWS.md docs/product/ARCHITECTURE.md docs/product/ROADMAP.md docs/product/RESEARCH-GATE.md docs/product/PROJECT-MEMORY.md docs/product/PUBLIC-NARRATIVE.md docs/product/CRATE-STATUS.md docs/adr/ADR-0014-executor-agnostic-validation-boundary.md evals/specs/executor-agnostic-validation-boundary.v0.1.md knowledge/research/2026-04-24-contract-over-prompt.md work/goals/goal_execution_agnostic_contract_boundary.md work/goals/goal_refine_executor_agnostic_validation_boundary.md work/reports/2026-04-24-execution-agnostic-contract-boundary.md work/reports/2026-04-24-executor-agnostic-validation-boundary-refinement.md work/STATUS.md --report work/reports/2026-04-24-executor-agnostic-validation-boundary-refinement.md`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

Result: PASS.

Docs governance emitted 11 pre-existing warning-style findings on touched canonical docs, but 0 failures.
