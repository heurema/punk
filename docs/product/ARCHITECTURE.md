---
id: docs_product_architecture
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-24
review_after: 2026-07-20
canonical_for:
  - architecture-boundaries
  - core-ownership
  - lifecycle-shape
  - local-trust-evidence-architecture
  - project-memory-plane-boundary
  - module-host-boundary
  - adapter-boundary
  - assessment-vs-decision-boundary
  - proofpack-provenance-boundary
  - eval-plane-boundary
  - execution-agnostic-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/MODULE-HOST.md
related_adrs:
  - docs/adr/ADR-0003-project-memory-plane.md
  - docs/adr/ADR-0012-minimal-proofpack-provenance.md
  - docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md
  - docs/adr/ADR-0014-execution-agnostic-contract-boundary.md
supersedes: []
superseded_by: null
---
# Architecture

## Target shape

```text
punk
├─ core laws
├─ flow controller
├─ eval plane
├─ project memory plane
├─ module host
├─ gate/proof
└─ adapters
```

## Core ownership

The core owns:

- project identity
- flow state
- rule snapshots
- contracts
- scope
- runs
- receipts
- decisions
- proofpacks
- event log
- inspectable views
- active-core eval definitions and deterministic reports

The core must not own:

- provider-specific orchestration
- module-specific business logic
- marketplace behavior
- hidden memory
- unbounded autonomy
- hosted eval or tracing truth

## Execution-agnostic boundary

Punk does not own execution.

An executor is any human, local model, coding agent, IDE, shell script, module, or adapter that attempts scoped work under a contract.

The executor is replaceable and non-authoritative. Its prompts, skills, model settings, provider defaults, local memories, tool rituals, and hidden runtime state are not project truth.

The core owns:

- the contract
- allowed scope
- expected artifacts
- run receipt shape
- validator results
- eval reports
- gate decision
- proofpack
- event log
- project-memory links

The core does not own:

- provider-specific orchestration
- global model prompts
- model-specific rituals
- hidden local memories
- autonomous execution policy
- user runtime configuration

Punk may describe the work. Punk may verify the result. Punk must not silently depend on how the executor achieved it.

Execution details become Punk evidence only when captured in a scoped run receipt, validator output, eval report, proofpack, or gate decision.

In short: Punk owns contract, evidence, validation, gate, proof, and memory. Punk does not own execution.

## Universal lifecycle

All modules use the same grammar:

```text
Goal -> Contract -> Run -> Receipt -> ModuleAssessment -> DecisionObject -> Proofpack
```

`ModuleAssessment` is advisory evidence.

`DecisionObject` is final and is written only by `gate`.

## Local trust evidence architecture

`punk` active-core uses local trust evidence instead of remote control-plane truth.

Canonical evidence surfaces:

- `.punk/events/*.jsonl` for append-only audit events
- `.punk/runs/` for run receipts
- `.punk/evals/` and `work/reports/` for eval outputs and linked summaries
- `.punk/decisions/` for gate decisions
- `.punk/proofs/` for proofpack manifests
- `.punk/views/` for derived, regenerable inspect views

Events may reference artifacts by id, relative path, and hash.

Events must not duplicate raw contract bodies, prompts, source snippets, secrets, environment values, provider payloads, or hidden remote-export state.

Inspectable state should be reconstructable from canonical evidence. Derived views are convenience, not truth.

## Workspace activation model

A crate or folder can exist before it is active.

Statuses:

- `active-core`
- `incubating`
- `parked`
- `retired`

The public CLI must only expose active capabilities.

## Parked adapter boundary: repo search

Repo/code retrieval may later be exposed through adapters, not the core.

A repo-search adapter may invoke local or remote retrieval tools such as text
search, fuzzy file search, symbol search, language-server navigation, or code
graph lookup. It must remain advisory evidence only.

A repo-search adapter must not:

- own project truth
- write final decisions
- bypass `gate`
- promote retrieved content into canonical knowledge
- hide non-canonical memory behind implementation truth
- expose editing or external side effects as part of read-only retrieval

