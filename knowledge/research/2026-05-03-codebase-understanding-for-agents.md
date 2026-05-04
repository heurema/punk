---
id: research_2026_05_03_codebase_understanding_for_agents
title: "Codebase Evidence Boundary for Agents"
kind: research
status: draft
authority: advisory
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
review_after: 2026-07-20
components:
  - project-memory
  - repo-search
  - codebase-evidence
  - contract-context-pack
  - adapters
  - eval
related_goals:
  - work/goals/goal_capture_codebase_evidence_boundary_for_agents_v0_1.md
related_files:
  - docs/adapters/repo-search.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/BROWNFIELD-INVENTORY.md
  - knowledge/research/2026-04-19-repo-search-adapter-options.md
  - knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md
  - evals/specs/repo-search-adapter-boundary.v0.1.md
  - evals/specs/codebase-context-pack-boundary.v0.1.md
source_refs:
  - README.md
  - docs/product/START-HERE.md
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/adapters/repo-search.md
  - knowledge/research/2026-04-19-repo-search-adapter-options.md
  - knowledge/research/2026-04-29-project-knowledge-vault-for-agents.md
confidence: medium
research_level: R2
---

# Codebase Evidence Boundary for Agents

## Question

How should Punk help agents use repository evidence without turning retrieval,
indexes, summaries, or executor memory into project truth?

## Decision context

Punk is an early-stage, local-first bounded work kernel. The current CLI
surface is intentionally small. Adapters, MCP integration, provider execution,
Dev module behavior, repo-search runtime, vector indexes, code graph runtime,
gate decision writers, broader runtime proofpack writer orchestration,
`.punk/proofs` writer behavior, and acceptance proof behavior are not active
unless a later bounded goal promotes them.

The active lifecycle remains:

```text
plot -> cut -> gate
```

Only `gate` writes final decisions. Modules may assess. Adapters may invoke.
Project memory is explicit and authority-tagged.

This note records R2 advisory research and eval/spec direction only. It does
not implement runtime behavior.

## Facts

- `docs/adapters/repo-search.md` already defines a parked, advisory,
  documentation-only repo-search adapter boundary.
- `docs/product/ARCHITECTURE.md` already states that retrieval receipts may be
  linked as advisory evidence and cannot own truth or write decisions.
- `docs/product/PROJECT-MEMORY.md` distinguishes project memory from retrieval
  memory. Derived indexes and views are rebuildable and non-authoritative.
- `docs/product/BROWNFIELD-INVENTORY.md` defines observable structure as
  advisory `observed_structure`, not claims, contracts, decisions, or proof.
- `evals/specs/context-pack-boundary.v0.1.md` already protects Contract Context
  Packs from orphan context, stale flattening, speculative source leakage, and
  retrieval authority creep.
- Current active CLI behavior does not include repo scanning, file walking,
  code graph construction, vector DB, MCP tools, Dev module runtime, agent
  execution, contract generation from code, or codebase reconstruction.

## Assumptions

- Agents will need bounded help finding repository evidence before they can
  safely attempt software work under future contracts.
- The safest first retrieval baseline is live local lexical search because it
  is easy to inspect, reproduce, budget, and receipt.
- Symbol, AST, graph, vector, memory, and remote search layers may be useful
  later, but they add freshness, authority, privacy, and ranking risks.
- A future Contract Context Pack can use repository evidence only when that
  evidence is clause-indexed, source-cited, and receipt-bearing.
- External source clusters in this note are used as directional evidence unless
  they are already repo-tracked or official project documentation.

## Sources reviewed

- Punk canonical docs: `README.md`, `START-HERE`, `PUNK-LAWS`,
  `ARCHITECTURE`, `ROADMAP`, `CRATE-STATUS`, `RESEARCH-GATE`,
  `PROJECT-MEMORY`, `BROWNFIELD-INVENTORY`.
- Existing Punk research: repo-search adapter options and Project Knowledge
  Vault for Agents.
- Existing eval specs: Context Pack boundary, Brownfield Inventory boundary,
  Brownfield Source Corpus Manifest boundary.
- External source clusters: RepoBench, SWE-bench, SWE-bench Lite,
  SWE-bench Verified, SWE-rebench, Multi-SWE-bench, Repository-Level Prompt
  Generation for LLMs of Code, REPOFUSE, Retrieval-Augmented Code Generation
  survey, SWE-agent, AutoCodeRover, LingmaAgent, RepoGraph, AgentFL, Serena,
  SCIP / LSIF / Sourcegraph / Zoekt, ripgrep / fd / tree-sitter,
  Lost in the Middle, Context Rot, Mem0, A-MEM, Codebase-Memory,
  MCP specification / MCP security guidance.

