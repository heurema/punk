//! Project identity and Level 0 manual project-memory initialization.

use std::fmt::Write as _;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROJECT_INIT_SCHEMA_VERSION: &str = "project-init-greenfield.v0.1";
pub const PROJECT_INIT_BROWNFIELD_SCHEMA_VERSION: &str = "project-init-brownfield-scaffold.v0.1";
pub const PROJECT_INIT_MODE: &str = "manual-project-memory-level0";
pub const PROJECT_INIT_ENTRY_MODE: &str = "greenfield";
pub const PROJECT_INIT_BROWNFIELD_ENTRY_MODE: &str = "brownfield";
pub const PROJECT_INIT_RUNTIME_PERSISTENCE: &str = "inactive";
pub const PROJECT_ID_FORMAT_NOTE: &str =
    "project id must be a lowercase ASCII slug: a-z, 0-9, and hyphen, starting and ending with a letter or digit";
pub const SOURCE_CORPUS_MANIFEST_MODEL_SCHEMA_VERSION: &str =
    "brownfield-source-corpus-manifest-model.v0.1";
pub const SOURCE_CORPUS_MANIFEST_SCHEMA_VERSION: &str = "brownfield-source-corpus-manifest.v0.1";
pub const INSTRUCTION_PAGE_INDEX_SCHEMA_VERSION: &str = "punk-instruction-page-index.v0.1";

const MEMORY_ROOT: &str = ".punk/memory";
const STATUS_PATH: &str = ".punk/memory/STATUS.md";
const INITIAL_GOAL_PATH: &str = ".punk/memory/goals/goal_initial_project_setup.md";
const BROWNFIELD_BASELINE_GOAL_PATH: &str =
    ".punk/memory/goals/goal_brownfield_reconstruction_baseline.md";
pub const INSTRUCTIONS_ROOT: &str = ".punk/instructions";
pub const INSTRUCTIONS_INDEX_PATH: &str = ".punk/instructions/INDEX.md";
pub const INSTRUCTION_PAGES_ROOT: &str = ".punk/instructions/pages";
pub const INSTRUCTION_MODULES_ROOT: &str = ".punk/instructions/modules";
pub const INSTRUCTION_PAGE_INDEX_VIEW_PATH: &str = ".punk/views/instructions/page-index.json";

fn work_status_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: TODO
ledger_version: work-ledger.v0.1
dogfooding_level: 0
project_id: "{project_id}"
entry_mode: greenfield
updated_at: TODO
current_phase: "Dogfooding Level 0 / compact project memory"
current_focus: "Initial project setup"
selected_next: "{INITIAL_GOAL_PATH}"
last_validated_commit: null
---

# Work Status

## Now

- Current stage: greenfield project initialized with Punk Level 0 compact manual memory.
- Current focus: complete initial project setup.
- Selected next: `{INITIAL_GOAL_PATH}`

## Blockers

- Fill owner/date TODOs before treating this ledger as current.

## Recent Evidence

- `punk init {project_id}` created the greenfield Level 0 scaffold.

## Open Drift Findings

None.
"#,
        project_id = project_id.as_str()
    )
}

fn initial_goal_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: goal_initial_project_setup
title: "Initial project setup"
status: ready
owner: TODO
module: "project"
priority: P1
authority: canonical
project_id: "{project_id}"
entry_mode: greenfield
created_at: TODO
updated_at: TODO
selected_at: TODO
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - "docs/**"
    - "design/**"
    - ".punk/README.md"
    - ".punk/project.toml"
    - ".punk/instructions/**"
    - ".punk/memory/**"
  exclude:
    - "work/**"
    - "knowledge/**"
    - "docs/adr/**"
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
acceptance:
  - "The canonical product/design/source documents are listed with repo-relative refs."
  - "Known constraints, non-goals, and acceptance criteria for the first implementation slice are recorded."
  - "Project id and greenfield entry mode remain visible in .punk/memory/STATUS.md."
  - "The next bounded implementation goal is created and selected in .punk/memory/STATUS.md."
knowledge_refs: []
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
research_gate:
  classification: R0
  required: false
  rationale: "Initial project setup only inventories user-provided/repo-tracked project inputs and does not change architecture."
  research_refs: []
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: initial-project-memory
  required_updates:
    - ".punk/memory/STATUS.md"
    - ".punk/memory/reports/**"
  rationale: "Initial project truth capture establishes the manual Level 0 memory baseline."
---

## Context

The project has been initialized as a Punk greenfield project with Level 0 manual project memory.

Project id: `{project_id}`

## Intent

Capture the current product, design, technical constraints, non-goals, and first implementation boundary before coding starts.

## Non-scope

Do not implement product behavior in this goal.

Do not write `.punk/` runtime stores.

Do not claim gate acceptance or proofpack coverage.
"#,
        project_id = project_id.as_str()
    )
}

fn brownfield_work_status_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: work_status
kind: manual-work-ledger
status: active
authority: canonical
owner: TODO
ledger_version: work-ledger.v0.1
dogfooding_level: 0
project_id: "{project_id}"
entry_mode: brownfield
reconstruction_status: not_started
updated_at: TODO
current_phase: "Dogfooding Level 0 / brownfield reconstruction scaffold"
current_focus: "Prepare brownfield reconstruction baseline"
selected_next: "{BROWNFIELD_BASELINE_GOAL_PATH}"
last_validated_commit: null
---

# Work Status

## Now

- Current stage: brownfield project initialized with Punk Level 0 advisory reconstruction workspace.
- Current focus: prepare a future source-linked brownfield reconstruction baseline.
- Selected next: `{BROWNFIELD_BASELINE_GOAL_PATH}`
- Reconstruction status: not_started.

## Brownfield Boundary

No project knowledge has been reconstructed yet.
Existing code/docs/history are not Punk truth.
Future reconstruction artifacts are advisory candidates until reviewed.

## Blockers

- Fill owner/date TODOs before treating this ledger as current.
- Create and review future source-linked reconstruction artifacts before drafting contracts.

## Recent Evidence

- `punk init {project_id} --mode brownfield` created the brownfield entry scaffold.

## Open Drift Findings

None.
"#,
        project_id = project_id.as_str()
    )
}

fn brownfield_baseline_goal_template(project_id: &ProjectId) -> String {
    format!(
        r#"---
id: goal_brownfield_reconstruction_baseline
title: "Brownfield reconstruction baseline"
status: ready
owner: TODO
module: "project"
priority: P1
authority: canonical
project_id: "{project_id}"
entry_mode: brownfield
reconstruction_status: not_started
created_at: TODO
updated_at: TODO
selected_at: TODO
started_at: null
completed_at: null
blocked_by: []
scope:
  include:
    - ".punk/memory/STATUS.md"
    - ".punk/memory/reconstruction/**"
    - ".punk/memory/reports/**"
    - ".punk/instructions/**"
  exclude:
    - "work/**"
    - "knowledge/**"
    - "docs/adr/**"
    - ".punk/events/**"
    - ".punk/contracts/**"
    - ".punk/runs/**"
    - ".punk/evals/**"
    - ".punk/decisions/**"
    - ".punk/proofs/**"
    - ".punk/indexes/**"
    - ".punk/views/**"
    - ".punk/runtime/**"
    - ".punk/cache/**"
acceptance:
  - "A future source corpus manifest boundary is defined before any inventory is generated."
  - "Future claim-ledger, unknowns, contradictions, and contract-readiness artifacts remain advisory until reviewed."
  - "No project knowledge is treated as reconstructed or accepted automatically."
  - "No repo scan, AI summary, contract generation, gate decision, proof, or Writer behavior is activated."
knowledge_refs: []
contract_refs: []
report_refs: []
decision_refs: []
proof_refs: []
latest_proof_ref: null
research_gate:
  classification: R1
  required: false
  rationale: "Brownfield baseline preparation records advisory reconstruction workspace boundaries without external research or repo analysis."
  research_refs: []
  external_research_refs: []
  blocked_reason: null
doc_impact:
  classification: project-memory
  required_updates:
    - ".punk/memory/STATUS.md"
    - ".punk/memory/reconstruction/**"
    - ".punk/memory/reports/**"
  rationale: "Brownfield reconstruction preparation changes the manual Level 0 project-memory baseline."
---

## Context

The project has been initialized as a Punk brownfield project with Level 0 advisory reconstruction workspace.

Project id: `{project_id}`

## Intent

Prepare a reviewed source-linked reconstruction baseline before any brownfield claims are promoted.

## Non-scope

Do not scan the repository.

Do not generate summaries, contracts, specs, claims, gate decisions, proofs, or acceptance claims.

Do not write `.punk/` runtime stores.
"#,
        project_id = project_id.as_str()
    )
}

const REPORTS_README_TEMPLATE: &str = r#"# Work Reports

Manual Level 0 outcome, evidence, and handoff reports live here.

Use repo-relative artifact refs. Do not treat reports as final gate decisions.
"#;

const ADR_README_TEMPLATE: &str = r#"# Architecture Decisions

Accepted architecture decisions live here.

Use ADRs for durable decisions that should outlive a single work report.
"#;

const RESEARCH_README_TEMPLATE: &str = r#"# Research Notes

Curated research notes live here.

Keep external source notes advisory until promoted through a goal, decision, contract, eval, or report.
"#;

const IDEAS_README_TEMPLATE: &str = r#"# Ideas

Raw or parked ideas live here.

Ideas are not implementation truth until promoted through the project workflow.
"#;

const INSTRUCTIONS_INDEX_TEMPLATE: &str = r#"# Punk Instructions

This is a thin local instruction index for this Punk project.

Use it to find the focused instruction page you need. Do not copy every rule
into this file.

## Start here

- [Getting started](pages/getting-started.md)
- [Project layout](pages/layout.md)
- [Init behavior](pages/init.md)
- [Modules](pages/modules.md)
- [Authority and generated views](pages/authority.md)

## Module instructions

Module-specific instruction trees live under `modules/<module-id>/` when a
module is explicitly added later.

No module is active just because this directory exists.

## Page index view

A future derived page index may live at:

```text
.punk/views/instructions/page-index.json
```

That view is rebuildable and advisory. The source instruction pages remain the
thing to inspect.
"#;

const INSTRUCTIONS_GETTING_STARTED_TEMPLATE: &str = r#"# Getting Started

This project has been initialized with Punk.

Start with:

1. Read `.punk/README.md`.
2. Read `.punk/project.toml`.
3. Read `.punk/memory/STATUS.md`.
4. Follow the selected next goal in `.punk/memory/goals/`.

Keep changes bounded and record evidence in project memory.
"#;

const INSTRUCTIONS_LAYOUT_TEMPLATE: &str = r#"# Project Layout

Current local Punk layout:

- `.punk/README.md` - thin project entrypoint.
- `.punk/project.toml` - setup metadata, not runtime authority.
- `.punk/instructions/` - local human and agent instructions.
- `.punk/memory/` - tracked durable Level 0 project memory.

Runtime and derived stores are not created by init unless an explicit later
slice activates them.

Generated views, if present later, are rebuildable views over source artifacts.
"#;

const INSTRUCTIONS_INIT_TEMPLATE: &str = r#"# Init

`punk init <project-id>` initializes the current directory in place.

It does not create a new subdirectory named `<project-id>`.

Default greenfield init writes compact Level 0 project memory and thin
instruction entrypoints.

`punk init <project-id> --mode brownfield` writes an advisory brownfield entry
scaffold. It does not scan the repository, reconstruct project truth, generate
contracts, write gate decisions, or create proof.
"#;

const INSTRUCTIONS_MODULES_TEMPLATE: &str = r#"# Modules

Modules add domain-specific guidance and assessments.

A module can add its own instruction subtree later:

```text
.punk/instructions/modules/<module-id>/INDEX.md
.punk/instructions/modules/<module-id>/pages/*.md
```

Modules may assess, collect receipts, and render inspect hints.

Modules may not write final decisions, bypass scope, create hidden truth
stores, or activate external side effects by documentation alone.
"#;

const INSTRUCTIONS_AUTHORITY_TEMPLATE: &str = r#"# Authority

Instruction pages help humans and agents navigate the project.

They do not replace contracts, receipts, gate decisions, proofpacks, or
canonical project memory.

Live work state is recorded in `.punk/memory/STATUS.md`.

Only future `gate` behavior may write final decisions.

Derived instruction indexes and HTML views are advisory and rebuildable.
"#;

const INSTRUCTION_MODULES_README_TEMPLATE: &str = r#"# Module Instructions

Module instruction trees live here after a module is explicitly added.

No module is active just because this directory exists.

Each module subtree should stay small:

- `INDEX.md` for the module table of contents.
- `pages/` for focused instructions.
- no hidden truth store.
- no external side effects by documentation alone.
"#;

const BROWNFIELD_RECONSTRUCTION_README_TEMPLATE: &str = r#"# Brownfield Reconstruction Workspace

This directory is the advisory workspace for future brownfield reconstruction artifacts.

No project knowledge has been reconstructed yet.

Existing code, docs, tests, history, and issues are not Punk truth.

Future reconstruction artifacts are advisory candidates until reviewed.

No repo scan, AI summary, contract, gate decision, proof, or acceptance claim has been created by init.
"#;

const SOURCE_CORPUS_MANIFEST_TEMPLATE: &str = r#"# Source Corpus Manifest

Purpose: Future source-linked inventory of files, docs, tests, CI, schemas, and history sources.

Status: not_started.

No generated inventory exists yet.

Do not infer project intent from this placeholder.
"#;

const CLAIM_LEDGER_TEMPLATE: &str = r#"# Claim Ledger

Purpose: Future ledger of source-linked reconstructed claims.

Status: not_started.

No claims are accepted automatically.

No claims have been reconstructed or reviewed yet.
"#;

const UNKNOWNS_TEMPLATE: &str = r#"# Unknowns

Purpose: Future unknowns/blockers for safe contract drafting.

Status: not_started.

No unknowns have been collected yet.
"#;

const CONTRADICTIONS_TEMPLATE: &str = r#"# Contradictions

Purpose: Future stale/conflicting docs/code/history findings.

Status: not_started.

No contradictions have been collected yet.
"#;

const CONTRACT_READINESS_TEMPLATE: &str = r#"# Contract Readiness

Purpose: Future Contract Readiness Baseline.

Status: not_started.

This is not a contract.
This is not a gate decision.
This is not proof.

No contract-readiness claims have been reconstructed or reviewed yet.
"#;

fn punk_readme_template(entry_mode: ProjectInitEntryMode) -> String {
    match entry_mode {
        ProjectInitEntryMode::Greenfield => r#"# .punk

This directory is the Punk project root marker.

Current active behavior is Dogfooding Level 0 compact manual project memory.

Tracked durable project memory lives under `.punk/memory/`.

Local instructions start at `.punk/instructions/INDEX.md`.

Authoritative live work state for this project is `.punk/memory/STATUS.md`.

Runtime and derived stores such as runtime, cache, events, contracts, runs, evals, decisions, proofs, indexes, and views are not active yet.
"#
        .to_owned(),
        ProjectInitEntryMode::Brownfield => r#"# .punk

This directory is the Punk project root marker.

Current active behavior is Dogfooding Level 0 brownfield entry scaffold.

Tracked durable project memory lives under `.punk/memory/`.

Local instructions start at `.punk/instructions/INDEX.md`.

Authoritative live work state for this project is `.punk/memory/STATUS.md`.

Brownfield reconstruction has not started.

The reconstruction workspace under `.punk/memory/reconstruction/` is advisory until reviewed.

Runtime and derived stores such as runtime, cache, events, contracts, runs, evals, decisions, proofs, indexes, and views are not active yet.
"#
        .to_owned(),
    }
}

fn punk_project_toml_template(project_id: &ProjectId, entry_mode: ProjectInitEntryMode) -> String {
    let brownfield_section = match entry_mode {
        ProjectInitEntryMode::Greenfield => "",
        ProjectInitEntryMode::Brownfield => {
            r#"
[brownfield]
reconstruction_status = "not_started"
authority = "advisory_candidates_only"
"#
        }
    };

    format!(
        r#"# Punk project marker.
# This file is setup metadata, not runtime authority.

schema_version = "punk.project.v0.1"
project_id = "{project_id}"
entry_mode = "{entry_mode}"
dogfooding_level = 0
runtime_persistence = "inactive"
live_work_state = ".punk/memory/STATUS.md"

[memory]
layout = "compact"
root = ".punk/memory"

[instructions]
root = ".punk/instructions"
index = ".punk/instructions/INDEX.md"
page_index_view = ".punk/views/instructions/page-index.json"
views_active = false

[runtime]
active = false
root = ".punk/runtime"

[authority]
live_state = ".punk/memory/STATUS.md"
final_decisions = "not_active"
proofpacks = "not_active"
{brownfield_section}
"#,
        project_id = project_id.as_str(),
        entry_mode = entry_mode.as_str(),
        brownfield_section = brownfield_section
    )
}

const PROJECT_INIT_BOUNDARY_NOTES: &[&str] = &[
    "writes only greenfield Level 0 compact project-memory scaffold files under .punk/memory",
    "records project_id and entry_mode = greenfield in the scaffold",
    "creates .punk as a project root marker and .punk/memory as tracked durable memory",
    "creates thin local instruction entrypoints under .punk/instructions",
    "does not create root-level work, knowledge, docs/adr, or publishing directories",
    "does not implement brownfield reconstruction or grayfield reconciliation",
    "does not create contracts, run receipts, gate artifacts, proofpacks, or acceptance claims",
    "does not perform repo scanning, AI summaries, generated truth, or network behavior",
    "uses create-new/no-overwrite behavior and reports conflicts fail-closed",
];

const PROJECT_INIT_DEFERRED_NOTES: &[&str] = &[
    "brownfield reconstruction and grayfield reconciliation remain deferred",
    ".punk/runtime project storage remains inactive",
    "flow persistence and event writing remain inactive",
    "contract writer, receipt writer, gate writer, proof writer, and proofpack writer remain inactive",
    "project-specific source/design refs must be filled manually after init",
];

const BROWNFIELD_PROJECT_INIT_BOUNDARY_NOTES: &[&str] = &[
    "writes only a brownfield Level 0 advisory reconstruction scaffold under .punk/memory",
    "records project_id and entry_mode = brownfield in the scaffold",
    "records reconstruction_status = not_started and authority = advisory_candidates_only",
    "creates .punk as a project root marker and .punk/memory as tracked durable memory",
    "creates thin local instruction entrypoints under .punk/instructions",
    "creates empty reconstruction placeholders for source corpus, claims, unknowns, contradictions, and contract readiness",
    "does not create root-level work, knowledge, docs/adr, or publishing directories",
    "does not scan the repository or infer project knowledge",
    "does not create contracts, specs, gate artifacts, proofpacks, or acceptance claims",
    "uses create-new/no-overwrite behavior and reports conflicts fail-closed",
];

