# Artifact hash policy v0.1

Date: 2026-04-25
Status: proposed boundary
Authority: advisory/design

## Purpose

Define the first artifact hash policy Punk needs before any hash computation, hash normalization, proofpack writer, gate/proof runtime, storage schema, or CLI implementation depends on hashes.

This is a design/spec artifact only.

It protects the user flow:

```text
artifact ref -> artifact digest -> later gate/proof inspection
```

from turning executor claims, unstable paths, missing digests, or narrative summaries into proof.

## Status and authority

This is a proposed v0.1 policy boundary.

It is not implementation.

It does not activate hash computation, hash normalization, schema files, proofpack writing, gate decisions, runtime storage, provider/model/agent adapters, automation, or CLI behavior.

It does not write `.punk/` state.

Future implementation requires a separate bounded goal.

The current Rust proofpack kernel only performs structural ref/digest integrity checks. It does not compute hashes or normalize artifact refs.

## Policy shorthand

```text
refs identify artifacts
hashes verify artifact bytes
missing is visible
executor claims are not proof
hash metadata is evidence, not acceptance
```

A hash can help prove that a referenced artifact has not changed.

A hash does not decide whether work is accepted.

Positive acceptance still requires an accepting gate decision and a matching proofpack under `evals/specs/proof-before-acceptance-semantics.v0.1.md`.

## Supported digest identity v0.1

The initial digest identity is:

```text
sha256:<64 lowercase hex chars>
```

Rules:

- algorithm label is lowercase `sha256`;
- separator is a single colon;
- digest value is exactly 64 lowercase hexadecimal characters;
- uppercase hex is invalid for canonical storage in v0.1;
- labels such as `SHA256`, `sha-256`, `sha512`, `md5`, or bare hex strings are not canonical v0.1 digests;
- placeholder values such as `unknown`, `pending`, `n/a`, empty string, or `sha256:` are not valid digests.

Future algorithms require a later policy/spec revision.

## Artifact ref policy

A future hash record must identify what was hashed with an explicit artifact kind and artifact ref.

Minimum shape:

```yaml
kind: run_receipt
ref: .punk/runs/run_123.json
hash: sha256:0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef
```

This shape is illustrative only. It is not a schema file.

### Artifact kinds

Known v0.1 artifact kinds are:

- `contract`
- `run_receipt`
- `eval_report`
- `semantic_assessment`
- `gate_decision`
- `proofpack`
- `event_range`
- `output_artifact`
- `work_report`
- `rule_set`
- `other`

A future implementation may narrow this vocabulary in code or schema, but it must not silently treat unknown kinds as trusted proof.

### Repo-relative refs

Repo file refs should be normalized as repo-relative paths:

- slash-separated;
- no leading slash;
- no `.` or `..` path segments;
- no empty path segments;
- no home-directory shorthand such as `~`;
- no URL scheme;
- no platform-specific backslashes;
- no absolute filesystem paths;
- case-sensitive;
- no implicit symlink resolution in this policy.

Examples:

```text
work/reports/2026-04-25-example.md
evals/specs/artifact-hash-policy.v0.1.md
.punk/runs/run_123.json
```

Non-examples:

```text
/Users/name/project/work/report.md
../work/report.md
work//report.md
work/./report.md
https://example.com/report.md
~/.punk/runs/run_123.json
```

### Runtime id refs

Some future artifacts may be referenced by stable ids rather than paths, for example `contract_123`, `run_123`, `decision_123`, or an event range.

If a runtime id is used:

- the field name or surrounding artifact must make the ref type explicit;
- implementations must not infer a filesystem path from the id without a separate mapping artifact;
- proof/gate checks must treat an unmapped id as not locally hash-verifiable.

## What bytes are hashed

For v0.1 policy, a digest identifies the exact bytes of the persisted artifact representation.

This policy does not define semantic hashing, Markdown normalization, JSON normalization, line-ending normalization, whitespace normalization, YAML normalization, or archive canonicalization.

If a future schema requires deterministic serialization, the hash should cover the serialized bytes after that schema-defined serialization.

If no deterministic serialization is defined, implementations must not claim stable cross-platform hashes beyond exact stored bytes.

## Missing, invalid, and unverified digests

Hash state should stay explicit.

Recommended future status vocabulary:

- `present` - a canonical digest string is present for the expected kind/ref;
- `missing` - a required digest is absent;
- `invalid_format` - a digest-like value is present but not canonical v0.1 format;
- `unsupported_algorithm` - a digest uses a non-v0.1 algorithm label;
- `ref_mismatch` - a digest exists for a different kind or ref;
- `unverified` - a digest was provided by an executor, adapter, or external tool but not recomputed by Punk;
- `not_required` - policy does not require a digest for this artifact.

Rules:

