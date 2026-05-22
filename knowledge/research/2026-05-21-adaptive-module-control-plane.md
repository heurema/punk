---
id: research_2026_05_21_adaptive_module_control_plane
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-05-21
updated_at: 2026-05-21
review_after: 2026-06-21
research_gate: R2
components:
  - modules
  - module-host
  - instruction-sources
  - project-memory
  - deliberation-budget
  - pubpunk
related_docs:
  - docs/product/MODULE-AUTHORING.md
  - docs/product/MODULE-HOST-CONTRACT.md
  - docs/product/INSTRUCTION-SOURCES.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/DELIBERATION-BUDGET.md
related_ideas:
  - knowledge/ideas/2026-05-21-adaptive-module-control-plane.md
related_goals:
  - work/goals/goal_capture_adaptive_module_control_plane_v0_1.md
supersedes: []
superseded_by: null
---

# Adaptive Module Control Plane

## Question

How should Punk support modules that can be tuned over time without rewriting
their implementation every time the operator changes workflow, voice, channel
policy, or recurring process rules?

The immediate pressure is PubPunk and adjacent operator modules such as a
job-search agent. The desired shape is:

```text
stable module implementation
  + editable behavior/control artifacts
  + explicit evidence from real use
  + periodic reflection that proposes improvements
  + human/gate promotion before anything changes
```

## Decision context

This is R2 design research because the idea touches module interface, module
instructions, project memory, persistent model-control artifacts, adapter
boundaries, side-effect policy, and future runtime behavior.

Current Punk already has:

- Module Authoring Baseline v0.1;
- Module Host Contract Stub v0.1;
- Instruction Sources;
- Project Memory;
- Deliberation Budget;
- incubating, side-effect-free PubPunk models.

This note does not activate a runtime, module loader, background automation,
adapter, browser flow, credential access, external publishing, gate writer,
proofpack writer, or self-modifying module behavior.

## Sources reviewed

| Source | Tier | Why included | Limitation |
|---|---:|---|---|
| Twelve-Factor App config | A | Separates deploy-varying configuration from code. | Config separation does not make behavior changes safe by itself. |
| Kubernetes controllers | A | Prior art for observing state and reconciling toward desired state. | Continuous auto-reconcile conflicts with Punk unless reduced to propose-only drift detection. |
| Kubernetes custom resources | A | Shows declarative extension objects and explicit API shape. | Kubernetes CRDs assume an active control plane; Punk is not there yet. |
| OPA decision logs | A | Useful shape for auditable policy decision records. | OPA does not solve Punk authority or gate boundaries by itself. |
| OpenTelemetry overview | A | Standard trace/metric/log vocabulary for evidence collection. | Full OTel export is premature and can create telemetry/privacy scope. |
| OpenFeature specification | A | Separates flag evaluation API from provider implementation. | Runtime flagging is overkill before active modules and can hide behavior drift. |
| VS Code extension manifest | A | Explicit extension metadata and capability-like activation surfaces. | Extension host trust differs from Punk's module authority model. |
| Claude Code Skills docs | A | Editable skill docs as progressively disclosed behavior aids. | Skills are tooling artifacts, not project truth. |
| LangGraph memory docs | B | Explicit short-term and long-term memory concepts for agents. | Hidden memory or giant prompt state conflicts with Punk project memory. |
| Reflexion paper | A | Shows reflection can improve language agents through feedback. | Needs bounded tasks, evals, and controls; reflection can amplify bad signals. |
| Self-Refine paper | A | Shows iterative self-feedback/refinement can improve outputs. | Self-feedback is not an authority model. |
| Voyager paper | A | Shows skill accumulation in an open-ended agent. | Autonomous skill libraries are unsafe for Punk without gate/eval/provenance. |
| Punk canonical docs listed above | A | Current source of truth for module, memory, instruction, and gate boundaries. | These docs do not yet define adaptive module control artifacts. |

Access notes:

- HTTP reachability was checked for the external web sources on 2026-05-21.
- The OpenAI Agents docs endpoint returned HTTP 403 from the local fetch path
  and is not used as supporting evidence here.
- The arXiv abstract pages were reachable during source scan; an arXiv API
  metadata request later returned 429 and is not used as evidence.

## Existing systems and prior art

### Config is useful, but behavior artifacts are not harmless config

Twelve-Factor config supports a clean code/config split. For Punk, that split
is useful as an editability boundary:

```text
module implementation changes rarely
behavior/control artifacts change more often
```

It is not a trust boundary. A small instruction or skill change can alter
external publishing, safety posture, or gate-adjacent behavior more than a code
patch. Therefore editable behavior artifacts require the same class of
evidence, eval, gate, and provenance discipline as any risky behavior change.

### Declarative manifests help only if they stay explicit

VS Code extension manifests and Kubernetes CRDs both show the value of explicit
metadata. Punk should extract the mechanism:

- module identity;
- version;
- behavior artifact refs;
- immutable core boundaries;
- capability requests;
- side-effect policy refs;
- evidence and receipt refs.

Punk should not import the assumption that declaring an extension object means
an active host should immediately run it.

