# Codebase Study

Status: parked / boundary only

## Purpose

Codebase Study is the future Punk module for bounded source observation.

Canonical module id:

```text
codebase-study
```

Prose name:

```text
Codebase Study
```

This module may later turn an explicit source observation request into an
advisory source inventory observation packet.

The Unix-style module shape is:

```text
explicit source observation request
  -> codebase-study module
  -> advisory source inventory observation packet
```

Public or shorter aliases such as `CodePunk`, `SourcePunk`, or `code` are not
canonical in this boundary.

## Current state

Codebase Study is not active as a module execution surface, CLI, scanner,
reader, crawler, indexer, manifest builder, adapter, or runtime behavior.

There is no `punk-mod-codebase-study` crate, module manifest, Module Host
execution path, `.punk` storage path, scanner, file walker, content reader,
hash collector, size collector, benchmark runner, or CLI command.

This document defines the module boundary before any implementation.

## Boundary model

Codebase Study is a domain module. It is not Brownfield core.

Brownfield core may define observation packet and handoff contracts. It must
not silently absorb codebase study into an implicit core scanner.

The Source Corpus Manifest model track may consume an accepted-for-handoff
advisory observation packet. It must not run Codebase Study, scan the
repository, walk directories, read source contents, compute source filesystem
hashes, collect sizes, or infer observations itself.

Codebase Study output is advisory evidence until host validation and later
gate/proof paths apply. It cannot own project truth, write final decisions,
approve contracts, write proof, claim acceptance, or promote benchmark results
to authority.

## Inputs

A future Codebase Study invocation must require an explicit source observation
request.

The request must be caller-supplied and bounded before any module execution is
considered. It must identify the intended observation scope through explicit
refs, capability requests, any later selected capability grants as separate
refs, privacy policy, expected output authority, and downstream handoff intent.

The request must not rely on ambient repo discovery, implicit current-working-
directory traversal, hidden provider context, provider-specific prompt state,
or local lab state.

Requests for autonomous discovery such as "find all Rust files" are not valid
Codebase Study input in this boundary. They need a later explicit traversal
boundary before implementation.

### Source observation request packet

The source observation request packet is the future caller-supplied envelope
for Codebase Study input.

It is request shape only. It is not a capability grant, runtime schema, parser,
module invocation, scanner plan, file walker plan, content-read approval,
manifest-generation request, `.punk` write request, lab run, benchmark result,
gate decision, proof, or acceptance.

Required request-packet surfaces:

- packet id and schema version;
- `module_id = codebase-study`;
- requested operation kind;
- requester or goal ref;
- explicit source scope refs and exclude refs;
- requested observation categories;
- capability requests, kept separate from grants;
- privacy policy ref, redaction policy ref, or explicit blockers;
- expected output authority;
- downstream handoff intent;
- evaluation requirement refs or blockers;
- warnings, limitations, blockers, or refusal state;
- `non_authority = true`.

The packet must keep:

```text
capability_grants = []
authority = request_only
```

It may request future capabilities such as scoped path observation over
caller-supplied refs. It must not treat those requests as approval to scan,
walk directories, read contents, compute hashes, collect sizes, import lab
code, run benchmarks, write storage, or make authority claims.

Invalid request-packet inputs include:

- ambient repository discovery;
- implicit current-working-directory traversal;
- "auto-discover all Rust files";
- "read these source files and summarize them";
- source snippets, document excerpts, raw environment values, or secret-like
  values;
- source filesystem hashing or size collection;
- Source Corpus Manifest generation;
- runtime `.punk` storage or event-log mutation;
- lab execution or benchmark authority;
- gate decisions, proof writing, or acceptance claims.

The deterministic eval target for this boundary is
`evals/specs/codebase-study-source-observation-request-packet.v0.1.md`.

## Outputs

A future Codebase Study invocation may return only an advisory source inventory
observation packet.

The packet may support the Brownfield observation packet boundary in
`docs/product/BROWNFIELD-INVENTORY.md` and the Source Corpus Manifest handoff
boundary in `docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md`.

The packet must keep:

```text
packet_status = advisory
authority = observed_structure
```

The packet is not project truth, a Source Corpus Manifest, a claim ledger, a
contract, a gate decision, proof, acceptance, or benchmark authority.

## Failure modes

Codebase Study must fail closed when a request asks for behavior outside this
boundary.

Allowed failure surfaces are:

- blockers;
- limitations;
- missing evidence;
- refusal;
- advisory uncertainty.

Failure output must not become a reason to guess, widen scope, traverse
unrequested paths, read contents, cache results, assemble a manifest, create
claims, or promote authority.

## Denied capabilities

Denied by default:

- repo scanning;
- file walking;
- source content reading;
- source filesystem hashing;
- size collection;
- manifest generation from repository state;
- source indexing;
- claim extraction;
- architecture recovery;
- intent recovery;
- AI summaries;
- runtime `.punk` storage;
- direct event-log mutation;
- CLI behavior;
- module execution;
- lab code import;
- benchmark execution;
- adapter invocation;
- network access;
- credential access;
- gate decisions;
- proof writing;
- acceptance claims.

Any future grant requires a later bounded goal, conformance review, and eval
coverage before implementation.

## Relationship to labs

`code-intel-kernel`, `agent-bench-lab`, Claude, agy, and vibe passes are
advisory inputs only.

They may inform boundary questions, risk lists, evaluation requirements, and
future conformance criteria. They are not product truth, implementation
authority, gate decisions, proof, acceptance, or benchmark authority.

Future active Codebase Study results need an `agent-bench-lab` evaluation route
before those results influence Brownfield decisions.

## Future conformance direction

Before implementation, Codebase Study must have a module conformance packet or
an explicit no-packet rationale.

The conformance packet must cite:

- Module Authoring Baseline;
- Module Host Contract Stub;
- Brownfield observation packet boundary;
- Source Corpus Manifest handoff boundary;
- Codebase Study module boundary eval spec.

## Non-goals

This v0.1 boundary does not define or activate Codebase Study module execution,
Module Host runtime, module manifest schema, module SDK, plugin packaging, repo
scanning, file walking, source content reading, source filesystem hashing, size
collection, manifest generation from repository state, source indexing, claim
extraction, AI summaries, runtime `.punk` storage, CLI behavior, lab code
import, benchmark suite execution, gate/proof authority, or acceptance.
