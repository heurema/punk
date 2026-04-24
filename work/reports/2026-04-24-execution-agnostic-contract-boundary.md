---
id: report_2026_04_24_execution_agnostic_contract_boundary
goal_id: goal_execution_agnostic_contract_boundary
actor: vitaly
created_at: 2026-04-24
kind: handoff
---

## Goal

Adopt `Contract over Prompt` as an execution-agnostic active-core boundary without adding model execution, provider adapters, prompt management, skill management, MCP integration, or runtime code.

## Research Gate

Classification: R2
Required: yes
Rationale:
This changes core laws, architecture, roadmap phase boundaries, Research Gate policy, project memory, and runner-aid authority boundaries.

Research refs:

- `knowledge/research/2026-04-24-contract-over-prompt.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ROADMAP.md`
- `docs/product/PROJECT-MEMORY.md`

Decision:
Proceed with docs/ADR/eval-policy boundary only, because adequate user-provided research and repo canonical context were available.

## What changed

- Added laws 44–48 to `docs/product/PUNK-LAWS.md` for contract-as-control-surface, replaceable execution runtime, non-authoritative prompts/skills/provider settings, and failure-to-validator/eval/proof/memory promotion.
- Added `Execution-agnostic boundary` to `docs/product/ARCHITECTURE.md`.
- Clarified Phase 3, Phase 7, and Phase 9 execution stance in `docs/product/ROADMAP.md`.
- Added Research Gate trigger/policy for persistent model-control artifacts in `docs/product/RESEARCH-GATE.md`.
- Added prompt/skill memory boundary to `docs/product/PROJECT-MEMORY.md`.
- Clarified public narrative should describe Punk as bounding/verifying proof-bearing work, not a specific model/provider/executor.
- Added crate-status note that no active-core runner/prompt/skill/provider crate should be introduced in the current phase.
- Added advisory authority headers to `AGENTS.md` and `.agents/skills/punk-workflow/SKILL.md`.
- Created `knowledge/research/2026-04-24-contract-over-prompt.md`.
- Created `docs/adr/ADR-0014-execution-agnostic-contract-boundary.md` with `Status: Proposed`.
- Created `evals/specs/execution-agnostic-boundary.v0.1.md`.

## Exact new law/principle text

```md
44. The task contract is the primary control surface; prompts, skills, playbooks, and model/provider settings are not project truth.
45. Execution runtime is replaceable and non-authoritative. Humans, local models, coding agents, scripts, modules, and adapters may attempt work, but only scoped artifacts, receipts, evals, decisions, and proofpacks become Punk evidence.
46. Punk verifies results; it does not depend on or silently govern a user's local model setup.
47. Persistent model-control artifacts require explicit scope, owner, failure/eval refs, and a review or retirement path.
48. A repeated executor failure becomes a contract clause, validator, eval case, proof requirement, or memory artifact before it becomes a global instruction.
```

Architecture principle added:

```text
Punk does not own execution.
Punk owns contract, evidence, validation, gate, proof, and memory.
```

## Why no code changes were made

The active goal is an architecture/law clarification and Research Gate policy boundary.

Code would be premature because Phase 3 must prove the contract loop without agents, and provider adapters / Dev module behavior stay later roadmap work.

## Future phases that implement runtime consequences

- Phase 3: receipt/evidence capture and gate verification for manual/BYO execution.
- Phase 7: Dev module may use external coding runtimes while preserving receipt/eval/gate/proof authority.
- Phase 9: adapters may invoke provider/tool capabilities without owning truth.

## Doc impact

```yaml
doc_impact:
  classification: architecture
  reason: "Promotes a researched execution-agnostic Contract over Prompt boundary into canonical product docs, ADR, eval spec, Research Gate policy, and advisory runner-aid headers."
  touched_surfaces:
    - docs/product/PUNK-LAWS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/CRATE-STATUS.md
    - docs/adr/ADR-0014-execution-agnostic-contract-boundary.md
    - evals/specs/execution-agnostic-boundary.v0.1.md
  required_updates:
    - docs/product/PUNK-LAWS.md
    - docs/product/ARCHITECTURE.md
    - docs/product/ROADMAP.md
    - docs/product/RESEARCH-GATE.md
    - docs/product/PROJECT-MEMORY.md
    - docs/product/PUBLIC-NARRATIVE.md
    - docs/product/CRATE-STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - evals/specs/execution-agnostic-boundary.v0.1.md
```

## Checks run

- `git diff --check`
- `python3 scripts/check_research_gate.py`
- `python3 scripts/check_work_ledger.py`
- `scripts/check.sh docs-governance --files docs/product/PUNK-LAWS.md docs/product/ARCHITECTURE.md docs/product/ROADMAP.md docs/product/RESEARCH-GATE.md docs/product/PROJECT-MEMORY.md docs/product/PUBLIC-NARRATIVE.md docs/product/CRATE-STATUS.md docs/adr/ADR-0014-execution-agnostic-contract-boundary.md evals/specs/execution-agnostic-boundary.v0.1.md AGENTS.md .agents/skills/punk-workflow/SKILL.md knowledge/research/2026-04-24-contract-over-prompt.md work/goals/goal_execution_agnostic_contract_boundary.md work/reports/2026-04-24-execution-agnostic-contract-boundary.md work/STATUS.md --report work/reports/2026-04-24-execution-agnostic-contract-boundary.md`
- `cargo check --workspace`
- `cargo run -q -p punk-cli -- eval run smoke`
- `grep -R "$PWD" -n work docs scripts .agents AGENTS.md knowledge evals || true`
- `git diff --name-only`

Result: PASS.

Docs governance emitted 11 pre-existing warning-style findings on touched canonical docs, but 0 failures.

## Scope boundaries preserved

- no Rust code changed;
- no `Cargo.toml` or `Cargo.lock` changed;
- no `.punk/` runtime state written;
- no public CLI behavior changed;
- no model runner, provider adapter, MCP integration, prompt manager, skill manager, or plugin runtime added;
- no claim that Punk executes coding-agent work today;
- ADR status is `Proposed`, not accepted.

## What remains

- Future `gate` must accept/reject ADR-0014 before it is final decision truth.
- Future runtime work may add receipt metadata or validators only through separate bounded goals.
- The previous `selected_next`, `work/goals/goal_extract_goalrail_process_shell_pilot.md`, remains the next ready goal after this interrupted task.

## Risks

- Docs now carry a strong policy boundary that future implementation must uphold.
- The eval spec is advisory until a deterministic checker is implemented.
- If runner-aid lifecycle metadata becomes important, a future goal should define where that metadata lives.

## Knowledge updates needed

- Save an architecture memory observation for the execution-agnostic contract boundary.
