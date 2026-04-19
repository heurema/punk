---
kind: research-note
status: active
authority: advisory
created: 2026-04-19
related_goal: work/goals/goal_contract_tracker_research_promotion.md
related_decision: docs/adr/ADR-0009-contract-tracker-core-primitives.md
---

# Contract Tracker Prior Art for Punk

## Question

How should Punk introduce a Contract Tracker without breaking the core laws, creating a second source of truth, or turning the product into a generic task tracker?

## Decision context

Punk already defines itself as an experimental, early-stage, local-first bounded work kernel.

The active-core target is the kernel itself: flow state machine, append-only event log, eval harness, contract lifecycle, gate decision, proofpack, inspectable state, and project memory.

The question is not whether Punk needs a task tracker product. The question is how to express bounded work as an inspectable ledger over the existing `plot / cut / gate` grammar.

## Sources reviewed

| Source | Tier | URL / ref | Why included | Key limitation |
|---|---|---|---|---|
| `README.md`, `docs/product/START-HERE.md`, `docs/product/PUNK-LAWS.md`, `docs/product/ARCHITECTURE.md`, `docs/product/ROADMAP.md`, `docs/product/PROJECT-MEMORY.md`, `docs/product/RESEARCH-GATE.md`, `docs/product/DOGFOODING.md` | A | repo-local docs | Primary source for current Punk laws, flow grammar, architecture, roadmap, project memory, and research rules | Internal design only; no external comparison |
| Linear: issue tracking is dead / context + agents framing | B | https://linear.app/next | Useful contrast between classical issue tracking and context-to-execution systems | Product narrative, not a direct architecture spec |
| Linear Agent Interaction Guidelines | A | https://linear.app/developers/aig | Strong prior art for disclosure, visible internal state, and no agent accountability | Focused on Linear platform behavior |
| Linear Agent introduction / skills / automations materials | B | https://linear.app/changelog/2026-03-24-introducing-linear-agent | Helpful examples for agent workflows, reusable skills, and automation boundaries | Too product-specific for direct adoption |
| Linear Triage Intelligence docs | B | https://linear.app/docs/product-intelligence | Useful inspiration for advisory triage ideas | Strongly productized and AI-heavy |
| Linear Customer Requests docs | B | https://linear.app/docs/customer-requests | Helpful pattern for external signal refs without making them truth | Customer-management oriented |
| GitHub required checks / branch protection docs | A | https://docs.github.com/articles/enabling-required-status-checks | Good precedent for evidence before integration and visible status requirements | Merge-centric, not contract-led |
| SLSA provenance guidance | A | https://slsa.dev/spec/v1.0/provenance | Good precedent for provenance attached to artifacts | Supply-chain scope is broader than Punk needs now |
| Open Policy Agent docs | A | https://www.openpolicyagent.org/docs | Clean separation between policy evaluation and enforcement/decision authority | Policy engine can still imply decisions if mapped naively |
| Cucumber executable specifications | B | https://cucumber.io/docs/gherkin/reference/ | Reinforces executable intent tied to bounded work/spec behavior | Spec language is not Punk's runtime grammar |
| Temporal durable execution concepts | B | https://docs.temporal.io/workflows | Useful inspiration for durable runs and inspectable execution history | Orchestration model is much heavier than Punk core |
| Shape Up bounded work | B | https://basecamp.com/shapeup | Strong fit for bounded slices and appetite-shaped execution | Product planning method, not runtime truth |

## Existing systems / prior art

### Repo truth already points to a ledger, not a tracker product

Current Punk docs already enforce the core boundary:

- `plot` shapes work and creates a contract;
- `cut` executes bounded work;
- `gate` verifies, decides, and proves;
- new capabilities cannot bypass flow, eval, inspectable artifacts, or `gate`;
- project memory already includes a Work Ledger / Contract Tracker surface.

This means the safest move is to tighten the ledger concept, not create a parallel subsystem.

### Linear: context and agents, with strong accountability limits

Useful ideas:

- context can be turned into execution surfaces;
- automation and skills can reduce repeated workflow setup;
- agent state should be visible;
- an agent should not be the accountable owner.

Fit for Punk:

- good inspiration for future advisory triage, delegation, and inspectable agent state;
- not a reason to make agents part of active-core now;
- any Linear-like closure semantics must be translated into gate semantics.

### GitHub required checks and SLSA provenance

