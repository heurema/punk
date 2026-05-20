---
id: report_2026_05_20_pubpunk_publish_request_packet_v0_1
kind: work-report
status: done
authority: canonical
owner: vitaly
created_at: 2026-05-20
updated_at: 2026-05-20
related_goal: work/goals/goal_add_pubpunk_publish_request_packet_v0_1.md
---

# PubPunk Publish Request Packet v0.1

## Summary

Added a side-effect-free PubPunk publish request packet model and smoke evidence
that chains it into the existing Module Host side-effect request proposal and
policy-gate preflight models.

## Verdict

The first PubPunk publish request packet is now covered as local advisory
evidence only. It prepares explicit refs for a future external publish request
without activating publishing runtime or adapter behavior.

```yaml
pubpunk_publish_request_packet_result:
  status: done
  candidate_ref_explicit: true
  channel_ref_explicit: true
  request_external_publish_grant_required: true
  side_effect_request_refs_ready: true
  module_host_side_effect_request_proposal_ready: true
  module_host_policy_gate_preflight_ready: true
  receipt_writer_active: false
  adapter_invocation_active: false
  external_publish_active: false
  side_effects: false
  non_authority: true
```

## What changed

- Added `PubPunkPublishRequestPacket`, readiness assessment, findings, and
  `PubPunkPublishSideEffectRequestRefs`.
- The packet requires explicit candidate, channel, side-effect request, intent,
  policy, adapter, payload, receipt proposal, instruction, allowed-source,
  expected-receipt, privacy, and optional token-cost refs.
- Added unit coverage for ready packets, missing/unsupported capabilities,
  unallowed refs, unsafe refs, and unsafe privacy.
- Added
  `eval_pubpunk_publish_request_packet_chains_to_host_side_effect_request`.
- Added `evals/specs/pubpunk-publish-request-packet.v0.1.md`.
- Updated PubPunk docs, crate status, documentation map, and work status.

## Boundary confirmation

- No PubPunk runtime was activated.
- No Module Host runtime was activated.
- No module invocation, plugin loading, or dynamic dispatch was added.
- No public CLI behavior was added.
- No workspace initialization, filesystem scanning, draft body read behavior,
  publishing behavior, or metrics collection was added.
- No provider orchestration or automatic token collection was added.
- No adapter invocation, external publishing, receipt writer, gate writer,
  proofpack writer, or acceptance claim was added.
- `selected_next` remains
  `work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md`.

## Provider review

```yaml
provider_review:
  status: completed_before_slice
  passes:
    - provider: claude_sonnet
      verdict: "side-effect request packet is the right next small slice"
    - provider: claude_opus
      verdict: "keep the packet as refs and preflight evidence only"
    - provider: vibe_mistral
      verdict: "do not activate publishing; chain to existing host policy/request models"
  blockers: []
```

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
    - pass_id: pass_codex_pubpunk_publish_request_packet
      token_source: unavailable
      total_tokens: null
      outcome: selected
```

## Doc impact

```yaml
doc_impact:
  classification: code-doc
  reason: "Adds PubPunk publish request packet evidence without runtime activation."
  touched_surfaces:
    - crates/punk-mod-pubpunk/src/lib.rs
    - crates/punk-eval/src/lib.rs
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/CRATE-STATUS.md
    - docs/product/DOCUMENTATION-MAP.md
    - evals/specs/pubpunk-publish-request-packet.v0.1.md
    - work/STATUS.md
    - work/goals/goal_add_pubpunk_publish_request_packet_v0_1.md
    - work/reports/2026-05-20-pubpunk-publish-request-packet-v0-1.md
  required_updates:
    - docs/modules/pubpunk.md
    - docs/modules/pubpunk-workspace-instructions.md
    - docs/product/CRATE-STATUS.md
    - docs/product/DOCUMENTATION-MAP.md
    - work/STATUS.md
```

## Validation

```text
cargo fmt
PASS

cargo test -p punk-mod-pubpunk
18 passed; 0 failed

cargo test -p punk-eval
6 passed; 0 failed

python3 scripts/check_research_gate.py
Research Gate check: PASS
Selected next: work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md

python3 scripts/check_work_ledger.py
Work ledger check: PASS
Selected next: work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md
Goals checked: 274

python3 scripts/check_docs_governance.py --files crates/punk-mod-pubpunk/src/lib.rs crates/punk-eval/src/lib.rs docs/modules/pubpunk.md docs/modules/pubpunk-workspace-instructions.md docs/product/CRATE-STATUS.md docs/product/DOCUMENTATION-MAP.md evals/specs/pubpunk-publish-request-packet.v0.1.md work/STATUS.md work/goals/goal_add_pubpunk_publish_request_packet_v0_1.md work/reports/2026-05-20-pubpunk-publish-request-packet-v0-1.md --report work/reports/2026-05-20-pubpunk-publish-request-packet-v0-1.md
Docs governance check: PASS
Changed files: 10
Canonical docs checked: 2
Reports checked: 1
Failures: 0
Warnings: 0

cargo check --workspace
Finished `dev` profile [unoptimized + debuginfo] target(s)

cargo run -p punk-cli -- eval run smoke
smoke_result: pass
New case: eval_pubpunk_publish_request_packet_chains_to_host_side_effect_request: pass

git diff --check
PASS
```
