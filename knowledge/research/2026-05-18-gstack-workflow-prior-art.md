---
id: research_gstack_workflow_prior_art_2026_05_18
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-05-18
updated_at: 2026-05-18
review_after: 2026-06-18
classification: r2-design-research
canonical_for: []
related_docs:
  - docs/product/PUNK-LAWS.md
  - docs/product/ARCHITECTURE.md
  - docs/product/ROADMAP.md
  - docs/product/CRATE-STATUS.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/RESEARCH-INTAKE.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RUNNER-AIDS.md
  - docs/product/REVIEW-ASSESSMENT.md
  - docs/product/INSTRUCTION-SOURCES.md
related_adrs:
  - docs/adr/ADR-0014-executor-agnostic-validation-boundary.md
  - docs/adr/ADR-0016-contract-context-pack-boundary.md
related_evals:
  - evals/specs/runner-aid-boundary.v0.1.md
  - evals/specs/review-assessment-receipt.v0.1.md
  - evals/specs/contract-intake-questions.v0.1.md
  - evals/specs/docs-drift-assessment.v0.1.md
  - evals/specs/instruction-source-freshness.v0.1.md
supersedes: []
superseded_by: null
---

# gstack workflow prior art for Punk

## Verdict

Adopt by extraction only.

`garrytan/gstack` is useful prior art for staged AI-assisted software work,
review lanes, browser-enabled QA, safety guardrails, generated skill docs, and
cross-agent runner aids.

Punk should not copy gstack as a product surface. Punk should extract bounded
mechanisms that strengthen active-core trust surfaces:

```text
runner aid / skill / external assessor
  -> advisory assessment
  -> receipt / evidence refs
  -> eval or report
  -> gate decision
  -> proofpack
  -> project-memory links
```

This note does not activate new CLI commands, Claude-specific workflows,
provider adapters, browser automation, cross-agent orchestration, Module Host
runtime, DevPunk, PubPunk, gate writing, proofpack writing, acceptance claims,
remote telemetry, deployment automation, or hidden executor memory.

## Question

Which mechanisms from `garrytan/gstack` should Punk adopt, defer, park, or avoid
while preserving Punk's laws: contract first, `plot / cut / gate`, only `gate`
writes final decisions, modules assess rather than decide, adapters invoke
rather than own truth, proof before acceptance, project memory from day zero,
and research before major decisions?

## Decision context

Punk is an executor-agnostic bounded work kernel. It may help humans, scripts,
models, coding agents, modules, and adapters attempt scoped work, but executor
prompts, skills, slash commands, local memories, provider defaults, and
self-reported success are not project truth.

The active Punk lifecycle remains:

```text
plot -> cut -> gate
```

The correct intake path for gstack is therefore:

1. classify external mechanisms through Research Intake;
2. define runner-aid and review-assessment boundaries;
3. add eval specs before implementation;
4. keep gstack-like surfaces advisory until a later phase deliberately promotes
   module, adapter, or DevPunk behavior;
5. preserve gate/proof/project-memory authority.

## Source quality table

| Source | URL or repo ref | Tier | Use |
|---|---|---:|---|
| gstack README | https://github.com/garrytan/gstack/blob/main/README.md | A/B | Primary product surface: workflow, roles, install, host coverage, browser, QA, review, ship, memory, guardrails. |
| gstack Architecture | https://github.com/garrytan/gstack/blob/main/ARCHITECTURE.md | A/B | Browser daemon, token model, tunnel separation, prompt-injection defense, ref system, generated skill-doc architecture. |
| gstack Skill Deep Dives | https://github.com/garrytan/gstack/blob/main/docs/skills.md | A/B | Concrete workflow lanes and skill behavior. |
| gstack Adding a Host | https://github.com/garrytan/gstack/blob/main/docs/ADDING_A_HOST.md | A/B | Declarative host transform model and generated skill-doc freshness discipline. |
| gstack Domain Skills | https://github.com/garrytan/gstack/blob/main/docs/domain-skills.md | A/B | Agent-authored memory/state machine and prompt-injection risk model. |
| Punk Laws | docs/product/PUNK-LAWS.md | A | Hard constraints for decision authority, proof, modules, adapters, and runner aids. |
| Punk Architecture | docs/product/ARCHITECTURE.md | A | Executor-agnostic validation boundary, task-scoped advisory handoffs, module/adapters boundaries. |
| Punk Research Gate | docs/product/RESEARCH-GATE.md | A | Persistent model-control artifacts require research before promotion. |
| Punk Research Intake | docs/product/RESEARCH-INTAKE.md | A | Adopt/defer/park/avoid classification. |
| Punk Project Memory | docs/product/PROJECT-MEMORY.md | A | Prompt/skill memory boundary and knowledge authority rules. |

