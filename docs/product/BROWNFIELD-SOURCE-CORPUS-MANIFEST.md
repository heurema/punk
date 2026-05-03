---
id: docs_product_brownfield_source_corpus_manifest
kind: product-doc
status: active
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
review_after: 2026-07-20
canonical_for:
  - brownfield-source-corpus-manifest-design
  - brownfield-source-corpus-manifest-schema
  - brownfield-manifest-path-policy
  - brownfield-manifest-content-policy
  - brownfield-manifest-hash-policy
related_docs:
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/ROADMAP.md
  - docs/product/GLOSSARY.md
related_evals:
  - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
supersedes: []
superseded_by: null
---
# Brownfield Source Corpus Manifest

## Purpose

This document defines the B2 design boundary for a future Brownfield Source
Corpus Manifest.

It is design/spec only.

It does not implement repo scanning, file walking, language detection,
manifest writing, hash computation, file content reading, claim extraction,
AI summaries, contracts, gate/proof runtime, Writer behavior, runtime `.punk`
storage, or grayfield reconciliation.

## Manifest status and authority

This section assigns the future manifest role as structured inventory for
observable repo-relative source corpus structure.

It is not:

- project memory truth;
- a claim ledger;
- a contract;
- a decision;
- proof;
- a semantic summary;
- an architecture map;
- intent recovery;
- contract readiness.

The manifest authority model is:

```text
manifest_status = advisory
authority = observed_structure
```

## Manifest schema

Future manifest output should use a stable top-level shape:

```yaml
manifest_id: brownfield-source-corpus-manifest.v0.1:<project_id>:<stable_run_id>
schema_version: brownfield-source-corpus-manifest.v0.1
project_id: <project-id>
entry_mode: brownfield
manifest_status: advisory
authority: observed_structure
created_at: <timestamp>
generated_at_policy: deterministic_run_timestamp
source_root_ref:
  kind: repo_root
  path: .
inventory_scope:
  include:
    - .
  exclude:
    - .git
    - .punk/runtime
    - .punk/cache
    - .punk/indexes
    - node_modules
    - target
excluded_paths: []
items: []
```

The manifest may include timestamps only as run metadata. Timestamps must not
be used as evidence of source intent, freshness, acceptance, or authority.

## Item schema

Each future item should represent one observable structural fact about one
repo-relative path:

```yaml
item_id: <stable-id-derived-from-repo-relative-path-and-kind>
repo_relative_path: crates/example/src/lib.rs
observed_kind: file
source_class: source_code
source_markers:
  - extension: rs
tracking_status: observed
sensitivity: normal
generated_or_vendored_candidate: none
size_policy:
  status: deferred
  value: null
hash_policy:
  status: deferred
  value: null
content_policy:
  read_contents: false
  store_snippets: false
  summarize_contents: false
evidence_ref:
  kind: manifest_observation
  ref: <manifest_id>#<item_id>
notes: []
```

Allowed `observed_kind` values:

```text
file
directory
symlink_candidate
unknown
```

`symlink_candidate` records only that a path appears symlink-like if a future
implementation can observe it safely. The manifest must not expand symlink
targets or record host-absolute target paths.

Allowed `tracking_status` values:

```text
observed
excluded
sensitive_redacted
unknown
```

Allowed `sensitivity` values:

```text
normal
caution
sensitive
excluded
unknown
```

## Path policy

Future manifest output must:

- use repo-relative paths only;
- reject or redact absolute paths;
- reject or redact home/user paths;
- reject parent traversal;
- reject backslash path separators in portable manifest refs;
- normalize `.` and duplicate separators before writing refs;
- avoid symlink target expansion in the manifest;
- fail closed on path escape;
- keep `.punk/runtime`, `.punk/cache`, and `.punk/indexes` out of source
  corpus items.

Manifest paths are inventory refs, not filesystem authority. They must not be
treated as proof that an artifact is canonical, accepted, safe, current, or
project-authored.

## Content policy

Default policy:

```text
read_contents = false
store_snippets = false
summarize_contents = false
```

The manifest must not store file contents, code snippets, environment values,
secret-like values, doc excerpts, or semantic summaries.

Any future content-reading exception requires a separate bounded goal, privacy
review, and explicit schema update.

## Hash policy

Hashing is deferred for the first manifest implementation.

The design permits optional future hash fields, but they are not required for
the first manifest writer.

Default policy:

```text
hash_policy.status = deferred
hash_policy.value = null
```

If a later bounded goal enables hashes, hashes must remain structural evidence
only. They must not imply accepted behavior, contracts, proof, freshness, or
project truth.

## Size policy

File sizes are deferred for the first manifest implementation.

Default policy:

```text
size_policy.status = deferred
size_policy.value = null
```

If later enabled, size must remain structural metadata only and must not be
used for semantic inference.

## Source class policy

Future manifest items may use only B1 source classes:

```text
source_code
docs
tests
ci_config
package_manifest
schema
migration
script
generated_candidate
vendored_candidate
unknown
```

If classification is uncertain, use `unknown`.

The manifest must not infer module purpose, requirements, ownership truth,
architecture, accepted behavior, invariants, or intent from a source class.

## Exclusion policy

Default excluded paths:

```text
.git
.punk/runtime
.punk/cache
.punk/indexes
node_modules
target
```

Default caution categories:

```text
secrets
env_files
local_cache
build_output
private_agent_transcripts
generated_candidate
vendored_candidate
unknown
```

Future implementations may add denylist entries. Removing these defaults
requires a separate boundary update.

## Generated and vendored candidates

Generated and vendored material must be marked as candidates when detected.

Candidate marking is not authorship proof. It is only a caution signal for
later review.

The manifest must not silently treat generated or vendored artifacts as
project-authored truth.

## Claim boundary

Manifest items must not include fields for:

- purpose;
- requirement;
- intent;
- invariant;
- owner truth;
- architecture decision;
- accepted behavior;
- contract readiness conclusion;
- canonical doc status;
- proof status;
- gate decision;
- risk or severity.

Candidate claims belong to a later reviewed reconstruction layer. The manifest
may provide source refs for that layer, but it must not create the claims.

## Privacy and no-network policy

Future manifest generation must be local-only by default.

Required rules:

- no network access;
- no remote AI calls;
- no telemetry upload;
- no hidden sync;
- no remote indexing;
- no raw secrets;
- no raw environment values;
- no absolute host paths;
- no private transcript import.

## Future implementation gates

Before any implementation, a later goal must define:

- deterministic traversal inputs;
- path allowlist and denylist behavior;
- atomic write and no-partial-manifest behavior;
- manifest target path;
- conflict/idempotency policy;
- exact output format;
- fixture expectations.

This document does not select that implementation.

## Non-goals

This design does not implement:

- repo scanning;
- file walking;
- language detection;
- manifest writing;
- hash computation;
- file content reading;
- source corpus manifest generation;
- claim extraction;
- claim ledger population;
- AI summaries;
- LLM calls;
- module maps;
- architecture recovery;
- intent recovery;
- contract generation;
- Contract Context Pack generation;
- Conformance Pack runtime;
- Migration Contract runtime;
- Regenerative Spec behavior;
- gate/proof runtime;
- Writer behavior;
- runtime `.punk` storage;
- grayfield reconciliation.
