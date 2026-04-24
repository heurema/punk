# Executor-agnostic validation boundary eval spec v0.1

Date: 2026-04-24
Status: proposed eval backlog
Authority: advisory/design

## Purpose

Define consistency checks that should protect Punk's executor-agnostic validation boundary before any model runner, provider adapter, prompt manager, skill manager, semantic assessor interface, or coding-agent execution path is promoted.

These evals assess docs and policy consistency.

They do not accept work.

## Boundary under test

```text
Punk is executor-agnostic, not validation-agnostic.
Punk runs in the user's environment, but does not trust the user's executor as truth.
Punk does not own the executor.
Punk owns contract, validation protocol, evidence, gate, proof, and memory.
```

Executors may be humans, local models, coding agents, scripts, IDEs, future modules, or adapters.

The user runtime is the substrate. It is not the authority.

Executor internals and executor claims are non-authoritative unless captured as scoped evidence and verified or explicitly marked unverified.

## Scope

The eval family covers:

- current docs do not claim Punk executes coding-agent work today;
- prompts, skills, playbooks, and repository instructions are advisory runner aids unless explicitly promoted;
- model/provider settings do not become project truth;
- user runtime is described as validation substrate, not acceptance authority;
- executor claims are not proof;
- semantic or LLM-based assessments are advisory evidence, not decisions;
- only `gate` writes final decisions;
- repeated executor failures route into contract clauses, validators, receipt/evidence fields, eval cases, proof requirements, or project memory before global instructions.

## Hard-gate candidates

### executor_no_active_agent_claim

Purpose: prevent docs from presenting coding-agent execution as a current operator path.

Expected result:

- active docs keep autonomous coding-agent execution, provider adapters, MCP integration, and prompt/skill product surfaces parked or deferred;
- roadmap phase text does not move agent execution into Phase 3.

Catches:

- accidental active-surface promotion;
- public/operator copy that overstates current capabilities.

### executor_runner_aids_non_authoritative

Purpose: keep AGENTS files, skills, prompts, and playbooks from becoming hidden product truth.

Expected result:

- runner-aid files identify themselves as advisory;
- canonical docs/ADRs/contracts/evals/decisions/proofpacks remain authority surfaces;
- runner-aid text cannot override Punk Laws or gate exclusivity.

Catches:

- local instruction files silently defining architecture;
- skills accumulating permanent product rules without evidence.

### executor_user_runtime_substrate_not_authority

Purpose: preserve the distinction between user runtime capability and acceptance authority.

Expected result:

- docs may say Punk runs validators in the user's environment;
- docs must not say the user's executor, local agent session, provider CLI, IDE, or toolchain is the truth root;
- docs must keep contract, receipt/evidence, eval/assessment, gate, proof, and memory links as the authority path.

Catches:

- overcorrecting into "Punk is detached from execution";
- local tool availability becoming acceptance authority.

### executor_provider_settings_non_truth

Purpose: prevent provider/model defaults from becoming acceptance criteria.

Expected result:

- docs treat provider behavior, model settings, and prompt scaffolds as non-authoritative;
- adapter docs say adapters may invoke but cannot own truth.

Catches:

- model-specific rituals promoted into active-core design;
- provider-specific execution assumptions hiding inside product docs.

### executor_claims_not_proof

Purpose: preserve evidence vs acceptance.

Expected result:

- executor output becomes Punk evidence only through scoped artifacts, receipts, validator outputs, eval reports, proofpacks, or gate decisions;
- executor claims such as "tests passed" or "done" are verified, captured as unverified, or rejected/deferred;
- no executor can write final decisions directly.

Catches:

- coding-agent output treated as accepted work;
- receipts/evals/proofs collapsed into decision authority;
- self-reported test success replacing validator output.

### semantic_assessors_assess_not_decide

Purpose: preserve the assessment-vs-decision boundary for semantic or LLM-based review.

Expected result:

- semantic assessors produce clause-specific advisory evidence;
- semantic assessor output cannot be the final gate decision;
- executor self-review cannot become acceptance;
- any future same-executor or same-model review risk is recorded as advisory-only unless a later accepted policy says otherwise.

Catches:

- LLM judge output treated as final acceptance;
- same-session self-review replacing gate;
- semantic review hidden inside executor claims.

### executor_failure_promotion_order

Purpose: prevent global instruction bloat after local failures.

Expected result:

- repeated executor failures are first captured as contract clauses, deterministic validators, receipt/evidence fields, eval/regression cases, proof requirements, project-memory links, or scoped runner aids;
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
- semantic assessor command interface;
- contract schema fields such as `executor_policy`, `validation_runtime`, `semantic_assessment`, or `gate_policy`;
- a public CLI command.

## Rollout

1. Use this spec as manual review criteria in the current docs-only patch.
2. Add deterministic docs/policy checks later if repeated drift appears.
3. Add contract/receipt schema checks only when the roadmap promotes the relevant runtime surface.
4. Add semantic assessor interface checks only after a separate Research Gate and accepted boundary artifact.
