---
id: research_2026_04_30_replayable_project_memory
kind: research-note
status: advisory
authority: research
owner: vitaly
created_at: 2026-04-30
updated_at: 2026-04-30
review_after: 2026-07-30
topic:
  - replayable-project-memory
  - regenerative-specs
  - conformance-pack
  - contract-transposition
  - project-memory
  - migration-contracts
related_docs:
  - docs/product/NORTH-STAR.md
  - docs/product/PROJECT-MEMORY.md
  - docs/product/RESEARCH-GATE.md
  - docs/product/ROADMAP.md
  - docs/product/GLOSSARY.md
related_ideas:
  - knowledge/ideas/2026-04-30-replayable-project-memory.md
related_evals:
  - evals/specs/replayability-conformance-pack.v0.1.md
supersedes: []
superseded_by: null
---

# Replayable Project Memory for Punk

## Research status and authority

This note is advisory research.

It does not activate runtime behavior, CLI behavior, Writer behavior, gate/proof writers, migration automation, code generation, agent execution, adapters, or spec-as-source behavior.

Research findings do not become product truth until promoted through the normal Punk path:

```text
research note
  -> ADR / roadmap decision
  -> goal / contract
  -> implementation
  -> eval
  -> gate / proof
  -> project-memory update
```

The core conclusion should be read as:

```text
preserve replayable obligations, not replayable code
```

Replayability means preserving enough linked intent, accepted behavior, decisions, evidence, examples, invariants, and proof refs so future migration/reimplementation work can be expressed as new bounded contracts and verified by conformance/equivalence gates.

It does not mean deterministic regeneration of a project from history.

## Executive summary and framing

### Executive summary

Strategic verdict: **DO** as a research concept, and **MODIFY** the first scope. Punk should adopt **Replayable Project Memory** as a long-term concept, but only in a disciplined, non-magical sense: not “replay the whole project and regenerate the codebase,” but “preserve enough reviewed intent, behavior, decisions, examples, and provenance during normal work that future migration or reimplementation can be scoped by contract and verified by conformance.” Public prior art supports that narrower framing. The strongest evidence comes from work on editable extracted specs, executable examples, property-based and differential testing, architecture recovery, traceability, and provenance systems—not from fully automatic spec-as-source claims. citeturn18view0turn18view1turn23view0turn23view3turn20view3turn26view0turn27search4turn35view0turn35view1

The most useful current analogues come from entity["company","GitHub","developer platform"] Next’s Extract, Edit, Apply and SpecLang work, entity["company","Amazon Web Services","cloud provider"] Kiro’s spec and bugfix flows, CodeSpeak’s takeover and intent-recovery direction, Augment Intent, Tessl, and repo-understanding systems from entity["company","Sourcegraph","code search company"], entity["company","Glean","enterprise search company"], and entity["company","Cognition","ai company"]. But public evidence does **not** show a mature solution to Punk’s authority problem. These systems are strongest when they (a) keep artifacts reviewable, (b) preserve source grounding, and (c) use specs as coordination tools; they are weakest when they imply that generated or self-updating specs are automatically trustworthy. citeturn18view0turn18view1turn18view2turn14view2turn14view3turn16view0turn16view1turn17search3turn19view6turn19view7turn19view8turn19view0turn19view2turn19view4

The key research conclusion is that **replayability should be split into layers**. Audit replay, intent replay, contract replay, and behavior replay are meaningfully achievable in bounded ways. Migration replay is achievable for selected subsystems if Punk preserves the right artifacts and gates equivalence explicitly. Regenerative replay remains future territory because round-trip engineering, model-driven development, and repository-level code translation all still struggle with source-of-truth ambiguity, non-injective transformations, architectural mismatch, and dependency/context failures. citeturn35view0turn35view1turn24search5turn24search16turn24search18turn35view3turn28search8turn28search9

The safest first direction is **not** a Regenerative Spec Pack. It is a **replayability-aware project memory discipline** for greenfield Punk work, plus a **bounded Conformance Pack** for one subsystem when needed. That pack should preserve portable intent, public contract surfaces, examples, fixtures, golden outputs, accepted invariants, characterization evidence, accepted differences, and equivalence policy—while clearly separating portable clauses from stack-specific clauses. Migration or reimplementation then becomes a new contract type that cites prior accepted contracts and the conformance pack, rather than a magical “replay.” citeturn23view0turn23view1turn20view3turn20view5turn21view4turn31search0turn31search3turn30search0turn30search3

### Definitions and vocabulary

**Replayable project memory** should mean: a project memory system that preserves enough source-linked, authority-tagged artifacts to let future humans or tools understand what happened, why it happened, what behavior was accepted, what evidence supported acceptance, and what future work must preserve or may revise. This is closer to provenance, traceability, executable examples, and conformance than to automatic code regeneration. A useful analogy is build provenance: reproducible builds and SLSA provenance record how an artifact was produced in a verifiable way; they do not by themselves decide what the artifact ought to be. Punk’s replayability concept should play the same role for software intent and behavior. That is an inference from the provenance literature plus software-specification practice. citeturn35view0turn35view1turn27search4turn18view1

**Replay** is retrospective: goals, contracts, runs, receipts, decisions, and proofpacks. **Intent replay** adds rationale, assumptions, non-goals, research refs, and decision refs. **Contract transposition** means using an accepted contract as an input artifact for a new contract in a different implementation context. **Behavior replay** means preserving examples, tests, fixtures, schemas, snapshots, invariants, and state-transition evidence that can check a new implementation. **Regeneration** means treating a high-level artifact as primary enough to generate implementation from it. That last step requires a much stronger source-of-truth posture than the earlier four. citeturn18view0turn18view1turn23view0turn20view3turn21view4

**Conformance** should be defined narrowly: not “the new system is identical in every internal detail,” but “the new implementation satisfies an explicitly stated equivalence policy over public behavior, schemas, accepted invariants, side effects, and accepted differences.” Pact’s provider verification model, executable specifications in Cucumber, characterization tests, differential testing, and formal executable specifications all support this narrower notion of conformance. citeturn20view3turn20view5turn31search0turn23view0turn23view3turn32search6

### Replay vs transposition vs regeneration

The cleanest distinction for Punk is this:

| Concept | What it reuses | Typical output | Honest claim | Main risk |
|---|---|---|---|---|
| Replay | historical artifacts | audit trail, onboarding, proof trace | “this is what happened and what was accepted” | incomplete or weak provenance |
| Intent replay | rationale + assumptions + decisions | current reasoning baseline | “this is why prior work was accepted” | stale or overconfident rationale |
| Contract transposition | prior accepted contract + current context | migration/reimplementation contract draft | “this prior contract constrains the new one” | stack leakage and hidden assumptions |
| Behavior replay | tests, examples, fixtures, invariants, schemas | conformance oracle | “this behavior must be preserved unless change is accepted” | preserving bugs or quirks blindly |
| Regeneration | high-level artifact as primary truth | generated implementation | “this artifact is strong enough to drive implementation” | false authority, drift, round-trip failure |

EEA is relevant because it makes extracted specs **ephemeral** and **editable**, not silently canonical. SpecLang, Intent, and CodeSpeak are relevant because they explore stronger spec-centric workflows. But the gap between “editable/changeable summary,” “living spec,” and “trusted regeneration source” is very large. Punk should name that gap explicitly instead of flattening it. citeturn18view0turn18view1turn18view2turn19view6turn16view0turn17search3

## Evidence base and prior art

### Source quality table

Access date for all rows: **2026-04-30**.

| Source | URL / source | Type | Quality tier | Why it matters | Limitations | Access date |
|---|---|---|---|---|---|---|
| Extract, Edit, Apply report | Official report citeturn18view0turn18view1 | official report | A | Best public articulation of extracting editable partial specs from existing code | concept report, not a production-grade authority model | 2026-04-30 |
| SpecLang project page | Official page citeturn18view2 | project page | B | Clear statement of spec-as-primary posture | public details are sparse; no strong brownfield/migration story | 2026-04-30 |
| GitHub Spec Kit repo + brownfield discussion | Repo + discussion citeturn14view0turn14view1 | official repo / community discussion | A/B | Shows current extension ecosystem and explicit brownfield demand | extension list is broad; maturity varies widely | 2026-04-30 |
| Kiro specs and bugfix docs | Official docs/blog/changelog citeturn14view2turn14view3turn14view4turn30search0turn30search2turn30search3turn30search7 | official docs | A/B | Strong current example of requirements/design/tasks and current/expected/unchanged behavior structure | still centered on implementation workflows, not replayability governance | 2026-04-30 |
| CodeSpeak docs/blog | Official docs/blog/product pages citeturn16view0turn16view1turn16view2turn16view3turn17search0turn17search3 | official docs / product materials | B | Closest public analogue for takeover, intent recovery, managed files, and spec-to-code ambition | alpha preview; many claims remain aspirational | 2026-04-30 |
| Augment Intent docs | Official docs citeturn19view6 | official docs | B | Shows “self-maintaining spec” posture and multi-agent coordination | authority posture is much stronger than Punk should adopt early | 2026-04-30 |
| Tessl docs | Official docs citeturn19view7turn19view8 | official docs | B | Strong on persistent evaluated context and review pause before implementation | not primarily a replay/conformance system | 2026-04-30 |
| Architecture recovery studies and ARCADE | papers/tool demo citeturn26view0turn26view1turn26view2turn26view3 | papers / research tool | A | Best evidence on recovering subsystem structure and architectural change/decay | accuracy and manual cost remain significant | 2026-04-30 |
| Requirements traceability overview and benchmarks | chapter/papers citeturn27search4turn25search15turn27search7turn27search20 | chapter / papers | A | Strong overview of traceability tasks, human role, and benchmarking discipline | trace links are not equivalent to intent or correctness | 2026-04-30 |
| Daikon | paper/manual citeturn21view4turn21view5 | paper / manual | A | Canonical source for dynamic invariant detection as likely invariant mining | inferred properties are execution-dependent and must be reviewed | 2026-04-30 |
| Characterization tests and approval tests | practitioner sources citeturn23view0turn23view1 | practitioner article / tool docs | A/B | Strongest practical pattern for preserving actual behavior during rewrites | captures observed behavior, not necessarily desired behavior | 2026-04-30 |
| Pact docs | official docs citeturn20view3turn20view4turn20view5 | official docs | A | Excellent model of contract-by-example, verification results, and compatibility matrix thinking | focused on service integration contracts, not full subsystem replay | 2026-04-30 |
| GraphQL / gRPC / OpenAPI docs | official docs/articles citeturn20view0turn20view1turn20view2 | official docs | A/B | Strong evidence for structural contract extraction and schema transparency | mostly structural; weak on rationale and non-goals | 2026-04-30 |
| Cucumber / Gherkin / EARS / QuickCheck | official docs / papers citeturn31search0turn31search4turn31search5turn31search3 | docs / papers | A/B | Good reference set for executable examples and structured requirements | examples/properties still need maintenance and scope discipline | 2026-04-30 |
| TLA+ / Alloy / session-type sources | papers/docs citeturn21view0turn21view1turn21view2turn21view3turn32search5 | papers / official docs | A/B | Best evidence for bounded, high-value formalization of protocols and invariants | high cost and specialized skills; not universal | 2026-04-30 |
| Round-trip engineering and MDE papers | papers | A/B | Important cautionary literature on synchronization, non-injective transforms, and source-of-truth drift citeturn24search5turn24search16turn24search18 | much of the evidence comes from model/code settings rather than LLM-era code/spec systems | 2026-04-30 |
| RepoBench and RustRepoTrans | benchmark papers citeturn35view2turn35view3 | benchmark papers | A | Useful evidence that repo-level context and migration remain substantially harder than function-level coding | they measure repository understanding/translation, not authority or intent | 2026-04-30 |
| Reproducible Builds and SLSA provenance | official specs/docs citeturn35view0turn35view1 | official specs/docs | A | Best external analogy for verifiable replay/proof discipline | centered on build/source provenance rather than product intent | 2026-04-30 |

## Source reference hygiene

This research note was created from a Deep Research report. Some citations may use Deep Research-local reference ids.

Before this research is used to support an ADR, roadmap promotion, contract, implementation, or eval policy, the cited sources should be materialized into durable source refs such as:

- source URL;
- official repository URL;
- paper DOI/arXiv URL;
- archived source path;
- access date;
- short source id.

Until that cleanup is complete, this note remains advisory and should not be treated as a canonical evidence bundle.

The claims and recommendations may be used to shape follow-up research, ideas, and bounded goals, but not to justify active product behavior without source-ref cleanup.

### Prior art matrix

