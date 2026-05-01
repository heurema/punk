---
id: docs_product_contract_schema
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-04-30
updated_at: 2026-05-01
review_after: 2026-07-30
canonical_for:
  - contract-schema-blueprint
  - contract-field-status-split
  - contract-clause-shape
  - contract-gate-policy-input-boundary
  - contract-proof-requirements-boundary
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
related_evals:
  - evals/specs/user-intent-to-contract-ux-flow-boundary.v0.1.md
supersedes: []
superseded_by: null
---

# Contract Schema Blueprint

## Status

This document is the canonical product description for `Contract Schema Blueprint v0.1`.

It preserves the target Contract shape for incremental implementation.

It does not claim that all fields are active runtime behavior.

Runtime storage, CLI behavior, gate/proof writers, Writer activation, agent execution, adapters, signatures, policy engines, and remote transparency remain out of scope unless explicitly promoted by roadmap, status, goals, and evidence.

## Purpose

A Punk Contract is the bounded work agreement that sits between user intent and execution.

It preserves:

```text
bounded intent
+ explicit scope
+ typed clauses
+ validation/evidence plan
+ receipt requirements
+ gate policy inputs
+ proof requirements
+ memory links
+ amendment/deviation policy
```

A Contract is not:

- a raw user request;
- a prompt;
- a final decision;
- a proofpack;
- a Writer;
- an execution engine;
- a smart contract;
- a legal-document simulator.

## Required distinctions

Punk must preserve these distinctions:

```text
raw request != intent
intent != contract draft
contract draft != approved contract
user confirmation != gate acceptance
approved contract != final decision
gate decision != proofpack
proofpack != acceptance authority
Writer != upstream planner
```

## Blueprint sections

`Contract Schema Blueprint v0.1` has these top-level sections:

| Section | Role | Status |
|---|---|---|
| `identity` | Identifies the contract and digest metadata. | `required_now` |
| `authority` | Links to upper rules and phase authority. | `required_now` with parked signature fields |
| `lifecycle` | Tracks contract lifecycle without owning closure. | `required_now` |
| `work` | Captures intent, rationale, assumptions, unknowns, research, and acceptance criteria. | `required_now` with future semantic helpers |
| `boundaries` | Defines include/exclude scope, non-scope, and side-effect boundaries. | `required_now` with deferred manifests |
| `clauses` | Defines typed contract clauses and coverage requirements. | `required_now` with deferred coverage matrix |
| `validation` | Defines validators, eval refs, and unsupported-clause policy. | `required_now` with deferred compatibility checks |
| `evidence` | Defines required artifacts, source policy, freshness, and contradiction handling. | `required_now` with deferred automation |
| `receipt_requirements` | Defines what every run receipt must record. | `required_now` |
| `gate_policy` | Declares inputs and policy for later gate review. | `required_now` with deferred partial acceptance workflow |
| `proof_requirements` | Declares required proof links and hashes. | `required_now` with parked transparency logs |
| `change_control` | Defines amendment, supersession, deviation, and waiver policy. | `required_now` with deferred waiver ledger |
| `memory_links` | Links goals, reports, research, ADRs, evals, decisions, proofs, docs, and public narrative. | `required_now` with future multi-agent merge helpers |

## Field status split

For v0.1, blueprint fields use this split:

| Status | Meaning |
|---|---|
| `required_now` | Active-core blueprint field that should be preserved in code/docs/evals now. |
| `deferred` | Needed later, but not active in the current slice. |
| `parked` | Known future-adjacent capability that must not appear in the current operator path. |
| `future` | Directional possibility only; do not implement without a later roadmap promotion. |

`required_now` means required in the side-effect-free blueprint/model now. It does not mean active CLI, runtime storage, or contract writer behavior.

### Required now

For v0.1, active-core blueprint fields include:

- `identity`: `kind`, `schema_version`, `id`, `title`, `created_at`, digest metadata;
- `authority`: core rules ref, project rules ref, roadmap phase ref, Research Gate ref, authority level, upper-rule narrowing/weaken flags;
- `lifecycle`: `draft`, `approved_for_run`, `superseded`, `cancelled`, approval refs, supersession refs, and `closed_by_gate_ref` as a link only;
- `work`: goal ref, intent, rationale, assumptions, unknowns, research refs, decision refs, acceptance criteria;
- `boundaries`: scope include/exclude, non-scope, forbidden effects, side-effect boundaries, executor/module/adapter boundaries;
- `clauses`: typed clause fields and hard-clause mapping rule;
- `validation`: validator plan, eval refs, unsupported-clause policy;
- `evidence`: evidence plan, required artifacts, source policy, freshness policy, stale/superseded/contradiction flags;
- `receipt_requirements`: contract id, executor id, timestamps, inputs, outputs, artifact refs/hashes, validator results, side effects, deviations, executor claims;
- `gate_policy`: required gate inputs, hard-fail conditions, accept-requires policy;
- `proof_requirements`: required links and hashes;
- `change_control`: supersede-only amendment policy, supersession policy, deviation policy;
- `memory_links`: goal, contract, context pack, report, research, ADR, eval, decision, proof, DocImpact, public narrative refs.

### Deferred

Deferred fields include:

- context-clause coverage matrix;
- waiver ledger;
- partial acceptance workflow;
- richer compatibility/version checks;
- module/adapter boundary manifests;
- stale/superseded source automation.

### Parked

Parked fields include:

- PKI signatures;
- remote transparency logs;
- OPA/Rego or policy engine integration;
- full capability manifests;
- contract negotiation workflow.