const BROWNFIELD_PROJECT_INIT_DEFERRED_NOTES: &[&str] = &[
    "brownfield reconstruction remains not_started",
    "grayfield reconciliation remains deferred",
    ".punk/runtime project storage remains inactive",
    "flow persistence and event writing remain inactive",
    "contract writer, receipt writer, gate writer, proof writer, and proofpack writer remain inactive",
    "all future reconstructed claims remain advisory candidates until reviewed",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstructionPageIndexNode {
    id: String,
    title: String,
    kind: String,
    status: String,
    authority: String,
    source_ref: String,
    module_id: String,
    children: Vec<String>,
}

impl InstructionPageIndexNode {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        kind: impl Into<String>,
        status: impl Into<String>,
        authority: impl Into<String>,
        source_ref: impl Into<String>,
        module_id: impl Into<String>,
        children: &[&str],
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            kind: kind.into(),
            status: status.into(),
            authority: authority.into(),
            source_ref: source_ref.into(),
            module_id: module_id.into(),
            children: children.iter().map(|child| (*child).to_owned()).collect(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn authority(&self) -> &str {
        &self.authority
    }

    pub fn source_ref(&self) -> &str {
        &self.source_ref
    }

    pub fn module_id(&self) -> &str {
        &self.module_id
    }

    pub fn children(&self) -> &[String] {
        &self.children
    }
}

pub fn default_instruction_page_index_nodes() -> Vec<InstructionPageIndexNode> {
    vec![
        InstructionPageIndexNode::new(
            "punk.instructions",
            "Punk Instructions",
            "instruction-index",
            "active",
            "advisory",
            INSTRUCTIONS_INDEX_PATH,
            "core",
            &[
                "punk.instructions.getting-started",
                "punk.instructions.layout",
                "punk.instructions.init",
                "punk.instructions.modules",
                "punk.instructions.authority",
            ],
        ),
        InstructionPageIndexNode::new(
            "punk.instructions.getting-started",
            "Getting Started",
            "instruction-page",
            "active",
            "advisory",
            ".punk/instructions/pages/getting-started.md",
            "core",
            &[],
        ),
        InstructionPageIndexNode::new(
            "punk.instructions.layout",
            "Project Layout",
            "instruction-page",
            "active",
            "advisory",
            ".punk/instructions/pages/layout.md",
            "core",
            &[],
        ),
        InstructionPageIndexNode::new(
            "punk.instructions.init",
            "Init",
            "instruction-page",
            "active",
            "advisory",
            ".punk/instructions/pages/init.md",
            "core",
            &[],
        ),
        InstructionPageIndexNode::new(
            "punk.instructions.modules",
            "Modules",
            "instruction-page",
            "active",
            "advisory",
            ".punk/instructions/pages/modules.md",
            "core",
            &[],
        ),
        InstructionPageIndexNode::new(
            "punk.instructions.authority",
            "Authority",
            "instruction-page",
            "active",
            "advisory",
            ".punk/instructions/pages/authority.md",
            "core",
            &[],
        ),
        InstructionPageIndexNode::new(
            "punk.instructions.modules.index",
            "Module Instructions",
            "module-instruction-root",
            "parked",
            "advisory",
            ".punk/instructions/modules/README.md",
            "module-host",
            &[],
        ),
    ]
}

pub fn render_instruction_page_index_json(nodes: &[InstructionPageIndexNode]) -> String {
    let mut output = String::new();
    output.push_str("{\n");
    instruction_page_index_write_json_field(
        &mut output,
        1,
        "schema_version",
        INSTRUCTION_PAGE_INDEX_SCHEMA_VERSION,
        true,
    );
    instruction_page_index_write_json_field(&mut output, 1, "authority", "advisory", true);
    instruction_page_index_write_json_field(&mut output, 1, "source_root", INSTRUCTIONS_ROOT, true);
    instruction_page_index_write_json_field(
        &mut output,
        1,
        "view_ref",
        INSTRUCTION_PAGE_INDEX_VIEW_PATH,
        true,
    );
    instruction_page_index_write_json_field(
        &mut output,
        1,
        "generated_from",
        "source_instruction_pages",
        true,
    );
    output.push_str("  \"nodes\": [\n");
    for (index, node) in nodes.iter().enumerate() {
        output.push_str("    {\n");
        instruction_page_index_write_json_field(&mut output, 3, "id", node.id(), true);
        instruction_page_index_write_json_field(&mut output, 3, "title", node.title(), true);
        instruction_page_index_write_json_field(&mut output, 3, "kind", node.kind(), true);
        instruction_page_index_write_json_field(&mut output, 3, "status", node.status(), true);
        instruction_page_index_write_json_field(
            &mut output,
            3,
            "authority",
            node.authority(),
            true,
        );
        instruction_page_index_write_json_field(
            &mut output,
            3,
            "source_ref",
            node.source_ref(),
            true,
        );
        instruction_page_index_write_json_field(
            &mut output,
            3,
            "module_id",
            node.module_id(),
            true,
        );
        output.push_str("      \"children\": [");
        for (child_index, child) in node.children().iter().enumerate() {
            if child_index > 0 {
                output.push_str(", ");
            }
            instruction_page_index_push_json_string(&mut output, child);
        }
        output.push_str("]\n");
        output.push_str("    }");
        if index + 1 != nodes.len() {
            output.push(',');
        }
        output.push('\n');
    }
    output.push_str("  ]\n");
    output.push_str("}\n");
    output
}

fn instruction_page_index_write_json_field(
    output: &mut String,
    indent: usize,
    key: &str,
    value: &str,
    trailing_comma: bool,
) {
    output.push_str(&"  ".repeat(indent));
    instruction_page_index_push_json_string(output, key);
    output.push_str(": ");
    instruction_page_index_push_json_string(output, value);
    if trailing_comma {
        output.push(',');
    }
    output.push('\n');
}

fn instruction_page_index_push_json_string(output: &mut String, value: &str) {
    output.push('"');
    for value_char in value.chars() {
        match value_char {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\u{08}' => output.push_str("\\b"),
            '\u{0C}' => output.push_str("\\f"),
            value_char if value_char.is_control() => {
                write!(output, "\\u{:04x}", value_char as u32)
                    .expect("writing to String should succeed");
            }
            value_char => output.push(value_char),
        }
    }
    output.push('"');
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectId(String);

impl ProjectId {
    pub fn parse(value: impl Into<String>) -> Result<Self, ProjectIdError> {
        let value = value.into();
        validate_project_id(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectIdError {
    Empty,
    InvalidFormat,
}

impl ProjectIdError {
    pub fn message(self) -> &'static str {
        match self {
            Self::Empty => "project id must not be empty",
            Self::InvalidFormat => PROJECT_ID_FORMAT_NOTE,
        }
    }
}

fn validate_project_id(value: &str) -> Result<(), ProjectIdError> {
    if value.is_empty() {
        return Err(ProjectIdError::Empty);
    }

    let mut previous_was_hyphen = false;
    for (index, character) in value.chars().enumerate() {
        let valid =
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-';
        if !valid {
            return Err(ProjectIdError::InvalidFormat);
        }
        if character == '-' {
            if index == 0 || previous_was_hyphen {
                return Err(ProjectIdError::InvalidFormat);
            }
            previous_was_hyphen = true;
        } else {
            previous_was_hyphen = false;
        }
    }

    if previous_was_hyphen {
        return Err(ProjectIdError::InvalidFormat);
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInitEntryMode {
    Greenfield,
    Brownfield,
}

impl ProjectInitEntryMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Greenfield => PROJECT_INIT_ENTRY_MODE,
            Self::Brownfield => PROJECT_INIT_BROWNFIELD_ENTRY_MODE,
        }
    }

    fn schema_version(self) -> &'static str {
        match self {
            Self::Greenfield => PROJECT_INIT_SCHEMA_VERSION,
            Self::Brownfield => PROJECT_INIT_BROWNFIELD_SCHEMA_VERSION,
        }
    }

    fn entries(self) -> &'static [ProjectInitEntry] {
        match self {
            Self::Greenfield => GREENFIELD_PROJECT_INIT_ENTRIES,
            Self::Brownfield => BROWNFIELD_PROJECT_INIT_ENTRIES,
        }
    }

    fn boundary_notes(self) -> &'static [&'static str] {
        match self {
            Self::Greenfield => PROJECT_INIT_BOUNDARY_NOTES,
            Self::Brownfield => BROWNFIELD_PROJECT_INIT_BOUNDARY_NOTES,
        }
    }

    fn deferred_notes(self) -> &'static [&'static str] {
        match self {
            Self::Greenfield => PROJECT_INIT_DEFERRED_NOTES,
            Self::Brownfield => BROWNFIELD_PROJECT_INIT_DEFERRED_NOTES,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInitArtifactKind {
    Directory,
    File,
}

impl ProjectInitArtifactKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Directory => "directory",
            Self::File => "file",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProjectInitArtifactStatus {
    Planned,
    Created,
    AlreadyExists,
    Conflict,
    Failed,
}

impl ProjectInitArtifactStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Created => "created",
            Self::AlreadyExists => "already_exists",
            Self::Conflict => "conflict",
            Self::Failed => "failed",
        }
    }

    pub fn is_blocking(self) -> bool {
        matches!(self, Self::Conflict | Self::Failed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInitArtifactReport {
    repo_relative_path: String,
    kind: ProjectInitArtifactKind,
    status: ProjectInitArtifactStatus,
    detail: Option<String>,
}

impl ProjectInitArtifactReport {
    fn new(
        repo_relative_path: impl Into<String>,
        kind: ProjectInitArtifactKind,
        status: ProjectInitArtifactStatus,
        detail: Option<String>,
    ) -> Self {
        Self {
            repo_relative_path: repo_relative_path.into(),
            kind,
            status,
            detail,
        }
    }

    pub fn repo_relative_path(&self) -> &str {
        &self.repo_relative_path
    }

    pub fn kind(&self) -> ProjectInitArtifactKind {
        self.kind
    }

    pub fn status(&self) -> ProjectInitArtifactStatus {
        self.status
    }

    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    pub fn is_blocking(&self) -> bool {
        self.status.is_blocking()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectInitReport {
    project_root: PathBuf,
    project_id: ProjectId,
    entry_mode: ProjectInitEntryMode,
    artifacts: Vec<ProjectInitArtifactReport>,
}

impl ProjectInitReport {
    pub fn project_root(&self) -> &Path {
        &self.project_root
    }

    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    pub fn entry_mode(&self) -> &'static str {
        self.entry_mode.as_str()
    }

    pub fn schema_version(&self) -> &'static str {
        self.entry_mode.schema_version()
    }

    pub fn artifacts(&self) -> &[ProjectInitArtifactReport] {
        &self.artifacts
    }

    pub fn boundary_notes(&self) -> &'static [&'static str] {
        self.entry_mode.boundary_notes()
    }

    pub fn deferred_notes(&self) -> &'static [&'static str] {
        self.entry_mode.deferred_notes()
    }

    pub fn blocked(&self) -> bool {
        self.artifacts
            .iter()
            .any(ProjectInitArtifactReport::is_blocking)
    }

    pub fn exit_code(&self) -> u8 {
        if self.blocked() {
            1
        } else {
            0
        }
    }

    pub fn result_label(&self) -> &'static str {
        if self.blocked() {
            "blocked"
        } else if self
            .artifacts
            .iter()
            .all(|artifact| artifact.status() == ProjectInitArtifactStatus::AlreadyExists)
        {
            "already_initialized"
        } else {
            "initialized"
        }
    }

    pub fn render_human(&self) -> String {
        let mut output = String::new();
        writeln!(&mut output, "punk init").expect("writing to String should succeed");
        writeln!(&mut output, "schema_version: {}", self.schema_version())
            .expect("writing to String should succeed");
        writeln!(&mut output, "mode: {PROJECT_INIT_MODE}")
            .expect("writing to String should succeed");
        writeln!(&mut output, "entry_mode: {}", self.entry_mode())
            .expect("writing to String should succeed");
        writeln!(&mut output, "project_id: {}", self.project_id().as_str())
            .expect("writing to String should succeed");
        writeln!(
            &mut output,
            "runtime_persistence: {PROJECT_INIT_RUNTIME_PERSISTENCE}"
        )
        .expect("writing to String should succeed");
        writeln!(&mut output, "target_root: .").expect("writing to String should succeed");
        writeln!(&mut output, "result: {}", self.result_label())
            .expect("writing to String should succeed");
        writeln!(&mut output, "artifacts:").expect("writing to String should succeed");

        for artifact in self.artifacts() {
            writeln!(&mut output, "  - path: {}", artifact.repo_relative_path())
                .expect("writing to String should succeed");
            writeln!(&mut output, "    kind: {}", artifact.kind().as_str())
                .expect("writing to String should succeed");
            writeln!(&mut output, "    status: {}", artifact.status().as_str())
                .expect("writing to String should succeed");
            if let Some(detail) = artifact.detail() {
                writeln!(&mut output, "    detail: {detail}")
                    .expect("writing to String should succeed");
            }
        }

        writeln!(&mut output, "notes:").expect("writing to String should succeed");
        for note in self.boundary_notes() {
            writeln!(&mut output, "  - {note}").expect("writing to String should succeed");
        }

        writeln!(&mut output, "deferred:").expect("writing to String should succeed");
        for note in self.deferred_notes() {
            writeln!(&mut output, "  - {note}").expect("writing to String should succeed");
        }

        output.trim_end().to_owned()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifestId(String);

impl SourceCorpusManifestId {
    pub fn parse(value: impl Into<String>) -> Result<Self, SourceCorpusManifestError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SourceCorpusManifestError::EmptyManifestId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusItemId(String);

impl SourceCorpusItemId {
    pub fn parse(value: impl Into<String>) -> Result<Self, SourceCorpusManifestError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SourceCorpusManifestError::EmptyItemId);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusEvidenceRef(String);

impl SourceCorpusEvidenceRef {
    pub fn parse(value: impl Into<String>) -> Result<Self, SourceCorpusManifestError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SourceCorpusManifestError::EmptyEvidenceRef);
        }
        Ok(Self(value))
    }

    pub fn for_manifest_item(
        manifest_id: &SourceCorpusManifestId,
        item_id: &SourceCorpusItemId,
    ) -> Result<Self, SourceCorpusManifestError> {
        Self::parse(format!(
            "{manifest_id}#{item_id}",
            manifest_id = manifest_id.as_str(),
            item_id = item_id.as_str()
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusRepoRelativePath(String);

impl SourceCorpusRepoRelativePath {
    pub fn parse(value: impl Into<String>) -> Result<Self, SourceCorpusPathError> {
        let value = value.into();
        validate_source_corpus_repo_relative_path(&value)?;
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusPathError {
    Empty,
    CurrentDirectory,
    Absolute,
    HomePath,
    UrlLike,
    ParentTraversal,
    CurrentSegment,
    EmptySegment,
    Backslash,
}

impl SourceCorpusPathError {
    pub fn message(self) -> &'static str {
        match self {
            Self::Empty => "source corpus path must not be empty",
            Self::CurrentDirectory => "source corpus item path must not be .",
            Self::Absolute => "source corpus path must be repo-relative, not absolute",
            Self::HomePath => "source corpus path must not use home/user path syntax",
            Self::UrlLike => "source corpus path must not be URL-like",
            Self::ParentTraversal => "source corpus path must not contain parent traversal",
            Self::CurrentSegment => "source corpus path must not contain . segments",
            Self::EmptySegment => "source corpus path must not contain empty segments",
            Self::Backslash => "source corpus path must use forward slash separators",
        }
    }
}

fn validate_source_corpus_repo_relative_path(value: &str) -> Result<(), SourceCorpusPathError> {
    if value.is_empty() {
        return Err(SourceCorpusPathError::Empty);
    }
    if value == "." {
        return Err(SourceCorpusPathError::CurrentDirectory);
    }
    if value.starts_with('/') || is_windows_drive_rooted(value) {
        return Err(SourceCorpusPathError::Absolute);
    }
    if value.starts_with('~') {
        return Err(SourceCorpusPathError::HomePath);
    }
    if value.contains("://") {
        return Err(SourceCorpusPathError::UrlLike);
    }
    if value.contains('\\') {
        return Err(SourceCorpusPathError::Backslash);
    }

    for segment in value.split('/') {
        if segment.is_empty() {
            return Err(SourceCorpusPathError::EmptySegment);
        }
        if segment == "." {
            return Err(SourceCorpusPathError::CurrentSegment);
        }
        if segment == ".." {
            return Err(SourceCorpusPathError::ParentTraversal);
        }
    }

    Ok(())
}

fn is_windows_drive_rooted(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'/' || bytes[2] == b'\\')
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestStatus {
    Advisory,
}

impl SourceCorpusManifestStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestAuthority {
    ObservedStructure,
}

impl SourceCorpusManifestAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ObservedStructure => "observed_structure",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusObservedKind {
    File,
    Directory,
    SymlinkCandidate,
    Unknown,
}

impl SourceCorpusObservedKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::File => "file",
            Self::Directory => "directory",
            Self::SymlinkCandidate => "symlink_candidate",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusSourceClass {
    SourceCode,
    Docs,
    Tests,
    CiConfig,
    PackageManifest,
    Schema,
    Migration,
    Script,
    GeneratedCandidate,
    VendoredCandidate,
    Unknown,
}

impl SourceCorpusSourceClass {
    pub const ALL: [Self; 11] = [
        Self::SourceCode,
        Self::Docs,
        Self::Tests,
        Self::CiConfig,
        Self::PackageManifest,
        Self::Schema,
        Self::Migration,
        Self::Script,
        Self::GeneratedCandidate,
        Self::VendoredCandidate,
        Self::Unknown,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceCode => "source_code",
            Self::Docs => "docs",
            Self::Tests => "tests",
            Self::CiConfig => "ci_config",
            Self::PackageManifest => "package_manifest",
            Self::Schema => "schema",
            Self::Migration => "migration",
            Self::Script => "script",
            Self::GeneratedCandidate => "generated_candidate",
            Self::VendoredCandidate => "vendored_candidate",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusSourceMarkerKind {
    Extension,
    Basename,
    ManifestType,
    DocMarker,
    TestMarker,
    ConfigMarker,
    SchemaMarker,
    MigrationMarker,
    ScriptMarker,
}

impl SourceCorpusSourceMarkerKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::Basename => "basename",
            Self::ManifestType => "manifest_type",
            Self::DocMarker => "doc_marker",
            Self::TestMarker => "test_marker",
            Self::ConfigMarker => "config_marker",
            Self::SchemaMarker => "schema_marker",
            Self::MigrationMarker => "migration_marker",
            Self::ScriptMarker => "script_marker",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusSourceMarker {
    kind: SourceCorpusSourceMarkerKind,
    value: String,
}

impl SourceCorpusSourceMarker {
    pub fn new(
        kind: SourceCorpusSourceMarkerKind,
        value: impl Into<String>,
    ) -> Result<Self, SourceCorpusManifestError> {
        let value = value.into();
        if value.trim().is_empty() {
            return Err(SourceCorpusManifestError::EmptySourceMarker);
        }
        Ok(Self { kind, value })
    }

    pub fn kind(&self) -> SourceCorpusSourceMarkerKind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusTrackingStatus {
    Observed,
    Excluded,
    SensitiveRedacted,
    Unknown,
}

impl SourceCorpusTrackingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Observed => "observed",
            Self::Excluded => "excluded",
            Self::SensitiveRedacted => "sensitive_redacted",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusSensitivityClass {
    Normal,
    Caution,
    Sensitive,
    Excluded,
    Unknown,
}

impl SourceCorpusSensitivityClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Caution => "caution",
            Self::Sensitive => "sensitive",
            Self::Excluded => "excluded",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusGeneratedOrVendoredCandidate {
    None,
    GeneratedCandidate,
    VendoredCandidate,
    Unknown,
}

impl SourceCorpusGeneratedOrVendoredCandidate {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::GeneratedCandidate => "generated_candidate",
            Self::VendoredCandidate => "vendored_candidate",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCorpusContentPolicy {
    read_contents: bool,
    store_snippets: bool,
    summarize_contents: bool,
}

impl SourceCorpusContentPolicy {
    pub fn no_content() -> Self {
        Self {
            read_contents: false,
            store_snippets: false,
            summarize_contents: false,
        }
    }

    pub fn reads_contents(self) -> bool {
        self.read_contents
    }

    pub fn stores_snippets(self) -> bool {
        self.store_snippets
    }

    pub fn summarizes_contents(self) -> bool {
        self.summarize_contents
    }
}

impl Default for SourceCorpusContentPolicy {
    fn default() -> Self {
        Self::no_content()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusHashPolicy {
    Deferred,
}

impl SourceCorpusHashPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
        }
    }

    pub fn requires_filesystem_hashing(self) -> bool {
        false
    }
}

impl Default for SourceCorpusHashPolicy {
    fn default() -> Self {
        Self::Deferred
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusSizePolicy {
    Deferred,
}

impl SourceCorpusSizePolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "deferred",
        }
    }

    pub fn requires_filesystem_metadata(self) -> bool {
        false
    }
}

impl Default for SourceCorpusSizePolicy {
    fn default() -> Self {
        Self::Deferred
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusCautionClass {
    Secrets,
    EnvFiles,
    LocalCache,
    BuildOutput,
    PrivateAgentTranscripts,
    GeneratedCandidate,
    VendoredCandidate,
    Unknown,
}

impl SourceCorpusCautionClass {
    pub const ALL: [Self; 8] = [
        Self::Secrets,
        Self::EnvFiles,
        Self::LocalCache,
        Self::BuildOutput,
        Self::PrivateAgentTranscripts,
        Self::GeneratedCandidate,
        Self::VendoredCandidate,
        Self::Unknown,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Secrets => "secrets",
            Self::EnvFiles => "env_files",
            Self::LocalCache => "local_cache",
            Self::BuildOutput => "build_output",
            Self::PrivateAgentTranscripts => "private_agent_transcripts",
            Self::GeneratedCandidate => "generated_candidate",
            Self::VendoredCandidate => "vendored_candidate",
            Self::Unknown => "unknown",
        }
    }
}

pub const SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS: &[&str] = &[
    ".git",
    ".punk/runtime",
    ".punk/cache",
    ".punk/indexes",
    "node_modules",
    "target",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCorpusExclusionPolicy {
    excluded_paths: &'static [&'static str],
    caution_classes: &'static [SourceCorpusCautionClass],
}

impl SourceCorpusExclusionPolicy {
    pub fn default_boundary() -> Self {
        Self {
            excluded_paths: SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS,
            caution_classes: &SourceCorpusCautionClass::ALL,
        }
    }

    pub fn excluded_paths(&self) -> &'static [&'static str] {
        self.excluded_paths
    }

    pub fn caution_classes(&self) -> &'static [SourceCorpusCautionClass] {
        self.caution_classes
    }
}

impl Default for SourceCorpusExclusionPolicy {
    fn default() -> Self {
        Self::default_boundary()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusSourceRootKind {
    RepoRoot,
}

impl SourceCorpusSourceRootKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RepoRoot => "repo_root",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusSourceRootRef {
    kind: SourceCorpusSourceRootKind,
    path: &'static str,
}

impl SourceCorpusSourceRootRef {
    pub fn repo_root() -> Self {
        Self {
            kind: SourceCorpusSourceRootKind::RepoRoot,
            path: ".",
        }
    }

    pub fn kind(&self) -> SourceCorpusSourceRootKind {
        self.kind
    }

    pub fn path(&self) -> &'static str {
        self.path
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCorpusManifestModelCapabilities {
    scans_repository: bool,
    walks_files: bool,
    reads_file_contents: bool,
    computes_file_hashes: bool,
    writes_manifest: bool,
    creates_claims: bool,
    infers_intent: bool,
    uses_network: bool,
    uses_remote_ai: bool,
}

impl SourceCorpusManifestModelCapabilities {
    pub fn side_effect_free() -> Self {
        Self {
            scans_repository: false,
            walks_files: false,
            reads_file_contents: false,
            computes_file_hashes: false,
            writes_manifest: false,
            creates_claims: false,
            infers_intent: false,
            uses_network: false,
            uses_remote_ai: false,
        }
    }

    pub fn scans_repository(self) -> bool {
        self.scans_repository
    }

    pub fn walks_files(self) -> bool {
        self.walks_files
    }

    pub fn reads_file_contents(self) -> bool {
        self.reads_file_contents
    }

    pub fn computes_file_hashes(self) -> bool {
        self.computes_file_hashes
    }

    pub fn writes_manifest(self) -> bool {
        self.writes_manifest
    }

    pub fn creates_claims(self) -> bool {
        self.creates_claims
    }

    pub fn infers_intent(self) -> bool {
        self.infers_intent
    }

    pub fn uses_network(self) -> bool {
        self.uses_network
    }

    pub fn uses_remote_ai(self) -> bool {
        self.uses_remote_ai
    }
}

impl Default for SourceCorpusManifestModelCapabilities {
    fn default() -> Self {
        Self::side_effect_free()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusItem {
    item_id: SourceCorpusItemId,
    repo_relative_path: SourceCorpusRepoRelativePath,
    observed_kind: SourceCorpusObservedKind,
    source_class: SourceCorpusSourceClass,
    source_markers: Vec<SourceCorpusSourceMarker>,
    tracking_status: SourceCorpusTrackingStatus,
    sensitivity: SourceCorpusSensitivityClass,
    generated_or_vendored_candidate: SourceCorpusGeneratedOrVendoredCandidate,
    size_policy: SourceCorpusSizePolicy,
    hash_policy: SourceCorpusHashPolicy,
    content_policy: SourceCorpusContentPolicy,
    evidence_ref: SourceCorpusEvidenceRef,
    notes: Vec<String>,
}

impl SourceCorpusItem {
    pub fn new(
        manifest_id: &SourceCorpusManifestId,
        item_id: SourceCorpusItemId,
        repo_relative_path: SourceCorpusRepoRelativePath,
        observed_kind: SourceCorpusObservedKind,
        source_class: SourceCorpusSourceClass,
    ) -> Result<Self, SourceCorpusManifestError> {
        let evidence_ref = SourceCorpusEvidenceRef::for_manifest_item(manifest_id, &item_id)?;
        Ok(Self {
            item_id,
            repo_relative_path,
            observed_kind,
            source_class,
            source_markers: Vec::new(),
            tracking_status: SourceCorpusTrackingStatus::Observed,
            sensitivity: SourceCorpusSensitivityClass::Normal,
            generated_or_vendored_candidate: SourceCorpusGeneratedOrVendoredCandidate::None,
            size_policy: SourceCorpusSizePolicy::default(),
            hash_policy: SourceCorpusHashPolicy::default(),
            content_policy: SourceCorpusContentPolicy::default(),
            evidence_ref,
            notes: Vec::new(),
        })
    }

    pub fn with_source_marker(mut self, marker: SourceCorpusSourceMarker) -> Self {
        self.source_markers.push(marker);
        self
    }

    pub fn with_sensitivity(mut self, sensitivity: SourceCorpusSensitivityClass) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    pub fn with_generated_or_vendored_candidate(
        mut self,
        candidate: SourceCorpusGeneratedOrVendoredCandidate,
    ) -> Self {
        self.generated_or_vendored_candidate = candidate;
        self
    }

    pub fn item_id(&self) -> &SourceCorpusItemId {
        &self.item_id
    }

    pub fn repo_relative_path(&self) -> &SourceCorpusRepoRelativePath {
        &self.repo_relative_path
    }

    pub fn observed_kind(&self) -> SourceCorpusObservedKind {
        self.observed_kind
    }

    pub fn source_class(&self) -> SourceCorpusSourceClass {
        self.source_class
    }

    pub fn source_markers(&self) -> &[SourceCorpusSourceMarker] {
        &self.source_markers
    }

    pub fn tracking_status(&self) -> SourceCorpusTrackingStatus {
        self.tracking_status
    }

    pub fn sensitivity(&self) -> SourceCorpusSensitivityClass {
        self.sensitivity
    }

    pub fn generated_or_vendored_candidate(&self) -> SourceCorpusGeneratedOrVendoredCandidate {
        self.generated_or_vendored_candidate
    }

    pub fn size_policy(&self) -> SourceCorpusSizePolicy {
        self.size_policy
    }

    pub fn hash_policy(&self) -> SourceCorpusHashPolicy {
        self.hash_policy
    }

    pub fn content_policy(&self) -> SourceCorpusContentPolicy {
        self.content_policy
    }

    pub fn evidence_ref(&self) -> &SourceCorpusEvidenceRef {
        &self.evidence_ref
    }

    pub fn notes(&self) -> &[String] {
        &self.notes
    }

    pub fn has_claim_authority(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifest {
    manifest_id: SourceCorpusManifestId,
    project_id: ProjectId,
    entry_mode: ProjectInitEntryMode,
    status: SourceCorpusManifestStatus,
    authority: SourceCorpusManifestAuthority,
    source_root_ref: SourceCorpusSourceRootRef,
    exclusion_policy: SourceCorpusExclusionPolicy,
    items: Vec<SourceCorpusItem>,
    capabilities: SourceCorpusManifestModelCapabilities,
}

impl SourceCorpusManifest {
    pub fn new(
        manifest_id: SourceCorpusManifestId,
        project_id: ProjectId,
        items: Vec<SourceCorpusItem>,
    ) -> Self {
        Self {
            manifest_id,
            project_id,
            entry_mode: ProjectInitEntryMode::Brownfield,
            status: SourceCorpusManifestStatus::Advisory,
            authority: SourceCorpusManifestAuthority::ObservedStructure,
            source_root_ref: SourceCorpusSourceRootRef::repo_root(),
            exclusion_policy: SourceCorpusExclusionPolicy::default(),
            items,
            capabilities: SourceCorpusManifestModelCapabilities::default(),
        }
    }

    pub fn schema_version(&self) -> &'static str {
        SOURCE_CORPUS_MANIFEST_MODEL_SCHEMA_VERSION
    }

    pub fn manifest_schema_version(&self) -> &'static str {
        SOURCE_CORPUS_MANIFEST_SCHEMA_VERSION
    }

    pub fn manifest_id(&self) -> &SourceCorpusManifestId {
        &self.manifest_id
    }

    pub fn project_id(&self) -> &ProjectId {
        &self.project_id
    }

    pub fn entry_mode(&self) -> ProjectInitEntryMode {
        self.entry_mode
    }

    pub fn status(&self) -> SourceCorpusManifestStatus {
        self.status
    }

    pub fn authority(&self) -> SourceCorpusManifestAuthority {
        self.authority
    }

    pub fn source_root_ref(&self) -> &SourceCorpusSourceRootRef {
        &self.source_root_ref
    }

    pub fn exclusion_policy(&self) -> SourceCorpusExclusionPolicy {
        self.exclusion_policy
    }

    pub fn items(&self) -> &[SourceCorpusItem] {
        &self.items
    }

    pub fn capabilities(&self) -> SourceCorpusManifestModelCapabilities {
        self.capabilities
    }

    pub fn has_project_truth_authority(&self) -> bool {
        false
    }

    pub fn creates_claims(&self) -> bool {
        self.capabilities.creates_claims()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestError {
    EmptyManifestId,
    EmptyItemId,
    EmptyEvidenceRef,
    EmptySourceMarker,
}

impl SourceCorpusManifestError {
    pub fn message(self) -> &'static str {
        match self {
            Self::EmptyManifestId => "source corpus manifest id must not be empty",
            Self::EmptyItemId => "source corpus item id must not be empty",
            Self::EmptyEvidenceRef => "source corpus evidence ref must not be empty",
            Self::EmptySourceMarker => "source corpus source marker must not be empty",
        }
    }
}

pub const SOURCE_CORPUS_FORBIDDEN_CLAIM_FIELDS: &[&str] = &[
    "intent",
    "requirement",
    "module_purpose",
    "architecture_decision",
    "accepted_behavior",
    "invariant",
    "contract_ref_as_claim",
    "proof_ref_as_truth",
    "claims_created",
    "contract_readiness",
    "gate_decision",
    "proof_status",
    "risk",
    "severity",
];

pub fn source_corpus_manifest_claim_field_allowed(field_name: &str) -> bool {
    !SOURCE_CORPUS_FORBIDDEN_CLAIM_FIELDS
        .iter()
        .any(|forbidden| *forbidden == field_name)
}

pub const SOURCE_CORPUS_MANIFEST_WRITER_PREFLIGHT_MODEL_SCHEMA_VERSION: &str =
    "brownfield-source-corpus-manifest-writer-preflight-model.v0.1";
pub const SOURCE_CORPUS_MANIFEST_WRITER_FIRST_SLICE_SCHEMA_VERSION: &str =
    "brownfield-source-corpus-manifest-writer-first-slice.v0.1";
pub const SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH: &str =
    ".punk/memory/reconstruction/source-corpus-manifest.md";
pub const SOURCE_CORPUS_MANIFEST_WRITER_RECONSTRUCTION_DIR: &str = ".punk/memory/reconstruction";
pub const SOURCE_CORPUS_MANIFEST_WRITER_FORBIDDEN_RUNTIME_TARGET_PREFIXES: &[&str] = &[
    ".punk/runtime",
    ".punk/events",
    ".punk/runs",
    ".punk/decisions",
    ".punk/proofs",
    ".punk/cache",
    ".punk/indexes",
];

static SOURCE_CORPUS_MANIFEST_WRITER_TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterTarget(String);

impl SourceCorpusManifestWriterTarget {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn default_manifest_path() -> Self {
        Self::new(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterTargetPolicy {
    DefaultManifestPath,
    ConfiguredUnderReconstructionDir,
}

impl SourceCorpusManifestWriterTargetPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DefaultManifestPath => "default_manifest_path",
            Self::ConfiguredUnderReconstructionDir => "configured_under_reconstruction_dir",
        }
    }
}

impl Default for SourceCorpusManifestWriterTargetPolicy {
    fn default() -> Self {
        Self::DefaultManifestPath
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterConflictPolicy {
    MissingTarget,
    IdenticalExistingTarget,
    DifferentExistingTarget,
    Unknown,
}

impl SourceCorpusManifestWriterConflictPolicy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingTarget => "missing_target",
            Self::IdenticalExistingTarget => "identical_existing_target",
            Self::DifferentExistingTarget => "different_existing_target",
            Self::Unknown => "unknown",
        }
    }
}

impl Default for SourceCorpusManifestWriterConflictPolicy {
    fn default() -> Self {
        Self::MissingTarget
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterParentStatus {
    Valid,
    Missing,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterSymlinkAncestorStatus {
    Safe,
    Escape,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterManifestStatusEvidence {
    Advisory,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterManifestAuthorityEvidence {
    ObservedStructure,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterManifestInspection {
    status_evidence: SourceCorpusManifestWriterManifestStatusEvidence,
    authority_evidence: SourceCorpusManifestWriterManifestAuthorityEvidence,
    contains_absolute_path: bool,
    contains_content_snippet: bool,
    contains_summary: bool,
    claim_fields: Vec<String>,
}

impl SourceCorpusManifestWriterManifestInspection {
    pub fn for_manifest(manifest: &SourceCorpusManifest) -> Self {
        let status_evidence = match manifest.status() {
            SourceCorpusManifestStatus::Advisory => {
                SourceCorpusManifestWriterManifestStatusEvidence::Advisory
            }
        };
        let authority_evidence = match manifest.authority() {
            SourceCorpusManifestAuthority::ObservedStructure => {
                SourceCorpusManifestWriterManifestAuthorityEvidence::ObservedStructure
            }
        };
        Self {
            status_evidence,
            authority_evidence,
            contains_absolute_path: false,
            contains_content_snippet: false,
            contains_summary: false,
            claim_fields: Vec::new(),
        }
    }

    pub fn with_status_evidence(
        mut self,
        status_evidence: SourceCorpusManifestWriterManifestStatusEvidence,
    ) -> Self {
        self.status_evidence = status_evidence;
        self
    }

    pub fn with_authority_evidence(
        mut self,
        authority_evidence: SourceCorpusManifestWriterManifestAuthorityEvidence,
    ) -> Self {
        self.authority_evidence = authority_evidence;
        self
    }

    pub fn with_absolute_path(mut self) -> Self {
        self.contains_absolute_path = true;
        self
    }

    pub fn with_content_snippet(mut self) -> Self {
        self.contains_content_snippet = true;
        self
    }

    pub fn with_summary(mut self) -> Self {
        self.contains_summary = true;
        self
    }

    pub fn with_claim_field(mut self, field_name: impl Into<String>) -> Self {
        self.claim_fields.push(field_name.into());
        self
    }

    pub fn status_evidence(&self) -> SourceCorpusManifestWriterManifestStatusEvidence {
        self.status_evidence
    }

    pub fn authority_evidence(&self) -> SourceCorpusManifestWriterManifestAuthorityEvidence {
        self.authority_evidence
    }

    pub fn contains_absolute_path(&self) -> bool {
        self.contains_absolute_path
    }

    pub fn contains_content_snippet(&self) -> bool {
        self.contains_content_snippet
    }

    pub fn contains_summary(&self) -> bool {
        self.contains_summary
    }

    pub fn claim_fields(&self) -> &[String] {
        &self.claim_fields
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterPreflightInput {
    target: SourceCorpusManifestWriterTarget,
    target_policy: SourceCorpusManifestWriterTargetPolicy,
    conflict_policy: SourceCorpusManifestWriterConflictPolicy,
    parent_status: SourceCorpusManifestWriterParentStatus,
    symlink_ancestor_status: SourceCorpusManifestWriterSymlinkAncestorStatus,
    manifest: SourceCorpusManifest,
    manifest_inspection: SourceCorpusManifestWriterManifestInspection,
}

impl SourceCorpusManifestWriterPreflightInput {
    pub fn new(target: SourceCorpusManifestWriterTarget, manifest: SourceCorpusManifest) -> Self {
        let manifest_inspection =
            SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest);
        Self {
            target,
            target_policy: SourceCorpusManifestWriterTargetPolicy::default(),
            conflict_policy: SourceCorpusManifestWriterConflictPolicy::default(),
            parent_status: SourceCorpusManifestWriterParentStatus::Valid,
            symlink_ancestor_status: SourceCorpusManifestWriterSymlinkAncestorStatus::Safe,
            manifest,
            manifest_inspection,
        }
    }

    pub fn with_target_policy(
        mut self,
        target_policy: SourceCorpusManifestWriterTargetPolicy,
    ) -> Self {
        self.target_policy = target_policy;
        self
    }

    pub fn with_conflict_policy(
        mut self,
        conflict_policy: SourceCorpusManifestWriterConflictPolicy,
    ) -> Self {
        self.conflict_policy = conflict_policy;
        self
    }

    pub fn with_parent_status(
        mut self,
        parent_status: SourceCorpusManifestWriterParentStatus,
    ) -> Self {
        self.parent_status = parent_status;
        self
    }

    pub fn with_symlink_ancestor_status(
        mut self,
        symlink_ancestor_status: SourceCorpusManifestWriterSymlinkAncestorStatus,
    ) -> Self {
        self.symlink_ancestor_status = symlink_ancestor_status;
        self
    }

    pub fn with_manifest_inspection(
        mut self,
        manifest_inspection: SourceCorpusManifestWriterManifestInspection,
    ) -> Self {
        self.manifest_inspection = manifest_inspection;
        self
    }

    pub fn target(&self) -> &SourceCorpusManifestWriterTarget {
        &self.target
    }

    pub fn target_policy(&self) -> SourceCorpusManifestWriterTargetPolicy {
        self.target_policy
    }

    pub fn conflict_policy(&self) -> SourceCorpusManifestWriterConflictPolicy {
        self.conflict_policy
    }

    pub fn parent_status(&self) -> SourceCorpusManifestWriterParentStatus {
        self.parent_status
    }

    pub fn symlink_ancestor_status(&self) -> SourceCorpusManifestWriterSymlinkAncestorStatus {
        self.symlink_ancestor_status
    }

    pub fn manifest(&self) -> &SourceCorpusManifest {
        &self.manifest
    }

    pub fn manifest_inspection(&self) -> &SourceCorpusManifestWriterManifestInspection {
        &self.manifest_inspection
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterPreflightFinding {
    TargetAllowed,
    TargetNotUnderReconstructionDir,
    TargetAbsolutePath,
    TargetPathEscape,
    TargetSymlinkAncestorEscape,
    TargetSymlinkAncestorUnknown,
    TargetMissingParent,
    TargetParentUnknown,
    TargetConflictMissing,
    TargetConflictIdentical,
    TargetConflictDifferentBlocks,
    TargetConflictUnknown,
    ManifestAuthorityNotObservedStructure,
    ManifestStatusNotAdvisory,
    ManifestContainsAbsolutePath,
    ManifestContainsContentSnippet,
    ManifestContainsSummary,
    ManifestContainsClaimField,
    ManifestContainsClaimsCreated,
    RuntimeStorageTargetForbidden,
    OperationEvidenceIsNotProof,
}

impl SourceCorpusManifestWriterPreflightFinding {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TargetAllowed => "target_allowed",
            Self::TargetNotUnderReconstructionDir => "target_not_under_reconstruction_dir",
            Self::TargetAbsolutePath => "target_absolute_path",
            Self::TargetPathEscape => "target_path_escape",
            Self::TargetSymlinkAncestorEscape => "target_symlink_ancestor_escape",
            Self::TargetSymlinkAncestorUnknown => "target_symlink_ancestor_unknown",
            Self::TargetMissingParent => "target_missing_parent",
            Self::TargetParentUnknown => "target_parent_unknown",
            Self::TargetConflictMissing => "target_conflict_missing",
            Self::TargetConflictIdentical => "target_conflict_identical",
            Self::TargetConflictDifferentBlocks => "target_conflict_different_blocks",
            Self::TargetConflictUnknown => "target_conflict_unknown",
            Self::ManifestAuthorityNotObservedStructure => {
                "manifest_authority_not_observed_structure"
            }
            Self::ManifestStatusNotAdvisory => "manifest_status_not_advisory",
            Self::ManifestContainsAbsolutePath => "manifest_contains_absolute_path",
            Self::ManifestContainsContentSnippet => "manifest_contains_content_snippet",
            Self::ManifestContainsSummary => "manifest_contains_summary",
            Self::ManifestContainsClaimField => "manifest_contains_claim_field",
            Self::ManifestContainsClaimsCreated => "manifest_contains_claims_created",
            Self::RuntimeStorageTargetForbidden => "runtime_storage_target_forbidden",
            Self::OperationEvidenceIsNotProof => "operation_evidence_is_not_proof",
        }
    }

    pub fn blocks_write(self) -> bool {
        !matches!(
            self,
            Self::TargetAllowed
                | Self::TargetConflictMissing
                | Self::TargetConflictIdentical
                | Self::OperationEvidenceIsNotProof
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterPreflightCapabilities {
    scans_repository: bool,
    walks_files: bool,
    reads_file_contents: bool,
    computes_file_hashes: bool,
    writes_manifest: bool,
    generates_manifest: bool,
    creates_claims: bool,
    infers_intent: bool,
    uses_network: bool,
    uses_remote_ai: bool,
    writes_runtime_storage: bool,
}

impl SourceCorpusManifestWriterPreflightCapabilities {
    pub fn side_effect_free() -> Self {
        Self {
            scans_repository: false,
            walks_files: false,
            reads_file_contents: false,
            computes_file_hashes: false,
            writes_manifest: false,
            generates_manifest: false,
            creates_claims: false,
            infers_intent: false,
            uses_network: false,
            uses_remote_ai: false,
            writes_runtime_storage: false,
        }
    }

    pub fn scans_repository(self) -> bool {
        self.scans_repository
    }

    pub fn walks_files(self) -> bool {
        self.walks_files
    }

    pub fn reads_file_contents(self) -> bool {
        self.reads_file_contents
    }

    pub fn computes_file_hashes(self) -> bool {
        self.computes_file_hashes
    }

    pub fn writes_manifest(self) -> bool {
        self.writes_manifest
    }

    pub fn generates_manifest(self) -> bool {
        self.generates_manifest
    }

    pub fn creates_claims(self) -> bool {
        self.creates_claims
    }

    pub fn infers_intent(self) -> bool {
        self.infers_intent
    }

    pub fn uses_network(self) -> bool {
        self.uses_network
    }

    pub fn uses_remote_ai(self) -> bool {
        self.uses_remote_ai
    }

    pub fn writes_runtime_storage(self) -> bool {
        self.writes_runtime_storage
    }
}

impl Default for SourceCorpusManifestWriterPreflightCapabilities {
    fn default() -> Self {
        Self::side_effect_free()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterPreflightResult {
    target: SourceCorpusManifestWriterTarget,
    manifest_id: SourceCorpusManifestId,
    findings: Vec<SourceCorpusManifestWriterPreflightFinding>,
    capabilities: SourceCorpusManifestWriterPreflightCapabilities,
}

impl SourceCorpusManifestWriterPreflightResult {
    pub fn target(&self) -> &SourceCorpusManifestWriterTarget {
        &self.target
    }

    pub fn manifest_id(&self) -> &SourceCorpusManifestId {
        &self.manifest_id
    }

    pub fn findings(&self) -> &[SourceCorpusManifestWriterPreflightFinding] {
        &self.findings
    }

    pub fn has_finding(&self, finding: SourceCorpusManifestWriterPreflightFinding) -> bool {
        self.findings.contains(&finding)
    }

    pub fn capabilities(&self) -> SourceCorpusManifestWriterPreflightCapabilities {
        self.capabilities
    }

    pub fn allowed_to_write(&self) -> bool {
        self.findings.iter().all(|finding| !finding.blocks_write())
    }

    pub fn blocked(&self) -> bool {
        !self.allowed_to_write()
    }

    pub fn operation_evidence_is_proof(&self) -> bool {
        false
    }

    pub fn operation_evidence_is_gate_decision(&self) -> bool {
        false
    }

    pub fn operation_evidence_is_acceptance(&self) -> bool {
        false
    }

    pub fn operation_evidence_is_project_truth(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterPreflight;

impl SourceCorpusManifestWriterPreflight {
    pub fn schema_version() -> &'static str {
        SOURCE_CORPUS_MANIFEST_WRITER_PREFLIGHT_MODEL_SCHEMA_VERSION
    }

    pub fn evaluate(
        input: SourceCorpusManifestWriterPreflightInput,
    ) -> SourceCorpusManifestWriterPreflightResult {
        let mut findings = Vec::new();
        let result_target = input.target().clone();
        let result_manifest_id = input.manifest().manifest_id().clone();
        let target = result_target.as_str();

        if source_corpus_manifest_writer_target_is_runtime_storage(target) {
            findings
                .push(SourceCorpusManifestWriterPreflightFinding::RuntimeStorageTargetForbidden);
        }

        match validate_source_corpus_repo_relative_path(target) {
            Ok(()) => {
                if source_corpus_manifest_writer_target_allowed(target, input.target_policy()) {
                    findings.push(SourceCorpusManifestWriterPreflightFinding::TargetAllowed);
                } else {
                    findings.push(
                        SourceCorpusManifestWriterPreflightFinding::TargetNotUnderReconstructionDir,
                    );
                }
            }
            Err(error) => findings.push(source_corpus_manifest_writer_path_error_finding(error)),
        }

        match input.parent_status() {
            SourceCorpusManifestWriterParentStatus::Valid => {}
            SourceCorpusManifestWriterParentStatus::Missing => {
                findings.push(SourceCorpusManifestWriterPreflightFinding::TargetMissingParent);
            }
            SourceCorpusManifestWriterParentStatus::Unknown => {
                findings.push(SourceCorpusManifestWriterPreflightFinding::TargetParentUnknown);
            }
        }

        match input.symlink_ancestor_status() {
            SourceCorpusManifestWriterSymlinkAncestorStatus::Safe => {}
            SourceCorpusManifestWriterSymlinkAncestorStatus::Escape => {
                findings
                    .push(SourceCorpusManifestWriterPreflightFinding::TargetSymlinkAncestorEscape);
            }
            SourceCorpusManifestWriterSymlinkAncestorStatus::Unknown => {
                findings
                    .push(SourceCorpusManifestWriterPreflightFinding::TargetSymlinkAncestorUnknown);
            }
        }

        match input.conflict_policy() {
            SourceCorpusManifestWriterConflictPolicy::MissingTarget => {
                findings.push(SourceCorpusManifestWriterPreflightFinding::TargetConflictMissing);
            }
            SourceCorpusManifestWriterConflictPolicy::IdenticalExistingTarget => {
                findings.push(SourceCorpusManifestWriterPreflightFinding::TargetConflictIdentical);
            }
            SourceCorpusManifestWriterConflictPolicy::DifferentExistingTarget => findings
                .push(SourceCorpusManifestWriterPreflightFinding::TargetConflictDifferentBlocks),
            SourceCorpusManifestWriterConflictPolicy::Unknown => {
                findings.push(SourceCorpusManifestWriterPreflightFinding::TargetConflictUnknown);
            }
        }

        let inspection = input.manifest_inspection();
        if inspection.status_evidence()
            != SourceCorpusManifestWriterManifestStatusEvidence::Advisory
        {
            findings.push(SourceCorpusManifestWriterPreflightFinding::ManifestStatusNotAdvisory);
        }
        if inspection.authority_evidence()
            != SourceCorpusManifestWriterManifestAuthorityEvidence::ObservedStructure
        {
            findings.push(
                SourceCorpusManifestWriterPreflightFinding::ManifestAuthorityNotObservedStructure,
            );
        }
        if inspection.contains_absolute_path() {
            findings.push(SourceCorpusManifestWriterPreflightFinding::ManifestContainsAbsolutePath);
        }
        if inspection.contains_content_snippet() {
            findings
                .push(SourceCorpusManifestWriterPreflightFinding::ManifestContainsContentSnippet);
        }
        if inspection.contains_summary() {
            findings.push(SourceCorpusManifestWriterPreflightFinding::ManifestContainsSummary);
        }
        if !inspection.claim_fields().is_empty() {
            findings.push(SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimField);
        }
        if inspection
            .claim_fields()
            .iter()
            .any(|field| field == "claims_created")
        {
            findings
                .push(SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimsCreated);
        }

        findings.push(SourceCorpusManifestWriterPreflightFinding::OperationEvidenceIsNotProof);

        SourceCorpusManifestWriterPreflightResult {
            target: result_target,
            manifest_id: result_manifest_id,
            findings,
            capabilities: SourceCorpusManifestWriterPreflightCapabilities::default(),
        }
    }
}

fn source_corpus_manifest_writer_target_allowed(
    target: &str,
    policy: SourceCorpusManifestWriterTargetPolicy,
) -> bool {
    match policy {
        SourceCorpusManifestWriterTargetPolicy::DefaultManifestPath => {
            target == SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH
        }
        SourceCorpusManifestWriterTargetPolicy::ConfiguredUnderReconstructionDir => {
            target == SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH
                || source_corpus_manifest_writer_target_under_reconstruction_dir(target)
        }
    }
}

fn source_corpus_manifest_writer_target_under_reconstruction_dir(target: &str) -> bool {
    let prefix = format!("{SOURCE_CORPUS_MANIFEST_WRITER_RECONSTRUCTION_DIR}/");
    target.starts_with(&prefix)
}

fn source_corpus_manifest_writer_target_is_runtime_storage(target: &str) -> bool {
    SOURCE_CORPUS_MANIFEST_WRITER_FORBIDDEN_RUNTIME_TARGET_PREFIXES
        .iter()
        .any(|prefix| target == *prefix || target.starts_with(&format!("{prefix}/")))
}

fn source_corpus_manifest_writer_path_error_finding(
    error: SourceCorpusPathError,
) -> SourceCorpusManifestWriterPreflightFinding {
    match error {
        SourceCorpusPathError::Absolute
        | SourceCorpusPathError::HomePath
        | SourceCorpusPathError::UrlLike => {
            SourceCorpusManifestWriterPreflightFinding::TargetAbsolutePath
        }
        SourceCorpusPathError::Empty
        | SourceCorpusPathError::CurrentDirectory
        | SourceCorpusPathError::ParentTraversal
        | SourceCorpusPathError::CurrentSegment
        | SourceCorpusPathError::EmptySegment
        | SourceCorpusPathError::Backslash => {
            SourceCorpusManifestWriterPreflightFinding::TargetPathEscape
        }
    }
}

pub fn source_corpus_manifest_render_canonical_bytes(manifest: &SourceCorpusManifest) -> Vec<u8> {
    let mut output = String::new();

    source_corpus_manifest_write_quoted_field(
        &mut output,
        0,
        "manifest_id",
        manifest.manifest_id().as_str(),
    );
    source_corpus_manifest_write_quoted_field(
        &mut output,
        0,
        "schema_version",
        manifest.manifest_schema_version(),
    );
    source_corpus_manifest_write_quoted_field(
        &mut output,
        0,
        "project_id",
        manifest.project_id().as_str(),
    );
    writeln!(
        &mut output,
        "entry_mode: {}",
        manifest.entry_mode().as_str()
    )
    .expect("writing to String should succeed");
    writeln!(
        &mut output,
        "manifest_status: {}",
        manifest.status().as_str()
    )
    .expect("writing to String should succeed");
    writeln!(&mut output, "authority: {}", manifest.authority().as_str())
        .expect("writing to String should succeed");
    writeln!(&mut output, "generated_at_policy: no_runtime_clock")
        .expect("writing to String should succeed");
    writeln!(&mut output, "source_root_ref:").expect("writing to String should succeed");
    writeln!(
        &mut output,
        "  kind: {}",
        manifest.source_root_ref().kind().as_str()
    )
    .expect("writing to String should succeed");
    source_corpus_manifest_write_quoted_field(
        &mut output,
        2,
        "path",
        manifest.source_root_ref().path(),
    );
    writeln!(&mut output, "inventory_scope:").expect("writing to String should succeed");
    writeln!(&mut output, "  include:").expect("writing to String should succeed");
    source_corpus_manifest_write_quoted_list_item(&mut output, 4, ".");
    writeln!(&mut output, "  exclude:").expect("writing to String should succeed");
    for path in manifest.exclusion_policy().excluded_paths() {
        source_corpus_manifest_write_quoted_list_item(&mut output, 4, path);
    }
    writeln!(&mut output, "caution_classes:").expect("writing to String should succeed");
    for caution_class in manifest.exclusion_policy().caution_classes() {
        writeln!(&mut output, "  - {}", caution_class.as_str())
            .expect("writing to String should succeed");
    }
    writeln!(&mut output, "items:").expect("writing to String should succeed");
    if manifest.items().is_empty() {
        writeln!(&mut output, "  []").expect("writing to String should succeed");
    } else {
        for item in manifest.items() {
            source_corpus_manifest_write_quoted_field(
                &mut output,
                2,
                "- item_id",
                item.item_id().as_str(),
            );
            source_corpus_manifest_write_quoted_field(
                &mut output,
                4,
                "repo_relative_path",
                item.repo_relative_path().as_str(),
            );
            writeln!(
                &mut output,
                "    observed_kind: {}",
                item.observed_kind().as_str()
            )
            .expect("writing to String should succeed");
            writeln!(
                &mut output,
                "    source_class: {}",
                item.source_class().as_str()
            )
            .expect("writing to String should succeed");
            writeln!(&mut output, "    source_markers:").expect("writing to String should succeed");
            if item.source_markers().is_empty() {
                writeln!(&mut output, "      []").expect("writing to String should succeed");
            } else {
                for marker in item.source_markers() {
                    writeln!(&mut output, "      - kind: {}", marker.kind().as_str())
                        .expect("writing to String should succeed");
                    source_corpus_manifest_write_quoted_field(
                        &mut output,
                        8,
                        "value",
                        marker.value(),
                    );
                }
            }
            writeln!(
                &mut output,
                "    tracking_status: {}",
                item.tracking_status().as_str()
            )
            .expect("writing to String should succeed");
            writeln!(
                &mut output,
                "    sensitivity: {}",
                item.sensitivity().as_str()
            )
            .expect("writing to String should succeed");
            writeln!(
                &mut output,
                "    generated_or_vendored_candidate: {}",
                item.generated_or_vendored_candidate().as_str()
            )
            .expect("writing to String should succeed");
            writeln!(&mut output, "    size_policy:").expect("writing to String should succeed");
            writeln!(&mut output, "      status: {}", item.size_policy().as_str())
                .expect("writing to String should succeed");
            writeln!(&mut output, "      value: null").expect("writing to String should succeed");
            writeln!(&mut output, "    hash_policy:").expect("writing to String should succeed");
            writeln!(&mut output, "      status: {}", item.hash_policy().as_str())
                .expect("writing to String should succeed");
            writeln!(&mut output, "      value: null").expect("writing to String should succeed");
            writeln!(&mut output, "    content_policy:").expect("writing to String should succeed");
            writeln!(
                &mut output,
                "      read_contents: {}",
                item.content_policy().reads_contents()
            )
            .expect("writing to String should succeed");
            writeln!(
                &mut output,
                "      store_snippets: {}",
                item.content_policy().stores_snippets()
            )
            .expect("writing to String should succeed");
            writeln!(
                &mut output,
                "      summarize_contents: {}",
                item.content_policy().summarizes_contents()
            )
            .expect("writing to String should succeed");
            writeln!(&mut output, "    evidence_ref:").expect("writing to String should succeed");
            writeln!(&mut output, "      kind: manifest_observation")
                .expect("writing to String should succeed");
            source_corpus_manifest_write_quoted_field(
                &mut output,
                6,
                "ref",
                item.evidence_ref().as_str(),
            );
            writeln!(&mut output, "    notes:").expect("writing to String should succeed");
            if item.notes().is_empty() {
                writeln!(&mut output, "      []").expect("writing to String should succeed");
            } else {
                for note in item.notes() {
                    source_corpus_manifest_write_quoted_list_item(&mut output, 6, note);
                }
            }
        }
    }

    output.into_bytes()
}

fn source_corpus_manifest_write_quoted_field(
    output: &mut String,
    indent: usize,
    key: &str,
    value: &str,
) {
    source_corpus_manifest_push_indent(output, indent);
    write!(output, "{key}: ").expect("writing to String should succeed");
    source_corpus_manifest_push_quoted(output, value);
    output.push('\n');
}

fn source_corpus_manifest_write_quoted_list_item(output: &mut String, indent: usize, value: &str) {
    source_corpus_manifest_push_indent(output, indent);
    output.push_str("- ");
    source_corpus_manifest_push_quoted(output, value);
    output.push('\n');
}

fn source_corpus_manifest_push_indent(output: &mut String, indent: usize) {
    for _ in 0..indent {
        output.push(' ');
    }
}

fn source_corpus_manifest_push_quoted(output: &mut String, value: &str) {
    output.push('"');
    for value_char in value.chars() {
        match value_char {
            '\\' => output.push_str("\\\\"),
            '"' => output.push_str("\\\""),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            value_char if value_char.is_control() => {
                write!(output, "\\u{:04x}", value_char as u32)
                    .expect("writing to String should succeed");
            }
            value_char => output.push(value_char),
        }
    }
    output.push('"');
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceCorpusManifestWriterOperationEvidenceStatus {
    Attempted,
    Blocked,
    Written,
    Idempotent,
    Conflict,
    Error,
}

impl SourceCorpusManifestWriterOperationEvidenceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Attempted => "attempted",
            Self::Blocked => "blocked",
            Self::Written => "written",
            Self::Idempotent => "idempotent",
            Self::Conflict => "conflict",
            Self::Error => "error",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceCorpusManifestWriterFirstSliceBlocker {
    BlockingPreflightFinding,
    PreflightTargetMismatch,
    PreflightManifestMismatch,
    PreflightTargetNotAllowed,
    PreflightTargetStateMismatch,
    TargetRootMissing,
    TargetRootRelative,
    TargetRootUnavailable,
    TargetRootNotDirectory,
    TargetRootSymlink,
    TargetPathInvalid,
    TargetPathNotUnderReconstructionDir,
    TargetRuntimeStorageForbidden,
    TargetParentMissing,
    TargetParentNotDirectory,
    TargetParentSymlink,
    TargetSymlink,
    ExistingTargetDifferent,
    ExistingTargetReadFailed,
    TempFileCreateFailed,
    TempFileWriteFailed,
    TempFileLinkFailed,
    TempFileCleanupFailed,
}

impl SourceCorpusManifestWriterFirstSliceBlocker {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BlockingPreflightFinding => "blocking_preflight_finding",
            Self::PreflightTargetMismatch => "preflight_target_mismatch",
            Self::PreflightManifestMismatch => "preflight_manifest_mismatch",
            Self::PreflightTargetNotAllowed => "preflight_target_not_allowed",
            Self::PreflightTargetStateMismatch => "preflight_target_state_mismatch",
            Self::TargetRootMissing => "target_root_missing",
            Self::TargetRootRelative => "target_root_relative",
            Self::TargetRootUnavailable => "target_root_unavailable",
            Self::TargetRootNotDirectory => "target_root_not_directory",
            Self::TargetRootSymlink => "target_root_symlink",
            Self::TargetPathInvalid => "target_path_invalid",
            Self::TargetPathNotUnderReconstructionDir => "target_path_not_under_reconstruction_dir",
            Self::TargetRuntimeStorageForbidden => "target_runtime_storage_forbidden",
            Self::TargetParentMissing => "target_parent_missing",
            Self::TargetParentNotDirectory => "target_parent_not_directory",
            Self::TargetParentSymlink => "target_parent_symlink",
            Self::TargetSymlink => "target_symlink",
            Self::ExistingTargetDifferent => "existing_target_different",
            Self::ExistingTargetReadFailed => "existing_target_read_failed",
            Self::TempFileCreateFailed => "temp_file_create_failed",
            Self::TempFileWriteFailed => "temp_file_write_failed",
            Self::TempFileLinkFailed => "temp_file_link_failed",
            Self::TempFileCleanupFailed => "temp_file_cleanup_failed",
        }
    }

    pub fn is_fail_closed(self) -> bool {
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterFirstSliceCapabilities {
    takes_manifest_model_input: bool,
    renders_canonical_bytes: bool,
    requires_preflight_pass: bool,
    writes_one_safe_target: bool,
    emits_operation_evidence: bool,
    scans_repository: bool,
    walks_files: bool,
    reads_source_file_contents: bool,
    computes_source_file_hashes: bool,
    generates_manifest_from_repo: bool,
    creates_claims: bool,
    promotes_manifest_authority: bool,
    writes_runtime_storage: bool,
    activates_punk_writer: bool,
    writes_gate_or_proof_artifacts: bool,
    uses_network: bool,
    uses_remote_ai: bool,
}

impl SourceCorpusManifestWriterFirstSliceCapabilities {
    pub fn minimal_writer_slice() -> Self {
        Self {
            takes_manifest_model_input: true,
            renders_canonical_bytes: true,
            requires_preflight_pass: true,
            writes_one_safe_target: true,
            emits_operation_evidence: true,
            scans_repository: false,
            walks_files: false,
            reads_source_file_contents: false,
            computes_source_file_hashes: false,
            generates_manifest_from_repo: false,
            creates_claims: false,
            promotes_manifest_authority: false,
            writes_runtime_storage: false,
            activates_punk_writer: false,
            writes_gate_or_proof_artifacts: false,
            uses_network: false,
            uses_remote_ai: false,
        }
    }

    pub fn takes_manifest_model_input(self) -> bool {
        self.takes_manifest_model_input
    }

    pub fn renders_canonical_bytes(self) -> bool {
        self.renders_canonical_bytes
    }

    pub fn requires_preflight_pass(self) -> bool {
        self.requires_preflight_pass
    }

    pub fn writes_one_safe_target(self) -> bool {
        self.writes_one_safe_target
    }

    pub fn emits_operation_evidence(self) -> bool {
        self.emits_operation_evidence
    }

    pub fn scans_repository(self) -> bool {
        self.scans_repository
    }

    pub fn walks_files(self) -> bool {
        self.walks_files
    }

    pub fn reads_source_file_contents(self) -> bool {
        self.reads_source_file_contents
    }

    pub fn computes_source_file_hashes(self) -> bool {
        self.computes_source_file_hashes
    }

    pub fn generates_manifest_from_repo(self) -> bool {
        self.generates_manifest_from_repo
    }

    pub fn creates_claims(self) -> bool {
        self.creates_claims
    }

    pub fn promotes_manifest_authority(self) -> bool {
        self.promotes_manifest_authority
    }

    pub fn writes_runtime_storage(self) -> bool {
        self.writes_runtime_storage
    }

    pub fn activates_punk_writer(self) -> bool {
        self.activates_punk_writer
    }

    pub fn writes_gate_or_proof_artifacts(self) -> bool {
        self.writes_gate_or_proof_artifacts
    }

    pub fn uses_network(self) -> bool {
        self.uses_network
    }

    pub fn uses_remote_ai(self) -> bool {
        self.uses_remote_ai
    }
}

impl Default for SourceCorpusManifestWriterFirstSliceCapabilities {
    fn default() -> Self {
        Self::minimal_writer_slice()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceCorpusManifestWriterFirstSliceResult {
    schema_version: &'static str,
    manifest_id: SourceCorpusManifestId,
    target: SourceCorpusManifestWriterTarget,
    evidence_status: SourceCorpusManifestWriterOperationEvidenceStatus,
    write_attempted: bool,
    canonical_byte_len: usize,
    blockers: Vec<SourceCorpusManifestWriterFirstSliceBlocker>,
    preflight_findings: Vec<SourceCorpusManifestWriterPreflightFinding>,
    capabilities: SourceCorpusManifestWriterFirstSliceCapabilities,
}

impl SourceCorpusManifestWriterFirstSliceResult {
    fn new(
        manifest: &SourceCorpusManifest,
        target: SourceCorpusManifestWriterTarget,
        evidence_status: SourceCorpusManifestWriterOperationEvidenceStatus,
        write_attempted: bool,
        canonical_byte_len: usize,
        blockers: Vec<SourceCorpusManifestWriterFirstSliceBlocker>,
        preflight_findings: &[SourceCorpusManifestWriterPreflightFinding],
    ) -> Self {
        Self {
            schema_version: SOURCE_CORPUS_MANIFEST_WRITER_FIRST_SLICE_SCHEMA_VERSION,
            manifest_id: manifest.manifest_id().clone(),
            target,
            evidence_status,
            write_attempted,
            canonical_byte_len,
            blockers,
            preflight_findings: preflight_findings.to_vec(),
            capabilities: SourceCorpusManifestWriterFirstSliceCapabilities::default(),
        }
    }

    pub fn schema_version(&self) -> &str {
        self.schema_version
    }

    pub fn manifest_id(&self) -> &SourceCorpusManifestId {
        &self.manifest_id
    }

    pub fn target(&self) -> &SourceCorpusManifestWriterTarget {
        &self.target
    }

    pub fn evidence_status(&self) -> SourceCorpusManifestWriterOperationEvidenceStatus {
        self.evidence_status
    }

    pub fn write_attempted(&self) -> bool {
        self.write_attempted
    }

    pub fn canonical_byte_len(&self) -> usize {
        self.canonical_byte_len
    }

    pub fn blockers(&self) -> &[SourceCorpusManifestWriterFirstSliceBlocker] {
        &self.blockers
    }

    pub fn has_blocker(&self, blocker: SourceCorpusManifestWriterFirstSliceBlocker) -> bool {
        self.blockers.contains(&blocker)
    }

    pub fn blockers_fail_closed(&self) -> bool {
        self.blockers.iter().all(|blocker| blocker.is_fail_closed())
    }

    pub fn preflight_findings(&self) -> &[SourceCorpusManifestWriterPreflightFinding] {
        &self.preflight_findings
    }

    pub fn capabilities(&self) -> SourceCorpusManifestWriterFirstSliceCapabilities {
        self.capabilities
    }

    pub fn wrote_manifest(&self) -> bool {
        self.evidence_status == SourceCorpusManifestWriterOperationEvidenceStatus::Written
    }

    pub fn is_idempotent(&self) -> bool {
        self.evidence_status == SourceCorpusManifestWriterOperationEvidenceStatus::Idempotent
    }

    pub fn has_conflict(&self) -> bool {
        self.evidence_status == SourceCorpusManifestWriterOperationEvidenceStatus::Conflict
    }

    pub fn is_blocked(&self) -> bool {
        self.evidence_status == SourceCorpusManifestWriterOperationEvidenceStatus::Blocked
    }

    pub fn is_error(&self) -> bool {
        self.evidence_status == SourceCorpusManifestWriterOperationEvidenceStatus::Error
    }

    pub fn operation_evidence_is_proof(&self) -> bool {
        false
    }

    pub fn operation_evidence_is_gate_decision(&self) -> bool {
        false
    }

    pub fn operation_evidence_is_acceptance(&self) -> bool {
        false
    }

    pub fn operation_evidence_is_project_truth(&self) -> bool {
        false
    }

    pub fn creates_claims(&self) -> bool {
        false
    }

    pub fn promotes_manifest_authority(&self) -> bool {
        false
    }
}

pub fn source_corpus_manifest_writer_write_first_slice(
    manifest: &SourceCorpusManifest,
    target_root_path: impl AsRef<Path>,
    target: SourceCorpusManifestWriterTarget,
    preflight: &SourceCorpusManifestWriterPreflightResult,
) -> SourceCorpusManifestWriterFirstSliceResult {
    let canonical_bytes = source_corpus_manifest_render_canonical_bytes(manifest);
    let mut blockers = source_corpus_manifest_writer_first_slice_precondition_blockers(
        manifest,
        target_root_path.as_ref(),
        &target,
        preflight,
    );

    if preflight
        .has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictDifferentBlocks)
    {
        source_corpus_manifest_writer_push_first_slice_blocker(
            &mut blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent,
        );
        return SourceCorpusManifestWriterFirstSliceResult::new(
            manifest,
            target,
            SourceCorpusManifestWriterOperationEvidenceStatus::Conflict,
            false,
            canonical_bytes.len(),
            blockers,
            preflight.findings(),
        );
    }

    if !blockers.is_empty() {
        return SourceCorpusManifestWriterFirstSliceResult::new(
            manifest,
            target,
            SourceCorpusManifestWriterOperationEvidenceStatus::Blocked,
            false,
            canonical_bytes.len(),
            blockers,
            preflight.findings(),
        );
    }

    let target_path = target_root_path.as_ref().join(target.as_str());

    if preflight.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictIdentical) {
        return match fs::symlink_metadata(&target_path) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                source_corpus_manifest_writer_push_first_slice_blocker(
                    &mut blockers,
                    SourceCorpusManifestWriterFirstSliceBlocker::TargetSymlink,
                );
                SourceCorpusManifestWriterFirstSliceResult::new(
                    manifest,
                    target,
                    SourceCorpusManifestWriterOperationEvidenceStatus::Blocked,
                    false,
                    canonical_bytes.len(),
                    blockers,
                    preflight.findings(),
                )
            }
            Ok(metadata) if metadata.is_file() => match fs::read(&target_path) {
                Ok(existing_bytes) if existing_bytes == canonical_bytes => {
                    SourceCorpusManifestWriterFirstSliceResult::new(
                        manifest,
                        target,
                        SourceCorpusManifestWriterOperationEvidenceStatus::Idempotent,
                        false,
                        canonical_bytes.len(),
                        blockers,
                        preflight.findings(),
                    )
                }
                Ok(_) => {
                    source_corpus_manifest_writer_push_first_slice_blocker(
                        &mut blockers,
                        SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent,
                    );
                    SourceCorpusManifestWriterFirstSliceResult::new(
                        manifest,
                        target,
                        SourceCorpusManifestWriterOperationEvidenceStatus::Conflict,
                        false,
                        canonical_bytes.len(),
                        blockers,
                        preflight.findings(),
                    )
                }
                Err(_) => {
                    source_corpus_manifest_writer_push_first_slice_blocker(
                        &mut blockers,
                        SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetReadFailed,
                    );
                    SourceCorpusManifestWriterFirstSliceResult::new(
                        manifest,
                        target,
                        SourceCorpusManifestWriterOperationEvidenceStatus::Blocked,
                        false,
                        canonical_bytes.len(),
                        blockers,
                        preflight.findings(),
                    )
                }
            },
            Ok(_) | Err(_) => {
                source_corpus_manifest_writer_push_first_slice_blocker(
                    &mut blockers,
                    SourceCorpusManifestWriterFirstSliceBlocker::PreflightTargetStateMismatch,
                );
                SourceCorpusManifestWriterFirstSliceResult::new(
                    manifest,
                    target,
                    SourceCorpusManifestWriterOperationEvidenceStatus::Blocked,
                    false,
                    canonical_bytes.len(),
                    blockers,
                    preflight.findings(),
                )
            }
        };
    }

    match fs::symlink_metadata(&target_path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                &mut blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::TargetSymlink,
            );
            return SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Blocked,
                false,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            );
        }
        Ok(_) => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                &mut blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent,
            );
            return SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Conflict,
                false,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            );
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(_) => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                &mut blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::PreflightTargetStateMismatch,
            );
            return SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Blocked,
                false,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            );
        }
    }

    let temp_path = source_corpus_manifest_writer_first_slice_temp_path(&target_path);
    match source_corpus_manifest_writer_write_temp_bytes(&temp_path, &canonical_bytes) {
        Ok(()) => {}
        Err(error) => {
            let blocker = if error.kind() == io::ErrorKind::AlreadyExists {
                SourceCorpusManifestWriterFirstSliceBlocker::TempFileCreateFailed
            } else {
                SourceCorpusManifestWriterFirstSliceBlocker::TempFileWriteFailed
            };
            source_corpus_manifest_writer_push_first_slice_blocker(&mut blockers, blocker);
            let _ = fs::remove_file(&temp_path);
            return SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Error,
                true,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            );
        }
    }

    match fs::hard_link(&temp_path, &target_path) {
        Ok(()) => match fs::remove_file(&temp_path) {
            Ok(()) => SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Written,
                true,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            ),
            Err(_) => {
                source_corpus_manifest_writer_push_first_slice_blocker(
                    &mut blockers,
                    SourceCorpusManifestWriterFirstSliceBlocker::TempFileCleanupFailed,
                );
                SourceCorpusManifestWriterFirstSliceResult::new(
                    manifest,
                    target,
                    SourceCorpusManifestWriterOperationEvidenceStatus::Error,
                    true,
                    canonical_bytes.len(),
                    blockers,
                    preflight.findings(),
                )
            }
        },
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                &mut blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent,
            );
            let _ = fs::remove_file(&temp_path);
            SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Conflict,
                true,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            )
        }
        Err(_) => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                &mut blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::TempFileLinkFailed,
            );
            let _ = fs::remove_file(&temp_path);
            SourceCorpusManifestWriterFirstSliceResult::new(
                manifest,
                target,
                SourceCorpusManifestWriterOperationEvidenceStatus::Error,
                true,
                canonical_bytes.len(),
                blockers,
                preflight.findings(),
            )
        }
    }
}

fn source_corpus_manifest_writer_first_slice_precondition_blockers(
    manifest: &SourceCorpusManifest,
    target_root_path: &Path,
    target: &SourceCorpusManifestWriterTarget,
    preflight: &SourceCorpusManifestWriterPreflightResult,
) -> Vec<SourceCorpusManifestWriterFirstSliceBlocker> {
    let mut blockers = Vec::new();

    if preflight.target() != target {
        source_corpus_manifest_writer_push_first_slice_blocker(
            &mut blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::PreflightTargetMismatch,
        );
    }

    if preflight.manifest_id() != manifest.manifest_id() {
        source_corpus_manifest_writer_push_first_slice_blocker(
            &mut blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::PreflightManifestMismatch,
        );
    }

    if preflight.blocked() {
        source_corpus_manifest_writer_push_first_slice_blocker(
            &mut blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::BlockingPreflightFinding,
        );
    }

    if !preflight.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetAllowed) {
        source_corpus_manifest_writer_push_first_slice_blocker(
            &mut blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::PreflightTargetNotAllowed,
        );
    }

    if !preflight.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictMissing)
        && !preflight
            .has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictIdentical)
    {
        source_corpus_manifest_writer_push_first_slice_blocker(
            &mut blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::PreflightTargetStateMismatch,
        );
    }

    source_corpus_manifest_writer_first_slice_path_blockers(
        target_root_path,
        target,
        &mut blockers,
    );

    blockers
}

fn source_corpus_manifest_writer_first_slice_path_blockers(
    target_root_path: &Path,
    target: &SourceCorpusManifestWriterTarget,
    blockers: &mut Vec<SourceCorpusManifestWriterFirstSliceBlocker>,
) {
    if target_root_path.as_os_str().is_empty() {
        source_corpus_manifest_writer_push_first_slice_blocker(
            blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::TargetRootMissing,
        );
    }

    if !target_root_path.is_absolute() {
        source_corpus_manifest_writer_push_first_slice_blocker(
            blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::TargetRootRelative,
        );
    }

    match fs::symlink_metadata(target_root_path) {
        Ok(metadata) if metadata.file_type().is_symlink() => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::TargetRootSymlink,
            );
        }
        Ok(metadata) if !metadata.is_dir() => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::TargetRootNotDirectory,
            );
        }
        Ok(_) => {}
        Err(_) => {
            source_corpus_manifest_writer_push_first_slice_blocker(
                blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::TargetRootUnavailable,
            );
        }
    }

    if source_corpus_manifest_writer_target_is_runtime_storage(target.as_str()) {
        source_corpus_manifest_writer_push_first_slice_blocker(
            blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::TargetRuntimeStorageForbidden,
        );
    }

    if validate_source_corpus_repo_relative_path(target.as_str()).is_err() {
        source_corpus_manifest_writer_push_first_slice_blocker(
            blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::TargetPathInvalid,
        );
    }

    if !source_corpus_manifest_writer_target_under_reconstruction_dir(target.as_str())
        && target.as_str() != SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH
    {
        source_corpus_manifest_writer_push_first_slice_blocker(
            blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::TargetPathNotUnderReconstructionDir,
        );
    }

    if !blockers.is_empty() {
        return;
    }

    source_corpus_manifest_writer_first_slice_parent_path_blockers(
        target_root_path,
        Path::new(target.as_str()),
        blockers,
    );
}

fn source_corpus_manifest_writer_first_slice_parent_path_blockers(
    target_root_path: &Path,
    target_relative_path: &Path,
    blockers: &mut Vec<SourceCorpusManifestWriterFirstSliceBlocker>,
) {
    let Some(parent_relative_path) = target_relative_path.parent() else {
        source_corpus_manifest_writer_push_first_slice_blocker(
            blockers,
            SourceCorpusManifestWriterFirstSliceBlocker::TargetParentMissing,
        );
        return;
    };

    let mut current_parent = target_root_path.to_path_buf();
    for component in parent_relative_path.components() {
        let std::path::Component::Normal(component) = component else {
            source_corpus_manifest_writer_push_first_slice_blocker(
                blockers,
                SourceCorpusManifestWriterFirstSliceBlocker::TargetPathInvalid,
            );
            return;
        };

        current_parent.push(component);
        match fs::symlink_metadata(&current_parent) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                source_corpus_manifest_writer_push_first_slice_blocker(
                    blockers,
                    SourceCorpusManifestWriterFirstSliceBlocker::TargetParentSymlink,
                );
                return;
            }
            Ok(metadata) if !metadata.is_dir() => {
                source_corpus_manifest_writer_push_first_slice_blocker(
                    blockers,
                    SourceCorpusManifestWriterFirstSliceBlocker::TargetParentNotDirectory,
                );
                return;
            }
            Ok(_) => {}
            Err(_) => {
                source_corpus_manifest_writer_push_first_slice_blocker(
                    blockers,
                    SourceCorpusManifestWriterFirstSliceBlocker::TargetParentMissing,
                );
                return;
            }
        }
    }
}

