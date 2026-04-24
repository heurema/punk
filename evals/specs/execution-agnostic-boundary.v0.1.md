# Execution-agnostic boundary eval spec v0.1

Date: 2026-04-24
Status: proposed eval backlog
Authority: advisory/design

## Purpose

Define the consistency checks that should protect Punk's execution-agnostic boundary before any model runner, provider adapter, prompt manager, skill manager, or coding-agent execution path is promoted.

These evals assess docs and policy consistency.

They do not accept work.

## Boundary under test

```text
Punk does not own execution.
Punk owns contract, evidence, validation, gate, proof, and memory.
```

Executors may be humans, local models, coding agents, scripts, IDEs, future modules, or adapters.

Executor internals are non-authoritative unless captured as scoped evidence.

## Scope

The eval family covers:

- current docs do not claim Punk executes coding-agent work today;
- prompts, skills, playbooks, and repository instructions are advisory runner aids unless explicitly promoted;
- model/provider settings do not become project truth;
- executor output is evidence only after receipt/eval/gate/proof linkage;
- only `gate` writes final decisions;
- repeated executor failures route into contract clauses, validators, eval cases, proof requirements, or project memory before global instructions.

## Hard-gate candidates

### execution_no_active_agent_claim

Purpose: prevent docs from presenting coding-agent execution as a current operator path.

Expected result:

- active docs keep autonomous coding-agent execution, provider adapters, MCP integration, and prompt/skill product surfaces parked or deferred;
- roadmap phase text does not move agent execution into Phase 3.

Catches:

- accidental active-surface promotion;
- public/operator copy that overstates current capabilities.

### execution_runner_aids_non_authoritative

Purpose: keep AGENTS files, skills, prompts, and playbooks from becoming hidden product truth.

Expected result:

- runner-aid files identify themselves as advisory;
- canonical docs/ADRs/contracts/evals/decisions/proofpacks remain authority surfaces;
- runner-aid text cannot override Punk Laws or gate exclusivity.

Catches:

- local instruction files silently defining architecture;
- skills accumulating permanent product rules without evidence.

### execution_provider_settings_non_truth

Purpose: prevent provider/model defaults from becoming acceptance criteria.

Expected result:

- docs treat provider behavior, model settings, and prompt scaffolds as non-authoritative;
- adapter docs say adapters may invoke but cannot own truth.

Catches:

- model-specific rituals promoted into active-core design;
- provider-specific execution assumptions hiding inside product docs.

### execution_output_evidence_not_acceptance

Purpose: preserve evidence vs acceptance.

Expected result:

- executor output becomes Punk evidence only through scoped artifacts, receipts, validator outputs, eval reports, proofpacks, or gate decisions;
- no executor can write final decisions directly.

Catches:

- coding-agent output treated as accepted work;
- receipts/evals/proofs collapsed into decision authority.

### execution_failure_promotion_order

Purpose: prevent global instruction bloat after local failures.

Expected result:

- repeated executor failures are first captured as contract clauses, validators, eval/regression cases, proof requirements, project-memory updates, or scoped runner aids;
- global instructions are last resort and carry scope, owner, evidence, and review/retirement path.

Catches:

- prompt accretion replacing deterministic checks;
- permanent instructions without failure/eval evidence.

## Initial deterministic inputs

- changed paths from the current diff;
- `docs/product/PUNK-LAWS.md`;
- `docs/product/ARCHITECTURE.md`;
- `docs/product/ROADMAP.md`;
- `docs/product/RESEARCH-GATE.md`;
- `docs/product/PROJECT-MEMORY.md`;
- `docs/product/CRATE-STATUS.md`;
- `AGENTS.md`;
- `.agents/skills/punk-workflow/SKILL.md`;
- related report `DocImpact`.

## Non-goals

This spec does not define:

- model execution;
- provider adapters;
- prompt management;
- skill management;
- MCP integration;
- runtime storage;
- gate/proof implementation;
- a public CLI command.

## Rollout

1. Use this spec as manual review criteria in the current docs-only patch.
2. Add deterministic docs/policy checks later if repeated drift appears.
3. Add contract/receipt schema checks only when the roadmap promotes the relevant runtime surface.
