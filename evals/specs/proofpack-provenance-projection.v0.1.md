# Proofpack Provenance Projection v0.1

Date: 2026-05-04
Status: proposed boundary
Authority: advisory/design

## Purpose

Specify how proofpack thinking can align with provenance models without adding signatures, transparency logs, remote attestations, or compliance claims.

Signatures, key management, remote transparency logs, SLSA compliance claims, in-toto layout enforcement, and release signing are deferred/parked unless separately promoted.

## Status and authority

This is an eval/spec artifact only.

It describes a local-only provenance projection over Punk artifacts. It does not write proofpacks, decisions, signatures, or release attestations.

## Definitions

### subject artifact refs

The output artifacts, reports, or other subject artifacts that a proofpack may later make inspectable.

### materials

Materials are existing artifacts considered by proofpack generation:

- contract;
- run receipt;
- eval report;
- gate decision;
- output artifacts;
- source/context refs.

### activity

Activities are lifecycle steps and proof operations:

- run;
- validation;
- gate;
- proofpack generation.

### actor/agent as observed participant, not authority

Human, script, model, module, adapter, or other executor identity may be recorded as an observed participant where policy allows. It does not become final authority.

### hashes

Proofpack projection should reference artifact hashes where applicable. Hashes make artifacts inspectable; they do not decide acceptance.

### event range/root

Proofpack projection may include event range or event root refs when event runtime exists.

### rule/guard version refs

Proofpack projection may include refs to rules, guards, validators, or policy versions used during validation and gate.

### local-only provenance projection

The projection is local and repo/runtime-artifact based. It does not require remote attestation systems.

### rebuildable/non-authoritative derived view

The projection can be rebuilt from artifact refs and hashes. It is not the source of truth and does not replace the gate decision.

## Positive cases

### PPP-001: proofpack references gate decision

Proofpack projection includes gate decision ref and hash.

Expected result: valid post-gate provenance shape.

### PPP-002: proofpack includes contract and run receipt hashes

Projection includes contract hash and run receipt hash.

Expected result: contract and receipt are inspectable.

### PPP-003: actor recorded as participant only

Projection records executor identity as observed participant, while decision authority remains `gate`.

Expected result: actor metadata does not become acceptance authority.

### PPP-004: local-only materials and activity projection

Projection lists materials and activities without requiring remote signing, transparency logs, or in-toto layout.

Expected result: local provenance is valid without compliance claims.

## Required negative cases

### PPP-101: proofpack without gate decision

A canonical proofpack projection without gate decision ref is invalid.

Expected result: reject as pre-decision proof.

### PPP-102: proofpack missing contract hash

Projection without contract hash cannot make contract scope inspectable.

Expected result: reject or mark incomplete.

### PPP-103: proofpack missing run receipt hash

Projection without required run receipt hash cannot make run evidence inspectable.

Expected result: reject or mark incomplete.

### PPP-104: proofpack references output artifact without digest

Output artifact ref without digest is incomplete for proof inspection.

Expected result: reject or require digest.

### PPP-105: proofpack makes a decision

A proofpack projection that accepts, rejects, or writes final decision violates gate authority.

Expected result: reject.

### PPP-106: remote transparency log required in active-core

Requiring remote transparency logs for current active-core proof is invalid.

Expected result: reject as premature external side-effect surface.

### PPP-107: signer/key management required in current phase

Requiring signer/key management in the current phase is invalid unless separately promoted.

Expected result: reject as deferred/parked scope.

## Minimal fixture shape

```yaml
proofpack_provenance_projection:
  proofpack_ref: proofs/example-proofpack.json
  subject_artifact_refs:
    - ref: work/reports/example.md
      hash: sha256:example-output
  materials:
    contract:
      ref: contracts/example.md
      hash: sha256:example-contract
    run_receipt:
      ref: runs/example.json
      hash: sha256:example-receipt
    eval_report:
      ref: evals/reports/example.json
      hash: sha256:example-eval
    gate_decision:
      ref: decisions/example.json
      hash: sha256:example-decision
    source_context_refs:
      - docs/product/PUNK-LAWS.md
  activities:
    - run
    - validation
    - gate
    - proofpack_generation
  observed_participants:
    - kind: human_or_executor
      authority: observed_participant
  event_range: null
  rule_guard_version_refs:
    - docs/product/PUNK-LAWS.md
  projection_authority: derived_non_authoritative
```

## Non-goals

This spec does not add signatures, key management, remote transparency logs, SLSA compliance claims, in-toto layout enforcement, release signing, proofpack writer behavior, gate writer behavior, runtime storage, or CLI behavior.

It does not make proofpack a decision surface.

## Future implementation constraints

Any future proofpack provenance work must stay post-gate, reference artifacts by refs and hashes, keep actor/agent metadata non-authoritative, and keep compliance or signing scope separate unless a later bounded goal promotes it.
