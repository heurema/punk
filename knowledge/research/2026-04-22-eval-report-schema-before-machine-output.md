---
kind: research-note
status: active
authority: advisory
created: 2026-04-22
related_goal: work/goals/goal_research_eval_report_schema_before_machine_output.md
related_report: work/reports/2026-04-22-eval-report-schema-research.md
---

# Eval Report Schema Before Machine Output

## Question

What boundary should Punk adopt before implementing machine-readable smoke eval reports?

More specifically:

- what should remain the canonical in-code shape for smoke eval results now;
- what must be decided before any stable machine-readable output exists;
- which external formats are useful precedents versus compatibility exports only;
- what should be adopted, deferred, parked, or avoided.

## Decision context

Punk now has three bounded eval surfaces in place:

1. a deterministic library-first smoke harness in `crates/punk-eval/`;
2. a narrow `punk eval run smoke` CLI wrapper in `crates/punk-cli/`;
3. an internal `SmokeEvalReport` with a human renderer.

The next tempting step would be machine-readable output. However, that step crosses multiple boundaries documented in repo truth:

- eval report shape;
- public CLI contract;
- possible runtime storage under `.punk/evals/`;
- future baseline and waiver semantics;
- eventual compatibility/export formats.

Because of that blast radius, this is a research-first decision rather than another small implementation diff.

Research is advisory.
It does not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Punk constraints extracted from repo docs

From `docs/product/RESEARCH-GATE.md`, `docs/product/EVAL-PLANE.md`, and `docs/product/ARCHITECTURE.md`:

- eval is a trust surface, but not the final decision writer;
- only `gate` writes final decisions;
- the first eval plane is local, deterministic-first, and repo-tracked where meaningful;
- runtime and derived storage under `.punk/` is phase-gated, not automatically active;
- public CLI contract changes require explicit Research Gate handling;
- new artifacts must not create a second source of truth;
- future machine output, storage, baseline, and waiver behavior should be inspectable and explicit;
- compatibility/export formats may exist later, but should not silently become canonical core truth.

## Sources reviewed

