---
kind: research-note
status: active
authority: advisory
created: 2026-04-22
related_goal: work/goals/goal_research_eval_storage_and_baseline_boundary.md
related_report: work/reports/2026-04-22-eval-storage-baseline-boundary-research.md
---

# Eval Storage, Baseline, and Waiver Boundary

## Question

What boundary should Punk adopt before implementing `.punk/evals` storage, baseline comparison, and waiver behavior?

More specifically:

- when should local eval report artifacts start being written under `.punk/evals/`;
- what is canonical versus derived in future eval storage;
- how should stored eval reports relate to the event log, future gate decisions, and future proofpacks;
- what is the minimum useful baseline comparison model;
- what is the minimum acceptable waiver model;
- which next step is safe without accidentally turning eval into hidden authority.

## Decision context

Punk already has the first bounded eval chain in place:

1. deterministic library-first smoke harness;
2. narrow CLI wrapper;
3. internal `SmokeEvalReport` shape;
4. schema-only `smoke-eval-report.v0.1` proposal;
5. opt-in JSON output matching the proposed shape.

The next tempting steps are:

- writing reports to `.punk/evals`;
- keeping a latest report or report history;
- comparing runs to a baseline;
- adding waiver records;
- later feeding report hashes into `gate` / proofpack provenance.

These steps cross multiple canonical boundaries:

- storage model;
- eval policy;
- assessment-vs-decision semantics;
- future proofpack provenance shape;
- project memory and inspectable-view boundaries.

Because of that blast radius, this is research-first R2 work, not another small implementation diff.

Research is advisory.
It does not become product truth until promoted through ADR, roadmap, contract, implementation, eval, and proof.

## Punk constraints extracted from repo docs

From `docs/product/EVAL-PLANE.md`, `docs/product/ARCHITECTURE.md`, `docs/product/PUNK-LAWS.md`, `docs/product/PROJECT-MEMORY.md`, and `docs/product/RESEARCH-GATE.md`:

- eval assesses, but does not write final decisions;
- only `gate` writes final decisions;
- `.punk/` is runtime/derived space, not a second source of truth;
- inspectable views must be reconstructable from canonical evidence;
- future `.punk/evals` artifacts may exist, but they must not silently become hidden authority;
- baseline regressions and waivers are part of the target eval plane, but remain phase-gated;
- waivers must be explicit, inspectable, and linked to reason and follow-up;
- proofpacks may reference eval artifacts later, but proof remains after decision.

## Sources reviewed

