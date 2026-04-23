# Gate decision boundary

Date: 2026-04-22
Status: advisory
Authority: research

## Question

What is a gate decision in Punk, why must only `gate` write it, how must it remain distinct from flow/event/receipt/eval/proof surfaces, and what is the smallest safe future `DecisionObject` boundary before any implementation starts?

## Decision context

Punk now has bounded contract, flow, event, run-receipt, and smoke-eval surfaces.

Those surfaces intentionally stop at assessment or evidence:

- contract approval authorizes bounded work;
- flow validates transitions and emits guard evidence;
- the event log records append-only lifecycle evidence;
- run receipts record bounded run evidence;
- eval reports assess behavior.

The next trust-surface risk is semantic collapse: a green eval, a receipt, or a transition result can start sounding like final closure even though Punk Laws reserve that authority for `gate` only.

This research asks what a future gate decision must be, what it may reference, and what it must never absorb from earlier evidence surfaces.

## Sources reviewed

Repo-tracked sources:

- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/eval-storage-boundary.v0.1.md`
- `evals/specs/eval-baseline-waiver-boundary.v0.1.md`
- `work/reports/2026-04-22-run-receipt-boundary-research.md`
- `work/reports/2026-04-22-run-receipt-boundary-v0-1.md`
- `work/reports/2026-04-22-run-receipt-minimal.md`
- `work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md`
- `work/reports/2026-04-22-run-receipt-smoke-eval.md`

External authoritative prior art:

- [Open Policy Agent decision logs](https://www.openpolicyagent.org/docs/latest/management-decision-logs/)
- [GitHub REST API: check runs](https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28)
- [GitHub Actions workflow artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [SLSA provenance v1.0](https://slsa.dev/spec/v1.0/provenance)
- [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/)

## Source quality table

| Source | Tier | Why it matters |
|---|---|---|
| `docs/product/PUNK-LAWS.md` | A | Canonical law that states every run writes a receipt and only `gate` writes the final decision. |
| `docs/product/ARCHITECTURE.md` | A | Canonical architecture boundary between evidence surfaces and `DecisionObject`. |
| `docs/product/CONTRACT-TRACKER.md` | A | Canonical object-model direction for `GateDecision` vs receipt/proof/event artifacts. |
| `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md` | A | Accepted ADR that separates policy assessment from gate decision authority. |
| `docs/adr/ADR-0012-minimal-proofpack-provenance.md` | A | Accepted ADR describing proofpack as provenance that references a gate decision instead of replacing it. |
| `docs/product/ROADMAP.md` | A | Canonical phase direction showing gate decision and proof requirements before later promotions. |
| OPA decision logs | A | Official vendor docs showing that policy results can have ids and logs without becoming the only project truth surface. |
| GitHub Checks API | A | Official API showing status/conclusion/output separation for check-like assessment surfaces. |
| GitHub Actions artifacts | A | Official docs showing artifacts as stored evidence separate from workflow conclusions. |
| SLSA provenance | A | Official provenance model showing subjects, build definition, and run details as evidence primitives, not closure authority. |
| OpenTelemetry logs data model | A | Official telemetry model for timestamps, attributes, and trace correlation; useful for evidence correlation only. |
| Recent work reports | B | Repo-tracked implementation evidence showing how receipt/eval/flow surfaces currently behave in Punk. |

## Prior art / existing systems

### Punk repo-tracked prior art

- Punk Laws already state that every run writes a receipt and only `gate` writes the final decision.
- Architecture already names `ModuleAssessment` as advisory evidence and `DecisionObject` as final.
- ADR-0013 already says modules and policies may assess, while `gate` remains the single final decision writer.
- ADR-0012 already says proofpack should reference the gate decision, making proof inspectable rather than authoritative.
- Run receipt boundary work already says receipts may exist before gate and must not imply final acceptance.

### OPA decision logs

OPA distinguishes policy query results from the wider system that consumes them. Its decision logs record fields such as queried path, input, result, and `decision_id` for audit and debugging. That is useful for Punk because it shows a strong pattern: assessment or policy surfaces can emit inspectable decision-shaped evidence, but the surrounding system still decides what that result means.

Adoptable insight:
- guard or policy outcomes may have ids, reasons, and inputs;
- those artifacts still should not become Punk's final closure artifact.

### GitHub check runs

GitHub check runs separate:

- `status`
- `conclusion`
- `output`
- `details_url`

This is useful because it shows a distinction between execution state, summarized assessment output, and a linked inspect view. It does not collapse logs, artifacts, and final repository merge policy into one object.

Adoptable insight:
- a future `DecisionObject` can reference evidence and carry a concise closure outcome without embedding all evidence bodies.

### GitHub Actions artifacts

Artifacts are uploaded, retained, and downloaded separately from workflow conclusions. The artifact layer preserves evidence without becoming the workflow's final governance surface.

Adoptable insight:
- proofpacks, eval reports, and receipts should stay linked evidence artifacts rather than turning the storage layer into the final authority.

### SLSA provenance

SLSA provenance distinguishes:

- subjects
- build definition
- resolved dependencies
- run details

This is useful because it keeps provenance about what happened and what produced an artifact separate from the higher-level policy decision about whether the result is acceptable.

Adoptable insight:
- future Punk decisions should reference hashes/refs of evidence artifacts, not absorb all provenance bodies into the decision itself.

### OpenTelemetry logs data model

OpenTelemetry logs define timestamps, observed timestamps, trace ids, attributes, and bodies for correlation. This is evidence and observability structure, not closure authority.

Adoptable insight:
- timestamps, event refs, and trace-like correlation are useful for later debugging, but logs remain evidence only.

## Failure modes

1. **Receipt becomes implicit acceptance.**
   A successful run receipt starts sounding like "the work passed" even though it only records execution evidence.

2. **Eval pass becomes implicit closure.**
   A green eval report is interpreted as final approval rather than advisory assessment.

3. **Event log becomes authority by accumulation.**
   An append-only event trail is mistaken for a closure artifact just because it records everything.

4. **Proofpack absorbs decision semantics.**
   A provenance bundle starts being treated as the thing that accepted work.

5. **Decision artifact duplicates raw evidence.**
   A future gate decision starts storing full receipts, full eval outputs, raw prompts, or full event logs, causing hidden truth duplication and privacy drift.

6. **Mutable latest view becomes canonical decision state.**
   A derived summary or latest pointer becomes the effective decision truth instead of an append-only decision artifact.

7. **Policy or module result bypasses gate.**
   A guard result or module outcome uses decision-like wording and silently becomes final in practice.

8. **Ordering ambiguity between decision and proof.**
   If gate and proof are not clearly separated, later implementation may create circular dependencies or ambiguous acceptance timing.

## Options considered

### Option A — No explicit gate decision artifact yet; infer closure from receipts/evals/events

Pros:
- no new artifact design work now;
- simplest short-term path.

Cons:
- violates Punk Laws in practice;
- collapses evidence into authority;
- makes future proof and acceptance semantics ambiguous;
- encourages hidden "latest" truth.

Assessment: avoid.

### Option B — Minimal `DecisionObject` as the only final closure artifact, referencing earlier evidence

Pros:
- matches Punk Laws and architecture;
- keeps contract/flow/event/receipt/eval surfaces non-authoritative;
- makes proofpack inspectable rather than authoritative;
- supports append-only storage later.

Cons:
- requires careful field and reference design before implementation;
- requires later eval coverage to ensure exclusivity.

Assessment: adopt.

### Option C — Let policy, eval, or receipt write acceptance directly

Pros:
- superficially simpler;
- reduces one explicit step.

Cons:
- breaks the assessment-vs-decision boundary;
- spreads closure authority across multiple surfaces;
- makes appeals, replay, and inspection harder;
- conflicts with ADR-0013 and Punk Laws.

Assessment: avoid.

### Option D — Let proofpack become the decision carrier

Pros:
- evidence and closure appear in one place;
- one artifact could be easier to inspect.

Cons:
- provenance bundle becomes authority surface;
- mixes evidence packaging with governance outcome;
- makes non-accept outcomes awkward;
- conflicts with ADR-0012, which treats proofpack as evidence linked to a decision.

Assessment: avoid.

### Option E — Keep gate local, but let the decision artifact reference evidence hashes/refs without owning evidence bodies

Pros:
- fits local-first trust model;
- keeps canonical decision small and inspectable;
- keeps storage and hash semantics open for later bounded work.

Cons:
- exact hash/ref rules still need later spec work;
- ordering with proofpack needs a clearer design step.

Assessment: adopt as part of Option B.

## Recommendation

### Core recommendation

Only `gate` should write the future final decision artifact.

Everything that precedes `gate` remains non-authoritative evidence:

- contract approval authorizes bounded work;
- flow validates transitions;
- the event log records append-only lifecycle evidence;
- run receipts record what a run did;
- eval reports assess behavior;
- proofpacks later make evidence inspectable.

None of those surfaces should close the loop.

### Why only `gate` writes the final decision

Because Punk needs exactly one closure authority.

If closure authority leaks into receipts, evals, policies, or proof artifacts, then:

- the project loses a stable answer to "what actually closed this work?";
- replay and inspection become ambiguous;
- storage and derived-view work can silently mutate truth;
- different surfaces can disagree about whether work is accepted.

A single gate decision artifact keeps closure explicit, searchable, and inspectable.

### What can precede gate and still remain non-authoritative

All of the following may precede gate and must remain evidence-only:

- contract approval;
- allowed or denied flow transitions;
- append-only event evidence;
- run receipts;
- eval reports;
- proof preparation or proofpack assembly;
- policy or guard results;
- module findings, warnings, or recommendations.

They may be necessary inputs.

They must not become final closure.

### What a future gate decision must reference

The minimum future `DecisionObject` should reference:

- `decision_id`;
- `contract_ref`;
- `contract_hash` or artifact-hash concept;
- one or more `run_receipt_refs` and later hashes when available;
- relevant `eval_report_refs` and later hashes when available;
- relevant `event_refs` or event range/root concept;
- `decision_outcome`;
- `reason_code` and human-readable rationale;
- `decided_at`;
- `guard_or_policy_set_ref` if gate evaluated a specific rule set;
- `proof_ref` or `proof_requirement_status`, when that surface exists;
- `boundary_notes`.

### What a gate decision must never absorb from earlier surfaces

A future gate decision should **reference** earlier evidence, not absorb it.

It must not inline or silently own:

- full event logs;
- full run receipts;
- full eval reports;
- raw prompts or provider payloads;
- mutable latest views;
- baseline or waiver state as implicit acceptance;
- raw secrets or environment payloads;
- external system truth that bypasses local evidence.

### How a gate decision differs from adjacent artifacts

#### Event evidence

- event evidence records lifecycle facts and guard outcomes;
- gate decision records final closure authority.

Event evidence can explain why something happened.

It does not decide whether the work is finally closed.

#### Run receipt

- run receipt records what a bounded run did;
- gate decision records whether the evidence is sufficient for final closure.

A receipt may exist before gate.

A receipt cannot imply acceptance.

#### Eval report

- eval report records assessment results;
- gate decision records the final governance outcome.

A green eval may support acceptance.

It must never be acceptance by itself.

#### Proofpack

- proofpack packages or references verifiable evidence;
- gate decision records the authority outcome over that evidence.

Proofpack supports inspectability.

It does not become the authority surface.

### Should proofpack include the gate decision, or should gate include proof refs, or both?

The safest future design is **both**, but asymmetrically:

- gate decision should reference proof refs or proof requirement status when proof exists or is required for the chosen outcome;
- proofpack should later include the gate decision id/hash because the proof story is incomplete without the closure artifact it supports.

This keeps the two artifacts separate while preserving a bidirectional inspectable link.

### Minimum future `DecisionObject` shape

A future minimal shape could look like this:

```yaml
schema_version: punk.gate-decision.v0.1
decision_id: decision_...
outcome: accept|reject
contract:
  ref: .punk/contracts/contract_...json
  hash: sha256:...
