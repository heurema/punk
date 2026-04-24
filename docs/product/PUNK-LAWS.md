---
id: docs_product_punk_laws
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-24
review_after: 2026-07-20
canonical_for:
  - core-laws
  - rule-precedence
  - dogfooding-laws
  - research-laws
  - public-narrative-laws
  - telemetry-and-research-intake-laws
  - executor-agnostic-validation-laws
related_docs:
  - docs/product/START-HERE.md
  - docs/product/ARCHITECTURE.md
  - docs/product/DOC-GOVERNANCE.md
supersedes: []
superseded_by: null
---
# Punk Laws

## Core laws

1. One CLI: `punk`.
2. One lifecycle grammar: `plot / cut / gate`.
3. Every executable goal enters a flow.
4. Every executable goal resolves to a contract.
5. Every contract has scope.
6. Every transition is validated by runtime state.
7. Every transition writes an event.
8. `cut` cannot run unapproved work.
9. Every run writes a receipt.
10. Only `gate` writes the final decision.
11. Proof comes before acceptance.
12. Proof links to the artifacts that were verified.
13. Modules can assess; modules cannot decide.
14. Adapters can invoke; adapters cannot own truth.
15. Project memory is explicit and authority-tagged.
16. Raw ideas are not implementation truth by default.
17. Project drift must be inspectable.
18. New capabilities require eval coverage or explicit no-eval rationale.
19. Baseline regressions block promotion unless explicitly waived.
20. Overrides require an explicit reason and are ledgered.

## Rule precedence

```text
Core Rules
  > Project Rules
    > Module Rules
      > Profile Rules
        > Run Contract
```

Lower layers can narrow or specialize upper rules. They cannot weaken them.


## Dogfooding laws

21. `punk` should track its own development from day zero.
22. Self-execution must not outrun earned trust level.
23. Core trust-surface changes require stronger review than ordinary work.
24. Dogfooded work must leave goal/report/eval/proof refs appropriate to its level.
25. The project must not claim a higher dogfooding level than the artifacts prove.


## Research laws

26. Important decisions require curated research before implementation.
27. Research must be stored in repo-tracked knowledge, not only chat.
28. Research notes must distinguish primary evidence from weak commentary.
29. Research does not become product truth until promoted through ADR/roadmap/contract.
30. Contracts for important architecture work should cite research refs or explain why research was not required.

## Public narrative laws

31. Public build artifacts are project memory.
32. Manual publication is allowed before PubPunk automation.
33. Published posts require publication receipts.
34. Metrics snapshots must identify source, channel, and collection time.
35. PubPunk must adopt the repo-tracked public structure instead of creating a hidden content store.
36. Public claims must be supported by canonical knowledge or framed as opinion.

## Telemetry and research-intake laws

37. Research may suggest mechanisms; it does not override active-core scope.
38. Telemetry is local trust evidence, not hidden analytics.
39. Remote export is parked until promoted through Research Gate and ADR.
40. Guard results, module outputs, and adapter outputs are assessments, not final decisions.
41. Proofpacks must reference verifiable artifacts and hashes.
42. Derived inspect views are regenerable and must not become a second source of truth.
43. Project memory links must preserve explicit refs between goals, contracts, reports, evals, decisions, proofs, and public narrative.

## Executor-agnostic validation boundary laws

44. Punk is executor-agnostic, not validation-agnostic. Work may be attempted by any user-chosen human, script, model, coding agent, IDE, module, or adapter.
45. User runtime is the substrate, not the authority. Punk may run validators in the user's environment, but final authority comes from contracts, receipts, evals, gate decisions, proofpacks, and project-memory links.
46. Executor claims are not proof. A claim such as "tests passed", "scope preserved", or "done" must be verified, captured as unverified, or rejected/deferred.
47. Semantic assessors may assess, not decide. LLM-based review can produce clause-specific evidence, but cannot write final acceptance.
48. Persistent prompts, skills, playbooks, model settings, and provider settings are runner aids, not project truth. Durable runner aids require explicit scope, owner, evidence refs, and a review or retirement path.
49. A repeated executor failure becomes a contract clause, validator, receipt field, eval case, proof requirement, or memory artifact before it becomes a global instruction.
