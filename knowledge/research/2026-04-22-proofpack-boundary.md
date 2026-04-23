# Proofpack boundary

Date: 2026-04-22
Status: advisory
Authority: research

## Question

What is a proofpack in Punk, how must it remain distinct from event evidence, run receipts, eval reports, and gate decisions, and what is the smallest safe boundary before any proofpack or `.punk/proofs` implementation begins?

## Decision context

Punk now has bounded evidence and authority surfaces:

- contract authorizes bounded work;
- flow and event log record lifecycle evidence;
- run receipt records bounded run evidence;
- eval report records assessment evidence;
- gate decision boundary now fixes `gate` as the only future closure authority.

The next trust-surface risk is proof confusion:

- a proofpack could become a second decision surface;
- a proofpack could duplicate too much raw evidence and become hidden storage truth;
- a proofpack could appear too early and start competing with receipt or event semantics;
- decision and proof sequencing could become ambiguous.

This research asks what a future proofpack must be, what it should contain directly, what it should reference instead of absorbing, and which step is safe before any implementation starts.

## Sources reviewed

Repo-tracked sources:

- `docs/product/PUNK-LAWS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/RESEARCH-GATE.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/CONTRACT-TRACKER.md`
- `docs/adr/ADR-0012-minimal-proofpack-provenance.md`
- `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md`
- `evals/specs/run-receipt-boundary.v0.1.md`
- `evals/specs/gate-decision-boundary.v0.1.md`
- `work/reports/2026-04-22-run-receipt-boundary-research.md`
- `work/reports/2026-04-22-run-receipt-boundary-v0-1.md`
- `work/reports/2026-04-22-run-receipt-minimal.md`
- `work/reports/2026-04-22-connect-run-receipt-to-contract-flow.md`
- `work/reports/2026-04-22-run-receipt-smoke-eval.md`
- `work/reports/2026-04-22-gate-decision-boundary-research.md`
- `work/reports/2026-04-22-gate-decision-boundary-v0-1.md`

External authoritative prior art:

- [SLSA provenance v1.0](https://slsa.dev/spec/v1.0/provenance)
- [in-toto Statement v1](https://github.com/in-toto/attestation/blob/main/spec/v1/statement.md)
- [GitHub Actions workflow artifacts](https://docs.github.com/en/actions/using-workflows/storing-workflow-data-as-artifacts)
- [Open Policy Agent decision logs](https://www.openpolicyagent.org/docs/latest/management-decision-logs/)
- [OpenTelemetry Logs Data Model](https://opentelemetry.io/docs/specs/otel/logs/data-model/)

## Source quality table

| Source | Tier | Why it matters |
|---|---|---|
| `docs/product/PUNK-LAWS.md` | A | Canonical law that says only `gate` writes the final decision and proof is required before acceptance. |
| `docs/product/ARCHITECTURE.md` | A | Canonical architecture boundary that places `Proofpack` after `DecisionObject` while keeping it separate from decision authority. |
| `docs/product/CONTRACT-TRACKER.md` | A | Canonical direction for proofpack as inspectable evidence that makes gate decision understandable rather than authoritative by itself. |
| `docs/adr/ADR-0012-minimal-proofpack-provenance.md` | A | Accepted ADR describing the minimal proofpack manifest and its required refs/hashes. |
| `docs/adr/ADR-0013-policy-assessment-gate-decision-boundary.md` | A | Accepted ADR preserving the assessment-vs-decision split that proofpack must not violate. |
| `docs/product/ROADMAP.md` | A | Canonical phase direction that requires proof link/hash integrity and gate-decision exclusivity. |
| SLSA provenance v1.0 | A | Official provenance model showing subject, predicate, run details, and digests as provenance structure rather than closure authority. |
| in-toto Statement v1 | A | Official attestation model showing immutable subjects and typed predicates as a bundle over evidence, not a decision surface. |
| GitHub Actions artifacts | A | Official artifact model showing stored evidence plus SHA256 digest verification separate from workflow conclusions. |
| OPA decision logs | A | Official policy-log model useful as contrast: decision-shaped records can remain evidence/audit surfaces instead of owning project closure. |
| OpenTelemetry Logs Data Model | A | Official event/log correlation model showing timestamps and attributes as evidence structure only. |
| Recent repo work reports | B | Repo-tracked implementation evidence showing current receipt, eval, and gate-boundary behavior in Punk. |

## Prior art / existing systems

### Punk repo-tracked prior art

- Punk Laws require proof before acceptance and reserve final decision writing for `gate`.
- Architecture treats `DecisionObject` as final and `Proofpack` as a later separate artifact.
- ADR-0012 already says proofpack should reference contract, receipt, eval, decision, artifacts, events, and rules.
- ADR-0013 already says assessments stay separate from decisions, which means proofpack cannot become final authority just by bundling evidence.
- Run receipt and gate decision boundary work already separate pre-gate evidence from final closure authority.

### SLSA provenance

SLSA provenance models provenance as structured references over build definition, run details, subjects, and digests. It is useful because it keeps provenance about how an artifact was produced separate from the later policy conclusion about whether the artifact is acceptable.

Adoptable insight:
- proofpack should look like provenance over immutable refs and digests;
- provenance structure should not become the closure authority by itself.

### in-toto Statement

The in-toto Statement is a typed attestation envelope that binds a predicate to immutable `subject` artifacts identified by digest. This is useful because it shows how a higher-level evidence bundle can stay reference-oriented and typed without absorbing the full underlying artifacts.

Adoptable insight:
- proofpack should be a typed evidence bundle over referenced artifacts;
- subject identity should later depend on stable refs and hashes.

### GitHub Actions artifacts

GitHub Actions artifacts are uploaded, retained, downloaded, and digest-checked separately from workflow conclusions. Evidence storage and workflow conclusion remain distinct.

Adoptable insight:
- proofpack should link to evidence artifacts and hashes instead of becoming the workflow conclusion itself;
- digest verification belongs to evidence integrity, not closure authority.

### OPA decision logs

OPA decision logs show that decision-shaped records can still be audit/log surfaces instead of the whole system's only truth. This is useful as contrast: Punk should not let proofpack become an implicit decision log that replaces explicit gate closure.

Adoptable insight:
- proofpack may later capture evidence about what was checked;
- it must not replace the explicit gate decision artifact.

### OpenTelemetry logs data model

OpenTelemetry logs define timestamps, trace context, and attributes for correlation. This is useful only as evidence/correlation structure.

Adoptable insight:
- proofpack may later reference timestamps, event ranges, or correlation metadata;
- such correlation remains evidence-only and must not become decision semantics.

## Failure modes

1. **Proofpack becomes the decision artifact.**
   A provenance bundle starts being treated as the thing that accepted or rejected work.

2. **Proofpack duplicates all evidence bodies.**
   The proof artifact absorbs full receipts, full evals, raw events, raw prompts, or provider payloads, turning itself into hidden storage truth.

3. **Proofpack exists too early and competes with receipt semantics.**
   A pre-gate proof artifact starts overlapping with run receipts or guard evidence.

4. **Decision/proof ordering becomes ambiguous.**
   It becomes unclear whether gate must reference proof, proof must reference gate, or both, and in what order artifacts should be finalized.

5. **Proofpack becomes narrative-only.**
   The bundle contains summaries without stable refs or hashes, making inspection theatrical instead of verifiable.

6. **Mutable latest proof view becomes canonical truth.**
   A derived summary or mutable pointer becomes the effective source of truth instead of append-only proof artifacts.

7. **Proofpack silently imports sensitive content.**
   Raw user payloads, secrets, provider responses, or environment values leak into proof artifacts.

8. **Proofpack bypasses local-first trust boundaries.**
   Remote attestations, transparency logs, or cloud provenance become the effective canonical proof surface too early.

## Options considered

### Option A — No distinct proofpack artifact; rely on decision + receipt + eval refs only

Pros:
- simplest short-term implementation shape;
- fewer artifact types.

Cons:
- loses a dedicated inspectable provenance bundle;
- weakens the `proof comes before acceptance` law in practice;
- makes artifact integrity and evidence-link checking harder;
- conflicts with ADR-0012 and roadmap eval directions.

Assessment: avoid.

### Option B — Minimal proofpack as a separate evidence bundle that references decision and other evidence by ref/hash

Pros:
- matches ADR-0012 and architecture direction;
- keeps decision and proof separate;
- preserves inspectability without duplicating evidence bodies;
- supports later hash integrity evals.

Cons:
- exact sequencing and hash normalization still need a spec step;
- requires discipline to keep proofpack narrow.

Assessment: adopt.

### Option C — Proofpack as a decision artifact that closes the loop by itself

Pros:
- one artifact to inspect;
- superficially simpler mental model.

Cons:
- violates the gate-decision boundary;
- collapses evidence and authority;
- conflicts with Punk Laws, architecture, and ADR-0013.

Assessment: avoid.

### Option D — Draft proofpack before gate plus final proofpack after gate

Pros:
- may help internal assembly before closure;
- could model pre-decision evidence completeness.

Cons:
- introduces two proof semantics too early;
- risks overlap with receipt and eval artifacts;
- complicates the first boundary without implementation pressure.

Assessment: defer. Keep any earlier notion out of scope for now.

### Option E — Remote attestation / transparency-log-first proof model

Pros:
- stronger external provenance story later;
- useful for publishing or release flows.

Cons:
- premature external trust dependency;
- violates local-first sequencing for current phase;
- creates storage/export pressure too early.

Assessment: park.

## Recommendation

### Core recommendation

Proofpack should remain a separate evidence bundle.

It is not a decision artifact.

It is not a receipt.

It is not an eval report.

It is not the event log.

Its job is to make a future gate decision inspectable by linking stable evidence refs and hashes across the artifacts that matter.

### Is proofpack an evidence bundle, a decision artifact, or both?

Proofpack should be an evidence bundle only.

Gate decision remains closure authority.

Proofpack explains and links evidence around that closure; it does not replace the closure artifact.

### What proofpack should include directly

A future minimal proofpack should include directly:

- `proof_id`;
- `project_id` or local project identity concept;
- `created_at` or manifest timestamp;
- explicit boundary or privacy notes;
- typed sections for referenced evidence groups;
- verification outcome summary for hash/ref consistency, if that becomes part of the first spec.

### What proofpack should reference rather than absorb

Proofpack should reference rather than duplicate:

- contract artifacts;
- run receipt artifacts;
- eval report artifacts;
- gate decision artifacts;
- event ranges or event-root concepts;
- produced artifact refs;
- rule or guard-set refs.

It should not absorb full raw receipts, full eval bodies, full event logs, raw prompts, raw provider payloads, or mutable latest views.

### Should proofpack exist only after gate, or should any earlier notion remain out of scope?

The safest boundary is:

- canonical proofpack exists after `gate` and references the gate decision;
- any earlier draft or pre-gate notion remains out of scope for now.

This keeps the first boundary simple and prevents proofpack from competing with receipt or eval semantics.

To satisfy `proof comes before acceptance`, future gate implementation should require proof-relevant evidence completeness before an accepting outcome is written, but that does not require a separate canonical pre-gate proofpack artifact in the first implementation step.

### How proofpack differs from receipt

- receipt = one run's execution evidence artifact;
- proofpack = provenance bundle across multiple artifacts.

Receipt says what a bounded run did.

Proofpack says which contract, receipts, evals, decision, events, and output artifacts together support inspectability.

### How proofpack differs from gate decision

- gate decision = closure authority;
- proofpack = evidence bundle that makes closure inspectable.

Decision answers: what final outcome did the project write?

Proofpack answers: what verifiable evidence supports that outcome?

### How hashes/refs should be handled later

Later proofpack work should require:

- stable relative refs or artifact ids;
- explicit hash/digest fields where the artifact model supports them;
- event range or event-root concept instead of duplicated event bodies;
- no mutable `latest` pointers as canonical proof inputs;
- later explicit cross-links between proofpack and gate decision.

Hash normalization, file path normalization, and digest scope should remain deferred until the next spec step.

### What implementation should still remain deferred

Still defer:

- `.punk/proofs` implementation;
- proofpack Rust types or writers;
- proofpack validators;
- CLI proof commands;
- signing or transparency-log behavior;
- remote attestation/export behavior;
- draft-vs-final proofpack modeling;
- exact hash normalization rules.

## Adoption map

### Adopt

- proofpack remains a separate evidence bundle;
- gate decision remains the closure authority;
- proofpack references contract, receipt, eval, event, decision, and output artifact refs/hashes instead of absorbing their bodies;
- canonical proofpack should be treated as post-gate for the first boundary;
- derived latest or summary proof views must remain non-canonical.

### Defer

- `.punk/proofs` implementation;
- proofpack schema validators;
- exact digest normalization rules;
- draft or pre-gate proofpack variants;
- signing and identity hardening.

### Park

- remote transparency logs;
- Sigstore/Rekor integration;
- cloud provenance stores;
- full SLSA compliance as an active-core requirement.

### Avoid

- proofpack as final decision artifact;
- proofpack absorbing full raw evidence bodies;
- mutable latest proof as canonical truth;
- remote provenance as current project truth.

## What stays out of scope

This research does not define:

- proofpack implementation;
- `.punk/proofs` layout;
- Rust proof types;
- validators;
- CLI proof surfaces;
- signing or transparency-log integration;
- product-doc changes.

## Impact on roadmap

This research tightens Phase 3 and later phases by clarifying that:

- proofpack should remain a separate artifact from gate decision;
- proofpack integrity work should verify refs and hashes rather than narrative summaries;
- later storage work for proofs must preserve append-only canonical-vs-derived separation;
- gate/proof implementation should remain blocked behind a proofpack boundary spec.

## Required evals

Future implementation work should add eval coverage for at least:

- proofpack requires gate decision ref/hash;
- proofpack requires contract and run receipt refs/hashes;
- proofpack references eval refs/hashes when present;
- proofpack references event range or event-root evidence rather than duplicating the log;
- proofpack link/hash integrity stays verifiable;
- proofpack does not become decision authority.

## Required docs/ADRs/contracts

Before implementation, the next bounded step should produce a proofpack boundary spec.

A later implementation branch may then require:

- a proofpack boundary spec;
- possibly an ADR if proofpack sequencing or storage semantics become canonical architecture;
- implementation goals that cite this research note and the later boundary spec;
- eval additions for proofpack integrity and decision linkage.

## Open questions

1. Should the first proofpack spec require both proof->decision and decision->proof cross-links, or can one side be mandatory first?
2. Should the first proofpack boundary require output artifact hashes immediately, or allow explicit deferral when outputs are absent?
3. Is an event range sufficient for the first proofpack, or does Punk need an event-root concept from the start?
4. Should `proof comes before acceptance` be enforced via proof readiness checks inside gate, or via same-transaction proofpack finalization requirements?
5. When should actor identity or signer identity enter scope for proof artifacts?