| Project / tool / paper | Category | Inputs | Outputs | Source-grounding method | Confidence / review model | Replay / regeneration relevance | Risks |
|---|---|---|---|---|---|---|---|
| EEA | editable code-to-spec assists | existing code + pivot | ephemeral partial spec + code change assist | extraction around a user topic | user edits, then accept/reject apply | very high for transposition/replay framing | pivot quality and language ambiguity citeturn18view0turn18view1 |
| SpecLang | spec-as-primary toolchain | natural-language spec | executable code | spec is primary artifact | iterative feedback loop, but public details limited | high as future contrast case | greenfield bias; precision/control burden citeturn18view2 |
| CodeSpeak | takeover / spec-driven system | specs, code, tests, optional sessions | managed code + specs | managed files, scoped specs, tests | alpha preview; human edits present | high for future regeneration research, medium for first safe step | strong spec-as-source ambition; remote/privacy concerns citeturn16view1turn16view2turn16view3turn17search3 |
| Spec Kit | structured spec workflow | prompt + repo + templates | spec / plan / tasks | template-driven process | human steering; extension ecosystem | medium for memory discipline | brownfield and migration support still uneven citeturn14view0turn14view1 |
| Kiro | requirements/design/tasks workflow | prompts + indexed code/docs | requirements, design, tasks, bugfix docs | workspace indexing + structured docs | human review implied, supervised execution features | high for bugfix/transposition patterns | auto-spec context growth can become heavy; still implementation-centric citeturn14view4turn30search0turn30search3turn30search7 |
| Augment Intent | living spec + agent orchestration | codebase + prompts | self-updating spec, coordinated work | spec plus context engine | manual edits allowed | medium/high for long-term direction | self-maintaining spec can overclaim authority for Punk’s early phase citeturn19view6 |
| Tessl | persistent agent context | docs, skills, codebase context | evaluated context bundles + gated spec workflow | version-matched context | explicit approval before implementation | medium for replayability-aware memory | not itself a conformance framework citeturn19view7turn19view8 |
| Cody / Glean / DeepWiki | repo understanding and retrieval | code, docs, activity, permissions | grounded answers, wiki/docs, search results | retrieval over symbols, docs, diffs, activity | advisory | high for evidence gathering | retrieval can be mistaken for truth if not separately governed citeturn19view0turn19view1turn19view2turn19view3turn19view4 |
| Characterization / approval tests | behavior capture | running code + scenarios | observed behavior tests, snapshots, goldens | execution against current system | highly reviewable | very high for behavior replay | preserves bugs/quirks unless reclassified citeturn23view0turn23view1 |
| Pact | contract-by-example | consumer tests + provider verification | contracts, verification results, compatibility matrix | executed examples | explicit verification and deploy checks | very high for bounded API conformance | describes used interactions, not whole domain semantics citeturn20view3turn20view5 |
| Daikon / typestate mining | invariant and protocol mining | traces, tests, executions | likely invariants, mined protocols | dynamic/static analysis | candidate properties, not final truth | high for advisory clauses | false positives and accidental properties citeturn21view4turn32search1turn32search19 |
| TLA+ / Alloy / session types | formal and semi-formal specs | abstract model / protocol | analyzable spec, model checks | formal semantics | explicit human modeling | medium for bounded high-risk subsystems | too expensive as default discipline citeturn21view0turn21view2turn32search5 |
| Architecture recovery / ARCADE | subsystem recovery | source code + dependencies + history | architecture views and change/decay metrics | static structure and semantic clustering | manual verification required | very high for migration seam detection | even best techniques still have limited accuracy citeturn26view0turn26view1turn26view2 |
| Traceability recovery | requirements, code, tests, issues | candidate trace links | IR, embeddings, LLM/RAG | probabilistic ranking + human review | high for linking old to new contracts | links can be convincing but wrong citeturn27search4turn25search15turn27search20 |
| Round-trip engineering / MDE | models + code | synchronized artifacts | transform/synchronize models and code | model/code transformations | depends on tool discipline | high as cautionary precedent | non-injective transforms and source-of-truth drift citeturn24search5turn24search16turn24search18 |
| RepoBench / RustRepoTrans | evaluation benchmarks | repositories and translated tasks | retrieval/completion/translation scores | executable tests and repo context | benchmark scoring | high for evaluation design | says little about intent or authority citeturn35view2turn35view3 |

### Detailed notes on each major project, tool, and research strand

The clearest positive lesson comes from EEA: extracted specifications should be **topic-scoped, editable, partial, and ephemeral**. That matches Punk’s need to produce contract inputs without pretending they are timeless truth. EEA is also unusually explicit about ambiguity, instability, and the “reasonable defaults” problem in language-first coding. That makes it more relevant to Punk than systems that assume the spec can become primary immediately. citeturn18view0turn18view1

SpecLang is best read as a **future contrast case**, not a first template. Its project page is candid that the spec is the main source of truth and generated code is secondary. That is exactly the posture Punk should **not** adopt early, but it is still strategically useful because it shows what kinds of artifacts would need to become far stronger before such a posture is safe. citeturn18view2

CodeSpeak is the nearest public product to Punk’s replay/regeneration frontier. Public docs show mixed-mode work on existing repos, spec-managed file scope, dependency ordering between specs, and a takeover workflow intended to bring code under spec management. The product also explicitly markets intent recovery from agent sessions and equivalence-like goals such as “if we delete the code, an equivalent implementation can be generated from the spec.” The trust-relevant part is not the generation claim itself, but the scoping mechanisms: managed files, explicit project initialization, and warnings when non-managed files change. Those are valuable design cues for Punk. The risky part is that public evidence remains alpha-grade and strongly oriented toward generation after takeover. citeturn16view1turn16view2turn16view3turn17search0turn17search3

Kiro is valuable for a different reason: its bugfix workflow cleanly distinguishes **current behavior**, **expected behavior**, and **unchanged behavior**. That is exactly the classification Punk needs to avoid silently turning every observed quirk into a requirement. Kiro’s EARS-shaped requirements and bugfix specs are good evidence that structured natural-language artifacts can improve reviewability and traceability, but they still need a human decision about what must remain unchanged and what is a defect. Kiro’s own docs also show the practical danger of large spec stores becoming context blobs, which reinforces Punk’s “ledger plus selective views” stance. citeturn14view3turn30search0turn30search2turn30search3turn30search7turn30search5