Exact external source details were not revalidated in this patch unless already
present in repo-tracked research notes. This source set is used as directional
evidence only.

## Source quality table

| Source set | Tier | Use in this decision | Key limitation |
|---|---:|---|---|
| Punk canonical docs | A | Define active vs parked scope, authority model, project memory, adapter laws, gate/proof boundary, and no-network posture | Internal project truth, not external validation |
| Existing Punk repo-search and Knowledge Vault research | A/B | Provides already accepted advisory direction for retrieval receipts, non-canonical retrieval memory, and derived views | Draft/advisory until promoted |
| Existing context-pack and brownfield eval specs | A | Provide local boundary patterns for clause mapping, observable structure, no-content defaults, and advisory evidence | They define future checks, not active runtime |
| RepoBench and SWE-bench family | A/B | Shows repository-level software tasks need multi-file evidence and benchmark discipline | Benchmark tasks can shape architecture too narrowly |
| Repository-level prompt and retrieval-augmented code generation research | A/B | Supports selective context over raw full-repo stuffing | Often optimizes answer quality, not authority boundaries |
| Agent systems such as SWE-agent, AutoCodeRover, LingmaAgent, AgentFL | A/B | Show tool-mediated navigation, tests, and fault localization can help agents | They may include execution policies outside Punk active-core |
| RepoGraph, Codebase-Memory, graph and tree-sitter systems | A/B | Support structural code navigation and graph-assisted context selection | Derived structure can become stale or over-authoritative |
| Serena, LSP, SCIP / LSIF / Sourcegraph / Zoekt | A/B | Useful for symbol, definition, reference, and indexed search boundary design | Index/cache freshness and remote side effects vary |
| ripgrep, fd, tree-sitter | A/B | Practical local-first baseline and future fixture target | Lexical/structural coverage is limited by query and language support |
| Lost in the Middle and Context Rot | A/B | Supports avoiding indiscriminate long-context repository maps | Does not by itself define a Punk artifact model |
| Mem0, A-MEM, persistent memory systems | B/C | Useful failure-mode evidence for continuity and hidden memory risk | Agent memory goals differ from project memory authority |
| MCP specification and MCP security guidance | A/B | Useful for future adapter/tool authority and side-effect review | MCP runtime is not active in Punk |

## Existing systems / prior art

Repo-level benchmarks show that software tasks often depend on evidence spread
across multiple files, tests, docs, build configuration, and history. They are
useful for fixture design, but they should not drive Punk toward
leaderboard-shaped architecture.

Coding-agent systems combine repository navigation, execution, patching, tests,
and model planning. Punk should extract the evidence boundary and receipt
discipline from these systems, not import autonomous agent execution into
active-core.

Lexical search tools such as `ripgrep` and file discovery tools such as `fd`
are the safest first baseline. They are local, inspectable, cheap to run,
bounded by explicit budgets, and easy to reproduce in fixture evals.

Symbol/LSP, Tree-Sitter, and code graph systems can improve precision and
multi-file navigation. They require capability declarations, freshness
metadata, and unsupported-mode reporting before they can safely back a
repo-search adapter.

Vector and hybrid retrieval can help discovery when exact terms are unknown,
but ranking, staleness, personalization, and citation gaps make it a poor early
default for Punk authority-sensitive work.

Long-context repository maps are tempting. They can flatten source authority,
hide omissions, and dilute relevant evidence. They should remain bounded,
cited, and clause-indexed if used later.

Persistent agent memory can help runners continue work. It must not become
project memory. Any durable fact must enter Punk through source refs, reports,
research, ADRs, contracts, evals, gate decisions, proofpacks, or public memory.

Remote code search and SaaS indexing can help scale, but they remain parked
until an external side-effect policy exists.

## Mechanism taxonomy

| Family | Description | Recommendation class |
|---|---|---|
| A. Plain local lexical search | `ripgrep`, `fd`, fuzzy file search, literal and regex grep | adopt as safest future baseline / parked adapter design |
| B. Symbol/LSP retrieval | symbols, definitions, references, language-server navigation | defer as optional backend capability |
| C. Tree-Sitter AST and local structural retrieval | parser-backed structural search and local code shape | defer or incubate after lexical baseline |
| D. Full knowledge graph / repository graph | derived repository graph, symbol graph, relation graph | park as future derived advisory layer |
| E. Vector or hybrid semantic retrieval | embeddings, semantic rankers, hybrid lexical/vector search | park / avoid as early default |
| F. Long-context repository maps | generated whole-repo maps or large summaries | avoid as default; use only bounded summaries with citations later |
| G. Test/fault-localisation-assisted navigation | tests, traces, fault localization, failing-case evidence | defer to future Dev module or assessor boundary |
| H. Agent-computer interface design | tool protocols and UX for agent navigation/execution | defer to Dev module; do not import agent execution into core |
| I. Persistent agent memory systems | runner-local long-term memory and preference stores | avoid as project truth; park as possible runner aid only |
| J. Remote code search / SaaS indexing | Sourcegraph-like, Zoekt service, hosted indexes | park until external side-effect policy exists |

