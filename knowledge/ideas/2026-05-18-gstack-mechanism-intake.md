---
id: idea_2026_05_18_gstack_mechanism_intake
kind: idea
status: active
authority: advisory
owner: vitaly
created_at: 2026-05-18
review_after: 2026-06-18
related_research:
  - knowledge/research/2026-05-18-gstack-workflow-prior-art.md
related_docs:
  - docs/product/RUNNER-AIDS.md
  - docs/product/REVIEW-ASSESSMENT.md
  - docs/product/INSTRUCTION-SOURCES.md
related_evals:
  - evals/specs/runner-aid-boundary.v0.1.md
  - evals/specs/review-assessment-receipt.v0.1.md
  - evals/specs/contract-intake-questions.v0.1.md
  - evals/specs/docs-drift-assessment.v0.1.md
  - evals/specs/instruction-source-freshness.v0.1.md
---

# gstack mechanism intake

## Intent

Capture the useful mechanisms from `garrytan/gstack` without turning Punk into a
Claude-specific agent suite or a slash-command workflow clone.

Punk should learn from gstack's workflow discipline while keeping Punk's own
kernel laws:

```text
plot -> cut -> gate
```

Only `gate` writes final decisions. Proof comes before acceptance. Runner aids,
modules, tools, adapters, and reviewers can assess; they cannot decide.

## Extracted primitives

| Primitive | Punk interpretation | Intake outcome |
|---|---|---|
| Workflow stages | auxiliary lanes inside `plot / cut / gate` | adopt |
| Product forcing questions | contract-intake readiness checks | adopt |
| CEO/eng/design/DX/security/QA personas | advisory assessment kinds | adopt model, defer runtime |
| Code review / second opinion | review assessment receipt | adopt model, defer runtime |
| Browser QA | side-effect-heavy adapter evidence | park |
| Scope freeze / careful / guard | contract scope and side-effect preflight | adopt mechanism |
| Docs release pass | docs-drift assessment | adopt eval/spec |
| Generated skills | instruction source freshness and generated-view boundary | adopt boundary |
| Domain skills / learnings | executor memory with promotion path | park runtime |
| Cross-agent browser pairing | coordinated adapters / parallel flows | park |
| Deploy/canary | external side-effect adapters | park |

## Adopt

Adopt now as docs/eval/spec direction:

1. Runner aids are non-authoritative.
2. Review assessments are advisory and receipt-shaped.
3. Contract-intake questions improve `plot` before `cut`.
4. Docs drift checks are evidence, not automatic canonical rewrites.
5. Instruction source pages and generated instruction views stay distinct.
6. Scope guards become contract/eval/receipt/gate discipline.

## Defer

Defer until later roadmap phases or module-host readiness:

- DevPunk review module;
- security assessor module;
- cross-model second-opinion runner;
- generated runner-aid renderer;
- local runner-aid registry;
- review dashboard;
- contract-derived executor briefs.

Trigger condition: contract loop, receipts, gate decision, proofpack, and
assessment receipt boundaries have at least one green eval path.

## Park

Park until adapter and side-effect policy are promoted:

- browser daemon;
- cookie import;
- authenticated browser QA;
- pair-agent or shared browser control;
- deploy, canary, release automation;
- cross-project memory sync;
- parallel sprint orchestration;
- remote telemetry or hosted dashboards.

Trigger condition: adapter boundary, side-effect receipt policy, gate inputs,
and proof requirements are defined and covered by evals.

## Avoid

Avoid permanently unless Punk laws change:

- AI or slash command writes final decision;
- runner aid output becomes proof;
- hidden memory becomes project truth;
- provider-specific workflow becomes active-core architecture;
- generated docs/views own canonical truth;
- hidden network calls or analytics;
- browser/deploy side effects without explicit receipts.

## First useful slice

Keep this slice documentation-only and eval/spec-only:

```text
knowledge/research/2026-05-18-gstack-workflow-prior-art.md
docs/product/RUNNER-AIDS.md
docs/product/REVIEW-ASSESSMENT.md
docs/product/INSTRUCTION-SOURCES.md
evals/specs/runner-aid-boundary.v0.1.md
evals/specs/review-assessment-receipt.v0.1.md
evals/specs/contract-intake-questions.v0.1.md
evals/specs/docs-drift-assessment.v0.1.md
evals/specs/instruction-source-freshness.v0.1.md
```

No Rust code, runtime storage, CLI behavior, browser, module invocation, adapter
invocation, external side effect, gate writer, proofpack writer, or acceptance
claim belongs in this slice.