Augment Intent and Tessl are useful because they foreground **context persistence**, **review checkpoints**, and **spec-guided execution**. The key positive lesson is to preserve structured context and require review before implementation. The key warning is that once a “living spec” becomes self-updating or self-maintaining, it is easy for the spec to become de facto authority without a sufficiently strong promotion model. Punk should borrow their discipline around structure and review, not their early source-of-truth posture. citeturn19view6turn19view7turn19view8

The testing literature is even more important than the LLM literature. Characterization tests document what the system actually does and are specifically useful when refactoring or rewriting; approval tests and golden masters are effective when asserting complex outputs; differential testing compares old and new systems at scale; metamorphic testing adds relations between executions when exact outputs are hard to specify; and contract-by-example tools like Pact operationalize compatibility through executed interactions and explicit verification results. Together these form the best practical foundation for **behavior replay** and **conformance packs**. citeturn23view0turn23view1turn23view3turn22search7turn20view3turn20view5

Formal and semi-formal approaches matter, but only selectively. TLA+ and Alloy prove that bounded, high-value models can check system properties rigorously. Session types and typestate mining show how protocols can be specified or inferred and then checked for conformance. The lesson for Punk is not “formalize everything”; it is “reserve stronger formalisms for bounded subsystems and high-risk interfaces where informal examples are not enough.” citeturn21view0turn21view1turn21view2turn21view3turn32search1turn32search5

Architecture recovery, traceability, and round-trip engineering provide the clearest limits. Architecture-recovery research shows that structure can be recovered, but accuracy remains imperfect and manual verification is costly. Traceability research shows that useful links can be recovered and benchmarked, but links remain probabilistic and human review is essential. Round-trip engineering literature shows repeatedly why synchronized artifacts drift: transformations are partial, many changes are not reversibly representable, and source-of-truth boundaries blur. Punk should take those as design constraints, not historical curiosities. citeturn26view0turn26view1turn27search4turn25search15turn24search5turn24search16

## What replayability can and cannot honestly mean

### Common architectural patterns

Across the best prior art, six patterns recur. First, systems preserve **structured intermediate artifacts** rather than only free-form conversations. Second, they distinguish **requirements / design / tasks / tests / outputs** rather than storing one giant summary. Third, they rely on **execution-backed evidence**—tests, verification results, model checks, or snapshots. Fourth, they attach **source grounding**—file references, schemas, or explicit example traces. Fifth, useful systems require **review checkpoints** before code or acceptance. Sixth, mature systems separate **operational views** from **canonical artifacts**. Those patterns appear in Kiro, Pact, EEA, Tessl, CodeSpeak, and provenance systems like reproducible builds and SLSA. citeturn14view2turn20view5turn18view1turn19view7turn16view2turn35view0turn35view1

The implication for Punk is straightforward: replayability should be engineered as **artifact discipline**, not as a larger prompt. A robust design is a **ledger** of accepted contracts, decisions, evidence refs, examples, invariants, and proof refs, with derived views for onboarding, migration planning, and conformance. That avoids the two classic failure modes: round-trip ambiguity and hidden-memory authority. citeturn24search16turn18view1

### Common failure modes

The dominant failure mode is **false equivalence**: assuming the new implementation is “the same” because tests passed, translation compiled, or examples matched. RustRepoTrans shows that repository-level translation gets much harder once dependencies, cross-file interactions, and architectural constraints enter the picture, and even the best tested models drop significantly relative to simpler settings. Differential testing literature likewise shows that output differences still require human interpretation, especially when some changes are expected. citeturn35view3turn23view3

The second major failure mode is **bug preservation by accident**. Characterization tests are intentionally about actual behavior, not desired behavior. Kiro’s bugfix structure is useful precisely because it forces teams to separate defective behavior from unchanged behavior. Without that separation, behavior replay becomes bug replay. citeturn23view0turn30search0turn30search3

The third failure mode is **artifact drift and false authority**. Documentation drift studies show that outdated code references are common, and round-trip engineering history is full of cases where models and code lose synchrony. Self-updating docs or self-maintaining specs can be valuable, but only if they remain clearly reviewable and non-canonical until accepted. citeturn34search0turn34search14turn34search2turn24search5turn24search16

### What can be preserved during Greenfield Punk work

### Status wording note

In the table below, `adopt-now discipline` means the concept should influence Punk's memory/contract design direction now. It does not mean the field is implemented as active runtime behavior, active CLI behavior, required storage, or mandatory schema for all current contracts.

Use this mapping:

- `adopt-now discipline` - safe to adopt as design direction and future contract-memory expectation;
- `defer` - useful but should wait for a bounded goal/eval;
- `park` - future-only or special-case;
- `avoid` - conflicts with Punk's trust posture.

The biggest strategic advantage of Greenfield Punk work is that it can **record** what Brownfield systems can only infer. The table below classifies what Punk should preserve over time.

| Artifact or field | Why it matters later | Status |
|---|---|---|
| portable intent | future contract transposition needs the stack-neutral “what” and “why” | **adopt-now discipline** |
| stack-specific assumptions | migration risk is mostly hidden here | **adopt-now discipline** |
| examples and fixtures | strongest low-cost behavior replay surface | **adopt-now discipline** |
| validator refs and evidence refs | future conformance and proof need exact provenance | **adopt-now discipline** |
| decision refs and accepted differences | avoids back-solving rationale later | **adopt-now discipline** |
| public API contracts and schemas | migration and compatibility work start here | **adopt-now discipline** |
| domain invariants explicitly accepted | helps distinguish domain truth from implementation quirks | **defer**, begin in bounded areas |
| side-effect model | rewrites break here more often than at pure logic | **defer** |
| error semantics | critical for external compatibility | **defer** |
| security/privacy constraints | needed for safe migration, but often costly to capture fully | **defer** |
| performance expectations / envelopes | important for some subsystems, but expensive everywhere | **defer** |
| state-transition models | very valuable for protocol-heavy subsystems | **park outside selected high-risk areas** |
| formal models | high-value only when justified | **park** |
| regeneration hints | too close to premature spec-as-source | **park** |
| automatic generated code as canonical | conflicts with current trust posture | **avoid** |

This recommendation is supported by the relative maturity of examples, schemas, contract tests, characterization tests, provenance, and structured bugfix specs compared with the much weaker state of full round-trip regeneration. citeturn20view3turn20view5turn20view1turn20view2turn23view0turn35view0turn35view1turn30search0

### What can only be reconstructed in Brownfield

