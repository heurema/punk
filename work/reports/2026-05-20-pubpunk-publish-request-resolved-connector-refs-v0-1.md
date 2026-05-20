---
id: report_2026_05_20_pubpunk_publish_request_resolved_connector_refs_v0_1
kind: work-report
status: done
authority: canonical
owner: vitaly
created_at: 2026-05-20
updated_at: 2026-05-20
related_goal: work/goals/goal_add_pubpunk_publish_request_resolved_connector_refs_v0_1.md
---

# PubPunk Publish Request Resolved Connector Refs v0.1

## Summary

Extended the side-effect-free PubPunk publish request packet so it requires
resolved connector refs before projecting Module Host side-effect request refs.

## Verdict

The PubPunk publish request packet now consumes the prior channel connector
profile resolution boundary. Direct adapter, channel, or payload refs are not
enough to make a publish request ready.

```yaml
pubpunk_publish_request_resolved_connector_refs_result:
  status: done
  connector_profile_resolution_ref_required: true
  connector_profile_ref_required: true
  selected_connector_strategy_ref_required: true
  connector_refs_must_be_allowed_sources: true
  connector_receipt_fields_required: true
  side_effect_request_refs_include_connector_refs: true
  module_host_chain_preflight_only: true
  pubpunk_calls_api: false
  pubpunk_opens_browser: false
  pubpunk_reads_credentials: false
  adapter_invocation_active: false
  external_publish_active: false
  metrics_collection_active: false
  side_effects_in_pubpunk_model: false
  non_authority: true
```

## What changed

- Added `connector_profile_resolution_ref`, `connector_profile_ref`, and
  `selected_connector_strategy_ref` to `PubPunkPublishRequestPacket`,
  assessment refs, and `PubPunkPublishSideEffectRequestRefs`.
- Publish request readiness now blocks missing, unsafe, or unallowed connector
  resolution/profile/strategy refs.
- Expected receipt fields now require `connector_profile_resolution`,
  `connector_profile_ref`, and `selected_connector_strategy`.
- The publish request smoke case now builds a connector profile resolution
  packet first, then consumes its connector profile ref before entering the
  Module Host side-effect request and policy-gate preflight chain.
- Updated PubPunk docs, workspace instructions, crate status, and the publish
  request eval spec.

## Boundary confirmation

- No PubPunk runtime was activated.
- No Module Host runtime was activated.
- No module invocation, plugin loading, or dynamic dispatch was added.
- No public CLI behavior was added.
- No workspace initialization, filesystem scanning, draft body read behavior,
  publishing behavior, metrics collection, receipt writing, or event-log
  mutation was added.
- No API call, browser automation, credential read, adapter invocation,
  external publishing, gate writer, proofpack writer, or acceptance claim was
  added.
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
    - pass_id: pass_codex_pubpunk_publish_request_resolved_connector_refs
      token_source: unavailable
      total_tokens: null
      outcome: selected_implementation
```

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Extends PubPunk publish request readiness to require resolved connector refs without PubPunk runtime activation, browser/API calls, adapter invocation, metrics collection, or external publishing."
  touched_surfaces:
    - crates/punk-mod-pubpunk/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/pubpunk-publish-request-packet.v0.1.md
    - work/STATUS.md
    - work/goals/goal_add_pubpunk_publish_request_resolved_connector_refs_v0_1.md
    - work/reports/2026-05-20-pubpunk-publish-request-resolved-connector-refs-v0-1.md
  required_updates:
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/CRATE-STATUS.md
    - evals/specs/pubpunk-publish-request-packet.v0.1.md
    - work/STATUS.md
```

## Validation

```text
cargo fmt
PASS

cargo test -p punk-mod-pubpunk
42 passed; 0 failed

cargo test -p punk-eval
6 passed; 0 failed

python3 scripts/check_research_gate.py
Research Gate check: PASS
Selected next: work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md

python3 scripts/check_work_ledger.py
Work ledger check: PASS
Selected next: work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md
Goals checked: 281

python3 scripts/check_docs_governance.py --files crates/punk-mod-pubpunk/src/lib.rs crates/punk-eval/src/lib.rs docs/modules/pubpunk.md docs/modules/pubpunk-workspace-instructions.md docs/product/CRATE-STATUS.md evals/specs/pubpunk-publish-request-packet.v0.1.md work/STATUS.md work/goals/goal_add_pubpunk_publish_request_resolved_connector_refs_v0_1.md work/reports/2026-05-20-pubpunk-publish-request-resolved-connector-refs-v0-1.md --report work/reports/2026-05-20-pubpunk-publish-request-resolved-connector-refs-v0-1.md
Docs governance check: PASS
Changed files: 9
Canonical docs checked: 1
Reports checked: 1
Failures: 0
Warnings: 0

cargo check --workspace
Finished `dev` profile [unoptimized + debuginfo] target(s)

cargo run -p punk-cli -- eval run smoke
smoke_result: pass
Case `eval_pubpunk_publish_request_packet_chains_to_host_side_effect_request`: pass

git diff --check
PASS
```
