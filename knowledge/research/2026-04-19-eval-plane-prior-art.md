---
kind: research-note
status: active
authority: advisory
created: 2026-04-19
related_decision: docs/adr/ADR-0008-eval-plane.md
---

# Eval Plane Prior Art for Punk

## Question

What existing eval frameworks, benchmarks, and research should inform Punk's initial evaluation plane?

## Decision context

Punk's active-core scope is the stable bounded work kernel: project identity, flow state machine, event log, minimal local eval harness, contract lifecycle, gate decision, proofpack, and inspectable state.

Eval is a core trust surface because Punk's laws require validated transitions, event writing, proof before acceptance, eval coverage for new capabilities, regression blocking, and ledgered overrides. The initial eval plane must therefore exist before feature growth and before modules/adapters/agents become active.

## Punk constraints extracted from repo docs

- Punk is experimental, early-stage, local-first, and not production-ready.
- One CLI: `punk`.
- One lifecycle grammar: `plot / cut / gate`.
- Every executable goal enters a flow and resolves to a contract.
- Every transition is validated by runtime state.
- Every transition writes an event.
- `cut` cannot run unapproved work.
- Every run writes a receipt.
- Only `gate` writes final decision.
- Proof comes before acceptance and links verified artifacts.
- Modules can assess, not decide.
- Adapters can invoke, not own truth.
- New capabilities require eval coverage or explicit no-eval rationale.
- Baseline regressions block promotion unless waived.
- Waivers require explicit reason and are ledgered.
- Research notes are advisory until promoted by ADR/roadmap/contract.
- Initial eval plane is local, repo-stored, deterministic-first, and fixture-based.
- Public CLI exposes only active capabilities.
- Phase 2 requires `punk eval list`, `punk eval run smoke`, deterministic local runner, machine JSON report, and human Markdown report.

## Sources reviewed

| Source | Type | Tier | Date checked | Maintainer / institution | URL | Why relevant to Punk |
|---|---|---:|---|---|---|---|
| Pydantic Evals | official docs | A | 2026-04-19 | Pydantic | https://pydantic.dev/docs/ai/evals/evals/ | Dataset/case/evaluator/report concepts; YAML/JSON datasets. |
| Inspect AI | official docs | A | 2026-04-19 | UK AI Security Institute | https://inspect.aisi.org.uk/ | Broad eval framework, logs, tools, sandbox/agent patterns later. |
| Inspect Evals | official docs/repo | A | 2026-04-19 | UK AISI + contributors | https://inspect.aisi.org.uk/evals/ | Examples of community benchmark implementations. |
| OpenAI Evals | official repo | A | 2026-04-19 | OpenAI | https://github.com/openai/evals | Eval registry/framework patterns; avoid hosted/API dependency in core. |
| LangChain Agent Evals / OpenEvals | official docs/repo | B | 2026-04-19 | LangChain | https://docs.langchain.com/oss/python/langchain/evals | Trajectory evals and LLM-as-judge patterns; mostly not core. |
| SWE-bench Verified | benchmark docs | A | 2026-04-19 | SWE-bench / OpenAI collaboration | https://www.swebench.com/verified.html | Future Dev module inspiration, contamination warning. |
| AppWorld | paper/site/repo | A | 2026-04-19 | Stony Brook NLP et al. | https://appworld.dev/ | State-based unit tests and collateral-damage checks. |
| tau-bench | arXiv/repo | A | 2026-04-19 | Sierra Research / Princeton | https://arxiv.org/abs/2406.12045 | Tool-agent-user benchmark, database-state comparison, pass^k. |
| tau2/tau3-bench | repo/blog | B | 2026-04-19 | Sierra Research | https://github.com/sierra-research/tau2-bench | Reliability, task fixes, dual-control ideas for later. |
| Agent-Diff | arXiv | A | 2026-04-19 | Pysklo et al. | https://arxiv.org/abs/2602.11224 | State-diff-based evaluation for API tasks. |
| MCPToolBench++ | arXiv | A | 2026-04-19 | Fan et al. | https://arxiv.org/abs/2508.07575 | Future MCP/tool-use eval challenges. |
| MCP Security Bench / MCPSecBench | arXiv | A | 2026-04-19 | multiple | https://arxiv.org/abs/2510.15994 | Future adapter/MCP attack taxonomy. |
| AgentBench | arXiv/repo | A | 2026-04-19 | Tsinghua et al. | https://arxiv.org/abs/2308.03688 | Multi-environment interactive agent evaluation. |
| OSWorld | paper/site | A | 2026-04-19 | HKU/Salesforce/CMU/Waterloo | https://os-world.github.io/ | Execution-based evaluation in real computer environments; future only. |
| ReliabilityBench | arXiv | A | 2026-04-19 | Gupta | https://arxiv.org/abs/2601.06112 | Repeated runs, perturbations, fault injection; later reliability layer. |
| Survey on Evaluation of LLM-based Agents | arXiv survey | A | 2026-04-19 | Yehudai et al. | https://arxiv.org/abs/2503.16416 | Taxonomy and gaps: safety, robustness, cost, scalability. |

