# ADR-0012: Minimal proofpack provenance

Status: Accepted
Date: 2026-04-19

Research refs:

- `knowledge/research/2026-04-19-project-ideas-intake.md`

## Context

`punk` acceptance requires proof before acceptance. A proofpack should make a gate decision inspectable by linking what was requested, what ran, what evidence was produced, and what final decision was written.

Full supply-chain provenance systems are too large for the current phase, but their primitives are useful: subjects, materials, run details, executor identity, artifact digests, and verifiable references.

## Decision

Adopt a minimal local proofpack manifest.

A proofpack must reference, at minimum:

- contract id and contract hash
- run receipt id and hash
- eval report id and hash, when applicable
- gate decision id and hash
- output artifact refs and hashes
- relevant event range or event root hash
- rule/guard version refs
- timestamp and local project id

## Minimal schema sketch

```yaml
schema_version: punk.proofpack.v0.1
proof_id: proof_...
project_id: project_...
contract:
  id: contract_...
  hash: sha256:...
run_receipts:
  - id: run_...
    hash: sha256:...
eval_reports:
  - id: eval_...
    hash: sha256:...
decision:
  id: decision_...
  hash: sha256:...
artifacts:
  - ref: work/reports/...
    hash: sha256:...
events:
  range: events-0001:120..148
  root_hash: sha256:...
rules:
  guard_set: punk-rules-v0.1
privacy:
  contains_raw_user_content: false
```

## Consequences

Positive:

- gate decisions become inspectable
- proof evidence can be checked without remote services
- later SLSA or in-toto mapping remains possible
- dogfooding reports can link evidence instead of narrating trust

Negative:

- hash stability must be specified
- file path normalization matters
- proofpack generation must not silently include sensitive content

## Out of scope

- public transparency logs
- Sigstore/Rekor integration
- SBOM generation
- full SLSA compliance
- release signing
