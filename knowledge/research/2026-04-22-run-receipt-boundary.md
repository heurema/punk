---
kind: research-note
status: active
authority: advisory
created: 2026-04-22
related_goal: work/goals/goal_research_run_receipt_boundary.md
related_report: work/reports/2026-04-22-run-receipt-boundary-research.md
---

# Run Receipt Boundary

## Question

What boundary should Punk adopt for future run receipts before implementing any `.punk/runs` storage or receipt schema?

More specifically:

- what a run receipt is in Punk;
- what it must reference;
- what the minimum useful receipt model is;
- how it differs from event log, eval report, gate decision, and proofpack;
- where it should live later;
- what is canonical versus derived;
- which next bounded step is safe.

## Decision context

Punk now has:

1. flow state and append-only event evidence;
2. deterministic smoke eval coverage for flow and contract-aware guards;
3. minimal contract lifecycle primitives;
4. contract-to-flow guard integration.

The next tempting step is a run receipt because core docs already say:

- every run writes a receipt;
- gate decisions may later reference receipts;
- proof should later hash or reference receipts;
- `.punk/runs/` is the future runtime evidence surface for receipts.

That makes run receipts an R2 boundary question, not a small implementation diff. Receipts touch:

- runtime evidence shape;
- artifact ordering between contract, run, receipt, gate, and proof;
- provenance and later hash/reference needs;
- canonical-vs-derived boundaries inside `.punk/`.

Research is advisory.
It does not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Punk constraints extracted from repo docs

From `docs/product/PUNK-LAWS.md`, `docs/product/ARCHITECTURE.md`, `docs/product/ROADMAP.md`, `docs/product/CONTRACT-TRACKER.md`, `docs/product/PROJECT-MEMORY.md`, and `docs/product/EVAL-PLANE.md`:

- every executable goal resolves to a contract;
- every transition writes an event;
- `cut` cannot run unapproved work;
- every run writes a receipt;
- only `gate` writes the final decision;
- proof references artifacts and hashes later;
- evals and guard results assess, but do not decide;
- `.punk/runs/` is the intended future runtime evidence surface for run receipts;
- derived inspect views must not become a second source of truth;
- receipt existence before gate is already part of the eval-plane target.

## Sources reviewed

