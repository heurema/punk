---
kind: research-note
status: active
authority: advisory
created: 2026-04-25
related_goal: work/goals/goal_adopt_executor_discipline_aids.md
related_report: work/reports/2026-04-25-executor-discipline-aids.md
---

# Karpathy-style Executor Discipline Aids Review

## Question

Should Punk adopt ideas from `forrestchang/andrej-karpathy-skills`, and if yes, where should they live without becoming a canonical skill, prompt system, runtime feature, provider integration, schema change, or acceptance authority?

## Short answer

Recommendation: **ADOPT IDEAS, AVOID AUTHORITY MODEL**.

Use the prior art to improve executor attempts through advisory docs, examples, and eval-policy artifacts. Do not install, vendor, copy, or promote the external skill or any `CLAUDE.md`/Cursor/plugin packaging as Punk project truth.

## Source-quality table

| Source | Tier | Why used | Punk takeaway |
|---|---|---|---|
| `forrestchang/andrej-karpathy-skills` README and instruction surfaces | B | Direct prior-art artifact for the requested review. | Four useful executor-discipline principles: surface assumptions, prefer simplicity, make surgical changes, and define verifiable goals. |
| Claude Code Skills docs | A | Official documentation for Claude Code skill mechanics. | Skills are optional prompt/tooling surfaces loaded when relevant; this is useful packaging prior art, not Punk authority. |
| Anthropic engineering post: Agent Skills | A/B | Vendor engineering writeup on progressive disclosure, eval-first skill creation, and security review. | Executor aids should be scoped, evaluated against observed gaps, progressively disclosed, and audited before reuse. |
| Punk `START-HERE`, `ARCHITECTURE`, `ROADMAP`, `RESEARCH-GATE` | Canonical internal | Source of truth for Punk active-core and promotion boundaries. | No runtime, provider, agent execution, prompt manager, or skill promotion enters active core through this patch. |

Checked on: 2026-04-25.

Source refs:

- `https://github.com/forrestchang/andrej-karpathy-skills`
- `https://docs.anthropic.com/en/docs/claude-code/skills`
- `https://www.anthropic.com/engineering/equipping-agents-for-the-real-world-with-agent-skills`
- `docs/product/START-HERE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`

## What the prior art is

`andrej-karpathy-skills` is a compact prompt/skill-layer package. It presents one `CLAUDE.md`-style instruction file, skill/package surfaces, Cursor rules, and examples. Its content is meant to steer a coding executor toward better behavior.

Its useful core is four executor-discipline principles:

1. **Think Before Coding** - surface assumptions, ambiguity, tradeoffs, and confusion before changing files.
2. **Simplicity First** - prefer the minimum scoped solution and avoid speculative flexibility.
3. **Surgical Changes** - touch only what the task requires and avoid drive-by refactors or unrelated formatting.
4. **Goal-Driven Execution** - convert vague work into verifiable success criteria and checks.

## Why it is useful

These principles target failure modes Punk already cares about:

- silent executor assumptions;
- overengineered or speculative changes;
- unrelated edits hidden inside a task diff;
- self-reported success without verifiable evidence;
- vague goals that cannot be gated.

They make executor attempts better before validation begins. Better attempts reduce review cost, but they do not change Punk's trust model.

## Why Punk should not adopt it as a skill or prompt authority

The prior art is packaged as a runner aid. That is exactly the boundary Punk must preserve.

Punk active core is contract, validation protocol, evidence, gate, proof, and project memory. Executor prompts, skills, local settings, model defaults, hidden memories, and self-reported success are not project truth.

A persistent skill or `CLAUDE.md`-style file can help a human, model, agent, script, IDE, or other executor attempt work. It must not:

- define acceptance criteria;
- override a contract;
- weaken validators;
- write gate decisions;
- replace proofpacks;
- become project memory without explicit refs and review;
- promote a provider, model, or executor into active-core architecture.

## Mapping to Punk concepts

| Karpathy-style principle | Punk-native mapping |
|---|---|
| Think Before Coding | `plot` surfaces assumptions, ambiguity, tradeoffs, and confusion before contract approval. |
| Simplicity First | Active-core discipline, no speculative scope, minimal bounded change, no future capability exposure. |
| Surgical Changes | Contract allowed paths, protected paths, receipt/evidence fields, diff checks, and future eval-policy cases. |
| Goal-Driven Execution | Acceptance criteria, validators, proof requirements, gate decisions, and proofpack preservation. |

## Adoption classification

| Classification | Items |
|---|---|
| adopt | Assumptions before edits, simplicity, surgical scope, task-to-contract examples, verifiable goal framing, and eval-policy examples. |
| defer | Generated task-scoped executor briefs from `plot`; any formal receipt field for an executor brief hash. |
| park | Skill packaging/export, prompt bundle management, provider-specific runner integration, and executor-specific adapters. |
| avoid | Treating `CLAUDE.md`, AGENTS files, skills, prompts, local memories, or executor claims as project truth or acceptance authority. |

## Decision

Adopt the ideas as Punk-native advisory artifacts:

- executor brief template;
- task-to-contract transformation examples;
- bad/good workflow examples;
- surgical-change eval-policy spec;
- small architecture, roadmap, Research Gate, and dogfooding notes.

Do not adopt the external repository as a dependency. Do not install or vendor its skill. Do not create a canonical Punk skill, prompt system, runtime feature, provider integration, schema field, or CLI command.
