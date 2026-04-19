# ADR-0008: Punk-native eval plane before feature growth

Status: Accepted
Date: 2026-04-19

## Context

`punk` is an experimental, early-stage, local-first bounded work kernel. The active-core target includes flow, event log, eval smoke harness, contract lifecycle, gate, proofpack, project identity, and inspectable state.

The project already requires minimal evals as part of core, not as a later module. A capability cannot be promoted unless it has flow semantics, eval coverage or explicit no-eval rationale, inspectable artifacts, and no bypass around `gate`.

Prior-art research reviewed Pydantic Evals, Inspect AI, OpenAI Evals, SWE-bench, AppWorld, tau-bench, Agent-Diff, MCP security benchmarks, OSWorld, AgentBench, ReliabilityBench, and recent LLM-agent evaluation surveys.

## Decision

Build a Punk-native deterministic eval harness for active-core.

The Phase 2 smoke harness must be:

- local-first;
- repo-stored;
- deterministic-first;
- fixture-based;
- hard-gate plus scorecard;
- able to emit machine JSON and human Markdown reports;
- compatible with Pydantic Evals concepts later, without depending on Python, web UI, hosted tracing, or SaaS in the core path.

Adopt now:

- Punk-native suite/case/report schema;
- hard gates for core laws;
- scorecard metrics as advisory signal;
- baseline regression blocking;
- explicit waiver ledger;
- state/artifact checks instead of brittle trajectory matching.

Adapt now:

- Pydantic-style dataset/case/evaluator/report vocabulary;
- AppWorld-style state-based and collateral-damage checks;
- Agent-Diff-style state-diff thinking;
- Inspect-style log/provenance concepts for future report detail.

Defer:

- external runner integration;
- Pydantic export/import;
- repeated-run reliability surfaces;
- perturbation/fault-injection;
- sandboxed agent evals.

Park:

- SWE-bench, OSWorld, AgentBench, MCP benchmarks as future module/adapter research inputs.

Avoid:

- hosted evals as a core dependency;
- LLM-as-judge hard gates;
- public leaderboard chasing;
- nondeterministic network calls in smoke suite;
- autonomous self-certification of core trust-surface changes.

## Consequences

Positive:

- Core trust surfaces get regression signal before feature growth.
- Eval files become project memory, not chat-only research.
- Future frameworks can interoperate without owning truth.
- Phase promotion has inspectable evidence.

Negative / costs:

- Punk must maintain its own minimal runner and schema.
- Compatibility with external eval frameworks is delayed.
- Early cases may be verbose until runner ergonomics improve.

## Scope classification

- Active-core: smoke harness, hard gates, scorecard, report shape, baselines, waivers.
- Incubating: Pydantic Evals compatibility/export, richer report views.
- Parked: Dev-agent benchmarks, MCP tool benchmarks, GUI/OS benchmarks.
- Future: module conformance, adapter/tool safety, reliability stress testing.

## Touched laws / phases / risks

Laws: 6, 7, 8, 9, 10, 11, 12, 18, 19, 20, 26-30.

Phases:

- Phase 1: flow/event kernel supplies transitions/events to test.
- Phase 2: eval smoke harness implements this ADR.
- Phase 3: contract loop adds schema/receipt/gate/proof cases.
- Phase 6+: module conformance and adapter safety extend the taxonomy.

Risks:

- eval runner becomes another source of truth;
- scorecard becomes a hidden decision layer;
- external frameworks pull core away from local-first;
- green smoke suite is overclaimed as production readiness.

Mitigation:

- only `gate` writes final decisions;
- evals report evidence but do not accept work;
- core runner remains local and deterministic;
- documentation must keep `punk` experimental and early-stage.

## Required project updates

- `docs/product/EVAL-PLANE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `evals/README.md`
- `evals/suites/smoke.yaml`
- `evals/_schema/eval-case.v1.yaml`
- `evals/_templates/eval-case.v1.yaml`
- first active-core eval cases under `evals/cases/`
- `knowledge/research/2026-04-19-eval-plane-prior-art.md`
- `knowledge/ideas/2026-04-19-eval-plane-ideas.md`

## Open questions

1. Should the Phase 2 runner be pure Rust from day one, or can a temporary script validate YAML while `punk-eval` matures?
2. What exact waiver ledger path should be canonical: `.punk/evals/waivers/` only, or also a repo-tracked work/report reference?
3. Should `core.yaml` include all active-core evals, while `smoke.yaml` stays minimal, or should smoke be the promotion suite for early phases?
4. What is the minimal schema validation library acceptable for `punk-core` without bloating dependencies?
