---
id: report_2026_05_20_pubpunk_inventory_reader_v0_1
kind: work-report
status: done
authority: canonical
owner: vitaly
created_at: 2026-05-20
updated_at: 2026-05-20
related_goal: work/goals/goal_add_pubpunk_inventory_reader_v0_1.md
---

# PubPunk Inventory Reader v0.1

## Summary

Added a side-effect-free PubPunk inventory reader model before the existing
input packet.

## Verdict

The reader is the first deterministic metadata-reader boundary for PubPunk. It
does not read the filesystem. It accepts explicit observed refs, validates them
against allowed source refs, and builds `PubPunkInventoryInputPacket` only when
ready.

```yaml
pubpunk_inventory_reader_result:
  status: done
  module_id: pubpunk
  workspace_policy: split_explicit_refs
  capability_grant: read_workspace_metadata
  observed_refs_must_be_allowed: true
  empty_inventory_allowed: true
  builds_input_packet_when_ready: true
  runtime_activation: false
  filesystem_scanning: false
  draft_body_reads: false
  side_effects: false
  non_authority: true
```

## What changed

- Added `PubPunkInventoryReaderInput`.
- Added `PubPunkInventoryReaderAssessment`.
- Added `assess_pubpunk_inventory_reader_input`.
- Added `build_pubpunk_inventory_input_packet_from_reader_input`.
- Added unit coverage for ready reader input, empty new-project inventory,
  unallowed observed refs, raw bodies, unsupported capabilities, and unsafe
  refs.
- Added a smoke eval case for the reader boundary.
- Added `evals/specs/pubpunk-inventory-reader.v0.1.md`.
- Updated PubPunk docs, crate status, documentation map, and work status.

## Boundary confirmation

- No PubPunk runtime was activated.
- No Module Host runtime was activated.
- No public CLI behavior was added.
- No workspace initialization, filesystem scanning, or draft body read behavior
  was added.
- No provider orchestration or token collection was added.
- No adapter invocation, external publishing, metrics collection, receipt
  writer, gate writer, proofpack writer, or acceptance claim was added.
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
    - pass_id: pass_codex_pubpunk_inventory_reader
      token_source: unavailable
      total_tokens: null
      outcome: selected
```

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds an incubating PubPunk inventory reader model and updates docs/eval/work-ledger boundaries without runtime activation."
  touched_surfaces:
    - crates/punk-mod-pubpunk/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/CRATE-STATUS.md
    - docs/product/DOCUMENTATION-MAP.md
    - evals/specs/pubpunk-inventory-reader.v0.1.md
    - work/STATUS.md
    - work/goals/goal_add_pubpunk_inventory_reader_v0_1.md
    - work/reports/2026-05-20-pubpunk-inventory-reader-v0-1.md
  required_updates:
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/CRATE-STATUS.md
    - docs/product/DOCUMENTATION-MAP.md
    - work/STATUS.md
```

## Validation

```text
cargo fmt: PASS
cargo test -p punk-mod-pubpunk: PASS
cargo test -p punk-eval: PASS
python3 scripts/check_research_gate.py: PASS
python3 scripts/check_work_ledger.py: PASS
python3 scripts/check_docs_governance.py --files docs/modules/pubpunk.md docs/modules/pubpunk-workspace-instructions.md docs/product/CRATE-STATUS.md docs/product/DOCUMENTATION-MAP.md evals/specs/pubpunk-inventory-reader.v0.1.md work/STATUS.md work/goals/goal_add_pubpunk_inventory_reader_v0_1.md work/reports/2026-05-20-pubpunk-inventory-reader-v0-1.md --report work/reports/2026-05-20-pubpunk-inventory-reader-v0-1.md: PASS, Warnings: 0
git diff --check: PASS
cargo check --workspace: PASS
cargo run -p punk-cli -- eval run smoke: PASS
```