Brownfield recovery can recover a surprising amount, but it is weaker than recorded memory at precisely the moments Punk cares about most. Structure, public interfaces, tests, fixtures, schemas, dependency maps, and parts of decision history are often recoverable. Original intent, non-goals, accepted differences, and the boundary between “quirk” and “requirement” usually are not. That asymmetry is exactly why replayability-aware memory is worth recording from day zero. citeturn26view0turn27search4turn23view0turn34search0

### What cannot be reconstructed reliably

The hardest things to reconstruct later are: original stack-neutral intent, explicit non-goals, which behaviors were knowingly preserved for compatibility, which defects were accepted temporarily, and which performance or security properties were actually contractually important. Traceability can suggest links, history can suggest rationale, and mined invariants can suggest patterns; none of them can turn those suggestions into authoritative truth automatically. citeturn27search4turn27search20turn21view4turn26view2

### Project Entry Mode, Artifact Authority Mode, and Replayability Mode

These are useful dimensions, with one refinement: **Entry Mode** describes evidence confidence, **Authority Mode** describes what artifact leads future work, and **Replayability Mode** describes what future work is safely supported.

| Dimension | Values | Use in Punk |
|---|---|---|
| Project Entry Mode | greenfield / brownfield / grayfield | drives confidence and reconstruction burden |
| Artifact Authority Mode | code-first discipline / spec-anchored work / intent-native workflow / future spec-as-source experiment | tells Punk what is allowed to constrain code today |
| Replayability Mode | audit / intent / contract / behavior / migration / regenerative | tells Punk what future operations are justified |

That three-axis model is better than a single maturity ladder because a project may have high audit replay and low behavior replay, or strong behavior replay without spec-as-source authority. The public ecosystem already shows these combinations. citeturn35view0turn20view5turn19view6turn18view1

### Replayability levels

| Level | Meaning | Safe use |
|---|---|---|
| RL-0 Audit replay | goals, contracts, runs, receipts, decisions, proofs trace what happened | audit, onboarding, proof navigation |
| RL-1 Intent replay | rationale, assumptions, research refs, non-goals, unknowns are preserved | planning, maintenance, impact analysis |
| RL-2 Contract replay / transposition | accepted contracts can seed new contracts | migration contract drafting with review |
| RL-3 Behavior replay | examples, fixtures, tests, schemas, accepted invariants and differences preserved | conformance planning and regression control |
| RL-4 Migration replay | a bounded subsystem can be reimplemented and checked against explicit equivalence policy | subsystem rewrite / language migration |
| RL-5 Regenerative replay | high-level artifact is strong enough to regenerate implementation | future-only experiment |

Punk can realistically aim for RL-0 through RL-3 in the medium term, RL-4 only for bounded subsystems with a conformance pack, and RL-5 only after substantial evaluation and governance maturity. citeturn35view0turn35view1turn23view0turn20view5turn35view3

### Greenfield recorded replay memory vs Brownfield reconstructed replay candidates

Greenfield Punk memory should be treated as **recorded authority candidates** with better provenance. Brownfield replay artifacts should be treated as **reconstructed candidates** with weaker confidence. Grayfield needs explicit reconciliation: preserve the recorded chain where it exists, reconstruct only the gaps, and never let reconstructed candidates overwrite recorded accepted artifacts. This is an inference from provenance discipline plus traceability and reconstruction research. citeturn35view1turn27search4turn26view0

## Punk model

### Contract transposition model

A transposed contract should not be generated by textual substitution. It should be built from four inputs: (a) prior accepted contract(s), (b) current accepted decisions and constraints, (c) a conformance pack for the subsystem, and (d) new stack-specific unknowns. The portable parts of a prior contract are usually intent, public behavior, public schemas, accepted examples, and accepted invariants. The non-portable parts are dependency choices, runtime assumptions, deployment details, performance tricks, error-library conventions, and internal structure. citeturn20view3turn20view5turn20view1turn20view2turn35view3

Recommended transposition logic:

1. Identify the bounded subsystem and prior accepted contract set.
2. Extract portable clauses and mark stack-specific clauses explicitly.
3. Attach the subsystem’s conformance pack.
4. Add migration-specific unknowns and accepted-difference candidates.
5. Draft a new migration contract.
6. Require human review of portability and differences before `cut`.
7. Require an explicit equivalence policy before `gate`.

This is closer to “contract inheritance with review” than to “spec replay.” It mirrors how Pact separates consumer expectations from provider verification and how Kiro separates unchanged behavior from the fix. citeturn20view3turn20view5turn30search0turn30search3

### Regenerative Spec Pack and Conformance Pack model

The best naming split is:

- **Replayable Project Memory** — strategic concept
- **Conformance Pack** — first bounded technical artifact
- **Migration Contract Pack** — future contract-support bundle derived from prior contracts + conformance pack
- **Regenerative Spec Pack** — future-only experiment, parked for now

A **Conformance Pack** for one subsystem should include:

| Included item | Necessary for bounded rewrite? | Review level |
|---|---|---|
| subsystem boundary and public entry points | yes | human review |
| public API/schema contracts | yes | human review |
| accepted examples/fixtures/golden outputs | yes | human review |
| characterization tests or behavior snapshots | yes | human review |
| accepted invariants and property tests where available | strongly recommended | human review |
| side-effect model and external dependency map | strongly recommended | human review |
| error behavior and accepted differences register | strongly recommended | high review |
| security/privacy constraints if relevant | required for sensitive subsystems | high review |
| performance envelope if relevant | required when performance is contractually important | high review |
| source refs to originating contracts / decisions / proof refs | yes | low review once provenance is sound |
| generation instructions | no | park |
| formal full-state model | optional for special cases | park/defer |

This pack should be **source-linked, reviewable, and non-self-executing**. It is not a gate decision. It is not a proofpack. It is a preparation artifact for a later migration contract and a later equivalence gate. citeturn23view0turn23view1turn20view3turn20view5turn21view4turn31search0

### Equivalence and conformance model

A realistic equivalence policy should separate **exact equivalence** from **acceptable compatibility**. Exact equivalence is rare outside pure computation or narrow data transforms. For most subsystem rewrites, Punk should define an explicit checklist of conformance dimensions:

- public API behavior,
- data/schema compatibility,
- state-transition compatibility where relevant,
- error semantics,
- side-effect behavior,
- security/privacy invariants,
- performance envelope where contractually required,
- user-visible behavior,
- example/fixture compatibility,
- protocol compatibility. citeturn20view3turn20view5turn20view1turn20view2turn32search6turn32search5