| Source | Type | Quality | Date checked | URL / ref | Why relevant |
|---|---|---|---|---|---|
| GitLab unit test reports | official docs | A (official) | 2026-04-22 | [docs.gitlab.com/ci/testing/unit_test_reports](https://docs.gitlab.com/ci/testing/unit_test_reports/) | Clear precedent for report artifact vs job exit status separation. |
| GitHub SARIF support for code scanning | official docs | A (official) | 2026-04-22 | [docs.github.com/.../sarif-support-for-code-scanning](https://docs.github.com/en/code-security/code-scanning/integrating-with-code-scanning/sarif-support-for-code-scanning) | Strong precedent for rules/results split and stable fingerprints across runs. |
| JSON Schema Draft 2020-12 overview | official docs | A (official) | 2026-04-22 | [json-schema.org/draft/2020-12](https://json-schema.org/draft/2020-12) | Current schema baseline and vocabulary split for future machine-readable contracts. |
| JSON Schema output schema | official docs | A (official) | 2026-04-22 | [json-schema.org/draft/2020-12/output/schema](https://json-schema.org/draft/2020-12/output/schema) | Useful precedent for `valid` plus structured `errors` / `annotations` output. |
| OpenTelemetry Logs Data Model | official spec docs | A (official) | 2026-04-22 | [opentelemetry.io/docs/specs/otel/logs/data-model](https://opentelemetry.io/docs/specs/otel/logs/data-model/) | Useful provenance/correlation field model for future eval outputs. |
| TAP 13 specification | official spec | A (official) | 2026-04-22 | [testanything.org/tap-version-13-specification.html](https://testanything.org/tap-version-13-specification.html) | Useful simple stdout protocol precedent for compatibility-only output. |
| `docs/product/RESEARCH-GATE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/RESEARCH-GATE.md` | Defines when schema/storage/public CLI work requires R2 research. |
| `docs/product/EVAL-PLANE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/EVAL-PLANE.md` | Documents future report, storage, baseline, and waiver direction. |
| `docs/product/ARCHITECTURE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/ARCHITECTURE.md` | Confirms assessment-vs-decision boundary and derived-view limits. |
| `work/reports/2026-04-22-smoke-eval-harness.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-smoke-eval-harness.md` | Captures current smoke harness scope and deferred items. |
| `work/reports/2026-04-22-smoke-eval-cli-command.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-smoke-eval-cli-command.md` | Confirms current CLI boundary and no-storage posture. |
| `work/reports/2026-04-22-smoke-eval-report-artifact-shape.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-smoke-eval-report-artifact-shape.md` | Confirms current internal report shape and deferred machine output. |

## Source quality table

| Source | Reliability | What we can safely use | Cautions |
|---|---|---|---|
| GitLab JUnit docs | High | JUnit-as-artifact, separate from job failure status | JUnit is a CI interchange format, not a canonical internal model for Punk. |
| GitHub SARIF docs | High | rules/results split, stable fingerprinting across runs | SARIF is shaped for code scanning; Punk smoke eval is broader than static-analysis alerts. |
| JSON Schema 2020-12 docs | High | future schema versioning and validation baseline | Introducing schema files too early would prematurely freeze the contract. |
| OTel Logs Data Model | High | provenance and correlation field inspiration | OTLP/export/trace integration is far beyond active-core scope. |
| TAP 13 spec | High | simple text protocol ideas for stdout compatibility | TAP is a transport/protocol style, not a good canonical core artifact model. |
| Repo canonical docs | High for Punk context | architecture, policy, roadmap, trust boundaries | They define Punk truth, not external interoperability constraints. |
| Repo work reports | Medium-high | what is actually implemented and deferred right now | They are execution evidence, not architectural approval by themselves. |

## Prior art / existing systems

### 1. JUnit XML as CI artifact precedent

GitLab's unit test report docs state that unit test reports:

- display test results directly in merge requests and pipeline details;
- require JUnit XML artifacts;
- do **not** affect job status by themselves;
- require the job script to exit non-zero if failure should fail the job.

This is directly useful for Punk:

- eval report artifact != final decision;
- command exit code != gate decision;
- CI/export format can exist without becoming product truth.

Takeaway:

- good precedent for separating artifact/reporting from command/process result;
- not a good canonical internal report model for Punk.

### 2. SARIF as rule/result/fingerprint precedent

GitHub's SARIF docs explicitly distinguish:

- rules: relatively stable tool/rule metadata;
- results: concrete findings for a specific run.

They also describe fingerprint usage across runs and note that stable paths/fingerprints prevent duplicate alerts.

Useful design constraints for Punk:

- eval suite/case definitions should stay separate from run results;
- future machine-readable results should preserve stable case identifiers across runs;
- if Punk later introduces cross-run matching, it should prefer stable identifiers/fingerprints rather than ad-hoc text matching.

Takeaway:

- adopt the conceptual split now;
- defer SARIF itself as a compatibility export only.

### 3. OpenTelemetry Logs as provenance/correlation precedent

OpenTelemetry's Logs Data Model emphasizes a common record shape and fields such as:

- `Timestamp`;
- `ObservedTimestamp`;
- `Resource`;
- `InstrumentationScope`;
- `Attributes`;
- `EventName`.

Useful implications for Punk:

- future eval machine output should preserve provenance and correlation refs;
- future report schemas likely need explicit produced/observed timing, source, and attributes-like extension fields;
- these ideas are useful even if Punk never adopts OTLP or tracing.

Takeaway:

- adopt provenance/correlation as a future design requirement;
- defer OTel export and tracing integration entirely.

### 4. JSON Schema 2020-12 as future validation baseline

JSON Schema Draft 2020-12 is the current general-purpose schema baseline and includes an output schema with concepts such as:

- `valid`;
- `errors`;
- `annotations`.

Useful implications for Punk:

- if Punk later introduces a stable machine-readable report, it should likely be versioned;
- JSON Schema 2020-12 is a good candidate for future validation/documentation;
- validation-style output suggests a useful split between pass/fail data and annotations/notes.

Takeaway:

- strong future candidate for schema validation baseline;
- do not introduce actual schema files yet.

### 5. TAP as simple stdout protocol precedent

TAP 13 is a simple text protocol with:

- plan (`1..N`);
- `ok` / `not ok` lines;
- optional YAML diagnostics;
- harness-on-stdout assumptions.

Useful implications for Punk:

- a text-first compatibility output is possible later;
- TAP may be useful as an adapter/export for CI or shell tooling;
- TAP should not define Punk's internal eval report model.

Takeaway:

- park as compatibility-only option, not canonical model.

## Failure modes found

1. **Ad-hoc JSON without schema/version**
   - creates unstable downstream consumers;
   - leaks implementation details into accidental API contracts.

2. **Report artifact mistaken for final decision**
   - violates the assessment-vs-decision boundary;
   - risks turning `punk eval run smoke` into pseudo-gate behavior.

3. **External export format becoming canonical internal truth**
   - JUnit, SARIF, or TAP could distort Punk's core model if adopted too early as the main representation.

4. **Rules/cases mixed with run results**
   - makes change tracking noisy;
   - prevents clean future baseline/fingerprint logic.

5. **Hidden runtime storage introduced before contract clarity**
   - `.punk/evals` could become accidental truth before schema/version/ownership is explicit.

6. **Premature baseline and waiver coupling**
   - baseline and waiver semantics depend on a stable report boundary;
   - introducing them before report-shape clarity will cause churn.

7. **Overfitting to CI interoperability**
   - JUnit/TAP are useful, but not a good reason to lock Punk's internal artifact model too early.

8. **Over-modeling provenance too early**
   - OTel-like richness is useful later, but overkill for current active-core smoke eval output.

## Options considered

### Option A — Stay internal-only indefinitely

Keep `SmokeEvalReport` as internal Rust shape plus human renderer only.

Pros:

- maximally flexible;
- no accidental external contract;
- preserves current bounded implementation posture.

Cons:

- blocks interoperability;
- makes future machine-readable evolution less deliberate if no proposal follows.

### Option B — Add machine-readable JSON immediately

Pros:

- quick interoperability win;
- easy scripting/CI ingestion.

Cons:

- premature contract freeze;
- likely ad-hoc unless schema/versioning is settled first;
- crosses R2 boundaries immediately.

### Option C — Define a schema-only proposal before implementation

Pros:

- preserves research-first discipline;
- clarifies versioning, required fields, and boundary notes before code;
- separates contract definition from storage/export/runtime concerns.

Cons:

- adds one more design step before machine output exists.

### Option D — Adopt JUnit XML as canonical machine output

Pros:

- existing CI ecosystem support.

Cons:

- too test-runner-oriented;
- poor fit for Punk-specific fields like boundary notes, deferred notes, provenance, and future eval artifacts.

### Option E — Adopt SARIF as canonical machine output

Pros:

- strong rules/results separation;
- fingerprint precedent;
- rich result metadata.

Cons:

- code-scanning orientation;
- heavy and semantically mismatched for the current smoke harness.

### Option F — Adopt TAP as canonical output

Pros:

- very simple;
- shell-friendly.

Cons:

- too shallow for Punk's future report needs;
- not a good canonical artifact model.

### Option G — Jump directly to `.punk/evals` report storage

Pros:

- creates concrete artifact paths early.

Cons:

- introduces storage before contract clarity;
- risks hidden truth and churn.

## Recommendation

Recommend **Option C**.

That means:

1. keep the current internal `SmokeEvalReport` as the canonical in-code shape for the local smoke harness;
2. do **not** implement machine-readable output yet;
3. define a schema-only proposal next, before any JSON output, storage, baseline, or waiver work;
4. treat JUnit, SARIF, and TAP as possible compatibility exports later, not as canonical internal models.

## Adoption map

### Adopt

- keep current internal `SmokeEvalReport` + human renderer as the canonical in-code shape for the local smoke harness;
- preserve the split between eval suite/case definitions and eval run results;
- preserve assessment-only semantics;
- require future machine-readable artifacts to carry explicit versioning;
- require stable case identifiers across runs;
- preserve boundary and deferred notes in future output shapes.

### Defer

- stable JSON output;
- JSON Schema 2020-12 implementation files;
- JUnit XML export;
- SARIF export;
- TAP export;
- `.punk/evals` storage;
- baseline and waiver semantics.

### Park

- OpenTelemetry export or OTLP integration;
- remote/SaaS eval integrations;
- code-scanning integration.

### Avoid

- eval report as gate decision;
- CLI smoke pass as project acceptance;
- hidden report storage;
- ad-hoc machine-readable JSON without schema/version;
- using CI/export format as canonical internal truth.

## What stays out of scope

- any code changes to `crates/punk-eval` or `crates/punk-cli`;
- any JSON output implementation;
- any schema file implementation;
- any `.punk/evals` runtime storage;
- any baseline or waiver implementation;
- any gate/proof integration;
- any export adapters.

## Impact on roadmap

Near-term roadmap implication:

- before machine-readable smoke eval output, insert one schema-only proposal step.

Phase implication:

- current active-core can continue using internal report shape + human renderer;
- machine-readable output should wait until schema/versioning is explicitly proposed;
- storage and waiver work remain later than schema definition.

## Required evals

If a future schema-only proposal is accepted, the later implementation should include at least:

- fixture tests for required fields in machine-readable output;
- version field presence checks;
- stable case-id checks across runs;
- tests that boundary/deferred notes are preserved where intended;
- tests that machine-readable output still does not imply final decision authority.

## Required docs / ADRs / contracts

Before stable machine-readable output lands, likely required:

- one schema-only proposal goal (`work/goals/goal_define_smoke_eval_report_schema_v0_1.md`);
- possible update to `docs/product/EVAL-PLANE.md` once schema direction is accepted;
- possibly an ADR if the schema/export boundary becomes a durable public contract.

No ADR is required for this research note itself.

## Open questions

1. Should future machine-readable output share the exact same source model as human rendering, or allow a separate export view?
2. Should future machine-readable reports include a run identifier before `.punk/evals` storage exists?
3. When Punk eventually wants cross-run matching, is stable `case_id` enough or will result fingerprints also be needed?
4. Should schema-only proposal start as Markdown/spec text first, or as a draft schema file under `evals/_schema/` without runtime use?
