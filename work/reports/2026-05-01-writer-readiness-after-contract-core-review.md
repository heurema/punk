---
id: report_2026_05_01_writer_readiness_after_contract_core_review
kind: work-report
status: complete
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal: work/goals/goal_review_writer_readiness_after_contract_core_v0_1.md
selected_next: work/goals/goal_review_acceptance_claim_boundary_v0_1.md
---

# Writer readiness after contract-core review

## 1. Summary

Review verdict:

```text
continue_with_acceptance_claim_boundary_review_before_writer_boundary
```

Punk is **not ready to define a Writer boundary yet**. The contract-core chain, gate/proof boundary, and proofpack-writer track are coherent enough to continue review, but the acceptance claim surface is still too implicit for Writer boundary design.

Current trusted chain remains:

```text
approved contract
-> run receipt / evidence
-> gate decision
-> proofpack
-> acceptance claim
```

The review found:

- Writer must remain downstream of accepted gate/proof evidence, not upstream planning or authority.
- Existing contract/gate/proof models already block Writer from defining contracts, writing gate decisions, claiming acceptance, or replacing proof.
- The proofpack-writer track has one already-reviewed bounded exact-byte writer slice, but no new proofpack writer behavior was added by this review.
- Acceptance claim semantics exist at high level through proof-before-acceptance docs/specs and positive acceptance precondition helpers, but there is no explicit acceptance claim boundary review for future Writer handoff.

Chosen next option:

```text
Option B — acceptance claim boundary review
```

Reason: a future Writer boundary may eventually write acceptance claim records, project-memory links, derived inspect views, or reports. Those outputs require an explicit acceptance claim boundary first, so Writer does not drift into gate decision authority, proof authority, project truth ownership, or canonical generated-doc authority.

No product code, product docs, runtime behavior, CLI, Writer, storage, gate writer, proof writer, new proofpack writer behavior, new artifact hash runtime, acceptance claim writer, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, or spec-as-source behavior was added by this review.

## 2. Files inspected

Git state:

```bash
git status --short
git log --oneline -8
```

Observed latest commits at review start:

```text
dff9bd5 chore(work): record proofpack writer track review
960c9de chore(work): record gate proof alignment review
4845caf chore(work): select gate proof alignment review
be9b067 chore(work): record contract-core checkpoint verification
e05ff20 chore(work): checkpoint contract-core stabilization tree
3737d11 ci: pin pr intake gate v0.2.0
bc01233 chore(github): remove local PR intake engine
fd33207 chore(github): use shared PR intake gate
```

Working tree at review start:

```text
clean
```

Work ledger and reports:

- `work/STATUS.md`
- `work/goals/goal_review_writer_readiness_after_contract_core_v0_1.md`
- `work/reports/2026-05-01-proofpack-writer-track-after-checkpoint-review.md`
- `work/reports/2026-05-01-gate-proof-model-alignment-review-v0-1.md`
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

## 3. Current readiness state

The side-effect-free contract-core chain is coherent through proof requirements:

```text
raw request
-> intent
-> contract draft
-> user confirmation
-> approved_for_run
-> hard clause mapping
-> receipt requirements
-> gate input policy
-> proof requirements
```

Gate/proof remains coherent:

```text
contract
-> run receipt / evidence
-> gate decision
-> proofpack
-> acceptance claim
```

Proofpack-writer track classification from the prior review:

| Surface | Current classification |
|---|---|
| Proofpack/provenance helpers, manifest renderer, manifest digest, link/hash integrity | `active-core` |
| Writer preflight, operation evidence, file IO plan/outcome/error, target path policy, canonical artifact, host path, concrete path/storage, hash/reference integration | `side-effect-free model` |
| `proofpack_writer_write_first_active_slice` | `active bounded writer slice` |
| Broader writer orchestration, `.punk/proofs`, index/latest, schema/CLI, referenced artifact active verification | `parked/future` |

Important wording constraint:

```text
Do not say broadly: no proofpack writer exists.
Do say: only the already-reviewed bounded exact-byte writer slice exists;
new proofpack writer behavior was not added by this review;
broader proofpack writer/runtime/storage remains parked/future.
```

## 4. Writer downstream boundary check

Result: **pass** for review readiness, not implementation readiness.

`crates/punk-contract/src/lib.rs` exposes boundary helpers showing Writer is not upstream contract authority:

```text
writer_is_upstream_planner = false
writer_defines_contract = false
writer_writes_gate_decision = false
writer_claims_acceptance = false
```

The user intent and contract draft models also keep:

```text
raw_request_is_contract = false
intent_is_execution_permission = false
user_confirmation_is_gate_acceptance = false
executor_claims_are_proof = false
writer_is_downstream_only = true
writes_gate_decision = false
writes_proofpack = false
```

A future Writer must be downstream of:

```text
approved contract
-> run receipt/evidence
-> gate decision
-> proofpack
```

A future Writer must not operate as authority over:

- raw request;
- intent only;
- contract draft;
- `approved_for_run` without gate/proof;
- executor claim only.

## 5. Allowed future Writer outputs

This review does not authorize any of these outputs now.

| Possible future Writer output | Allowed now? | Future prerequisites |
|---|---:|---|
| Proofpack manifests | No, not by Writer. | Keep proofpack writer as a separate bounded surface; requires gate decision, proof requirements, proofpack writer boundary, eval coverage, and explicit selection if Writer ever orchestrates it. |
| Acceptance claim records | No. | Requires acceptance claim boundary review, accepting gate decision, matching proofpack, artifact refs/hashes as policy requires, unresolved deviation policy, idempotency rules, and separate ADR/eval/goal before any writer. |
| Project-memory links | No active Writer output. | Requires accepted chain refs, project-memory authority rules, source refs, and explicit promotion path; generated links must not become canonical truth by themselves. |
| Derived inspect views | No active Writer output. | Future only; must remain regenerable/advisory and must not become acceptance source of truth. |
| Reports | No active Writer output. | Future only; reports can summarize evidence but must not decide, prove, or claim acceptance without gate/proof/acceptance chain. |

## 6. Forbidden Writer outputs

A future Writer must never write or own:

- gate decisions;
- contract approval;
- contract truth;
- module decisions;
- adapter truth;
- executor claims as proof;
- raw/generated docs as canonical truth;
- proofpack-as-decision authority;
- acceptance without accepting gate decision plus matching proof.

Writer must not become:

```text
planner
authority
gate substitute
proof substitute
acceptance claim writer by default
runtime storage owner
CLI behavior by implication
```

## 7. Proofpack-writer dependency check

Result: **partially sufficient for review, insufficient for Writer boundary design by itself**.

The proofpack-writer review found the active bounded writer slice coherent:

```text
proofpack_writer_write_first_active_slice
```

It writes exact supplied canonical artifact bytes only to an explicit caller-provided storage root plus target-relative path when preflight and concrete path/storage policy are ready.

It does not activate:

- `.punk/proofs`;
- runtime storage;
- index/latest pointers;
- persisted operation evidence;
- gate decisions;
- acceptance claims;
- CLI;
- Writer;
- broader proofpack writer orchestration.

This reduces risk for future Writer discussion because file IO and proofpack writer behavior are bounded. It does not define what a Writer may write, how acceptance claims are represented, or when project memory may link accepted work.

## 8. Acceptance claim boundary check

Result: **needs review before Writer boundary design**.

Existing evidence:

- `docs/product/ARCHITECTURE.md` says positive acceptance claims require an accepting gate decision plus matching proof.
- `docs/product/ROADMAP.md` records `goal -> contract -> approve -> run -> gate decision -> proofpack -> acceptance claim`.
- `docs/product/PUNK-LAWS.md` says proof comes before acceptance.
- `evals/specs/proof-before-acceptance-semantics.v0.1.md` defines high-level proof-before-acceptance semantics.
- `crates/punk-gate/src/lib.rs` makes accepting decisions require matching proof for acceptance claims while not creating acceptance claims.
- `crates/punk-proof/src/lib.rs` exposes `positive_acceptance_preconditions_met`, requiring an accepting gate decision plus matching proof readiness.

Remaining gap:

```text
Acceptance claim is a lifecycle concept, not yet an explicit reviewed boundary for Writer handoff.
```

Before Writer boundary design, Punk should review or define:

- what an acceptance claim record is;
- what it may cite;
- whether it is writable by future Writer or only by a later acceptance surface;
- required refs and hashes;
- idempotency and immutability expectations;
- whether negative/non-accepting outcomes have proofpack-only audit records but no acceptance claim;
- deviation handling and accepted-difference references;
- project-memory update rules after acceptance;
- whether generated/public narrative can link to acceptance without becoming canonical truth.

No active acceptance claim writer exists now.

## 9. Docs honesty check

Result: **pass with accepted/deferred existing warnings**.

Reviewed docs distinguish current behavior from future surfaces:

- `docs/product/CONTRACT-SCHEMA.md` states runtime storage, CLI behavior, gate/proof writers, Writer activation, agent execution, adapters, signatures, policy engines, and remote transparency remain out of scope unless explicitly promoted.
- `docs/product/CRATE-STATUS.md` accurately states `punk-proof` has one minimal first active proofpack writer write slice and still has no broader orchestration, `.punk/proofs` writer, runtime storage, CLI, schema writer, gate decision writer, or acceptance claim writer.
- `docs/product/ARCHITECTURE.md` keeps proofpack post-gate and says positive acceptance claims require accepting gate decision plus matching proof.
- `docs/product/ROADMAP.md` keeps the closure order `gate decision -> proofpack -> acceptance claim`.