### Future-only

Future-only fields include:

- semantic intent recovery;
- contract-as-source generation;
- spec-native takeover;
- multi-agent coordinated contracts;
- semantic merge of contracts/specs/code.

## Lifecycle boundary

Allowed Contract statuses are:

```text
draft
approved_for_run
superseded
cancelled
```

Do not add `accepted`, `rejected`, or `partially_accepted` to Contract status.

Those belong to gate outcome semantics, not to Contract lifecycle.

`closed_by_gate_ref` is only a link. It does not make Contract the decision surface.

## Typed clause model

For v0.1, Contract clauses use this shape:

```text
ContractClause {
  id
  kind
  text
  mode
  severity
  source_refs
  validator_refs
  required_receipt_fields
  proof_requirement_refs
  gate_review_required
  human_gate_review_reason
  unsupported_clause_finding
}
```

`kind` values:

- `precondition`
- `invariant`
- `acceptance`
- `evidence`
- `prohibition`
- `rationale`

`mode` values:

- `human`
- `machine`
- `hybrid`

`severity` values:

- `hard`
- `soft`
- `advisory`

Every hard clause must map to at least one of:

- validator refs;
- required receipt fields;
- proof requirement refs;
- explicit human gate review with a non-empty reason.

An unsupported hard clause is not a valid mapping. It must remain a blocking
finding until the clause is mapped to validator refs, required receipt fields,
proof requirement refs, explicit human gate review with reason, downgraded to
soft/advisory, removed, or converted into an assumption/unknown that requires
clarification.

Validator refs are references only in this blueprint. They do not execute
validators. Explicit human gate review declares a future gate input only; it
does not write gate outcomes or imply acceptance.

## Receipt requirements boundary

`receipt_requirements` declare what a future run receipt must contain.

They may require fields such as:

- `contract_id`;
- `executor_id`;
- `started_at`;
- `ended_at`;
- `inputs`;
- `outputs`;
- `artifact_refs`;
- `artifact_hashes`;
- `validator_results`;
- `side_effects`;
- `deviation_flags`;
- `executor_claims`.

If a hard clause maps to a required receipt field, the contract receipt
requirements must include that field before the draft can become
`approved_for_run` model state.

Receipt requirements do not:

- write runtime receipts;
- write `.punk/runs`;
- execute validators;
- verify executor claims;
- decide acceptance;
- write gate outcomes;
- create proofpacks;
- invoke Writer.

`executor_claims` may be required as a future receipt field, but executor
claims remain unverified unless linked validator or evidence surfaces exist.

## Gate policy boundary

`gate_policy` declares required gate inputs.

It does not write gate outcomes.

It must preserve:

```text
gate_only_final_decision = true
```

Required inputs include:

- contract ref;
- `approved_for_run` status;
- run receipt ref;
- receipt requirement coverage;
- hard-clause mapping assessment;
- validator/eval/assessment refs when applicable;
- scope status;
- deviation status;
- executor claim status.

Hard-fail inputs include missing contract ref, missing `approved_for_run`
status, missing run receipt ref, missing receipt requirement coverage, missing
hard-clause mapping assessment, unmapped hard clauses, and unsupported gate
inputs.

`approved_for_run` is not the same as `ready_for_gate`.

A contract can be approved for run before a run receipt exists. Gate readiness is
a later assessment over post-run evidence.

Gate input policy must not require an existing proofpack. Proofpack creation is
downstream of gate outcome:

```text
gate outcome -> proofpack -> acceptance claim
```

If proof is mentioned at this boundary, it is only a pending post-gate
obligation, not an input required before gate can decide.

## Proof requirements boundary

`proof_requirements` declares what a future proofpack must link and hash after
a gate outcome exists.

It does not create proofpacks.

It does not compute hashes, read files, verify artifact contents, write
`.punk/proofs`, write indexes, update `latest` pointers, write gate outcomes, or
create acceptance claims.

It does not make proofpack a second authority surface.

Required links:

- `contract_ref`;
- `run_receipt_ref`;
- `gate_decision_ref`;
- `output_artifact_refs` when output artifacts are expected;
- `eval_report_ref` when eval reports are required.

Required hashes:

- `contract_hash`;
- `run_receipt_hash`;
- `gate_decision_hash`;
- `output_artifact_hashes` when output artifacts are expected;
- `eval_report_hash` when eval reports are required.

Proof requirements are declaration-only. They may require future proofpack
targets, but they do not make proofpack available before gate:

```text
gate outcome -> proofpack -> acceptance claim
```

Acceptance claims remain downstream and require an accepting gate outcome plus
matching proof evidence.

## Change control

For v0.1:

```text
amendment_policy.mode = supersede_only
deviation_record_required = true
waiver_policy = deferred
```

A contract can be superseded or cancelled, but it cannot silently mutate after approval.

## Avoid

Do not reinterpret this blueprint as:

- smart-contract-style automatic finality;
- oracle-driven truth;
- on-chain execution;
- gas-shaped design;
- proofpack as decision authority;
- Writer as upstream planner.

## Current activation boundary

Active now:

- side-effect-free Rust types/model for the blueprint;
- docs owner for the blueprint;
- local smoke evals for boundary checks.

Not active now:

- `punk plot` / `punk cut` / `punk gate` runtime flow beyond existing narrow surfaces;
- `.punk/contracts` storage;
- runtime contract writer;
- Writer activation;
- gate outcome writer;
- proofpack writer expansion;
- provider/model/agent execution;
- adapter invocation;
- PKI signatures;
- remote transparency logs.