### Reconcile loops must be reduced to proposal loops

Kubernetes controllers are valuable prior art for comparing desired state with
observed state. Punk should not adopt continuous autonomous reconciliation for
module behavior. The safe extraction is:

```text
observed evidence
  -> drift or improvement finding
  -> tuning proposal
  -> eval/gate/provenance
  -> human-promoted behavior artifact
```

The control loop may propose. It must not apply.

### Reflection is a signal generator, not an authority

Reflexion, Self-Refine, and Voyager show that reflection and skill accumulation
can improve agents. They also expose the central risk: feedback loops can
optimize against weak, self-produced, or stale evidence.

Punk should use reflection only as an advisory proposal generator. Reflection
state must be an explicit artifact with source refs and review date, not hidden
module memory.

### Observability is evidence, not priority or control by itself

OpenTelemetry and OPA decision logs are useful evidence shapes. They should
not become automatic control inputs. A weekly reflection pass can cite receipts,
events, and decision logs, but a metric spike or repeated action pattern should
produce a proposal, not silently change module behavior.

## Failure modes found

| Failure mode | Punk risk | Guard |
|---|---|---|
| Treating behavior artifacts as safe config | Skill/instruction edits bypass code review and change behavior silently. | Define them as audited behavior artifacts, not safety-light config. |
| Reflection loop becomes soft authority | The reflector decides what should change and can bias future behavior. | Reflection outputs only tuning proposals with evidence refs and limitations. |
| Self-produced evidence loop | A module generates the evidence that justifies tuning itself. | Require evidence independence or explicit limitation before gate promotion. |
| Approval fatigue | Weekly proposal stream becomes rubber-stamped user approval. | Reuse gate/eval/provenance, expire stale proposals, record rationale. |
| Drift ratchet | Many small accepted edits compose into unreviewed behavior drift. | Add drift budget and re-audit trigger for cumulative behavior artifact changes. |
| Capability creep through instructions | Instructions pressure already-granted capabilities or request broader grants. | Editable artifacts cannot grant capabilities or override capability envelopes. |
| Hidden reflection memory | Weekly reflection accumulates unreviewed long-term state. | Reflection state must be explicit, linked, reviewable, and non-authoritative. |
| Artifact conflicts | Two behavior artifacts contradict each other and the host silently chooses. | Add conflict detection and fail-closed proposal/conformance findings. |
| Runtime flagging hides behavior | OpenFeature-like flags make behavior drift dynamic and hard to audit. | Defer runtime flags; prefer versioned artifact diffs and receipts. |
| Active controller too early | Reconcile loop applies behavior changes without gate. | Use propose-only control plane until a later promoted runtime exists. |

## Options considered

### Option A: hard-coded modules only

Every behavior change requires code changes.

Pros:

- high reviewability;
- simple authority boundary;
- no prompt/config drift.

Cons:

- too rigid for daily operator workflows;
- every PubPunk style or channel change becomes code churn;
- discourages fast local iteration.

Verdict: avoid as the long-term module model, but keep implementation kernels
stable and small.

### Option B: editable skill/config layer with direct use

Module binary reads editable instruction files and immediately changes behavior.

Pros:

- very flexible;
- familiar to agent-skill workflows;
- easy to operate manually.

Cons:

- behavior changes bypass gate/eval/provenance;
- hidden prompt drift can become project truth;
- no cumulative drift accounting.

Verdict: modify. Keep the editable layer, but treat it as audited behavior
artifacts, not harmless config.

### Option C: adaptive module control plane

Module binary remains stable. Behavior artifacts are editable, but changes
flow through evidence-backed proposals, evals, gate, and provenance.

Pros:

- flexible without silent self-modification;
- fits current Module Authoring and Module Host boundaries;
- preserves project memory as linked artifacts;
- works for PubPunk and future modules.

Cons:

- requires more upfront schema/eval discipline;
- weekly proposals can still create review load;
- full runtime support is later work.

Verdict: adopt as design direction.

### Option D: autonomous self-improving modules

Modules observe their use, update their own skills, and continue.

Pros:

- fast adaptation;
- attractive demo path.

Cons:

- violates Punk authority boundaries;
- creates hidden memory and drift;
- turns self-produced evidence into control input.

Verdict: avoid.

### Option E: full plugin/runtime/observability stack now

Add active manifests, module host runtime, OpenTelemetry, OpenFeature, and
reflection jobs.

Pros:

- complete architecture quickly;
- clear runtime story.

Cons:

- exceeds current phase;
- activates module runtime and side-effect surfaces too early;
- creates large trust and privacy surface.

Verdict: defer or park until docs/evals/contracts are promoted.

## Recommendation

Adopt **Adaptive Module Control Plane v0.1** as an architecture direction:

```text
stable module implementation
  -> explicit module control manifest
  -> editable behavior artifacts
  -> behavior-provenance binding
  -> evidence-backed tuning proposal
  -> eval/gate promotion
  -> versioned project-memory update
```

The key rule:

```text
Editable behavior artifact changes are behavior changes.
They are not trusted merely because they are text, config, or skills.
```