Useful ideas:

- visible evidence before acceptance;
- artifact-linked verification;
- provenance attached to what was actually produced.

Fit for Punk:

- proofpack should link evidence to contract/run/decision artifacts;
- do not import GitHub's merge-first worldview or heavy provenance machinery into Phase 3.

### OPA: assessment vs enforcement

Useful idea:

- policy output is structured evaluation over input, separate from the actor that enforces consequences.

Fit for Punk:

- module/rule/eval outputs should be advisory assessments;
- only `gate` writes final decisions.

### Temporal and Shape Up

Useful ideas:

- durable execution history matters;
- bounded slices matter more than broad task lists.

Fit for Punk:

- treat `Cut` as the bounded execution slice;
- keep runtime history inspectable;
- avoid importing heavyweight orchestration concepts into active-core.

## Failure modes found

- Building a tracker product alongside the kernel creates a second source of truth.
- Reintroducing `done` as a closure status undermines gate-only final decisions.
- Letting modules or adapters write decision-like artifacts weakens the laws.
- AI triage too early can solidify unstable semantics.
- Agent delegation can hide accountability if the owner model is unclear.
- Proofpacks can become performative if they collect green checks without evidence value.
- External integrations can dominate the shape of the model before local-first core artifacts stabilize.
- A dedicated tracker crate would encourage feature creep before Phase 3-5 are proven.

## Options considered

### Option A — Build a separate Contract Tracker module

Pros:

- clear surface area;
- could feel product-complete quickly.

Cons:

- duplicates core lifecycle concepts;
- encourages second-source-of-truth behavior;
- contradicts current roadmap and crate boundaries.

### Option B — Treat the Contract Tracker as a Work Ledger over core artifacts

Pros:

- aligns with existing laws and architecture;
- keeps gate as final authority;
- fits Phase 3-5 roadmap naturally;
- avoids new module pressure.

Cons:

- inspect UX may remain minimal for a while;
- `Cut` semantics still need refinement.

### Option C — Delay the concept entirely until agents/modules exist

Pros:

- fewer docs now;
- avoids naming decisions early.

Cons:

- leaves Project Memory under-specified;
- increases odds of ad hoc tracker semantics later;
- wastes the current research pass.

## Recommendation

Adopt Option B.

Define the Contract Tracker as Punk's Work Ledger view over the lifecycle:

```text
Plot -> Contract -> Cut -> Run -> Receipt -> Proofpack -> GateDecision -> EventLog
```

Adopt now:

- documentation for the ledger concept;
- a proposed ADR;
- research and idea-bank artifacts;
- a dogfooding goal for promotion work.

Defer:

- roadmap wording updates until review/gate;
- any new public CLI commitments;
- any new crate or module split;
- advanced ledger views and dependency graphs.

Park:

- AI triage;
- agent delegation;
- skills;
- customer/support ingestion;
- external adapters.

## What stays out of scope

- Jira-style workflow builders;
- agent-owned accountability;
- status-only completion truth;
- adapter-owned tracker state;
- customer/request ingestion in active-core;
- public narrative automation.

## Impact on roadmap

No roadmap change is required immediately.

The concept fits existing phases:

- Phase 3: prove contract loop without agents;
- Phase 4: expose minimal project-memory ledger surface;
- Phase 5: use accepted ledger history for coherence/drift checks.

Any wording change in `docs/product/ROADMAP.md` should happen only after review/gate.

## Required evals

When implementation starts, the minimal eval direction should include:

- no `cut` before approved contract;
- no acceptance without proof-linked evidence;
- only `gate` writes final decision;
- proofpack links or hashes contract, receipt, and decision;
- advisory assessments cannot masquerade as final decisions.

## Required docs / ADRs / contracts

Add now:

- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0009-contract-tracker-core-primitives.md`
- `knowledge/ideas/2026-04-19-contract-tracker-ideas.md`
- `work/goals/goal_contract_tracker_research_promotion.md`

Later, after review/gate:

- roadmap wording updates;
- explicit contract/eval artifacts for Phase 3 implementation.

## Open questions

1. Is `Cut` a persisted first-class entity or a derived execution slice?
2. Where should accepted proof summaries surface?
3. What is the minimal gate outcome vocabulary?
4. When should dependencies and follow-up obligations become first-class?
5. How should evidence freshness be modeled without heavy automation?
