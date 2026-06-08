---
id: research_brownfield_observation_local_lab_refs_2026_06_08
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-06-08
updated_at: 2026-06-08
review_after: 2026-07-08
classification: r2-design-research
canonical_for: []
related_docs:
  - docs/product/BROWNFIELD-INVENTORY.md
  - docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md
related_evals:
  - evals/specs/brownfield-inventory-boundary.v0.1.md
  - evals/specs/brownfield-source-corpus-manifest-boundary.v0.1.md
supersedes: []
superseded_by: null
---

# Brownfield Observation Local Lab References

## Verdict

Use `code-intel-kernel` as an advisory local lab for observation-packet ideas
and `agent-bench-lab` as the required evaluation planning surface for future
active observation/scanner results.

Do not treat either lab as Punk product truth. Local lab checkout state must be
verified at execution time before any idea, result, benchmark, or evaluation
claim is reused.

## Question

How should the next Brownfield source inventory observation-packet boundary use
local labs without accidentally activating scanning, importing lab behavior, or
promoting benchmark results into authority?

## Source quality table

| Source | Tier | Use |
|---|---:|---|
| `docs/product/BROWNFIELD-INVENTORY.md` | A | Current Punk inventory authority and non-inference boundary. |
| `docs/product/BROWNFIELD-SOURCE-CORPUS-MANIFEST.md` | A | Current manifest model/writer boundary and no-scan/no-claim limits. |
| `work/goals/goal_prepare_brownfield_source_inventory_observation_packet_boundary_v0_1.md` | A | Current selected goal to refine. |
| `code-intel-kernel` local lab | B | Advisory idea source for code observation boundaries and code-intel packet shape. Verify current checkout state before use. |
| `agent-bench-lab` local lab | B | Evaluation planning source for future active observation/scanner results. Verify current checkout state before use. |

## Adoption guidance

The next boundary should include two explicit passes:

1. Advisory idea pass over `code-intel-kernel`.
2. Evaluation pass through `agent-bench-lab`.

The `code-intel-kernel` pass may inspire observation-packet fields, blocker
classes, source classification ideas, and failure modes. It must not import lab
code, lab state, or lab output as Punk product truth.

The `agent-bench-lab` pass should define how future active observation/scanner
results will be assessed before they influence Brownfield decisions. It should
focus on result validity, failure visibility, benchmark integrity, and
comparison shape rather than leaderboard-style claims.

## Required boundary implications

The selected Brownfield observation-packet boundary should record:

- local lab refs are advisory;
- current lab checkout state must be verified before use;
- observation packet output remains `advisory` and `observed_structure`;
- future active scanner results need explicit evaluation before influencing
  decisions;
- benchmark or lab results are not gate decisions, proof, acceptance, project
  truth, or contract readiness.

## Non-scope

This note does not run either lab, import lab code, execute benchmarks, start
repo scanning, walk files, read source contents, compute source hashes, create
claims, write `.punk` state, activate CLI behavior, or promote lab results to
Punk authority.