fn source_corpus_manifest_writer_push_first_slice_blocker(
    blockers: &mut Vec<SourceCorpusManifestWriterFirstSliceBlocker>,
    blocker: SourceCorpusManifestWriterFirstSliceBlocker,
) {
    if !blockers.contains(&blocker) {
        blockers.push(blocker);
    }
}

fn source_corpus_manifest_writer_first_slice_temp_path(target_path: &Path) -> PathBuf {
    let parent = target_path.parent().unwrap_or_else(|| Path::new("."));
    let file_name = target_path
        .file_name()
        .map(|name| name.to_string_lossy())
        .unwrap_or_else(|| "source-corpus-manifest.md".into());
    let counter = SOURCE_CORPUS_MANIFEST_WRITER_TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);

    parent.join(format!(
        ".{file_name}.tmp.{}.{}",
        std::process::id(),
        counter
    ))
}

fn source_corpus_manifest_writer_write_temp_bytes(
    temp_path: &Path,
    canonical_bytes: &[u8],
) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(temp_path)?;
    file.write_all(canonical_bytes)?;
    file.flush()?;
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectInitEntry {
    Directory(&'static str),
    File(&'static str, &'static str),
    GeneratedFile(&'static str, ProjectInitTemplate),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectInitTemplate {
    WorkStatus,
    InitialGoal,
    PunkReadme,
    PunkProjectToml,
}

const GREENFIELD_PROJECT_INIT_ENTRIES: &[ProjectInitEntry] = &[
    ProjectInitEntry::Directory(".punk"),
    ProjectInitEntry::Directory(MEMORY_ROOT),
    ProjectInitEntry::Directory(INSTRUCTIONS_ROOT),
    ProjectInitEntry::Directory(INSTRUCTION_PAGES_ROOT),
    ProjectInitEntry::Directory(INSTRUCTION_MODULES_ROOT),
    ProjectInitEntry::Directory(".punk/memory/goals"),
    ProjectInitEntry::Directory(".punk/memory/reports"),
    ProjectInitEntry::Directory(".punk/memory/knowledge"),
    ProjectInitEntry::Directory(".punk/memory/knowledge/research"),
    ProjectInitEntry::Directory(".punk/memory/knowledge/ideas"),
    ProjectInitEntry::Directory(".punk/memory/adr"),
    ProjectInitEntry::GeneratedFile(STATUS_PATH, ProjectInitTemplate::WorkStatus),
    ProjectInitEntry::GeneratedFile(INITIAL_GOAL_PATH, ProjectInitTemplate::InitialGoal),
    ProjectInitEntry::File(".punk/memory/reports/README.md", REPORTS_README_TEMPLATE),
    ProjectInitEntry::File(".punk/memory/adr/README.md", ADR_README_TEMPLATE),
    ProjectInitEntry::File(
        ".punk/memory/knowledge/research/README.md",
        RESEARCH_README_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/knowledge/ideas/README.md",
        IDEAS_README_TEMPLATE,
    ),
    ProjectInitEntry::File(INSTRUCTIONS_INDEX_PATH, INSTRUCTIONS_INDEX_TEMPLATE),
    ProjectInitEntry::File(
        ".punk/instructions/pages/getting-started.md",
        INSTRUCTIONS_GETTING_STARTED_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/layout.md",
        INSTRUCTIONS_LAYOUT_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/init.md",
        INSTRUCTIONS_INIT_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/modules.md",
        INSTRUCTIONS_MODULES_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/authority.md",
        INSTRUCTIONS_AUTHORITY_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/modules/README.md",
        INSTRUCTION_MODULES_README_TEMPLATE,
    ),
    ProjectInitEntry::GeneratedFile(".punk/README.md", ProjectInitTemplate::PunkReadme),
    ProjectInitEntry::GeneratedFile(".punk/project.toml", ProjectInitTemplate::PunkProjectToml),
];

const BROWNFIELD_PROJECT_INIT_ENTRIES: &[ProjectInitEntry] = &[
    ProjectInitEntry::Directory(".punk"),
    ProjectInitEntry::Directory(MEMORY_ROOT),
    ProjectInitEntry::Directory(INSTRUCTIONS_ROOT),
    ProjectInitEntry::Directory(INSTRUCTION_PAGES_ROOT),
    ProjectInitEntry::Directory(INSTRUCTION_MODULES_ROOT),
    ProjectInitEntry::Directory(".punk/memory/goals"),
    ProjectInitEntry::Directory(".punk/memory/reports"),
    ProjectInitEntry::Directory(".punk/memory/reconstruction"),
    ProjectInitEntry::GeneratedFile(STATUS_PATH, ProjectInitTemplate::WorkStatus),
    ProjectInitEntry::GeneratedFile(
        BROWNFIELD_BASELINE_GOAL_PATH,
        ProjectInitTemplate::InitialGoal,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/README.md",
        BROWNFIELD_RECONSTRUCTION_README_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/source-corpus-manifest.md",
        SOURCE_CORPUS_MANIFEST_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/claim-ledger.md",
        CLAIM_LEDGER_TEMPLATE,
    ),
    ProjectInitEntry::File(".punk/memory/reconstruction/unknowns.md", UNKNOWNS_TEMPLATE),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/contradictions.md",
        CONTRADICTIONS_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/memory/reconstruction/contract-readiness.md",
        CONTRACT_READINESS_TEMPLATE,
    ),
    ProjectInitEntry::File(INSTRUCTIONS_INDEX_PATH, INSTRUCTIONS_INDEX_TEMPLATE),
    ProjectInitEntry::File(
        ".punk/instructions/pages/getting-started.md",
        INSTRUCTIONS_GETTING_STARTED_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/layout.md",
        INSTRUCTIONS_LAYOUT_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/init.md",
        INSTRUCTIONS_INIT_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/modules.md",
        INSTRUCTIONS_MODULES_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/pages/authority.md",
        INSTRUCTIONS_AUTHORITY_TEMPLATE,
    ),
    ProjectInitEntry::File(
        ".punk/instructions/modules/README.md",
        INSTRUCTION_MODULES_README_TEMPLATE,
    ),
    ProjectInitEntry::GeneratedFile(".punk/README.md", ProjectInitTemplate::PunkReadme),
    ProjectInitEntry::GeneratedFile(".punk/project.toml", ProjectInitTemplate::PunkProjectToml),
];

pub fn init_level0_project(
    project_root: impl AsRef<Path>,
    project_id: ProjectId,
) -> ProjectInitReport {
    init_project(project_root, project_id, ProjectInitEntryMode::Greenfield)
}

pub fn init_project(
    project_root: impl AsRef<Path>,
    project_id: ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> ProjectInitReport {
    let project_root = project_root.as_ref().to_path_buf();

    match fs::metadata(&project_root) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => {
            let artifacts = vec![ProjectInitArtifactReport::new(
                ".",
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some("project root exists but is not a directory".to_owned()),
            )];
            return ProjectInitReport {
                project_root,
                project_id,
                entry_mode,
                artifacts,
            };
        }
        Err(error) => {
            let artifacts = vec![ProjectInitArtifactReport::new(
                ".",
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some(format!("project root must already exist: {error}")),
            )];
            return ProjectInitReport {
                project_root,
                project_id,
                entry_mode,
                artifacts,
            };
        }
    }

    let preflight = preflight_init_entries(&project_root, &project_id, entry_mode);
    if preflight.iter().any(ProjectInitArtifactReport::is_blocking) {
        return ProjectInitReport {
            project_root,
            project_id,
            entry_mode,
            artifacts: preflight,
        };
    }

    let artifacts = apply_init_entries(&project_root, &project_id, entry_mode, &preflight);

    ProjectInitReport {
        project_root,
        project_id,
        entry_mode,
        artifacts,
    }
}

fn render_init_template(
    template: ProjectInitTemplate,
    project_id: &ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> String {
    match (template, entry_mode) {
        (ProjectInitTemplate::WorkStatus, ProjectInitEntryMode::Greenfield) => {
            work_status_template(project_id)
        }
        (ProjectInitTemplate::WorkStatus, ProjectInitEntryMode::Brownfield) => {
            brownfield_work_status_template(project_id)
        }
        (ProjectInitTemplate::InitialGoal, ProjectInitEntryMode::Greenfield) => {
            initial_goal_template(project_id)
        }
        (ProjectInitTemplate::InitialGoal, ProjectInitEntryMode::Brownfield) => {
            brownfield_baseline_goal_template(project_id)
        }
        (ProjectInitTemplate::PunkReadme, _) => punk_readme_template(entry_mode),
        (ProjectInitTemplate::PunkProjectToml, _) => {
            punk_project_toml_template(project_id, entry_mode)
        }
    }
}

fn preflight_init_entries(
    project_root: &Path,
    project_id: &ProjectId,
    entry_mode: ProjectInitEntryMode,
) -> Vec<ProjectInitArtifactReport> {
    entry_mode
        .entries()
        .iter()
        .map(|entry| match *entry {
            ProjectInitEntry::Directory(path) => preflight_init_directory(project_root, path),
            ProjectInitEntry::File(path, contents) => {
                preflight_init_file(project_root, path, contents.as_bytes())
            }
            ProjectInitEntry::GeneratedFile(path, template) => {
                let contents = render_init_template(template, project_id, entry_mode);
                preflight_init_file(project_root, path, contents.as_bytes())
            }
        })
        .collect()
}

fn apply_init_entries(
    project_root: &Path,
    project_id: &ProjectId,
    entry_mode: ProjectInitEntryMode,
    preflight: &[ProjectInitArtifactReport],
) -> Vec<ProjectInitArtifactReport> {
    entry_mode
        .entries()
        .iter()
        .zip(preflight)
        .map(|(entry, preflight_report)| {
            if preflight_report.status() == ProjectInitArtifactStatus::AlreadyExists {
                return preflight_report.clone();
            }

            match *entry {
                ProjectInitEntry::Directory(path) => create_init_directory(project_root, path),
                ProjectInitEntry::File(path, contents) => {
                    create_init_file(project_root, path, contents.as_bytes())
                }
                ProjectInitEntry::GeneratedFile(path, template) => {
                    let contents = render_init_template(template, project_id, entry_mode);
                    create_init_file(project_root, path, contents.as_bytes())
                }
            }
        })
        .collect()
}

fn preflight_init_directory(
    project_root: &Path,
    repo_relative_path: &'static str,
) -> ProjectInitArtifactReport {
    let path = project_root.join(repo_relative_path);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::AlreadyExists,
                None,
            )
        }
        Ok(_) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Conflict,
            Some("path already exists and is not a plain directory".to_owned()),
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Planned,
            None,
        ),
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

