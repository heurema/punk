---
id: report_2026_05_01_acceptance_claim_boundary_review_v0_1
kind: work-report
status: complete
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal: work/goals/goal_review_acceptance_claim_boundary_v0_1.md
selected_next: work/goals/goal_define_acceptance_claim_boundary_v0_1.md
---

# Acceptance claim boundary review v0.1

## 1. Summary

Review verdict:

```text
continue_with_acceptance_claim_boundary_definition
```

Acceptance claim is a distinct trust surface that should be explicitly defined before any Writer boundary design.

Reviewed lifecycle:

```text
contract
-> run receipt / evidence
-> gate decision
-> proofpack
-> acceptance claim
```

The current docs/specs and model helpers already preserve the high-level rule:

```text
acceptance claim requires accepting gate decision + matching proofpack
```

But this is not yet captured as its own boundary artifact for future Writer, project-memory, or runtime/storage handoff. Therefore the next selected goal is Option A:

```text
work/goals/goal_define_acceptance_claim_boundary_v0_1.md
```

This next goal must remain boundary/docs/model only. It must not implement an acceptance claim writer, Writer, CLI, runtime storage, gate writer, proof writer, proofpack writer expansion, artifact hash runtime, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior.

No product code, product docs, runtime behavior, CLI, Writer, storage, gate writer, proof writer, new proofpack writer behavior, new artifact hash runtime, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added by this review.

## 2. Files inspected

Git state:

```bash
git status --short
git log --oneline -8
```

Observed latest commits at review start:

```text
2f64f0f chore(work): record writer readiness review
dff9bd5 chore(work): record proofpack writer track review
960c9de chore(work): record gate proof alignment review
4845caf chore(work): select gate proof alignment review
be9b067 chore(work): record contract-core checkpoint verification
e05ff20 chore(work): checkpoint contract-core stabilization tree
3737d11 ci: pin pr intake gate v0.2.0
bc01233 chore(github): remove local PR intake engine
```

Working tree at review start:

```text
clean
```

Work ledger and reports:

- `work/STATUS.md`
- `work/goals/goal_review_acceptance_claim_boundary_v0_1.md`
- `work/reports/2026-05-01-writer-readiness-after-contract-core-review.md`
- `work/reports/2026-05-01-gate-proof-model-alignment-review-v0-1.md`
- `work/reports/2026-05-01-proofpack-writer-track-after-checkpoint-review.md`
- `work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md`
- `work/goals/goal_define_proof_before_acceptance_semantics_v0_1.md`
- `work/reports/2026-04-25-proof-before-acceptance-semantics-v0-1.md`

Product docs and specs:

- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/CONTRACT-SCHEMA.md`
- `docs/product/PROJECT-MEMORY.md`
- `evals/specs/proof-before-acceptance-semantics.v0.1.md`

Code surfaces:

- `crates/punk-contract/src/lib.rs`
- `crates/punk-gate/src/lib.rs`
- `crates/punk-proof/src/lib.rs`
- `crates/punk-eval/src/lib.rs`

Replayability advisory artifacts:

- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`

## 3. Acceptance claim definition

Acceptance claim in Punk terms:

```text
A downstream project-memory claim that a bounded work item may be described as accepted because it references an accepting gate decision and a matching proofpack.
```

Acceptance claim is not:

- gate decision;
- proofpack;
- executor claim;
- Writer decision;
- module assessment;
- adapter output;
- generated documentation;
- raw project memory.

Authority split:

| Surface | Role |
|---|---|
| Gate decision | Final decision surface over evidence. |
| Proofpack | Post-gate provenance and link/hash inspectability. |
| Acceptance claim | Downstream claim that acceptance is now claimable because decision and proof match. |
| Writer | Future downstream writer only if separately scoped; never decision authority. |

## 4. Acceptance claim prerequisites

Minimum prerequisites for a positive acceptance claim should be:

- contract ref;
- accepting gate decision ref;
- matching proofpack ref;
- run receipt ref;
- eval or assessment refs when the gate decision relied on them;
- artifact refs and hashes where the proof policy or proofpack requires them;
- no unresolved hard deviations unless explicitly waived or accepted by gate policy;
- proofpack links to the required decision/evidence/artifact refs;
- project-memory link policy, if the claim will update project memory or public narrative.

Eval report refs appear conditional in v0.1: required when the gate decision, proof requirements, or proofpack policy depends on eval/assessment evidence; not universally required by every acceptance claim unless a future boundary says so.

## 5. Matching proofpack definition

Boundary-level matching relation, without implementation:

A proofpack matches an acceptance claim candidate when it references the same accepted evidence chain that the accepting gate decision decided over.

Minimum matching candidates:

- same contract id/ref and, when available, contract hash;
- same run receipt id/ref and, when available, receipt hash;
- same gate decision id/ref and, when available, decision hash;
- same eval/assessment refs and hashes when required by gate/proof policy;
- same output artifact refs and hashes when applicable;
- event range/root when applicable;
- rule/guard version refs when applicable;
- no missing required digest/link coverage that policy says blocks proof.

This review does not implement matching logic. It only identifies the boundary that a later docs/model/eval goal should capture.

## 6. Forbidden acceptance claim behavior

Acceptance claim must never:

- write final decision;
- override gate;
- weaken proof requirements;
- treat executor claim as proof;
- accept work without proofpack;
- accept work without gate decision;
- create proofpack;
- run validators;
- invoke Writer as authority;
- make proofpack a decision surface;
- turn generated docs or derived views into canonical truth.

`ContractStatus` must still exclude:

```text
accepted
rejected
partially_accepted
```

Accept/reject-style outcomes belong to gate decision, not Contract.

## 7. Writer relationship

If a future Writer writes acceptance claims, all of the following must be true first:

- an accepting gate decision exists;
- a matching proofpack exists;
- required contract, run receipt, eval/assessment, event, artifact, rule/guard, and proof refs are linked as policy requires;
- no unresolved hard deviations remain except those explicitly waived or accepted by gate policy;
- the Writer output is inspectable and linked;
- Writer does not decide acceptance;
- Writer does not infer acceptance from proofpack alone;
- Writer does not create hidden project truth;
- Writer does not write gate decisions or proofpacks unless a separate bounded surface explicitly selects such orchestration.

This review does not authorize Writer boundary design or implementation.

## 8. Project memory relationship

Potential future locations or views for acceptance claims:

| Candidate location/view | Current status | Notes |
|---|---|---|
| `.punk/claims/` future runtime | future/parked | Requires runtime storage boundary, schema, writer, and evals. |
| `work/reports/` manual Level 0 evidence | current manual process only | Reports can state review conclusions but are not active acceptance claim writer artifacts. |
| project-memory link graph | future | Must link gate/proof/contract/receipt refs and preserve authority tags. |
| derived inspect view | future | Regenerable view only; must not become source of truth. |

Current state:

```text
No active acceptance claim writer or runtime storage exists.
```

## 9. Replayability isolation check

Result: **pass**.

Replayable Project Memory remains advisory research with the safe thesis:

```text
preserve replayable obligations, not replayable code
```

Conformance Pack, Migration Contract Pack, Regenerative Spec Pack, code generation, and spec-as-source behavior must not create acceptance claims or bypass gate/proof.

No replayability artifact was promoted into active acceptance, Writer, gate, proof, or runtime behavior by this review.

## 10. Docs honesty check

Result: **pass**.

Reviewed docs/specs say:

- proof comes before acceptance;
- only gate writes final decisions;
- proofpack is post-gate provenance and not a decision surface;
- positive acceptance claims require accepting gate decision plus matching proof;
- no active acceptance claim writer exists in `punk-core`, `punk-contract`, `punk-gate`, or `punk-proof`;
- `punk-proof` has one already-reviewed bounded exact-byte proofpack writer slice, but no `.punk/proofs`, runtime storage, index/latest, persisted operation evidence, gate decision, or acceptance claim writer.