run_receipts:
  - ref: .punk/runs/run_...json
    hash: sha256:...
eval_reports:
  - ref: .punk/evals/smoke_...json
    hash: sha256:...
events:
  range: events-0001:120..148
  root_hash: sha256:...
proof:
  ref: .punk/proofs/proof_...json
  hash: sha256:...
reason:
  code: GATE_ACCEPTS_VERIFIED_SCOPE
  summary: bounded scope satisfied and linked evidence is complete
decided_at: 2026-04-22T12:00:00Z
boundary_notes:
  - only gate writes final decisions
  - referenced artifacts remain separate evidence surfaces
```

This is a boundary sketch, not an implementation contract.

Exact outcome vocabulary, hashing rules, and storage layout should remain deferred until the next design step.

## Adoption map

### Adopt

- gate decision remains the only final closure authority;
- prior surfaces stay evidence-only;
- future `DecisionObject` references contract, receipt, eval, event, and proof artifacts by ref/hash instead of absorbing them;
- decision storage later should be append-only and decision-scoped;
- derived latest or summary views must remain non-canonical.

### Defer

- gate implementation;
- proofpack implementation details;
- `.punk/decisions` storage layout implementation;
- exact hash normalization rules;
- actor identity and signing model;
- outcome vocabulary beyond the smallest useful set.

### Park

- remote policy engines as active closure authority;
- remote transparency logs;
- cloud decision stores;
- full supply-chain signing or attestation stacks.

### Avoid

- letting eval, receipt, event, or flow surfaces imply final acceptance;
- mutating `latest` as canonical decision truth;
- embedding raw evidence bodies into the decision object;
- treating proofpack as the closure artifact.

## What stays out of scope

This research does not define:

- gate Rust code;
- CLI gate commands;
- `.punk/decisions` implementation;
- proofpack implementation;
- decision schema validators;
- signature or transparency-log behavior;
- storage activation for decisions or proofs;
- product-doc changes.

## Impact on roadmap

This research strengthens Phase 3 and later phases by clarifying that:

- Phase 3 needs a gate decision artifact boundary before any implementation starts writing closure state;
- proofpack and decision remain separate artifacts with explicit links;
- later storage work for receipts, evals, decisions, and proofs must preserve canonical-vs-derived separation;
- future evals should verify exclusivity: only gate writes final closure artifacts.

## Required evals

Future implementation work should add eval coverage for at least:

- only `gate` can write a decision artifact;
- receipt, eval, and event artifacts never imply acceptance;
- decision refs to contract, receipt, eval, and proof artifacts remain inspectable;
- non-accept outcomes still remain explicit decision artifacts;
- mutable latest or summary views do not become canonical truth.

## Required docs/ADRs/contracts

Before implementation, the next bounded step should produce a design/spec artifact for the gate decision boundary.

A later implementation branch may then require:

- a gate decision boundary spec;
- possibly an ADR if decision storage or proof linkage semantics become canonical architecture;
- implementation goals that cite this research note and the later boundary spec;
- eval additions for decision exclusivity and proof linkage.

## Open questions

1. Should the first decision outcome vocabulary be only `accept|reject`, or does Punk need a third explicit closure state such as `defer`?
2. For accepting outcomes, must proof refs be mandatory from the first implementation, or can the first decision model carry an explicit proof obligation field?
3. Should the first decision artifact reference individual event ids, event ranges, or an event-root concept?
4. Does the first gate boundary spec need to settle proof ordering, or can it defer exact sequencing while still requiring explicit bidirectional refs later?
5. How much rule-set or policy-set identity should the first `DecisionObject` carry before policy surfaces become more explicit?
