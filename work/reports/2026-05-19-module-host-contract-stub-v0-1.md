---
id: report_2026_05_19_module_host_contract_stub_v0_1
kind: work-report
status: done
authority: canonical
owner: vitaly
created_at: 2026-05-19
updated_at: 2026-05-19
related_goal: work/goals/goal_define_module_host_contract_stub_v0_1.md
---

# Module Host Contract Stub v0.1

## Summary

Defined Module Host Contract Stub v0.1 as a docs/eval-only boundary before
PubPunk or DevPunk skeleton work.

## Why

The Module Conformance Packet checks module readiness, but a module still needs
a shared host-facing contract target. Without that, PubPunk could become the
first module and accidentally invent its own input, capability, side-effect,
or output boundary.

The slice also records deliberation token-cost accounting so future
multi-provider decisions can track accepted, rejected, discarded, and unknown
token spend without making token spend authoritative.

## Advisory review input

Three independent advisory passes reviewed whether PubPunk can be first:

- Claude: PubPunk is a valid conformance candidate, but a host contract stub
  and trivial read-only dry run should precede the first PubPunk skeleton.
- Gemini: PubPunk can be first after conformance packet validation if workspace
  policy and baseline metadata are complete.
- Vibe/Mistral: PubPunk can be first after the conformance packet; DevPunk and
  unparked Module Host runtime are nice-to-have, not blockers.

## Synthesis

- PubPunk remains the first urgent business-module target.
- PubPunk should not be the first artifact to invent host contract shape.
- The smallest missing layer is a docs/eval host contract stub, not Module Host
  runtime and not DevPunk implementation.
- A trivial read-only dry run may come before PubPunk skeleton to test the
  conformance process without publishing urgency.

## What changed

- Added `docs/product/MODULE-HOST-CONTRACT.md`.
- Added `evals/specs/module-host-contract-stub.v0.1.md`.
- Added a Documentation Map canonical-owner row.
- Linked Module Conformance to the host contract stub.
- Added Deliberation Budget token-cost accounting fields and eval cases.
- Added this goal/report and Work Status completion note.

## Boundary confirmation

- No Rust code was changed.
- No Module Host runtime was activated.
- No module loading, module invocation, manifest parser, packet parser,
  capability enforcement, conformance checker, provider orchestration, token
  accounting implementation, workspace initializer, instruction generator,
  adapter, publishing behavior, metrics collection, receipt writer, gate
  writer, proofpack writer, or acceptance claim was added.
- `selected_next` remains
  `work/goals/goal_pause_after_brownfield_manifest_writer_first_slice_v0_1.md`.

## Knowledge impact

- Canonical product surface added:
  - `docs/product/MODULE-HOST-CONTRACT.md`
- Eval/spec surface added:
  - `evals/specs/module-host-contract-stub.v0.1.md`
- Deliberation Budget affected:
  - `docs/product/DELIBERATION-BUDGET.md`
  - `evals/specs/deliberation-budget-boundary.v0.1.md`
- Future follow-up:
  - run a trivial read-only module dry run through Module Conformance Packet
    and Module Host Contract Stub before PubPunk skeleton work.

## Doc impact
```yaml
doc_impact:
  classification: docs-only
  reason: "Adds Module Host Contract Stub and Deliberation Budget token-cost accounting as docs/eval boundaries before DevPunk or PubPunk implementation, without activating runtime/module behavior."
  touched_surfaces:
    - docs/product/MODULE-HOST-CONTRACT.md
    - docs/product/MODULE-HOST.md
    - docs/product/MODULE-CONFORMANCE.md
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/DELIBERATION-BUDGET.md
    - evals/specs/module-host-contract-stub.v0.1.md
    - evals/specs/deliberation-budget-boundary.v0.1.md
    - work/STATUS.md
    - work/goals/goal_define_module_host_contract_stub_v0_1.md
    - work/reports/2026-05-19-module-host-contract-stub-v0-1.md
  required_updates:
    - docs/product/DOCUMENTATION-MAP.md
    - docs/product/MODULE-CONFORMANCE.md
    - docs/product/DELIBERATION-BUDGET.md
    - work/STATUS.md
```

## Validation

- `python3 scripts/check_research_gate.py`: PASS.
- `python3 scripts/check_work_ledger.py`: PASS.
- `python3 scripts/check_docs_governance.py --files docs/product/MODULE-AUTHORING.md docs/product/MODULE-CONFORMANCE.md docs/product/MODULE-HOST.md docs/product/MODULE-HOST-CONTRACT.md docs/product/MODULES.md docs/product/DOCUMENTATION-MAP.md docs/product/DELIBERATION-BUDGET.md evals/specs/module-authoring-baseline.v0.1.md evals/specs/module-conformance-packet.v0.1.md evals/specs/module-host-contract-stub.v0.1.md evals/specs/deliberation-budget-boundary.v0.1.md work/STATUS.md work/goals/goal_define_module_authoring_baseline_v0_1.md work/goals/goal_define_module_conformance_packet_v0_1.md work/goals/goal_define_module_host_contract_stub_v0_1.md work/reports/2026-05-19-module-authoring-baseline-v0-1.md work/reports/2026-05-19-module-conformance-packet-v0-1.md work/reports/2026-05-19-module-host-contract-stub-v0-1.md --report work/reports/2026-05-19-module-authoring-baseline-v0-1.md --report work/reports/2026-05-19-module-conformance-packet-v0-1.md --report work/reports/2026-05-19-module-host-contract-stub-v0-1.md`: PASS, warnings 0.
- `git diff --check`: PASS.

No Rust code changed; `cargo check --workspace` was not run for this
docs/eval-only slice.

## Recommended next work

Run a trivial read-only module dry run through Module Conformance Packet v0.1
and Module Host Contract Stub v0.1, then apply the same packet to PubPunk.