## Failure modes found

- benchmark-shaped architecture drift;
- leaderboard-driven design;
- long-context dilution;
- context rot;
- silent omission;
- stale index failure;
- fallback navigation misrepresented as precise navigation;
- personalized or nondeterministic ranking;
- heuristic symbol overclaiming;
- hidden derived authority;
- graph/vector memory becoming project memory;
- remote search leaking private metadata or code;
- MCP tool authority creep;
- agent execution bypassing contract, gate, and proof;
- retrieval results treated as proof;
- LLM-generated summaries treated as canonical.

## Options considered

| Option | Result | Verdict |
|---|---|---|
| Build a coding-agent understanding layer now | Imports agent execution and hidden authority before core surfaces are ready | avoid |
| Promote repo-search runtime now | Requires runtime, receipts, storage, evals, and capability manifests that are not implemented | avoid |
| Define Codebase Evidence Boundary as advisory research/eval direction | Captures the safe boundary without activating runtime | adopt |
| Start future repo-search with live lexical modes | Gives a reproducible and local-first baseline for later fixtures | adopt |
| Start with graph/vector/memory systems | Increases hidden state and freshness risk before receipts and evals exist | park |
| Use remote code search first | Useful later, but conflicts with local-first and no-network default until policy exists | park |

## Trade-off matrix

Scores are design judgments, not benchmark claims. For the first columns,
higher is better. For hidden truth risk and implementation complexity, higher
is worse.

| Family | Local-first | Advisory-only | Provenance | Reproducible | Bounded budget | Stale detectability | Token efficiency | Multi-file reasoning | Context-pack support | Roadmap fit | Hidden truth risk | Implementation complexity |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| A. Plain local lexical search | strong | strong | strong | strong | strong | strong | moderate | moderate | strong | strong | low | low |
| B. Symbol/LSP retrieval | strong | strong | strong | moderate | strong | moderate | strong | strong | strong | moderate | moderate | moderate |
| C. Tree-Sitter AST | strong | strong | strong | moderate | strong | moderate | strong | moderate | moderate | moderate | moderate | moderate |
| D. Full graph/KG | moderate | moderate | moderate | moderate | strong | moderate | strong | strong | strong | future | high | high |
| E. Vector/hybrid | moderate | moderate | moderate | moderate | strong | weak | strong | moderate | moderate | future | high | high |
| F. Long-context maps | moderate | weak | weak | moderate | weak | weak | weak | moderate | weak | weak | high | moderate |
| G. Test/fault localization | strong | strong | strong | moderate | moderate | strong | moderate | strong | strong | future | moderate | high |
| H. ACI design | variable | variable | variable | variable | moderate | variable | moderate | moderate | moderate | future | moderate | high |
| I. Persistent memory | variable | weak | weak | weak | moderate | weak | strong | moderate | weak | weak | high | moderate |
| J. Remote search | weak | moderate | moderate | moderate | strong | moderate | strong | strong | moderate | parked | high | high |

## Recommended Punk architecture boundary

The correct Punk framing is:

```text
bounded local repository evidence
  -> retrieval receipt
  -> clause-indexed Contract Context Pack
  -> advisory assessment
  -> gate/proof path later
```

Punk can help agents find bounded repository evidence. Retrieval is advisory
evidence. Retrieval outputs require receipts. Derived indexes are rebuildable
and non-authoritative. Only `gate` writes final decisions.

Hardening rule:

```text
live workspace evidence first
derived indexes second
semantic memory never truth
```

Recommended staged model:

1. Brownfield Source Corpus Manifest.
   - observes repo structure only;
   - makes no semantic claims;
   - stays local-only;
   - stores no file contents by default;
   - uses advisory `observed_structure` authority.
2. Read-only Repo Search Adapter.
   - supports `files`, `grep`, and `multi_grep` first;
   - may later support `symbols`, `definitions`, and `references`;
   - is bounded by query budget and path constraints;
   - emits retrieval receipts;
   - declares backend capabilities and limitations;
   - cannot write files, project memory, decisions, contracts, or proofpacks.