fn preflight_init_file(
    project_root: &Path,
    repo_relative_path: &'static str,
    contents: &[u8],
) -> ProjectInitArtifactReport {
    let path = project_root.join(repo_relative_path);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {
            match fs::read(&path) {
                Ok(existing) if existing == contents => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::AlreadyExists,
                    None,
                ),
                Ok(_) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Conflict,
                    Some("file already exists with different contents; not overwritten".to_owned()),
                ),
                Err(error) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Failed,
                    Some(format!("could not read existing file: {error}")),
                ),
            }
        }
        Ok(_) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Conflict,
            Some("path already exists and is not a plain file".to_owned()),
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Planned,
            None,
        ),
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

fn create_init_directory(
    project_root: &Path,
    repo_relative_path: &'static str,
) -> ProjectInitArtifactReport {
    let path = project_root.join(repo_relative_path);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::AlreadyExists,
                None,
            )
        }
        Ok(_) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Conflict,
            Some("path already exists and is not a plain directory".to_owned()),
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => match fs::create_dir(&path) {
            Ok(()) => ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Created,
                None,
            ),
            Err(create_error) => ProjectInitArtifactReport::new(
                repo_relative_path,
                ProjectInitArtifactKind::Directory,
                ProjectInitArtifactStatus::Failed,
                Some(format!("could not create directory: {create_error}")),
            ),
        },
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::Directory,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

