---
id: report_2026_05_03_brownfield_inventory_boundary_verification_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md
---

# Brownfield Inventory Boundary Verification v0.1

## Summary

Verified the B1 Brownfield Inventory Boundary after PR #19 merged.

Verdict: pass.

The boundary defines inventory as observable structure only, keeps inventory
separate from claims and project truth, defines allowed and sensitive source
classes, preserves advisory `observed_structure` authority, and keeps source
inventory implementation inactive.

## Files inspected

- `docs/product/BROWNFIELD-INVENTORY.md`
- `evals/specs/brownfield-inventory-boundary.v0.1.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `work/STATUS.md`
- `work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md`
- `work/reports/2026-05-03-brownfield-inventory-boundary-v0-1.md`

## Inventory vs claim verification

Pass.

`docs/product/BROWNFIELD-INVENTORY.md` defines:

- Source Corpus Manifest as future deterministic inventory of observable
  artifacts;
- Inventory Item as a repo-relative observable fact;
- Claim as an interpretation requiring source refs, confidence, and review;
- Contract Readiness as a future advisory assessment outside inventory
  authority.

The doc explicitly says inventory must not infer intent, requirements, module
purpose, architecture decisions, canonical docs, accepted behavior,
invariants, non-goals, ownership truth, project memory truth, contract
readiness, bug versus feature classification, risk/severity, compatibility, or
conformance.

## Source class verification

Pass.

The boundary defines allowed future source classes:

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

The eval spec covers the same bounded vocabulary and requires `unknown` when
classification is uncertain.

## Sensitive and caution class verification

Pass.

The boundary defines sensitive or caution classes:

```text
secrets
env_files
local_cache
build_output
node_modules
target
.git
.punk/runtime
.punk/cache
.punk/indexes
private_agent_transcripts
```

The eval spec verifies that raw secret values, raw environment values, local
cache contents, build output contents, `.git` internals, runtime/cache/index
state, and private agent transcripts must not be exposed.

## Authority and privacy verification

Pass.

Future inventory output is constrained to:

```text
status = advisory
authority = observed_structure
```

It must not be canonical, accepted, a contract, a decision, proof, or project
truth.

Privacy rules are present:

- local-only by default;
- no network access;
- no remote AI calls;
- no raw secret or environment values;
- no local absolute path leakage;
- repo-relative paths only;
- redaction for sensitive path segments;
- no public report output with private local content;
- no hidden upload, sync, telemetry, or remote indexing.

The boundary also requires future inventory to avoid file contents by default.

## Anti-overclaim grep result

Pass.

Commands inspected wording for overclaim:

```bash
rg -n "understands|reconstructs|infers intent|accepted behavior|project truth|canonical|contract generation|AI summaries|repo scan active" docs/product/BROWNFIELD-INVENTORY.md evals/specs/brownfield-inventory-boundary.v0.1.md docs/product/PROJECT-MEMORY.md docs/product/ROADMAP.md

rg -n "brownfield reconstruction active|source inventory implementation|source corpus manifest implementation|repo scanning active|file walker active|claim extraction active|AI summaries active|contract generation active|understands existing|understand existing|reconstructs existing|infers intent" docs/product evals work
```

Matches were acceptable:

- `canonical` describes the product doc itself or is used as a forbidden
  inventory-output authority;
- `project truth`, `accepted behavior`, `AI summaries`, `contract generation`,
  and scan-related terms appear as explicit prohibitions or inactive/future
  boundaries;
- no wording says brownfield reconstruction, repo scanning, source inventory
  implementation, claim extraction, AI summaries, or contract generation are
  active.

## Eval spec coverage

Pass.

The eval spec covers:

- observable structure only;
- no claims created;
- bounded source classes;
- sensitive classes not exposed;
- repo-relative paths only;
- no network and no remote AI;
- file contents excluded by default;
- docs are not canonical by default;
- tests are not proof;
- deterministic and atomic output for future implementation;
- generated and vendored candidates stay marked;
- contract readiness remains advisory and future.

## Drift observed

No implementation drift observed.

`git diff --name-only` was empty before writing this verification evidence.
The verification changed only work-ledger/report artifacts.

No `crates/**`, `.punk/**`, `schemas/**`, CLI behavior, runtime storage,
source inventory implementation, repo traversal, or writer behavior was
changed.

## Checks run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files work/STATUS.md work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md work/goals/goal_prepare_brownfield_source_corpus_manifest_design_v0_1.md work/reports/2026-05-03-brownfield-inventory-boundary-verification-v0-1.md --report work/reports/2026-05-03-brownfield-inventory-boundary-verification-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next selected goal

`work/goals/goal_prepare_brownfield_source_corpus_manifest_design_v0_1.md`

Reason: B1 boundary verification passed. The next safe step is design/spec
for the future source corpus manifest, not implementation.

## Boundaries preserved

No source inventory implementation, repo scan, file walker, language detector,
inventory writer, source corpus manifest generator, claim extraction, claim
ledger population, AI summaries, LLM calls, module map, architecture recovery,
intent recovery, contract generation, Contract Context Pack generation,
Conformance Pack runtime, Migration Contract runtime, Regenerative Spec
behavior, gate/proof runtime, Writer behavior, runtime `.punk` storage,
grayfield reconciliation, or spec-as-source behavior was activated.