Tier C sources were not used as decision-supporting evidence.

## What gstack gets right

### 1. Staged work beats one-shot prompting

gstack's strongest primitive is not any individual command. It is the staged
shape:

```text
think -> plan -> build -> review -> test -> ship -> reflect
```

For Punk, this should not replace `plot / cut / gate`. It should become
contract-linked advisory structure inside the existing lifecycle:

```text
plot: intent intake, forcing questions, review of scope, contract drafting
cut: scoped execution, receipts, validator output, assessor reports
gate: final decision, proof requirements, project-memory links
```

### 2. Review lanes are useful when non-authoritative

gstack separates CEO/product review, engineering review, design review,
security review, QA, documentation, and release review. The useful mechanism is
not the persona branding. It is clause-specific assessment from multiple lenses.

Punk should model those outputs as `ReviewAssessment` artifacts. They may feed
gate, but they must not write final decisions.

### 3. Forcing questions are a contract-quality tool

`/office-hours`-style intake pushes raw requests through specific examples,
status quo, narrowest wedge, assumptions, and implementation alternatives.

For Punk, this belongs in `plot` as a contract-intake question model:

```text
raw request -> assumptions -> unknowns -> options -> selected wedge -> contract draft readiness
```

### 4. Scope guards should become contract guards

gstack's `/freeze`, `/careful`, and `/guard` are useful because they reduce
unrelated edits and destructive commands.

Punk should not implement them as provider-specific slash commands in active
core. The mechanism should appear as:

- contract `allowed_paths` / `denied_paths`;
- validator scope-diff checks;
- receipt changed-artifact refs;
- side-effect preflight blockers;
- gate deviation policy.

### 5. Browser QA is powerful but side-effect-heavy

gstack's persistent browser is a high-leverage QA tool. It also touches cookies,
local browser state, network behavior, prompt-injection surfaces, and tunnel
security.

For Punk, browser QA should remain parked until adapter boundaries,
side-effect policy, receipts, and external-action proof requirements are ready.

### 6. Generated skill docs suggest a freshness check

gstack's host-specific generated skills and freshness tests are a good
mechanism for preventing stale runner instructions.

Punk can adopt the safer version now: source instruction pages are repo-tracked,
generated views are rebuildable and non-authoritative, and freshness/drift evals
flag inconsistencies without creating runtime behavior.

### 7. Agent memory must be quarantined

gstack domain skills illustrate both value and risk: agent-authored notes can
compound, but they are prompt-injection and hidden-memory vectors.

Punk should keep executor-local memory outside project truth unless promoted
through scoped, authority-tagged project artifacts and reviewed links.

## Failure modes found

| Failure mode | Punk risk | Required guard |
|---|---|---|
| Slash command output becomes acceptance | Bypasses `gate` | Review assessments are advisory; only `gate` decides. |
| Provider-specific skill becomes architecture | Executor lock-in | Runner aids are non-authoritative and task-scoped. |
| Browser/cookie workflow enters active core early | Hidden network/secrets/side effects | Park browser QA until adapter/side-effect policy. |
| Generated skill docs become truth | Second source of truth | Source pages own instruction truth; generated views rebuildable only. |
| Agent memory injected into future prompts | Hidden project memory | Promote through authority-tagged artifacts only. |
| Auto-fix review modifies unrelated files | Scope drift | Contract scope guards, receipt changed refs, gate deviation check. |
| Cross-model pass/fail becomes decision | Module/adapter decides | Cross-model review is evidence only. |
| Docs auto-rewrite silently changes canonical truth | Documentation drift | Docs-drift assessment may recommend; canonical docs still need review. |
| Parallel agent work outruns evidence | Uninspectable chaos | Multiple flows require persisted state, receipts, evals, gate/proof. |
| Benchmark/telemetry becomes truth | Hidden analytics or opaque scoring | Local-only evidence with explicit refs; remote export parked. |

## Adoption map

### Adopt now as active-core docs/eval/spec direction

- Runner aid boundary: prompts, skills, playbooks, persistent model-control
  artifacts, and provider settings are aids, not authority.
- Review assessment receipt shape for code review, security, QA, DX, design,
  docs drift, and second-opinion outputs.