3. Contract Context Pack.
   - selects bounded evidence for a contract;
   - maps context items to contract clauses, validators, proof requirements,
     risks, non-goals, or explicit unknowns;
   - records exclusions, stale sources, contradictions, unknowns, and retrieval
     receipt refs;
   - remains advisory evidence only.
4. Future Code Graph / Knowledge Vault layer.
   - derived and rebuildable;
   - source-cited;
   - authority/status-aware;
   - useful for finding context;
   - cannot own truth.
5. Future Dev Module.
   - may use repo-search and context-pack evidence;
   - emits assessment/receipt only;
   - `gate` remains the final decision writer.

## Adoption map

| Idea / mechanism | Punk mapping | Classification | Scope label | Reason | Required evals before promotion |
|---|---|---|---|---|---|
| Codebase Evidence Boundary terminology | Research and adapter docs vocabulary | adopt | advisory | Names the boundary without claiming agent-owned knowledge | no active-surface claim scan |
| live lexical search baseline | Future repo-search `files`, `grep`, `multi_grep` baseline | adopt | parked | Local, reproducible, inspectable first path | known-file, known-literal grep, multi-grep, bounded budget |
| retrieval receipt schema | Future adapter/run/context evidence shape | adopt | advisory | Makes retrieval inspectable and non-authoritative | receipt completeness, redaction, warnings |
| bounded search budgets | Future adapter constraints | adopt | advisory | Prevents unbounded agent loops and result floods | max_results, max_elapsed_ms, max_answer_chars |
| omission/stale/truncation reporting | Future receipt warning requirements | adopt | advisory | Keeps missing evidence visible | truncation, stale index, skipped file warnings |
| clause-indexed Contract Context Pack | Contract-linked evidence selection | adopt | active-core boundary / future runtime | Already matches context-pack boundary | no orphan context, material clause coverage |
| optional LSP/symbol retrieval | Backend capability extension | defer | future adapter capability | Useful after lexical baseline | declared backend, unsupported mode, symbol lookup |
| Tree-Sitter structural retrieval | Local structural backend | defer | incubating candidate | Useful but language coverage matters | parser coverage, unsupported language, stale cache |
| test/fault-localisation navigation | Dev module assessment input | defer | future Dev module | Evidence value is high, execution boundary belongs later | execution receipt, no-decision, scope guard |
| code graph / repository graph | Derived advisory layer | park | future Knowledge Vault / adapter | Useful for context, but high freshness and authority risk | rebuildability, source citation, stale invalidation |
| vector or hybrid semantic retrieval | Candidate discovery only | park | future adapter candidate | Useful discovery, weak early authority posture | provenance, ranking disclosure, reproducibility |
| persistent agent memory | Runner aid only | avoid | runner-local / parked | High hidden authority risk | explicit scope, no project-memory write, review path |
| remote/SaaS code search | External retrieval adapter | park | future external side-effect surface | Requires side-effect, privacy, and auth policy | no-network default, explicit grant, receipt |
| MCP repo tools | Adapter candidate | park | future adapter candidate | Tool protocol is useful, authority creep risk remains | capability manifest, side-effect denial, receipt |
| autonomous code editing | Separate Dev module execution concern | avoid | out of scope | Violates current no agent execution boundary | Dev module, contract, receipt, gate/proof evals later |
| LLM-generated codebase summaries as truth | Anti-pattern | avoid | forbidden | Summaries can omit, flatten, or invent authority | canonical/advisory separation, source refs |

## What stays out of scope

This note does not add or promote:

- CLI behavior;
- runtime storage;
- `.punk/events`;
- `.punk/runs`;
- `.punk/evals`;
- `.punk/contracts`;
- `.punk/decisions`;
- `.punk/proofs`;
- repo scanning implementation;
- file walking implementation;
- MCP integration;
- provider adapters;
- agent execution;
- Dev module runtime;
- vector DB;
- embedding index;
- code graph writer;
- Knowledge Vault runtime;
- Context Compiler runtime;
- broader runtime proofpack writer orchestration;
- `.punk/proofs` writer behavior;
- acceptance proof behavior;
- gate decision writer behavior.

## Impact on Punk roadmap

This reinforces the existing Phase 9 repo-search adapter boundary and the
Phase 3/4 Contract Context Pack and Project Memory boundaries.

No active-core implementation changes are recommended now.

Before any repo-search adapter promotion, Punk needs:

- R2 research accepted or replaced by a stronger note;
- explicit receipt schema;
- conformance eval specs;
- backend capability manifest shape;
- no-write/no-decision/no-network checks;
- proof that retrieval entering a Contract Context Pack remains advisory and
  receipt-bearing.

