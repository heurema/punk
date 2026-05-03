---
id: report_2026_05_03_brownfield_inventory_boundary_v0_1
kind: work-report
status: accepted
authority: canonical
owner: vitaly
created_at: 2026-05-03
updated_at: 2026-05-03
goal_ref: work/goals/goal_prepare_brownfield_inventory_boundary_v0_1.md
---

# Brownfield Inventory Boundary v0.1

## Summary

- Defined Brownfield Inventory as observable structure only.
- Added canonical product boundary docs and an eval/spec boundary before any source corpus manifest implementation.
- Kept Brownfield reconstruction, repo scanning, AI summaries, claims, contracts/specs, gate/proof runtime, Writer behavior, runtime storage, and grayfield behavior inactive.

## Why inventory boundary is needed before implementation

B0 brownfield init creates an advisory reconstruction workspace, but it does not inspect the repository.

The next risky step is source corpus inventory. Without a boundary, inventory could quietly become claim extraction, intent recovery, generated docs, or project truth. This report records the B1 guardrail before any implementation exists.

## Allowed inventory outputs

Future inventory may record:

- repo-relative path existence;
- path kind as file or directory;
- file extension or known basename marker;
- source class candidate;
- known package manifest type;
- doc/test/config/schema/migration/script marker;
- generated, vendored, or unknown candidate marker;
- git repository metadata presence;
- size or hash only if a later bounded goal explicitly allows it.

## Forbidden inferences

Future inventory must not infer:

- intent;
- requirements;
- module purpose;
- architecture decisions;
- canonical docs;
- accepted behavior;
- invariants;
- non-goals;
- ownership truth;
- project memory truth;
- contract readiness;
- bug versus feature classification;
- risk or severity;
- compatibility or conformance.

## Source classes

Allowed future source classes:

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

Sensitive or forbidden classes requiring caution:

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

## Authority model

Future brownfield inventory output must be:

```text
status = advisory
authority = observed_structure
```

It must not be canonical, accepted, a contract, a decision, proof, or project truth.

## Privacy and no-network rules

Future inventory must be local-only by default.

Rules defined:

- no network access;
- no remote AI calls;
- no raw secret values;
- no raw environment values;
- no local absolute path leakage;
- repo-relative paths only;
- redaction for sensitive path segments;
- no public report output with private local content;
- no hidden upload, sync, telemetry, or remote indexing.

## Difference between inventory item and claim

An inventory item is an observed structural fact, such as a repo-relative path and a candidate source class.

A claim is an interpretation, such as module purpose, requirement, invariant, accepted behavior, architecture decision, or non-goal. Claims need source refs, confidence, and review.

Inventory must not create claims.

## Future B2 implementation constraints

Future source corpus manifest implementation must:

- use explicit allowlist and denylist rules;
- write repo-relative paths only;
- avoid file contents by default;
- avoid semantic summaries;
- avoid hidden project truth;
- be deterministic for the same checkout inputs;
- preflight writes and avoid partial manifests on conflict;
- mark generated, vendored, and unknown candidates;
- keep inventory output separate from claim ledger output;
- keep `status = advisory` and `authority = observed_structure`.

## Docs and eval specs created

- `docs/product/BROWNFIELD-INVENTORY.md`
- `evals/specs/brownfield-inventory-boundary.v0.1.md`
- `work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md`

Updated:

- `docs/product/DOCUMENTATION-MAP.md`
- `docs/product/GLOSSARY.md`
- `docs/product/PROJECT-MEMORY.md`
- `docs/product/ROADMAP.md`
- `work/goals/goal_prepare_brownfield_inventory_boundary_v0_1.md`
- `work/STATUS.md`

## Drift observed

No implementation drift was observed.

The current active brownfield CLI remains only:

```text
punk init <project-id> --mode brownfield
```

It creates a scaffold. It does not inventory source files.

## Checks run

- `python3 scripts/check_research_gate.py` - PASS.
- `python3 scripts/check_work_ledger.py` - PASS.
- `cargo check --workspace` - PASS.
- `cargo test --workspace` - PASS.
- `~/.local/bin/punk-dev eval run smoke` - PASS.
- `git diff --check` - PASS.
- `scripts/check.sh docs-governance --files docs/product/BROWNFIELD-INVENTORY.md docs/product/DOCUMENTATION-MAP.md docs/product/GLOSSARY.md docs/product/PROJECT-MEMORY.md docs/product/ROADMAP.md evals/specs/brownfield-inventory-boundary.v0.1.md work/STATUS.md work/goals/goal_prepare_brownfield_inventory_boundary_v0_1.md work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md work/reports/2026-05-03-brownfield-inventory-boundary-v0-1.md --report work/reports/2026-05-03-brownfield-inventory-boundary-v0-1.md` - PASS with 0 failures and 0 warnings.

## Next selected goal

`work/goals/goal_verify_brownfield_inventory_boundary_v0_1.md`

Reason: this is a trust boundary. It should be verified before any source corpus manifest implementation.

## Boundaries preserved

No source inventory implementation, repo scan, file walker, language detector, inventory writer, source corpus manifest generator, claim extraction, claim ledger population, AI summaries, LLM calls, module map, architecture recovery, intent recovery, contract generation, Contract Context Pack generation, Conformance Pack runtime, Migration Contract runtime, Regenerative Spec behavior, gate/proof runtime, Writer behavior, runtime `.punk` storage, grayfield reconciliation, or spec-as-source behavior was activated.

## Doc impact
```yaml
  classification: docs-only
  reason: "Defines the brownfield inventory boundary before any implementation, source corpus manifest writer, or claim reconstruction work."
```
