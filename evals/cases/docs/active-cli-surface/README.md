# Active CLI surface cases

Fixture-style cases for guarding active/current Punk CLI documentation.

Scope of this slice:

- hard-fail when active/current docs describe unimplemented Punk CLI commands as implemented or active behavior;
- allow future/target/deferred command mentions when clearly labeled;
- keep the implemented CLI truth limited to `punk init <project-id>`, `punk init <project-id> --mode brownfield`, `punk flow inspect`, `punk publishing locate`, `punk publishing locate [--project-root <path>] [--json]`, `punk eval run smoke`, and `punk eval run smoke --format json`.

Check family name used in these fixtures:

- `docs_active_cli_surface_truth`
