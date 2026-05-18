---
id: docs_product_runner_aids
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-18
updated_at: 2026-05-18
review_after: 2026-07-18
canonical_for:
  - runner-aid-boundary
  - task-scoped-advisory-handoff-boundary
  - persistent-model-control-artifact-boundary
  - executor-aid-non-authority-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/RESEARCH-INTAKE.md
related_research:
  - knowledge/research/2026-05-18-gstack-workflow-prior-art.md
related_evals:
  - evals/specs/runner-aid-boundary.v0.1.md
supersedes: []
superseded_by: null
---

# Runner Aids

## Purpose

Define how Punk treats prompts, skills, playbooks, checklists, slash-command
recipes, model settings, provider settings, task briefs, and other executor aids.

Runner aids may help a human, script, model, coding agent, IDE, module, or
adapter attempt bounded work.

Runner aids do not own project truth.

## Principle

A runner aid can help execution.

It cannot decide acceptance.

```text
runner aid -> executor attempt -> receipt/evidence -> eval/assessment -> gate -> proof
```

Only `gate` writes final decisions. Proof comes before acceptance.

## Definition

A runner aid is any task-scoped or persistent artifact that helps an executor
perform or assess work without becoming the authority for the work.

Examples:

- task brief;
- implementation checklist;
- review checklist;
- slash-command guide;
- `AGENTS.md` / `CLAUDE.md`-style instruction;
- model-specific skill;
- provider setup guide;
- local playbook;
- generated instruction view;
- prompt template;
- manual QA script;
- pair-programming note;
- temporary safety rule.

## Authority

Runner aids are advisory by default.

They may reference contracts, context packs, docs, ADRs, evals, reports, and
proof requirements, but they cannot override them.

| Surface | Authority |
|---|---|
| Contract | Normative task scope and acceptance criteria. |
| Runner aid | Advisory executor support. |
| Receipt | Evidence of what was attempted or observed. |
| Assessment | Advisory analysis of evidence. |
| Gate decision | Final decision, written only by `gate`. |
| Proofpack | Inspectable provenance for the gate decision. |
| Project memory | Authority-tagged linked artifacts, not hidden prompts. |

## Allowed

A runner aid may:

- restate current contract scope;
- list non-goals and denied paths;
- remind an executor to ask clarifying questions;
- suggest a safe order of operations;
- provide review or QA checklists;
- explain how to collect receipts;
- link to relevant docs, ADRs, evals, and context packs;
- describe known executor failure modes;
- help produce an advisory assessment;
- be generated from canonical sources if the output is rebuildable and marked
  non-authoritative.

## Not allowed

A runner aid must not:

- define acceptance criteria not present in the contract;
- approve work;
- reject work as a final decision;
- claim proof;
- write or mutate gate decisions;
- bypass `plot / cut / gate`;
- weaken core laws, project rules, module rules, profile rules, or run contract;
- turn hidden executor memory into project truth;
- treat provider settings as authority;
- make network, browser, deploy, publishing, or credential side effects active;
- mutate event logs, contracts, receipts, decisions, or proofpacks unless a
  later promoted writer explicitly owns that behavior;
- become the only source of a requirement, invariant, or project decision.

## Task-scoped aids

A task-scoped runner aid should be derived from a specific contract or current
contract draft.

It should record:

```yaml
runner_aid:
  id: runner_aid_example
  kind: task_brief
  contract_ref: contract_example
  source_refs:
    - docs/product/PUNK-LAWS.md
  status: advisory
  authority: non_authoritative
  generated_from_refs: []
  review_after: 2026-06-18
```

A task-scoped aid may be useful during `cut`, but `gate` verifies receipts,
evals, assessment refs, and proof requirements rather than trusting the aid.

## Persistent aids

Persistent runner aids such as global prompts, skills, playbooks, provider
settings, `AGENTS.md` files, or `CLAUDE.md` files require stronger discipline.

A persistent aid should have:

- explicit scope;
- owner;
- created/review dates;
- source refs;
- failure/eval evidence or temporary-workaround rationale;
- retirement path;
- non-authority statement;
- no hidden acceptance logic.

Preferred fix order for repeated executor failures:

1. contract clause;
2. deterministic validator;
3. receipt or evidence field;
4. eval/regression case;
5. proof requirement;
6. project-memory link;
7. scoped runner aid;
8. global instruction only as last resort.

## Relationship to Contract Context Packs

A Contract Context Pack may select evidence for a contract.

A runner aid may be derived from that pack, but it is not the pack, not a
prompt-memory store, not project truth, and not a fourth lifecycle phase.

The aid should cite the contract and context-pack refs it depends on. It should
not copy large canonical bodies into a hidden prompt surface.

## Relationship to modules and adapters

A module may emit a runner aid as advisory output.

An adapter may invoke a provider or tool that consumes a runner aid.

Neither can make the runner aid authoritative. Modules assess. Adapters invoke.
Only `gate` decides.

## gstack intake boundary

`garrytan/gstack` demonstrates useful runner-aid mechanics: staged workflow
skills, review roles, safety guardrails, generated skill docs, browser QA, and
memory tools.

For Punk, those mechanics are allowed only after remapping:

```text
gstack skill -> Punk runner aid or assessment producer -> receipt/evidence -> gate/proof
```

This doc does not activate any gstack command, Claude Code behavior, browser
runtime, provider adapter, deployment behavior, or cross-agent coordination.

## Non-goals

This document does not define:

- a public `punk runner-aid` CLI;
- executor brief generation;
- provider-specific prompt formats;
- skill installation;
- browser automation;
- runtime `.punk/` storage;
- module invocation;
- adapter invocation;
- gate decision writing;
- proofpack writing;
- acceptance claims.