| Source | Type | Quality | Date checked | URL / ref | Why relevant |
|---|---|---|---|---|---|
| GitHub Actions workflow artifacts | official docs | A (official) | 2026-04-22 | [docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts) | Shows artifacts as stored workflow outputs with retention, sharing, and download semantics rather than canonical decision truth. |
| GitHub Checks API | official docs | A (official) | 2026-04-22 | [docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#create-a-check-run](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28#create-a-check-run) | Clear precedent for `status`, `conclusion`, `external_id`, and report summary fields separate from artifact storage. |
| GitLab unit test reports | official docs | A (official) | 2026-04-22 | [docs.gitlab.com/ci/testing/unit_test_reports](https://docs.gitlab.com/ci/testing/unit_test_reports/) | Strong precedent for report artifacts that do not decide job status and for explicit branch-to-branch comparison data. |
| Bazel Test Encyclopedia | official docs | A (official) | 2026-04-22 | [bazel.build/reference/test-encyclopedia](https://bazel.build/reference/test-encyclopedia) | Useful for hermetic test expectations, JUnit-like XML output, and undeclared outputs stored under test logs. |
| Bazel output directory layout | official docs | A (official) | 2026-04-22 | [bazel.build/remote/output-directories](https://bazel.build/remote/output-directories) | Good precedent for per-run output layouts that avoid collisions and support rebuildable derived artifacts. |
| cargo-nextest JUnit support | official docs | A (official) | 2026-04-22 | [nexte.st/docs/machine-readable/junit](https://nexte.st/docs/machine-readable/junit/) | Confirms that machine-readable exports are useful compatibility surfaces, not necessarily canonical internal models. |
| OpenTelemetry Logs Data Model | official spec docs | A (official) | 2026-04-22 | [opentelemetry.io/docs/specs/otel/logs/data-model](https://opentelemetry.io/docs/specs/otel/logs/data-model/) | Useful future reference for provenance/correlation fields if eval artifacts are exported later. |
| SLSA provenance v1.0 | official spec docs | A (official) | 2026-04-22 | [slsa.dev/spec/v1.0/provenance](https://slsa.dev/spec/v1.0/provenance) | Useful future reference for how proof/provenance can reference subjects and dependencies without changing current eval storage semantics. |
| `docs/product/EVAL-PLANE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/EVAL-PLANE.md` | Defines current target direction for eval reports, baselines, waivers, and runtime layout. |
| `docs/product/ARCHITECTURE.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/ARCHITECTURE.md` | Defines canonical-vs-derived evidence and assessment-vs-decision boundaries. |
| `docs/product/PUNK-LAWS.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/PUNK-LAWS.md` | Confirms that only `gate` writes final acceptance and proof follows decision. |
| `docs/product/PROJECT-MEMORY.md` | repo canonical | project canonical | 2026-04-22 | `docs/product/PROJECT-MEMORY.md` | Useful for repo-tracked truth vs derived runtime-memory boundaries. |
| `work/reports/2026-04-22-smoke-eval-harness.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-smoke-eval-harness.md` | Captures the original no-storage, no-baseline scope. |
| `work/reports/2026-04-22-smoke-eval-json-output-v0-1.md` | repo evidence | repo evidence | 2026-04-22 | `work/reports/2026-04-22-smoke-eval-json-output-v0-1.md` | Confirms current JSON output boundary and what remains deferred. |

## Source quality table

| Source | Reliability | What we can safely use | Cautions |
|---|---|---|---|
| GitHub artifacts docs | High | retention, upload/download, artifact lifecycle | Artifacts are workflow outputs, not product truth or decision records. |
| GitHub Checks API docs | High | status/conclusion separation, external ids, output summary/text, annotations | Check runs are CI/integration surfaces, not canonical local storage models. |
| GitLab unit report docs | High | report artifacts do not fail jobs alone, branch comparison needs explicit baseline data | JUnit is a compatibility format, not Punk's internal model. |
| Bazel test docs | High | hermeticity, per-run logs/outputs, generated XML/test artifacts | Bazel is a build/test system; its storage and cache semantics do not map 1:1 to Punk. |
| Bazel output layout docs | High | non-colliding per-run output layout requirements | Still infrastructure-oriented, not product trust-policy guidance by itself. |
| cargo-nextest docs | High | export formats as useful machine-readable compatibility surfaces | Export support does not answer canonical storage or waiver policy. |
| OTel logs docs | High | provenance and correlation field inspiration | Export and telemetry systems are parked for Punk today. |
| SLSA provenance docs | High | future subject/hash/provenance linking ideas | This is future proofpack territory, not current eval storage implementation. |
| Repo canonical docs | High for Punk context | laws, architecture boundaries, eval/storage policy direction | They define project truth but not all external interoperability tradeoffs. |
| Repo work reports | Medium-high | current implemented scope and repeated human corrections | They are execution evidence, not architecture approval on their own. |

## Prior art / existing systems

### 1. GitHub artifacts and check runs separate artifact storage from run state

GitHub Actions artifacts are uploaded, retained, downloaded, and shared as workflow outputs. The docs emphasize artifact upload/download and configurable retention periods, which makes them clearly derived run outputs rather than canonical system truth.

GitHub Checks add another useful split:

- `status` vs `conclusion`;
- `external_id` for correlation;
- structured `output` with `title`, `summary`, `text`, and annotations.

Takeaway for Punk:

- future `.punk/evals` artifacts should be run outputs, not decision objects;
- run storage should support explicit correlation ids / hashes;
- report summary surfaces and stored artifacts are related but not the same thing.

### 2. GitLab unit test reports show that baseline comparison needs explicit comparison data

GitLab unit test reports:

- parse JUnit XML artifacts;
- do not affect job status by themselves;
- compare source branch results against target branch results when baseline data exists.

GitLab also documents that missing baseline data on the target branch prevents useful comparison.

Takeaway for Punk:

- baseline comparison needs an explicit reference run, not a hidden mutable “latest”; 
- baseline should not be inferred from whatever file happens to be newest;
- report artifacts and final pass/fail decisions remain separate concerns.

### 3. Bazel treats test outputs as per-run artifacts and values hermeticity

Bazel's Test Encyclopedia emphasizes that tests should confirm properties of source-controlled inputs and that non-hermetic tests reduce test value.

Bazel also provides per-run outputs:

- XML output paths;
- test logs;
- zipped undeclared outputs under `bazel-testlogs`.

Takeaway for Punk:

- if Punk later stores eval artifacts, they should be run-scoped and append-only, not mutable “latest only” state;
- baseline comparison should consider only deterministic fields;
- stored artifacts should preserve raw run evidence rather than overwrite prior history.

### 4. cargo-nextest treats JUnit as a machine-readable export, not a canonical internal model

cargo-nextest supports JUnit output as a machine-readable format understood by external tools.

Takeaway for Punk:

- machine-readable output and storage are different questions;
- compatibility exports are useful later, but should not dictate canonical local evidence layout.

### 5. OTel and SLSA are better future references than current implementation requirements

OpenTelemetry Logs provide useful ideas around:

- timestamps;
- event names;
- attributes;
- correlation and source fields.

SLSA provenance provides useful future ideas around:

- `subject` identifiers;
- resolved dependencies;
- ignoring unknown fields in evolving attestations;
- provenance as separate evidence linked to produced artifacts.

Takeaway for Punk:

- future proofpacks may reference eval report hashes and metadata;
- current eval storage should not try to become a full telemetry or provenance system.

## Failure modes

1. **Mutable `latest.json` becomes hidden truth**
   - a latest-only file makes it easy to overwrite history and creates accidental hidden authority.

2. **Stored eval report mistaken for final decision**
   - this violates Punk's assessment-vs-decision boundary and weakens `gate` exclusivity.

3. **Baseline inferred from newest run instead of explicit reference**
   - comparisons become nondeterministic and hard to inspect.

4. **Waiver added before baseline and hard-gate semantics are stable**
   - waivers become a policy escape hatch without clear scope or expiry.

5. **Storage introduced without run identity and hashes**
   - report history becomes difficult to correlate with CLI output, work reports, future events, and future proofpacks.

6. **Report persistence introduced without append-only posture**
   - stored evals can drift into mutable state that is neither canonical nor clearly derived.

7. **CI export format drives core storage design**
   - JUnit or CI check models distort Punk's local-first evidence model if adopted as canonical too early.

8. **Baseline pass treated as project acceptance**
   - this would incorrectly promote eval output into decision authority.

9. **Waiver record lacks owner/scope/expiry**
   - unresolved failures can silently persist as unbounded exceptions.

10. **Proof integration attempted before storage boundary is explicit**
   - proofpacks could end up referencing mutable report views instead of stable artifacts/hashes.

## Options considered

### Option A — No storage yet

Keep smoke eval output CLI-only for now.

Pros:

- avoids premature storage design;
- preserves current bounded implementation;
- no hidden mutable truth.

Cons:

- no durable local run evidence;
- no stable anchor for later baseline comparisons;
- no later report hash to reference from gate/proof.

### Option B — Store latest report only

Store only the most recent report under `.punk/evals/latest.*`.

Pros:

- simple to implement;
- easy to inspect manually.

Cons:

- mutable “latest only” state becomes hidden truth;
- destroys history or requires overwriting semantics;
- makes baseline selection implicit instead of explicit.

Assessment:

- avoid.

### Option C — Append-only `.punk/evals` report artifacts

Store each run as its own local artifact later, for example under `.punk/evals/runs/<run-id>/`.

Pros:

- preserves history;
- supports explicit correlation to CLI runs, work reports, future event refs, and future proofpacks;
- keeps storage append-only and inspectable.

Cons:

- requires a prior design for run identity, directory layout, and hash/reference fields;
- can still create hidden authority if introduced without a clear canonical-vs-derived contract.

Assessment:

- future candidate; not current implementation step.

### Option D — Baseline comparison

Compare a current run against a chosen reference run.

Pros:

- supports regression blocking later;
- aligns with Phase 2 roadmap direction.

Cons:

- only meaningful after stored run artifacts and explicit reference selection exist;
- dangerous if comparison includes nondeterministic fields or implicit baselines.

Minimum viable future model:

- explicit baseline reference;
- compare deterministic fields only:
  - `schema_version`
  - `suite_id`
  - per-case `status`
  - bounded summary fields if deterministic
  - runner/report version refs if introduced.

Assessment:

- defer until storage boundary is defined.

### Option E — Waiver ledger

Allow explicit override of a failing eval requirement.

Pros:

- necessary for future hard-gate operations where temporary exceptions must stay inspectable.

Cons:

- extremely dangerous before hard gates and baseline semantics are stable;
- easy to turn into silent policy debt.

Minimum viable future record should include:

- waiver id;
- suite id and case/assertion scope;
- linked report ref or report hash;
- reason;
- owner/approver;
- created_at;
- expiry or revisit trigger;
- linked follow-up goal.

Assessment:

- defer.

### Option F — Proofpack integration later

Let future gate/proof objects reference eval report hashes or ids.

Pros:

- keeps eval as advisory evidence while allowing proof provenance later;
- aligns with Punk's proof-after-decision posture.

Cons:

- only useful after stored run artifacts, hashes, and explicit decision objects exist.

Assessment:

- defer to later proofpack work.

## Recommendation

### Recommended now

1. **Do not implement `.punk/evals` yet.**
   The project has JSON output and a schema proposal, but not yet a settled storage boundary, run identity model, or baseline/waiver semantics.

2. **Keep eval output assessment-only.**
   Human and JSON smoke reports remain run outputs, not decisions.

3. **If storage is introduced later, prefer append-only run-scoped artifacts.**
   The first safe storage design should be local, append-only, run-scoped, and explicit about schema version and report hash.

4. **Require explicit baseline references later.**
   Do not infer a baseline from “latest” state. Baseline comparison should reference a chosen prior run or chosen baseline artifact.

5. **Do not add waivers until hard-gate and baseline semantics are stable.**
   Waivers should arrive only after the system can state exactly what is being waived and why.

6. **Treat proof integration as future reference-by-hash work.**
   Gate/proof should later reference eval report ids/hashes, not mutable views or summaries.

### Safe next step

The safest next bounded step is **design/spec-only** work:

- define the eval storage boundary;
- define canonical vs derived artifacts;
- define run identity expectations;
- define what is intentionally deferred for baseline and waivers.

That means the next goal should be a design/spec goal, not an implementation goal.

## Adoption map

### Adopt

- keep current CLI/human/JSON smoke output as assessment-only;
- require explicit schema/version on any future stored report artifact;
- prefer append-only, run-scoped eval artifacts if storage is later added;
- require explicit reference selection for future baselines;
- keep future proof integration as reference-by-id/hash, not mutable latest-state linkage.

### Defer

- `.punk/evals` storage implementation;
- baseline comparison implementation;
- waiver ledger implementation;
- proofpack integration over eval artifacts;
- report-history UI or richer inspect views over stored eval runs.

### Park

- remote eval stores;
- dashboards;
- CI-hosted report services as canonical storage;
- OpenTelemetry export;
- JUnit/SARIF/TAP export adapters as anything more than compatibility surfaces.

### Avoid

- mutable `latest` report as truth;
- baseline pass interpreted as acceptance;
- waiver records without explicit reason/owner/scope/expiry;
- hidden storage outside the repo/runtime evidence model;
- letting CI export formats define Punk's internal storage model.

## What stays out of scope

- Rust implementation;
- `.punk/evals` directories or files;
- baseline comparison code;
- waiver files or waiver ledger;
- report persistence model implementation;
- gate/proof implementation;
- export adapters;
- dashboarding or remote services.

## Impact on roadmap

This research tightens Phase 2 sequencing:

1. smoke eval harness;
2. CLI wrapper;
3. internal report shape;
4. schema-only proposal;
5. opt-in JSON output;
6. **storage-boundary design/spec work**;
7. only then consider `.punk/evals` implementation;
8. baseline and waiver work remain behind storage-boundary clarification.

This keeps the roadmap aligned with Punk Laws by preventing eval storage from becoming hidden decision authority.

## Required evals

When storage is eventually implemented, add deterministic evals for:

- append-only eval run artifact creation;
- no overwrite of prior run artifacts;
- explicit run id or report id presence;
- report hash/reference stability;
- stored eval artifact remains assessment-only;
- baseline comparison uses explicit reference, not implicit latest;
- waiver records require reason, scope, owner, and expiry;
- gate/proof may reference eval artifacts but eval artifacts never become final decision objects.

## Required docs/ADRs/contracts

Before storage implementation, likely add:

- a design/spec artifact for eval storage boundary v0.1;
- possibly an ADR only if the storage design materially changes canonical storage or runtime evidence architecture;
- later contract/goal refs for baseline and waiver semantics once the storage boundary is fixed.

## Open questions

1. Should future stored eval runs have a separate `run_id` independent of event ids and decision ids, or should they derive from a shared run identity scheme?
2. Should the first stored artifact include only JSON, or JSON plus human Markdown together?
3. Should report hashes be embedded into future event references when storage lands, or only linked from gate/proof objects?
4. Which deterministic fields belong in the first baseline comparison beyond case pass/fail status?
5. Should waivers attach to suite/case/assertion scope, or only to specific report hashes?