No wording overclaim was found requiring product-doc edits in this review.

## 11. Future eval implications

Future evals before any acceptance claim writer should cover:

- acceptance claim requires accepting gate decision;
- acceptance claim requires matching proofpack;
- proofpack alone cannot create acceptance claim;
- rejected, blocked, needs-work, or deferred gate decision blocks positive acceptance claim;
- missing required artifact hash blocks positive acceptance claim when policy requires that hash;
- missing required eval/assessment ref blocks positive acceptance claim when gate/proof policy requires it;
- executor claim cannot satisfy acceptance claim;
- Writer cannot write acceptance without gate/proof;
- ContractStatus still excludes accepted/rejected/partially_accepted;
- generated docs or derived views cannot become canonical acceptance truth.

Do not implement these evals in this review. The next boundary definition goal can decide whether to add an eval-spec outline or side-effect-free model later.

## 12. Drift observed

No immediate docs or code drift requiring correction was found.

Observed gap:

```text
Acceptance claim has high-level proof-before-acceptance semantics, but not its own explicit boundary artifact.
```

That gap should be closed before Writer boundary design.

## 13. Remaining warnings

Accepted/deferred docs-governance warnings remain from prior reviews:

- `docs/product/DOCUMENTATION-MAP.md` — `Research notes` heading looks like an undeclared glossary term.
- `docs/product/CRATE-STATUS.md` — `Current implemented subset boundary` heading looks like a duplicate definition candidate.
- `docs/product/CRATE-STATUS.md` — `Current CLI surface` heading looks like a duplicate definition candidate.
- `docs/product/PROJECT-MEMORY.md` — `Project coherence` heading looks like a duplicate definition candidate.

They are non-blocking for this review.

Engram memory transport failed during the session (`Transport closed`). This is a tooling warning outside the repo diff and does not block the work ledger.

## 14. Recommendation

Recommendation:

```text
continue_with_boundary_definition
```

Choose Option A:

```text
work/goals/goal_define_acceptance_claim_boundary_v0_1.md
```

Reason: acceptance claim is a separate trust surface. It should be captured as a boundary/docs/model artifact before any Writer boundary design, runtime storage, or acceptance claim writer.

## 15. Next selected goal

Selected next:

```text
work/goals/goal_define_acceptance_claim_boundary_v0_1.md
```

Chosen option:

```text
Option A — define acceptance claim boundary docs/model, no writer
```

## 16. Scope and non-scope for next selected goal

Next goal scope:

- define acceptance claim boundary as docs/model/eval-spec artifact;
- preserve acceptance claim downstream of accepting gate decision plus matching proof;
- specify prerequisites, forbidden behavior, and matching proofpack expectations;
- keep future Writer relationship explicit but inactive.

Next goal non-scope:

- no acceptance claim writer;
- no Writer implementation;
- no runtime storage;
- no CLI;
- no gate writer;
- no proof writer;
- no proofpack writer expansion;
- no artifact hash runtime expansion;
- no Conformance Pack runtime;
- no Migration Contract runtime;
- no Regenerative Spec behavior;
- no spec-as-source behavior.

## 17. Checks run and results

- `python3 scripts/check_research_gate.py` — PASS.
- `python3 scripts/check_work_ledger.py` — PASS.
- `cargo check --workspace` — PASS.
- `cargo test -p punk-contract -p punk-eval` — PASS.
- `git diff --check` — PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_review_acceptance_claim_boundary_v0_1.md work/goals/goal_define_acceptance_claim_boundary_v0_1.md work/reports/2026-05-01-acceptance-claim-boundary-review-v0-1.md --report work/reports/2026-05-01-acceptance-claim-boundary-review-v0-1.md` — PASS.