## Required evals

Required eval ideas before promotion:

- known-file retrieval;
- known-literal grep retrieval;
- multi-grep bounded OR search;
- known-symbol retrieval, optional;
- known-reference retrieval, optional;
- bounded search budget enforcement;
- output truncation reporting;
- stale-index detection;
- reproducibility across repeated runs;
- no-write/no-decision conformance;
- no-network default;
- secret/env redaction;
- retrieval receipt completeness;
- contract-clause context coverage;
- orphan context rejection;
- stale/superseded source surfacing;
- contradiction surfacing;
- speculative/advisory source exclusion;
- token budget efficiency.

## Required receipt schema fields

Design direction only. This is not an active runtime schema unless later
promoted.

```yaml
retrieval_receipt:
  id: string
  created_at: timestamp
  contract_ref: optional string
  goal_ref: optional string

  backend:
    name: string
    version: string
    kind: lexical | fuzzy | lsp | tree_sitter | code_graph | vector | remote
    capability_manifest_ref: string
    local_only: boolean
    network_used: boolean
    side_effects: none | declared

  query:
    mode: files | grep | multi_grep | symbols | definitions | references
    raw_query: string
    constraints:
      include_paths: []
      exclude_paths: []
      globs: []
      languages: []
      max_results: integer
      max_elapsed_ms: integer
      max_answer_chars: integer

  snapshot:
    repo_root_ref: string
    vcs_ref: optional string
    working_tree_state: clean | dirty | unknown
    index_state: none | live | stale | unknown
    index_basis: optional string

  results:
    result_count: integer
    truncated: boolean
    top_result_refs:
      - path: string
        line_start: optional integer
        line_end: optional integer
        kind: file | match | symbol | reference
        score: optional number
        snippet_hash: optional string

  ranking:
    deterministic: boolean
    ranking_factors: []
    personalized: boolean
    fallback_mode_used: boolean

  warnings:
    stale_index: []
    skipped_files: []
    large_file_omissions: []
    unsupported_modes: []
    truncation: []
    privacy_or_secret_warnings: []
```

## Required docs / ADRs / contracts

- `docs/adapters/repo-search.md` should keep the parked advisory boundary and
  link this note and the new eval specs.
- `evals/specs/repo-search-adapter-boundary.v0.1.md` should define
  no-write/no-decision/no-network/receipt/budget eval boundaries.
- `evals/specs/codebase-context-pack-boundary.v0.1.md` should define
  clause-indexed codebase evidence boundaries.
- A later ADR is optional only if Punk promotes this research into accepted
  architecture or runtime behavior.
- A later bounded goal/contract is required before any runtime adapter,
  context-pack writer, Dev module, MCP, graph, vector, or remote search work.

## Open questions

- Should a later repo-search receipt live only in run receipts, only in context
  packs, or as a separate artifact linked by both?
- How much snippet capture is safe in receipts before privacy and secret
  exposure risks outweigh reproducibility value?
- Should first lexical search be a thin wrapper around shell tools or a small
  internal adapter model?
- What freshness metadata is mandatory for LSP, Tree-Sitter, graph, and vector
  backends?
- Which fixture repo should define the first known-file and known-literal
  retrieval cases?
- Does a later ADR need to precede repo-search implementation, or is research +
  eval spec + bounded contract enough?

## Final recommendation

Recommendation: adopt as R2 advisory research and eval/spec direction. Do not implement runtime yet.

Punk should treat codebase understanding for agents as a bounded repository evidence boundary:
local live search first, receipts always, clause-indexed context packs, derived graphs later, no hidden truth.

This strengthens Punk's contract/evidence/gate/proof posture without activating agent execution, MCP, vector memory, repo-search runtime, or Dev module.

## Knowledge impact

- Canonical artifacts changed: none by this note alone.
- Advisory artifacts changed: this R2 research note, repo-search adapter docs,
  and two eval/spec boundaries.
- Project-memory claims affected: future repo-search and codebase context
  evidence must stay advisory, receipt-bearing, and clause-indexed.
- Docs / ADRs / evals possibly stale: existing repo-search research may need
  future supersession if this note is accepted or promoted.
- Active / parked / future scope affected: repo-search remains parked; Contract
  Context Pack remains advisory evidence; Dev module, MCP, code graph, vector,
  remote search, and agent execution remain future or parked.
- Public narrative impact: none.
- Derived views to rebuild later: none active.
- Follow-up goals or drift findings: optional ADR if this boundary is promoted;
  future fixture eval implementation before adapter promotion.
- Unknowns / contradictions: exact external source details were not fully
  revalidated in this patch.
