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
