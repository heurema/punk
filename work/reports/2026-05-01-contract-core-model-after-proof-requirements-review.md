---
id: report_2026_05_01_contract_core_model_after_proof_requirements_review
kind: work-report
status: final
authority: work-ledger
owner: vitaly
created_at: 2026-05-01
updated_at: 2026-05-01
goal_ref: work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md
selected_next: work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md
---

# Contract Core Model After Proof Requirements Review

## 1. Summary

Reviewed the accumulated side-effect-free contract-core model after proof requirements.

Verdict: **continue only after stabilization**. The contract-core chain is coherent and the review found no required code fix, but the local uncommitted tree is large and spans multiple tracks. The next selected goal is therefore Option D: stabilize the uncommitted contract-core tree before any new implementation, Writer, or runtime step.

Confirmed chain:

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

Preserved lifecycle order:

```text
contract
-> run receipt / evidence
-> gate outcome
-> proofpack
-> acceptance claim
```

No runtime/storage/Writer behavior was added by this review.

## 2. Files inspected

Work ledger and reports:

- `work/STATUS.md`
- `work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md`
- `work/reports/2026-04-30-user-intent-to-contract-draft-model-v0-1.md`
- `work/reports/2026-04-30-contract-schema-blueprint-v0-1.md`
- `work/reports/2026-04-30-contract-draft-confirmation-boundary-v0-1.md`
- `work/reports/2026-04-30-hard-clause-mapping-v0-1.md`
- `work/reports/2026-05-01-contract-receipt-requirements-v0-1.md`
- `work/reports/2026-05-01-contract-gate-input-policy-v0-1.md`
- `work/reports/2026-05-01-contract-proof-requirements-v0-1.md`

Product docs:

- `docs/product/CONTRACT-SCHEMA.md`
- `docs/product/CRATE-STATUS.md`
- `docs/product/ARCHITECTURE.md`
- `docs/product/ROADMAP.md`
- `docs/product/PUNK-LAWS.md`
- `docs/product/PROJECT-MEMORY.md`

Code surfaces:

- `crates/punk-contract/src/lib.rs`
- `crates/punk-eval/src/lib.rs`
- `crates/punk-domain/src/lib.rs`
- `crates/punk-gate/src/lib.rs`
- `crates/punk-proof/src/lib.rs`

Replayability advisory artifacts:

- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`

## 3. Current contract-core chain

The local tree models the chain coherently:

| Seam | Review result |
|---|---|
| raw request -> intent | Present in the intent-to-contract draft model; raw request remains a summary input, not a contract. |
| intent -> contract draft | Present; readiness classification is side-effect-free and blocks clarification/refusal/missing evidence. |
| contract draft -> user confirmation | Present; explicit confirmation is required before `approved_for_run`. |
| user confirmation -> `approved_for_run` | Present; confirmation is not gate acceptance and does not create proof. |
| hard clause mapping | Present; hard clauses require validator refs, required receipt fields, proof requirement refs, or explicit human gate review. |
| receipt requirements | Present; declares future run receipt fields and blocks approval when hard-clause receipt fields are missing/unsupported. |
| gate input policy | Present; declares future gate readiness inputs and keeps `approved_for_run != ready_for_gate`. |
| proof requirements | Present; declares future proofpack link/hash targets after gate outcome. |

No lifecycle order reversal was found.

## 4. Boundary checks

### 4.1 Model chain coherence

Result: **pass**.

The reports and model code line up around the intended sequence:

```text
request -> intent -> draft -> confirmation -> approved_for_run
-> hard clause mapping
-> receipt requirements
-> gate input policy
-> proof requirements
```

The chain does not skip confirmation, does not treat receipt requirements as receipts, does not treat gate input policy as a gate outcome, and does not treat proof requirements as a proofpack writer.

### 4.2 Approval boundary

Result: **pass**.

`approved_for_run` means the bounded contract may be executed. It does not mean:

- work accepted;
- gate passed;
- proof exists;
- Writer should run.

`ContractStatus` remains:

```text
draft
approved_for_run
superseded
cancelled
```

It does not include:

```text
accepted
rejected
partially_accepted
```

### 4.3 Hard clause boundary

Result: **pass**.

Valid hard-clause paths are modeled as:

- validator ref;
- required receipt field;
- proof requirement ref;
- explicit human gate review with non-empty reason.

Unsupported or invalid hard clauses block approval or surface as blocking findings. Soft/advisory clauses do not override hard-clause blockers.

### 4.4 Receipt requirements boundary

Result: **pass**.

Receipt requirements declare future run receipt fields. They do not write receipts, do not create `.punk/runs`, do not run validators, and do not decide acceptance.

Executor claims remain evidence-only. `executor_claims` may be a required receipt field, but executor claims are not proof unless linked evidence exists in a later seam.

### 4.5 Gate input policy boundary

Result: **pass**.

Gate input policy declares future gate inputs. It does not write gate outcomes, create proofpacks, create acceptance claims, invoke Writer, run validators, or write storage.

It distinguishes:

```text
approved_for_run != ready_for_gate
```

It rejects proofpack-shaped inputs as gate inputs. Proofpack remains a post-gate expectation.

### 4.6 Proof requirements boundary

Result: **pass**.

Proof requirements declare future proofpack links/hashes for:

- contract ref/hash;
- run receipt ref/hash;
- gate outcome ref/hash;
- eval report refs/hashes when applicable;
- output artifact refs/hashes when expected.

They do not create proofpacks, write `.punk/proofs`, write indexes/latest pointers, compute artifact hashes, read files, write gate outcomes, create acceptance claims, or invoke Writer.

The preserved order is:

```text
gate outcome -> proofpack -> acceptance claim
```

### 4.7 Writer boundary

Result: **pass**.

No reviewed contract-core seam treats Writer as:

- planner;
- authority;
- approval mechanism;
- gate substitute;
- proof substitute.

Writer remains downstream of approved contract, run receipt/evidence, gate outcome, and proofpack.

### 4.8 Runtime/storage boundary

Result: **pass for contract-core review scope**.

No new active behavior was added by this review for:

- CLI;
- `.punk` runtime storage;
- contract writer;
- receipt writer;
- gate writer;
- proofpack writer;
- acceptance claim writer;
- artifact hash runtime.

Important existing-tree note: `docs/product/CRATE-STATUS.md` already records a prior minimal first active proofpack writer write slice for explicit caller-provided test targets. That belongs to the separate proofpack-writer track, not this contract-core review. It does not activate `.punk/proofs` storage, runtime proof storage, gate outcomes, acceptance claims, or Writer. Because it coexists in the same large uncommitted tree, stabilization should group it separately before more work.

## 5. Documentation honesty check

Result: **pass with non-blocking warnings**.

`docs/product/CONTRACT-SCHEMA.md` clearly says the blueprint is a target/source-of-truth shape, not proof that all fields are active runtime behavior. It also states that `accepted`, `rejected`, and `partially_accepted` do not belong in Contract status.

`docs/product/CRATE-STATUS.md` honestly lists the current implemented subset and preserves explicit non-scope for:

- no `.punk/contracts` storage;
- no `.punk/runs` storage;
- no `.punk/decisions` storage;
- no `.punk/proofs` storage;
- no runtime contract writer;
- no runtime receipt writer;
- no runtime gate writer;
- no runtime proofpack writer;
- no Writer activation;
- no runtime validator execution;
- no acceptance claim writer.

`required_now` is scoped as Contract Schema Blueprint/model terminology, not active runtime behavior. `deferred`, `parked`, and `future` remain distinct. The review did not find `active-core now` overclaim in the replayability artifacts; that research uses `adopt-now discipline` with an explicit warning.

Remaining docs-governance warnings are non-blocking but should be handled during stabilization or a later docs cleanup:

- `Current implemented subset boundary` duplicate-definition candidate in `docs/product/CRATE-STATUS.md`.
- `Current CLI surface` duplicate-definition candidate in `docs/product/CRATE-STATUS.md`.
- `Research notes` glossary warning in `docs/product/DOCUMENTATION-MAP.md` is already tracked in `work/STATUS.md` and was not part of this docs-governance file set.

## 6. Eval coverage summary

Current `punk-eval` smoke coverage includes:

- intent-to-contract readiness;
- clarification/refusal/blocking intent states;
- explicit confirmation boundary;
- blocked/refused/clarification drafts cannot approve;
- hard clause mapping paths and blockers;
- receipt requirement coverage and blockers;
- executor claims as unverified evidence;
- gate input policy required/missing/unsupported inputs;
- no proofpack as gate input;
- `approved_for_run` separate from `ready_for_gate`;
- proof requirements required/missing/unsupported/duplicate targets;
- no proofpack writing, `.punk/proofs`, artifact hash runtime, gate outcome writing, acceptance claim, or Writer behavior from proof requirements;
- `ContractStatus` excludes `accepted`, `rejected`, and `partially_accepted` across the contract-core seams.

Review result: **coverage is sufficient for the current side-effect-free checkpoint**.

## 7. Replayability research isolation check

Result: **pass**.

Replayable Project Memory remains advisory research/idea memory.

The research and backlog explicitly state that they do not activate:

- runtime behavior;
- CLI behavior;
- Writer;
- gate/proof writers;
- agent execution;
- adapters;
- migration automation;
- code generation;
- Conformance Pack runtime;
- Migration Contract Pack runtime;
- Regenerative Spec behavior;
- spec-as-source behavior.

The core thesis remains:

```text
preserve replayable obligations, not replayable code
```

Conformance Pack remains future/deferred. Regenerative Spec Pack remains parked/future. Neither appears in the current active contract-core loop.

## 8. Drift observed

- The local working tree is large and contains many uncommitted files from multiple tracks: contract-core, replayability research, proofpack writer boundary/model work, docs, goals, reports, and eval specs.
- Existing docs-governance warnings remain visible but non-blocking.
- `punk-proof` has prior proofpack-writer track work in the local tree, including a minimal first active exact-byte write slice for explicit caller-provided targets. This was not expanded by the contract-core review, but it increases the need to stabilize/split the tree before further implementation.
- Engram persistent memory tools returned `Transport closed`; memory save/session-summary attempts are not reliable until the transport is restored.

## 9. Remaining warnings

| Warning | Severity | Blocking? | Route |
|---|---:|---:|---|
| Large uncommitted local tree spans multiple concerns. | Medium | Yes for further implementation | Option D stabilization next. |
| Docs-governance duplicate-definition candidates in `docs/product/CRATE-STATUS.md`. | Low | No | Stabilization or docs cleanup. |
| Replayability research still contains Deep Research-local citation ids. | Low | No for contract-core | Source-ref cleanup before ADR/promotion. |
| Engram transport failure. | Low | No for repo checks | Retry later; report records failure. |

## 10. Recommendation

Recommendation: **stop implementation and stabilize**.

Chosen next direction: **Option D — stop and commit / reduce uncommitted tree**.

Reason:

The model is coherent enough to stop the implementation chain. Continuing into Writer/runtime now would add risk because the current local tree is already large and crosses several conceptual tracks.

Do not choose Writer implementation.

Do not choose runtime implementation.

## 11. Next selected goal

`work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md`

## 12. Scope and non-scope for the next selected goal

Scope:

- inventory current uncommitted files;
- group files into reviewable stabilization units;
- distinguish contract-core, replayability side-track, proofpack writer track, docs-governance warnings, and unrelated prior work;
- recommend commit/PR split strategy;
- run checks and record stabilization evidence.

Non-scope:

- no implementation;
- no runtime behavior;
- no CLI;
- no `.punk` storage;
- no Writer;
- no gate writer;
- no proofpack writer;
- no artifact hash runtime;
- no acceptance claim writer;
- no adapters;
- no policy engine;
- no `punk init`.

Do not stage, commit, push, or open PRs unless explicitly requested.

## 13. Checks run and results

- `python3 scripts/check_research_gate.py` — pass; selected next is `work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md`.
- `python3 scripts/check_work_ledger.py` — pass; selected next is `work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md`; 191 goals checked.
- `cargo fmt --check` — pass.
- `cargo check --workspace` — pass.
- `cargo test -p punk-contract -p punk-eval` — pass, 39 `punk-contract` tests and 6 `punk-eval` tests.
- `git diff --check` — pass.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md --report work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md` — pass with 0 warnings.

## Doc impact

```yaml
doc_impact:
  classification: docs-only
  reason: "Review accumulated side-effect-free contract-core model after proof requirements and select tree stabilization before further implementation."
  touched_surfaces:
    - work/STATUS.md
    - work/goals/goal_review_contract_core_model_after_proof_requirements_v0_1.md
    - work/goals/goal_stabilize_uncommitted_contract_core_tree_v0_1.md
    - work/reports/2026-05-01-contract-core-model-after-proof-requirements-review.md
  required_updates:
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required: []
```
