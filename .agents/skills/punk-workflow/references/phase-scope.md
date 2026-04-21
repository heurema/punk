# Current phase scope

Current active direction:

- Dogfooding Level 0 with repo-tracked work artifacts.
- Phase 1 foundations for flow and event trust surfaces.
- Small bounded diffs that preserve inspectable state and evidence.

Current active implementation order:

1. pure flow state kernel
2. append-only event log
3. flow inspect command
4. smoke eval harness

Not active now unless a selected goal explicitly says otherwise:

- `.punk/` runtime persistence
- plugins, adapters, or module host
- Knowledge Vault as project truth
- PubPunk automation
- SaaS or cloud sync surfaces
- UI-first workflow

If a planned diff touches one of these not-active surfaces, stop and re-check `docs/product/START-HERE.md`, `docs/product/ROADMAP.md`, and the selected goal before editing.
