---
kind: idea-bank
status: active
authority: speculative
created: 2026-04-19
related_research: knowledge/research/2026-04-19-eval-plane-prior-art.md
related_decision: docs/adr/ADR-0008-eval-plane.md
---

# Eval Plane Ideas to Preserve

These are useful ideas from prior art that should not be adopted into active-core yet.

They are speculative until promoted through ADR, roadmap, goal/contract, implementation, and eval results.

## Defer / preserve

### State-diff outcome checks

Source inspiration: AppWorld and Agent-Diff.

Idea: evaluate final state and artifact diff instead of exact trajectory. This fits Punk because two valid workflows may write equivalent final artifacts while taking different internal steps.

Potential future use:

- proofpack hash/diff checks;
- project memory drift checks;
- adapter side-effect receipts;
- module receipt collateral-damage checks.

### Collateral-damage checks

Source inspiration: AppWorld.

Idea: a task can succeed while damaging unrelated state. Punk should eventually assert both expected changes and forbidden changes.

Potential future use:

- project memory update checks;
- external side-effect surfaces;
- public narrative publication receipts;
- adapter write permissions.

### pass^k / repeated-run reliability

Source inspiration: tau-bench and ReliabilityBench.

Idea: measure reliability over repeated runs, not just one pass. This is not needed for deterministic Phase 2, but becomes relevant for LLM-driven or tool-driven modules.

Potential future use:

- contract drafting quality;
- Dev module bounded execution;
- Pub module style/fact-check workflows;
- adapter retries and failure handling.

### Reliability surface R(k, epsilon, lambda)

Source inspiration: ReliabilityBench.

Idea: model reliability across repeated runs, task perturbations, and tool/API failures.

Potential future use:

- Phase 7 Dev module;
- Phase 8 Pub module;
- Phase 9 adapters;
- provider/tool outage simulation.

### MCP security attack taxonomy

Source inspiration: MCPToolBench++, MSB, MCPSecBench, and MCP security papers.

Idea: when adapters/MCP enter scope, evaluate name-collision attacks, malicious tool descriptions, retrieval injection, false errors, tool-transfer attacks, out-of-scope parameters, and user-impersonating responses.

Potential future use:

- adapter allowlist policy;
- tool description trust rules;
- side-effect receipts;
- hidden-memory and secret-leak prevention.

### Inspect-style eval log provenance

Source inspiration: Inspect AI.

Idea: eval logs can carry invalidation/edit provenance. Punk waivers and report edits should record who changed what and why.

Potential future use:

- waiver ledger schema;
- report invalidation;
- dogfooding handoff reports.

### Benchmark task-quality audit loop

Source inspiration: tau3 task fixes and SWE-bench contamination discussions.

Idea: benchmark cases themselves need review because tasks become stale, ambiguous, contaminated, or impossible.

Potential future use:

- eval case review cadence;
- `eval_case.status: retired` policy;
- stale fixture detection;
- no leaderboard-chasing rule.

## Do not promote without new ADR

- Hosted eval dashboards as a core dependency.
- LLM-as-judge hard gates.
- Public benchmark leaderboard goals.
- MCP or provider adapters in active-core.
- Coding-agent benchmarks as Phase 2 requirements.
