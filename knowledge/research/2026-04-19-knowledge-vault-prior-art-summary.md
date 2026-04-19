---
id: research_2026_04_19_knowledge_vault_prior_art
title: "Knowledge Vault prior art summary for punk"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-19
review_after: 2026-07-19
components: [project-memory, knowledge-vault, research]
related_goals:
  - goal_research_knowledge_vault
related_files:
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-GATE.md
  - docs/adr/ADR-0008-knowledge-vault-boundaries.md
source_refs:
  - README.md
  - docs/product/PUNK-LAWS.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/RESEARCH-GATE.md
  - https://github.com/mozilla-ai/cq
  - https://docs.sigstore.dev/logging/overview/
  - https://slsa.dev/spec/
  - https://owasp.org/www-project-top-10-for-large-language-model-applications/
confidence: medium
research_level: R2
---

# Knowledge Vault prior art summary for punk

## Question

What should `Knowledge Vault` mean for `punk`, and which prior-art patterns should be adopted, deferred, parked, or avoided?

## Decision context

This affects the Project Memory Plane, future retrieval behavior, knowledge promotion rules, eval policy, and adapter/module boundaries.

`punk` is currently core-first: flow, event log, eval, contract lifecycle, `gate`, proof, and inspectable state are active before LLM drafting, MCP, embeddings, or plugin-marketplace behavior.

## Sources reviewed

| Source | Tier | Why included | Key limitation |
|---|---|---|---|
| `README.md`, `docs/product/PUNK-LAWS.md`, `PROJECT-MEMORY.md`, `ARCHITECTURE.md`, `ROADMAP.md`, `RESEARCH-GATE.md` | A | Current project constraints and source of truth for active-core boundaries | Internal intent, not external validation |
| `mozilla-ai/cq` repo and docs | A | Closest adjacent system for structured shared agent knowledge | Exploratory 0.x system, not a proven fit for `punk` |
| Sigstore/Rekor docs | A | Provenance and transparency-log pattern for proof-linked promotion | Supply-chain pattern, not agent memory directly |
| SLSA spec | A | Provenance framing and attestation vocabulary | Broad supply-chain framing, not retrieval design |
| OWASP LLM Top 10 | A | Security failure modes for memory/retrieval systems | Threat catalog, not architecture blueprint |

## Existing systems / prior art

### `punk` docs already define the safe boundary

Current docs already establish the most important constraints:

- project memory is explicit and repo-tracked,
- only `gate` writes final decisions,
- modules assess and adapters invoke, but neither owns truth,
- speculative knowledge is excluded from implementation truth by default,
- important architecture changes require research before implementation.

### cq offers useful lifecycle ideas, not a wholesale architecture

Useful cq patterns:

- structured knowledge units
- propose / confirm / flag lifecycle
- explicit staleness and supersession
- human review before promotion

Patterns to defer or avoid for active-core:

- local MCP server as a first-class surface
- remote/team/global commons
- memory becoming a runtime decision surface

### Provenance systems matter as much as memory systems

Sigstore/Rekor and SLSA are strong prior art for future proof-linked knowledge promotion:

- transparency logs
- attestations
- provenance separate from trust

These patterns fit `punk` better than hidden auto-promoted memory because they strengthen inspectability instead of weakening it.

## Failure modes found

- hidden or derived indexes silently become the real source of truth
- stale or superseded knowledge keeps influencing implementation
- contradictory knowledge is flattened instead of surfaced
- prompt-injected or secret-bearing artifacts leak into retrieval
- confidence or reputation scores are mistaken for authority
- remote/shared memory expands scope before local project memory is stable

## Options considered

### Option A — Repo-tracked artifacts first, derived indexes later

Pros:

- aligns with current `punk` laws and docs
- keeps truth inspectable in Git
- supports deterministic, rebuildable derived indexes later

Cons:

- slower than adopting an off-the-shelf memory layer
- requires explicit metadata and evals before retrieval becomes useful

### Option B — Derived local index as early active surface

Pros:

- faster search and retrieval experiments
- lower manual lookup cost

Cons:

- risks turning `.punk/` into hidden authority
- adds schema and rebuild correctness pressure too early

### Option C — Shared or MCP-backed memory

Pros:

- broader recall and cross-agent sharing
- rich future ecosystem potential

Cons:

- conflicts with current active-core scope
- expands security and governance risk immediately
- weak fit before local repo memory is stable

## Recommendation

Adopt now as a documentation boundary:

- Knowledge Vault = repo-tracked, authority-tagged artifacts plus reconstructable derived indexes.
- Retrieval stays advisory-only.
- Speculative knowledge stays excluded by default.
- Promotion remains proof-linked and review-bound.

Do not adopt now:

- embeddings as source of truth
- MCP memory
- remote/shared commons
- autonomous capture or promotion

## What stays out of scope

- vector DB as canonical memory
- remote/team/global memory stores
- autonomous promotion of agent-learned knowledge
- reputation or confidence as a review bypass
- network side effects in active-core retrieval

## Impact on roadmap

- Phase 4 should define retrieval-relevant metadata and at least one knowledge eval or lint before retrieval promotion.
- Phase 5 may later use Knowledge Vault artifacts as advisory drift signals.
- No roadmap phase should treat retrieval as a replacement for `gate` or proof.

## Required evals

- authority filtering
- stale/superseded handling
- contradiction surfacing
- proof-link integrity
- no hidden source of truth after derived index rebuild
- prompt-injection quarantine
- PII/secret leakage prevention
- local-first/no-network behavior
- `gate` boundary enforcement

## Required docs / ADRs / contracts

- `docs/adr/ADR-0008-knowledge-vault-boundaries.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`
- future bounded contract for any retrieval experiment

## Adoption map

- Adopt: repo-tracked truth, authority tags, supersession awareness, advisory retrieval, proof-linked promotion
- Defer: derived local indexes, deterministic retrieval fixtures, knowledge lint
- Park: tool-gap signals, temporal graph ideas, model-family diversity confirmation
- Avoid for now: vector DB truth, MCP-first memory, shared commons, autonomous promotion

## Open questions

- Should retrieval-relevant metadata extend the current knowledge schema now or only when a first retrieval experiment exists?
- Should ideas stay under `knowledge/ideas/` or move if the project adopts a different backlog convention later?
- Is the first retrieval experiment even necessary before schema, validation, and manual review rules are stable?
