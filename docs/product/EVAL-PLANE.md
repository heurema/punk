---
id: docs_product_eval_plane
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-19
updated_at: 2026-04-20
review_after: 2026-07-20
canonical_for:
  - eval-runtime-report-shape
  - deterministic-eval-architecture
  - eval-signal-model
  - eval-storage-model
  - eval-runner-shape
related_docs:
  - docs/product/EVAL.md
  - docs/product/ROADMAP.md
  - docs/product/START-HERE.md
related_adrs:
  - docs/adr/ADR-0008-eval-plane.md
related_evals:
  - evals/_schema/eval-case.v1.yaml
  - evals/specs/2026-04-19-core-trust-evals.md
supersedes: []
superseded_by: null
---

# Eval Plane

Status: accepted design direction for active-core, implementation still phase-gated.
Scope: Phase 2 active-core eval smoke harness, with future extension hooks.
Research ref: `knowledge/research/2026-04-19-eval-plane-prior-art.md`
Decision ref: `docs/adr/ADR-0008-eval-plane.md`

## Purpose

The eval plane is a core trust surface. It exists before feature growth so that `punk` can detect regressions in flow, events, contracts, gate decisions, proofpacks, and inspectable state.

The first eval plane is not a benchmark leaderboard, not an LLM judge harness, and not a hosted observability product. It is a local, repo-stored, deterministic-first regression layer for Punk Laws.

## Decision

Adopt a Punk-native deterministic eval harness for core behavior.

Adapt ideas from prior art:

- Pydantic Evals: case/dataset/evaluator/report vocabulary and YAML/JSON compatibility.
- AppWorld: state-based checks and collateral-damage checks.
- Agent-Diff: state-diff evaluation over brittle trajectory matching.
- Inspect AI: structured eval logs, provenance, and sandbox concepts later.
- tau-bench / ReliabilityBench: repeated-run reliability and perturbation/fault-injection hooks later.
- MCP security benchmarks: adapter/tool attack taxonomy later.

Do not adopt any external runner as the core dependency in Phase 2.

## Active-core scope

Phase 2 must cover:

- flow transition validity;
- append-only event writing;
- `cut` cannot run unapproved work;
- receipt existence before gate;
- only `gate` writes final decision;
- proof only after decision;
- proof links and hashes verified artifacts;
- contract scope/schema conformance;
- eval report shape;
- baseline regression blocking;
- explicit waiver ledger;
- parked capabilities not exposed through public CLI.

## Out of scope for Phase 2

- LLM contract drafting;
- coding agent execution;
- provider adapters;
- MCP integration;
- PubPunk automation;
- knowledge embeddings;
- module marketplace;
- hosted eval dependency;
- nondeterministic network calls;
- LLM-as-judge hard gates;
- public leaderboard chasing;
- autonomous self-certification of trust-surface changes.

## Eval signal model

### Hard gates

Hard gates are deterministic checks that block promotion when red.

Examples:

- illegal transition is denied;
- allowed transition writes an event;
- `cut` cannot run before approval;
- gate is the only final decision writer;
- proof cannot exist before decision;
- proof hashes contract, receipt, and decision;
- parked capability is not public CLI.

### Scorecard

Scorecard metrics are informative unless explicitly promoted to hard gate.

Examples:

- report completeness ratio;
- number of fixture assertions;
- artifact coverage by type;
- runtime duration;
- skipped optional checks;
- follow-up eval suggestions.

### Waivers

A hard-gate failure can be waived only with an explicit reason and ledger entry. Waivers must include:

- eval case id;
- failing assertion;
- reason;
- approver;
- expiry or follow-up goal;
- linked report or contract.

Waivers do not make the failed behavior correct. They only make the promotion decision explicit and inspectable.

## Storage model

Repo-tracked:

```text
evals/
  _schema/
  _templates/
  suites/
  cases/
knowledge/research/
knowledge/ideas/
docs/adr/
docs/product/
```

Runtime / derived:

```text
.punk/evals/runs/<run-id>/report.json
.punk/evals/runs/<run-id>/report.md
.punk/evals/baselines/<suite>.json
.punk/evals/waivers/<waiver-id>.yaml
```

When an eval run is part of meaningful dogfooded work, the human-readable summary should be linked from `work/reports/`.

## Initial case schema

The first Phase 2 active-core case format is YAML and intentionally close to Pydantic Evals concepts without requiring Python:

```yaml
id: eval_flow_cannot_cut_before_approve
phase: 2
status: draft
scope_status: active-core
law_refs: ["8", "18", "19"]
flow_refs: ["run_contract"]
fixture_refs:
  - fixtures/flow/contract_drafted_without_approval
command_under_test:
  cli: "punk cut run --contract fixtures/contracts/draft.yaml"
pre_state:
  flow_state: contract_drafted
expected_result:
  exit_code: 1
  denied: true
expected_events: []
expected_artifacts:
  forbidden:
    - run_ref
    - receipt_ref
hard_gate: true
scorecard_metrics:
  - transition_guard_checked
waiver_policy:
  allowed: false
report_output:
  include_in: ["machine_json", "human_markdown"]
```

Older non-smoke sketches may remain in lighter pre-v1 form until migrated. The `smoke` suite should be kept on the v1 path.

## Runner shape

CLI:

```text
punk eval list
punk eval run smoke
punk eval run <suite-id>
punk eval report <run-id>
```

Phase 2 report requirements:

- machine JSON report;
- human Markdown report;
- suite id, run id, timestamp, repo ref if available;
- case-level pass/fail/skip;
- hard-gate summary;
- scorecard summary;
- baseline comparison;
- waiver references;
- artifact refs.

## Baseline rule

Baseline regressions block phase promotion unless an accepted waiver exists. Baselines should be compared on deterministic outputs only.

Initial baseline dimensions:

- case status;
- hard-gate status;
- expected event count;
- expected artifact refs;
- schema version;
- runner version.

## Compatibility with Pydantic Evals later

Punk should support export/import mapping later:

| Punk | Pydantic Evals analogue |
|---|---|
| suite | Dataset |
| case | Case |
| expected_result / expected_artifacts | expected_output / metadata |
| hard_gate | deterministic evaluator |
| scorecard_metrics | metrics / attributes |
| report.json | EvaluationReport |

This compatibility is incubating/future tooling. It must not become a core runtime dependency unless a later ADR changes the local-first policy.

## Promotion rules

A capability may move toward active surface only if:

1. flow transition exists;
2. eval case exists or explicit no-eval rationale is ledgered;
3. inspectable artifacts are written;
4. it does not bypass gate;
5. it does not create another source of truth;
6. smoke suite is green;
7. phase-specific evals are green;
8. docs and ADRs are updated.

## Source status

| Area | Decision |
|---|---|
| Punk-native eval harness | adopt now |
| Pydantic Evals compatibility | adapt now, implement later |
| AppWorld state-based checks | adapt now |
| Agent-Diff state-diff checks | save/adapt as design idea |
| Inspect AI runner | defer |
| OpenAI Evals / hosted evals | avoid as core, learn from patterns |
| SWE-bench / OSWorld / AgentBench | park for future Dev/module research |
| tau-bench reliability ideas | defer, keep schema hooks |
| ReliabilityBench-style stress testing | defer |
| MCP security benchmarks | park for adapters phase |