A migration contract should therefore carry an **equivalence policy** with statuses such as:

| Status | Meaning |
|---|---|
| must_preserve | required compatibility surface |
| may_change | implementation detail or deliberate redesign space |
| bug_candidate | currently observed, not accepted as future requirement |
| legacy_quirk | preserved for compatibility unless explicitly retired |
| deprecated | allowed short-term, should be removed by later contract |
| accepted_difference | reviewed difference accepted for the new implementation |
| unknown | insufficient evidence |
| requires_review | cannot be decided automatically |

Punk should require the old and new implementations to be compared against this policy using the conformance pack. Differential testing, contract verification, golden-example comparison, property-based checks, and where relevant model checking all contribute evidence. DiffKemp is instructive here: semantic difference tools can reveal incompatibilities, but they still work against an explicit comparison target and stated assumptions. citeturn23view3turn20view5turn31search3turn28search3turn28search11

### Bug vs requirement classification

This is the classification Punk most needs to make explicit.

| Class | Detection hints | Default authority | Can become preserved requirement automatically? |
|---|---|---|---|
| intended behavior | accepted contract, decision, maintained docs, reviewed examples | high | yes, if already accepted |
| observed behavior | tests, fixtures, characterization, runtime traces | medium | no |
| public compatibility requirement | consumers depend on it; provider/consumer contract or compatibility policy says preserve | high after review | only after review |
| implementation detail | internal-only behavior without public contract relevance | low | no |
| legacy quirk | known oddity preserved for compatibility | medium after review | no automatic promotion |
| bug candidate | current behavior conflicts with accepted intent or expected behavior | low | no |
| temporary workaround | issue/decision explicitly marks it transitional | low | no |
| deprecated behavior | still present but scheduled for removal | medium after review | only until superseded |
| unknown | conflicting or missing evidence | none | no |

Feathers’ characterization-testing warning and Kiro’s current/expected/unchanged split should become core Punk discipline here. Characterization is evidence; it is not a final classification. citeturn23view0turn30search0turn30search3

### Mapping to Punk Contract Schema Blueprint v0.1

| Contract section | Replayability role | Migration role | Must preserve? | Stack-neutral? | Stack-specific risk | Eval / proof requirement |
|---|---|---|---|---|---|---|
| `identity` | links contracts across time | identifies subsystem continuity | yes | mostly | low | provenance/citation correctness |
| `authority` | shows who accepted what | critical for which old claims may constrain new work | yes | mostly | medium | review of authority chain |
| `lifecycle` | records acceptance and promotion path | defines how migration work enters `plot -> cut -> gate` | yes | mostly | medium | process proof |
| `work.intent` | core of intent replay | seeds portable migration purpose | yes | partly | high | human review, contradiction checks |
| `boundaries` | preserves subsystem seam | essential for reimplementation scope | yes | partly | high | boundary accuracy eval |
| `clauses` | preserve accepted behavioral and non-behavioral promises | portable clauses become migration constraints | yes selectively | mixed | very high | clause portability review |
| `validation` | preserves validator refs and examples | becomes conformance candidate set | yes | mixed | medium | validator quality + completeness |
| `evidence` | preserves proof-bearing artifacts | supplies migration oracle and provenance | yes | mostly | low | evidence completeness |
| `receipt_requirements` | enables audit replay | enables comparable receipts across implementations | defer selectively | mostly | medium | receipt schema checks |
| `gate_policy` | records what acceptance depended on | defines migration equivalence gate inputs | defer | mixed | high | policy explicitness |
| `proof_requirements` | links accepted artifacts and proofpack structure | must describe future equivalence proof hints | defer | mixed | high | proofpack completeness |
| `change_control` | captures assumptions and supersession | needs migration assumptions and accepted differences | yes selectively | partly | high | supersession hygiene |
| `memory_links` | forms replay graph | links prior contracts to migration contracts and conformance pack | yes | mostly | low | link integrity |

Specific design recommendations:

- `work.intent` should eventually distinguish **portable intent** from **stack-specific assumptions**.
- `boundaries` should distinguish **domain boundary** from **implementation boundary**.
- `clauses` should distinguish **portable clauses** from **stack-specific clauses** and **conformance clauses** from **implementation clauses**.
- `validation` should allow **conformance candidates** such as golden examples, Pact verification, characterized behavior, and property tests.
- `evidence` should preserve examples, fixtures, golden files, schema snapshots, and proof refs explicitly.
- `proof_requirements` should allow future equivalence hints, but only as hints until a migration contract demands them.
- `change_control` should record accepted differences and migration assumptions.
- `memory_links` should explicitly connect prior accepted contracts, decisions, and proofs to any migration contract.

These follow directly from the evidence above: executable examples, contract verification, provenance, and reviewable structured specs are much more dependable than holistic regeneration claims. citeturn20view3turn20view5turn31search0turn35view1turn23view0

## Evaluation, security, governance, and adoption

### Eval and benchmark plan

Punk needs two evaluation layers: **artifact quality** and **downstream usefulness**.

Artifact-quality evals should measure:

- intent replay quality,
- citation coverage and citation correctness,
- contract transposition quality,
- behavior replay coverage,
- bug-preservation rate,
- accepted-difference quality,
- stack-specific assumption detection,
- unknowns quality,
- proof-evidence completeness,
- false-equivalence rate. citeturn27search4turn25search15turn35view3turn35view2

Downstream-usefulness evals should measure:

- time to first safe migration contract,
- maintainer correction rate,
- migration contract completeness,
- conformance-pack completeness,
- equivalence-gate precision,
- blocker detection,
- quality of accepted differences,
- quality of refusal when evidence is insufficient. citeturn23view3turn35view3turn20view5

Useful external benchmarks exist, but each measures only part of the problem:

| Benchmark family | What it measures | What it does **not** measure | Relevance to Punk |
|---|---|---|---|
| RepoBench | repository-level retrieval/completion pipeline | intent, authority, migration governance | useful for context/retrieval quality citeturn35view2 |
| RustRepoTrans / repo translation benchmarks | repository-level migration difficulty and translation failures | whether preserved behavior and intent were the right ones | useful for migration difficulty realism citeturn35view3turn28search8 |
| traceability benchmarks / TraceLab | link recovery precision/recall | contract portability, accepted differences | useful for evidence-link quality citeturn25search15turn27search20 |
| invariant/protocol mining evaluations | candidate invariant extraction | whether mined invariant should become contract | useful for advisory clause mining citeturn21view4turn32search1 |
| differential/metamorphic testing studies | regression and oracle-power patterns | authority classification | useful for conformance-test design citeturn23view3turn22search7 |