## Existing systems / prior art matrix

| Source | What it evaluates | Evaluation signal | Deterministic / state support | Fit for Punk | Decision |
|---|---|---|---|---|---|
| Pydantic Evals | AI systems from simple calls to agents | evaluators + reports | supports deterministic evaluators and YAML/JSON datasets | Good vocabulary; Python dependency not core | Adapt now, bridge later |
| Inspect AI | coding, agentic tasks, reasoning, behavior, multimodal | scorers + logs | supports datasets, logs, tools, sandbox concepts | Strong later module/adapters fit | Defer |
| OpenAI Evals | LLM/model/system evals | eval registry + runner | useful patterns, less local-first | Hosted/API orientation conflicts with core | Learn/avoid core |
| LangChain OpenEvals | LLM apps and agent trajectories | deterministic matching / LLM judge | trajectory-focused, LangSmith optional | Useful for later agent modules | Defer |
| SWE-bench Verified | real GitHub software issues | patch passes tests | deterministic tests but public benchmark contamination risk | Future Dev module only | Park |
| AppWorld | interactive coding agents across app APIs | state-based unit tests | strong state/artifact checks | Excellent pattern for Punk artifacts | Adapt ideas now |
| tau-bench | tool-agent-user workflows | DB state vs goal + pass^k | state comparison, repeated-run reliability | Later tool/module reliability | Defer ideas |
| tau2/tau3 | dual-control/multimodal/knowledge variants | task success + fixes | task-quality audit useful | Later modules/adapters | Save ideas |
| Agent-Diff | enterprise API agent tasks | state-diff success | very relevant outcome-over-trace pattern | Useful design idea | Save/adapt |
| MCPToolBench++ | MCP tool use | tool selection/use success | tool output variability | Future adapters only | Park |
| MCP Security Bench / MCPSecBench | MCP attacks | robustness/security metrics | real/malicious tool scenarios | Future adapter safety | Park |
| AgentBench | LLMs as agents in 8 environments | environment reward/success | interactive environments | Too broad for core | Park |
| OSWorld | GUI/CLI computer-use agents | execution-based scripts | real OS environment | Too heavy for core; future Dev/agent | Park |
| ReliabilityBench | production-like reliability | R(k, epsilon, lambda) | repeated runs + perturbations + faults | Later non-deterministic modules | Defer |
| Agent eval surveys | taxonomy and gaps | research synthesis | n/a | Helps avoid blind spots | Use as map |

## Benchmark ideas relevant to Punk

Do not copy any benchmark wholesale. Extract patterns:

1. Hard deterministic gates for core laws.
2. State transition validity over CLI surface.
3. Append-only event log integrity.
4. Contract, receipt, decision, proofpack, and report schema conformance.
5. Receipt completeness before gate.
6. Gate decision exclusivity.
7. Proofpack hash/link integrity.
8. Inspectable state views.
9. Baseline regression detection.
10. Project memory authority rules.
11. Module conformance later.
12. Adapter/tool safety later.
13. Reliability across repeated runs later.
14. Perturbation/fault-injection later.
15. Collateral-damage checks for unexpected state changes.
16. Task-quality audit loop for stale/ambiguous eval cases.

