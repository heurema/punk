# Evals

Status: active-core design direction; runner implementation remains phase-gated.

Initial direction:

- local first
- repo-stored
- deterministic first
- fixture-based
- Pydantic Evals-compatible later
- hard gates plus scorecard
- baseline regression blocks promotion unless explicitly waived

Do not add a capability without adding or updating an eval case, or recording an explicit no-eval rationale.

## What evals are allowed to decide

Evals can report evidence and block promotion when hard gates fail.

Evals cannot accept work. Only `gate` writes the final decision.

## Layout

```text
evals/
  README.md
  _schema/
    eval-case.v1.yaml
  _templates/
    eval-case.v1.yaml
  suites/
    smoke.yaml
    core.yaml
  cases/
    flow/
    contract/
    run/
    gate/
    proof/
    eval/
    project/
    modules/
```

## Migration note

`evals/_schema/eval-case.v1.yaml` is the target format for Phase 2 active-core cases, especially `smoke`.

Some older non-smoke sketches may still use a lighter format until the runner lands and the rest of the suite is migrated.

## Case categories

- `flow`: transition validity and event writing.
- `contract`: contract scope/schema/research refs.
- `run`: receipt requirements and run artifact checks.
- `gate`: decision exclusivity and gate semantics.
- `proof`: proofpack link/hash integrity.
- `eval`: runner/report/baseline/waiver behavior.
- `project`: project identity, public CLI exposure, memory roots.
- `modules`: future module conformance; parked unless roadmap phase promotes it.

## Hard gates vs scorecard

Hard gates are deterministic checks that block promotion.

Scorecard metrics are advisory unless a case marks them as `hard_gate: true` or a later ADR promotes them.

## Required Phase 2 smoke cases

The first smoke suite should cover at least:

1. cannot cut before approval
2. cannot proof before decision
3. every allowed transition writes an event
4. flow resumes from persisted state
5. contract scope is required before cut
6. run writes receipt before gate
7. only gate writes final decision
8. proof hashes contract, receipt, and decision
9. eval runner writes machine JSON and human Markdown reports
10. parked capability is not exposed in public CLI
11. baseline regression blocks promotion
12. waiver requires explicit reason and ledger entry

## Report outputs

Default runtime output:

```text
.punk/evals/runs/<run-id>/report.json
.punk/evals/runs/<run-id>/report.md
```

When evals are tied to dogfooded work, link the human report from `work/reports/`.

## Compatibility stance

Pydantic Evals compatibility is a design target, not a core dependency.

Mapping:

| Punk | Pydantic Evals analogue |
|---|---|
| suite | Dataset |
| case | Case |
| hard gate | deterministic evaluator |
| scorecard metric | metric/attribute |
| report JSON | evaluation report |

External frameworks may be used later for export, comparison, or non-core experiments, but they must not own truth for active-core decisions.
