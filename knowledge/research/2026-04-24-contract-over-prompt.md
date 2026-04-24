---
kind: research-note
status: active
authority: advisory
created: 2026-04-24
related_goal: work/goals/goal_execution_agnostic_contract_boundary.md
related_report: work/reports/2026-04-24-execution-agnostic-contract-boundary.md
---

# Contract over Prompt / Execution-agnostic Contract Boundary

## Question

Should Punk adopt `Contract over Prompt` now, and if yes, where should it live without turning Punk into an agent runtime, provider integration layer, or prompt manager?

## Decision context

Punk already defines the active trust surfaces as contract, scope, event evidence, run receipts, eval reports, gate decisions, proofpacks, and project-memory links.

The next risk is that persistent prompts, repo instructions, local skills, model defaults, and provider-specific rituals could become the real architecture outside Punk's proof-bearing artifact loop.

The proposed boundary is:

```text
Punk does not own execution.
Punk owns contract, evidence, validation, gate, proof, and memory.
```

This is an R2 architecture/policy question because it touches core laws, eval policy, project memory, roadmap phase boundaries, adapter boundaries, and repository instructions.

## Sources reviewed

| Source | Type | Quality | Date checked | URL / ref | Why relevant |
|---|---|---|---|---|---|
| `README.md` | repo entry doc | project canonical/public entry | 2026-04-24 | `README.md` | Defines Punk as early-stage, local-first, not production-ready, with coding-agent execution and provider adapters not active. |
| `docs/product/START-HERE.md` | repo canonical doc | project canonical | 2026-04-24 | `docs/product/START-HERE.md` | Defines active operator surfaces and `plot / cut / gate` boundaries. |
| `docs/product/PUNK-LAWS.md` | repo canonical doc | project canonical | 2026-04-24 | `docs/product/PUNK-LAWS.md` | Defines contracts, receipts, gate exclusivity, proof, adapters, and project memory laws. |
| `docs/product/ARCHITECTURE.md` | repo canonical doc | project canonical | 2026-04-24 | `docs/product/ARCHITECTURE.md` | Defines core ownership and non-ownership of provider orchestration, hidden memory, and unbounded autonomy. |
| `docs/product/ROADMAP.md` | repo canonical doc | project canonical | 2026-04-24 | `docs/product/ROADMAP.md` | Phase 3 is explicitly contract loop without agents; adapters and agent execution remain later/parked. |
| `docs/product/PROJECT-MEMORY.md` | repo canonical doc | project canonical | 2026-04-24 | `docs/product/PROJECT-MEMORY.md` | Project memory graph must be built from canonical artifacts, not a giant prompt or hidden runtime state. |
| `docs/product/RESEARCH-GATE.md` | repo canonical doc | project canonical | 2026-04-24 | `docs/product/RESEARCH-GATE.md` | Defines Research Gate, promotion path, and eval implications for architecture decisions. |
| Anthropic Claude Code quality postmortem | engineering postmortem | B | 2026-04-24 | https://www.anthropic.com/engineering/april-23-postmortem | User research packet reports quality regressions from product-layer prompt/default/cache changes, supporting eval-backed policy over prompt trust. |
| OpenAI evals guide | official docs | A | 2026-04-24 | https://platform.openai.com/docs/guides/evals | Evals check outputs against criteria, especially across model changes, supporting validation over over-instruction. |
| OpenAI structured outputs guide | official docs | A | 2026-04-24 | https://platform.openai.com/docs/guides/structured-outputs | Schema adherence is a stronger control surface than asking a model to return valid structure. |
| Anthropic Agent Skills | vendor engineering doc | A/B | 2026-04-24 | https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills | Skills are useful procedural context, but should remain composable runner aids rather than project truth. |

## Source quality table

| Source class | Reliability | What Punk can safely use | Cautions |
|---|---|---|---|
| Repo canonical docs | High for Punk intent | Current active/parked boundaries and truth ownership | They need explicit promotion to cover prompt/skill runtime boundaries. |
| Official OpenAI docs | High for eval/schema principles | Validation and schema control are safer than instruction-only control | API mechanisms do not imply Punk should integrate providers now. |
| Anthropic postmortem | Medium-high field evidence | Product-layer prompt/default changes can measurably affect coding quality | Single-vendor incident; use as failure mode evidence, not direct architecture copy. |
| Anthropic Skills doc | Medium-high vendor evidence | Skills can be scoped procedural aids | Vendor framing is product-specific; skills should not become Punk authority by default. |

## Existing systems / prior art

### Evals as external verification

OpenAI's eval guidance frames evals as checks against expected criteria, especially when changing or upgrading models. Punk should map this to deterministic-first validation and eval reports, not to model-specific prompt rituals.

### Structured outputs as schema over instruction

Structured Outputs show the same principle at a lower layer: define a schema and validate adherence instead of relying on prose prompts. Punk should prefer contract clauses, validators, eval cases, receipt shapes, and proof requirements over global instructions.

### Product-layer prompt/default failures

The Anthropic postmortem described quality issues from product-layer defaults and prompt/cache behavior. The lesson for Punk is not to own a model runner now; it is to avoid making hidden product/runtime settings the trust root.