## Failure modes found

- Benchmarks can overfit to public tasks and become contaminated.
- Trace matching can be brittle; outcome/state checks are usually better for workflows.
- LLM-as-judge is useful for subjective tasks but too weak for core hard gates.
- Hosted services introduce nondeterminism, privacy issues, and second-source-of-truth risk.
- Hidden state or hidden memory makes results non-reproducible.
- Weak sandboxing hides side effects.
- Single-run success hides reliability failures.
- Task success without proof artifacts is not enough for Punk.
- Modules/adapters/tooling can accidentally bypass final decision authority.
- Leaderboards can distort product goals.
- Expensive harnesses can block iteration.
- Eval cases themselves need lifecycle management: draft, active, parked, retired.

## Options considered

### Option A — Build Punk-native deterministic eval harness only

Pros: best local-first fit, minimal dependencies, directly maps to Punk Laws, deterministic hard gates.

Cons: Punk must maintain schema/runner/report code; fewer ready-made tools.

Fit: best for Phase 2.

### Option B — Adopt Pydantic Evals directly

Pros: strong case/dataset/report model, YAML/JSON datasets, deterministic and LLM evaluators.

Cons: Python dependency; not Rust-native; potential mismatch with Punk CLI as source of truth.

Fit: compatibility target, not Phase 2 dependency.

### Option C — Use Inspect AI as main harness

Pros: mature eval framework, logs, tools, sandbox/agent ecosystem, prebuilt evals.

Cons: Python-oriented and broader than active-core; can pull project toward agent benchmarks too early.

Fit: defer to modules/adapters research.

### Option D — Use OpenAI Evals / hosted evals

Pros: known eval registry patterns and private eval workflow ideas.

Cons: hosted/API orientation and optional external storage conflict with local-first core.

Fit: learn from patterns, avoid core dependency.

### Option E — Hybrid: Punk-native core with compatibility/export bridges later

Pros: preserves Punk Laws and roadmap while keeping interoperability open.

Cons: requires explicit mapping layer later.

Fit: recommended.

## Recommendation

Adopt Option E.

Build a Punk-native deterministic eval plane now. Treat external frameworks and benchmarks as prior art, not as product truth.

Active-core:

- `evals/_schema/eval-case.v1.yaml`
- `evals/_templates/eval-case.v1.yaml`
- `evals/suites/smoke.yaml`
- first core eval cases for flow/event/contract/run/gate/proof/report/baseline/waiver/public CLI
- machine JSON + human Markdown reports
- hard gates + scorecard
- baseline and waiver model

Incubating:

- Pydantic Evals export/import mapping
- richer eval report views
- fixture snapshot comparison helpers

Parked/future:

- SWE-bench, OSWorld, AgentBench, MCP benchmarks, tau variants, ReliabilityBench-style stress testing.

Avoid:

- hosted eval dependency in core
- LLM-as-judge hard gates
- leaderboard-driven roadmap
- nondeterministic network calls in smoke suite

## Proposed Punk eval taxonomy

### Core hard gates

- illegal transition denied
- every allowed transition writes event
- cut cannot run unapproved work
- gate is only final decision writer
- proof cannot exist before decision
- proof links/hash checks contract + receipt + decision

### Artifact conformance

- contract schema
- receipt schema
- decision schema
- proofpack schema
- report schema

### Regression baselines

- smoke suite green
- fixture snapshots stable
- baseline regression blocks phase promotion unless explicit waiver exists

### Project memory

- goal/report/knowledge links
- authority-tagged knowledge
- speculative knowledge excluded from implementation truth
- missing knowledge update detection later

### Module conformance later

- module can assess but not decide
- module cannot bypass scope
- module emits receipt/assessment

### Adapter/tool safety later

- adapters invoke but do not own truth
- no hidden memory
- external side-effect receipts

## Proposed initial eval case schema

See `evals/_schema/eval-case.v1.yaml` and `evals/_templates/eval-case.v1.yaml`.

Required fields:

- id
- phase
- status
- scope_status
- law_refs
- flow_refs
- fixture_refs
- command_under_test
- pre_state
- expected_result
- expected_events
- expected_artifacts
- hard_gate
- scorecard_metrics
- waiver_policy
- report_output

## Proposed runner/report shape

CLI:

```text
punk eval list
punk eval run smoke
punk eval run <suite-id>
punk eval report <run-id>
```

Report:

- machine JSON: `.punk/evals/runs/<run-id>/report.json`
- human Markdown: `.punk/evals/runs/<run-id>/report.md`
- suite, run id, runner version, repo ref if available
- case results
- hard-gate summary
- scorecard summary
- baseline comparison
- waiver refs

## Impact on roadmap

- Phase 1: flow and events become testable fixtures.
- Phase 2: implements smoke harness and core eval schema.
- Phase 3: adds contract/receipt/decision/proof conformance cases.
- Phase 4: adds project memory authority and link checks.
- Phase 5: adds project drift/coherence evals.
- Phase 6: adds module conformance suite.
- Phase 7: reopens SWE-bench/AppWorld/OSWorld style research for Dev module.
- Phase 8: adds public narrative/PubPunk receipt/style/fact-check evals.
- Phase 9: reopens MCP/security/tool-use research for adapters.

## Required docs / ADRs / evals / knowledge updates

- `docs/product/EVAL-PLANE.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `evals/README.md`
- `evals/suites/smoke.yaml`
- `evals/_schema/eval-case.v1.yaml`
- `evals/_templates/eval-case.v1.yaml`
- `evals/cases/**/*.yaml`
- `knowledge/research/2026-04-19-eval-plane-prior-art.md`
- `knowledge/ideas/2026-04-19-eval-plane-ideas.md`
- `docs/adr/ADR-0008-eval-plane.md`

## What stays out of scope

- LLM contract drafting
- coding agent execution
- provider adapters
- MCP integration
- PubPunk automation
- knowledge embeddings
- plugin marketplace
- leaderboard chasing
- external SaaS dependency in core eval path
- nondeterministic network calls in smoke suite
- autonomous self-certification of trust-surface changes

## Open questions

1. Rust-native runner from day one or temporary validation script before `punk-eval` is implemented?
2. Exact waiver ledger path and whether waiver summaries should be repo-tracked in `work/reports/`.
3. How strict should schema validation be before Phase 3 artifacts exist?
4. Should `smoke.yaml` be the only Phase 2 promotion suite, or should `core.yaml` become the comprehensive suite?

## Final recommendation summary

Adopt now:

- Punk-native deterministic eval harness.
- Hard gates plus scorecard.
- Machine JSON and human Markdown reports.
- Baseline regression blocking and waiver ledger.

Adapt now:

- Pydantic-style case/dataset/report vocabulary.
- AppWorld/Agent-Diff-style state/artifact checks.

Defer:

- Inspect/Pydantic/OpenEvals runner integration.
- Reliability stress testing.
- Repeated-run pass^k.

Park:

- SWE-bench, OSWorld, AgentBench, MCP benchmarks.

Avoid:

- hosted evals as core dependency.
- LLM-as-judge hard gates.
- leaderboard-driven product goals.

Minimal Phase 2 eval harness design:

- YAML suites and cases under `evals/`.
- `punk eval list` and `punk eval run smoke`.
- deterministic runner only.
- report JSON + Markdown under `.punk/evals/runs/`.
- fail promotion on hard-gate failures or baseline regressions.

Required first 10 eval cases:

1. `eval_flow_cannot_cut_before_approve`
2. `eval_flow_cannot_proof_before_decision`
3. `eval_flow_every_transition_writes_event`
4. `eval_flow_resume_from_persisted_state`
5. `eval_contract_scope_required_for_cut`
6. `eval_run_writes_receipt_before_gate`
7. `eval_gate_only_gate_writes_decision`
8. `eval_proof_hashes_verified_artifacts`
9. `eval_eval_report_writes_machine_and_human_outputs`
10. `eval_project_no_parked_capability_exposed`

Also recommended:

11. `eval_baseline_regression_blocks_promotion`
12. `eval_waiver_requires_reason_and_ledger`