The first durable change should be documentation and eval direction only:

- name behavior artifacts;
- bind them to manifests and hashes;
- define tuning proposals as advisory artifacts;
- require evidence refs, eval refs, gate rationale, and drift accounting;
- keep runtime reflection and auto-application out of scope.

## Adoption map

### Adopt

- Stable implementation plus explicit behavior artifacts.
- Behavior artifact changes as audited changes.
- Module control manifest as a future authoring/conformance surface.
- Behavior-provenance binding: module outputs cite the artifact set used.
- Tuning proposals as first-class advisory artifacts.
- Reflection as proposal generation only.
- Reuse existing gate/eval/provenance path for promotion.
- Drift budget and re-audit trigger for cumulative behavior edits.

### Defer

- Runtime module manifest parsing.
- Background weekly reflection job.
- OpenTelemetry integration or export.
- OpenFeature-style live flags.
- LangGraph-style long-term memory integration.
- Automated proposal ranking or prioritization.
- Deterministic checker implementation.

### Park

- Plugin marketplaces or third-party adaptive modules.
- Autonomous reconcile loops.
- Growing self-written skill libraries.
- Provider-specific module-control runtime.
- Remote telemetry-backed tuning.

### Avoid

- Silent self-modification.
- Hidden reflection memory.
- Treating prompts, skills, local memories, or generated views as project truth.
- Capability grants through editable artifacts.
- User approval as a replacement for gate/eval/provenance.
- Evidence based only on the module's own unvalidated narrative.

## What stays out of scope

- Rust code.
- Module Host runtime activation.
- Module loading or dynamic dispatch.
- Manifest parser or registry.
- Background automation or scheduler.
- `.punk/` runtime state writes.
- Browser/API/credential/publishing adapters.
- Metrics collection implementation.
- Gate writer or proofpack writer.
- Acceptance claim.

## Impact on roadmap

No active runtime phase changes.

This research changes the recommended shape of future module work:

- module authoring should reserve a behavior artifact/control plane surface;
- PubPunk should not become a one-off hard-coded publisher;
- future reflection should produce proposal artifacts, not mutate module state;
- implementation should wait for contract/eval promotion.

## Required evals

Future promotion should add deterministic eval/spec cases for:

- behavior artifact refs are explicit and source-linked;
- behavior artifact changes require tuning proposals;
- tuning proposals cite evidence refs and artifact-set hashes;
- editable artifacts cannot grant capabilities;
- immutable core boundaries cannot be moved into editable artifacts;
- reflection state is explicit and non-authoritative;
- proposal evidence is not solely self-produced by the tuned module;
- artifact conflicts are detected instead of silently resolved;
- cumulative drift triggers re-audit;
- stale proposals expire instead of auto-applying.

## Required docs, ADRs, and contracts

Recommended next artifacts:

- update `docs/product/MODULE-AUTHORING.md` with adaptive control plane rules;
- add or extend module-authoring eval cases;
- later define `docs/product/MODULE-CONTROL-PLANE.md` if the topic outgrows
  Module Authoring;
- later define a tuning proposal schema;
- later define behavior-provenance and drift-budget contracts;
- later add ADR only when moving from advisory direction to active runtime.

## Deliberation packet summary

Budget class: `standard`.

Trigger reasons:

- module_boundary;
- first_module_strategy;
- recurring_workflow_rule;
- project_truth;
- persistent_model_control_artifact.

Independent advisory passes:

| Pass | Assessor | Summary | Limitation |
|---|---|---|---|
| Codex source synthesis | Codex | Selected adaptive module control plane as a docs/eval direction and rejected silent self-modification. | Same session as implementation. |
| Claude Sonnet | External model | Recommended binary/skill split, manifest, evidence store, bounded reflection, and promotion protocol. | Framed config split too optimistically before adversarial critique. |
| Gemini | External model | Recommended engine/artifact separation, declarative capability manifests, propose-only adaptation loop, and eval-gated upgrades. | Suggested gate updating skill artifacts; Punk must keep gate semantics precise in later docs. |
| Claude Opus | External model | Warned that behavior artifacts are not safety-light config and must use the same gate/eval/provenance path as risky code. | Advisory critique only; no repo authority. |

Selected synthesis:

- Use editable behavior artifacts, but classify them as behavior-changing,
  auditable artifacts.
- Reflection can propose only.
- Capability envelopes and immutable core rules cannot be modified by behavior
  artifacts.
- Promotion requires evidence refs, evals, gate rationale, and provenance.

Rejected synthesis:

- Treating the editable layer as harmless config.
- Background reconcile loops.
- User approval as a replacement for gate/eval/provenance.
- Runtime flags before active module runtime exists.

## Open questions

- What is the exact minimal `module_control_manifest` shape?
- Should behavior artifact hashes live in future module receipts, assessments,
  or both?
- What is the smallest useful drift-budget rule?
- Which first PubPunk behavior artifact should be used as the fixture?
- When does repeated manual tuning justify a background reflection job?
- How should private operator evidence be redacted before reflection?