- Contract-intake forcing questions as `plot` readiness checks.
- Docs-drift assessment as an eval/report discipline, not auto-accepted rewrite.
- Instruction source freshness: source instruction pages vs generated views.
- Scope guard mechanism as contract/eval/receipt/gate discipline.

### Defer

- DevPunk review module.
- Security assessment module.
- Second-opinion cross-model assessment.
- Host-specific runner-aid generators.
- Runner-aid rendering from contract context packs.
- Structured review dashboard.
- Persistent local runner-aid registry.

### Park

- Browser daemon and browser QA adapter.
- Cookie import and authenticated web testing.
- Pair-agent / shared browser coordination.
- Deployment, canary, and release automation.
- Cross-project agent memory or GBrain-like memory.
- Parallel sprint orchestration.
- Remote telemetry, dashboards, or SaaS control plane.

### Avoid

- AI, skill, module, adapter, or reviewer writing final decisions.
- Slash-command output as proof.
- Hidden prompt/session memory as project truth.
- Provider-specific workflow as active-core architecture.
- Raw prompt/source telemetry.
- Browser or deploy side effects without receipts and side-effect policy.
- Generated docs/views as canonical truth.

## Punk mapping

| gstack mechanism | Punk mapping | Status |
|---|---|---|
| `/office-hours` | contract-intake questions in `plot` | adopt as eval/spec direction |
| `/plan-*` reviews | advisory `ReviewAssessment` | adopt model/spec, defer runtime |
| `/review` | code review assessment receipt | adopt model/spec, defer module |
| `/cso` | security assessment receipt | defer module |
| `/qa` and browser | browser QA adapter with side-effect receipts | park |
| `/document-release` | docs-drift assessment | adopt eval/spec direction |
| `/careful`, `/freeze`, `/guard` | contract scope and side-effect guard policy | adopt mechanism |
| `/codex` | independent assessor / second opinion | defer |
| `/learn`, domain skills | executor-local memory with promotion path | park runtime, adopt boundary |
| generated `SKILL.md` | source instruction freshness / generated views | adopt boundary |
| `/ship`, deploy, canary | release/deploy side-effect adapters | park |
| parallel sprints | multiple flow instances with persisted evidence | park until flow/evidence mature |

## Required evals

- `evals/specs/runner-aid-boundary.v0.1.md`
- `evals/specs/review-assessment-receipt.v0.1.md`
- `evals/specs/contract-intake-questions.v0.1.md`
- `evals/specs/docs-drift-assessment.v0.1.md`
- `evals/specs/instruction-source-freshness.v0.1.md`

## Required docs

- `docs/product/RUNNER-AIDS.md`
- `docs/product/REVIEW-ASSESSMENT.md`
- `docs/product/INSTRUCTION-SOURCES.md`
- `docs/product/DOCUMENTATION-MAP.md` owner rows for the three new boundary docs
- `work/STATUS.md` side-track completion note, preserving `selected_next`

## Roadmap impact

This intake does not change the current selected next goal.

It strengthens Phase 3/4/6+ readiness by giving names and eval boundaries to
runner aids, review assessments, and instruction-source freshness before Punk
promotes executor-facing behavior.

It also protects active-core from accidental gstack-shaped scope creep: no
browser, no provider-specific agent suite, no deployment automation, no hidden
memory, and no AI decision authority.

## What stays out of scope

- public CLI additions;
- new Rust runtime behavior;
- Module Host runtime;
- browser daemon or web QA;
- provider adapter invocation;
- Claude/Codex-specific command surfaces;
- automated docs rewriting;
- deployment/canary behavior;
- remote telemetry;
- gate decision writing;
- proofpack writing;
- acceptance claims.

## Recommendation

Proceed with a bounded research/spec/docs patch only.

Do not implement gstack-like commands. Do not modify current executable surface.
Do not change the current selected next goal. Treat all new artifacts as
research-backed boundaries and eval-intake material.

## Open questions

1. Should `ReviewAssessment` live in `punk-domain`, `punk-contract`, or a future
   module-host boundary when code is selected?
2. Should runner aids be represented as a distinct artifact type, or as
   contract-linked reports/receipts only?
3. Which review lanes should get first fixtures: code review, docs drift, or
   contract-intake questions?
4. Should instruction-source freshness become a docs consistency smoke case or
   stay as a separate eval spec until generated views are active?
5. Should a future browser QA adapter share the side-effect receipt writer model
   already incubating in Module Host, or define a stricter browser-specific
   receipt policy first?