The missing benchmark category is specifically Punk-shaped: **Does replayable memory make migration contracts safer and faster without increasing false authority?** No public benchmark directly answers that today. citeturn35view2turn35view3turn27search4

### Security, privacy, and local-first constraints

Punk’s first replayability steps should remain **local-only by default**. Repo-indexing products and spec-driven tools are often cloud-connected; provenance systems and reproducible-build practices show that strong trust depends on explicit, inspectable artifacts and verifiable recording, not hidden service behavior. For Punk, that implies: no-network default, explicit path allowlists, local indexing only, transcript import only by opt-in, redaction before rendering reports or packs, and receipts that state exactly which source classes were touched. citeturn19view2turn29view1turn17search4turn35view0turn35view1

Special sensitivity applies to agent transcripts, issue/PR history, and generated artifacts that may reveal private behavior or licensed third-party snippets. Those should be treated as secondary evidence with stronger review/redaction requirements. Public evidence for stable, normalized session schemas remains uneven, which is another reason not to make transcripts a default authority source. citeturn17search0turn13search0turn11search18turn11search20

### Source of truth and authority model

The governance model should be:

- **recorded** vs **reconstructed**,
- **candidate** vs **accepted**,
- **portable** vs **stack-specific**,
- **observed** vs **intended**,
- **derived view** vs **canonical artifact**.

A conformance pack should therefore carry:

```text
claim ids
source refs
authority status
confidence
review status
accepted differences
unknowns
superseded-by links
review date
gate links if promoted
```

And Punk should preserve these hard rules:

- old behavior is not automatically intent,
- tests are not automatically proof,
- generated specs are not automatically contracts,
- conformance packs are not gate decisions,
- migration contracts must state equivalence criteria explicitly,
- regenerative packs remain future-only. citeturn23view0turn20view5turn18view1turn35view1

### Adoption map

| Category | Recommendation | Rationale |
|---|---|---|
| adopt now | replayability-aware memory fields; explicit portable intent vs stack assumptions; examples/fixtures/evidence refs; accepted-difference register; decision refs; research note and ADR boundary | strongest value, lowest authority risk |
| defer | bounded subsystem conformance pack; migration contract pack; side-effect modeling; error-semantics recording; limited invariant capture | highly useful, but needs evaluation and review tooling |
| park | regenerative spec pack; broad formal models by default; automatic cross-stack regeneration; full-project replay | too much unearned authority |
| avoid | deterministic replay promises; equating Brownfield reconstruction with Greenfield proof; generated code or specs as canonical source early; remote analysis by default; silent acceptance of differences | directly conflicts with Punk’s trust model |

### Proposed first Punk research note outline

**Recommended file:** `knowledge/research/2026-04-30-replayable-project-memory.md`

Suggested outline:

- problem statement and trust posture,
- replay vs transposition vs regeneration,
- replayability levels,
- what Greenfield Punk should preserve,
- conformance pack concept,
- contract transposition rules,
- anti-patterns,
- open evaluation questions,
- links to ADR and eval spec.

### Proposed ADR outline

**Recommended file:** `docs/adr/ADR-XXXX-replayable-project-memory-boundary.md`

Suggested outline:

- context,
- decision,
- status: proposed,
- scope boundary,
- candidate vs accepted model,
- portable vs stack-specific distinction,
- conformance pack boundary,
- migration contract boundary,
- local-only and redaction constraints,
- consequences,
- rejected alternatives.

### Proposed eval spec outline

**Recommended file:** `evals/specs/replayability-conformance-pack.v0.1.md`

Suggested outline:

- goal,
- bounded-subsystem selection criteria,
- artifact schema under test,
- gold-label process for intent/bug/compatibility classes,
- conformance dimensions,
- metric definitions,
- citation checks,
- false-equivalence checks,
- accepted-difference review checks,
- maintainer usefulness study,
- promotion criteria for any later runtime feature.

### Open questions

The biggest unresolved questions are not about retrieval or translation alone. They are about **governance and classification**:

- How should Punk calibrate confidence across recorded vs reconstructed evidence?
- What is the minimal accepted-difference model that maintainers will actually use?
- How can stack-specific assumptions be surfaced early enough to help transposition without bloating every contract?
- When is a property test or mined invariant good enough to move from advisory evidence into accepted contract support?
- What are good, low-cost ways to preserve side-effect models for ordinary projects?
- How should transcript evidence be normalized and redacted locally without becoming hidden memory? citeturn21view4turn27search4turn35view3turn17search0

## Punk integration guardrails

### What this research changes now

This research changes the strategic direction and idea backlog.

It supports preserving replayability-aware memory as a design quality:

```text
intent
contracts
evidence
decisions
examples
invariants
proof refs
accepted differences
```

It also supports the vocabulary:

```text
Replayable Project Memory
Contract Transposition
Conformance Pack
Migration Contract Pack
Regenerative Spec Pack
```

### What this research does not change now

This research does not change:

- active CLI surface;
- runtime storage;
- current contract lifecycle implementation;
- gate/proof writer activation;
- Writer activation;
- module/adapter status;
- current selected implementation goal;
- active project-memory authority model.

### Adoption map for Punk

| Recommendation | Scope |
|---|---|
| adopt now | strategic concept, vocabulary, idea artifact, research note |
| adopt now as discipline | preserve replayable obligations where current contracts/reports already support it |
| defer | Conformance Pack boundary, Migration Contract Pack, accepted-difference model, bug-vs-requirement classification, conformance/equivalence evals |
| park | Regenerative Spec Pack, full-project replay, spec-as-source authority, automatic code generation |
| avoid now | deterministic replay promises, generated specs as canonical truth, Conformance Pack as gate decision, preserving bugs as requirements |

### Promotion path

Any future Conformance Pack or Migration Contract Pack must be promoted through:

```text
research
  -> ADR / roadmap decision
  -> bounded goal / contract
  -> eval spec
  -> implementation if approved
  -> gate/proof
```

## Conformance Pack boundary

A Conformance Pack is the first plausible future artifact suggested by this research.

It should be understood as:

```text
a source-linked, reviewable artifact set that defines what a future implementation must preserve, may change, or must explicitly review
```

It may include:

- subsystem boundary;
- public entry points;
- public API/schema contracts;
- accepted examples/fixtures/golden outputs;
- characterization tests or behavior snapshots;
- accepted invariants and property tests where available;
- side-effect model and external dependency map;
- error behavior;
- accepted differences register;
- security/privacy constraints when relevant;
- performance envelope when relevant;
- source refs to originating contracts, decisions, and proof refs.

It must not:

- write gate decisions;
- act as a proofpack;
- accept work;
- become canonical truth automatically;
- generate code by default;
- replace contracts.

It is an input to a future migration/reimplementation contract and an equivalence/conformance gate.

## Contract Transposition boundary

Contract Transposition means using prior accepted contracts and evidence as inputs to a new migration/reimplementation contract.

It does not copy acceptance from the old contract.

It does not approve the new work.

It does not mean stack-specific implementation steps are portable.

A transposed contract draft should cite:

- prior accepted contract refs;
- prior gate/proof refs;
- relevant decision refs;
- Conformance Pack refs, if present;
- new stack-specific assumptions;
- unknowns and accepted-difference candidates.

The new migration/reimplementation work still requires:

```text
plot -> cut -> gate
```

and a new gate decision/proof before acceptance.

## Bug vs requirement boundary

Observed behavior is not automatically intended behavior.

Characterization tests, snapshots, logs, and golden outputs are evidence of what happened. They are not proof that the behavior should be preserved.

Future conformance work should classify behavior as one of:

- `must_preserve`
- `may_change`
- `bug_candidate`
- `legacy_quirk`
- `deprecated`
- `unknown`
- `requires_review`
- `accepted_difference`

Promotion of observed behavior into `must_preserve` requires review and, when used for acceptance, gate/proof.

## Decision Log candidate

**Date:** 2026-04-30
**Project:** Punk
**Decision:** Adopt Replayable Project Memory as a strategic concept, scoped to replayable obligations and bounded conformance rather than code regeneration.
**Reason:** Current evidence supports preserving intent, accepted behavior, examples, decisions, evidence, and provenance for future migration/reimplementation contracts, but does not support trustworthy full-project regeneration.
**What this prevents:** False equivalence, preserving bugs as intent, premature spec-as-source authority, generated specs as canonical truth, and Conformance Packs becoming decision surfaces.
**Review date:** 2026-07-30

## Recommended next artifacts

1. `knowledge/ideas/2026-04-30-replayability-backlog.md`
   - backlog of deferred/parked replayability ideas.

2. `docs/adr/ADR-XXXX-replayable-project-memory-boundary.md`
   - proposed ADR only after research source refs are durable.

3. `evals/specs/replayability-conformance-pack.v0.1.md`
   - future eval spec for Conformance Pack boundary.

4. `work/goals/goal_define_conformance_pack_boundary_v0_1.md`
   - future bounded goal, not selected now unless roadmap/work ledger chooses it.

## Final recommendation

Punk **should** adopt **Replayable Project Memory** as a strategic concept, but with a strict definition: not deterministic regeneration, and not a promise that old artifacts can fully recreate the project, but a commitment to preserving enough portable intent, accepted behavior, decisions, evidence, and provenance that future migration or reimplementation can be turned into a bounded contract and verified. That conclusion is strongly supported by the combined evidence from EEA, executable examples, characterization testing, Pact-style verification, provenance systems, and the current limits of repository-level translation. citeturn18view1turn23view0turn20view5turn35view0turn35view1turn35view3

From day zero in Greenfield projects, Punk should preserve: portable intent, stack-specific assumptions, public API and schema links, accepted examples and fixtures, validator refs, evidence refs, decision refs, accepted differences, and proofs of acceptance. Brownfield projects should only reconstruct those as **candidates** unless and until they are explicitly reviewed. Recorded Greenfield memory and reconstructed Brownfield candidates must never be treated as equal in authority. citeturn27search4turn35view1

The first safe artifact is **not** a Regenerative Spec Pack. It is a **replayability-aware extension to Punk’s memory and contract discipline**, followed by a **bounded Conformance Pack** for one subsystem when a rewrite or migration is actually being planned. A **Migration Contract Pack** can come later as a derived bundle that cites accepted prior contracts and the conformance pack. A Regenerative Spec Pack should remain parked. citeturn23view0turn20view5turn35view3turn24search16

The following must stay advisory:

- inferred invariants,
- characterization-derived behavior until classified,
- migration assumptions,
- accepted-difference proposals before review,
- any transcript-derived rationale,
- any generated natural-language “living spec” that has not been reviewed. citeturn21view4turn23view0turn17search0

The following require human review:

- portable intent,
- bug vs requirement classification,
- stack-specific assumption detection,
- accepted differences,
- side-effect model,
- security/privacy constraints,
- performance envelope clauses,
- any transposed contract. citeturn30search0turn23view0turn35view3

The following require `gate` and proof before acceptance:

- promotion of replayability artifacts into accepted project memory,
- acceptance of a conformance policy,
- acceptance of a migration contract,
- acceptance that a new implementation conforms sufficiently,
- any later runtime feature that persists accepted replayability state. citeturn20view5turn35view1

The following should remain parked:

- full-project replay,
- autonomous agent execution from replay artifacts,
- active code generation from replay artifacts by default,
- spec-as-source project authority,
- deterministic round-trip promises,
- remote analysis by default. citeturn24search5turn24search16turn29view1

The main remaining research gap is a Punk-specific evaluation framework for **safe migration usefulness**: not just whether a model can translate code, but whether preserved memory helps maintainers create better migration contracts, surface the right differences, avoid preserving bugs, and refuse unsafe claims when evidence is weak. citeturn35view3turn27search4turn25search15

The exact next artifacts Punk should create are:

- `knowledge/research/2026-04-30-replayable-project-memory.md`
- `knowledge/ideas/2026-04-30-replayability-backlog.md`
- `docs/adr/ADR-XXXX-replayable-project-memory-boundary.md`
- `evals/specs/replayability-conformance-pack.v0.1.md`

A concise decision-log candidate is:

**2026-04-30 / Punk / Adopt Replayable Project Memory as a strategic concept, but scope the first step to replayability-aware memory fields and bounded Conformance Packs rather than regeneration / Reason: current evidence strongly supports preserved intent + behavior + provenance for future migration, but not trustworthy full regeneration / Prevents: false equivalence, bug preservation as intent, premature spec-as-source authority, and hidden-memory drift / Review date: 2026-07-30.**