No wording was found that makes Writer active, proof writer active, acceptance claim active, `.punk` runtime active, spec-as-source active, Conformance Pack active, or Regenerative Spec behavior active.

## 10. Eval coverage summary

Current eval/smoke coverage includes:

- intent-to-contract readiness;
- contract draft confirmation boundary;
- hard-clause mapping;
- receipt requirements;
- gate input policy;
- proof requirements;
- proofpack is not a gate input;
- no gate/proof/Writer activation through contract models;
- `ContractStatus` excludes `accepted`, `rejected`, and `partially_accepted`;
- gate authority requires proof before acceptance;
- acceptance requires accepting decision plus matching proofpack;
- proofpack manifest/provenance and manifest digest helpers;
- proofpack writer preflight/operation evidence/file IO plan/outcome/error/target path/canonical artifact/target ref/host path/concrete path/hash-reference models;
- first active proofpack writer exact-byte write slice to explicit temporary targets only.

Future eval gaps before Writer implementation:

- acceptance claim boundary conformance;
- Writer forbidden-output checks;
- Writer downstream-input checks;
- project-memory link authority checks after acceptance;
- generated/public narrative non-canonical checks;
- idempotent acceptance claim record behavior, if such a writer is ever selected.

These are future goals, not implemented now.

## 11. Replayability isolation check

Result: **pass**.

Replayable Project Memory remains advisory research with the thesis:

```text
preserve replayable obligations, not replayable code
```

It did not activate:

- Conformance Pack runtime;
- Migration Contract Pack runtime;
- Regenerative Spec Pack behavior;
- code generation;
- spec-as-source behavior;
- Writer behavior;
- gate/proof/proofpack writer behavior.

Conformance Pack remains deferred/future. Regenerative Spec Pack remains parked/future.

## 12. Drift observed

No code drift or docs overclaim requiring immediate product-doc changes was found.

Observed review nuance:

```text
Do not describe the system as having no proofpack writer at all.
```

Correct wording:

```text
No new proofpack writer behavior was added by this review.
Only the already-reviewed bounded exact-byte proofpack writer slice exists.
Broader proofpack writer/runtime/storage remains parked/future.
```

## 13. Remaining warnings

Accepted/deferred docs-governance warnings remain from prior reviews:

- `docs/product/DOCUMENTATION-MAP.md` — `Research notes` heading looks like an undeclared glossary term.
- `docs/product/CRATE-STATUS.md` — `Current implemented subset boundary` heading looks like a duplicate definition candidate.
- `docs/product/CRATE-STATUS.md` — `Current CLI surface` heading looks like a duplicate definition candidate.
- `docs/product/PROJECT-MEMORY.md` — `Project coherence` heading looks like a duplicate definition candidate.

They are non-blocking for this review. A later docs-governance cleanup goal can handle them.

Engram memory transport failed during the session (`Transport closed`). This is a tooling warning outside the repo diff and does not block the work ledger.

## 14. Recommendation

Recommendation:

```text
cleanup/review first: acceptance claim boundary review
```

Do not proceed to Writer boundary design yet.

Rationale:

- Gate/proof/proofpack-writer boundaries are clean enough for the next review.
- The first active proofpack writer exact-byte slice is isolated.
- Acceptance claim writer remains inactive.
- Acceptance claim semantics are high-level and need a dedicated boundary review before future Writer outputs are planned.

## 15. Next selected goal

Selected next:

```text
work/goals/goal_review_acceptance_claim_boundary_v0_1.md
```

Chosen option:

```text
Option B — acceptance claim boundary review
```

## 16. Scope and non-scope for next goal

Next goal scope:

- review existing proof-before-acceptance semantics;
- classify acceptance claim as lifecycle concept, future record, future writer output, or project-memory link requirement;
- identify required refs/evidence before any future acceptance claim writer or Writer boundary;
- choose the smallest next step.

Next goal non-scope:

- no Writer implementation;
- no acceptance claim writer;
- no gate writer;
- no proof writer;
- no proofpack writer expansion;
- no artifact hash runtime expansion;
- no `.punk` runtime storage;
- no CLI;
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
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_review_writer_readiness_after_contract_core_v0_1.md work/goals/goal_review_acceptance_claim_boundary_v0_1.md work/reports/2026-05-01-writer-readiness-after-contract-core-review.md --report work/reports/2026-05-01-writer-readiness-after-contract-core-review.md` — PASS.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Record Writer-readiness review and select acceptance-claim boundary review without changing product behavior."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_review_writer_readiness_after_contract_core_v0_1.md
    - work/goals/goal_review_acceptance_claim_boundary_v0_1.md
    - work/reports/2026-05-01-writer-readiness-after-contract-core-review.md
  required_updates:
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
