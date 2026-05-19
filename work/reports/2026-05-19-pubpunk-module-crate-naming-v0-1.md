---
id: report_2026_05_19_pubpunk_module_crate_naming_v0_1
kind: work-report
status: done
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
related_goal: work/goals/goal_normalize_pubpunk_module_crate_naming_v0_1.md
---

# PubPunk Module Crate Naming v0.1

## Summary

Normalized PubPunk naming around the canonical root `pubpunk`.

## Verdict

Use a mechanically derived naming stack for domain modules.

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

## Provider advisory pass

| Provider | Recommendation | Adopted? | Reason |
|---|---|---|---|
| Vibe/Mistral | Use shorter product name `Pub`, keep crate `punk-mod-pub`. | No | `Pub` is too terse and ambiguous as a canonical product/module name. |
| Gemini | Use root `publish`, rename to `punk-publish` / Publish Module. | No | Generic naming loses the Punk module/persona convention and breaks the existing `pubpunk` module id/docs path. |
| Claude | Keep `PubPunk`/`pubpunk`, rename crate to `punk-mod-pubpunk`. | Yes | Preserves product/module language while making crate, directory, and import mechanically derivable from the module id. |

## Naming policy

Domain modules should declare one canonical root and derive names from it:

| Surface | Rule | PubPunk value |
|---|---|---|
| Canonical root | stable module root | `pubpunk` |
| Product/module prose name | product-owned PascalCase | `PubPunk` |
| Module id | canonical root in slug form | `pubpunk` |
| Module docs path | `docs/modules/<module-id>.md` | `docs/modules/pubpunk.md` |
| Rust crate/package | `punk-mod-<module-id>` | `punk-mod-pubpunk` |
| Rust import | crate name as snake case | `punk_mod_pubpunk` |
| Short alias | future UX/CLI alias only after selection | `pub` proposed-only |

Infrastructure crates such as `punk-module-host` are not domain modules and do
not need the `punk-mod-<module-id>` pattern.

## What changed

- Renamed the PubPunk crate package from `punk-mod-pub` to
  `punk-mod-pubpunk`.
- Renamed the crate directory from `crates/punk-mod-pub/` to
  `crates/punk-mod-pubpunk/`.
- Updated the `punk-eval` dependency/import from `punk_mod_pub` to
  `punk_mod_pubpunk`.
- Updated current docs, current eval refs, and work status to the normalized
  name.
- Added `evals/specs/module-naming-baseline.v0.1.md`.

Historical reports and older completed goals may still mention
`punk-mod-pub` as the name used when those artifacts were created. Current
refs should use `punk-mod-pubpunk`.

## Boundary confirmation

- No PubPunk model behavior was changed.
- No public CLI behavior was added.
- No PubPunk runtime was activated.
- No Module Host runtime was activated.
- No module invocation, provider orchestration, adapter invocation, workspace
  initializer, publishing behavior, metrics collection, receipt writer, gate
  writer, proofpack writer, or acceptance claim was added.
- `selected_next` remains
  `work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md`.

## Cost accounting

```yaml
cost_accounting:
  status: unavailable
  total_tokens: null
  accepted_tokens: null
  rejected_tokens: null
  discarded_tokens: null
  unknown_tokens: null
  pass_costs:
    - pass_id: pass_claude_pubpunk_naming
      token_source: unavailable
      total_tokens: null
      outcome: selected
    - pass_id: pass_gemini_pubpunk_naming
      token_source: unavailable
      total_tokens: null
      outcome: rejected
    - pass_id: pass_vibe_mistral_pubpunk_naming
      token_source: unavailable
      total_tokens: null
      outcome: rejected
```

## Doc impact
```yaml
doc_impact:
  classification: code-doc
  reason: "Renames the incubating PubPunk crate/package/path and codifies domain-module naming rules without behavior changes."
  touched_surfaces:
    - Cargo.toml
    - Cargo.lock
    - README.md
    - crates/punk-mod-pubpunk/Cargo.toml
    - crates/punk-mod-pubpunk/src/lib.rs
    - crates/punk-eval/Cargo.toml
    - crates/punk-eval/src/lib.rs
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/MODULE-AUTHORING.md
    - docs/product/MODULES.md
    - docs/product/CRATE-STATUS.md
    - docs/product/DOCUMENTATION-MAP.md
    - evals/cases/project/eval_project_no_parked_capability_exposed.yaml
    - evals/specs/module-naming-baseline.v0.1.md
    - evals/specs/pubpunk-workspace-instruction-packet.v0.1.md
    - work/STATUS.md
    - work/goals/goal_normalize_pubpunk_module_crate_naming_v0_1.md
    - work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md
  required_updates:
    - docs/product/MODULE-AUTHORING.md
    - docs/product/CRATE-STATUS.md
    - docs/product/MODULES.md
    - work/STATUS.md
```

## Validation

```text
cargo check --workspace
Finished `dev` profile [unoptimized + debuginfo] target(s)

cargo run -p punk-cli -- eval run smoke
smoke_result: pass

python3 scripts/check_research_gate.py
Research Gate check: PASS
Selected next: work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md

python3 scripts/check_work_ledger.py
Work ledger check: PASS
Selected next: work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md
Goals checked: 270

python3 scripts/check_docs_governance.py --files README.md Cargo.toml Cargo.lock crates/punk-eval/Cargo.toml crates/punk-eval/src/lib.rs crates/punk-mod-pubpunk/Cargo.toml crates/punk-mod-pubpunk/src/lib.rs docs/modules/pubpunk.md docs/modules/pubpunk-workspace-instructions.md docs/product/MODULE-AUTHORING.md docs/product/MODULES.md docs/product/CRATE-STATUS.md docs/product/DOCUMENTATION-MAP.md evals/cases/project/eval_project_no_parked_capability_exposed.yaml evals/specs/module-naming-baseline.v0.1.md evals/specs/pubpunk-workspace-instruction-packet.v0.1.md work/STATUS.md work/goals/goal_normalize_pubpunk_module_crate_naming_v0_1.md work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md --report work/reports/2026-05-19-pubpunk-module-crate-naming-v0-1.md
Docs governance check: PASS
Changed files: 19
Canonical docs checked: 4
Reports checked: 1
Failures: 0
Warnings: 0

git diff --check
PASS
```