- missing required digests must remain visible;
- invalid format must not be downgraded to missing without preserving the invalid value as evidence when safe;
- unsupported algorithms do not satisfy v0.1 proof requirements;
- executor-provided digest claims are evidence inputs, not proof;
- a digest for the wrong kind/ref does not satisfy a required digest;
- optional digests may be absent, but absence must not be represented as passing if policy requires them.

## Relationship to current proofpack integrity checks

The current proofpack kernel can structurally check whether declared proofpack refs have matching digest entries by artifact kind and ref.

This policy defines what future digest entries should mean.

It does not change the current kernel.

Current behavior remains:

- structural link/hash integrity only;
- no hash computation;
- no hash normalization;
- no proofpack writer;
- no gate decision writer;
- no runtime storage;
- no CLI behavior.

## Relationship to receipts, evals, gate, and proof

### Run receipts

A receipt may later reference produced artifacts and validator outputs with hashes.

A receipt remains run evidence.

A receipt hash does not make the receipt true or accepted; it only helps verify the referenced receipt artifact identity.

### Eval reports and assessments

Eval reports and semantic assessments may later be hash-referenced as evidence.

They remain assessments, not final decisions.

### Gate decisions

A gate decision may later cite evidence refs and hashes.

Only `gate` writes the final decision.

A hash mismatch or missing required hash may block or qualify a decision according to future gate policy, but this policy does not implement gate behavior.

### Proofpacks

A proofpack may later reference contract, receipt, eval, decision, event, output artifact, and rule-set hashes.

A proofpack is post-gate provenance.

A proofpack must not hide missing required digests.

A proofpack must not become the decision authority.

### Project Memory

Project Memory links may later cite hash-referenceable artifacts.

Derived views or `latest` pointers must stay rebuildable and must not become hash authority.

## Privacy and safety boundary

A hash is not automatically safe metadata.

Hashes can leak information when the source content is small, guessable, proprietary, or sensitive.

Future implementations should avoid hashing or exposing secrets, raw provider payloads, environment dumps, credentials, private user text, or sensitive generated artifacts unless the contract/policy explicitly allows that artifact to be referenced.

Proofpacks should prefer refs and hashes over raw bodies, but a hash reference does not remove the need for artifact privacy policy.

## Required deterministic eval cases

### AHP-001: canonical sha256 digest accepted

A digest string shaped as `sha256:` plus 64 lowercase hex chars is canonical v0.1 digest metadata.

### AHP-002: uppercase or bare digest rejected

Uppercase algorithm labels, uppercase hex, bare hex strings, and alternate labels do not satisfy canonical v0.1 digest requirements.

### AHP-003: absolute paths rejected

Artifact refs with absolute filesystem paths must not satisfy repo-relative ref requirements.

### AHP-004: parent traversal rejected

Artifact refs containing `..`, empty segments, or `.` path segments must not satisfy repo-relative ref requirements.

### AHP-005: missing required digest remains visible

If policy requires a digest for a ref, absence must produce a visible missing-digest state rather than a passing state.

### AHP-006: wrong kind/ref digest does not satisfy requirement

A digest for a different artifact kind or ref must not satisfy the required digest for another artifact.

### AHP-007: executor digest claim is not proof

A digest reported only by an executor or adapter must be marked unverified unless Punk or a trusted local validator recomputes it under policy.

### AHP-008: exact bytes boundary preserved

Without a schema-defined serialization, hash policy must not claim semantic, Markdown, JSON, line-ending, or whitespace normalization.

### AHP-009: proofpack does not hide digest gaps

A proofpack or proof readiness check must preserve required missing, invalid, unsupported, or unverified digest gaps.

### AHP-010: no runtime activation implied

Docs and reports that reference this policy must not describe hash computation, proofpack writing, gate decisions, `.punk/` storage, schemas, CLI commands, adapters, automation, or `punk init` as active implementation.

## Non-goals

This v0.1 policy does not define:

- hash computation implementation;
- hashing library/dependency choice;
- Rust API shape;
- schema files;
- proofpack writer behavior;
- gate decision writer behavior;
- `.punk/` storage layout;
- eval report storage;
- receipt runtime storage;
- semantic assessor implementation;
- CLI commands;
- `punk init`;
- provider/model/agent adapters;
- automation;
- signing;
- transparency logs;
- archive canonicalization;
- SLSA or in-toto mapping.

## Future implementation constraints

Any later implementation step should be explicit about:

- whether it computes hashes or only validates provided digest strings;
- which artifact kinds it supports;
- whether it accepts runtime ids, repo paths, or both;
- exact byte boundary and serialization rules;
- missing/invalid/unverified digest behavior;
- how hash mismatches affect gate decisions and proof readiness;
- how privacy-sensitive artifacts are excluded or redacted;
- how derived views remain non-authoritative.

## Deferred for later specs or goals

Still deferred after this policy:

- active hash computation;
- hash normalization implementation;
- proofpack writer implementation;
- decision/proof schema files;
- real `.punk` runtime storage;
- gate/eval/proof orchestration;
- semantic assessor implementation;
- possible future `punk init`.
