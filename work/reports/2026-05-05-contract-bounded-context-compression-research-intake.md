---
id: report_2026_05_05_contract_bounded_context_compression_research_intake
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-05
updated_at: 2026-05-05
goal_ref: work/goals/goal_apply_contract_context_pack_boundary_patch_v0_1.md
---

# Contract-Bounded Context Compression Research Intake

## Summary

Resolved the context-pack handoff conflict by merging missing research/eval
ideas into the existing Context Pack boundary line instead of replacing current
repo truth.

ADR-0017 was not created because `docs/adr/ADR-0016-contract-context-pack-boundary.md`
already covers the Contract Context Pack boundary decision. The new material is
research/eval detail: deterministic compression, raw fallback, stale derived
state, frozen-after-plot behavior, and lean-ctx-style parked runtime ideas.

## Files changed

- `evals/specs/context-pack-boundary.v0.1.md`
- `knowledge/research/2026-05-05-contract-bounded-context-compression.md`
- `knowledge/ideas/lean-ctx-style-context-runtime.md`
- `work/reports/2026-05-05-contract-bounded-context-compression-research-intake.md`
- `work/STATUS.md`

## What was added

- Advisory R2 research note for deterministic, contract-bounded context
  compression and prior art.
- Parked lean-ctx-style context runtime idea.
- Addendum in the existing context-pack boundary eval spec for raw fallback,
  summary-only proof blocking, stale cache/index fail-closed behavior,
  memory/advisory boundaries, deterministic rebuild, frozen-after-plot handling,
  CLI/log compaction integrity, and non-goal/exclusion coverage.

## Deliberately not changed

- No `ADR-0017` was created.
- `docs/adr/ADR-0016-contract-context-pack-boundary.md` was left unchanged.
- `evals/specs/context-pack-compiler-boundary.v0.1.md` was left unchanged
  because the missing invariants belong in the base context-pack boundary spec.
- Existing product docs were left unchanged because they already keep context
  packs advisory, derived, and below contract/gate/proof authority.

## Doc impact

```yaml
doc_impact:
  classification: research-promotion
  reason: "Adds advisory research and missing eval invariants to the existing Contract Context Pack boundary line without changing canonical product docs, ADR decisions, runtime behavior, CLI behavior, storage, adapters, proof, or gate authority."
  touched_surfaces:
    - evals/specs/context-pack-boundary.v0.1.md
    - knowledge/research/2026-05-05-contract-bounded-context-compression.md
    - knowledge/ideas/lean-ctx-style-context-runtime.md
    - work/reports/2026-05-05-contract-bounded-context-compression-research-intake.md
    - work/STATUS.md
  required_updates:
    - evals/specs/context-pack-boundary.v0.1.md
    - knowledge/research/2026-05-05-contract-bounded-context-compression.md
    - knowledge/ideas/lean-ctx-style-context-runtime.md
    - work/reports/2026-05-05-contract-bounded-context-compression-research-intake.md
    - work/STATUS.md
  supersedes: []
  archive_plan: []
  evals_required:
    - python3 scripts/check_research_gate.py
    - python3 scripts/check_work_ledger.py
    - scripts/check.sh docs-governance
```

## Knowledge impact

- Canonical product docs changed: None.
- Project-memory claims affected: Reinforces that context packs and compressed
  views are derived advisory evidence, not project truth.
- Docs / ADRs / evals possibly stale: None requiring immediate canonical-doc
  changes. ADR-0016 remains the active decision surface.
- Active / parked / future scope affected: Runtime context storage, MCP, shell
  hooks, session memory, embeddings, vector DB, and executor briefs remain
  deferred or parked.
- Public narrative impact: None.
- Derived views to rebuild later: Future context-pack/compiler eval fixtures
  should include the addendum invariants.
- Follow-up goals or drift findings: A later bounded side-effect-free model or
  eval fixture pass can encode these invariants in Rust/eval coverage.
- Unknowns / contradictions: Whether CLI/log compaction belongs inside context
  pack v0.1 or a separate shell/log receipt spec remains open.

## Drift observed

- The previous overlay tried to replace an existing eval spec and create a new
  ADR placeholder even though the repo already had ADR-0016 and context-pack
  eval specs.
- The correct resolution is merge/update against the existing boundary line,
  not duplicate truth surfaces.

## Out of scope

No runtime behavior, `.punk` writes, CLI command, dependency, MCP integration,
shell hook, embeddings, vector DB, proofpack writer, gate writer, hidden memory,
context-pack runtime store, executor brief generator, product-doc promotion, or
new ADR was added.

## Validation run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `scripts/check.sh docs-governance --files evals/specs/context-pack-boundary.v0.1.md knowledge/research/2026-05-05-contract-bounded-context-compression.md knowledge/ideas/lean-ctx-style-context-runtime.md work/reports/2026-05-05-contract-bounded-context-compression-research-intake.md work/STATUS.md --report work/reports/2026-05-05-contract-bounded-context-compression-research-intake.md` - PASS.
- `git diff --check` - PASS.
- `cargo check --workspace` - not run; no Rust code changed.