### Skills as scoped runner aids

Agent skills can improve executor performance when curated and scoped. Punk should allow them as advisory runner aids, but require explicit scope, failure/eval evidence, owner, and retirement path before any persistent model-control artifact becomes durable policy.

## Failure modes found

1. **Prompt accretion becomes architecture**
   - Global instructions silently replace contract clauses, validators, and eval cases.

2. **Skills become hidden product truth**
   - `.agents` or similar runner aids define behavior that canonical docs, ADRs, and proof artifacts cannot inspect.

3. **Provider defaults become acceptance criteria**
   - A model setting or provider-specific scaffold changes results without a Punk artifact recording the contract/evidence reason.

4. **Agent execution is implied too early**
   - Docs make Punk look like it executes coding-agent work today, conflicting with Phase 3 and current non-active warnings.

5. **Executor output is treated as accepted work**
   - A coding agent, script, or human output bypasses receipt/eval/gate/proof.

6. **Repeated failures become global rituals**
   - A local executor failure creates permanent instruction bloat instead of a contract clause, validator, eval case, proof requirement, or memory update.

7. **Project memory becomes accumulated prompt text**
   - Hidden or local memories replace explicit link graphs across goals, contracts, reports, evals, decisions, proofs, and public narrative.

## Options considered

### Option A — Do nothing

Pros:
- no diff;
- no new policy surface.

Cons:
- prompt/skill accretion can continue unchecked;
- AGENTS/skills may look more authoritative than intended;
- future provider work may enter with unclear truth ownership.

Assessment: avoid.

### Option B — Build a model runner/provider abstraction now

Pros:
- direct control over executor inputs and outputs.

Cons:
- violates current phase boundaries;
- introduces provider-specific behavior too early;
- competes with contract/evidence/gate/proof work.

Assessment: park.

### Option C — Ban prompts and skills

Pros:
- simple rule.

Cons:
- unrealistic for local operators;
- blocks useful scoped runner aids;
- confuses operational help with project truth.

Assessment: avoid.

### Option D — Adopt execution-agnostic boundary as docs/ADR/eval-policy now

Pros:
- strengthens active-core trust surfaces;
- preserves Phase 3 contract loop without agents;
- lets any executor attempt work while Punk verifies artifacts;
- keeps prompts/skills useful but non-authoritative.

Cons:
- requires better contracts, validators, receipts, eval specs, and proof requirements;
- external execution quality may vary;
- persistent runner aids need lifecycle discipline.

Assessment: adopt now.

## Recommendation

Adopt now as an architecture and Research Gate policy boundary:

```text
Contract over Prompt.
Validate, don't over-instruct.
Execution is replaceable and non-authoritative.
```

Do not implement model execution, provider adapters, prompt management, skill management, MCP integration, or new CLI behavior in this step.

## Adoption map

| Outcome | Items |
|---|---|
| adopt | Execution-agnostic boundary in core laws, architecture, roadmap, Research Gate, Project Memory, ADR, eval spec, and advisory runner-aid headers. |
| defer | Schema-level `executor_kind`, receipt metadata fields, validator implementation, and gate/proof runtime consequences. |
| park | `punk agent`, `punk provider`, `punk prompt`, `punk skill`, provider abstraction, MCP integration, coding-agent execution. |
| avoid | Treating AGENTS.md, skills, prompts, local memories, or provider defaults as project truth. |

## What stays out of scope

- model runner;
- provider adapters;
- autonomous coding execution;
- plugin runtime;
- prompt/skill product surface;
- public CLI changes;
- Rust code changes;
- `.punk/` runtime state writes.

## Impact on roadmap

- Phase 3 should explicitly allow manual/BYO execution while making receipt/evidence capture and gate verification the active-core requirement.
- Phase 7 Dev module should preserve bring-your-own runtime and treat coding-agent output as evidence only after receipt/eval/gate/proof.
- Phase 9 Adapters should wrap provider/tool capabilities without letting provider behavior, prompt scaffolds, or model defaults own truth.

## Required evals

Add an eval spec that checks docs/policy consistency for:

- no current claim that Punk executes agent work today;
- prompts/skills/AGENTS as advisory only;
- provider/model settings non-authoritative;
- executor output treated as evidence, not acceptance;
- only `gate` writes final decisions;
- repeated executor failures routed to contract/validator/eval/proof/memory before global instruction.

## Required docs/ADRs/contracts

- `docs/adr/ADR-0014-execution-agnostic-contract-boundary.md`
- `evals/specs/execution-agnostic-boundary.v0.1.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/PUBLIC-NARRATIVE.md`
- `docs/product/CRATE-STATUS.md`
- `AGENTS.md`
- `.agents/skills/punk-workflow/SKILL.md`

## Open questions

1. When contract schema changes resume, should `executor_kind: manual|external|module|adapter` be added as metadata or left to run receipts?
2. Should runner-aid lifecycle metadata live in each aid file or in a central `knowledge/ops/runner-aids.md` index?
3. Which deterministic check should first enforce this boundary: docs-governance extension, smoke eval case, or standalone policy lint?
