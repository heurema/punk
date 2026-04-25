# Karpathy-style executor failures in Punk workflows

These examples show how executor-discipline failures should be caught by Punk artifacts instead of becoming global prompt rituals.

| Failure | Wrong flow | Punk-correct flow | Artifact that catches it |
|---|---|---|---|
| Hidden assumption | Executor silently chooses an interpretation and edits files. | `plot` records assumptions and ambiguity; unclear choices become contract clauses or questions before `cut`. | Contract clause; memory link if repeated. |
| Overengineered solution | Executor adds a generic framework, new options, and extension points for a narrow task. | Contract states minimal success criteria and non-goals; review checks every changed file against the goal. | Eval spec; gate; receipt/evidence field. |
| Drive-by refactor | Executor reformats adjacent code and renames helpers while fixing a small issue. | Allowed paths and acceptance criteria constrain edits; unrelated cleanup becomes a separate goal. | Contract allowed paths; diff checks; surgical-change eval spec. |
| "Tests passed" self-report | Executor says tests passed but no command output or artifact is captured. | Receipt records exact commands and outputs; validators rerun or mark the claim unverified. | Receipt/evidence field; validator; proofpack. |
| Semantic assessor decides | LLM review declares the change accepted. | Semantic assessor emits advisory clause-specific evidence; `gate` writes the final decision. | Gate; assessment-vs-decision boundary; proofpack. |
| Prompt or skill becomes authority | A persistent skill says a behavior is accepted even though the contract does not. | Runner aids are advisory and scoped; durable aids require evidence, owner, and review/retirement path. | Research Gate; eval spec; project-memory link. |
| Broad "improve architecture" request | Executor rewrites multiple docs and proposes runtime surfaces. | `plot` narrows the request to one decision, explicit non-goals, and required research/eval refs. | Contract clause; Research Gate; gate. |
| Protected path touched | Executor edits runtime code while the contract is docs-only. | Protected paths block acceptance unless the contract is amended before work. | Contract protected paths; validator; gate. |
| Unavailable check treated as pass | Executor cannot run a validator and reports success anyway. | Receipt marks the check unavailable; gate accepts, rejects, or defers according to policy. | Receipt/evidence field; gate; proofpack. |
| Public overclaim | Docs say Punk executes coding-agent work today. | Public/operator docs preserve active-core vs future boundaries and cite canonical truth. | Executor-agnostic boundary eval; docs governance; gate. |

Rule of thumb: a repeated executor failure should become a contract clause, validator, receipt/evidence field, eval case, proof requirement, or project-memory artifact before it becomes a global instruction.
