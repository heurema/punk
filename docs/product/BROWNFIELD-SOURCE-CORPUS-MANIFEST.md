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
  - brownfield-source-corpus-manifest-writer-boundary
  - brownfield-source-corpus-manifest-writer-implementation-boundary
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

It also defines the future source corpus manifest writer boundary and writer
implementation boundary before any writer implementation is selected.

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

## Source corpus manifest writer boundary

The source corpus manifest writer boundary describes how a future writer may
persist a manifest. It does not implement the writer.

In this section, `writer` means only a future source corpus manifest file
writer. It does not mean Punk `Writer` behavior, orchestration, acceptance
claim writing, gate writing, proof writing, or runtime storage.

### Target path policy

Future source corpus manifest writes are limited to:

```text
.punk/memory/reconstruction/source-corpus-manifest.md
```

or to a clearly configured repo-relative target under:

```text
.punk/memory/reconstruction/
```

It must not write outside the repository, follow symlinks out of the
repository, infer hidden target paths, or use absolute host paths as output
targets.

### Writer preflight policy

Before writing, a future writer must preflight the full operation:

- target path is repo-relative and under `.punk/memory/reconstruction/`;
- target parent directory is valid;
- no path escape or parent traversal is present;
- no symlink ancestor can redirect the write outside the repository;
- target conflict policy is known before bytes are written;
- manifest authority is `observed_structure`;
- manifest status is `advisory`;
- manifest data contains no claim fields;
- manifest data contains no file contents, snippets, summaries, raw secrets, or
  raw environment values;
- manifest data contains no absolute host paths.

If any preflight condition fails, the future writer must fail closed before
modifying the target.

### Atomicity policy

Future writer behavior must not leave partial manifest files.

The expected implementation shape is:

- write complete bytes to a temporary file in the same target directory;
- flush or fsync if a later implementation goal requires durability semantics;
- atomically rename into place when the platform supports it;
- fail closed without modifying the target when atomic replacement cannot be
  guaranteed.

This policy is not implemented by this document.

### Conflict policy

For an existing target:

- missing target may be written after clean preflight;
- identical content may be treated as idempotent;
- different existing content must block unless a later bounded goal adds an
  explicit overwrite flag and review boundary.

No overwrite flag is selected by this boundary.

### Writer content and privacy policy

A future writer may only write manifest data already allowed by this document
and the side-effect-free model.

It must not read file contents, store snippets, store summaries, store secrets,
store raw environment values, store absolute paths, use network access, call
remote AI, or import private agent transcripts.

### Writer claim boundary

Future writer preflight must reject manifest data that contains claim-like
fields such as:

```text
intent
requirement
module_purpose
architecture_decision
accepted_behavior
invariant
contract
proof
claims_created
project_truth
```

The manifest writer may persist observed structure only. It must not create a
claim ledger, decide contract readiness, accept behavior, or promote source
corpus observations into project truth.

### Operation evidence boundary

If a future writer emits operation evidence, that evidence describes only the
write attempt and outcome.

Operation evidence is not proof, not a gate decision, not acceptance, not a
contract, and not project truth.

### Runtime storage boundary

Future source corpus manifest writer behavior must not activate or write:

```text
.punk/runtime
.punk/events
.punk/runs
.punk/decisions
.punk/proofs
```

Persisting a tracked source corpus manifest under `.punk/memory/reconstruction/`
must not be treated as runtime storage activation.

## Source corpus manifest writer implementation boundary

The source corpus manifest writer implementation boundary defines the smallest
future write slice that may later be selected. It does not implement that
slice.

The allowed future slice is:

```text
take an already-constructed SourceCorpusManifest model
render deterministic canonical manifest bytes
require a successful preflight result
write exactly one safe target
emit non-authoritative operation evidence
```

It must not scan repositories, walk directories, read source file contents,
compute source file hashes from the filesystem, generate manifest items, infer
source classes, generate claims, call AI, write runtime storage, or write
gate/proof artifacts.

### Future writer input boundary

A future writer may accept only:

- an already-constructed `SourceCorpusManifest` model;
- an explicit target path or the default safe target;
- an explicit preflight result for the same model and target.

It must not accept:

- a repository root to scan;
- directory lists to walk;
- raw source files to inspect;
- an AI prompt;
- claim ledger input.

### Future target path boundary

The default future target remains:

```text
.punk/memory/reconstruction/source-corpus-manifest.md
```

A configured target is allowed only when it is repo-relative and under:

```text
.punk/memory/reconstruction/
```

Future writer behavior must reject absolute targets, path escape, symlink
escape, hidden inferred targets, and writes to runtime or derived state
surfaces including:

```text
.punk/runtime
.punk/events
.punk/runs
.punk/decisions
.punk/proofs
.punk/cache
.punk/indexes
```

### Future preflight requirement before writing

The writer must require a successful preflight result before writing bytes.

No write is allowed when preflight contains blocking findings. Blocking
findings include:

```text
absolute target
path escape
symlink escape
different existing target
unknown target state
missing or unknown parent
non-advisory manifest
non-observed_structure authority
claim-like fields
absolute paths in manifest
content snippets
summaries
runtime storage target
```

The future writer must not treat operation evidence findings as proof, gate
decision, acceptance, or project truth.

### Future render boundary

Future rendering must produce deterministic canonical bytes from the supplied
manifest model.

Required rendering rules:

- stable field order;
- no hidden timestamps;
- no runtime clock reads unless a later bounded goal explicitly allows clock
  injection;
- no host paths;
- no environment values;
- no local usernames;
- no raw file contents.

If timestamps are needed, they must come from manifest model input unless a
later bounded goal explicitly changes the clock boundary.

### Future atomic write boundary

Future writing must avoid partial target state.

Required write rules:

- write a temporary file in the same target directory;
- flush or fsync only under an explicit later durability policy;
- use atomic rename where platform-compatible;
- leave no partial target on failure;
- block different existing target content unless a later reviewed overwrite
  boundary is selected;
- treat identical existing content as an idempotent outcome.

### Future operation evidence boundary

Future operation evidence may record only:

```text
attempted
blocked
written
idempotent
conflict
error
```

It must not be proof, gate decision, acceptance, project truth, claim ledger,
or contract readiness.

### Future authority, privacy, and runtime boundary

A written manifest remains:

```text
manifest_status = advisory
authority = observed_structure
```

Writing it does not promote it, create claims, decide contract readiness, or
create contract inputs.

Future writer output must not include absolute paths, home/user paths, raw
environment values, secrets, file contents, private agent transcripts, or AI
summaries.

Future writer behavior must not activate `.punk/runtime`, `.punk/events`,
`.punk/runs`, `.punk/decisions`, `.punk/proofs`, gate/proof runtime, Punk
`Writer` behavior, Conformance Pack runtime, Migration Contract runtime,
Regenerative Spec behavior, or spec-as-source behavior.

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