If retrieval is used by a contract or module, the run should leave a retrieval
receipt that records the backend, query, constraints, result summary, timing,
and any index/cache provenance needed for inspection.

The initial parked boundary is documented in `docs/adapters/repo-search.md`.

## Project memory plane

Project memory has four repo-tracked truth surfaces:

- `work/` — goals, reports, generated views
- `knowledge/` — product truth, architecture truth, decisions, research, ideas
- `docs/adr/` — tracked architecture decisions
- `publishing/` — public-build narrative and receipts

Runtime and derived data live under `.punk/`.

### Project-memory link graph

Project memory should keep explicit links across:

```text
goal -> contract -> report -> eval -> decision -> proof -> docs/public narrative
```

The link graph is bounded project memory, not a giant prompt. It should stay repo-tracked where possible and derive inspectable views from canonical artifacts when needed.

### Knowledge Vault architecture boundary

Knowledge Vault owns the repo-tracked knowledge artifact contract under `knowledge/`.

Retrieval is an inspectable advisory view over project memory. It can provide evidence to `plot`, `cut`, and `gate`, but it cannot approve work, execute work, or write final decisions.

Future storage and retrieval must preserve these boundaries:

- repo artifacts remain source inputs,
- `.punk/` indexes and views are derived and rebuildable,
- modules may assess memory relevance,
- modules cannot decide authority,
- adapters may invoke external systems,
- adapters cannot own project truth.

This does not change the lifecycle:

```text
plot -> cut -> gate
```

## Module host boundary

The module host is an architectural boundary, not an active plugin runtime.

A module is a Punk lifecycle participant. A plugin is only a possible future packaging or execution mechanism for a module.

The module host must remain Punk-owned: it invokes modules, validates module receipts and assessments, records allowed events, and leaves final decision writing to `gate`.

A future plugin runtime must not write final decisions, mutate the event log directly, create a hidden project memory store, bypass contract scope, bypass `gate`, or gain filesystem, network, environment, secret, process, shell, or publishing authority by default.

See `docs/product/MODULE-HOST.md` and `docs/adr/ADR-0010-defer-wasm-plugin-host.md`.

## Assessment vs decision boundary

Rules, modules, policies, and adapters may assess.

They may produce guard results, warnings, findings, patches, receipts, or recommendations.

They must not write final acceptance. Only `gate` writes final decisions.

Gate decisions may reference assessments, eval reports, run receipts, and proofpacks.

## Minimal proofpack provenance

A proofpack is a local provenance manifest. It should reference:

- contract id and hash
- run receipt id and hash
- eval report id and hash, when applicable
- gate decision id and hash
- output artifact refs and hashes
- relevant event range or event root
- rule/guard version refs

Remote transparency logs, release signing, and full supply-chain provenance are parked.

## Eval plane

Eval is not a later feature. Minimal evals are part of the core.

The initial eval plane is:

- local-first
- repo-stored
- deterministic-first
- fixture-based
- hard-gate plus scorecard
- Pydantic Evals-compatible later, but not dependent on Python/web/SaaS in the core path

The eval plane assesses evidence. It does not accept work. Only `gate` writes final decisions.

Phase 2 active-core evals must check:

- state transition validity
- append-only event writing
- `cut` scope and approval guards
- run receipt existence
- gate decision exclusivity
- proofpack artifact links and hashes
- report schema shape
- baseline regressions
- waiver ledgering
- parked capability isolation

See `docs/product/EVAL-PLANE.md` and `docs/adr/ADR-0008-eval-plane.md`.

## Flow controller

Commands are transitions over persisted state.

An LLM or other executor can suggest a next step, but runtime flow state decides whether the step is allowed.

## Research gate

Research is a governance layer before important architecture changes. It is not an execution mode and does not add a fourth runtime phase.

The lifecycle stays:

```text
plot -> cut -> gate
```

Research informs `plot` by providing prior-art evidence, failure modes, and recommended constraints.

Important contracts should cite research refs when the work changes core behavior, storage, module interfaces, flow, eval, adapters, or project memory.