fn create_init_file(
    project_root: &Path,
    repo_relative_path: &'static str,
    contents: &[u8],
) -> ProjectInitArtifactReport {
    let path = project_root.join(repo_relative_path);
    match fs::symlink_metadata(&path) {
        Ok(metadata) if metadata.file_type().is_file() && !metadata.file_type().is_symlink() => {
            match fs::read(&path) {
                Ok(existing) if existing == contents => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::AlreadyExists,
                    None,
                ),
                Ok(_) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Conflict,
                    Some("file already exists with different contents; not overwritten".to_owned()),
                ),
                Err(error) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Failed,
                    Some(format!("could not read existing file: {error}")),
                ),
            }
        }
        Ok(_) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Conflict,
            Some("path already exists and is not a plain file".to_owned()),
        ),
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            let write_result = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .and_then(|mut file| file.write_all(contents));

            match write_result {
                Ok(()) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Created,
                    None,
                ),
                Err(write_error) => ProjectInitArtifactReport::new(
                    repo_relative_path,
                    ProjectInitArtifactKind::File,
                    ProjectInitArtifactStatus::Failed,
                    Some(format!("could not create file: {write_error}")),
                ),
            }
        }
        Err(error) => ProjectInitArtifactReport::new(
            repo_relative_path,
            ProjectInitArtifactKind::File,
            ProjectInitArtifactStatus::Failed,
            Some(format!("could not inspect path: {error}")),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        default_instruction_page_index_nodes, init_level0_project, init_project,
        render_instruction_page_index_json, ProjectId, ProjectIdError, ProjectInitArtifactKind,
        ProjectInitArtifactStatus, ProjectInitEntryMode, BROWNFIELD_BASELINE_GOAL_PATH,
        INITIAL_GOAL_PATH, INSTRUCTIONS_INDEX_PATH, INSTRUCTION_PAGE_INDEX_SCHEMA_VERSION,
        INSTRUCTION_PAGE_INDEX_VIEW_PATH, PROJECT_INIT_BROWNFIELD_ENTRY_MODE,
        PROJECT_INIT_ENTRY_MODE, SOURCE_CORPUS_FORBIDDEN_CLAIM_FIELDS, STATUS_PATH,
    };
    use super::{
        source_corpus_manifest_claim_field_allowed, source_corpus_manifest_render_canonical_bytes,
        source_corpus_manifest_writer_write_first_slice, SourceCorpusCautionClass,
        SourceCorpusGeneratedOrVendoredCandidate, SourceCorpusHashPolicy, SourceCorpusItem,
        SourceCorpusItemId, SourceCorpusManifest, SourceCorpusManifestAuthority,
        SourceCorpusManifestId, SourceCorpusManifestStatus,
        SourceCorpusManifestWriterConflictPolicy, SourceCorpusManifestWriterFirstSliceBlocker,
        SourceCorpusManifestWriterManifestAuthorityEvidence,
        SourceCorpusManifestWriterManifestInspection,
        SourceCorpusManifestWriterManifestStatusEvidence,
        SourceCorpusManifestWriterOperationEvidenceStatus, SourceCorpusManifestWriterParentStatus,
        SourceCorpusManifestWriterPreflight, SourceCorpusManifestWriterPreflightFinding,
        SourceCorpusManifestWriterPreflightInput, SourceCorpusManifestWriterSymlinkAncestorStatus,
        SourceCorpusManifestWriterTarget, SourceCorpusObservedKind, SourceCorpusPathError,
        SourceCorpusRepoRelativePath, SourceCorpusSensitivityClass, SourceCorpusSizePolicy,
        SourceCorpusSourceClass, SourceCorpusSourceMarker, SourceCorpusSourceMarkerKind,
        SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS, SOURCE_CORPUS_MANIFEST_MODEL_SCHEMA_VERSION,
        SOURCE_CORPUS_MANIFEST_SCHEMA_VERSION, SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH,
        SOURCE_CORPUS_MANIFEST_WRITER_FIRST_SLICE_SCHEMA_VERSION,
        SOURCE_CORPUS_MANIFEST_WRITER_PREFLIGHT_MODEL_SCHEMA_VERSION,
    };
    use std::fs;
    use std::process;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static TEMP_COUNTER: AtomicU64 = AtomicU64::new(0);

    #[test]
    fn instruction_page_index_model_is_deterministic_and_advisory() {
        let nodes = default_instruction_page_index_nodes();
        let rendered = render_instruction_page_index_json(&nodes);
        let rendered_again = render_instruction_page_index_json(&nodes);

        assert_eq!(rendered, rendered_again);
        assert!(rendered.contains(&format!(
            "\"schema_version\": \"{INSTRUCTION_PAGE_INDEX_SCHEMA_VERSION}\""
        )));
        assert!(rendered.contains("\"authority\": \"advisory\""));
        assert!(rendered.contains("\"generated_from\": \"source_instruction_pages\""));
        assert!(rendered.contains(&format!(
            "\"view_ref\": \"{INSTRUCTION_PAGE_INDEX_VIEW_PATH}\""
        )));
        assert!(rendered.contains("\"source_ref\": \".punk/instructions/INDEX.md\""));
        assert!(rendered.contains("\"source_ref\": \".punk/instructions/pages/init.md\""));
        assert!(rendered.contains("\"module_id\": \"module-host\""));
        assert!(rendered.contains("\"status\": \"parked\""));
        assert!(!rendered.contains("raw_prompt"));
        assert!(!rendered.contains("transcript"));
    }

    #[test]
    fn init_creates_level0_manual_memory_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(!report.blocked());
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.result_label(), "initialized");
        assert_eq!(report.project_id().as_str(), "weekend-project");
        assert_eq!(report.entry_mode(), PROJECT_INIT_ENTRY_MODE);
        assert!(root.join(STATUS_PATH).is_file());
        assert!(root.join(INITIAL_GOAL_PATH).is_file());
        assert!(root.join(".punk/memory/reports/README.md").is_file());
        assert!(root.join(".punk/memory/adr/README.md").is_file());
        assert!(root
            .join(".punk/memory/knowledge/research/README.md")
            .is_file());
        assert!(root
            .join(".punk/memory/knowledge/ideas/README.md")
            .is_file());
        assert!(root.join(".punk/README.md").is_file());
        assert!(root.join(".punk/project.toml").is_file());
        assert!(root.join(INSTRUCTIONS_INDEX_PATH).is_file());
        assert!(root
            .join(".punk/instructions/pages/getting-started.md")
            .is_file());
        assert!(root.join(".punk/instructions/pages/layout.md").is_file());
        assert!(root.join(".punk/instructions/pages/init.md").is_file());
        assert!(root.join(".punk/instructions/pages/modules.md").is_file());
        assert!(root.join(".punk/instructions/pages/authority.md").is_file());
        assert!(root.join(".punk/instructions/modules/README.md").is_file());
        assert!(!root.join("work").exists());
        assert!(!root.join("knowledge").exists());
        assert!(!root.join("docs").exists());
        assert!(!root.join("docs/adr").exists());
        assert!(!root.join("publishing").exists());
        assert!(!root.join(".punk/runtime").exists());
        assert!(!root.join(".punk/cache").exists());
        assert!(!root.join(".punk/events").exists());
        assert!(!root.join(".punk/contracts").exists());
        assert!(!root.join(".punk/runs").exists());
        assert!(!root.join(".punk/evals").exists());
        assert!(!root.join(".punk/decisions").exists());
        assert!(!root.join(".punk/proofs").exists());
        assert!(!root.join(".punk/indexes").exists());
        assert!(!root.join(".punk/views").exists());

        let status =
            fs::read_to_string(root.join(STATUS_PATH)).expect("status template should be readable");
        assert!(status.contains("dogfooding_level: 0"));
        assert!(status.contains("project_id: \"weekend-project\""));
        assert!(status.contains("entry_mode: greenfield"));
        assert!(status.contains(&format!("selected_next: \"{INITIAL_GOAL_PATH}\"")));
        let goal = fs::read_to_string(root.join(INITIAL_GOAL_PATH))
            .expect("initial goal template should be readable");
        assert!(goal.contains("id: goal_initial_project_setup"));
        assert!(goal.contains("project_id: \"weekend-project\""));
        assert!(goal.contains("entry_mode: greenfield"));
        assert!(goal.contains("- \".punk/instructions/**\""));
        let punk_readme =
            fs::read_to_string(root.join(".punk/README.md")).expect("readme should be readable");
        assert!(punk_readme.contains("Local instructions start at `.punk/instructions/INDEX.md`."));
        let instruction_index = fs::read_to_string(root.join(INSTRUCTIONS_INDEX_PATH))
            .expect("instruction index should be readable");
        assert!(instruction_index.contains("# Punk Instructions"));
        assert!(instruction_index.contains("[Getting started](pages/getting-started.md)"));
        assert!(instruction_index.contains(".punk/views/instructions/page-index.json"));
        assert!(instruction_index.contains("rebuildable and advisory"));
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(project_marker.contains("schema_version = \"punk.project.v0.1\""));
        assert!(project_marker.contains("project_id = \"weekend-project\""));
        assert!(project_marker.contains("entry_mode = \"greenfield\""));
        assert!(project_marker.contains("runtime_persistence = \"inactive\""));
        assert!(project_marker.contains("live_work_state = \".punk/memory/STATUS.md\""));
        assert!(project_marker.contains("[memory]"));
        assert!(project_marker.contains("layout = \"compact\""));
        assert!(project_marker.contains("root = \".punk/memory\""));
        assert!(project_marker.contains("[instructions]"));
        assert!(project_marker.contains("root = \".punk/instructions\""));
        assert!(project_marker.contains("index = \".punk/instructions/INDEX.md\""));
        assert!(project_marker
            .contains("page_index_view = \".punk/views/instructions/page-index.json\""));
        assert!(project_marker.contains("views_active = false"));
        assert!(project_marker.contains("[runtime]"));
        assert!(project_marker.contains("active = false"));
        assert!(project_marker.contains("root = \".punk/runtime\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn greenfield_default_behavior_unchanged() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(!report.blocked());
        assert_eq!(report.entry_mode(), PROJECT_INIT_ENTRY_MODE);
        assert!(root.join(INITIAL_GOAL_PATH).is_file());
        assert!(!root.join(BROWNFIELD_BASELINE_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reconstruction").exists());

        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(project_marker.contains("entry_mode = \"greenfield\""));
        assert!(!project_marker.contains("[brownfield]"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_creates_compact_memory_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        assert_eq!(report.exit_code(), 0);
        assert_eq!(report.result_label(), "initialized");
        assert_eq!(report.entry_mode(), PROJECT_INIT_BROWNFIELD_ENTRY_MODE);
        assert!(root.join(STATUS_PATH).is_file());
        assert!(root.join(BROWNFIELD_BASELINE_GOAL_PATH).is_file());
        assert!(root.join(".punk/memory/reports").is_dir());
        assert!(root.join(".punk/memory/reconstruction").is_dir());
        assert!(root.join(".punk/README.md").is_file());
        assert!(root.join(".punk/project.toml").is_file());
        assert!(root.join(INSTRUCTIONS_INDEX_PATH).is_file());
        assert!(root.join(".punk/instructions/pages/init.md").is_file());
        assert!(root.join(".punk/instructions/modules/README.md").is_file());
        assert!(!root.join("work").exists());
        assert!(!root.join("knowledge").exists());
        assert!(!root.join("docs").exists());
        assert!(!root.join("docs/adr").exists());
        assert!(!root.join("publishing").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_records_entry_mode_brownfield() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert_eq!(report.entry_mode(), PROJECT_INIT_BROWNFIELD_ENTRY_MODE);
        let status =
            fs::read_to_string(root.join(STATUS_PATH)).expect("status template should be readable");
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(status.contains("entry_mode: brownfield"));
        assert!(project_marker.contains("entry_mode = \"brownfield\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_records_reconstruction_not_started() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        let status =
            fs::read_to_string(root.join(STATUS_PATH)).expect("status template should be readable");
        let project_marker = fs::read_to_string(root.join(".punk/project.toml"))
            .expect("project marker should be readable");
        assert!(status.contains("reconstruction_status: not_started"));
        assert!(status.contains(&format!(
            "selected_next: \"{BROWNFIELD_BASELINE_GOAL_PATH}\""
        )));
        assert!(status.contains("No project knowledge has been reconstructed yet."));
        assert!(status.contains("Existing code/docs/history are not Punk truth."));
        assert!(status
            .contains("Future reconstruction artifacts are advisory candidates until reviewed."));
        assert!(project_marker.contains("[brownfield]"));
        assert!(project_marker.contains("reconstruction_status = \"not_started\""));
        assert!(project_marker.contains("authority = \"advisory_candidates_only\""));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_creates_reconstruction_placeholders() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        for path in [
            ".punk/memory/reconstruction/README.md",
            ".punk/memory/reconstruction/source-corpus-manifest.md",
            ".punk/memory/reconstruction/claim-ledger.md",
            ".punk/memory/reconstruction/unknowns.md",
            ".punk/memory/reconstruction/contradictions.md",
            ".punk/memory/reconstruction/contract-readiness.md",
        ] {
            assert!(root.join(path).is_file(), "{path} should be created");
        }
        let source_manifest =
            fs::read_to_string(root.join(".punk/memory/reconstruction/source-corpus-manifest.md"))
                .expect("source corpus manifest should be readable");
        let claim_ledger =
            fs::read_to_string(root.join(".punk/memory/reconstruction/claim-ledger.md"))
                .expect("claim ledger should be readable");
        let contract_readiness =
            fs::read_to_string(root.join(".punk/memory/reconstruction/contract-readiness.md"))
                .expect("contract readiness should be readable");
        assert!(source_manifest.contains(
            "Future source-linked inventory of files, docs, tests, CI, schemas, and history sources."
        ));
        assert!(claim_ledger.contains("No claims are accepted automatically."));
        assert!(contract_readiness.contains("This is not a contract."));
        assert!(contract_readiness.contains("This is not a gate decision."));
        assert!(contract_readiness.contains("This is not proof."));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_does_not_scan_repo() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join("src")).expect("source dir should be created");
        fs::write(root.join("src/main.rs"), "fn main() {}\n").expect("source should be written");
        fs::write(root.join("README.md"), "# Existing project\n")
            .expect("readme should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        let source_manifest =
            fs::read_to_string(root.join(".punk/memory/reconstruction/source-corpus-manifest.md"))
                .expect("source corpus manifest should be readable");
        let claim_ledger =
            fs::read_to_string(root.join(".punk/memory/reconstruction/claim-ledger.md"))
                .expect("claim ledger should be readable");
        assert!(!source_manifest.contains("src/main.rs"));
        assert!(!source_manifest.contains("README.md"));
        assert!(!claim_ledger.contains("fn main"));
        assert!(!claim_ledger.contains("Existing project"));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_does_not_create_runtime_dirs() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        for path in [
            ".punk/events",
            ".punk/contracts",
            ".punk/runs",
            ".punk/evals",
            ".punk/decisions",
            ".punk/proofs",
            ".punk/views",
            ".punk/indexes",
            ".punk/runtime",
            ".punk/cache",
        ] {
            assert!(!root.join(path).exists(), "{path} should stay inactive");
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_does_not_create_contracts_or_claims() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(!report.blocked());
        assert!(!root.join(".punk/contracts").exists());
        assert!(!root.join(".punk/proofs").exists());
        let claim_ledger =
            fs::read_to_string(root.join(".punk/memory/reconstruction/claim-ledger.md"))
                .expect("claim ledger should be readable");
        let contract_readiness =
            fs::read_to_string(root.join(".punk/memory/reconstruction/contract-readiness.md"))
                .expect("contract readiness should be readable");
        assert!(claim_ledger.contains("No claims are accepted automatically."));
        assert!(claim_ledger.contains("No claims have been reconstructed or reviewed yet."));
        assert!(contract_readiness
            .contains("No contract-readiness claims have been reconstructed or reviewed yet."));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn brownfield_init_conflict_is_atomic_noop() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(STATUS_PATH), "custom brownfield status\n")
            .expect("custom status should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_project(&root, project_id, ProjectInitEntryMode::Brownfield);

        assert!(report.blocked());
        assert_eq!(report.exit_code(), 1);
        assert_eq!(report.result_label(), "blocked");
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == STATUS_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == BROWNFIELD_BASELINE_GOAL_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Planned
        }));
        assert_eq!(
            fs::read_to_string(root.join(STATUS_PATH)).expect("status should remain readable"),
            "custom brownfield status\n"
        );
        assert!(!root.join(BROWNFIELD_BASELINE_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reports").exists());
        assert!(!root.join(".punk/memory/reconstruction").exists());
        assert!(!root.join(INSTRUCTIONS_INDEX_PATH).exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_is_idempotent_when_existing_files_match() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let first = init_level0_project(&root, project_id.clone());
        let second = init_level0_project(&root, project_id);

        assert!(!first.blocked());
        assert!(!second.blocked());
        assert_eq!(second.result_label(), "already_initialized");
        assert!(second
            .artifacts()
            .iter()
            .all(|artifact| { artifact.status() == ProjectInitArtifactStatus::AlreadyExists }));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_reports_conflict_without_overwriting_existing_status() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(STATUS_PATH), "custom status\n")
            .expect("custom status should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert_eq!(report.exit_code(), 1);
        assert_eq!(report.result_label(), "blocked");
        assert_eq!(
            fs::read_to_string(root.join(STATUS_PATH)).expect("status should remain readable"),
            "custom status\n"
        );
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == STATUS_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reports/README.md").exists());
        assert!(!root.join(".punk/memory/adr/README.md").exists());
        assert!(!root
            .join(".punk/memory/knowledge/research/README.md")
            .exists());
        assert!(!root.join(".punk/memory/knowledge/ideas/README.md").exists());
        assert!(!root.join(INSTRUCTIONS_INDEX_PATH).exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_conflict_is_noop_for_all_other_artifacts() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory")).expect("memory dir should be created");
        fs::write(root.join(STATUS_PATH), "custom status\n")
            .expect("custom status should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == INITIAL_GOAL_PATH
                && artifact.status() == ProjectInitArtifactStatus::Planned
        }));
        assert_eq!(
            fs::read_to_string(root.join(STATUS_PATH)).expect("status should remain readable"),
            "custom status\n"
        );
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/reports/README.md").exists());
        assert!(!root.join(".punk/memory/adr/README.md").exists());
        assert!(!root.join(".punk/memory/knowledge").exists());
        assert!(!root.join(INSTRUCTIONS_INDEX_PATH).exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_failed_preflight_does_not_leave_partial_scaffold() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reports"))
            .expect("reports dir should be created");
        fs::write(
            root.join(".punk/memory/reports/README.md"),
            "custom report notes\n",
        )
        .expect("custom report readme should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert_eq!(report.result_label(), "blocked");
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == ".punk/memory/reports/README.md"
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert_eq!(
            fs::read_to_string(root.join(".punk/memory/reports/README.md"))
                .expect("report readme should remain readable"),
            "custom report notes\n"
        );
        assert!(!root.join(STATUS_PATH).exists());
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/memory/knowledge").exists());
        assert!(!root.join(".punk/memory/adr").exists());
        assert!(!root.join(".punk/instructions/pages/init.md").exists());
        assert!(!root.join(".punk/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn init_instruction_conflict_is_atomic_noop() {
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/instructions"))
            .expect("instructions dir should be created");
        fs::write(root.join(INSTRUCTIONS_INDEX_PATH), "custom instructions\n")
            .expect("custom instruction index should be written");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);

        assert!(report.blocked());
        assert_eq!(report.result_label(), "blocked");
        assert!(report.artifacts().iter().any(|artifact| {
            artifact.repo_relative_path() == INSTRUCTIONS_INDEX_PATH
                && artifact.kind() == ProjectInitArtifactKind::File
                && artifact.status() == ProjectInitArtifactStatus::Conflict
        }));
        assert_eq!(
            fs::read_to_string(root.join(INSTRUCTIONS_INDEX_PATH))
                .expect("instruction index should remain readable"),
            "custom instructions\n"
        );
        assert!(!root.join(STATUS_PATH).exists());
        assert!(!root.join(INITIAL_GOAL_PATH).exists());
        assert!(!root.join(".punk/instructions/pages/init.md").exists());
        assert!(!root.join(".punk/instructions/modules/README.md").exists());
        assert!(!root.join(".punk/project.toml").exists());

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn render_human_redacts_absolute_target_root() {
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("temp root should be created");
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");

        let report = init_level0_project(&root, project_id);
        let rendered = report.render_human();

        assert!(rendered.contains("target_root: ."));
        assert!(!rendered.contains(&root.display().to_string()));

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn manifest_default_authority_is_observed_structure() {
        let manifest = sample_source_corpus_manifest();

        assert_eq!(
            manifest.authority(),
            SourceCorpusManifestAuthority::ObservedStructure
        );
        assert_eq!(manifest.authority().as_str(), "observed_structure");
        assert!(!manifest.has_project_truth_authority());
    }

    #[test]
    fn manifest_default_status_is_advisory() {
        let manifest = sample_source_corpus_manifest();

        assert_eq!(manifest.status(), SourceCorpusManifestStatus::Advisory);
        assert_eq!(manifest.status().as_str(), "advisory");
        assert_eq!(
            manifest.schema_version(),
            SOURCE_CORPUS_MANIFEST_MODEL_SCHEMA_VERSION
        );
        assert_eq!(
            manifest.manifest_schema_version(),
            SOURCE_CORPUS_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(manifest.entry_mode(), ProjectInitEntryMode::Brownfield);
    }

    #[test]
    fn manifest_rejects_absolute_paths() {
        for invalid in [
            "/Users/vi/project/src/lib.rs",
            "/home/vi/project",
            "C:/repo/src/lib.rs",
        ] {
            assert_eq!(
                SourceCorpusRepoRelativePath::parse(invalid)
                    .expect_err("absolute path should fail"),
                SourceCorpusPathError::Absolute
            );
        }
    }

    #[test]
    fn manifest_rejects_home_paths() {
        for invalid in ["~/repo/src/lib.rs", "~vi/repo/src/lib.rs"] {
            assert_eq!(
                SourceCorpusRepoRelativePath::parse(invalid).expect_err("home path should fail"),
                SourceCorpusPathError::HomePath
            );
        }
    }

    #[test]
    fn manifest_uses_repo_relative_paths() {
        let path = SourceCorpusRepoRelativePath::parse("crates/example/src/lib.rs")
            .expect("repo-relative path should parse");

        assert_eq!(path.as_str(), "crates/example/src/lib.rs");

        for invalid in [
            "",
            ".",
            "crates/../secret.rs",
            "crates/./example.rs",
            "crates//example.rs",
            "crates\\example.rs",
            "https://example.test/repo",
        ] {
            assert!(
                SourceCorpusRepoRelativePath::parse(invalid).is_err(),
                "{invalid} should fail"
            );
        }
    }

    #[test]
    fn manifest_content_policy_defaults_to_no_contents() {
        let item = sample_source_corpus_item();
        let policy = item.content_policy();

        assert!(!policy.reads_contents());
        assert!(!policy.stores_snippets());
        assert!(!policy.summarizes_contents());
    }

    #[test]
    fn manifest_hash_policy_defaults_to_deferred_or_unset() {
        let item = sample_source_corpus_item();

        assert_eq!(item.hash_policy(), SourceCorpusHashPolicy::Deferred);
        assert_eq!(item.hash_policy().as_str(), "deferred");
        assert!(!item.hash_policy().requires_filesystem_hashing());
        assert_eq!(item.size_policy(), SourceCorpusSizePolicy::Deferred);
        assert_eq!(item.size_policy().as_str(), "deferred");
        assert!(!item.size_policy().requires_filesystem_metadata());
    }

    #[test]
    fn manifest_item_has_no_claim_fields() {
        let item = sample_source_corpus_item();

        assert!(!item.has_claim_authority());
        assert_eq!(item.observed_kind(), SourceCorpusObservedKind::File);
        assert_eq!(item.source_class(), SourceCorpusSourceClass::SourceCode);
        assert_eq!(item.tracking_status().as_str(), "observed");
        assert_eq!(item.sensitivity(), SourceCorpusSensitivityClass::Normal);
        assert_eq!(
            item.generated_or_vendored_candidate(),
            SourceCorpusGeneratedOrVendoredCandidate::None
        );
        assert_eq!(
            item.evidence_ref().as_str(),
            "brownfield-source-corpus-manifest.v0.1:weekend-project:test#item:test"
        );
        assert!(item.notes().is_empty());
    }

    #[test]
    fn manifest_item_evidence_ref_includes_manifest_id() {
        let manifest_id =
            SourceCorpusManifestId::parse("brownfield-source-corpus-manifest.v0.1:demo:run-1")
                .expect("manifest id should parse");
        let item = sample_source_corpus_item_for_manifest(&manifest_id);

        assert_eq!(
            item.evidence_ref().as_str(),
            "brownfield-source-corpus-manifest.v0.1:demo:run-1#item:test"
        );
    }

    #[test]
    fn manifest_does_not_allow_claims_created() {
        assert!(SOURCE_CORPUS_FORBIDDEN_CLAIM_FIELDS.contains(&"claims_created"));

        for forbidden in SOURCE_CORPUS_FORBIDDEN_CLAIM_FIELDS {
            assert!(
                !source_corpus_manifest_claim_field_allowed(forbidden),
                "{forbidden} should be forbidden"
            );
        }

        assert!(source_corpus_manifest_claim_field_allowed(
            "repo_relative_path"
        ));
        assert!(source_corpus_manifest_claim_field_allowed("source_class"));
    }

    #[test]
    fn manifest_source_classes_cover_boundary_docs() {
        let class_names: Vec<&str> = SourceCorpusSourceClass::ALL
            .iter()
            .map(|class| class.as_str())
            .collect();

        assert_eq!(
            class_names,
            vec![
                "source_code",
                "docs",
                "tests",
                "ci_config",
                "package_manifest",
                "schema",
                "migration",
                "script",
                "generated_candidate",
                "vendored_candidate",
                "unknown",
            ]
        );
    }

    #[test]
    fn manifest_sensitive_classes_are_caution_or_excluded() {
        let caution_names: Vec<&str> = SourceCorpusCautionClass::ALL
            .iter()
            .map(|class| class.as_str())
            .collect();

        assert_eq!(
            caution_names,
            vec![
                "secrets",
                "env_files",
                "local_cache",
                "build_output",
                "private_agent_transcripts",
                "generated_candidate",
                "vendored_candidate",
                "unknown",
            ]
        );
        assert_eq!(
            SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS,
            &[
                ".git",
                ".punk/runtime",
                ".punk/cache",
                ".punk/indexes",
                "node_modules",
                "target",
            ]
        );

        let manifest = sample_source_corpus_manifest();
        let policy = manifest.exclusion_policy();
        assert_eq!(
            policy.excluded_paths(),
            SOURCE_CORPUS_DEFAULT_EXCLUDED_PATHS
        );
        assert_eq!(policy.caution_classes(), &SourceCorpusCautionClass::ALL);
    }

    #[test]
    fn manifest_model_does_not_scan_repo() {
        let manifest = sample_source_corpus_manifest();

        assert!(!manifest.capabilities().scans_repository());
        assert!(!manifest.capabilities().uses_network());
        assert!(!manifest.capabilities().uses_remote_ai());
    }

    #[test]
    fn manifest_model_does_not_read_file_contents() {
        let manifest = sample_source_corpus_manifest();

        assert!(!manifest.capabilities().reads_file_contents());
        assert!(!manifest.capabilities().computes_file_hashes());
    }

    #[test]
    fn manifest_model_does_not_write_manifest() {
        let manifest = sample_source_corpus_manifest();

        assert!(!manifest.capabilities().walks_files());
        assert!(!manifest.capabilities().writes_manifest());
        assert!(!manifest.creates_claims());
        assert!(!manifest.capabilities().infers_intent());
    }

    #[test]
    fn preflight_allows_reconstruction_manifest_target() {
        let result =
            sample_writer_preflight_result(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);

        assert_eq!(
            SourceCorpusManifestWriterPreflight::schema_version(),
            SOURCE_CORPUS_MANIFEST_WRITER_PREFLIGHT_MODEL_SCHEMA_VERSION
        );
        assert!(result.allowed_to_write());
        assert!(result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetAllowed));
        assert!(
            result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictMissing)
        );
        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::OperationEvidenceIsNotProof));
    }

    #[test]
    fn preflight_rejects_absolute_target() {
        for target in [
            "/example/project/.punk/memory/reconstruction/source-corpus-manifest.md",
            "C:/repo/.punk/memory/reconstruction/source-corpus-manifest.md",
        ] {
            let result = sample_writer_preflight_result(target);

            assert!(result.blocked());
            assert!(
                result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetAbsolutePath),
                "{target} should be rejected as an absolute target"
            );
        }
    }

    #[test]
    fn preflight_rejects_path_escape() {
        let result =
            sample_writer_preflight_result(".punk/memory/reconstruction/../source-corpus.md");

        assert!(result.blocked());
        assert!(result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetPathEscape));
    }

    #[test]
    fn preflight_rejects_runtime_storage_target() {
        let result = sample_writer_preflight_result(".punk/runtime/source-corpus-manifest.md");

        assert!(result.blocked());
        assert!(result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::RuntimeStorageTargetForbidden
        ));
        assert!(result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::TargetNotUnderReconstructionDir
        ));
    }

    #[test]
    fn preflight_rejects_symlink_escape_when_reported() {
        let input =
            sample_writer_preflight_input(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
                .with_symlink_ancestor_status(
                    SourceCorpusManifestWriterSymlinkAncestorStatus::Escape,
                );
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::TargetSymlinkAncestorEscape));
    }

    #[test]
    fn preflight_rejects_missing_parent_when_reported() {
        let input =
            sample_writer_preflight_input(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
                .with_parent_status(SourceCorpusManifestWriterParentStatus::Missing);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetMissingParent));
    }

    #[test]
    fn preflight_blocks_different_existing_target() {
        let input =
            sample_writer_preflight_input(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
                .with_conflict_policy(
                    SourceCorpusManifestWriterConflictPolicy::DifferentExistingTarget,
                );
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::TargetConflictDifferentBlocks
        ));
    }

    #[test]
    fn preflight_allows_missing_target() {
        let input =
            sample_writer_preflight_input(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
                .with_conflict_policy(SourceCorpusManifestWriterConflictPolicy::MissingTarget);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.allowed_to_write());
        assert!(
            result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictMissing)
        );
    }

    #[test]
    fn preflight_allows_identical_existing_target_as_idempotent() {
        let input =
            sample_writer_preflight_input(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
                .with_conflict_policy(
                    SourceCorpusManifestWriterConflictPolicy::IdenticalExistingTarget,
                );
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.allowed_to_write());
        assert!(
            result.has_finding(SourceCorpusManifestWriterPreflightFinding::TargetConflictIdentical)
        );
    }

    #[test]
    fn preflight_rejects_manifest_with_non_advisory_status() {
        let manifest = sample_source_corpus_manifest();
        let inspection = SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
            .with_status_evidence(SourceCorpusManifestWriterManifestStatusEvidence::Other);
        let input = SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::default_manifest_path(),
            manifest,
        )
        .with_manifest_inspection(inspection);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::ManifestStatusNotAdvisory));
    }

    #[test]
    fn preflight_rejects_manifest_with_non_observed_authority() {
        let manifest = sample_source_corpus_manifest();
        let inspection = SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
            .with_authority_evidence(SourceCorpusManifestWriterManifestAuthorityEvidence::Other);
        let input = SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::default_manifest_path(),
            manifest,
        )
        .with_manifest_inspection(inspection);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::ManifestAuthorityNotObservedStructure
        ));
    }

    #[test]
    fn preflight_rejects_manifest_with_absolute_paths() {
        let manifest = sample_source_corpus_manifest();
        let inspection = SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
            .with_absolute_path();
        let input = SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::default_manifest_path(),
            manifest,
        )
        .with_manifest_inspection(inspection);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::ManifestContainsAbsolutePath));
    }

    #[test]
    fn preflight_rejects_manifest_with_content_snippets() {
        let manifest = sample_source_corpus_manifest();
        let inspection = SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
            .with_content_snippet();
        let input = SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::default_manifest_path(),
            manifest,
        )
        .with_manifest_inspection(inspection);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::ManifestContainsContentSnippet
        ));
    }

    #[test]
    fn preflight_rejects_manifest_with_claim_fields() {
        let manifest = sample_source_corpus_manifest();
        let inspection = SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
            .with_claim_field("intent");
        let input = SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::default_manifest_path(),
            manifest,
        )
        .with_manifest_inspection(inspection);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimField));
        assert!(!result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimsCreated
        ));
    }

    #[test]
    fn preflight_rejects_claims_created() {
        let manifest = sample_source_corpus_manifest();
        let inspection = SourceCorpusManifestWriterManifestInspection::for_manifest(&manifest)
            .with_claim_field("claims_created");
        let input = SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::default_manifest_path(),
            manifest,
        )
        .with_manifest_inspection(inspection);
        let result = SourceCorpusManifestWriterPreflight::evaluate(input);

        assert!(result.blocked());
        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimField));
        assert!(result.has_finding(
            SourceCorpusManifestWriterPreflightFinding::ManifestContainsClaimsCreated
        ));
    }

    #[test]
    fn preflight_does_not_write_manifest() {
        let result =
            sample_writer_preflight_result(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        let capabilities = result.capabilities();

        assert!(!capabilities.writes_manifest());
        assert!(!capabilities.generates_manifest());
        assert!(!capabilities.writes_runtime_storage());
    }

    #[test]
    fn preflight_does_not_scan_repo() {
        let result =
            sample_writer_preflight_result(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        let capabilities = result.capabilities();

        assert!(!capabilities.scans_repository());
        assert!(!capabilities.walks_files());
        assert!(!capabilities.uses_network());
        assert!(!capabilities.uses_remote_ai());
    }

    #[test]
    fn preflight_does_not_read_file_contents() {
        let result =
            sample_writer_preflight_result(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        let capabilities = result.capabilities();

        assert!(!capabilities.reads_file_contents());
        assert!(!capabilities.computes_file_hashes());
        assert!(!capabilities.creates_claims());
        assert!(!capabilities.infers_intent());
    }

    #[test]
    fn preflight_operation_evidence_is_not_proof() {
        let result =
            sample_writer_preflight_result(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);

        assert!(result
            .has_finding(SourceCorpusManifestWriterPreflightFinding::OperationEvidenceIsNotProof));
        assert!(!result.operation_evidence_is_proof());
        assert!(!result.operation_evidence_is_gate_decision());
        assert!(!result.operation_evidence_is_acceptance());
        assert!(!result.operation_evidence_is_project_truth());
    }

    #[test]
    fn writer_render_is_deterministic() {
        let manifest = sample_source_corpus_manifest();
        let first = source_corpus_manifest_render_canonical_bytes(&manifest);
        let second = source_corpus_manifest_render_canonical_bytes(&manifest);
        let rendered = String::from_utf8(first.clone()).expect("canonical bytes should be utf-8");

        assert_eq!(first, second);
        assert!(rendered.contains("manifest_status: advisory\n"));
        assert!(rendered.contains("authority: observed_structure\n"));
        assert!(rendered.contains("generated_at_policy: no_runtime_clock\n"));
        assert!(rendered.contains("read_contents: false\n"));
        assert!(rendered.contains("store_snippets: false\n"));
        assert!(rendered.contains("summarize_contents: false\n"));
        assert!(!rendered.contains("/Users/"));
        assert!(!rendered.contains("/home/"));
        assert!(!rendered.contains("claims_created"));
        assert!(!rendered.contains("created_at:"));
        assert!(!rendered.contains("generated_at:"));
    }

    #[test]
    fn writer_writes_canonical_manifest_to_safe_target() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reconstruction"))
            .expect("test should create explicit manifest parent");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let target_path = root.join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        let written = fs::read(&target_path).expect("manifest target should be written");
        let expected = source_corpus_manifest_render_canonical_bytes(&manifest);
        let temp_left = fs::read_dir(root.join(".punk/memory/reconstruction"))
            .expect("manifest parent should be readable")
            .any(|entry| {
                entry
                    .expect("directory entry should be readable")
                    .file_name()
                    .to_string_lossy()
                    .contains(".tmp.")
            });
        let runtime_exists = root.join(".punk/runtime").exists();
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert_eq!(
            result.schema_version(),
            SOURCE_CORPUS_MANIFEST_WRITER_FIRST_SLICE_SCHEMA_VERSION
        );
        assert_eq!(
            result.evidence_status(),
            SourceCorpusManifestWriterOperationEvidenceStatus::Written
        );
        assert!(result.wrote_manifest());
        assert!(result.write_attempted());
        assert_eq!(result.canonical_byte_len(), expected.len());
        assert!(result.blockers().is_empty());
        assert_eq!(written, expected);
        assert!(!temp_left);
        assert!(!runtime_exists);
    }

    #[test]
    fn writer_requires_preflight_pass() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reconstruction"))
            .expect("test should create explicit manifest parent");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::DifferentExistingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let target_exists = root
            .join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
            .exists();
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(preflight.blocked());
        assert_eq!(
            result.evidence_status(),
            SourceCorpusManifestWriterOperationEvidenceStatus::Conflict
        );
        assert!(result
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent));
        assert!(!result.write_attempted());
        assert!(!target_exists);
    }

    #[test]
    fn writer_blocks_on_failed_preflight() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("test root should exist");
        let target = SourceCorpusManifestWriterTarget::new(
            "/example/project/.punk/memory/reconstruction/source-corpus-manifest.md",
        );
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(preflight.blocked());
        assert!(result.is_blocked());
        assert!(result
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::BlockingPreflightFinding));
        assert!(!result.write_attempted());
    }

    #[test]
    fn writer_rejects_absolute_target() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("test root should exist");
        let target = SourceCorpusManifestWriterTarget::new(
            "/example/project/.punk/memory/reconstruction/source-corpus-manifest.md",
        );
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_blocked());
        assert!(result.has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::TargetPathInvalid));
        assert!(!result.write_attempted());
    }

    #[test]
    fn writer_rejects_path_escape() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reconstruction"))
            .expect("test should create explicit manifest parent");
        let target =
            SourceCorpusManifestWriterTarget::new(".punk/memory/reconstruction/../manifest.md");
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let escaped_target_exists = root.join(".punk/memory/manifest.md").exists();
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_blocked());
        assert!(result.has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::TargetPathInvalid));
        assert!(!result.write_attempted());
        assert!(!escaped_target_exists);
    }

    #[test]
    fn writer_rejects_runtime_storage_target() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("test root should exist");
        let target =
            SourceCorpusManifestWriterTarget::new(".punk/runtime/source-corpus-manifest.md");
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let runtime_exists = root.join(".punk/runtime").exists();
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_blocked());
        assert!(result.has_blocker(
            SourceCorpusManifestWriterFirstSliceBlocker::TargetRuntimeStorageForbidden
        ));
        assert!(!result.write_attempted());
        assert!(!runtime_exists);
    }

    #[test]
    fn writer_rejects_symlink_escape_when_reported() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reconstruction"))
            .expect("test should create explicit manifest parent");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight_input =
            SourceCorpusManifestWriterPreflightInput::new(target.clone(), manifest.clone())
                .with_symlink_ancestor_status(
                    SourceCorpusManifestWriterSymlinkAncestorStatus::Escape,
                );
        let preflight = SourceCorpusManifestWriterPreflight::evaluate(preflight_input);

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let target_exists = root
            .join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
            .exists();
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_blocked());
        assert!(result
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::BlockingPreflightFinding));
        assert!(!result.write_attempted());
        assert!(!target_exists);
    }

    #[cfg(unix)]
    #[test]
    fn writer_blocks_actual_intermediate_parent_symlink() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        let outside_root = unique_temp_path();
        fs::create_dir_all(root.join(".punk")).expect("test should create .punk parent");
        fs::create_dir_all(outside_root.join("reconstruction"))
            .expect("test should create outside reconstruction parent");
        std::os::unix::fs::symlink(&outside_root, root.join(".punk/memory"))
            .expect("test should create intermediate parent symlink");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let outside_target_exists = outside_root
            .join("reconstruction/source-corpus-manifest.md")
            .exists();
        fs::remove_dir_all(&root).expect("test root should clean up");
        fs::remove_dir_all(&outside_root).expect("outside root should clean up");

        assert!(result.is_blocked());
        assert!(
            result.has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::TargetParentSymlink)
        );
        assert!(!result.write_attempted());
        assert!(!outside_target_exists);
    }

    #[test]
    fn writer_blocks_different_existing_content() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        let target_path = root.join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        fs::create_dir_all(target_path.parent().expect("target should have parent"))
            .expect("test should create explicit manifest parent");
        fs::write(&target_path, b"different manifest bytes")
            .expect("test should pre-create conflicting target");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::DifferentExistingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let remaining = fs::read(&target_path).expect("target should remain readable");
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.has_conflict());
        assert!(result
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent));
        assert!(!result.write_attempted());
        assert_eq!(remaining, b"different manifest bytes");
    }

    #[test]
    fn writer_is_idempotent_on_identical_content() {
        let manifest = sample_source_corpus_manifest();
        let canonical = source_corpus_manifest_render_canonical_bytes(&manifest);
        let root = unique_temp_path();
        let target_path = root.join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        fs::create_dir_all(target_path.parent().expect("target should have parent"))
            .expect("test should create explicit manifest parent");
        fs::write(&target_path, &canonical).expect("test should pre-create matching target");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::IdenticalExistingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let remaining = fs::read(&target_path).expect("target should remain readable");
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_idempotent());
        assert!(!result.write_attempted());
        assert_eq!(remaining, canonical);
    }

    #[test]
    fn writer_rechecks_identical_target_bytes_before_idempotent() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        let target_path = root.join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        fs::create_dir_all(target_path.parent().expect("target should have parent"))
            .expect("test should create explicit manifest parent");
        fs::write(&target_path, b"changed after preflight")
            .expect("test should pre-create stale conflicting target");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::IdenticalExistingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let remaining = fs::read(&target_path).expect("target should remain readable");
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.has_conflict());
        assert!(result
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetDifferent));
        assert!(!result.write_attempted());
        assert_eq!(remaining, b"changed after preflight");
    }

    #[cfg(unix)]
    #[test]
    fn writer_blocks_when_identical_target_cannot_be_read() {
        use std::os::unix::fs::PermissionsExt;

        let manifest = sample_source_corpus_manifest();
        let canonical = source_corpus_manifest_render_canonical_bytes(&manifest);
        let root = unique_temp_path();
        let target_path = root.join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH);
        fs::create_dir_all(target_path.parent().expect("target should have parent"))
            .expect("test should create explicit manifest parent");
        fs::write(&target_path, &canonical).expect("test should pre-create matching target");
        let unreadable_permissions = std::fs::Permissions::from_mode(0o000);
        fs::set_permissions(&target_path, unreadable_permissions)
            .expect("test should make target unreadable");
        if fs::read(&target_path).is_ok() {
            // Privileged runners may still read 000 files; this fixture cannot
            // exercise the unreadable-target branch in that environment.
            let readable_permissions = std::fs::Permissions::from_mode(0o600);
            fs::set_permissions(&target_path, readable_permissions)
                .expect("test should restore target permissions");
            fs::remove_dir_all(&root).expect("test root should clean up");
            return;
        }
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::IdenticalExistingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let readable_permissions = std::fs::Permissions::from_mode(0o600);
        fs::set_permissions(&target_path, readable_permissions)
            .expect("test should restore target permissions");
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_blocked());
        assert!(result
            .has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::ExistingTargetReadFailed));
        assert!(!result.write_attempted());
    }

    #[test]
    fn writer_no_partial_target_on_failure() {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(&root).expect("test root should exist");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );

        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        let target_exists = root
            .join(SOURCE_CORPUS_MANIFEST_WRITER_DEFAULT_TARGET_PATH)
            .exists();
        let reconstruction_exists = root.join(".punk/memory/reconstruction").exists();
        fs::remove_dir_all(&root).expect("test root should clean up");

        assert!(result.is_blocked());
        assert!(
            result.has_blocker(SourceCorpusManifestWriterFirstSliceBlocker::TargetParentMissing)
        );
        assert!(!result.write_attempted());
        assert!(!target_exists);
        assert!(!reconstruction_exists);
    }

    #[test]
    fn writer_does_not_scan_repo() {
        let result = sample_writer_first_slice_result();
        let capabilities = result.capabilities();

        assert!(capabilities.takes_manifest_model_input());
        assert!(!capabilities.scans_repository());
        assert!(!capabilities.walks_files());
        assert!(!capabilities.uses_network());
        assert!(!capabilities.uses_remote_ai());
    }

    #[test]
    fn writer_does_not_read_file_contents() {
        let result = sample_writer_first_slice_result();
        let capabilities = result.capabilities();

        assert!(!capabilities.reads_source_file_contents());
        assert!(!capabilities.generates_manifest_from_repo());
    }

    #[test]
    fn writer_does_not_compute_file_hashes() {
        let result = sample_writer_first_slice_result();
        let capabilities = result.capabilities();

        assert!(!capabilities.computes_source_file_hashes());
    }

    #[test]
    fn writer_does_not_create_claims() {
        let result = sample_writer_first_slice_result();
        let capabilities = result.capabilities();

        assert!(!capabilities.creates_claims());
        assert!(!result.creates_claims());
    }

    #[test]
    fn writer_does_not_promote_manifest_authority() {
        let result = sample_writer_first_slice_result();
        let capabilities = result.capabilities();

        assert!(!capabilities.promotes_manifest_authority());
        assert!(!result.promotes_manifest_authority());
    }

    #[test]
    fn writer_operation_evidence_is_not_proof() {
        let result = sample_writer_first_slice_result();

        assert!(!result.operation_evidence_is_proof());
    }

    #[test]
    fn writer_operation_evidence_is_not_gate_decision() {
        let result = sample_writer_first_slice_result();

        assert!(!result.operation_evidence_is_gate_decision());
    }

    #[test]
    fn writer_operation_evidence_is_not_acceptance() {
        let result = sample_writer_first_slice_result();

        assert!(!result.operation_evidence_is_acceptance());
        assert!(!result.operation_evidence_is_project_truth());
    }

    #[test]
    fn project_id_requires_lowercase_slug() {
        assert_eq!(
            ProjectId::parse("").expect_err("empty project id should fail"),
            ProjectIdError::Empty
        );
        for invalid in [
            "Weekend",
            "weekend project",
            "-weekend",
            "weekend-",
            "weekend--project",
            "weekend_project",
            "weekend.project",
        ] {
            assert_eq!(
                ProjectId::parse(invalid).expect_err("invalid project id should fail"),
                ProjectIdError::InvalidFormat
            );
        }

        assert_eq!(
            ProjectId::parse("weekend-project-01")
                .expect("valid project id should parse")
                .as_str(),
            "weekend-project-01"
        );
    }

    fn sample_source_corpus_manifest() -> SourceCorpusManifest {
        let project_id = ProjectId::parse("weekend-project").expect("project id should parse");
        let manifest_id = SourceCorpusManifestId::parse(
            "brownfield-source-corpus-manifest.v0.1:weekend-project:local-smoke",
        )
        .expect("manifest id should parse");
        let item = sample_source_corpus_item_for_manifest(&manifest_id);
        SourceCorpusManifest::new(manifest_id, project_id, vec![item])
    }

    fn sample_source_corpus_item() -> SourceCorpusItem {
        let manifest_id = SourceCorpusManifestId::parse(
            "brownfield-source-corpus-manifest.v0.1:weekend-project:test",
        )
        .expect("manifest id should parse");
        sample_source_corpus_item_for_manifest(&manifest_id)
    }

    fn sample_source_corpus_item_for_manifest(
        manifest_id: &SourceCorpusManifestId,
    ) -> SourceCorpusItem {
        let item_id = SourceCorpusItemId::parse("item:test").expect("item id should parse");
        let repo_relative_path = SourceCorpusRepoRelativePath::parse("crates/example/src/lib.rs")
            .expect("repo-relative path should parse");
        let marker = SourceCorpusSourceMarker::new(SourceCorpusSourceMarkerKind::Extension, "rs")
            .expect("source marker should parse");
        let item = SourceCorpusItem::new(
            &manifest_id,
            item_id,
            repo_relative_path,
            SourceCorpusObservedKind::File,
            SourceCorpusSourceClass::SourceCode,
        )
        .expect("source corpus item should be built")
        .with_source_marker(marker);

        assert_eq!(item.item_id().as_str(), "item:test");
        assert_eq!(
            item.repo_relative_path().as_str(),
            "crates/example/src/lib.rs"
        );
        assert_eq!(item.source_markers()[0].kind().as_str(), "extension");
        assert_eq!(item.source_markers()[0].value(), "rs");

        item
    }

    fn sample_writer_preflight_input(target: &str) -> SourceCorpusManifestWriterPreflightInput {
        SourceCorpusManifestWriterPreflightInput::new(
            SourceCorpusManifestWriterTarget::new(target),
            sample_source_corpus_manifest(),
        )
    }

    fn sample_writer_preflight_result(
        target: &str,
    ) -> super::SourceCorpusManifestWriterPreflightResult {
        SourceCorpusManifestWriterPreflight::evaluate(sample_writer_preflight_input(target))
    }

    fn sample_writer_first_slice_preflight(
        manifest: &SourceCorpusManifest,
        target: SourceCorpusManifestWriterTarget,
        conflict_policy: SourceCorpusManifestWriterConflictPolicy,
    ) -> super::SourceCorpusManifestWriterPreflightResult {
        SourceCorpusManifestWriterPreflight::evaluate(
            SourceCorpusManifestWriterPreflightInput::new(target, manifest.clone())
                .with_conflict_policy(conflict_policy),
        )
    }

    fn sample_writer_first_slice_result() -> super::SourceCorpusManifestWriterFirstSliceResult {
        let manifest = sample_source_corpus_manifest();
        let root = unique_temp_path();
        fs::create_dir_all(root.join(".punk/memory/reconstruction"))
            .expect("test should create explicit manifest parent");
        let target = SourceCorpusManifestWriterTarget::default_manifest_path();
        let preflight = sample_writer_first_slice_preflight(
            &manifest,
            target.clone(),
            SourceCorpusManifestWriterConflictPolicy::MissingTarget,
        );
        let result =
            source_corpus_manifest_writer_write_first_slice(&manifest, &root, target, &preflight);
        fs::remove_dir_all(&root).expect("test root should clean up");

        result
    }

    fn unique_temp_path() -> std::path::PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock should be after epoch")
            .as_nanos();
        let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "punk-project-init-test-{}-{}-{}",
            process::id(),
            unique,
            counter
        ))
    }
}
