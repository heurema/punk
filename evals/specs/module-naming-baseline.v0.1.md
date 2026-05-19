# Module Naming Baseline v0.1

Date: 2026-05-19
Status: proposed boundary
Authority: advisory/design

## Purpose

Define deterministic expectations for naming Punk domain modules so product
docs, module ids, crate names, directory paths, imports, and future aliases do
not drift.

This spec does not activate Module Host runtime, module loading, provider
orchestration, public CLI behavior, publishing behavior, metrics collection,
receipt writing, gate writing, proofpack writing, or acceptance claims.

## Required deterministic eval cases

### MODULE-NAMING-001: canonical root is declared

Every domain module must declare one canonical root. PubPunk's root is
`pubpunk`.

### MODULE-NAMING-002: module id uses the root

The module id must be the canonical root in stable slug form. PubPunk uses
`pubpunk`.

### MODULE-NAMING-003: product name derives from the root

The product/module prose name must derive from the canonical root. PubPunk uses
`PubPunk`.

### MODULE-NAMING-004: domain module crate derives from the id

Domain module crates must use `punk-mod-<module-id>`. PubPunk uses
`punk-mod-pubpunk`.

### MODULE-NAMING-005: crate directory matches the crate name

Domain module crate directories should match the crate package name unless a
separate migration note records a temporary exception.

### MODULE-NAMING-006: Rust import derives from the crate

Rust imports must use the crate's snake-case form. `punk-mod-pubpunk` imports
as `punk_mod_pubpunk`.

### MODULE-NAMING-007: short aliases are non-canonical until selected

Short aliases such as `pub` may be proposed for future CLI or UX surfaces, but
they must not become canonical module ids, crate names, or documentation roots
unless a later work order selects them.

### MODULE-NAMING-008: infrastructure crates are excluded

Infrastructure crates such as `punk-module-host` are not domain modules and do
not need the `punk-mod-<module-id>` naming pattern.

### MODULE-NAMING-009: historical names are explicit

If a crate or module name is renamed, the cleanup report must record the old
name as historical and update current docs/status refs to the new name.

### MODULE-NAMING-010: behavior does not change

A naming cleanup must not change runtime behavior, public CLI behavior, module
authority, side-effect policy, publishing behavior, metrics collection,
receipt writing, gate/proof behavior, or acceptance claims.

## Expected advisory result

```yaml
module_naming_pubpunk_result:
  status: done
  canonical_root: pubpunk
  product_name: PubPunk
  module_id: pubpunk
  crate_name: punk-mod-pubpunk
  crate_dir: crates/punk-mod-pubpunk
  rust_import: punk_mod_pubpunk
  historical_names:
    - punk-mod-pub
  short_aliases:
    pub: proposed_only
  behavior_changed: false
  non_authority: true
```

## Non-goals

This spec does not define module runtime behavior, manifest parsing,
deterministic checker implementation, module registry behavior, Module Host
runtime, PubPunk runtime, workspace creation, instruction generation, adapter
behavior, publishing behavior, metrics collection, gate writing, proofpack
writing, or acceptance claims.