| Source | Type | Quality | Date checked | URL / ref | Why relevant |
|---|---|---|---|---|---|
| SLSA provenance v1.0 | official spec | A (official) | 2026-04-22 | [slsa.dev/spec/v1.0/provenance](https://slsa.dev/spec/v1.0/provenance) | Useful precedent for subject/artifact provenance, builder context, and future hash/reference design. |
| GitHub Actions workflow artifacts | official docs | A (official) | 2026-04-22 | [docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts) | Useful precedent for storing per-run outputs as artifacts rather than as canonical decision truth. |
| GitHub Checks API | official docs | A (official) | 2026-04-22 | [docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#create-a-check-run](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#create-a-check-run) | Useful precedent for status/conclusion/output/correlation fields without conflating them with final project truth. |
| GitLab unit test reports | official docs | A (official) | 2026-04-22 | [docs.gitlab.com/ci/testing/unit_test_reports](https://docs.gitlab.com/ci/testing/unit_test_reports/) | Strong precedent for report artifacts that do not decide job outcome on their own. |
| OpenTelemetry Logs Data Model | official spec | A (official) | 2026-04-22 | [opentelemetry.io/docs/specs/otel/logs/data-model](https://opentelemetry.io/docs/specs/otel/logs/data-model/) | Future reference for timestamp/correlation/attribute vocabulary. |
| W3C PROV-DM | official spec | A (official) | 2026-04-22 | [w3.org/TR/prov-dm](https://www.w3.org/TR/prov-dm/) | Useful provenance vocabulary around entities, activities, and responsible agents. |
| Bazel Test Encyclopedia | official docs | A (official) | 2026-04-22 | [bazel.build/reference/test-encyclopedia](https://bazel.build/reference/test-encyclopedia) | Useful precedent for per-run logs and XML outputs as execution artifacts. |
| Temporal event history docs | official docs | A (official) | 2026-04-22 | [docs.temporal.io/workflow-execution/event#event-history](https://docs.temporal.io/workflow-execution/event#event-history) | Useful precedent for append-only event history distinct from higher-level run/result summaries. |
| `docs/product/PUNK-LAWS.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/PUNK-LAWS.md` | Defines every-run-writes-receipt and only-gate-writes-final-decision laws. |
| `docs/product/ARCHITECTURE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/ARCHITECTURE.md` | Defines `.punk/runs/`, canonical evidence surfaces, and assessment-vs-decision boundaries. |
| `docs/product/ROADMAP.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/ROADMAP.md` | Defines phase expectations that run writes receipt before gate and proof later references it. |
| `docs/product/CONTRACT-TRACKER.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/CONTRACT-TRACKER.md` | Defines receipt as a core lifecycle primitive and ledger artifact. |
| `docs/product/PROJECT-MEMORY.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/PROJECT-MEMORY.md` | Useful for canonical-vs-derived and receipt/metadata inspectability boundaries. |
| `docs/product/EVAL-PLANE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/EVAL-PLANE.md` | Confirms receipt existence before gate and later proof/hash expectations. |
| `docs/adr/ADR-0009-contract-tracker-core-primitives.md` | repo ADR | project canonical | 2026-04-22 | `docs/adr/ADR-0009-contract-tracker-core-primitives.md` | Confirms receipt as active-core primitive and phase direction. |
| `work/reports/2026-04-22-contract-lifecycle-minimal.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-contract-lifecycle-minimal.md` | Shows current bounded contract semantics before receipts exist. |
| `work/reports/2026-04-22-connect-contract-to-flow-state.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-connect-contract-to-flow-state.md` | Shows current contract-to-flow authorization path. |
| `work/reports/2026-04-22-contract-flow-smoke-eval.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-contract-flow-smoke-eval.md` | Confirms contract-to-flow behavior is now regression-covered before receipts. |
| `evals/specs/smoke-eval-report.v0.1.md` | repo design | repo design | 2026-04-22 | `evals/specs/smoke-eval-report.v0.1.md` | Useful contrast: eval report is assessment-only. |
| `evals/specs/eval-storage-boundary.v0.1.md` | repo design | repo design | 2026-04-22 | `evals/specs/eval-storage-boundary.v0.1.md` | Useful precedent for append-only/run-scoped storage boundaries. |
| `evals/specs/eval-baseline-waiver-boundary.v0.1.md` | repo design | repo design | 2026-04-22 | `evals/specs/eval-baseline-waiver-boundary.v0.1.md` | Useful precedent for evidence/governance boundaries that do not become decisions. |

## Source quality table

| Source | Reliability | What we can safely use | Cautions |
|---|---|---|---|
| SLSA provenance | High | subject/artifact refs, builder/run details, provenance as separate evidence | Too heavyweight for immediate Punk runtime implementation. |
| GitHub artifacts | High | per-run uploaded artifacts with retention/download semantics | Artifacts are CI outputs, not canonical local lifecycle truth. |
| GitHub Checks API | High | `status`, `conclusion`, `external_id`, and structured output summary/text | Checks are integration surfaces, not final decision artifacts. |
| GitLab unit reports | High | report artifacts do not decide job status by themselves | JUnit/report tooling is compatibility precedent, not Punk's canonical receipt model. |
| OTel logs | High | timestamp/correlation/attributes vocabulary | Telemetry/export systems are parked for Punk. |
| W3C PROV-DM | High | entity/activity/agent framing for provenance | Too generic to dictate a concrete receipt model by itself. |
| Bazel test docs | High | per-run outputs/logs and deterministic test result artifacts | Build/test infra semantics do not map 1:1 to Punk lifecycle semantics. |
| Temporal event history | High | append-only event history as durable execution record | Event history alone is not enough to define a higher-level receipt artifact. |
| Repo canonical docs | High for Punk context | laws, lifecycle order, `.punk/` evidence surfaces, authority rules | They define target truth, not all interoperability tradeoffs. |
| Repo work reports | Medium-high | what is already implemented and what remains deferred | Execution evidence is not architecture approval by itself. |

## Prior art / existing systems

### 1. GitHub artifacts treat outputs as per-run artifacts, not final decisions

GitHub Actions artifact docs describe artifacts as data shared between jobs and stored after a workflow completes. That is a strong precedent for:

- per-run outputs;
- retention/download semantics;
- artifact storage separate from decision authority.

Takeaway for Punk:

- future run receipts should be evidence artifacts tied to a run;
- receipt storage should not be conflated with final project truth;
- later inspect views may summarize receipts, but should not replace them.

### 2. GitHub Checks separate status/conclusion/output from artifact truth

GitHub Checks expose:

- `status`;
- `conclusion`;
- `external_id`;
- structured `output` with `title`, `summary`, and `text`.

Takeaway for Punk:

- receipt correlation ids and human/machine summaries are useful;
- the existence of a status-like field does not make a receipt a final decision;
- correlation and output shape should stay separate from gate authority.

### 3. GitLab unit reports show report artifacts do not decide job status

GitLab explicitly says unit test reports require JUnit XML artifacts and do not affect job status by themselves; the job must still exit non-zero to fail.

Takeaway for Punk:

- artifact/report evidence and final enforcement are different layers;
- by analogy, a run receipt can exist before gate and still not decide acceptance;
- receipt status alone must not be treated as closure.

### 4. Bazel and Temporal distinguish raw execution history from higher-level result artifacts

Bazel test docs emphasize deterministic test environments and per-run outputs such as logs or XML outputs. Temporal documents an append-only event history used for durable execution and recovery.

Takeaway for Punk:

- event log and receipt should be distinct layers;
- append-only event history is useful, but it is not the same as a compact “what this run did” artifact;
- a receipt should summarize or reference execution artifacts, not replace the event stream.

### 5. SLSA and W3C PROV help shape future provenance without forcing implementation now

SLSA uses subjects, builder context, and run details for provenance. PROV-DM frames provenance around entities, activities, and agents.

Takeaway for Punk:

- future receipts should be referenceable by id/hash and later fit into proofpack provenance;
- actor/process provenance should be anticipated, but not overbuilt now;
- raw provider payloads or full supply-chain provenance are unnecessary for the next bounded step.

## Failure modes

1. **Receipt becomes a gate decision surrogate**
   - violates `only gate writes the final decision`.

2. **Receipt is stored only as mutable `latest`**
   - overwrites history and becomes hidden authority.

3. **Denied transition writes a receipt**
   - blurs the line between transition evidence and actual run execution.

4. **Receipt duplicates the full event log**
   - creates redundant truth surfaces and makes drift harder to inspect.

5. **Receipt duplicates raw eval report semantics**
   - mixes execution evidence with assessment output and weakens boundaries.

6. **Receipt stores secrets or raw provider payloads**
   - expands trust surface and violates inspectable/minimal evidence discipline.

7. **Receipt implementation starts before run identity and reference rules are explicit**
   - later proof/gate references become unstable or ad hoc.

8. **Receipt is treated as repo-tracked product truth**
   - confuses runtime evidence with canonical docs/research/work-ledger truth.

## Options considered

### Option A — No run receipts yet

Keep current contract-flow/eval evidence only.

Pros:

- no new runtime/storage surface;
- no premature schema or provenance model;
- keeps current scope safe.

Cons:

- the core law “every run writes a receipt” remains unimplemented;
- no dedicated execution artifact exists between run and future gate/proof;
- later proofpack design lacks a concrete receipt boundary.

Assessment:

- acceptable only as a temporary current state;
- not a good long-term endpoint.

### Option B — Minimal in-memory receipt model only

Define run receipt semantics in code without storage.

Pros:

- can clarify vocabulary before `.punk/runs`;
- avoids storage activation.

Cons:

- still mixes design and implementation;
- risks inventing semantics before boundary/spec agreement;
- does not solve canonical-vs-derived questions.

Assessment:

- defer until after a boundary/spec artifact exists.

### Option C — Append-only `.punk/runs` receipts later

Future candidate: store receipts as append-only run-scoped evidence artifacts under `.punk/runs/`.

Pros:

- aligns with `docs/product/ARCHITECTURE.md`;
- preserves run history;
- supports later gate/proof references by id/hash;
- matches local trust evidence architecture.

Cons:

- requires boundary/spec first;
- requires run identity and receipt shape decisions;
- increases runtime evidence surface.

Assessment:

- adopt as future direction, not immediate implementation.

### Option D — Event log only, no distinct receipt artifact

Treat append-only events as sufficient and skip a distinct receipt.

Pros:

- no new artifact class;
- simpler runtime model at first glance.

Cons:

- conflicts with Punk Laws and roadmap direction;
- event log records lifecycle transitions, not a compact run outcome artifact;
- makes later gate/proof references more awkward and less inspectable.

Assessment:

- avoid.

### Option E — Receipt as future proofpack input

Keep receipt as a distinct evidence artifact that later proofpacks reference.

Pros:

- aligns with architecture and roadmap;
- keeps receipt below gate/proof authority;
- supports later hash/reference integrity.

Cons:

- only safe after boundary/spec work;
- provenance details can sprawl if overdesigned too early.

Assessment:

- adopt as direction, defer implementation.

## Explicit boundary answers

| Question | Answer |
|---|---|
| Is a receipt a canonical artifact or a derived view? | A future receipt should be a canonical **run evidence artifact** inside the runtime evidence plane, while `latest` pointers, summaries, and inspect views over receipts remain derived. It is not canonical product-doc truth and not a final decision artifact. |
| Can a receipt exist before gate decision? | Yes. Core docs require run receipt before gate, and gate later references receipts. |
| Can a receipt imply final acceptance? | No. Only `gate` writes the final decision. |
| What must be true before a receipt is written? | A bounded run must actually exist: approved contract/scope guards have passed, a run identity exists, and execution has produced inspectable run evidence beyond mere denied transition evidence. |
| Does a denied transition produce a receipt, an event, or both? | Event only. A denied transition is guard evidence in the append-only event log, not a run receipt. |
| Should a receipt include command output, artifact refs, hashes, timing, actor, contract ref, flow state, eval refs? | Minimum future answer: include contract ref/id, run id, execution source/command identity, timing, outcome, produced artifact refs, and event refs; hashes, actor identity, and eval refs should be designed but may start optional in the first boundary/spec step. |
| What must not go into a receipt yet? | Secrets, raw provider payloads, full prompts, environment dumps, mutable “latest” semantics, gate decisions, proofpack bodies, and remote-export-specific data. |
| How does receipt differ from event log? | Event log records append-only lifecycle events and guard denials. Receipt records what a specific run actually did and what artifacts it produced. |
| How does receipt differ from eval report? | Receipt records execution evidence. Eval report records assessment/evaluation results. They may cross-reference later but should remain separate artifacts. |
| How does receipt differ from proofpack? | Receipt is one evidence artifact from a run. Proofpack is a later provenance bundle that references receipts, eval reports, decisions, and hashes. |

## Recommendation

Adopt the following boundary:

1. **A run receipt is a future evidence artifact, not a decision artifact.**
   It records what a run actually did after contract/flow authorization, before any gate decision.

2. **A denied transition should not write a receipt.**
   Denials remain event evidence. Receipt creation starts only when a real bounded run exists.

3. **Receipt and event log must stay distinct.**
   The event log is append-only lifecycle history. The receipt is a compact run artifact that references relevant events/artifacts rather than duplicating the full log.

4. **Receipt and eval report must stay distinct.**
   Eval report assesses. Receipt records execution. Neither decides.

5. **If receipt storage arrives later, it should be append-only and run-scoped under `.punk/runs/`.**
   Any `latest` or summary view must be derived.

6. **The next safe step is a design/spec-only boundary artifact, not implementation.**
   A spec should define minimum receipt fields, preconditions, and evidence-vs-decision boundaries before any `.punk/runs` activation.

## Adoption map

### Adopt

- receipt as future evidence artifact, not decision;
- receipt can exist before gate and should later be referenced by gate/proof;
- denied transition produces event evidence, not receipt;
- receipt should reference contract and run identity;
- future receipt storage should be append-only and run-scoped;
- receipt should later be hash-referenceable.

### Defer

- `.punk/runs` implementation;
- receipt hashing/provenance implementation details;
- actor/identity hardening;
- proofpack integration mechanics;
- exact receipt schema implementation;
- validator and conformance code.

### Park

- remote transparency logs;
- SLSA-level full provenance adoption;
- CI-specific export formats;
- OpenTelemetry export;
- Temporal-like durable execution runtime behavior.

### Avoid

- receipt as mutable `latest` truth;
- receipt as gate decision;
- receipt for denied transitions;
- storing secrets or raw provider payloads in receipts;
- making event log and receipt the same artifact.

## What stays out of scope

- Rust implementation;
- `.punk/runs`;
- receipt schema files;
- receipt validators;
- gate/proof implementation;
- proofpack manifests;
- CLI receipt commands;
- remote provenance/export systems.

## Impact on roadmap

- strengthens Phase 3 sequencing:
  - contract
  - approve
  - run
  - receipt
  - gate
  - proof
- reduces risk that receipt implementation will blur event, eval, and gate surfaces;
- suggests a spec-first detour before any runtime receipt work.

## Required evals

Before any future receipt implementation is promoted, add or require:

- receipt existence only after real run start/execution, not after denied transitions;
- receipt does not imply gate decision;
- receipt references contract/run identity correctly;
- receipt references produced artifact refs deterministically;
- proof/eval/gate links remain separate and inspectable.

## Required docs/ADRs/contracts

- next bounded goal should define `run-receipt-boundary.v0.1` as a design/spec artifact;
- if runtime receipt storage becomes canonical enough to affect broader architecture, consider ADR promotion after the boundary spec;
- later implementation goal should cite this research note and the boundary spec.

## Open questions

1. Should the first receipt boundary require both `started_at` and `completed_at`, or allow one timestamp plus outcome state?
2. Should the first receipt boundary require artifact hashes immediately, or make them optional pending proofpack work?
3. What is the minimum actor/process identity field if no stable identity model exists yet?
4. Should a receipt reference an event range, a primary event id, or both?
5. Should eval refs be omitted from the first receipt boundary if eval may be optional for some early runs?
